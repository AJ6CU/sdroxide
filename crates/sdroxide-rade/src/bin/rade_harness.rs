//! `rade-harness` — drive the RADE wrapper over stored files.
//!
//! No engine, no UI, no threads: this is the bench rig for the wrapper itself.
//! The feature dumps it writes are the same raw little-endian `f32` blocks
//! rade_c's own `rade_rx_wav -f` / `rade_tx_wav -f` produce, so a decode can be
//! byte-compared against upstream:
//!
//! ```text
//! vendor/rade_c/build/src/rade_tx_wav -f tx_ref.f32 speech16k.wav modem8k.wav
//! vendor/rade_c/build/src/rade_rx_wav -f rx_ref.f32 modem8k.wav out16k.wav
//! rade-harness rx --input modem8k.wav --features rx.f32 --speech out.wav
//! cmp rx_ref.f32 rx.f32
//! ```

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use num_complex::Complex32;
use sdroxide_dsp::{ComplexResampler, MonoResampler};
use sdroxide_rade::{
    MODEM_RATE, RX_REAL_SCALE, Rade, SPEECH_RATE, VocoderDec, VocoderEnc, vocoder,
};

#[derive(Parser)]
#[command(name = "rade-harness", about = "Decode/encode RADE V1 through the Rust wrapper")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Demodulate a modem WAV into features and (optionally) speech.
    Rx {
        /// Modem audio: 16-bit mono WAV, any sample rate (resampled to 8 kHz).
        #[arg(long)]
        input: PathBuf,
        /// Write raw f32 feature vectors here.
        #[arg(long)]
        features: Option<PathBuf>,
        /// Write synthesized 16 kHz speech here.
        #[arg(long)]
        speech: Option<PathBuf>,
        /// Write a per-call CSV of nin/sync/SNR/offset here.
        #[arg(long)]
        stats: Option<PathBuf>,
    },
    /// Modulate a speech WAV into a raw interleaved CF32 IQ file, for feeding
    /// the app's `--file` source.
    ///
    /// RADE's modem output is already the analytic (single-sideband) signal —
    /// its real part is what goes on the air — so resampling it up to the
    /// device rate gives a USB signal sitting at the modem's own audio offset
    /// above the tuned frequency, which is exactly what the receiver expects.
    Iq {
        /// Speech: 16-bit mono WAV, any sample rate (resampled to 16 kHz).
        #[arg(long)]
        input: PathBuf,
        /// Write the interleaved f32 I/Q here.
        #[arg(long)]
        output: PathBuf,
        /// Device sample rate to produce.
        #[arg(long, default_value_t = 48_000.0)]
        rate: f64,
        /// Amplitude of the modem signal, 0..1 full scale.
        #[arg(long, default_value_t = 0.25)]
        amplitude: f32,
    },
    /// Modulate a speech WAV into a RADE V1 modem WAV.
    Tx {
        /// Speech: 16-bit mono WAV, any sample rate (resampled to 16 kHz).
        #[arg(long)]
        input: PathBuf,
        /// Write the 8 kHz modem signal here.
        #[arg(long)]
        output: PathBuf,
        /// Write the raw f32 feature vectors fed to the modem here.
        #[arg(long)]
        features: Option<PathBuf>,
        /// Skip the End-of-Over frame (upstream's rade_tx_wav sends one).
        #[arg(long)]
        no_eoo: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Rx { input, features, speech, stats } => rx(&input, features, speech, stats),
        Cmd::Tx { input, output, features, no_eoo } => tx(&input, &output, features, no_eoo),
        Cmd::Iq { input, output, rate, amplitude } => iq(&input, &output, rate, amplitude),
    }
}

