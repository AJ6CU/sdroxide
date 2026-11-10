//! The `rtl_tcp` client against a server that is only pretending.
//!
//! The far end of this backend is a program on another machine, which is
//! exactly the sort of thing that never gets exercised until it is on a mast.
//! A fake server is cheap — twelve bytes of greeting and a socket — and it
//! pins down the two halves that matter: that the opening handshake configures
//! the dongle in the right order with the right numbers, and that the byte
//! stream comes back out as I/Q in the right order with nothing lost at a
//! segment boundary.
//!
//! What it cannot check is a real `rtl_tcp`'s behaviour. It answers nothing,
//! as the protocol requires, so a server that ignores a command looks
//! identical here to one that obeys it.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_rtlsdr::RtlSdrHandle;
use sdroxide_types::{RtlSdrAgc, RtlSdrHfMode, RtlTcpConfig};

/// Command opcodes, spelled out here rather than imported: a test that shares
/// the constants with the code under test cannot catch a renumbering.
const FREQ: u8 = 0x01;
const RATE: u8 = 0x02;
const GAIN_MODE: u8 = 0x03;
const GAIN: u8 = 0x04;
const PPM: u8 = 0x05;
const RTL_AGC: u8 = 0x08;
const DIRECT: u8 = 0x09;
const BIAS_TEE: u8 = 0x0e;

/// How long a test waits for the far end to catch up before giving up.
const DEADLINE: Duration = Duration::from_secs(5);

struct Fake {
    addr: SocketAddr,
    cmds: Arc<Mutex<Vec<(u8, u32)>>>,
    stop: Arc<AtomicBool>,
    join: Option<std::thread::JoinHandle<()>>,
}

impl Fake {
    /// Start a server that greets as `tuner` and then streams `stream_bytes`
    /// of a known pattern, recording every command it is sent.
    fn start(tuner: u32, stream_bytes: usize) -> Fake {
        Fake::start_with(tuner, stream_bytes, None)
    }

    /// A server that also sends the `rsp_tcp` extended-capability block, the
    /// way SDRplay's own server does when it is started with `-E`.
    fn start_rsp(stream_bytes: usize, sample_format: u32, capabilities: u32) -> Fake {
        let mut c = Vec::from(*b"RSP0");
        c.extend_from_slice(&1u32.to_be_bytes()); // version
        c.extend_from_slice(&capabilities.to_be_bytes());
        c.extend_from_slice(&0u32.to_be_bytes()); // __reserved__
        c.extend_from_slice(&3u32.to_be_bytes()); // hardware_version
        c.extend_from_slice(&sample_format.to_be_bytes());
        c.push(3); // antenna_input_count
        let mut name = [0u8; 13];
        name[..4].copy_from_slice(b"HiZ\0");
        c.extend_from_slice(&name);
        // third_antenna_freq_limit: the one field the real server does NOT
        // byte-swap, so the fake must not either.
        c.extend_from_slice(&30_000_000i32.to_le_bytes());
        c.push(2); // tuner_count
        c.push(20); // ifgr_min
        c.push(59); // ifgr_max
        assert_eq!(c.len(), 45, "the block is 45 bytes packed");
        // Tuner 5 (R820T) is what a real rsp_tcp server claims to be.
        Fake::start_with(5, stream_bytes, Some(c))
    }

