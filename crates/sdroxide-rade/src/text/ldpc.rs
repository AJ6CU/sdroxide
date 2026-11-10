//! LDPC(112,56) encoder and soft-decision sum-product decoder for the RADE V1
//! End-of-Over text channel.
//!
//! Ported from `ldpc_encode.cpp` / `ldpc_decode.cpp` in
//! <https://github.com/tmiw/freedv-backend> (BSD-2-Clause, Mooneer Salem).
//!
//! This is a deliberately *literal* port: the arithmetic is reproduced
//! operation for operation, including the places where the reference relies on
//! C's float/double promotion rules and on `std::max`'s NaN behaviour. Anything
//! that looks like it could be simplified almost certainly changes the decoder's
//! output on some input, so the odd-looking parts carry comments explaining why
//! they are the way they are.

// The index loops here are the reference's, and the index is usually the thing
// being reasoned about — a variable-node number, a bit position — rather than
// incidental. Rewriting them as iterator chains would break the line-by-line
// correspondence that makes this port auditable.
#![allow(clippy::needless_range_loop)]

use std::sync::LazyLock;

use super::hra::{self, K, N};

/// Saturation bound on every log-likelihood ratio.
const LLR_MAX: f32 = 10_000.0;
/// Belief-propagation iteration cap (the reference's default).
const MAX_ITER: usize = 30;

/// A received QPSK symbol, normalised to unit magnitude by the caller.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Sym {
    pub re: f32,
    pub im: f32,
}

/// What the decoder made of one codeword.
pub(crate) struct Decoded {
    /// Hard decisions for all [`N`] bits. Only the first [`K`] are information.
    pub bits: [u8; N],
    /// True when every parity check was satisfied.
    pub converged: bool,
}

// ─────────────────────────── encoder ───────────────────────────

/// Systematic LDPC(112,56) encode.
///
/// `H_b` is lower-bidiagonal, so the parity bits fall out by forward
/// substitution: `p[0] = r[0]` and `p[i] = r[i] ^ p[i-1]`, where `r = H_a · s`
/// over GF(2). No matrix inversion, no generator matrix.
pub(crate) fn encode(s: &[u8; K]) -> [u8; N] {
    let mut c = [0u8; N];
    c[..K].copy_from_slice(s);
    for i in 0..K {
        // Row parity over the systematic half.
        let mut parity = 0u8;
        for col in hra::HA_COLS[i] {
            parity ^= c[col as usize] & 1;
        }
        c[K + i] = if i == 0 { parity } else { parity ^ c[K + i - 1] };
    }
    c
}

/// True when `bits` satisfies every parity check.
pub(crate) fn syndrome_ok(bits: &[u8; N]) -> bool {
    (0..K).all(|i| h_row_parity(i, bits) == 0)
}

fn h_row_parity(i: usize, bits: &[u8; N]) -> u8 {
    hra::h_row(i).fold(0u8, |acc, j| acc ^ (bits[j] & 1))
}

// ─────────────────────────── Tanner graph ───────────────────────────

struct Graph {
    /// `(check, var)` per edge, enumerated row-major as the reference does.
    edge_var: Vec<usize>,
    /// Edge indices incident on each check node.
    check_edges: Vec<Vec<usize>>,
    /// Edge indices incident on each variable node.
    var_edges: Vec<Vec<usize>>,
}

/// Built once. Unlike the reference — whose message arrays are file-scope
/// statics and therefore not thread-safe — only this immutable structure is
/// shared; the per-decode messages live on the stack of [`decode`].
static GRAPH: LazyLock<Graph> = LazyLock::new(|| {
    let mut g = Graph {
        edge_var: Vec::with_capacity(279),
        check_edges: vec![Vec::new(); K],
        var_edges: vec![Vec::new(); N],
    };
    for i in 0..K {
        for j in hra::h_row(i) {
            let e = g.edge_var.len();
            g.edge_var.push(j);
            g.check_edges[i].push(e);
            g.var_edges[j].push(e);
        }
    }
    g
});

// ─────────────────────────── decoder ───────────────────────────

/// `phi(x) = ln((e^x + 1) / (e^x - 1))`, the sum-product check-node kernel.
///
/// Computed in `f64` and narrowed, exactly as the reference does.
///
/// It returns NaN for large `x`: `e^x` overflows to infinity, `(inf+1)/(inf-1)`
/// is NaN, and `ln(NaN)` is NaN. That happens on every saturated LLR — i.e. on
/// every strong signal — and the reference swallows it via `std::max(0.0f, …)`,
/// which returns `0.0f` because `0.0f < NaN` is false. Rust's `f32::max` ignores
/// NaN the same way, so `phi(x).max(0.0)` reproduces it. Do not "fix" this: the
/// NaN path is load-bearing, and removing it changes decoder behaviour.
fn phi(x: f32) -> f32 {
    if x < 1e-10 {
        return LLR_MAX;
    }
    let ex = (x as f64).exp();
    if ex < 1e-10 {
        return LLR_MAX;
    }
    (((ex + 1.0) / (ex - 1.0)).ln()) as f32
}