fn iq(input: &Path, output: &Path, rate: f64, amplitude: f32) -> Result<()> {
    let (audio, in_rate) = read_wav(input)?;
    let speech = resample(&audio, in_rate, SPEECH_RATE);

    let mut rade = Rade::open_v1()?;
    let mut enc = VocoderEnc::new()?;
    let frame = enc.frame_size();
    let n_block = rade.n_features();
    let (mut feats, mut iq, mut pcm) = (Vec::new(), Vec::new(), Vec::new());

    for chunk in speech.chunks_exact(frame) {
        pcm.clear();
        pcm.extend(chunk.iter().copied().map(sdroxide_rade::f32_to_i16));
        enc.encode(&pcm, &mut feats)?;
        while feats.len() >= n_block {
            let block: Vec<f32> = feats.drain(..n_block).collect();
            rade.tx(&block, &mut iq)?;
        }
    }
    rade.tx_eoo(&mut iq)?;

    let mut up = Vec::new();
    match ComplexResampler::new(MODEM_RATE, rate) {
        Some(mut r) => r.push(&iq, &mut up),
        None => up.extend_from_slice(&iq),
    }

    let mut f = BufWriter::new(File::create(output)?);
    for z in &up {
        f.write_all(&(z.re * amplitude).to_le_bytes())?;
        f.write_all(&(z.im * amplitude).to_le_bytes())?;
    }
    f.flush()?;
    eprintln!(
        "wrote {} — {} IQ samples @ {rate} Hz ({:.1} s)",
        output.display(),
        up.len(),
        up.len() as f64 / rate
    );
    Ok(())
}

fn rx(
    input: &Path,
    features_path: Option<PathBuf>,
    speech_path: Option<PathBuf>,
    stats_path: Option<PathBuf>,
) -> Result<()> {
    let (audio, rate) = read_wav(input)?;
    eprintln!("input: {} samples @ {rate} Hz ({:.1} s)", audio.len(), audio.len() as f64 / rate);

    let modem = resample(&audio, rate, MODEM_RATE);
    eprintln!("modem: {} samples @ {MODEM_RATE} Hz", modem.len());

    let mut rade = Rade::open_v1()?;
    let mut dec = VocoderDec::new()?;
    let n_feat = vocoder::n_features();

    let mut features_out = features_path.map(File::create).transpose()?.map(BufWriter::new);
    let mut stats_out = stats_path.map(File::create).transpose()?.map(BufWriter::new);
    if let Some(f) = stats_out.as_mut() {
        writeln!(f, "call,pos,nin,n_features,has_eoo,sync,snr_db,freq_offset_hz")?;
    }

    let mut feats = Vec::new();
    let mut eoo_bits = Vec::new();
    let mut pcm = Vec::new();
    let mut pos = 0usize;
    let mut call = 0usize;
    let (mut valid, mut eoo_count) = (0usize, 0usize);
    let mut snr_sum = 0.0f64;

    while pos < modem.len() {
        let nin = rade.nin();
        // Zero-pad the final short block so the last modem frame can flush,
        // matching rade_rx_wav.
        let mut block = vec![Complex32::default(); nin];
        let take = nin.min(modem.len() - pos);
        for (dst, &src) in block.iter_mut().zip(&modem[pos..pos + take]) {
            *dst = Complex32::new(src * RX_REAL_SCALE, 0.0);
        }
        pos += if take < nin { modem.len() - pos } else { nin };

        let out = rade.rx(&block, &mut feats, &mut eoo_bits)?;
        if out.has_eoo {
            eoo_count += 1;
            match sdroxide_rade::text::decode(&eoo_bits) {
                Some(c) => eprintln!("end-of-over at call {call}: callsign {c}"),
                None => eprintln!("end-of-over at call {call}: no callsign"),
            }
        }
        if out.n_features > 0 {
            valid += 1;
            snr_sum += rade.snr_3k_db() as f64;
            if let Some(f) = features_out.as_mut() {
                for v in &feats {
                    f.write_all(&v.to_le_bytes())?;
                }
            }
            for frame in feats.chunks_exact(n_feat) {
                dec.decode(frame, &mut pcm)?;
            }
        }
        if let Some(f) = stats_out.as_mut() {
            writeln!(
                f,
                "{call},{pos},{nin},{},{},{},{:.2},{:.2}",
                out.n_features,
                out.has_eoo as u8,
                rade.sync() as u8,
                rade.snr_3k_db(),
                rade.freq_offset_hz(),
            )?;
        }
        call += 1;
    }

    let mean_snr = if valid > 0 { snr_sum / valid as f64 } else { 0.0 };
    eprintln!(
        "calls: {call}  valid: {valid}  eoo: {eoo_count}  mean SNR: {mean_snr:.1} dB  \
         speech: {} samples ({:.1} s)",
        pcm.len(),
        pcm.len() as f64 / SPEECH_RATE
    );
    if let Some(p) = speech_path {
        write_wav(&p, &pcm, SPEECH_RATE as u32)?;
        eprintln!("wrote {}", p.display());
    }
    if valid == 0 {
        bail!("no frames decoded — the input never reached sync");
    }
    Ok(())
}

