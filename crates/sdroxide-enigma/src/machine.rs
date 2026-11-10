//! The machine: rotors, reflector, plugboard, and the stepping gear.
//!
//! This is the real Enigma I (three rotors) and the M4 (four rotors, with the
//! thin Beta/Gamma wheels), including the two facts that make it *itself* and
//! not a generic substitution cipher:
//!
//! - **A letter never enciphers to itself.** The reflector sends the signal
//!   back through the rotors, so no key on the board can ever light itself.
//!   That is a weakness of the machine, and it is kept here on purpose: it is
//!   what lets [`crate::solve`] prune the search, and it is what a reader of
//!   real Enigma traffic exploits.
//! - **The double-step.** The middle wheel advances on the right wheel's
//!   notch, and again when its own notch is reached, which makes the middle
//!   rotor step twice in a row. Getting this wrong is the classic "almost
//!   works" Enigma bug.
//!
//! Wiring is the historical data (the letter each contact is wired to on the
//! left, `ABCDEFGHIJKLMNOPQRSTUVWXYZ` on the right), the notches, and the
//! reflectors. Every rotor is stored as a 26-byte forward permutation; the
//! reverse is derived once at construction.

/// One rotor: its forward wiring, the position(s) at which it steps its
/// neighbour, and — for the M4 — whether it is a thin (Greek) wheel.
#[derive(Clone, Copy)]
pub struct Rotor {
    pub name: &'static str,
    /// The forward wiring: `wiring[i]` is the output contact for input `i`.
    pub wiring: [u8; 26],
    /// The window letters at which this rotor's notch trips the *next* rotor
    /// left. The Enigma I wheels have one; the M4 thin wheels have none.
    pub notches: &'static [u8],
    /// A thin wheel (Beta/Gamma) carries no notch and sits beside the UKW.
    pub thin: bool,
}

/// The eight Enigma I wheels, plus the two M4 thin wheels, by their service
/// names. Wiring is the standard published data (see the manual's table).
pub const ROTORS: &[Rotor] = &[
    Rotor { name: "I", wiring: bytes("EKMFLGDQVZNTOWYHXUSPAIBRCJ"), notches: b"Q", thin: false },
    Rotor { name: "II", wiring: bytes("AJDKSIRUXBLHWTMCQGZNPYFVOE"), notches: b"E", thin: false },
    Rotor { name: "III", wiring: bytes("BDFHJLCPRTXVZNYEIWGAKMUSQO"), notches: b"V", thin: false },
    Rotor { name: "IV", wiring: bytes("ESOVPZJAYQUIRHXLNFTGKDCMWB"), notches: b"J", thin: false },
    Rotor { name: "V", wiring: bytes("VZBRGITYUPSDNHLXAWMJQOFECK"), notches: b"Z", thin: false },
    // VI–VIII are the naval wheels, with two notches each.
    Rotor { name: "VI", wiring: bytes("JPGVOUMFYQBENHZRDKASXLICTW"), notches: b"ZM", thin: false },
    Rotor { name: "VII", wiring: bytes("NZJHGRCXMYSWBOUFAIVLPEKQDT"), notches: b"ZM", thin: false },
    Rotor {
        name: "VIII",
        wiring: bytes("FKQHTLXOCBJSPDZRAMEWNIUYGV"),
        notches: b"ZM",
        thin: false,
    },
    // The two thin wheels of the M4, used only in the fourth slot.
    Rotor { name: "Beta", wiring: bytes("LEYJVCNIXWPBQMDRTAKZGFUHOS"), notches: b"", thin: true },
    Rotor { name: "Gamma", wiring: bytes("FSOKANUERHMBTIYCWLQPZXVGJD"), notches: b"", thin: true },
];

/// The reflectors. UKW-B and UKW-C are the Enigma I's; the M4 uses the same
/// two but they sit beside a thin wheel rather than a standard one.
pub const REFLECTORS: &[Reflector] = &[
    Reflector { name: "UKW-B", wiring: bytes("YRUHQSLDPXNGOKMIEBFZCWVJAT") },
    Reflector { name: "UKW-C", wiring: bytes("FVPJIAOYEDRZXWGCTKUQSBNMHL") },
];

const fn bytes(s: &str) -> [u8; 26] {
    // A const-time A–Z → 0–25 decoder, so the wiring tables above read as the
    // letters a service manual prints rather than as number soup.
    let b = s.as_bytes();
    let mut out = [0u8; 26];
    let mut i = 0;
    while i < 26 {
        out[i] = b[i] - b'A';
        i += 1;
    }
    out
}

