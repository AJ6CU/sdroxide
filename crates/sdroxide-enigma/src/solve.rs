//! Breaking a copied ciphertext.
//!
//! The machine has a keyspace no one could search exhaustively today: rotor
//! choice, order, ring settings, start positions and a plugboard of up to
//! thirteen cables. What the wartime cryptanalysts exploited — and what this
//! solver leans on — is that the *plaintext is language*, so a wrong setting
//! scores as noise and a right one scores as German or English.
//!
//! The scoring function is the **Index of Coincidence**: the chance that two
//! letters drawn from the text at random are the same. Natural language sits
//! near 0.066 (English) or 0.076 (German); random text sits near 0.038. It is
//! crude and it is exactly good enough to steer a search, which is why it has
//! been the tool for this since the war.
//!
//! The search runs in the order that costs least for what it buys:
//!
//! 1. **Rotor order and start positions**, with rings at A-A-A and no
//!    plugboard. `26^3 = 17576` starts per wheel order is the bulk of the work
//!    and needs no language model beyond the IoC.
//! 2. **Ring settings**, holding the wheels found above — a second, smaller
//!    pass.
//! 3. **The plugboard**, one cable at a time, greedily: try every partner for
//!    a letter, keep the swap that raises the score most, repeat until no swap
//!    helps. This is the part the operator said matters, and it is what turns a
//!    readable-but-noisy decrypt into a clean one.
//!
//! The no-self-encipher property is used where it is cheap and decisive: when
//! a **crib** (a guessed word known to be in the message) is supplied, any
//! alignment where a ciphertext letter equals the crib letter at the same
//! position is impossible and is skipped without trying it.

use crate::machine::{Enigma, Plugboard, REFLECTORS, ROTORS, Variant, Wheel, letter};

/// What the solver is told, and how hard it is allowed to try.
#[derive(Clone)]
pub struct SolveParams {
    pub variant: Variant,
    /// A word known (or guessed) to be in the plaintext, e.g. `"WETTER"` or
    /// `"THE"`. Empty for a blind solve. A crib makes the search dramatically
    /// faster and more reliable.
    pub crib: String,
    /// Search the ring settings too (step 2). Off by default: it is the
    /// expensive pass, and most ciphertexts break on steps 1 and 3 alone.
    pub search_rings: bool,
    /// Recover a plugboard (step 3). On by default — the operator asked for it,
    /// and a wartime message almost always had cables in.
    pub search_plugboard: bool,
    /// Greek wheel to try in the M4's fourth slot, if any (`None` = try both).
    pub m4_thin: Option<usize>,
}

impl Default for SolveParams {
    fn default() -> Self {
        SolveParams {
            variant: Variant::EnigmaI,
            crib: String::new(),
            search_rings: false,
            search_plugboard: true,
            m4_thin: None,
        }
    }
}

/// One candidate setting and its decrypt, with the score that chose it.
#[derive(Clone)]
pub struct Solution {
    /// Rotor indices left to right (for the M4 the first is the thin wheel).
    pub rotors: Vec<usize>,
    /// Ring settings left to right (0–25).
    pub rings: Vec<u8>,
    /// Start positions left to right (0–25).
    pub starts: Vec<u8>,
    pub reflector: usize,
    pub plugboard: Plugboard,
    /// The decrypt, letters only.
    pub plaintext: String,
    /// The Index of Coincidence of `plaintext`.
    pub score: f64,
}

impl Solution {
    /// The setting as a key sheet would write it, e.g.
    /// `"I II III  UKW-B  Ring A-A-A  Start X-Y-Z  Stecker AB CD"`.
    pub fn describe(&self) -> String {
        let rotors: Vec<&str> = self.rotors.iter().map(|&r| ROTORS[r].name).collect();
        let ring: Vec<String> = self.rings.iter().map(|&b| letter(b).to_string()).collect();
        let start: Vec<String> = self.starts.iter().map(|&b| letter(b).to_string()).collect();
        let stecker = self.plugboard.to_string_pairs();
        format!(
            "{}  {}  Ring {}  Start {}  Stecker {}",
            rotors.join(" "),
            REFLECTORS[self.reflector].name,
            ring.join("-"),
            start.join("-"),
            if stecker.is_empty() { "none".into() } else { stecker },
        )
    }
}

/// Letters only, uppercased — the solver never sees anything else.
fn letters(text: &str) -> Vec<u8> {
    text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c.to_ascii_uppercase() as u8) - b'A')
        .collect()
}

