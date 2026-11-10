// AudioWorklet processors for the sdroxide web client.

// **Three units live in this file, and confusing them is what the first
// reader did.**
//
//   * source frames -- mono samples at 48 kHz: what the server sends, what the
//     ring holds, and the unit of every backlog, target and threshold below.
//   * output frames -- mono samples at the context's own `sampleRate`: what
//     `process` writes and what `rendered` counts.
//   * milliseconds -- only in what a person reads.
//
// The reader regulates a *backlog*: it aims to hold TARGET_SRC source frames
// between what it was handed and what it has played. It does that in three
// separate ways, on purpose:
//
//   1. a slow clock correction, +/-0.05 %, which removes the drift between the
//      server's audio clock and the sound card's. Inaudible (about one cent),
//      and far more than any real clock mismatch needs.
//   2. an *edit* for a burst the machine caused -- a frame that took too long,
//      a lump of audio delivered at once. The excess above the target is
//      removed in one place with a 5 ms equal-power crossfade, so nothing is
//      pitched and no cut is heard.
//   3. a *resync* when the stream is not a burst but a discontinuity: more than
//      RESYNC_EXCESS_SRC above the target is treated as audio that is simply
//      old. Past that size the audio to blend with is no longer in the ring --
//      the writer has overwritten it -- so the reader fades out, jumps, and
//      fades in: a 10 ms dip, which is audibly a gap rather than a cut.
//
// The ring is bounded at 500 ms and nothing in `process` allocates.

const SRC_HZ = 48_000;

// How much source audio the ring holds: 500 ms, 96 kB, and far above any
// target or threshold below.
const CAP_SRC = 24_000;

// The backlog the reader aims to hold. A lump per frame arrives every 20 to
// 50 ms on the harness's jittered cadence, so the level has to clear the worst
// gap: at 60 ms that run underran two or three times in twenty seconds, and at
// 80 ms it did not. A product choice, not a constant of nature; the README has
// the runs that chose it.
const TARGET_SRC = 3_840; // 80 ms

// Excess *above the target*, not total depth. Above the first, an edit; above
// the second, a resync. With the target, both stay inside the ring:
// 60 + 300 = 360 ms < 500 ms.
const EDIT_EXCESS_SRC = 5_760; // 120 ms
const RESYNC_EXCESS_SRC = 14_400; // 300 ms

// Clock correction. The error is measured over ~1.3 s of audio (the fill is
// noisy: audio arrives in lumps), 10 ms of it is ignored, and 40 ms of it asks
// for the full +/-0.05 %. That is about 0.9 cent, inaudible, and 500 times more
// than a real sound card's mismatch.
const DRIFT_MAX = 5e-4;
const DRIFT_FULL_SCALE_SRC = 1_920; // 40 ms
const FILL_DEADBAND_SRC = 480; // 10 ms
const FILL_SMOOTHING = 0.002;

// The interpolator: an 8-point Lagrange kernel, in a table of 1024 fractional
// positions. What the app needs is a *flat passband*, not an anti-alias filter:
// at 44.1 kHz anything above 22.05 kHz folds back, and every source on this
// path is already band-limited below that -- SSB 3 kHz, AM 6, WFM 15 (a
// 383-tap 15 kHz low-pass in `WfmDemod`, sdroxide-dsp), DAB and DRM about
// 20 kHz from their codecs. So the kernel is chosen for its passband, not for
// a stopband there is nothing to stop: eight points hold 15 kHz within half a
// decibel where a 4-point cubic loses more than one, and above 22.05 kHz the
// kernel attenuates by only about 4.5 dB -- measured, and written down in the
// harness's README, so the day a source does carry that band it is a known
// limit rather than a surprise.
const PHASES = 1024;
const TAPS = 8;

// Fades, in milliseconds. The crossfade is the same length in time whatever the
// context runs at.
const FADE_MS = 5;

