//! Safe wrappers over the FARGAN/LPCNet shim (`shim.h`).
//!
//! RADE's API deals in feature vectors, not speech: [`VocoderEnc`] turns
//! 16 kHz PCM into the vectors [`crate::Rade::tx`] wants, and [`VocoderDec`]
//! turns the vectors [`crate::Rade::rx`] returns back into 16 kHz PCM.

use crate::{RadeError, sys};

/// Speech samples per vocoder frame (10 ms at 16 kHz).
pub fn frame_size() -> usize {
    // Safety: no arguments, no state.
    unsafe { sys::sdrx_voc_frame_size() as usize }
}

/// Floats per feature vector.
pub fn n_features() -> usize {
    // Safety: no arguments, no state.
    unsafe { sys::sdrx_voc_n_features() as usize }
}

/// 16 kHz speech -> feature vectors. `Send`, not `Sync`.
pub struct VocoderEnc {
    h: *mut core::ffi::c_void,
    frame: usize,
    n_feat: usize,
}

// Safety: the handle owns only its own heap state and has no thread affinity.
unsafe impl Send for VocoderEnc {}

impl VocoderEnc {
    pub fn new() -> Result<Self, RadeError> {
        // Safety: allocates and returns an owned handle, or NULL.
        let h = unsafe { sys::sdrx_voc_enc_create() };
        if h.is_null() {
            return Err(RadeError::VocoderAlloc);
        }
        Ok(VocoderEnc { h, frame: frame_size(), n_feat: n_features() })
    }

    /// Samples consumed per [`VocoderEnc::encode`] call.
    pub fn frame_size(&self) -> usize {
        self.frame
    }

    /// Encode one frame, appending [`n_features`] floats to `features`.
    pub fn encode(&mut self, pcm16: &[i16], features: &mut Vec<f32>) -> Result<(), RadeError> {
        if pcm16.len() != self.frame {
            return Err(RadeError::BadLength { want: self.frame, got: pcm16.len() });
        }
        let base = features.len();
        features.resize(base + self.n_feat, 0.0);
        // Safety: `pcm16` holds exactly one frame and `features` has `n_feat`
        // writable slots from `base`; neither pointer is retained.
        unsafe { sys::sdrx_voc_enc(self.h, pcm16.as_ptr(), features[base..].as_mut_ptr()) };
        Ok(())
    }
}

impl Drop for VocoderEnc {
    fn drop(&mut self) {
        // Safety: `self.h` came from `sdrx_voc_enc_create` and is freed once.
        unsafe { sys::sdrx_voc_enc_destroy(self.h) };
    }
}

/// Feature vectors -> 16 kHz speech. `Send`, not `Sync`.
pub struct VocoderDec {
    h: *mut core::ffi::c_void,
    frame: usize,
    n_feat: usize,
}

// Safety: as for `VocoderEnc`.
unsafe impl Send for VocoderDec {}

impl VocoderDec {
    pub fn new() -> Result<Self, RadeError> {
        // Safety: allocates and returns an owned handle, or NULL.
        let h = unsafe { sys::sdrx_voc_dec_create() };
        if h.is_null() {
            return Err(RadeError::VocoderAlloc);
        }
        Ok(VocoderDec { h, frame: frame_size(), n_feat: n_features() })
    }

    /// Samples produced per synthesized frame.
    pub fn frame_size(&self) -> usize {
        self.frame
    }

    /// Synthesize one frame, appending [`VocoderDec::frame_size`] samples to
    /// `pcm` and returning `true`.
    ///
    /// Returns `false` for the first five frames of an over: FARGAN needs them
    /// to warm up and upstream emits no audio for them either, so `pcm` is left
    /// untouched.
    pub fn decode(&mut self, features: &[f32], pcm: &mut Vec<i16>) -> Result<bool, RadeError> {
        if features.len() != self.n_feat {
            return Err(RadeError::BadLength { want: self.n_feat, got: features.len() });
        }
        let base = pcm.len();
        pcm.resize(base + self.frame, 0);
        // Safety: `features` is one full vector and `pcm` has `frame` writable
        // slots from `base`.
        let r = unsafe { sys::sdrx_voc_dec(self.h, features.as_ptr(), pcm[base..].as_mut_ptr()) };
        if r <= 0 {
            pcm.truncate(base);
        }
        Ok(r > 0)
    }

    /// Drop synthesis state and re-arm the warm-up — call on loss of sync so
    /// the next over starts clean instead of continuing from stale state.
    pub fn reset(&mut self) {
        // Safety: `self.h` is a live handle.
        unsafe { sys::sdrx_voc_dec_reset(self.h) };
    }
}

impl Drop for VocoderDec {
    fn drop(&mut self) {
        // Safety: `self.h` came from `sdrx_voc_dec_create` and is freed once.
        unsafe { sys::sdrx_voc_dec_destroy(self.h) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_geometry() {
        assert_eq!(frame_size(), 160); // LPCNET_FRAME_SIZE, 10 ms @ 16 kHz
        assert_eq!(n_features(), 36); // NB_TOTAL_FEATURES
    }

    #[test]
    fn encoder_produces_one_vector_per_frame() {
        let mut enc = VocoderEnc::new().expect("enc");
        let mut feats = Vec::new();
        for _ in 0..4 {
            enc.encode(&vec![0i16; enc.frame_size()], &mut feats).expect("encode");
        }
        assert_eq!(feats.len(), 4 * n_features());
        assert!(matches!(enc.encode(&[0i16; 7], &mut feats), Err(RadeError::BadLength { .. })));
    }

    #[test]
    fn decoder_warms_up_for_five_frames_then_synthesizes() {
        let mut dec = VocoderDec::new().expect("dec");
        let feats = vec![0.0f32; n_features()];
        let mut pcm = Vec::new();
        for _ in 0..5 {
            assert!(!dec.decode(&feats, &mut pcm).expect("decode"));
        }
        assert!(pcm.is_empty(), "warm-up frames emit no audio");
        assert!(dec.decode(&feats, &mut pcm).expect("decode"));
        assert_eq!(pcm.len(), dec.frame_size());
    }
}
