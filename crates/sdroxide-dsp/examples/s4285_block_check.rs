//! The interleaver the other implementation uses: **blocks of 32 symbols, delays
//! counted in blocks.**
//!
//!     cargo run --release -p sdroxide-dsp --example s4285_block_check -- \
//!         ~/s4285-captures/keepers/6348.000kHz-20261022-044640Z.wav [carrier_hz]
//!
//! This fork's `ConvInterleaver` advances its commutator **per symbol**: 32 rows,
//! row `r` holding `(31-r)·j` symbols, written once every 32 symbols. The
//! independent implementation in `luwangg/signal-analysis` does something
//! structurally different — `insert(i0, i0+32)` takes a whole **32-symbol data
//! segment** at a time, puts one symbol into each row, and sizes the rows in
//! `incr*(31-i)` **blocks**, not symbols. Both are self-consistent, so a loopback
//! through either passes; on a real transmission only one of them can be right,
//! and the difference is a factor of 32 in every delay.
//!
//! The judge is a framed message, because the metric-versus-null ratio reads 1.02×
//! on a signal this receiver decodes and is therefore not evidence.

use sdroxide_dsp::stanag4285::{EqMode, S4285Event, S4285Receiver};
use sdroxide_dsp::stanag4285_fec as fec;

fn read_wav(path: &str) -> (Vec<f32>, f64) {
    let mut reader = hound::WavReader::open(path).expect("open the wav");
    let spec = reader.spec();
    let raw: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.expect("s")).collect(),
        hound::SampleFormat::Int => match spec.bits_per_sample {
            16 => reader.samples::<i16>().map(|s| s.expect("s") as f32 / 32_768.0).collect(),
            _ => reader.samples::<i32>().map(|s| s.expect("s") as f32 / 2_147_483_648.0).collect(),
        },
    };
    let ch = spec.channels.max(1) as usize;
    let audio = if ch == 1 {
        raw
    } else {
        raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
    };
    (audio, spec.sample_rate as f64)
}

/// The other implementation's deinterleaver, exactly: `incr`-sized rows measured
/// in blocks of 32, natural write order, `(9·i) mod 32` read order, and the
/// circular buffer's read happening *after* the write so a row delays by its own
/// length plus one.
struct BlockDeinterleaver {
    rows: Vec<Vec<f32>>,
    pos: Vec<usize>,
    incr: usize,
}

impl BlockDeinterleaver {
    fn new(incr: usize) -> Self {
        let rows = (0..32).map(|i| vec![0.0f32; incr * (31 - i) + 1]).collect();
        BlockDeinterleaver { rows, pos: vec![0; 32], incr }
    }

    /// One 32-symbol data segment in, 32 values out.
    fn insert(&mut self, block: &[f32]) -> [f32; 32] {
        for i in 0..32 {
            let len = self.rows[i].len();
            let p = self.pos[i];
            self.rows[i][p] = block[i];
            self.pos[i] = (p + 1) % len;
        }
        let mut out = [0.0f32; 32];
        for (i, slot) in out.iter_mut().enumerate() {
            let r = (9 * i) & 31;
            *slot = self.rows[r][self.pos[r]];
        }
        let _ = self.incr;
        out
    }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        eprintln!("usage: s4285_block_check <wav> [carrier_hz]");
        std::process::exit(2);
    }
    let (audio, rate) = read_wav(&a[0]);
    let carrier: f64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(1800.0);

    let mut rx = S4285Receiver::new(rate, carrier);
    rx.set_eq_mode(EqMode::Auto);
    rx.trace_coded_stream(true);
    let mut frames = 0;
    for chunk in audio.chunks(4096) {
        for ev in rx.process(chunk) {
            if let S4285Event::Frame(_) = ev {
                frames += 1;
            }
        }
    }
    let traced: Vec<f32> = rx.coded_stream().to_vec();
    println!("{}: {frames} frames, {} traced values at {carrier}", a[0], traced.len());

    let mut found = 0usize;
    let mut reported = 0usize;
    for rate in fec::CodedRate::BPSK {
        // One bit per symbol for the BPSK rates; the trace carries two.
        let soft: Vec<f32> = traced.iter().step_by(2).copied().collect();
        let times = rate.repeat();
        for incr in [6usize, 12, 24, 48] {
            for sign in [1.0f32, -1.0] {
                let mut di = BlockDeinterleaver::new(incr);
                let mut stream: Vec<f32> = Vec::with_capacity(soft.len());
                for chunk in soft.chunks_exact(32) {
                    let block: Vec<f32> = chunk.iter().map(|v| v * sign).collect();
                    stream.extend_from_slice(&di.insert(&block));
                }
                let pairs = fec::average_pairs(&stream, times);
                let bits = fec::viterbi_decode(&pairs);
                let mut fr = fec::MessageFramer::new(rate);
                let mut got = 0usize;
                let mut closed = 0usize;
                for b in bits {
                    if let Some(m) = fr.push(b) {
                        got += 1;
                        if m.closed {
                            closed += 1;
                        }
                        if got > 8 {
                            break;
                        }
                    }
                }
                if got > 0 || fr.best_som >= 30 {
                    reported += 1;
                    println!(
                        "  {} incr {incr} sign {sign:+.0}: {got} message(s), {closed} closed, best SOM {}/32",
                        rate.label(),
                        fr.best_som
                    );
                }
                found += closed;
            }
        }
    }
    if reported == 0 {
        println!("  no branch framed a message or came within 30/32 of the SOM");
    }
    println!("  closed messages in total: {found}");
}