class PcmPlayer extends AudioWorkletProcessor {
    constructor() {
        super();
        this.cap = CAP_SRC;
        this.ring = new Float32Array(this.cap);
        // Source frames written (integer) and read (fractional). Both are
        // monotonic since the last reset; the ring index is `n % cap`.
        this.w = 0;
        this.r = 0;
        this.primed = false;
        this.avg = TARGET_SRC;
        this.target = TARGET_SRC;

        this.xfadeLen = Math.max(1, Math.round((FADE_MS / 1000) * sampleRate));
        // Headroom the writer leaves unread, so a fade-out always has untouched
        // audio to play: with a full ring the writer is one slot ahead of the
        // reader and overwrites what the fade is reading.
        this.reserve = Math.max(480, 4 * this.xfadeLen);
        this.fadeLen = this.xfadeLen;
        this.fade = 0; // remaining fade-in, in output frames
        this.fadeOutSrc = Math.max(1, Math.round((FADE_MS / 1000) * SRC_HZ));

        // A crossfade in progress: `skip` source frames have already been
        // dropped from the read position, so the blend is between the old
        // stream at `r - skip` and the new one at `r`.
        this.xf = 0;
        this.skip = 0;
        this.wLimit = 0;

        // A reset asked for from the frame side: played out with a short fade
        // before the ring is dropped, so it is not a cut to silence.
        this.resetPending = false;
        this.resetFade = 0;
        this.resetFadeLen = this.xfadeLen;

        // A resync played as a fade-out, a jump, and a fade-in: after a jump of
        // more than a few hundred milliseconds there is no old audio left in the
        // ring to blend with, and blending with whatever has replaced it is a
        // cut wearing a fade's clothes.
        this.outFade = 0;
        this.pendingSkip = 0;

        // What reached the graph, and what happened on the way. Reported to the
        // page; `rendered` is also the page's evidence that the audio thread is
        // alive.
        this.rendered = 0;
        this.blocks = 0;
        this.underruns = 0;
        this.edits = 0;
        this.resyncs = 0;
        this.oversized = 0;
        this.overwritten = 0;
        this.resets = 0;
        this.wraps = 0;
        this.driftPpm = 0;

        // Lagrange weights for every fractional position, built once. The
        // nodes are -3..4, so `TAPS` neighbours around the whole part.
        this.table = new Float32Array(PHASES * TAPS);
        {
            const nodes = [-3, -2, -1, 0, 1, 2, 3, 4];
            const bw = nodes.map((j) => {
                let p = 1;
                for (const m of nodes) if (m !== j) p *= j - m;
                return 1 / p;
            });
            for (let ph = 0; ph < PHASES; ph++) {
                const t = (ph + 0.5) / PHASES;
                let sum = 0;
                for (let j = 0; j < TAPS; j++) {
                    const v = bw[j] / (t - nodes[j]);
                    this.table[ph * TAPS + j] = v;
                    sum += v;
                }
                for (let j = 0; j < TAPS; j++) this.table[ph * TAPS + j] /= sum;
            }
        }

        this.port.onmessage = (ev) => {
            const m = ev.data;
            if (!m) return;
            if (m.reset) {
                this.reset();
                return;
            }
            this.write(m);
        };
    }

    /// Drop everything, after the fade the process loop plays. The next audio
    /// primes afresh.
    reset() {
        this.resetPending = true;
    }

    /// Take source frames into the ring. What does not fit is dropped and
    /// counted -- the head of the ring is never overwritten, because that is
    /// audio the reader is about to play, and scrubbing the read position is a
    /// click. The reader's own resync then clears the room.
    write(src) {
        let n = src.length;
        if (n === 0) return;
        if (n > this.cap) {
            this.oversized++;
            n = this.cap;
        }
        const free = Math.max(0, this.cap - Math.ceil(this.w - this.r) - this.reserve);
        if (n > free) {
            this.overwritten += n - free;
            n = free;
        }
        if (n <= 0) return;
        // The newest of what was offered is the part worth keeping.
        const from = src.length - n;
        const at = this.w % this.cap;
        const first = Math.min(n, this.cap - at);
        this.ring.set(src.subarray(from, from + first), at);
        if (first < n) this.ring.set(src.subarray(from + first, from + n), 0);
        this.w += n;
        if (first < n) this.wraps++;
    }

