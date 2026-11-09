//! Run the decoder on a complex-float32 IQ recording of one VOR channel:
//! `cargo run --release -p sdroxide-vor --example iqfile -- FILE RATE_HZ [OFFSET_HZ]`.
//! The AM detector is the magnitude of each sample.

use std::io::Read;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("file");
    let rate: u32 = args.next().expect("rate").parse().expect("rate in Hz");
    let offset: f64 = args.next().map_or(0.0, |s| s.parse().expect("offset in Hz"));
    let mut bytes = Vec::new();
    std::fs::File::open(&path).expect("open").read_to_end(&mut bytes).expect("read");
    let mut d = sdroxide_vor::VorDecoder::new(rate).expect("rate");
    let mut audio = Vec::with_capacity(rate as usize);
    let (mut n, mut last_s) = (0u64, 0u64);
    let mut radials: Vec<f32> = Vec::new();
    for c in bytes.chunks_exact(8) {
        let (mut re, mut im) =
            (f32::from_le_bytes(c[..4].try_into().unwrap()), f32::from_le_bytes(c[4..].try_into().unwrap()));
        if offset != 0.0 {
            let (s, co) = (-std::f64::consts::TAU * offset * n as f64 / rate as f64).sin_cos();
            let (r, i) = (re as f64, im as f64);
            re = (r * co - i * s) as f32;
            im = (r * s + i * co) as f32;
        }
        audio.push(re.hypot(im));
        n += 1;
        if audio.len() == 4096 {
            let up = d.push(&audio);
            audio.clear();
            if let Some(id) = up.new_ident {
                println!("{:7.1}s ident {id}", n as f64 / rate as f64);
            }
            if n / rate as u64 != last_s {
                last_s = n / rate as u64;
                if let Some(r) = up.reading.or(d.reading()) {
                    if last_s % 5 == 0 {
                        println!("{last_s:5}s radial {:6.1} depth {:.2} ref {:.2} dev {:5.0} locked {}",
                            r.radial_deg, r.var_depth, r.ref_depth, r.ref_deviation_hz, r.locked);
                    }
                    if r.locked { radials.push(r.radial_deg); }
                }
            }
        }
    }
    println!("{} samples, {} locked seconds, final ident {:?}", n, radials.len(), d.ident());
    if let (Some(lo), Some(hi)) = (radials.iter().cloned().reduce(f32::min), radials.iter().cloned().reduce(f32::max)) {
        println!("radial range {lo:.1}..{hi:.1}");
    }
}