/// Linear approximation of `max*(x, y) = ln(e^x + e^y)`.
///
/// The reference's literals are `double`, which promotes `absdiff` for both the
/// comparisons and the polynomial before the result is narrowed back to `f32`.
/// Reproduced here, since evaluating it wholly in `f32` gives different last
/// bits.
fn max_star(x: f32, y: f32) -> f32 {
    let max_xy = if x < y { y } else { x };
    let absdiff = (x - y).abs() as f64;
    let log_xy: f32 = if absdiff <= 2.45 {
        (-0.24 * absdiff + 0.596) as f32
    } else if absdiff <= 3.5 {
        0.048
    } else {
        0.0
    };
    max_xy + log_xy
}

/// Log-likelihood that `sym` was the constellation point `(re, im)`.
///
/// `var` is Es/No. The multiplication is left-associative in the reference —
/// `((-var * a) * a) * err` — and is written that way here so the rounding
/// matches.
fn calc_likelihood(sym: Sym, var: f32, re: f32, im: f32, amp: f32, avg_amp: f32) -> f32 {
    let err_r = sym.re - re;
    let err_i = sym.im - im;
    let a = amp / avg_amp;
    -var * a * a * (err_r * err_r + err_i * err_i)
}

/// Channel LLRs for 56 QPSK symbols, by the simplified max-log-MAP.
///
/// Constellation (bit pair → point): `00 → (1,0)`, `01 → (0,1)`,
/// `10 → (0,-1)`, `11 → (-1,0)`. Bit `2k` is the I decision, bit `2k+1` the Q.
/// Positive LLR means the bit is more likely 0.
fn channel_llrs(syms: &[Sym; K], amps: &[f32; K], noise_var: f32) -> [f32; N] {
    let mean_amp = amps.iter().sum::<f32>() / K as f32;
    let es_no = 1.0 / (2.0 * noise_var);

    let mut llr = [0.0f32; N];
    for k in 0..K {
        let a = amps[k];
        let like = |re, im| calc_likelihood(syms[k], es_no, re, im, a, mean_amp);
        // Indexed by bit pair: 00, 01, 10, 11.
        let l = [like(1.0, 0.0), like(0.0, 1.0), like(0.0, -1.0), like(-1.0, 0.0)];

        let mut num = [-LLR_MAX; 2];
        let mut den = [-LLR_MAX; 2];
        // 00: both bits are 0.
        den[0] = max_star(den[0], l[0]);
        den[1] = max_star(den[1], l[0]);
        // 01: bit 0 is 0, bit 1 is 1.
        den[0] = max_star(den[0], l[1]);
        num[1] = max_star(num[1], l[1]);
        // 10: bit 0 is 1, bit 1 is 0.
        num[0] = max_star(num[0], l[2]);
        den[1] = max_star(den[1], l[2]);
        // 11: both bits are 1.
        num[0] = max_star(num[0], l[3]);
        num[1] = max_star(num[1], l[3]);

        // Negated so that positive means "more likely 0".
        llr[2 * k] = -(num[0] - den[0]).clamp(-LLR_MAX, LLR_MAX);
        llr[2 * k + 1] = -(num[1] - den[1]).clamp(-LLR_MAX, LLR_MAX);
    }
    llr
}

