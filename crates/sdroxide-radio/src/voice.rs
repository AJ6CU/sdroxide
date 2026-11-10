//! Voice keyer: ten recorded messages, stored as 48 kHz mono WAV files and
//! transmitted in place of the microphone.
//!
//! This owns nothing but samples and files. The engine decides *when* it may
//! record (never while keyed) and *when* the transmitter goes up and down; all
//! this type does is accumulate microphone audio into a slot, and hand blocks
//! of a stored message back out at the transmit rate.
//!
//! Everything is kept in memory at 48 kHz mono — ten two-minute messages is
//! ~46 MB in the worst case, and a real set of keyer messages is a fraction of
//! that — so playback never waits on the disk mid-over.

use std::io::Write;
use std::path::{Path, PathBuf};

use tracing::{info, warn};

use sdroxide_types::{VOICE_MAX_LEN_S, VOICE_SLOTS, VoiceSlotInfo, VoiceStatus};

/// The rate everything is stored and played at — the engine's transmit-audio
/// rate, so playback needs no resampling.
pub const VOICE_RATE: f64 = 48_000.0;

/// One stored message.
#[derive(Default)]
struct Slot {
    name: String,
    samples: Vec<f32>,
}

/// A message being transmitted: which slot, and how far in.
struct Playback {
    slot: usize,
    pos: usize,
}

pub struct VoiceKeyer {
    slots: Vec<Slot>,
    /// Slot being recorded into. Its samples accumulate in `slots[i].samples`
    /// only on stop, so an abandoned recording never overwrites a good message.
    rec: Option<(usize, Vec<f32>)>,
    play: Option<Playback>,
    /// Local monitor playback — the operator listening to a message through the
    /// speakers. Deliberately a separate cursor from `play`: previewing must
    /// never share state with what goes on the air.
    preview: Option<Playback>,
    /// Directory the WAVs live in. `None` when the config directory could not
    /// be resolved — the keyer then works for the session but stores nothing.
    dir: Option<PathBuf>,
}

impl VoiceKeyer {
    /// Load whatever is on disk. Never fails: a missing or unreadable slot is
    /// simply an empty one.
    pub fn load() -> VoiceKeyer {
        let dir = match sdroxide_config::voice_dir() {
            Ok(d) => Some(d),
            Err(e) => {
                warn!("no voice-keyer directory: {e}; recordings will not be stored");
                None
            }
        };
        let names = sdroxide_config::load_voice_names();
        let mut slots = Vec::with_capacity(VOICE_SLOTS);
        for i in 0..VOICE_SLOTS {
            let samples = dir
                .as_ref()
                .map(|d| slot_path(d, i))
                .filter(|p| p.exists())
                .and_then(|p| match read_wav_mono_48k(&p) {
                    Ok(s) => Some(s),
                    Err(e) => {
                        warn!(path = %p.display(), "voice slot unreadable: {e}");
                        None
                    }
                })
                .unwrap_or_default();
            slots.push(Slot { name: names.get(i).cloned().unwrap_or_default(), samples });
        }
        let stored = slots.iter().filter(|s| !s.samples.is_empty()).count();
        if stored > 0 {
            info!(slots = stored, "voice keyer loaded");
        }
        VoiceKeyer { slots, rec: None, play: None, preview: None, dir }
    }

    pub fn status(&self) -> VoiceStatus {
        VoiceStatus {
            slots: self
                .slots
                .iter()
                .map(|s| VoiceSlotInfo {
                    name: s.name.clone(),
                    len_s: s.samples.len() as f32 / VOICE_RATE as f32,
                })
                .collect(),
            recording: self.rec.as_ref().map(|(i, _)| *i as u8),
            playing: self.play.as_ref().map(|p| p.slot as u8),
            previewing: self.preview.as_ref().map(|p| p.slot as u8),
            position_s: self.position_s(),
            max_len_s: VOICE_MAX_LEN_S,
        }
    }

