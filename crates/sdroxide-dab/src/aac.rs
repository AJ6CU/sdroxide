//! Plain HE-AAC decode for DAB+, over the stock faad2 the binary already links.
//!
//! This is the counterpart of `dabradio`'s fdk-aac `AacDecoder`: the same three
//! calls (`new`, `configure`, `decode_au`) over the same DAB+ Access Units, but
//! through faad2 instead. `dabradio`'s optional-fdk-aac split exists so a caller
//! can do exactly this — its `SuperframeDecoder` (Fire code, Reed–Solomon,
//! Access Unit extraction) is always built, and only the AAC step is swapped.
//!
//! The C shim in `aac_shim.c` is deliberately the only `unsafe` in the crate:
//! the FFI lives there and this wrapper presents a safe surface.

use std::os::raw::{c_int, c_uchar, c_uint, c_ulong};

/// faad2's opaque decoder handle, as the shim sees it.
#[repr(C)]
struct DabAac {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn dab_aac_open() -> *mut DabAac;
    fn dab_aac_init(d: *mut DabAac, asc: *const c_uchar, asc_len: c_ulong) -> c_int;
    fn dab_aac_rate(d: *const DabAac) -> c_ulong;
    fn dab_aac_channels(d: *const DabAac) -> c_uint;
    fn dab_aac_decode(
        d: *mut DabAac,
        au: *const c_uchar,
        au_len: c_ulong,
        out: *mut i16,
        out_cap: c_ulong,
    ) -> c_int;
    fn dab_aac_close(d: *mut DabAac);
}

/// One AAC decoder, configured from a DAB+ AudioSpecificConfig.
pub struct AacDecoder {
    handle: *mut DabAac,
    /// Interleaved PCM scratch, sized for one DAB+ Access Unit with SBR.
    pcm: Vec<i16>,
    rate: u32,
    channels: usize,
}

// The faad2 handle is not shared across threads; a decoder is owned by the
// worker that reads it. The pointer is an owned resource, and `Send` is what a
// decoder on a decode thread needs.
unsafe impl Send for AacDecoder {}

impl AacDecoder {
    /// Open a decoder. `None` when faad2 cannot initialise — the one failure the
    /// caller has to be told about rather than have surface as silence.
    pub fn new() -> Option<Self> {
        let handle = unsafe { dab_aac_open() };
        if handle.is_null() {
            return None;
        }
        Some(AacDecoder {
            handle,
            // 2048 frames × 2 channels × 2 (SBR doubles the rate) — the largest
            // a DAB+ Access Unit can decode to.
            pcm: vec![0i16; 2048 * 2 * 2],
            rate: 0,
            channels: 0,
        })
    }

    /// Configure from the DAB+ format bytes (the AudioSpecificConfig). Whether
    /// it succeeds is the caller's to see: an unsupported profile is a real
    /// answer, and the service is then named with no audio rather than silent.
    pub fn configure(&mut self, asc: &[u8]) -> bool {
        let r = unsafe { dab_aac_init(self.handle, asc.as_ptr(), asc.len() as c_ulong) };
        if r == 0 {
            self.rate = unsafe { dab_aac_rate(self.handle) } as u32;
            self.channels = unsafe { dab_aac_channels(self.handle) } as usize;
            true
        } else {
            false
        }
    }

    /// The sample rate faad2 learned at configure time, in Hz.
    pub fn sample_rate(&self) -> u32 {
        self.rate
    }

    /// The channel count faad2 learned at configure time.
    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Decode one Access Unit to interleaved `f32` PCM in −1..=1. Empty when
    /// the unit does not decode — a dropped AU is a gap, not a panic.
    pub fn decode_au(&mut self, au: &[u8]) -> Vec<f32> {
        if au.is_empty() {
            return Vec::new();
        }
        let frames = unsafe {
            dab_aac_decode(
                self.handle,
                au.as_ptr(),
                au.len() as c_ulong,
                self.pcm.as_mut_ptr(),
                self.pcm.len() as c_ulong,
            )
        };
        if frames <= 0 {
            return Vec::new();
        }
        let ch = self.channels.max(1);
        let n = (frames as usize * ch).min(self.pcm.len());
        self.pcm[..n].iter().map(|&s| s as f32 / 32768.0).collect()
    }
}

impl Default for AacDecoder {
    fn default() -> Self {
        Self::new().expect("faad2 is linked into the binary")
    }
}

impl Drop for AacDecoder {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { dab_aac_close(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pack `(value, bits)` fields MSB-first, zero-padded to a whole byte.
    fn pack(fields: &[(u32, u32)]) -> Vec<u8> {
        let mut out = Vec::new();
        let (mut acc, mut n) = (0u32, 0u32);
        for &(v, bits) in fields {
            for i in (0..bits).rev() {
                acc = (acc << 1) | ((v >> i) & 1);
                n += 1;
                if n == 8 {
                    out.push(acc as u8);
                    (acc, n) = (0, 0);
                }
            }
        }
        if n > 0 {
            out.push((acc << (8 - n)) as u8);
        }
        out
    }

    /// A silent AAC-LC frame (one SCE with no bands) followed by a fill element
    /// carrying one byte of plain padding — the shape DAB+ encoders send. faad2
    /// built for DRM refused every such frame, which chopped DAB+ audio.
    #[test]
    fn a_frame_padded_with_a_fill_element_decodes() {
        // AudioSpecificConfig: AAC-LC, 48 kHz, mono.
        let asc = pack(&[(2, 5), (3, 4), (1, 4), (0, 3)]);
        let au = pack(&[
            (0, 3),   // ID_SCE
            (0, 4),   // element_instance_tag
            (100, 8), // global_gain
            (0, 11),  // ics_info: long window, max_sfb 0, no prediction
            (0, 3),   // no pulse, TNS or gain control
            (6, 3),   // ID_FIL
            (1, 4),   // count: one byte
            (0, 8),   // EXT_FILL
            (7, 3),   // ID_END
        ]);
        let mut dec = AacDecoder::new().expect("faad2");
        assert!(dec.configure(&asc), "faad2 took the config");
        let pcm: usize = (0..4).map(|_| dec.decode_au(&au).len()).sum();
        assert!(pcm > 0, "every frame carrying a fill element was refused");
    }
}
