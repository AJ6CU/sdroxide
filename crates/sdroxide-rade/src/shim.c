/*
 * Implementation of the vocoder shim declared in shim.h.
 *
 * The call sequences here mirror rade_c's own reference tools — feature
 * extraction follows `lpcnet_demo.c -features`, synthesis follows
 * `rade_rx_wav.c` (including its five-frame `fargan_cont()` warm-up and the
 * float -> int16 rounding) — so output matches the upstream implementation.
 */

#ifdef HAVE_CONFIG_H
#include "config.h"
#endif

#include <math.h>
#include <stdlib.h>
#include <string.h>

#include "arch.h"
#include "cpu_support.h"
#include "fargan.h"
#include "lpcnet.h"

#include "shim.h"

#define CONT_FRAMES 5

int sdrx_voc_frame_size(void) { return LPCNET_FRAME_SIZE; }
int sdrx_voc_n_features(void) { return NB_TOTAL_FEATURES; }

/* ------------------------------------------------------------------ encoder */

typedef struct {
    LPCNetEncState *net;
    int arch;
} sdrx_enc;

void *sdrx_voc_enc_create(void) {
    sdrx_enc *e = (sdrx_enc *)calloc(1, sizeof(sdrx_enc));
    if (e == NULL) return NULL;
    e->net = lpcnet_encoder_create();
    if (e->net == NULL) {
        free(e);
        return NULL;
    }
    e->arch = opus_select_arch();
    return e;
}

void sdrx_voc_enc_destroy(void *enc) {
    sdrx_enc *e = (sdrx_enc *)enc;
    if (e == NULL) return;
    lpcnet_encoder_destroy(e->net);
    free(e);
}

int sdrx_voc_enc(void *enc, const short *pcm16, float *feat) {
    sdrx_enc *e = (sdrx_enc *)enc;
    if (e == NULL || pcm16 == NULL || feat == NULL) return -1;
    lpcnet_compute_single_frame_features(e->net, (const opus_int16 *)pcm16, feat, e->arch);
    return 0;
}

/* ------------------------------------------------------------------ decoder */

typedef struct {
    FARGANState fargan;
    /* Warm-up: FARGAN wants five consecutive frames before the first
       synthesis. Buffered at NB_TOTAL_FEATURES stride, then repacked to the
       NB_FEATURES stride fargan_cont() expects. */
    float cont_buf[CONT_FRAMES * NB_TOTAL_FEATURES];
    int   cont_frames;
    int   ready;
} sdrx_dec;

static void dec_arm(sdrx_dec *d) {
    fargan_init(&d->fargan);
    d->cont_frames = 0;
    d->ready = 0;
}

void *sdrx_voc_dec_create(void) {
    sdrx_dec *d = (sdrx_dec *)calloc(1, sizeof(sdrx_dec));
    if (d == NULL) return NULL;
    dec_arm(d);
    return d;
}

void sdrx_voc_dec_destroy(void *dec) { free(dec); }

void sdrx_voc_dec_reset(void *dec) {
    sdrx_dec *d = (sdrx_dec *)dec;
    if (d != NULL) dec_arm(d);
}

int sdrx_voc_dec(void *dec, const float *feat, short *pcm16) {
    sdrx_dec *d = (sdrx_dec *)dec;
    if (d == NULL || feat == NULL || pcm16 == NULL) return -1;

    if (!d->ready) {
        memcpy(&d->cont_buf[d->cont_frames * NB_TOTAL_FEATURES], feat,
               (size_t)NB_TOTAL_FEATURES * sizeof(float));
        if (++d->cont_frames >= CONT_FRAMES) {
            /* fargan_cont() reads the five frames packed at NB_FEATURES stride,
               not NB_TOTAL_FEATURES — same repack rade_rx_wav.c does. */
            float packed[CONT_FRAMES * NB_FEATURES];
            for (int i = 0; i < CONT_FRAMES; i++) {
                memcpy(&packed[i * NB_FEATURES], &d->cont_buf[i * NB_TOTAL_FEATURES],
                       (size_t)NB_FEATURES * sizeof(float));
            }
            float zeros[FARGAN_CONT_SAMPLES];
            memset(zeros, 0, sizeof(zeros));
            fargan_cont(&d->fargan, zeros, packed);
            d->ready = 1;
        }
        /* Upstream does not synthesize any of the five warm-up frames,
           including the last one. */
        memset(pcm16, 0, (size_t)LPCNET_FRAME_SIZE * sizeof(short));
        return 0;
    }

    float fpcm[LPCNET_FRAME_SIZE];
    fargan_synthesize(&d->fargan, fpcm, feat);
    for (int s = 0; s < LPCNET_FRAME_SIZE; s++) {
        float v = fpcm[s] * 32768.0f;
        if (v > 32767.0f) v = 32767.0f;
        if (v < -32767.0f) v = -32767.0f;
        pcm16[s] = (short)floor(0.5 + (double)v);
    }
    return 1;
}