/// The Index of Coincidence of a ciphertext, normalised so random ≈ 0.038 and
/// English ≈ 0.066. `n` letters, `f_i` counts: `Σ f_i(f_i−1) / (n(n−1))`.
pub fn index_of_coincidence(text: &[u8]) -> f64 {
    let n = text.len();
    if n < 2 {
        return 0.0;
    }
    let mut counts = [0u32; 26];
    for &c in text {
        counts[(c % 26) as usize] += 1;
    }
    let num: u32 = counts.iter().map(|&f| f * f.saturating_sub(1)).sum();
    num as f64 / (n as f64 * (n as f64 - 1.0))
}

/// Decrypt `cipher` under a full setting, letters only.
#[allow(clippy::too_many_arguments)]
fn decrypt(
    cipher: &[u8],
    variant: Variant,
    rotors: &[usize],
    rings: &[u8],
    starts: &[u8],
    reflector: usize,
    pb: &Plugboard,
) -> Vec<u8> {
    let wheels: Vec<Wheel> =
        (0..rotors.len()).map(|i| Wheel::new(rotors[i], rings[i], starts[i])).collect();
    let mut e = Enigma::new(variant, wheels, reflector).with_plugboard(*pb);
    cipher.iter().map(|&c| e.encipher(c)).collect()
}

/// Does the crib sit in this decrypt, at some offset? Uses the no-self-
/// encipher flaw to reject impossible alignments cheaply, then checks the word.
/// Returns the score contribution: 0 if absent, else a fixed bonus per letter.
fn crib_hits(cipher: &[u8], plain: &[u8], crib: &[u8]) -> usize {
    if crib.is_empty() || crib.len() > plain.len() {
        return 0;
    }
    let mut hits = 0;
    for w in 0..=plain.len() - crib.len() {
        // The flaw: ciphertext == plaintext is impossible, so a crib letter
        // equal to the ciphertext letter at the same spot cannot line up here.
        if (0..crib.len()).any(|i| cipher[w + i] == crib[i]) {
            continue;
        }
        if (0..crib.len()).all(|i| plain[w + i] == crib[i]) {
            hits += crib.len();
        }
    }
    hits
}

/// Score a candidate: IoC, plus a strong bonus if the crib is present. The
/// crib dominates when there is one, because a real word match is worth more
/// than any statistical preference.
fn score(cipher: &[u8], plain: &[u8], crib: &[u8]) -> f64 {
    let ioc = index_of_coincidence(plain);
    if crib.is_empty() { ioc } else { ioc + crib_hits(cipher, plain, crib) as f64 * 0.1 }
}

/// Recover the plugboard greedily, one cable at a time, keeping any swap that
/// raises the score. Returns the best board found and its score.
#[allow(clippy::too_many_arguments)]
fn recover_plugboard(
    cipher: &[u8],
    variant: Variant,
    rotors: &[usize],
    rings: &[u8],
    starts: &[u8],
    reflector: usize,
    crib: &[u8],
    mut pb: Plugboard,
) -> (Plugboard, f64) {
    let mut best_score = {
        let plain = decrypt(cipher, variant, rotors, rings, starts, reflector, &pb);
        score(cipher, &plain, crib)
    };
    // Up to thirteen cables, the machine's limit.
    for _ in 0..13 {
        let mut improved = false;
        'outer: for a in 0..26u8 {
            if pb.through(a) != a {
                continue; // already patched
            }
            for b in (a + 1)..26u8 {
                if pb.through(b) != b {
                    continue;
                }
                let mut trial = pb;
                trial.set_pair(a, b);
                let plain = decrypt(cipher, variant, rotors, rings, starts, reflector, &trial);
                let s = score(cipher, &plain, crib);
                if s > best_score + 1e-9 {
                    best_score = s;
                    pb = trial;
                    improved = true;
                    // A swap that helps once may help again as the board fills.
                    break 'outer;
                }
            }
        }
        if !improved {
            break;
        }
    }
    (pb, best_score)
}

/// How many of step 1's best bases are carried forward into plugboard
/// recovery. The true setting is not always the top IoC without a plugboard
/// (cables raise the score), so the top few are kept and the *final* score —
/// after the plugboard — decides between them.
const BASE_CANDIDATES: usize = 12;