    /// Seconds into whichever of record/transmit/preview is running (0 when
    /// none is; they are mutually exclusive).
    fn position_s(&self) -> f32 {
        let n = match (&self.rec, &self.play, &self.preview) {
            (Some((_, buf)), _, _) => buf.len(),
            (None, Some(p), _) => p.pos,
            (None, None, Some(p)) => p.pos,
            _ => 0,
        };
        n as f32 / VOICE_RATE as f32
    }

    pub fn is_recording(&self) -> bool {
        self.rec.is_some()
    }

    pub fn is_playing(&self) -> bool {
        self.play.is_some()
    }

    pub fn is_previewing(&self) -> bool {
        self.preview.is_some()
    }

    pub fn has_audio(&self, slot: usize) -> bool {
        self.slots.get(slot).is_some_and(|s| !s.samples.is_empty())
    }

    /// Begin recording into `slot`, discarding whatever it held. Returns false
    /// for an out-of-range slot.
    pub fn start_record(&mut self, slot: usize) -> bool {
        if slot >= self.slots.len() {
            return false;
        }
        self.play = None;
        self.preview = None;
        self.rec = Some((slot, Vec::new()));
        true
    }

    /// Microphone audio at 48 kHz, while recording. Returns false once the
    /// length cap is reached, so the caller can stop and store the message.
    pub fn push_mic(&mut self, pcm_48k: &[f32]) -> bool {
        let Some((_, buf)) = self.rec.as_mut() else { return true };
        let cap = (VOICE_MAX_LEN_S as f64 * VOICE_RATE) as usize;
        let room = cap.saturating_sub(buf.len());
        buf.extend_from_slice(&pcm_48k[..pcm_48k.len().min(room)]);
        buf.len() < cap
    }

    /// Stop recording and store what was captured. A recording with no audio in
    /// it (the operator stopped immediately) leaves the slot as it was.
    pub fn stop_record(&mut self) {
        let Some((slot, buf)) = self.rec.take() else { return };
        // Under ~0.1 s is a mis-click, not a message.
        if buf.len() < (VOICE_RATE / 10.0) as usize {
            info!(slot, samples = buf.len(), "voice recording too short; slot unchanged");
            return;
        }
        info!(slot, secs = buf.len() as f32 / VOICE_RATE as f32, "voice message recorded");
        self.slots[slot].samples = buf;
        self.write_slot(slot);
    }

    /// Start transmitting `slot`. False when the slot is out of range or empty —
    /// which is what makes a bound key over an unrecorded slot a no-op rather
    /// than a keyed transmitter with nothing to say.
    pub fn start_play(&mut self, slot: usize) -> bool {
        if !self.has_audio(slot) {
            return false;
        }
        self.rec = None;
        self.preview = None;
        self.play = Some(Playback { slot, pos: 0 });
        true
    }

    pub fn stop_play(&mut self) {
        self.play = None;
    }

    /// Start monitoring `slot` locally (speakers, not the transmitter). False
    /// for an empty or out-of-range slot, exactly as [`Self::start_play`].
    pub fn start_preview(&mut self, slot: usize) -> bool {
        if !self.has_audio(slot) {
            return false;
        }
        self.rec = None;
        self.preview = Some(Playback { slot, pos: 0 });
        true
    }

    pub fn stop_preview(&mut self) {
        self.preview = None;
    }

    /// Copy up to `out.len()` samples of the previewed message into `out` and
    /// report how many were written. Zero means it has played out — unlike
    /// [`Self::fill_tx_block`] there is no tail to keep clean, so the caller
    /// simply stops.
    pub fn fill_preview(&mut self, out: &mut [f32]) -> usize {
        let Some(p) = self.preview.as_mut() else { return 0 };
        let Some(slot) = self.slots.get(p.slot) else { return 0 };
        let take = slot.samples.len().saturating_sub(p.pos).min(out.len());
        out[..take].copy_from_slice(&slot.samples[p.pos..p.pos + take]);
        p.pos += take;
        take
    }

