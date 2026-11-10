//! LZHUF compression as the FBB binary forwarding protocols B, B1 and B2 use
//! it — LZSS sliding-window matching with an adaptive-Huffman entropy coder
//! over the top.
//!
//! # Provenance
//!
//! LZSS by Haruhiko Okumura, the adaptive Huffman coder by Haruyasu Yoshizaki,
//! translated to English by Kenji Rikitake (`LZHUF.C`, 1988). The authors
//! settled the licensing in 2001 on `comp.compression`: "use, distribute, and
//! modify this program freely", with Okumura stating explicitly that taking
//! `LZHUF.C` as GPL'd or LGPL'd poses no problem. The LZH patent
//! (US 4,906,991) has long since expired. This is an independent Rust port
//! written against the published algorithm, cross-checked for wire
//! compatibility against the fixtures in `la5nta/wl2k-go` (MIT).
//!
//! # The Winlink deviations
//!
//! Two things differ from stock `LZHUF.C`, and both are load-bearing:
//!
//! * **The ring buffer is 2048 bytes, not 4096.** Nothing in the stream says
//!   so; a 4096-byte window simply decodes to garbage. This is the single
//!   easiest way to get a plausible-looking but wrong implementation.
//! * **B2 prepends a CRC-16/XMODEM**, and it covers the four-byte length field
//!   *and* the compressed payload — not the payload alone, which the usual
//!   phrasing ("a checksum of the compressed data") can be read to suggest.
//!   Reference implementations write it in the augmented form, which looks
//!   like a different algorithm and is not; see [`crc16`] for the trap.
//!
//! So a B2 block is `[crc16 LE u16][uncompressed length LE i32][payload]`,
//! and [`compress`] / [`decompress`] speak exactly that.

/// Ring-buffer size. **2048 for Winlink**, against 4096 in stock `LZHUF.C`.
const N: usize = 2048;
/// Lookahead-buffer size: the longest match that can be coded.
const F: usize = 60;
/// Matches this short or shorter are cheaper to emit as literals.
const THRESHOLD: usize = 2;
/// Sentinel "no such node" for the binary search tree.
const NIL: usize = N;

/// Alphabet size: 256 literals, minus the match lengths that never occur,
/// plus one code per codeable match length. 314.
const N_CHAR: usize = 256 - THRESHOLD + F;
/// Huffman table size — a full binary tree over `N_CHAR` leaves. 627.
const T: usize = N_CHAR * 2 - 1;
/// Index of the tree root. 626.
const R: usize = T - 1;
/// Halve every frequency once the root reaches this, so the counts cannot
/// overflow and the code adapts to recent input rather than all of it.
const MAX_FREQ: u32 = 0x8000;

/// Upper six bits of a match position, encoded (`p_code` / `p_len` in the
/// original). The position's low six bits ride verbatim.
#[rustfmt::skip]
const P_CODE: [u8; 64] = [
    0x00, 0x20, 0x30, 0x40, 0x50, 0x58, 0x60, 0x68,
    0x70, 0x78, 0x80, 0x88, 0x90, 0x94, 0x98, 0x9C,
    0xA0, 0xA4, 0xA8, 0xAC, 0xB0, 0xB4, 0xB8, 0xBC,
    0xC0, 0xC2, 0xC4, 0xC6, 0xC8, 0xCA, 0xCC, 0xCE,
    0xD0, 0xD2, 0xD4, 0xD6, 0xD8, 0xDA, 0xDC, 0xDE,
    0xE0, 0xE2, 0xE4, 0xE6, 0xE8, 0xEA, 0xEC, 0xEE,
    0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7,
    0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF,
];

#[rustfmt::skip]
const P_LEN: [u8; 64] = [
    0x03, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x06, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
];

