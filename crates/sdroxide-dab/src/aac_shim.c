/* Plain HE-AAC decode for DAB+, over the stock faad2 the binary already links.
 *
 * `dabradio`'s own `DabPlusDecoder` decodes through fdk-aac, which is optional
 * in its split precisely so a caller with its own AAC decoder can take the
 * Reed-Solomon-corrected Access Units instead. That caller is this: DAB+ carries
 * MPEG-4 HE-AAC, which is exactly what faad2's `NeAACDec*` decodes, and faad2 is
 * already in the binary for DRM. Nothing from fdk-aac is linked.
 *
 * The interface is the three calls the Rust side needs — open, configure from
 * the AudioSpecificConfig DAB+ carries, decode one Access Unit to interleaved
 * 16-bit PCM — kept in C so the Rust wrapper has no unsafe FFI of its own.
 */

#include <neaacdec.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

/* One decoder, opaque to the caller. */
typedef struct {
    NeAACDecHandle handle;
    unsigned long rate;
    unsigned char channels;
} dab_aac;

/* Open a decoder. NULL on failure (faad2 could not initialise). */
dab_aac *dab_aac_open(void) {
    dab_aac *d = calloc(1, sizeof(dab_aac));
    if (!d) return NULL;
    d->handle = NeAACDecOpen();
    if (!d->handle) {
        free(d);
        return NULL;
    }
    /* DAB+ is HE-AAC (SBR): ask for plain 16-bit output and leave implicit SBR
     * upsampling on — the default — so the audio comes out at the full rate the
     * superframe advertises rather than half of it. */
    NeAACDecConfigurationPtr cfg = NeAACDecGetCurrentConfiguration(d->handle);
    if (cfg) {
        cfg->outputFormat = FAAD_FMT_16BIT;
        NeAACDecSetConfiguration(d->handle, cfg);
    }
    return d;
}

/* Configure from the AudioSpecificConfig (the DAB+ format bytes). 0 on success. */
int dab_aac_init(dab_aac *d, const unsigned char *asc, unsigned long asc_len) {
    if (!d || !d->handle) return -1;
    long r = NeAACDecInit2(d->handle, (unsigned char *)asc, asc_len, &d->rate, &d->channels);
    return r < 0 ? -1 : 0;
}

/* Rate/channels learned at init. */
unsigned long dab_aac_rate(const dab_aac *d) { return d ? d->rate : 0; }
unsigned int dab_aac_channels(const dab_aac *d) { return d ? d->channels : 0; }

/* Decode one Access Unit.
 *
 * Returns the number of *frames* (channel-interleaved sample sets) written to
 * `out`, or -1 on error. `out` must hold at least `out_cap` int16 samples; the
 * caller sizes it generously (a DAB+ AU is at most 2048 frames × 2 ch × 2 for
 * SBR). The PCM is interleaved exactly as faad2 produces it.
 */
int dab_aac_decode(dab_aac *d, const unsigned char *au, unsigned long au_len,
                   int16_t *out, unsigned long out_cap) {
    if (!d || !d->handle) return -1;
    NeAACDecFrameInfo info;
    void *pcm = NeAACDecDecode(d->handle, &info, (unsigned char *)au, au_len);
    if (info.error > 0) return -1;
    unsigned long samples = info.samples; /* interleaved total */
    if (samples > out_cap) samples = out_cap;
    if (pcm && samples) {
        memcpy(out, pcm, samples * sizeof(int16_t));
    }
    /* `samples` is the interleaved count; report frames for a clean contract. */
    unsigned ch = info.channels ? info.channels : 1;
    return (int)(samples / ch);
}

void dab_aac_close(dab_aac *d) {
    if (!d) return;
    if (d->handle) NeAACDecClose(d->handle);
    free(d);
}