    /// One source frame at absolute position `p`, interpolated with the
    /// 8-point Lagrange table. A whole position -- the case a 48 kHz context
    /// takes when no correction is applied -- is a plain copy.
    ///
    /// A position before the stream began is **silence**, never a wrapped
    /// sample: the modulo makes an address valid, it does not make the data
    /// behind it belong to this stream.
    sample(p) {
        if (p < 0) return 0;
        const cap = this.cap;
        const ring = this.ring;
        const k = Math.floor(p);
        const t = p - k;
        const b = ring[((k % cap) + cap) % cap];
        if (t === 0) return b;
        if (k < 3 || k + 4 >= this.wLimit) {
            // Near an edge, where the 8-point window is not there: linear from
            // what is. Only the first instants after a prime, the tail of an
            // underrun, and a crossfade whose old side starts before the ring
            // take this path -- and the index is wrapped here too, because a
            // negative one reads past the array and poisons the blend.
            if (k + 1 >= this.wLimit) return b;
            const c = ring[(((k + 1) % cap) + cap) % cap];
            return b + (c - b) * t;
        }
        const off = Math.min(PHASES - 1, (t * PHASES) | 0) * TAPS;
        const tbl = this.table;
        let acc = 0;
        for (let j = 0; j < TAPS; j++) {
            const idx = k - 3 + j;
            acc += tbl[off + j] * ring[((idx % cap) + cap) % cap];
        }
        return acc;
    }

