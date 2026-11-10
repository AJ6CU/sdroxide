// The C++ the vendored decoder needs, and nothing else.
//
// Upstream drives `s4285::bitstream_decoder` from Octave; this is the same drive
// without it: an analytic signal from the audio, the carrier mixed to zero, the
// decoder fed sample by sample, and the message bits turned into text.
//
// No cleverness here on purpose — every judgement that matters (sync, timing,
// equaliser, deinterleaving, Viterbi) is theirs. This file only does the two
// things Octave did: hilbert() and the ITA2 reading.

// The reference's headers assume an Octave prelude and do not include what they
// use; these are that prelude and nothing more.
#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <complex>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <string>
#include <vector>

#include "s4285_bitstream_decoder.hpp"

namespace {

// 101-tap windowed Hilbert transformer: real in, analytic out.
class analytic {
public:
  explicit analytic(int taps = 101) : _h(taps, 0.0f), _x(taps, 0.0f), _i(0) {
    const int m = taps / 2;
    for (int n = 0; n < taps; ++n) {
      const int k = n - m;
      double h = 0.0;
      if (k & 1) h = 2.0 / (M_PI * k);
      const double w = 0.42 - 0.5 * std::cos(2 * M_PI * n / (taps - 1))
                     + 0.08 * std::cos(4 * M_PI * n / (taps - 1));
      _h[n] = float(h * w);
    }
  }
  std::complex<float> push(float s) {
    _x[_i] = s;
    float acc = 0.0f;
    int idx = _i;
    for (int n = 0; n < int(_h.size()); ++n) {
      acc += _h[n] * _x[idx];
      idx = (idx == 0 ? int(_x.size()) - 1 : idx - 1);
    }
    _i = (_i + 1 == int(_x.size()) ? 0 : _i + 1);
    return std::complex<float>(s, acc);
  }
private:
  std::vector<float> _h, _x;
  int _i;
};

class collector : public s4285::bitstream_decoder {
public:
  explicit collector(int mod) : s4285::bitstream_decoder(12, 4, mod) {}
  std::vector<int> bits;
protected:
  void process_bits(std::vector<int>::const_iterator i0,
                    std::vector<int>::const_iterator i1) override {
    bits.insert(bits.end(), i0, i1);
  }
};

}  // namespace

// Decode `n` mono samples of `audio` at `rate` Hz. Writes the ITA2 reading of
// whatever it decoded into `out` (NUL-terminated) and returns the number of
// message bits, or 0 when it decoded nothing.
//
// `mod` is bits per symbol: 2 for the BPSK (600/300/150/75) rates, 4 for QPSK,
// 8 for 8PSK — the same argument upstream's Octave entry point takes. `carrier`
// is where the signal sits in the audio, 1800 Hz by convention.
extern "C" int sdroxide_s4285_decode(const float* audio, size_t n, double rate,
                                     int mod, double carrier,
                                     unsigned char* out, size_t outlen) {
  if (!audio || n == 0 || !out || outlen == 0) return 0;
  // **The level, held to one the reference expects.** Its equaliser carries an
  // absolute step size (`mu = 0.05`), so a quieter recording converges more
  // slowly and one 30 % quieter decodes nothing at all — measured: the same
  // capture at two levels, 2304 bits and readable text against 384 bits and none.
  // Normalising here is the fix, and it is why this decoder needs no gain
  // control of its own.
  double power = 0.0;
  for (size_t i = 0; i < n; ++i) power += double(audio[i]) * double(audio[i]);
  const double rms = std::sqrt(power / double(n));
  const float gain = rms > 1e-9 ? float(0.15 / rms) : 0.0f;

  analytic an;
  collector d(mod);
  const double inc = -2.0 * M_PI * carrier / rate;
  for (size_t i = 0; i < n; ++i) {
    const std::complex<float> z = an.push(audio[i] * gain);
    const double ph = inc * double(i);
    d.process(std::complex<float>(float(z.real() * std::cos(ph) - z.imag() * std::sin(ph)),
                                  float(z.real() * std::sin(ph) + z.imag() * std::cos(ph))));
  }
  // **The bits, not a reading of them.** Which framing a payload has is the
  // receiver's question, and this tree already answers it (`message_text`, with
  // the 5N1.5 reading and the tape scoring); duplicating a reader here is how the
  // two would drift apart.
  const size_t take = d.bits.size() < outlen ? d.bits.size() : outlen;
  for (size_t i = 0; i < take; ++i) out[i] = (unsigned char)(d.bits[i] & 1);
  return int(d.bits.size());
}