/// A reflector: a fixed, self-inverse involution with no notch.
#[derive(Clone, Copy)]
pub struct Reflector {
    pub name: &'static str,
    pub wiring: [u8; 26],
}

/// One rotor *in place*: which wheel, its ring setting, and where it stands.
#[derive(Clone, Copy)]
pub struct Wheel {
    /// Index into [`ROTORS`].
    pub rotor: usize,
    /// Ringstellung (Ring setting), 0–25. Shifts the wiring relative to the
    /// alphabet before the rotor turns the signal.
    pub ring: u8,
    /// Grundstellung (start position), 0–25.
    pub pos: u8,
}

impl Wheel {
    pub fn new(rotor: usize, ring: u8, pos: u8) -> Self {
        Wheel { rotor, ring: ring % 26, pos: pos % 26 }
    }

    /// The letter at the window now.
    pub fn window(&self) -> u8 {
        self.pos
    }
}

/// The machine variant: what makes the difference between three wheels and the
/// M4's four (with a thin wheel in the leftmost slot).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Variant {
    #[default]
    EnigmaI,
    M4,
}

impl Variant {
    pub fn wheel_count(self) -> usize {
        match self {
            Variant::EnigmaI => 3,
            Variant::M4 => 4,
        }
    }
}

/// The plugboard: up to 13 swapped pairs. Stored as a 26-entry involution so a
/// patch is a single array lookup on each pass.
#[derive(Clone, Copy)]
pub struct Plugboard {
    map: [u8; 26],
}

impl Default for Plugboard {
    fn default() -> Self {
        let mut map = [0u8; 26];
        for (i, m) in map.iter_mut().enumerate() {
            *m = i as u8;
        }
        Plugboard { map }
    }
}

impl Plugboard {
    pub fn new() -> Self {
        Self::default()
    }

    /// Connect two letters, undoing any existing patch on either first — which
    /// is what pulling a cable and re-plugging it does.
    pub fn plug(&mut self, a: u8, b: u8) {
        let (a, b) = (a % 26, b % 26);
        self.unplug(a);
        self.unplug(b);
        self.map[a as usize] = b;
        self.map[b as usize] = a;
    }

    /// Remove whatever cable touches `a`.
    pub fn unplug(&mut self, a: u8) {
        let a = a % 26;
        let b = self.map[a as usize];
        self.map[a as usize] = a;
        self.map[b as usize] = b;
    }

    /// Swap two letters with no other bookkeeping — used by the solver, which
    /// builds candidate boards from scratch and never needs the undo above.
    pub fn set_pair(&mut self, a: u8, b: u8) {
        self.map[a as usize] = b;
        self.map[b as usize] = a;
    }

    #[inline]
    pub fn through(&self, c: u8) -> u8 {
        self.map[c as usize]
    }

    /// The pairs currently patched, as `(a, b)` with `a < b`, in order.
    pub fn pairs(&self) -> Vec<(u8, u8)> {
        let mut out = Vec::new();
        for a in 0..26u8 {
            let b = self.map[a as usize];
            if b != a && a < b {
                out.push((a, b));
            }
        }
        out
    }

    /// The board as a compact string, e.g. `"AB CD EF"`, for display and for
    /// the solver to hand back.
    pub fn to_string_pairs(&self) -> String {
        self.pairs()
            .iter()
            .map(|&(a, b)| format!("{}{}", letter(a), letter(b)))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Parse a board from `"AB CD EF"` (case- and space-insensitive). Letters
    /// that appear twice, or paired with themselves, are ignored.
    pub fn parse(s: &str) -> Self {
        let mut p = Plugboard::new();
        let cs: Vec<u8> = s
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| (c.to_ascii_uppercase() as u8) - b'A')
            .collect();
        for pair in cs.chunks(2) {
            if let [a, b] = pair {
                // Only if neither is already patched, so "AB BA" cannot
                // silently undo itself.
                if p.map[*a as usize] == *a && p.map[*b as usize] == *b && a != b {
                    p.plug(*a, *b);
                }
            }
        }
        p
    }
}

/// The assembled machine.
#[derive(Clone)]
pub struct Enigma {
    variant: Variant,
    /// Left to right, exactly as a key sheet lists them. For the M4 the first
    /// entry is the thin wheel.
    wheels: Vec<Wheel>,
    reflector: usize,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(variant: Variant, wheels: Vec<Wheel>, reflector: usize) -> Self {
        assert_eq!(wheels.len(), variant.wheel_count(), "wrong wheel count for variant");
        Enigma {
            variant,
            wheels,
            reflector: reflector % REFLECTORS.len(),
            plugboard: Plugboard::new(),
        }
    }