fn tx(input: &Path, output: &Path, features_path: Option<PathBuf>, no_eoo: bool) -> Result<()> {
    let (audio, rate) = read_wav(input)?;
    eprintln!("input: {} samples @ {rate} Hz ({:.1} s)", audio.len(), audio.len() as f64 / rate);

    let speech = resample(&audio, rate, SPEECH_RATE);
    let mut rade = Rade::open_v1()?;
    let mut enc = VocoderEnc::new()?;
    let frame = enc.frame_size();
    let n_block = rade.n_features();

    let mut features_out = features_path.map(File::create).transpose()?.map(BufWriter::new);
    let mut feats = Vec::new();
    let mut iq = Vec::new();
    let mut pcm = Vec::with_capacity(frame);

    let emit = |block: &[f32],
                rade: &mut Rade,
                iq: &mut Vec<Complex32>,
                out: &mut Option<BufWriter<File>>|
     -> Result<()> {
        if let Some(f) = out.as_mut() {
            for v in block {
                f.write_all(&v.to_le_bytes())?;
            }
        }
        rade.tx(block, iq)?;
        Ok(())
    };

    for chunk in speech.chunks_exact(frame) {
        pcm.clear();
        pcm.extend(chunk.iter().copied().map(sdroxide_rade::f32_to_i16));
        enc.encode(&pcm, &mut feats)?;
        while feats.len() >= n_block {
            let block: Vec<f32> = feats.drain(..n_block).collect();
            emit(&block, &mut rade, &mut iq, &mut features_out)?;
        }
    }
    // Zero-pad the last partial block so the tail of the speech still goes out,
    // as rade_tx_wav does.
    if !feats.is_empty() {
        feats.resize(n_block, 0.0);
        let block = std::mem::take(&mut feats);
        emit(&block, &mut rade, &mut iq, &mut features_out)?;
    }

    let frames = iq.len() / rade.n_tx_out();
    if !no_eoo {
        rade.tx_eoo(&mut iq)?;
    }
    eprintln!("modem frames: {frames}{}", if no_eoo { "" } else { " + EOO" });

    // int16 conversion matching rade_tx_wav's write_iq_real().
    let modem: Vec<i16> = iq
        .iter()
        .map(|z| {
            let v = (z.re * sdroxide_rade::INT16_SCALE).clamp(-32767.0, 32767.0);
            (0.5 + v as f64).floor() as i16
        })
        .collect();
    write_wav(output, &modem, MODEM_RATE as u32)?;
    eprintln!(
        "wrote {} ({:.1} s @ {MODEM_RATE} Hz)",
        output.display(),
        modem.len() as f64 / MODEM_RATE
    );
    Ok(())
}

/// Read a 16-bit mono WAV as `+/-1.0` floats, returning the samples and rate.
fn read_wav(path: &Path) -> Result<(Vec<f32>, f64)> {
    let mut r = hound::WavReader::open(path).with_context(|| format!("open {}", path.display()))?;
    let spec = r.spec();
    if spec.channels != 1 {
        bail!("{}: expected mono, got {} channels", path.display(), spec.channels);
    }
    if spec.sample_format != hound::SampleFormat::Int || spec.bits_per_sample != 16 {
        bail!("{}: expected 16-bit PCM", path.display());
    }
    let samples =
        r.samples::<i16>().map(|s| s.map(|v| v as f32 / 32768.0)).collect::<Result<Vec<_>, _>>()?;
    Ok((samples, spec.sample_rate as f64))
}

fn write_wav(path: &Path, pcm: &[i16], rate: u32) -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut w = hound::WavWriter::create(path, spec)
        .with_context(|| format!("create {}", path.display()))?;
    for &s in pcm {
        w.write_sample(s)?;
    }
    w.finalize()?;
    Ok(())
}

fn resample(input: &[f32], from: f64, to: f64) -> Vec<f32> {
    match MonoResampler::new(from, to) {
        Some(mut r) => {
            let mut out = Vec::with_capacity((input.len() as f64 * to / from) as usize);
            r.push(input, &mut out);
            out
        }
        None => input.to_vec(),
    }
}