    /// Fill one transmit block with the message, zero-padding past its end.
    /// Playback stays "active" (returning silence) until the engine stops it,
    /// so a mode with a transmit tail — RADE's end-of-over frame — is never fed
    /// live microphone audio for the last few blocks of the over.
    pub fn fill_tx_block(&mut self, out: &mut [f32]) {
        out.fill(0.0);
        let Some(p) = self.play.as_mut() else { return };
        let Some(slot) = self.slots.get(p.slot) else { return };
        let take = (slot.samples.len() - p.pos.min(slot.samples.len())).min(out.len());
        out[..take].copy_from_slice(&slot.samples[p.pos..p.pos + take]);
        p.pos += take;
    }

    /// Whether the message has been read out in full (the tail may still be
    /// draining through the transmit chain).
    pub fn play_finished(&self) -> bool {
        match self.play.as_ref() {
            Some(p) => self.slots.get(p.slot).is_none_or(|s| p.pos >= s.samples.len()),
            None => false,
        }
    }

    /// Erase a slot's recording (the label is kept; it is what the operator
    /// named the *slot*, not the take).
    pub fn clear(&mut self, slot: usize) {
        let Some(s) = self.slots.get_mut(slot) else { return };
        s.samples = Vec::new();
        if self.play.as_ref().is_some_and(|p| p.slot == slot) {
            self.play = None;
        }
        if self.preview.as_ref().is_some_and(|p| p.slot == slot) {
            self.preview = None;
        }
        if self.rec.as_ref().is_some_and(|(i, _)| *i == slot) {
            self.rec = None;
        }
        if let Some(dir) = self.dir.as_ref() {
            let path = slot_path(dir, slot);
            if path.exists()
                && let Err(e) = std::fs::remove_file(&path)
            {
                warn!(path = %path.display(), "could not delete voice slot: {e}");
            }
        }
    }

    pub fn rename(&mut self, slot: usize, name: String) {
        let Some(s) = self.slots.get_mut(slot) else { return };
        // Long enough for a legible label, short enough not to break the row.
        s.name = name.trim().chars().take(24).collect();
        let names: Vec<String> = self.slots.iter().map(|s| s.name.clone()).collect();
        if let Err(e) = sdroxide_config::save_voice_names(&names) {
            warn!("saving voice slot names: {e}");
        }
    }

    fn write_slot(&self, slot: usize) {
        let Some(dir) = self.dir.as_ref() else { return };
        let path = slot_path(dir, slot);
        if let Err(e) = write_wav_mono_48k(&path, &self.slots[slot].samples) {
            warn!(path = %path.display(), "could not store voice message: {e}");
        }
    }
}

fn slot_path(dir: &Path, slot: usize) -> PathBuf {
    dir.join(format!("slot{}.wav", slot + 1))
}

// ── WAV (16-bit PCM, mono, 48 kHz) ──────────────────────────────────────────
//
// Hand-rolled rather than pulled from a crate: the keyer only ever reads and
// writes this one flavour, and a plain file means a message can be edited in
// any audio editor and dropped back in.