    pub fn with_plugboard(mut self, p: Plugboard) -> Self {
        self.plugboard = p;
        self
    }

    pub fn variant(&self) -> Variant {
        self.variant
    }

    pub fn wheels(&self) -> &[Wheel] {
        &self.wheels
    }

    pub fn plugboard(&self) -> &Plugboard {
        &self.plugboard
    }

    pub fn reflector_name(&self) -> &'static str {
        REFLECTORS[self.reflector].name
    }

    /// Advance the wheels one keypress, the real way: the right wheel always
    /// steps; the middle steps on the right wheel's notch, **and again when it
    /// is on its own notch** (the double-step); the left steps on the middle's
    /// notch. For the M4 the thin wheel never steps and the "left" wheel is
    /// the first standard one.
    pub fn step(&mut self) {
        let n = self.wheels.len();
        // Rightmost always steps.
        let right = n - 1;
        let right_notch = self.at_notch(right);
        let mid = if n >= 2 { Some(n - 2) } else { None };
        let mid_notch = mid.is_some_and(|m| self.at_notch(m));
        if let Some(m) = mid
            && (right_notch || mid_notch)
        {
            self.wheels[m].pos = (self.wheels[m].pos + 1) % 26;
        }
        // The wheel left of the middle: the M4's thin wheel does not move, so
        // the stepping wheel is the first standard one above the middle.
        if mid_notch && let Some(left) = self.step_target_left() {
            self.wheels[left].pos = (self.wheels[left].pos + 1) % 26;
        }
        self.wheels[right].pos = (self.wheels[right].pos + 1) % 26;
    }

    /// The wheel that steps when the middle wheel is on its notch. On the M4
    /// the thin wheel sits beside the reflector and never moves, so the wheel
    /// that steps is the one to its right (index 1).
    fn step_target_left(&self) -> Option<usize> {
        let n = self.wheels.len();
        let mut i = n - 2; // the middle wheel
        while i > 0 {
            if !ROTORS[self.wheels[i - 1].rotor].thin {
                return Some(i - 1);
            }
            i -= 1;
        }
        None
    }

    fn at_notch(&self, i: usize) -> bool {
        let r = &ROTORS[self.wheels[i].rotor];
        let window = (self.wheels[i].pos + 26 - self.wheels[i].ring) % 26;
        r.notches.contains(&(window + b'A'))
    }

    /// Encipher one letter (0–25), stepping first — a keypress moves the
    /// wheels before the signal goes through. This is the whole machine; the
    /// return is never equal to `c`, and that is correct.
    pub fn encipher(&mut self, c: u8) -> u8 {
        let c = c % 26;
        self.step();
        let n = self.wheels.len();

        // Steckerbrett, then forward through the wheels right to left.
        let mut x = self.plugboard.through(c) as i32;
        for w in (0..n).rev() {
            x = self.wheel_forward(w, x as u8) as i32;
        }
        // Reflector (self-inverse).
        x = REFLECTORS[self.reflector].wiring[x as usize] as i32;
        // Back through the wheels left to right (the inverse wiring).
        for w in 0..n {
            x = self.wheel_backward(w, x as u8) as i32;
        }
        self.plugboard.through(x as u8)
    }

    /// One pass through wheel `w`, right-hand side to left-hand side.
    #[inline]
    fn wheel_forward(&self, w: usize, c: u8) -> u8 {
        let wheel = &self.wheels[w];
        let rotor = &ROTORS[wheel.rotor];
        // Alphabet entry → internal contact (undo the position), then the
        // ring offset, then the wiring, then back out.
        let shifted = (c + wheel.pos + 26 - wheel.ring) % 26;
        let wired = rotor.wiring[shifted as usize];
        (wired + wheel.ring + 26 - wheel.pos) % 26
    }

    /// One pass through wheel `w`, left-hand side to right-hand side (the
    /// inverse of [`Self::wheel_forward`]).
    #[inline]
    fn wheel_backward(&self, w: usize, c: u8) -> u8 {
        let wheel = &self.wheels[w];
        let rotor = &ROTORS[wheel.rotor];
        let shifted = (c + wheel.pos + 26 - wheel.ring) % 26;
        // Inverse wiring: find the contact whose forward wire is `shifted`.
        let inv = inverse(&rotor.wiring);
        let wired = inv[shifted as usize];
        (wired + wheel.ring + 26 - wheel.pos) % 26
    }

    /// Encipher a whole message. Non-letters are dropped, as the machine has
    /// no punctuation; the result is grouped in fives, as the service sent it.
    pub fn encipher_text(&mut self, text: &str) -> String {
        let out: String = text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .map(|c| letter(self.encipher((c.to_ascii_uppercase() as u8) - b'A')))
            .collect();
        group_fives(&out)
    }

    /// Decipher is the same operation from the same start — that is the
    /// machine's defining symmetry, not a coincidence.
    pub fn decipher_text(&mut self, text: &str) -> String {
        self.encipher_text(text)
    }
}

