/*
 * sdroxide_rade shim: a stable, minimal C surface over Opus's FARGAN/LPCNet
 * vocoder.
 *
 * RADE's own API (rade_api.h) starts and ends at FARGAN feature vectors:
 * rade_tx() consumes them, rade_rx() produces them. Turning 16 kHz speech into
 * features and back is the caller's job, and needs Opus internals (`lpcnet.h`,
 * `fargan.h`) that are not meant to be consumed from outside the Opus build.
 * Rather than binding those headers, this shim exposes the four operations we
 * actually need behind opaque handles.
 *
 * Frame geometry (fixed by Opus): one call is LPCNET_FRAME_SIZE = 160 samples
 * = 10 ms at 16 kHz, producing NB_TOTAL_FEATURES = 36 floats. RADE V1 packs 12
 * such frames into each 120 ms modem frame.
 */

#ifndef SDROXIDE_RADE_SHIM_H
#define SDROXIDE_RADE_SHIM_H

#ifdef __cplusplus
extern "C" {
#endif

/* Speech samples consumed/produced per call (LPCNET_FRAME_SIZE). */
int sdrx_voc_frame_size(void);
/* Floats per feature vector (NB_TOTAL_FEATURES). */
int sdrx_voc_n_features(void);

/* --- speech -> features --- */

void *sdrx_voc_enc_create(void);
void  sdrx_voc_enc_destroy(void *enc);
/* pcm16: sdrx_voc_frame_size() samples. feat: sdrx_voc_n_features() floats.
   Returns 0 on success, non-zero on a null handle. */
int   sdrx_voc_enc(void *enc, const short *pcm16, float *feat);

/* --- features -> speech --- */

void *sdrx_voc_dec_create(void);
void  sdrx_voc_dec_destroy(void *dec);
/* feat: sdrx_voc_n_features() floats. pcm16: sdrx_voc_frame_size() samples.
   FARGAN needs a five-frame warm-up before it can synthesize; the shim buffers
   those internally and writes silence until it is primed. Returns 1 when
   pcm16[] holds synthesized speech, 0 while priming, negative on error. */
int   sdrx_voc_dec(void *dec, const float *feat, short *pcm16);
/* Discard synthesis state and re-arm the warm-up (call on loss of sync). */
void  sdrx_voc_dec_reset(void *dec);

#ifdef __cplusplus
}
#endif

#endif /* SDROXIDE_RADE_SHIM_H */