/// The decode side of [`P_CODE`]: one byte of lookahead picks the position's
/// upper six bits (`d_code`) and says how many bits it cost (`d_len`).
#[rustfmt::skip]
const D_CODE: [u8; 256] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
    0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09,
    0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
    0x0B, 0x0B, 0x0B, 0x0B, 0x0B, 0x0B, 0x0B, 0x0B,
    0x0C, 0x0C, 0x0C, 0x0C, 0x0D, 0x0D, 0x0D, 0x0D,
    0x0E, 0x0E, 0x0E, 0x0E, 0x0F, 0x0F, 0x0F, 0x0F,
    0x10, 0x10, 0x10, 0x10, 0x11, 0x11, 0x11, 0x11,
    0x12, 0x12, 0x12, 0x12, 0x13, 0x13, 0x13, 0x13,
    0x14, 0x14, 0x14, 0x14, 0x15, 0x15, 0x15, 0x15,
    0x16, 0x16, 0x16, 0x16, 0x17, 0x17, 0x17, 0x17,
    0x18, 0x18, 0x19, 0x19, 0x1A, 0x1A, 0x1B, 0x1B,
    0x1C, 0x1C, 0x1D, 0x1D, 0x1E, 0x1E, 0x1F, 0x1F,
    0x20, 0x20, 0x21, 0x21, 0x22, 0x22, 0x23, 0x23,
    0x24, 0x24, 0x25, 0x25, 0x26, 0x26, 0x27, 0x27,
    0x28, 0x28, 0x29, 0x29, 0x2A, 0x2A, 0x2B, 0x2B,
    0x2C, 0x2C, 0x2D, 0x2D, 0x2E, 0x2E, 0x2F, 0x2F,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
];

#[rustfmt::skip]
const D_LEN: [u8; 256] = [
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
];

/// What can go wrong reading somebody else's compressed block.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum LzhufError {
    #[error("lzhuf block truncated: {0} bytes is too short to hold a header")]
    Truncated(usize),
    #[error("lzhuf checksum mismatch: header says {want:#06x}, computed {got:#06x}")]
    Checksum { want: u16, got: u16 },
    #[error("lzhuf length mismatch: header says {want} bytes, decoded {got}")]
    Length { want: usize, got: usize },
    #[error("lzhuf header declares a negative length ({0})")]
    NegativeLength(i32),
    #[error("lzhuf block declares {0} bytes, over the {1} byte limit")]
    TooLarge(usize, usize),
}

/// Refuse to allocate for a header that claims more than this. A Winlink
/// message is capped far below it; the point is that the length arrives over a
/// socket and is otherwise a `Vec::with_capacity` argument chosen by a peer.
pub const MAX_DECOMPRESSED: usize = 32 * 1024 * 1024;