fn write_wav_mono_48k(path: &Path, samples: &[f32]) -> std::io::Result<()> {
    let data_len = samples.len() * 2;
    let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
    f.write_all(b"RIFF")?;
    f.write_all(&((36 + data_len) as u32).to_le_bytes())?;
    f.write_all(b"WAVEfmt ")?;
    f.write_all(&16u32.to_le_bytes())?; // PCM fmt chunk size
    f.write_all(&1u16.to_le_bytes())?; // format: PCM
    f.write_all(&1u16.to_le_bytes())?; // channels
    f.write_all(&(VOICE_RATE as u32).to_le_bytes())?;
    f.write_all(&((VOICE_RATE as u32) * 2).to_le_bytes())?; // byte rate
    f.write_all(&2u16.to_le_bytes())?; // block align
    f.write_all(&16u16.to_le_bytes())?; // bits per sample
    f.write_all(b"data")?;
    f.write_all(&(data_len as u32).to_le_bytes())?;
    for &s in samples {
        f.write_all(&((s.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes())?;
    }
    f.flush()
}

/// Read a WAV file back as mono 48 kHz f32.
///
/// Deliberately tolerant, because the operator may have dropped in a file some
/// editor wrote: any chunk order, 16- or 24-bit PCM or 32-bit float, mono or
/// multi-channel (downmixed), any sample rate (linearly resampled to 48 kHz).
fn read_wav_mono_48k(path: &Path) -> std::io::Result<Vec<f32>> {
    use std::io::{Error, ErrorKind};
    let bytes = std::fs::read(path)?;
    let bad = |m: &str| Error::new(ErrorKind::InvalidData, m.to_string());
    if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err(bad("not a RIFF/WAVE file"));
    }
    let u16at = |o: usize| u16::from_le_bytes([bytes[o], bytes[o + 1]]);
    let u32at = |o: usize| u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]]);

    let (mut fmt, mut data) = (None, None);
    let mut pos = 12;
    // Chunk headers are 8 bytes and chunks are word-aligned.
    while pos + 8 <= bytes.len() {
        let id = &bytes[pos..pos + 4];
        let len = u32at(pos + 4) as usize;
        let body = pos + 8;
        let end = body.saturating_add(len).min(bytes.len());
        match id {
            b"fmt " if len >= 16 => fmt = Some(body),
            b"data" => data = Some((body, end)),
            _ => {}
        }
        pos = body + len + (len & 1);
    }
    let (Some(fmt), Some((from, to))) = (fmt, data) else {
        return Err(bad("missing fmt or data chunk"));
    };

    let format = u16at(fmt);
    let channels = u16at(fmt + 2).max(1) as usize;
    let rate = u32at(fmt + 4) as f64;
    let bits = u16at(fmt + 14) as usize;
    // 0xFFFE is WAVE_FORMAT_EXTENSIBLE; its real format sits in the extension,
    // but for the two encodings we accept the bit depth already decides.
    if !matches!(format, 1 | 3 | 0xFFFE) {
        return Err(bad("unsupported WAV encoding (not PCM or float)"));
    }
    let bytes_per = bits / 8;
    if bytes_per == 0 || !matches!(bits, 16 | 24 | 32) {
        return Err(bad("unsupported WAV sample width"));
    }
    let raw = &bytes[from..to];
    let frame = bytes_per * channels;
    if frame == 0 {
        return Err(bad("zero-width WAV frames"));
    }

    let mut mono = Vec::with_capacity(raw.len() / frame);
    for f in raw.chunks_exact(frame) {
        let mut sum = 0.0f32;
        for c in f.chunks_exact(bytes_per) {
            sum += match (bits, format) {
                (32, 3) => f32::from_le_bytes([c[0], c[1], c[2], c[3]]),
                (32, _) => i32::from_le_bytes([c[0], c[1], c[2], c[3]]) as f32 / 2_147_483_648.0,
                (24, _) => {
                    // Sign-extend the 24-bit sample into an i32.
                    (i32::from_le_bytes([0, c[0], c[1], c[2]]) >> 8) as f32 / 8_388_608.0
                }
                _ => i16::from_le_bytes([c[0], c[1]]) as f32 / 32_768.0,
            };
        }
        mono.push(sum / channels as f32);
    }

    if (rate - VOICE_RATE).abs() < 1.0 || rate <= 0.0 {
        return Ok(mono);
    }
    // Linear resample. Only ever runs on a file the operator supplied — the
    // keyer's own recordings are already at the engine rate.
    let ratio = rate / VOICE_RATE;
    let out_len = (mono.len() as f64 / ratio) as usize;
    let mut out = Vec::with_capacity(out_len);
    for i in 0..out_len {
        let x = i as f64 * ratio;
        let i0 = x as usize;
        let f = (x - i0 as f64) as f32;
        let a = mono.get(i0).copied().unwrap_or(0.0);
        let b = mono.get(i0 + 1).copied().unwrap_or(a);
        out.push(a + (b - a) * f);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A keyer with no slots and no directory — nothing here touches the disk.
    fn bare() -> VoiceKeyer {
        VoiceKeyer { slots: Vec::new(), rec: None, play: None, preview: None, dir: None }
    }

    fn tone(n: usize) -> Vec<f32> {
        (0..n).map(|i| 0.5 * (std::f32::consts::TAU * 700.0 * i as f32 / 48_000.0).sin()).collect()
    }

    #[test]
    fn wav_round_trips_through_the_file() {
        let path = std::env::temp_dir().join(format!("sdroxide-voice-{}.wav", std::process::id()));
        let samples = tone(4_800);
        write_wav_mono_48k(&path, &samples).expect("write");
        let back = read_wav_mono_48k(&path).expect("read");
        let _ = std::fs::remove_file(&path);
        assert_eq!(back.len(), samples.len());
        for (a, b) in samples.iter().zip(&back) {
            assert!((a - b).abs() < 1e-3, "{a} vs {b}");
        }
    }

    /// A file at another rate is resampled to the engine's, so a message
    /// dropped in from an editor plays at the right pitch and length.
    #[test]
    fn foreign_sample_rates_are_resampled() {
        let path =
            std::env::temp_dir().join(format!("sdroxide-voice-8k-{}.wav", std::process::id()));
        // Hand-build a 24 kHz mono header around one second of samples.
        let n = 24_000usize;
        let mut f = std::fs::File::create(&path).unwrap();
        let data_len = n * 2;
        f.write_all(b"RIFF").unwrap();
        f.write_all(&((36 + data_len) as u32).to_le_bytes()).unwrap();
        f.write_all(b"WAVEfmt ").unwrap();
        f.write_all(&16u32.to_le_bytes()).unwrap();
        f.write_all(&1u16.to_le_bytes()).unwrap();
        f.write_all(&1u16.to_le_bytes()).unwrap();
        f.write_all(&24_000u32.to_le_bytes()).unwrap();
        f.write_all(&48_000u32.to_le_bytes()).unwrap();
        f.write_all(&2u16.to_le_bytes()).unwrap();
        f.write_all(&16u16.to_le_bytes()).unwrap();
        f.write_all(b"data").unwrap();
        f.write_all(&(data_len as u32).to_le_bytes()).unwrap();
        for i in 0..n {
            let s = (0.25 * (i as f32 / n as f32) * 32767.0) as i16;
            f.write_all(&s.to_le_bytes()).unwrap();
        }
        drop(f);
        let back = read_wav_mono_48k(&path).expect("read");
        let _ = std::fs::remove_file(&path);
        assert!(
            back.len().abs_diff(48_000) < 4,
            "one second at 24 kHz should be 48 000 samples, got {}",
            back.len()
        );
    }

    #[test]
    fn playback_zero_pads_past_the_end_and_reports_finished() {
        let mut k = bare();
        k.slots.push(Slot { name: String::new(), samples: tone(700) });
        assert!(k.start_play(0));

        let mut block = [0.0f32; 480];
        k.fill_tx_block(&mut block);
        assert!(!k.play_finished());
        assert!(block.iter().any(|s| *s != 0.0));

        // Second block runs past the end: the tail is silence, not stale audio.
        k.fill_tx_block(&mut block);
        assert!(k.play_finished());
        assert!(block[220..].iter().all(|s| *s == 0.0));

        // And it keeps returning silence rather than looping.
        k.fill_tx_block(&mut block);
        assert!(block.iter().all(|s| *s == 0.0));
    }

    #[test]
    fn an_empty_slot_never_starts_a_transmission() {
        let mut k = bare();
        k.slots.push(Slot::default());
        assert!(!k.start_play(0));
        assert!(!k.start_play(7), "out-of-range slots are refused too");
        assert!(!k.is_playing());
    }

    #[test]
    fn recording_stops_at_the_length_cap() {
        let mut k = bare();
        k.slots.push(Slot::default());
        assert!(k.start_record(0));
        let cap = (VOICE_MAX_LEN_S as f64 * VOICE_RATE) as usize;
        let chunk = vec![0.1f32; 48_000];
        let mut pushed = 0;
        let mut room = true;
        while room && pushed < cap + 48_000 {
            room = k.push_mic(&chunk);
            pushed += chunk.len();
        }
        assert!(!room, "the cap should have been reported");
        assert_eq!(k.status().position_s.round(), VOICE_MAX_LEN_S.round());
    }
}