    fn start_with(tuner: u32, stream_bytes: usize, caps: Option<Vec<u8>>) -> Fake {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let cmds = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));

        let thread_cmds = Arc::clone(&cmds);
        let thread_stop = Arc::clone(&stop);
        let join = std::thread::spawn(move || {
            let Ok((sock, _)) = listener.accept() else { return };
            serve(sock, tuner, stream_bytes, caps, &thread_cmds, &thread_stop);
        });

        Fake { addr, cmds, stop, join: Some(join) }
    }

    fn endpoint(&self) -> String {
        self.addr.to_string()
    }

    fn cmds(&self) -> Vec<(u8, u32)> {
        self.cmds.lock().expect("lock").clone()
    }

    /// Wait until at least `n` commands have arrived, then return them all.
    /// Returns whatever it has if the deadline passes, so the assertion that
    /// follows reports the real difference rather than a timeout.
    fn wait_for_cmds(&self, n: usize) -> Vec<(u8, u32)> {
        let until = Instant::now() + DEADLINE;
        loop {
            let c = self.cmds();
            if c.len() >= n || Instant::now() > until {
                return c;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    }
}

impl Drop for Fake {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        // The flag is only looked at between reads, so a server still waiting
        // for its first client would never see it. One connection that is
        // never used gets it out of `accept` and into the loop that can.
        let _ = TcpStream::connect(self.addr);
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

fn serve(
    mut sock: TcpStream,
    tuner: u32,
    mut stream_bytes: usize,
    caps: Option<Vec<u8>>,
    cmds: &Mutex<Vec<(u8, u32)>>,
    stop: &AtomicBool,
) {
    let mut greeting = Vec::from(*b"RTL0");
    greeting.extend_from_slice(&tuner.to_be_bytes());
    greeting.extend_from_slice(&29u32.to_be_bytes());
    if sock.write_all(&greeting).is_err() {
        return;
    }
    // An rsp_tcp server started with -E follows the greeting with this, in the
    // same breath. Everything else goes straight to samples.
    if let Some(c) = &caps
        && sock.write_all(c).is_err()
    {
        return;
    }
    sock.set_read_timeout(Some(Duration::from_millis(5))).expect("timeout");

    let mut acc: Vec<u8> = Vec::new();
    let mut pattern: u8 = 0;
    let mut buf = [0u8; 256];
    while !stop.load(Ordering::Relaxed) {
        match sock.read(&mut buf) {
            // The client hung up. Anything it sent before that has already
            // been delivered, so the record is complete.
            Ok(0) => return,
            Ok(n) => acc.extend_from_slice(&buf[..n]),
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(_) => return,
        }
        while acc.len() >= 5 {
            let rest = acc.split_off(5);
            let arg = u32::from_be_bytes([acc[1], acc[2], acc[3], acc[4]]);
            cmds.lock().expect("lock").push((acc[0], arg));
            acc = rest;
        }

        // Deliberately awkward chunk lengths, none of them even: a client that
        // drops the odd byte at the end of a read instead of carrying it into
        // the next one swaps I with Q and fails the comparison downstream.
        if stream_bytes > 0 {
            let n = stream_bytes.min(997);
            let chunk: Vec<u8> = (0..n)
                .map(|_| {
                    let b = pattern;
                    pattern = pattern.wrapping_add(1);
                    b
                })
                .collect();
            if sock.write_all(&chunk).is_err() {
                return;
            }
            stream_bytes -= n;
        }
    }
}

fn config(endpoint: &str) -> RtlTcpConfig {
    RtlTcpConfig {
        address: endpoint.to_string(),
        sample_rate_hz: 1_024_000.0,
        ppm: -7,
        tuner_gain_db: 30.0,
        agc: RtlSdrAgc::Manual,
        hf_mode: RtlSdrHfMode::Auto,
        bias_tee: false,
        iq_correction: true,
        // The rsp_tcp controls are not what these tests are about, and the
        // opening handshake does not send them — an SDRplay server is the only
        // thing that would understand them.
        ..RtlTcpConfig::default()
    }
}

/// The value the sample lookup table produces for one wire byte. Written out
/// rather than shared with the driver, for the same reason as the opcodes.
fn expect_sample(b: u8) -> f32 {
    (b as f32 - 127.4) / 128.0
}

/// The opening sequence, in the order the far end needs it: the crystal
/// correction before anything is computed against it, the rate before the
/// tuning, the HF path before the frequency it decides how to reach, and the
/// gains after the path switch that would have reset them.
#[test]
fn the_handshake_configures_the_far_end_in_dependency_order() {
    let fake = Fake::start(5, 0);
    let handle = RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");

    let cmds = fake.wait_for_cmds(8);
    assert_eq!(
        cmds,
        vec![
            (PPM, (-7i32) as u32),
            (RATE, 1_024_000),
            // 100 MHz is well above the crossover, so the far end is told to
            // use its tuner — explicitly, because what a previous client left
            // it in cannot be read back.
            (DIRECT, 0),
            (FREQ, 100_100_000),
            // Manual gain: mode 1, then 30.0 dB as tenths, then the
            // demodulator's own AGC off.
            (GAIN_MODE, 1),
            (GAIN, 300),
            (RTL_AGC, 0),
            (BIAS_TEE, 0),
        ]
    );
    drop(handle);
}

/// Tuning to HF has to switch the far end's front end *and* restate the gains,
/// because the tuner re-init that comes with the switch resets them.
#[test]
fn tuning_below_the_crossover_switches_the_far_end_to_direct_sampling() {
    let fake = Fake::start(5, 0);
    let handle = RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");
    let opening = fake.wait_for_cmds(8).len();

    handle.set_center_hz(7_100_000.0);

    let cmds = fake.wait_for_cmds(opening + 5);
    assert_eq!(
        &cmds[opening..],
        &[(DIRECT, 2), (GAIN_MODE, 1), (GAIN, 300), (RTL_AGC, 0), (FREQ, 7_100_000)],
        "the gains have to follow the path switch, and the frequency the path"
    );
    drop(handle);
}

/// A Blog V4 upconverts on the far end, and the protocol gives no way to tell
/// one from a plain R828D — so the tuner is what decides, and it is left alone.
#[test]
fn an_r828d_is_not_switched_to_direct_sampling() {
    let fake = Fake::start(6, 0);
    let handle = RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");
    let opening = fake.wait_for_cmds(8).len();

    handle.set_center_hz(7_100_000.0);

    let cmds = fake.wait_for_cmds(opening + 1);
    assert_eq!(&cmds[opening..], &[(FREQ, 7_100_000)], "an upconverting dongle just tunes");
    drop(handle);
}

/// Gain and AGC changes go out as a group, because a server running its tuner
/// AGC discards a gain it is sent while the mode still says automatic.
#[test]
fn a_gain_change_restates_the_mode_with_it() {
    let fake = Fake::start(5, 0);
    let handle = RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");
    let opening = fake.wait_for_cmds(8).len();

    handle.set_gain_db(12.5);
    let cmds = fake.wait_for_cmds(opening + 3);
    assert_eq!(&cmds[opening..], &[(GAIN_MODE, 1), (GAIN, 125), (RTL_AGC, 0)]);

    // Handing the far end its own loops: no gain value at all, since it owns
    // the figure now.
    let n = cmds.len();
    handle.set_agc(RtlSdrAgc::Both);
    let cmds = fake.wait_for_cmds(n + 2);
    assert_eq!(&cmds[n..], &[(GAIN_MODE, 0), (RTL_AGC, 1)]);
    assert_eq!(handle.effective_gain_db(), None, "no figure to report under the far end's AGC");
    drop(handle);
}

/// The bias tee outlives the connection — `rtl_tcp` keeps the dongle open for
/// the next client — so leaving it on at shutdown would leave DC on a coax
/// that is somewhere else entirely.
#[test]
fn the_remote_bias_tee_is_turned_off_on_the_way_out() {
    let fake = Fake::start(5, 0);
    let mut handle =
        RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");
    let opening = fake.wait_for_cmds(8).len();

    handle.set_bias_tee(true);
    let cmds = fake.wait_for_cmds(opening + 1);
    assert_eq!(&cmds[opening..], &[(BIAS_TEE, 1)]);

    handle.release();
    let cmds = fake.wait_for_cmds(opening + 2);
    assert_eq!(cmds.last(), Some(&(BIAS_TEE, 0)), "the far end was left powering the coax");
}

/// Every byte the server sent, in order, as interleaved I/Q — the check that
/// nothing is dropped or transposed at a read boundary. The chunk lengths the
/// fake uses are odd on purpose, so a read almost always ends between an I and
/// its Q.
#[test]
fn the_sample_stream_survives_the_segment_boundaries() {
    const BYTES: usize = 8_191;
    let fake = Fake::start(5, BYTES);
    let mut handle =
        RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");

    // One byte of the odd total stays behind, held for a partner that never
    // comes.
    let want = BYTES - 1;
    let mut got: Vec<f32> = Vec::with_capacity(want);
    let mut buf = vec![0f32; 4096];
    let until = Instant::now() + DEADLINE;
    while got.len() < want && Instant::now() < until {
        let n = handle.rx_read(&mut buf);
        assert_eq!(n % 2, 0, "an odd float count would put I and Q out of step");
        got.extend_from_slice(&buf[..n]);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    assert_eq!(got.len(), want, "the stream stopped short");
    for (i, v) in got.iter().enumerate() {
        let expect = expect_sample((i % 256) as u8);
        assert!(
            (v - expect).abs() < 1e-6,
            "sample {i}: {v} != {expect} — the stream slipped a byte here"
        );
    }
    handle.release();
}

/// Something else listening on the port.
#[test]
fn a_server_that_is_not_rtl_tcp_says_so() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    std::thread::spawn(move || {
        if let Ok((mut sock, _)) = listener.accept() {
            let _ = sock.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n");
            std::thread::sleep(Duration::from_millis(200));
        }
    });
    let Err(e) = RtlSdrHandle::connect(&config(&addr.to_string()), 100e6) else {
        panic!("a web server must not be taken for rtl_tcp");
    };
    assert!(e.to_string().contains("RTL0"), "unhelpful: {e}");
}

/// An `rsp_tcp` server in extended mode is accepted, and its 45-byte
/// capability block does **not** end up in the sample stream.
///
/// This is the regression that motivated the whole change. An SDRplay server
/// greets with `"RTL0"` exactly like a dongle — the `"RSP0"` magic belongs to
/// the block that follows, not to the greeting — so before this the block was
/// read as I/Q and put a burst of noise at the head of every extended session,
/// with nothing on screen to say why.
#[test]
fn an_rsp_tcp_capability_block_is_read_and_not_streamed() {
    const BYTES: usize = 4_097;
    // 8-bit format, and every capability bit set.
    let fake = Fake::start_rsp(BYTES, 1, 0xff);
    let mut handle =
        RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");

    let want = BYTES - 1;
    let mut got: Vec<f32> = Vec::with_capacity(want);
    let mut buf = vec![0f32; 4096];
    let until = Instant::now() + DEADLINE;
    while got.len() < want && Instant::now() < until {
        let n = handle.rx_read(&mut buf);
        got.extend_from_slice(&buf[..n]);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    assert_eq!(got.len(), want, "the stream stopped short");
    // The first sample must be the first byte of the *pattern*, not the fifth
    // byte of the capability block.
    for (i, v) in got.iter().enumerate() {
        let expect = expect_sample((i % 256) as u8);
        assert!(
            (v - expect).abs() < 1e-6,
            "sample {i}: {v} != {expect} — the capability block leaked into the stream"
        );
    }
    handle.release();
}

/// A server streaming 16-bit samples is decoded as 16-bit.
///
/// `rsp_tcp -b 16` sends signed little-endian `i16` instead of the 8-bit
/// unsigned every rtl_tcp client expects. The only way a client can know is the
/// capability block, so this and the extended mode go together — a `-b 16`
/// server started without `-E` is undetectable and reads as noise, which the
/// manual says out loud rather than pretending otherwise.
#[test]
fn a_sixteen_bit_server_is_decoded_as_sixteen_bit() {
    const BYTES: usize = 4_096;
    // sample_format 2 = INT16.
    let fake = Fake::start_rsp(BYTES, 2, 0xff);
    let mut handle =
        RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");

    // Two wire bytes per component now, so the same byte count is half as many
    // floats.
    let want = BYTES / 2;
    let mut got: Vec<f32> = Vec::with_capacity(want);
    let mut buf = vec![0f32; 4096];
    let until = Instant::now() + DEADLINE;
    while got.len() < want && Instant::now() < until {
        let n = handle.rx_read(&mut buf);
        got.extend_from_slice(&buf[..n]);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    assert_eq!(got.len(), want, "16-bit halves the float count for the same bytes");
    for (i, v) in got.iter().enumerate() {
        let lo = ((2 * i) % 256) as u8;
        let hi = ((2 * i + 1) % 256) as u8;
        let expect = i16::from_le_bytes([lo, hi]) as f32 / 32768.0;
        assert!(
            (v - expect).abs() < 1e-6,
            "sample {i}: {v} != {expect} — 16-bit samples read as 8-bit look like noise"
        );
    }
    handle.release();
}

/// A plain `rtl_tcp` server must lose nothing to the peek that looks for the
/// capability block. Four bytes are read either way; when they turn out not to
/// be the magic they are samples and have to be put back.
#[test]
fn peeking_for_the_capability_block_costs_a_plain_server_nothing() {
    const BYTES: usize = 2_048;
    let fake = Fake::start(5, BYTES);
    let mut handle =
        RtlSdrHandle::connect(&config(&fake.endpoint()), 100_100_000.0).expect("connects");

    let mut got: Vec<f32> = Vec::with_capacity(BYTES);
    let mut buf = vec![0f32; 4096];
    let until = Instant::now() + DEADLINE;
    while got.len() < BYTES && Instant::now() < until {
        let n = handle.rx_read(&mut buf);
        got.extend_from_slice(&buf[..n]);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(5));
        }
    }
    assert_eq!(got.len(), BYTES);
    for (i, v) in got.iter().enumerate() {
        let expect = expect_sample((i % 256) as u8);
        assert!((v - expect).abs() < 1e-6, "sample {i} moved: the peek ate the head");
    }
    handle.release();
}

#[test]
fn nothing_listening_names_the_thing_to_start() {
    // A port that was ours a moment ago is a port nothing is listening on.
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    drop(listener);

    let Err(e) = RtlSdrHandle::connect(&config(&addr.to_string()), 100e6) else {
        panic!("connected to a port nothing is listening on");
    };
    assert!(e.to_string().contains("rtl_tcp -a 0.0.0.0"), "unhelpful: {e}");
}

/// The port may be left off the address, because the protocol has a default
/// and every rtl_tcp instruction on the internet uses it.
#[test]
fn the_default_port_is_supplied() {
    // The address the client dials, rather than a connection: 1234 is not a
    // port a test may assume it can bind.
    assert_eq!(config("raspberrypi.local").endpoint(), "raspberrypi.local:1234");
    assert_eq!(config("192.168.1.5").endpoint(), "192.168.1.5:1234");
    // One that already carries a port is dialled as written...
    assert_eq!(config("192.168.1.5:5678").endpoint(), "192.168.1.5:5678");
    // ...and an IPv6 literal is all colons, so the brackets are what decide.
    assert_eq!(config("[fe80::1]").endpoint(), "[fe80::1]:1234");
    assert_eq!(config("[fe80::1]:5678").endpoint(), "[fe80::1]:5678");
}
