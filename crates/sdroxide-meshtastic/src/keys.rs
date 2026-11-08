// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// The 1-byte channel-hash dispatch (xorHash(name) XOR xorHash(psk)) and the
// simpleN PSK derivation match the upstream Meshtastic firmware at
// https://github.com/meshtastic/firmware (GPL-3.0-or-later).
//
// Multi-key dispatch: which keys may have encrypted a frame, by its channel
// hash byte.

use crate::protocol::{DEFAULT_PSK, PRESETS, channel_hash, simple_psk};

/// Most keys one set holds.
pub const MAX_ENTRIES: usize = 64;
/// Most keys that may share one channel hash.
pub const MAX_PER_HASH: usize = 7;

/// One channel key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEntry {
    pub channel_name: String,
    /// 0 bytes (no encryption), 16 (AES-128) or 32 (AES-256).
    pub psk: Vec<u8>,
    pub channel_hash: u8,
}

/// Why a key was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyError {
    /// The key is not 0, 16 or 32 bytes.
    BadLength(usize),
    /// The text is not `default`, `none`, `simpleN`, `hex:` or base64.
    BadSpec(String),
    /// The set is full, or too many keys share one channel hash.
    Full,
}

impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyError::BadLength(n) => write!(f, "a key is 0, 16 or 32 bytes, not {n}"),
            KeyError::BadSpec(s) => write!(f, "cannot read the key {s:?}"),
            KeyError::Full => write!(f, "too many keys"),
        }
    }
}

impl std::error::Error for KeyError {}

/// The set of channel keys a receiver tries.
#[derive(Debug, Clone, Default)]
pub struct KeySet {
    entries: Vec<KeyEntry>,
    /// `buckets[hash]` lists indices into `entries`.
    buckets: Vec<Vec<usize>>,
}

fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len() / 2).map(|i| u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok()).collect()
}

/// Base64 in either alphabet (`+/` or `-_`), padding optional.
fn b64_decode(s: &str) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut accum = 0u32;
    let mut bits = 0;
    for c in s.trim().trim_end_matches('=').chars() {
        let v = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 26,
            '0'..='9' => c as u32 - '0' as u32 + 52,
            '+' | '-' => 62,
            '/' | '_' => 63,
            _ => return None,
        };
        accum = (accum << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((accum >> bits) & 0xff) as u8);
        }
    }
    Some(out)
}

impl KeySet {
    pub fn new() -> Self {
        Self { entries: Vec::new(), buckets: vec![Vec::new(); 256] }
    }