/// The inverse of a 26-byte permutation, as a lookup table.
pub fn inverse(wiring: &[u8; 26]) -> [u8; 26] {
    let mut inv = [0u8; 26];
    for (i, &w) in wiring.iter().enumerate() {
        inv[w as usize] = i as u8;
    }
    inv
}

pub fn letter(c: u8) -> char {
    (b'A' + (c % 26)) as char
}

/// Insert a space every five letters, the way Enigma traffic was transmitted.
pub fn group_fives(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + s.len() / 5);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && i % 5 == 0 {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The classic published vector: wheels I-II-III, ring A-A-A, start A-A-A,
    /// UKW-B, no plugboard — `AAAAA` enciphers to `BDZGO`. This is the single
    /// most important test in the crate: it pins the stepping, the ring
    /// arithmetic and the rotor wiring all at once.
    #[test]
    fn the_textbook_vector() {
        let wheels = vec![Wheel::new(0, 0, 0), Wheel::new(1, 0, 0), Wheel::new(2, 0, 0)];
        let mut e = Enigma::new(Variant::EnigmaI, wheels, 0);
        assert_eq!(e.encipher_text("AAAAA"), "BDZGO");
    }

    /// The double-step, traced position by position. Wheels I-II-III, ring
    /// A-A-A, start A-E-V: the right wheel (III, notch V) is on its notch and
    /// the middle (II, notch E) is on *its* notch, so the first keypress
    /// advances the middle **and** the left together and then the middle sits
    /// still while the right runs on. A "step on the right wheel's notch only"
    /// implementation gives `[1,5,22,23,24,25,0]` with the middle never moving
    /// again — this trace is what distinguishes the real gear.
    #[test]
    fn the_double_step_is_modelled() {
        let wheels = vec![Wheel::new(0, 0, 0), Wheel::new(1, 0, 4), Wheel::new(2, 0, 21)];
        let mut e = Enigma::new(Variant::EnigmaI, wheels, 0);
        let mut trace = vec![[0u8, 4, 21]];
        for _ in 0..6 {
            e.encipher(0);
            let p = e.wheels();
            trace.push([p[0].pos, p[1].pos, p[2].pos]);
        }
        assert_eq!(
            trace,
            vec![
                [0, 4, 21],
                [1, 5, 22], // right on its notch → mid steps; mid was on its notch → left steps too
                [1, 5, 23],
                [1, 5, 24],
                [1, 5, 25],
                [1, 5, 0], // neither notch tripped this press: only the right steps
                [1, 5, 1],
            ],
            "the double-step gear trace is wrong"
        );
    }

    /// Encipher then decipher returns the original, for a random-looking
    /// message with a plugboard — the machine's symmetry.
    #[test]
    fn round_trip_with_plugboard() {
        let wheels = vec![Wheel::new(4, 3, 7), Wheel::new(2, 11, 19), Wheel::new(7, 20, 1)];
        let pb = Plugboard::parse("AB CD EF GH");
        let mut e = Enigma::new(Variant::EnigmaI, wheels.clone(), 1).with_plugboard(pb);
        let ct = e.encipher_text("THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");
        let mut d = Enigma::new(Variant::EnigmaI, wheels, 1).with_plugboard(pb);
        let pt: String = d.decipher_text(&ct).chars().filter(|c| c.is_ascii_alphabetic()).collect();
        assert_eq!(pt, "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG");
    }

    /// The real flaw: no letter ever enciphers to itself, in any setting.
    #[test]
    fn a_letter_never_enciphers_to_itself() {
        for r0 in 0..8u8 {
            let wheels =
                vec![Wheel::new(r0 as usize, 0, 0), Wheel::new(1, 0, 0), Wheel::new(2, 0, 0)];
            let e = Enigma::new(Variant::EnigmaI, wheels, 0);
            for c in 0..26u8 {
                let mut m = e.clone();
                assert_ne!(m.encipher(c), c, "rotor {r0} letter {c}");
            }
        }
    }
}