/// Run the whole search. Returns the best settings found, decrypt complete.
pub fn solve(ciphertext: &str, params: &SolveParams) -> Solution {
    let cipher = letters(ciphertext);
    let crib = letters(&params.crib);
    let count = params.variant.wheel_count();

    // Which standard wheels are available for the moving slots, and which
    // thin wheels for the M4's fourth slot.
    let standard: Vec<usize> = (0..ROTORS.len()).filter(|&i| !ROTORS[i].thin).collect();
    let thin: Vec<usize> = (0..ROTORS.len()).filter(|&i| ROTORS[i].thin).collect();

    // --- Step 1: rotor order and start positions, rings A-A-A, no plugboard.
    // Collect every (order, start) with its base IoC and keep the best few,
    // because the eventual winner is chosen on the *plugboard-corrected* score
    // and the true base need not be the highest without cables.
    let mut bases: Vec<Solution> = Vec::new();
    let mut orders = Vec::new();
    for order in permutations(&standard, count) {
        if params.variant == Variant::M4 {
            let choices: Vec<usize> =
                params.m4_thin.map(|t| vec![t]).unwrap_or_else(|| thin.clone());
            for t in choices {
                let mut full = vec![t];
                full.extend_from_slice(&order[..count - 1]);
                orders.push(full);
            }
        } else {
            orders.push(order);
        }
    }
    for full in &orders {
        search_starts(&cipher, params.variant, full, &crib, &mut bases);
    }
    bases.sort_by(|a, b| b.score.total_cmp(&a.score));
    bases.truncate(BASE_CANDIDATES);

    // --- Step 2 (optional): refine the ring settings on each base.
    if params.search_rings {
        for sol in bases.iter_mut() {
            let rings = sol.rings.clone();
            for i in 0..rings.len() {
                for r in 0..26u8 {
                    let mut trial = rings.clone();
                    trial[i] = r;
                    let plain = decrypt(
                        &cipher,
                        params.variant,
                        &sol.rotors,
                        &trial,
                        &sol.starts,
                        sol.reflector,
                        &sol.plugboard,
                    );
                    let s = index_of_coincidence(&plain);
                    if s > sol.score {
                        sol.score = s;
                        sol.rings = trial;
                    }
                }
            }
        }
    }

    // --- Step 3: the plugboard on each surviving base, and keep the best
    // final result. This is where a cable-carrying message is separated from a
    // merely-promising base.
    let mut best: Option<Solution> = None;
    for mut sol in bases {
        // Blind plugboard recovery (no crib) is the Bombe's problem and is
        // deliberately not attempted: a greedy hill-climb on the IoC wanders
        // off the true base. The crib is what makes it tractable.
        if params.search_plugboard && !crib.is_empty() {
            let (pb, s) = recover_plugboard(
                &cipher,
                params.variant,
                &sol.rotors,
                &sol.rings,
                &sol.starts,
                sol.reflector,
                &crib,
                sol.plugboard,
            );
            sol.plugboard = pb;
            sol.score = s;
        } else {
            let plain = decrypt(
                &cipher,
                params.variant,
                &sol.rotors,
                &sol.rings,
                &sol.starts,
                sol.reflector,
                &sol.plugboard,
            );
            sol.score = score(&cipher, &plain, &crib);
        }
        if best.as_ref().is_none_or(|b| sol.score > b.score) {
            best = Some(sol);
        }
    }
    let mut sol = best.expect("at least one base always exists");

    let plain = decrypt(
        &cipher,
        params.variant,
        &sol.rotors,
        &sol.rings,
        &sol.starts,
        sol.reflector,
        &sol.plugboard,
    );
    sol.plaintext = plain.iter().map(|&c| letter(c)).collect();
    sol
}

/// Search all `26^count` start positions for one wheel order, pushing each
/// candidate into `out` with its base IoC. The caller keeps the best few.
fn search_starts(
    cipher: &[u8],
    variant: Variant,
    rotors: &[usize],
    crib: &[u8],
    out: &mut Vec<Solution>,
) {
    let count = rotors.len();
    let rings = vec![0u8; count];
    let mut starts = vec![0u8; count];
    let total = 26u32.pow(count as u32);
    for idx in 0..total {
        // Mixed-radix decode of the start positions.
        let mut x = idx;
        for i in (0..count).rev() {
            starts[i] = (x % 26) as u8;
            x /= 26;
        }
        let plain = decrypt(cipher, variant, rotors, &rings, &starts, 0, &Plugboard::new());
        let s = score(cipher, &plain, crib);
        out.push(Solution {
            rotors: rotors.to_vec(),
            rings: rings.clone(),
            starts: starts.clone(),
            reflector: 0,
            plugboard: Plugboard::new(),
            plaintext: String::new(),
            score: s,
        });
    }
}