/// The CCITT table: the CRC of each byte value in the high position, poly
/// 0x1021. Shared with textbook CRC-16/XMODEM — it is only the recurrence in
/// [`crc16_update`] that differs.
const CRC_TABLE: [u16; 256] = {
    let mut table = [0u16; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut c = (i as u16) << 8;
        let mut bit = 0;
        while bit < 8 {
            c = if c & 0x8000 != 0 { (c << 1) ^ 0x1021 } else { c << 1 };
            bit += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
};

/// The B2 header checksum over `parts`: plain CRC-16/XMODEM.
///
/// The reference implementations write this the *augmented* way — a recurrence
/// that indexes the table on the register alone and XORs the data byte in
/// afterwards, then runs two zero bytes through at the end to flush it:
///
/// ```text
/// sum = ((sum << 8) & 0xff00) ^ TABLE[(sum >> 8) & 0xff] ^ byte   // then feed 0x00 0x00
/// ```
///
/// That is algebraically the same function as the standard form below, and the
/// tests assert the two agree on the reference block. The trap is mixing them:
/// running the *standard* recurrence and still appending the two zero bytes
/// counts them twice and yields a checksum no CMS will accept. The failure
/// then looks like a corrupt message rather than a bad checksum, which is why
/// the reference fixture in the tests is load-bearing.
fn crc16(parts: &[&[u8]]) -> u16 {
    let mut sum = 0u16;
    for part in parts {
        for &byte in *part {
            sum = (sum << 8) ^ CRC_TABLE[(((sum >> 8) ^ byte as u16) & 0xff) as usize];
        }
    }
    sum
}

/// The adaptive-Huffman tree, plus the LZSS match tree. One instance encodes or
/// decodes exactly one block: the tables adapt as they go, so an encoder and
/// its decoder only agree if both start fresh.
struct Lzhuf {
    freq: [u32; T + 1],
    /// Parent links. The tail `[T..T + N_CHAR]` holds leaf positions rather
    /// than parents, which is how a symbol finds its way into the tree.
    prnt: [u16; T + N_CHAR],
    son: [u16; T],

    dad: [u16; N + 1],
    lson: [u16; N + 1],
    rson: [u16; N + 257],

    text_buf: [u8; N + F - 1],
    match_length: usize,
    match_position: usize,
}

impl Lzhuf {
    fn new() -> Box<Self> {
        // Boxed and zeroed first: these tables are ~14 kB and there is no
        // reason to build them on the stack.
        let mut z = Box::new(Lzhuf {
            freq: [0; T + 1],
            prnt: [0; T + N_CHAR],
            son: [0; T],
            dad: [0; N + 1],
            lson: [0; N + 1],
            rson: [0; N + 257],
            text_buf: [0; N + F - 1],
            match_length: 0,
            match_position: 0,
        });

        for i in 0..N_CHAR {
            z.freq[i] = 1;
            z.son[i] = (i + T) as u16;
            z.prnt[i + T] = i as u16;
        }
        let mut i = 0usize;
        let mut j = N_CHAR;
        while j <= R {
            z.freq[j] = z.freq[i] + z.freq[i + 1];
            z.son[j] = i as u16;
            z.prnt[i] = j as u16;
            z.prnt[i + 1] = j as u16;
            i += 2;
            j += 1;
        }
        z.freq[T] = 0xffff;
        z.prnt[R] = 0;
        z
    }

    /// Reset the LZSS search tree. Encode-side only; the decoder never searches.
    fn init_tree(&mut self) {
        for i in N + 1..=N + 256 {
            self.rson[i] = NIL as u16;
        }
        for i in 0..N {
            self.dad[i] = NIL as u16;
        }
    }

    /// Halve every frequency and rebuild, so the counts stay bounded and the
    /// code tracks recent input.
    fn reconst(&mut self) {
        let mut j = 0usize;
        for i in 0..T {
            if self.son[i] as usize >= T {
                // Round up: the original's `(freq + 1) / 2`. Halving to zero
                // would drop a symbol out of the tree entirely.
                self.freq[j] = self.freq[i].div_ceil(2);
                self.son[j] = self.son[i];
                j += 1;
            }
        }

        // Rebuild the interior nodes, keeping `freq` sorted by insertion.
        let mut i = 0usize;
        let mut j = N_CHAR;
        while j < T {
            let k = i + 1;
            let f = self.freq[i] + self.freq[k];
            self.freq[j] = f;

            // Walk down to the insertion point, then shift right by one.
            let mut k = j;
            while k > 0 && f < self.freq[k - 1] {
                k -= 1;
            }
            let count = j - k;
            self.freq.copy_within(k..k + count, k + 1);
            self.freq[k] = f;
            self.son.copy_within(k..k + count, k + 1);
            self.son[k] = i as u16;

            i += 2;
            j += 1;
        }

        for i in 0..T {
            let k = self.son[i] as usize;
            if k >= T {
                self.prnt[k] = i as u16;
            } else {
                self.prnt[k] = i as u16;
                self.prnt[k + 1] = i as u16;
            }
        }
    }

    /// Count one occurrence of symbol `c` and re-sort the tree around it.
    fn update(&mut self, c: usize) {
        if self.freq[R] == MAX_FREQ {
            self.reconst();
        }

        let mut c = self.prnt[c + T] as usize;
        loop {
            self.freq[c] += 1;
            let k = self.freq[c];

            // If this node has overtaken its right-hand neighbour, swap it up
            // to where it belongs. `freq[T] = 0xffff` stops the scan.
            if k > self.freq[c + 1] {
                let mut l = c + 1;
                while k > self.freq[l + 1] {
                    l += 1;
                }

                self.freq[c] = self.freq[l];
                self.freq[l] = k;

                let i = self.son[c] as usize;
                self.prnt[i] = l as u16;
                if i < T {
                    self.prnt[i + 1] = l as u16;
                }

                let j = self.son[l] as usize;
                self.son[l] = i as u16;

                self.prnt[j] = c as u16;
                if j < T {
                    self.prnt[j + 1] = c as u16;
                }
                self.son[c] = j as u16;

                c = l;
            }

            c = self.prnt[c] as usize;
            if c == 0 {
                break;
            }
        }
    }

    fn delete_node(&mut self, p: usize) {
        if self.dad[p] as usize == NIL {
            return;
        }

        let q;
        if self.rson[p] as usize == NIL {
            q = self.lson[p] as usize;
        } else if self.lson[p] as usize == NIL {
            q = self.rson[p] as usize;
        } else {
            let mut t = self.lson[p] as usize;
            if self.rson[t] as usize != NIL {
                while self.rson[t] as usize != NIL {
                    t = self.rson[t] as usize;
                }
                let dad_t = self.dad[t] as usize;
                self.rson[dad_t] = self.lson[t];
                let lson_t = self.lson[t] as usize;
                self.dad[lson_t] = self.dad[t];
                self.lson[t] = self.lson[p];
                let lson_p = self.lson[p] as usize;
                self.dad[lson_p] = t as u16;
            }
            self.rson[t] = self.rson[p];
            let rson_p = self.rson[p] as usize;
            self.dad[rson_p] = t as u16;
            q = t;
        }

        self.dad[q] = self.dad[p];
        let dad_p = self.dad[p] as usize;
        if self.rson[dad_p] as usize == p {
            self.rson[dad_p] = q as u16;
        } else {
            self.lson[dad_p] = q as u16;
        }
        self.dad[p] = NIL as u16;
    }

    /// Insert the string at `r` into the search tree, leaving the longest match
    /// found in `match_length` / `match_position`.
    fn insert_node(&mut self, r: usize) {
        let mut cmp: i32 = 1;
        let mut p = N + 1 + self.text_buf[r] as usize;
        self.rson[r] = NIL as u16;
        self.lson[r] = NIL as u16;
        self.match_length = 0;

        loop {
            if cmp >= 0 {
                if self.rson[p] as usize != NIL {
                    p = self.rson[p] as usize;
                } else {
                    self.rson[p] = r as u16;
                    self.dad[r] = p as u16;
                    return;
                }
            } else if self.lson[p] as usize != NIL {
                p = self.lson[p] as usize;
            } else {
                self.lson[p] = r as u16;
                self.dad[r] = p as u16;
                return;
            }

            let mut i = 1usize;
            while i < F {
                cmp = self.text_buf[r + i] as i32 - self.text_buf[p + i] as i32;
                if cmp != 0 {
                    break;
                }
                i += 1;
            }

            if i > THRESHOLD {
                if i > self.match_length {
                    self.match_position = ((r.wrapping_sub(p)) & (N - 1)) - 1;
                    self.match_length = i;
                    if self.match_length >= F {
                        break;
                    }
                }
                if i == self.match_length {
                    let c = ((r.wrapping_sub(p)) & (N - 1)) - 1;
                    if c < self.match_position {
                        self.match_position = c;
                    }
                }
            }
        }

        self.dad[r] = self.dad[p];
        self.lson[r] = self.lson[p];
        self.rson[r] = self.rson[p];
        let lson_p = self.lson[p] as usize;
        self.dad[lson_p] = r as u16;
        let rson_p = self.rson[p] as usize;
        self.dad[rson_p] = r as u16;
        let dad_p = self.dad[p] as usize;
        if self.rson[dad_p] as usize == p {
            self.rson[dad_p] = r as u16;
        } else {
            self.lson[dad_p] = r as u16;
        }
        self.dad[p] = NIL as u16;
    }
}

// --- encode ---------------------------------------------------------------

struct BitWriter {
    out: Vec<u8>,
    buf: u32,
    len: u8,
}

impl BitWriter {
    fn new() -> Self {
        BitWriter { out: Vec::new(), buf: 0, len: 0 }
    }

    /// Emit the top `l` bits of `c`, MSB first.
    fn put_code(&mut self, l: u8, c: u32) {
        self.buf |= c >> self.len;
        self.len += l;
        if self.len < 8 {
            return;
        }

        self.out.push((self.buf >> 8) as u8);
        self.len -= 8;

        if self.len >= 8 {
            self.out.push(self.buf as u8);
            self.len -= 8;
            self.buf = c << (l - self.len);
        } else {
            self.buf <<= 8;
        }
        self.buf &= 0xffff;
    }

    fn finish(mut self) -> Vec<u8> {
        if self.len > 0 {
            self.out.push((self.buf >> 8) as u8);
        }
        self.out
    }
}

/// Compress `input` into a B2 block: `[crc16][length][payload]`.
pub fn compress(input: &[u8]) -> Vec<u8> {
    let payload = encode_raw(input);

    let mut out = Vec::with_capacity(payload.len() + 6);
    let len_bytes = (input.len() as i32).to_le_bytes();
    let sum = crc16(&[&len_bytes, &payload]);
    out.extend_from_slice(&sum.to_le_bytes());
    out.extend_from_slice(&len_bytes);
    out.extend_from_slice(&payload);
    out
}

/// The LZHUF bit stream alone, with no B2 header.
fn encode_raw(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut z = Lzhuf::new();
    z.init_tree();
    let mut w = BitWriter::new();

    let mut s = 0usize;
    let mut r = N - F;
    // The window starts as spaces, so an early match has something to point
    // at. Both ends do this identically or the first back-reference diverges.
    for i in 0..r {
        z.text_buf[i] = b' ';
    }

    // Fill the lookahead buffer, then register every suffix of it.
    let mut len = 0usize;
    while len < F && len < input.len() {
        z.text_buf[r + len] = input[len];
        len += 1;
    }
    let mut next = len;
    for i in 1..=F {
        z.insert_node(r - i);
    }
    z.insert_node(r);

    loop {
        if z.match_length > len {
            z.match_length = len;
        }
        if z.match_length <= THRESHOLD {
            z.match_length = 1;
            let literal = z.text_buf[r] as usize;
            encode_char(&mut z, &mut w, literal);
        } else {
            let sym = 255 - THRESHOLD + z.match_length;
            encode_char(&mut z, &mut w, sym);
            let pos = z.match_position;
            encode_position(&mut w, pos);
        }

        let last_match_length = z.match_length;
        let mut i = 0usize;
        while i < last_match_length && next < input.len() {
            let c = input[next];
            next += 1;
            z.delete_node(s);
            z.text_buf[s] = c;
            // The first F-1 bytes are mirrored past the end so a match can run
            // off the top of the ring and still read contiguous bytes.
            if s < F - 1 {
                z.text_buf[s + N] = c;
            }
            s = (s + 1) & (N - 1);
            r = (r + 1) & (N - 1);
            z.insert_node(r);
            i += 1;
        }
        while i < last_match_length {
            i += 1;
            z.delete_node(s);
            s = (s + 1) & (N - 1);
            r = (r + 1) & (N - 1);
            len -= 1;
            if len > 0 {
                z.insert_node(r);
            }
        }

        if len == 0 {
            break;
        }
    }

    w.finish()
}

fn encode_char(z: &mut Lzhuf, w: &mut BitWriter, c: usize) {
    let mut i: u32 = 0;
    let mut j: u8 = 0;
    let mut k = z.prnt[c + T] as usize;

    // Walk leaf to root, accumulating the path as bits (an odd node index is
    // the right-hand child, which is a 1).
    loop {
        i >>= 1;
        if k & 1 != 0 {
            i += 0x8000;
        }
        j += 1;
        k = z.prnt[k] as usize;
        if k == R {
            break;
        }
    }
    w.put_code(j, i);
    z.update(c);
}

fn encode_position(w: &mut BitWriter, c: usize) {
    let i = c >> 6;
    w.put_code(P_LEN[i], (P_CODE[i] as u32) << 8);
    w.put_code(6, ((c & 0x3f) as u32) << 10);
}

// --- decode ---------------------------------------------------------------

struct BitReader<'a> {
    src: &'a [u8],
    pos: usize,
    buf: u16,
    len: u8,
}

impl<'a> BitReader<'a> {
    fn new(src: &'a [u8]) -> Self {
        BitReader { src, pos: 0, buf: 0, len: 0 }
    }

    /// Past the end reads as zero bits, matching the reference decoder — a
    /// truncated block yields short output rather than an error here, and the
    /// length check in [`decompress`] is what catches it.
    fn fill(&mut self) {
        while self.len <= 8 {
            let byte = self.src.get(self.pos).copied().unwrap_or(0);
            self.pos += 1;
            self.buf |= (byte as u16) << (8 - self.len);
            self.len += 8;
        }
    }

    fn bit(&mut self) -> u16 {
        self.fill();
        let i = self.buf;
        self.buf <<= 1;
        self.len -= 1;
        (i & 0x8000) >> 15
    }

    fn byte(&mut self) -> u16 {
        self.fill();
        let i = self.buf;
        self.buf <<= 8;
        self.len -= 8;
        (i & 0xff00) >> 8
    }
}

/// Decompress a B2 block written by [`compress`], verifying both the checksum
/// and the declared length.
pub fn decompress(input: &[u8]) -> Result<Vec<u8>, LzhufError> {
    if input.len() < 6 {
        return Err(LzhufError::Truncated(input.len()));
    }

    let want_crc = u16::from_le_bytes([input[0], input[1]]);
    let got_crc = crc16(&[&input[2..]]);
    if want_crc != got_crc {
        return Err(LzhufError::Checksum { want: want_crc, got: got_crc });
    }

    let declared = i32::from_le_bytes([input[2], input[3], input[4], input[5]]);
    if declared < 0 {
        return Err(LzhufError::NegativeLength(declared));
    }
    let declared = declared as usize;
    if declared > MAX_DECOMPRESSED {
        return Err(LzhufError::TooLarge(declared, MAX_DECOMPRESSED));
    }

    let out = decode_raw(&input[6..], declared);
    if out.len() != declared {
        return Err(LzhufError::Length { want: declared, got: out.len() });
    }
    Ok(out)
}

/// The LZHUF bit stream alone, stopping at `size` bytes of output.
fn decode_raw(payload: &[u8], size: usize) -> Vec<u8> {
    if size == 0 {
        return Vec::new();
    }

    let mut z = Lzhuf::new();
    let mut br = BitReader::new(payload);
    let mut out = Vec::with_capacity(size);

    let mut r = N - F;
    for i in 0..r {
        z.text_buf[i] = b' ';
    }

    // Every pass through this loop pushes at least one byte — a literal is one
    // and the shortest codeable match is three — so the length bound alone
    // terminates it, even on a corrupt block whose bit stream has run out and
    // is reading zero padding.
    while out.len() < size {
        let c = decode_char(&mut z, &mut br) as usize;
        if c < 256 {
            out.push(c as u8);
            z.text_buf[r] = c as u8;
            r = (r + 1) & (N - 1);
        } else {
            let i = (r.wrapping_sub(decode_position(&mut br)).wrapping_sub(1)) & (N - 1);
            let j = c - 255 + THRESHOLD;
            for k in 0..j {
                // A match may legally run past the declared length on the last
                // symbol; the caller's length check wants the exact count, so
                // stop here rather than trimming afterwards.
                if out.len() >= size {
                    break;
                }
                let b = z.text_buf[(i + k) & (N - 1)];
                out.push(b);
                z.text_buf[r] = b;
                r = (r + 1) & (N - 1);
            }
        }
    }

    out
}

fn decode_char(z: &mut Lzhuf, br: &mut BitReader<'_>) -> u16 {
    let mut c = z.son[R] as usize;
    // Root to leaf: 0 takes the smaller child, 1 the bigger.
    while c < T {
        c += br.bit() as usize;
        c = z.son[c] as usize;
    }
    c -= T;
    z.update(c);
    c as u16
}

fn decode_position(br: &mut BitReader<'_>) -> usize {
    let mut i = br.byte() as usize;
    let c = (D_CODE[i] as usize) << 6;
    let mut j = D_LEN[i] as usize;

    j -= 2;
    while j > 0 {
        i = (i << 1) + br.bit() as usize;
        j -= 1;
    }
    c | (i & 0x3f)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The reference pair from `la5nta/wl2k-go`'s test data — a real B2 block
    /// produced by a known-good implementation. Decoding it bit-exactly is the
    /// only thing that proves wire compatibility; a round-trip against
    /// ourselves would pass just as happily with the wrong window size.
    const GETTYSBURG_TXT: &[u8] = include_bytes!("../testdata/gettysburg.txt");
    const GETTYSBURG_LZH: &[u8] = include_bytes!("../testdata/gettysburg.txt.lzh");

    /// A second reference pair, large enough that the coder passes `MAX_FREQ`
    /// and calls [`Lzhuf::reconst`]. Nothing under ~32k symbols reaches that
    /// path, so without this fixture the trickiest function in the file — the
    /// insertion-sorted tree rebuild — would be entirely unexercised, and a
    /// bug in it would be symmetric and survive any round-trip test.
    const E_TXT: &[u8] = include_bytes!("../testdata/e.txt");
    const E_LZH: &[u8] = include_bytes!("../testdata/e.txt.lzh");

    #[test]
    fn decodes_reference_block() {
        let got = decompress(GETTYSBURG_LZH).expect("reference block should decode");
        assert_eq!(got, GETTYSBURG_TXT);
    }

    /// The augmented recurrence the FBB and JNOS sources are written in: index
    /// on the register alone, XOR the byte in afterwards, and flush with two
    /// zero bytes at the end.
    fn crc16_augmented(data: &[u8]) -> u16 {
        let mut sum = 0u16;
        for &byte in data.iter().chain(&[0, 0]) {
            sum = ((sum << 8) & 0xff00) ^ CRC_TABLE[((sum >> 8) & 0xff) as usize] ^ byte as u16;
        }
        sum
    }

    #[test]
    fn crc_table_is_ccitt() {
        assert_eq!(CRC_TABLE[1], 0x1021);
        // The catalogue check value for CRC-16/XMODEM.
        assert_eq!(crc16(&[b"123456789"]), 0x31c3);
    }

    #[test]
    fn crc_matches_the_augmented_form() {
        // The two formulations agree, on the reference block and in general.
        // Pinned because the standard form is what is implemented and the
        // augmented one is what every reference you will read looks like.
        let body = &GETTYSBURG_LZH[2..];
        assert_eq!(crc16(&[body]), 0xb632, "must match the reference block's header");
        assert_eq!(crc16_augmented(body), 0xb632);
        for case in [&b""[..], &b"x"[..], &b"the quick brown fox"[..], GETTYSBURG_TXT] {
            assert_eq!(crc16(&[case]), crc16_augmented(case));
        }
    }

    #[test]
    fn decodes_reference_block_across_reconst() {
        let got = decompress(E_LZH).expect("large reference block should decode");
        assert_eq!(got.len(), E_TXT.len());
        assert!(got == E_TXT, "decoded bytes differ from the reference plaintext");
    }

    #[test]
    fn round_trips_across_reconst() {
        let packed = compress(E_TXT);
        assert_eq!(decompress(&packed).unwrap(), E_TXT);
    }

    #[test]
    fn reference_header_is_as_documented() {
        assert_eq!(u16::from_le_bytes([GETTYSBURG_LZH[0], GETTYSBURG_LZH[1]]), 0xb632);
        let size = i32::from_le_bytes([
            GETTYSBURG_LZH[2],
            GETTYSBURG_LZH[3],
            GETTYSBURG_LZH[4],
            GETTYSBURG_LZH[5],
        ]);
        assert_eq!(size as usize, GETTYSBURG_TXT.len());
    }

    #[test]
    fn round_trips() {
        for case in
            [&b""[..], &b"a"[..], &b"hello hello hello hello"[..], GETTYSBURG_TXT, &[0u8; 5000][..]]
        {
            let packed = compress(case);
            let got = decompress(&packed).expect("our own block should decode");
            assert_eq!(got, case, "round trip failed for {} bytes", case.len());
        }
    }

    #[test]
    fn round_trips_binary() {
        // Attachments are arbitrary bytes, not text.
        let data: Vec<u8> =
            (0..9000u32).map(|i| (i.wrapping_mul(2654435761) >> 24) as u8).collect();
        let packed = compress(&data);
        assert_eq!(decompress(&packed).unwrap(), data);
    }

    #[test]
    fn rejects_corrupt_checksum() {
        let mut packed = compress(b"the quick brown fox");
        packed[0] ^= 0xff;
        assert!(matches!(decompress(&packed), Err(LzhufError::Checksum { .. })));
    }

    #[test]
    fn rejects_short_block() {
        assert_eq!(decompress(&[1, 2, 3]), Err(LzhufError::Truncated(3)));
    }

    #[test]
    fn rejects_absurd_declared_length() {
        // A peer-supplied length is a Vec::with_capacity argument; it has to be
        // bounded before it is believed.
        let mut packed = compress(b"short");
        packed[2..6].copy_from_slice(&(MAX_DECOMPRESSED as i32 + 1).to_le_bytes());
        let fixed = crc16(&[&packed[2..]]).to_le_bytes();
        packed[0..2].copy_from_slice(&fixed);
        assert!(matches!(decompress(&packed), Err(LzhufError::TooLarge(..))));
    }
}