    /// A set holding the stock default key under every preset channel name.
    pub fn with_defaults() -> Self {
        let mut k = Self::new();
        for p in &PRESETS {
            let _ = k.add(p.channel_name, &DEFAULT_PSK);
        }
        k
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[KeyEntry] {
        &self.entries
    }

    fn push(&mut self, entry: KeyEntry) -> Result<(), KeyError> {
        if self.buckets.is_empty() {
            self.buckets = vec![Vec::new(); 256];
        }
        if self.entries.len() >= MAX_ENTRIES
            || self.buckets[entry.channel_hash as usize].len() >= MAX_PER_HASH
        {
            return Err(KeyError::Full);
        }
        self.buckets[entry.channel_hash as usize].push(self.entries.len());
        self.entries.push(entry);
        Ok(())
    }

    /// Add a key under a channel name (an empty name means `LongFast`). A
    /// 1-byte key is the firmware's `simpleN` shorthand and is expanded.
    pub fn add(&mut self, channel_name: &str, psk: &[u8]) -> Result<(), KeyError> {
        let psk: Vec<u8> = match psk.len() {
            1 => simple_psk(psk[0]).to_vec(),
            0 | 16 | 32 => psk.to_vec(),
            n => return Err(KeyError::BadLength(n)),
        };
        let name = if channel_name.is_empty() { "LongFast" } else { channel_name };
        self.push(KeyEntry {
            channel_name: name.to_string(),
            channel_hash: channel_hash(name, &psk),
            psk,
        })
    }

    /// Add a key whose channel hash is known but whose name is not.
    pub fn add_raw(
        &mut self,
        channel_hash: u8,
        psk: &[u8],
        display_name: &str,
    ) -> Result<(), KeyError> {
        if !matches!(psk.len(), 0 | 16 | 32) {
            return Err(KeyError::BadLength(psk.len()));
        }
        let name = if display_name.is_empty() { "(discovered)" } else { display_name };
        self.push(KeyEntry { channel_name: name.to_string(), psk: psk.to_vec(), channel_hash })
    }

    /// Add one `[Name=]key` item. The key is `default`, `none`/`unencrypted`,
    /// `simpleN`, `hex:...`, `base64:...`, or bare base64 then bare hex. A bare
    /// `default` registers the stock key under every preset channel name.
    pub fn add_spec(&mut self, spec: &str) -> Result<(), KeyError> {
        let spec = spec.trim();
        if spec.is_empty() {
            return Ok(());
        }
        let (name, value) = match spec.split_once('=') {
            Some((n, v)) => (Some(n.trim()), v.trim()),
            None => (None, spec),
        };
        let lower = value.to_ascii_lowercase();
        let bad = || KeyError::BadSpec(spec.to_string());
        let psk: Vec<u8> = if lower == "default" {
            if name.is_none() {
                let mut rc = Ok(());
                for p in &PRESETS {
                    if let Err(e) = self.add(p.channel_name, &DEFAULT_PSK) {
                        rc = Err(e);
                    }
                }
                return rc;
            }
            DEFAULT_PSK.to_vec()
        } else if lower == "none" || lower == "unencrypted" {
            Vec::new()
        } else if let Some(n) =
            lower.strip_prefix("simple").filter(|r| r.starts_with(|c: char| c.is_ascii_digit()))
        {
            let n: u32 = n.parse().map_err(|_| bad())?;
            if n > 255 {
                return Err(bad());
            }
            if n == 0 { Vec::new() } else { simple_psk(n as u8).to_vec() }
        } else if lower.starts_with("hex:") {
            hex_decode(&value[4..]).filter(|v| !v.is_empty()).ok_or_else(bad)?
        } else if lower.starts_with("base64:") {
            b64_decode(&value[7..]).filter(|v| !v.is_empty()).ok_or_else(bad)?
        } else {
            b64_decode(value)
                .filter(|v| !v.is_empty())
                .or_else(|| hex_decode(value).filter(|v| !v.is_empty()))
                .ok_or_else(bad)?
        };
        self.add(name.unwrap_or(""), &psk)
    }

    /// Add a comma-, semicolon- or newline-separated list of specs. Returns
    /// how many were accepted.
    pub fn add_csv(&mut self, csv: &str) -> usize {
        csv.split([',', ';', '\n'])
            .filter(|t| !t.trim().is_empty())
            .filter(|t| self.add_spec(t).is_ok())
            .count()
    }

    /// The keys that may have encrypted a frame with this channel-hash byte.
    pub fn candidates(&self, hash: u8) -> impl Iterator<Item = &KeyEntry> {
        self.buckets.get(hash as usize).into_iter().flatten().filter_map(|&i| self.entries.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_one_byte_key_is_expanded_the_simple_way() {
        let mut k = KeySet::new();
        k.add("LongFast", &[0x01]).unwrap();
        assert_eq!(k.entries()[0].psk, DEFAULT_PSK.to_vec());
        assert_eq!(k.entries()[0].channel_hash, 0x08);
        k.add("Other", &[0x02]).unwrap();
        assert_eq!(k.entries()[1].psk[15], 0x02);
        assert_eq!(&k.entries()[1].psk[..15], &DEFAULT_PSK[..15]);
    }

    #[test]
    fn a_bad_length_is_refused() {
        let mut k = KeySet::new();
        assert_eq!(k.add("x", &[0; 5]), Err(KeyError::BadLength(5)));
    }

    #[test]
    fn default_registers_every_preset_name() {
        let mut k = KeySet::new();
        k.add_spec("default").unwrap();
        assert_eq!(k.len(), PRESETS.len());
        assert!(k.candidates(0x08).any(|e| e.channel_name == "LongFast"));
    }

    #[test]
    fn specs_in_every_form() {
        let mut k = KeySet::new();
        k.add_spec("Private=none").unwrap();
        k.add_spec("A=simple3").unwrap();
        k.add_spec("B=hex:00112233445566778899aabbccddeeff").unwrap();
        k.add_spec("C=base64:AAECAwQFBgcICQoLDA0ODw==").unwrap();
        k.add_spec("D=AQ==").unwrap();
        assert!(k.entries()[0].psk.is_empty());
        assert_eq!(k.entries()[1].psk[15], 3);
        assert_eq!(k.entries()[2].psk[0], 0x00);
        assert_eq!(k.entries()[2].psk[15], 0xff);
        assert_eq!(k.entries()[3].psk, (0..16u8).collect::<Vec<_>>());
        assert_eq!(k.entries()[4].psk, DEFAULT_PSK.to_vec());
    }

    #[test]
    fn garbage_specs_are_refused() {
        let mut k = KeySet::new();
        assert!(k.add_spec("X=hex:zz").is_err());
        assert!(k.add_spec("X=simple999").is_err());
        assert!(k.add_spec("X=!!!!").is_err());
        assert!(k.is_empty());
    }

    #[test]
    fn csv_counts_the_accepted_ones() {
        let mut k = KeySet::new();
        assert_eq!(k.add_csv("A=simple2, B=nonsense!!, C=none\nD=simple4;"), 3);
    }

    #[test]
    fn candidates_come_from_the_hash_bucket_only() {
        let mut k = KeySet::new();
        k.add("LongFast", &DEFAULT_PSK).unwrap();
        assert_eq!(k.candidates(0x08).count(), 1);
        assert_eq!(k.candidates(0x09).count(), 0);
    }
}