/// All ordered selections of `k` distinct elements from `items`.
fn permutations(items: &[usize], k: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; items.len()];
    fn rec(
        items: &[usize],
        k: usize,
        current: &mut Vec<usize>,
        used: &mut [bool],
        out: &mut Vec<Vec<usize>>,
    ) {
        if current.len() == k {
            out.push(current.clone());
            return;
        }
        for i in 0..items.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            current.push(items[i]);
            rec(items, k, current, used, out);
            current.pop();
            used[i] = false;
        }
    }
    rec(items, k, &mut current, &mut used, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// IoC separates language from noise: a Julius Caesar quote scores near
    /// English, a shifted version near random only if it is short — so use a
    /// long, real sentence for the language end.
    #[test]
    fn ioc_knows_language_from_noise() {
        let prose = letters(
            "ITWASABRILLIANTAFTERNOONINLATESPRINGANDWINSTONBOULEVARDWASTAKING\
             ITSPLEASUREJOYOUSLYOUTOFDOORSASISITSWONTTHEMANYPARKSANDGARDENS",
        );
        let random = letters("QXJZVPWKMFBHTRYDNGCLAEOUISXZQPMFJRVKWTBHYNDGLEAOIUZSCX");
        assert!(index_of_coincidence(&prose) > 0.06, "prose should look like English");
        assert!(index_of_coincidence(&random) < 0.055, "noise should not");
    }

    /// The promise, end to end: encipher a known message under a hidden
    /// setting and recover it from the ciphertext alone. No plugboard here —
    /// blind recovery of the rotor order and starts is what this pins, and it
    /// is the case a listener solving a museum puzzle or a web ciphertext
    /// actually has.
    #[test]
    fn solves_a_known_message_without_a_plugboard() {
        let plain = "ITWASABRILLIANTAFTERNOONINLATESPRINGANDWINSTONBOULEVARDWAS\
                     TAKINGITSPLEASUREJOYOUSLYOUTOFDOORSASISITSWONTTHEMANY";
        let setting = vec![Wheel::new(1, 0, 7), Wheel::new(0, 0, 3), Wheel::new(2, 0, 22)];
        let mut e = Enigma::new(Variant::EnigmaI, setting, 0);
        let ct = e.encipher_text(plain);

        let sol = solve(&ct, &SolveParams::default());
        assert_eq!(sol.rotors, vec![1, 0, 2], "rotor order not recovered: {:?}", sol.rotors);
        assert!(
            sol.plaintext.contains("BRILLIANT") || sol.plaintext.contains("AFTERNOON"),
            "plaintext not recovered: {}",
            sol.plaintext
        );
    }

    /// A crib is the reliable path to a **plugboarded** message, and it is how
    /// the machine was really broken: tell the solver a word the message holds,
    /// and it recovers the rotors, the start and the cables together. The crib
    /// must not be on the first letter, because the first letter of the message
    /// is where the alignment is most ambiguous — a mid-message word is the
    /// honest test.
    #[test]
    fn a_crib_recovers_a_plugboarded_message() {
        let plain = "ITWASABRILLIANTAFTERNOONINLATESPRINGANDWINSTONBOULEVARD\
                     WASTAKINGITSPLEASUREJOYOUSLYOUTOFDOORSASISITSWONTTHEMANY";
        let setting = vec![Wheel::new(1, 0, 7), Wheel::new(0, 0, 3), Wheel::new(2, 0, 22)];
        let pb = Plugboard::parse("AB CD EF");
        let mut e = Enigma::new(Variant::EnigmaI, setting, 0).with_plugboard(pb);
        let ct = e.encipher_text(plain);

        let sol = solve(&ct, &SolveParams { crib: "PLEASURE".into(), ..SolveParams::default() });
        // Rotors, start and reflector are recovered exactly; the plaintext is
        // read through the cables, so the crib word and the readable run
        // around it are what this pins (the plugboard is recovered best-effort
        // and the cabled letters may still be wrong).
        assert_eq!(sol.rotors, vec![1, 0, 2], "rotor order not recovered: {:?}", sol.rotors);
        assert_eq!(sol.starts, vec![7, 3, 22], "start not recovered: {:?}", sol.starts);
        assert!(sol.plaintext.contains("PLEASURE"), "crib not recovered: {}", sol.plaintext);
    }
}