/// Soft-decision decode of one codeword.
///
/// `syms` must be normalised to unit magnitude with the original magnitudes
/// passed separately in `amps`; `noise_var` is the per-component AWGN variance.
pub(crate) fn decode(syms: &[Sym; K], amps: &[f32; K], noise_var: f32) -> Decoded {
    let noise_var = if noise_var < 1e-10 { 1e-10 } else { noise_var };
    let llr_ch = channel_llrs(syms, amps, noise_var);

    let g = &*GRAPH;
    let n_edges = g.edge_var.len();
    // Variable-to-check and check-to-variable messages, one per edge.
    let mut m_vc: Vec<f32> = g.edge_var.iter().map(|&j| llr_ch[j]).collect();
    let mut m_cv: Vec<f32> = vec![0.0; n_edges];

    let mut out = Decoded { bits: [0u8; N], converged: false };

    for _ in 0..MAX_ITER {
        // Check → variable: r(i→j) = ∏sign(q) · phi(Σ phi(|q|)), both excluding
        // the edge being written.
        for i in 0..K {
            let ce = &g.check_edges[i];
            let mut sign = 1.0f32;
            let mut sum = 0.0f32;
            for &e in ce {
                let v = m_vc[e];
                sign *= if v >= 0.0 { 1.0 } else { -1.0 };
                sum += phi(v.abs()).max(0.0);
            }
            for &e in ce {
                let v = m_vc[e];
                // Dividing this edge out of the product: its own sign multiplied
                // back in cancels it, since every sign is ±1.
                let inv_sign = if v >= 0.0 { 1.0 } else { -1.0 };
                m_cv[e] =
                    (inv_sign * sign * phi(sum - phi(v.abs())).max(0.0)).clamp(-LLR_MAX, LLR_MAX);
            }
        }

        // Variable → check: q(j→i) = λ_j + Σ r, excluding the edge written.
        for j in 0..N {
            let ve = &g.var_edges[j];
            let mut total = llr_ch[j];
            for &e in ve {
                total += m_cv[e];
            }
            for &e in ve {
                m_vc[e] = (total - m_cv[e]).clamp(-LLR_MAX, LLR_MAX);
            }
        }

        // Posterior, hard decision, syndrome check.
        for j in 0..N {
            let mut l = llr_ch[j];
            for &e in &g.var_edges[j] {
                l += m_cv[e];
            }
            out.bits[j] = u8::from(l < 0.0);
        }
        if syndrome_ok(&out.bits) {
            out.converged = true;
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// xorshift64* — deterministic, so a failure is always reproducible.
    struct Rng(u64);
    impl Rng {
        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }
        fn bit(&mut self) -> u8 {
            (self.next_u64() >> 33) as u8 & 1
        }
    }

    fn random_info(rng: &mut Rng) -> [u8; K] {
        let mut s = [0u8; K];
        for b in &mut s {
            *b = rng.bit();
        }
        s
    }

    #[test]
    fn encode_gives_a_zero_syndrome() {
        let mut rng = Rng(0x1234_5678_9abc_def0);
        for _ in 0..1000 {
            let s = random_info(&mut rng);
            let c = encode(&s);
            assert!(syndrome_ok(&c), "H · c != 0 for {s:?}");
        }
    }

    #[test]
    fn encode_is_systematic() {
        let mut rng = Rng(99);
        for _ in 0..50 {
            let s = random_info(&mut rng);
            assert_eq!(encode(&s)[..K], s[..]);
        }
    }

    /// Ideal (noiseless) symbols for a codeword, without interleaving —
    /// the interleaver is the caller's business.
    fn modulate(c: &[u8; N]) -> [Sym; K] {
        let mut syms = [Sym::default(); K];
        for (k, s) in syms.iter_mut().enumerate() {
            *s = match (c[2 * k], c[2 * k + 1]) {
                (0, 0) => Sym { re: 1.0, im: 0.0 },
                (0, 1) => Sym { re: 0.0, im: 1.0 },
                (1, 0) => Sym { re: 0.0, im: -1.0 },
                _ => Sym { re: -1.0, im: 0.0 },
            };
        }
        syms
    }

    #[test]
    fn noiseless_decode_is_exact() {
        let mut rng = Rng(0xdead_beef);
        for _ in 0..100 {
            let s = random_info(&mut rng);
            let c = encode(&s);
            let d = decode(&modulate(&c), &[1.0; K], 1e-6);
            assert!(d.converged, "clean symbols must converge");
            assert_eq!(d.bits, c, "clean symbols must decode exactly");
        }
    }

    #[test]
    fn a_single_bit_error_is_corrected() {
        let mut rng = Rng(4242);
        for trial in 0..N {
            let s = random_info(&mut rng);
            let c = encode(&s);
            let mut syms = modulate(&c);
            // Rotate one symbol onto a neighbouring constellation point.
            let k = trial % K;
            syms[k] = Sym { re: syms[k].im, im: syms[k].re };
            let d = decode(&syms, &[1.0; K], 0.25);
            assert!(d.converged && d.bits == c, "symbol {k} not recovered");
        }
    }

    #[test]
    fn phi_saturates_rather_than_leaking_nan() {
        // The load-bearing NaN path: phi(LLR_MAX) is NaN, and `.max(0.0)` must
        // turn it into zero exactly as the C++ `std::max(0.0f, …)` does.
        assert!(phi(LLR_MAX).is_nan());
        assert_eq!(phi(LLR_MAX).max(0.0), 0.0);
        assert_eq!(phi(0.0), LLR_MAX, "phi(0) saturates");
        assert!(phi(1.0).is_finite());
    }

    #[test]
    fn max_star_matches_the_reference_breakpoints() {
        // Below 2.45 the linear piece applies, then a constant, then nothing.
        assert!((max_star(0.0, 0.0) - 0.596).abs() < 1e-6);
        assert!((max_star(3.0, 0.0) - (3.0 + 0.048)).abs() < 1e-6);
        assert_eq!(max_star(10.0, 0.0), 10.0);
        // Symmetric in its arguments.
        assert_eq!(max_star(1.5, -0.5), max_star(-0.5, 1.5));
    }
}