    process(_inputs, outputs) {
        const out = outputs[0][0];
        const n = out.length;
        this.rendered += n;
        if (++this.blocks >= 64) {
            this.blocks = 0;
            this.report();
        }

        let w = this.w;
        let r = this.r;
        this.wLimit = w;

        let primedNow = false;
        if (!this.primed) {
            if (w - r >= this.target) {
                // Too far behind to be a burst: keep only the target, before
                // anything is played, so the fade-in starts from silence.
                if (w - r - this.target > RESYNC_EXCESS_SRC) {
                    r = w - this.target;
                    this.resyncs++;
                }
                // Whole source frames: the only state in which the 48 kHz path
                // copies rather than interpolates. Four past the start, so the
                // 8-point window behind the read position exists.
                r = Math.max(4, Math.ceil(r));
                this.primed = true;
                // The correction starts from the level the prime actually left,
                // not from the target: a lump overshoots, and starting the loop
                // at the wrong place makes it swing.
                this.avg = w - r;
                this.fade = this.fadeLen;
                primedNow = true;
            } else {
                out.fill(0);
                this.r = r;
                return true;
            }
        }

        // A slow correction towards the target, from the smoothed fill. One
        // value per quantum: the position advances by `step` per output frame.
        const fill = w - r;
        this.avg += (fill - this.avg) * FILL_SMOOTHING;
        const err = this.avg - this.target;
        const drift =
            Math.abs(err) < FILL_DEADBAND_SRC
                ? 0
                : Math.max(
                      -DRIFT_MAX,
                      Math.min(DRIFT_MAX, (err * DRIFT_MAX) / DRIFT_FULL_SCALE_SRC),
                  );
        const step = (SRC_HZ / sampleRate) * (1 + drift);
        this.driftPpm = Math.round(drift * 1e6);

        // One planned correction per quantum, never during a fade.
        if (!primedNow && this.outFade === 0 && this.xf === 0) {
            const excess = w - r - this.target;
            if (excess > EDIT_EXCESS_SRC) {
                // A crossfade blends the next few milliseconds of the stream
                // being dropped with the one that replaces it. It needs the
                // audio *behind* the read position for its window, and only the
                // ring's last `cap` written frames exist: before that the stream
                // never existed, and a wrapped index would return a plausible
                // sample from an older generation. With that history, blend;
                // without it, or past RESYNC_EXCESS_SRC, fade out, jump, and
                // fade in -- a transition that reads only the current position,
                // which is valid by definition. The counters name the
                // transition, not the cause.
                const historyStart = Math.max(0, w - this.cap);
                if (r - 3 >= historyStart && excess <= RESYNC_EXCESS_SRC) {
                    r += excess;
                    this.skip = excess;
                    this.xf = this.xfadeLen;
                    // The blend is the transition: a prime's fade-in still
                    // counting down here would keep attenuating the audio
                    // *after* the fade, once the blend has finished.
                    this.fade = 0;
                    this.edits++;
                } else {
                    this.outFade = this.fadeLen;
                    this.pendingSkip = excess;
                    this.resyncs++;
                }
            }
        }

        if (this.resetPending) {
            if (this.resetFade <= 0) this.resetFade = this.resetFadeLen;
        }

        let i = 0;
        while (i < n) {
            if (this.resetFade > 0) {
                // Play the last few milliseconds out before dropping the ring.
                this.resetFade--;
                const avail = w - r;
                const v = avail >= 1 ? this.sample(r) : 0;
                out[i] = v * (this.resetFade / this.resetFadeLen);
                r += step;
                i++;
                continue;
            }
            if (this.resetPending) {
                // The fade is done: drop everything and wait for a fresh prime.
                this.resetPending = false;
                this.primed = false;
                this.fade = 0;
                this.xf = 0;
                this.skip = 0;
                this.avg = this.target;
                this.resets++;
                w = 0;
                r = 0;
                this.wLimit = 0;
                out.fill(0, i);
                break;
            }
            const avail = w - r;
            if (this.outFade > 0) {
                // Fade the stream out, then jump and fade in: the resync's
                // 10 ms dip, which is audibly a gap and not a cut.
                this.outFade--;
                const v = avail >= 1 ? this.sample(r) : 0;
                out[i] = v * (this.outFade / this.fadeLen);
                r += step;
                if (this.outFade === 0) {
                    r += this.pendingSkip;
                    this.pendingSkip = 0;
                    this.fade = this.fadeLen;
                }
                i++;
                continue;
            } else if (this.xf > 0) {
                // Equal-power blend between the old stream (r - skip) and the
                // new one (r). Both advance together, so nothing jumps at
                // either end of the fade.
                const a = 1 - this.xf / this.xfadeLen;
                const g = (a * Math.PI) / 2;
                const oldp = r - this.skip;
                const va = this.sample(oldp);
                const vb = this.sample(r);
                out[i] = Math.cos(g) * va + Math.sin(g) * vb;
                this.xf--;
            } else {
                if (avail < 1) {
                    out.fill(0, i);
                    this.primed = false;
                    this.underruns++;
                    break;
                }
                let v = this.sample(r);
                if (this.fade > 0) {
                    v *= 1 - this.fade / this.fadeLen;
                    this.fade--;
                }
                if (avail < this.fadeOutSrc) v *= avail / this.fadeOutSrc;
                out[i] = v;
            }
            r += step;
            i++;
        }

        this.w = w;
        this.r = r;
        return true;
    }

    report() {
        this.port.postMessage({
            rendered: this.rendered,
            // Source frames still to play: the reader's backlog. Same unit as
            // before this file was rewritten, so `health()` and the harness
            // keep reading the same number.
            queued: Math.max(0, Math.round(this.w - this.r)),
            capSrc: this.cap,
            primed: this.primed,
            targetSrc: this.target,
            underruns: this.underruns,
            edits: this.edits,
            resyncs: this.resyncs,
            oversized: this.oversized,
            overwritten: this.overwritten,
            resets: this.resets,
            wraps: this.wraps,
            driftPpm: this.driftPpm,
        });
    }
}

// Forwards mic input blocks to the main thread.
class MicCapture extends AudioWorkletProcessor {
    process(inputs, _outputs) {
        const input = inputs[0];
        if (input.length > 0 && input[0].length > 0) {
            this.port.postMessage(new Float32Array(input[0]));
        }
        return true;
    }
}

registerProcessor("pcm-player", PcmPlayer);
registerProcessor("mic-capture", MicCapture);
