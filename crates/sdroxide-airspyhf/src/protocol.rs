//! The Airspy HF+ wire protocol, with no `nusb` in it.
//!
//! Every constant and every encode/decode lives here as a pure function, so
//! that the half of this driver most likely to be transcribed wrong is also the
//! half that is fully testable without a receiver. [`crate::usb`] turns these
//! into control transfers; nothing here does I/O.
//!
//! Transcribed from libairspyhf's `airspyhf_commands.h` and `airspyhf.c`. Two
//! things in this file are the opposite of what the rest of the device does and
//! are the likeliest bugs, so both have tests that assert against literals:
//! [`encode_freq_khz`] is **big-endian**, and the samples arrive **Q before I**
//! (see [`crate::convert`]).

/// USB vendor and product id. One pair for **every** model — a Dual, a
/// Discovery and a Ranger are indistinguishable until [`Request::SerialBoardId`]
/// is asked.
pub const VID: u16 = 0x03eb;
pub const PID: u16 = 0x800c;

/// The SAM-BA bootloader's id. This driver never enters it and cannot flash
/// firmware; it is here only so a receiver stuck in DFU can be recognised and
/// explained rather than reported as absent.
pub const PID_BOOTLOADER: u16 = 0x6124;

/// The configuration, interface and alternate setting the sample endpoint lives
/// in. Alt setting 1 is not optional: alt 0 is the zero-bandwidth setting and
/// has no bulk endpoint at all.
pub const CONFIGURATION: u8 = 1;
pub const INTERFACE: u8 = 0;
pub const ALT_SETTING: u8 = 1;

/// The bulk IN endpoint carrying the sample stream.
pub const BULK_EP: u8 = 0x81;

/// Control-transfer timeout, matching libairspyhf's `LIBUSB_CTRL_TIMEOUT_MS`.
pub const CTRL_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(500);

/// Bytes per complex sample on the wire: two little-endian `i16`.
pub const BYTES_PER_SAMPLE: usize = 4;

/// Complex samples in one transfer, and the transfer size that follows from it.
/// Upstream queues 16 of these.
pub const SAMPLES_PER_TRANSFER: usize = 4096;
pub const TRANSFER_BYTES: usize = SAMPLES_PER_TRANSFER * BYTES_PER_SAMPLE;

/// Full-scale for the `i16` samples.
pub const SAMPLE_SCALE: f32 = 1.0 / 32768.0;

/// How far above the wanted signal the synthesiser is parked on a zero-IF rate,
/// so the LO's own leakage lands clear of it. The host oscillator shifts it
/// back; with the host DSP off there is no shift, so there is no offset either.
pub const DEFAULT_IF_SHIFT_HZ: f64 = 5_000.0;

/// Synthesiser floors, in kHz. Below these the LO clamps and the host
/// oscillator does all the tuning — which is how this receiver reaches VLF.
pub const MIN_ZERO_IF_LO_KHZ: u32 = 180;
pub const MIN_LOW_IF_LO_KHZ: u32 = 84;

/// Marks a flash page as carrying a real calibration rather than erased bytes.
pub const CALIBRATION_MAGIC: u32 = 0xA5CA_71B0;

/// The flash configuration page is one 256-byte block.
pub const FLASH_CONFIG_BYTES: usize = 256;

/// The prefix every HF+ puts in front of its serial number.
pub const SERIAL_PREFIX: &str = "AIRSPYHF SN:";
/// Hex digits following it.
pub const SERIAL_DIGITS: usize = 16;

/// Vendor requests, as `bRequest`. Values from `airspyhf_commands.h`.
///
/// Requests 14 and above were added over the life of the product and older
/// firmware stalls them. Nothing here version-sniffs the firmware string:
/// [`Request::is_optional`] says which may simply fail, and a failure is read
/// as "this firmware does not have it".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Request {
    ReceiverMode = 1,
    SetFreq = 2,
    GetSampleRates = 3,
    SetSampleRate = 4,
    ConfigRead = 5,
    ConfigWrite = 6,
    SerialBoardId = 7,
    SetUserOutput = 8,
    GetVersionString = 9,
    SetAgc = 10,
    SetAgcThreshold = 11,
    SetAtt = 12,
    SetLna = 13,
    GetSampleRateArchitectures = 14,
    GetFilterGain = 15,
    GetFreqDelta = 16,
    SetVctcxoCalibration = 17,
    SetFrontendOptions = 18,
    GetAttSteps = 19,
    GetBiasTeeCount = 20,
    GetBiasTeeName = 21,
    SetBiasTee = 22,
}

impl Request {
    pub fn code(self) -> u8 {
        self as u8
    }

    /// Whether a firmware is allowed not to have this request.
    ///
    /// Every one of these was added after the product shipped. An older
    /// receiver stalls them, and a stalled control transfer self-clears, so the
    /// only correct response is to carry on with a documented fallback.
    pub fn is_optional(self) -> bool {
        matches!(
            self,
            Request::GetSampleRateArchitectures
                | Request::GetFilterGain
                | Request::GetFreqDelta
                | Request::GetAttSteps
                | Request::GetBiasTeeCount
                | Request::GetBiasTeeName
                | Request::SetBiasTee
        )
    }
}

/// The LO frequency, in kHz, **big-endian**.
///
/// This is the one big-endian field on the device — the sample stream, the rate
/// table, the attenuator table and the flash page are all little-endian. Sending
/// it the other way round tunes to a wildly wrong frequency that still looks
/// like a plausible number, so this has a test asserting against literals.
pub fn encode_freq_khz(khz: u32) -> [u8; 4] {
    khz.to_be_bytes()
}

/// The synthesiser's residual error for the LO it was last given, in Hz.
///
/// The four bytes are not a plain integer: `b[0]` is a binary scale exponent and
/// `b[1..=3]` a signed 24-bit value with its **most** significant byte last.
/// Returns `None` on a short reply.
pub fn decode_freq_delta(b: &[u8]) -> Option<f64> {
    if b.len() < 4 {
        return None;
    }
    // `b[3] as i8` is what carries the sign; the OR then fills the low 16 bits
    // exactly as the C does, because a negative high part has zeroes there.
    let raw = ((b[3] as i8 as i32) << 16) | ((b[2] as i32) << 8) | (b[1] as i32);
    // The C shifts by this without checking. A firmware that returned 200 here
    // would be undefined behaviour there and a panic here, so clamp it: a
    // nonsense exponent is worth ignoring, not crashing over.
    let exp = b[0].min(31);
    Some(raw as f64 * 1e3 / (1u64 << exp) as f64)
}

/// The receiver's stored calibration, from the 256-byte flash page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FlashConfig {
    /// Whether the magic matched. When it did not, every field below is zero
    /// and the page carries nothing — which is a normal state for a receiver
    /// that has never been calibrated by the vendor tool.
    pub valid: bool,
    /// Frequency calibration in parts per billion.
    pub calibration_ppb: i32,
    /// VCTCXO trim, pushed back to the device as-is.
    pub calibration_vctcxo: i32,
    /// Front-end option flags, pushed back to the device as-is.
    pub frontend_options: u32,
}

impl FlashConfig {
    /// Parse the flash page. Anything without the magic yields
    /// [`FlashConfig::default`] — all zeroes, `valid: false`.
    ///
    /// Reading erased flash as a calibration would put a receiver several kHz
    /// off with nothing on screen to say why, so the magic check is the whole
    /// point of this function.
    pub fn parse(page: &[u8]) -> FlashConfig {
        if page.len() < 16 {
            return FlashConfig::default();
        }
        let word = |i: usize| u32::from_le_bytes([page[i], page[i + 1], page[i + 2], page[i + 3]]);
        if word(0) != CALIBRATION_MAGIC {
            return FlashConfig::default();
        }
        FlashConfig {
            valid: true,
            calibration_ppb: word(4) as i32,
            calibration_vctcxo: word(8) as i32,
            frontend_options: word(12),
        }
    }
}

/// The sample rates the receiver offers, in Hz, in the order it lists them.
pub fn parse_rates(b: &[u8]) -> Vec<f64> {
    b.chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]) as f64)
        .filter(|r| *r > 0.0)
        .collect()
}

/// Which of those rates are low-IF, index-parallel to [`parse_rates`].
///
/// The request asks for four bytes per rate and the firmware answers with one,
/// so a short reply is normal here and not a fault. Rates the reply does not
/// cover are reported zero-IF, which is the conservative answer: it leaves the
/// image balancer running where it might not be needed rather than switching it
/// off where it is.
pub fn parse_architectures(b: &[u8], rates: usize) -> Vec<bool> {
    (0..rates).map(|i| b.get(i).is_some_and(|v| *v != 0)).collect()
}

/// The attenuator steps the receiver offers, in dB, as little-endian `f32`.
pub fn parse_att_steps(b: &[u8]) -> Vec<f64> {
    b.chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]) as f64)
        .filter(|v| v.is_finite() && *v >= 0.0)
        .collect()
}

/// A `u32` count, as the two-step "ask for the count, then ask for that many"
/// queries return it.
pub fn parse_count(b: &[u8]) -> Option<u32> {
    (b.len() >= 4).then(|| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// The board id word and the 64-bit serial from `GET_SERIALNO_BOARDID`.
///
/// The reply is `{ u32 part_id; u32 serial_no[4] }`; only the first two serial
/// words carry anything.
pub fn parse_board_id(b: &[u8]) -> Option<(u32, u64)> {
    if b.len() < 20 {
        return None;
    }
    let word = |i: usize| u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
    let serial = ((word(4) as u64) << 32) | word(8) as u64;
    Some((word(0), serial))
}

/// A NUL-padded ASCII reply, such as the firmware version string, as a `String`.
pub fn parse_ascii(b: &[u8]) -> String {
    let end = b.iter().position(|c| *c == 0).unwrap_or(b.len());
    String::from_utf8_lossy(&b[..end]).trim().to_string()
}

/// The 16 hex digits out of an `AIRSPYHF SN:0123456789ABCDEF` descriptor.
///
/// Anything that is not exactly that shape returns `None` rather than a
/// substring: a serial is used to pin one receiver out of several, and a
/// half-parsed one would pin the wrong device.
pub fn serial_from_iserial(s: &str) -> Option<String> {
    let digits = s.strip_prefix(SERIAL_PREFIX)?;
    if digits.len() != SERIAL_DIGITS || !digits.bytes().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some(digits.to_ascii_uppercase())
}

/// Whether an operator-supplied serial names this receiver.
///
/// Case-insensitive and whitespace-tolerant, because the value in `radio.json`
/// was typed or pasted by a human and the descriptor's case is the firmware's
/// choice, not theirs.
pub fn serial_matches(want: &str, found: Option<&str>) -> bool {
    match found {
        Some(f) => want.trim().eq_ignore_ascii_case(f.trim()),
        None => false,
    }
}

/// Undo the processing gain the firmware applies, from the dB figure it
/// reports.
///
/// The reply is one byte. A value outside a sane range means the request is not
/// really there — some firmware answers a request it does not implement with
/// whatever was in the buffer — so it yields unity rather than an absurd scale
/// factor that would make the receiver look broken.
pub fn filter_gain_from_db(b: &[u8]) -> f32 {
    match b.first() {
        Some(db) if *db <= 60 => 10f32.powf(*db as f32 * -0.05),
        _ => 1.0,
    }
}

/// The `wValue` for `SET_BIAS_TEE`, which the firmware reads as a signed byte.
///
/// Upstream casts `int8_t` into a `uint16_t`, so −1 goes out as `0xFFFF`.
/// Reproducing the sign extension matters: a driver that sent `0x00FF` would be
/// asking for bias-tee index 255.
pub fn bias_tee_wvalue(v: i8) -> u16 {
    v as i16 as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The one big-endian field on the whole device, and the single most likely
    /// transcription slip in the driver. Asserted against literals rather than
    /// against `to_be_bytes`, so that a "fix" to little-endian fails here.
    #[test]
    fn the_lo_frequency_goes_out_big_endian() {
        assert_eq!(encode_freq_khz(10_000), [0x00, 0x00, 0x27, 0x10]);
        assert_eq!(encode_freq_khz(14_079), [0x00, 0x00, 0x36, 0xff]);
        assert_eq!(encode_freq_khz(180), [0x00, 0x00, 0x00, 0xb4]);
        assert_eq!(encode_freq_khz(0), [0; 4]);
    }

    /// `b[3]` carries the sign and `b[0]` is a binary scale, neither of which
    /// looks like it from the bytes.
    #[test]
    fn the_frequency_delta_sign_extends_and_scales() {
        // -1 in the high byte, scaled by 2^10: (-65536 * 1000) / 1024.
        assert_eq!(decode_freq_delta(&[10, 0x00, 0x00, 0xff]), Some(-64_000.0));
        // A scale of zero divides by one, not by zero.
        assert_eq!(decode_freq_delta(&[0, 0x00, 0x01, 0x00]), Some(256_000.0));
        assert_eq!(decode_freq_delta(&[0, 0x00, 0x00, 0x00]), Some(0.0));
        // The low bytes still contribute when the high byte is negative, the
        // same way the C's OR does: 0xFFFF8000 is -32768.
        assert_eq!(decode_freq_delta(&[0, 0x00, 0x80, 0xff]), Some(-32_768_000.0));
        // A nonsense exponent is clamped rather than shifting off the end.
        assert!(decode_freq_delta(&[200, 0x01, 0x00, 0x00]).unwrap().abs() < 1e-3);
        assert_eq!(decode_freq_delta(&[0, 0, 0]), None);
    }

    /// Reading erased flash as a calibration would put the receiver kHz off
    /// with nothing on screen to explain it.
    #[test]
    fn the_flash_config_is_ignored_unless_the_magic_matches() {
        let mut page = [0u8; FLASH_CONFIG_BYTES];
        page[0..4].copy_from_slice(&CALIBRATION_MAGIC.to_le_bytes());
        page[4..8].copy_from_slice(&(-1234i32).to_le_bytes());
        page[8..12].copy_from_slice(&2048i32.to_le_bytes());
        page[12..16].copy_from_slice(&3u32.to_le_bytes());

        let good = FlashConfig::parse(&page);
        assert!(good.valid);
        assert_eq!(good.calibration_ppb, -1234);
        assert_eq!(good.calibration_vctcxo, 2048);
        assert_eq!(good.frontend_options, 3);

        // Erased flash, a foreign page, and a truncated read all mean "nothing
        // stored" — never a calibration made of whatever was there.
        for bad in [[0xffu8; FLASH_CONFIG_BYTES], [0x00; FLASH_CONFIG_BYTES]] {
            assert_eq!(FlashConfig::parse(&bad), FlashConfig::default());
        }
        page[0] ^= 0x01;
        assert_eq!(FlashConfig::parse(&page), FlashConfig::default());
        assert_eq!(FlashConfig::parse(&[]), FlashConfig::default());
        assert_eq!(FlashConfig::parse(&[0; 15]), FlashConfig::default());
    }

    /// The architecture request asks for four bytes per rate and the firmware
    /// answers with one. Both lengths have to decode the same way, and a reply
    /// that runs out has to fall back rather than panic.
    #[test]
    fn the_architecture_reply_may_be_shorter_than_asked_for() {
        let rates = [768_000.0, 384_000.0, 256_000.0, 192_000.0];
        let short = [0u8, 1, 1, 1];
        assert_eq!(parse_architectures(&short, rates.len()), vec![false, true, true, true]);
        // Absent entirely: everything reads as zero-IF, which leaves the
        // balancer running rather than switching it off where it is needed.
        assert_eq!(parse_architectures(&[], rates.len()), vec![false; 4]);
        assert_eq!(parse_architectures(&[1], rates.len()), vec![true, false, false, false]);
        // Any non-zero byte is low-IF, not just 1.
        assert_eq!(parse_architectures(&[7], 1), vec![true]);
    }

    #[test]
    fn the_rate_and_attenuator_tables_are_little_endian() {
        let mut b = Vec::new();
        for r in [768_000u32, 384_000, 256_000] {
            b.extend_from_slice(&r.to_le_bytes());
        }
        assert_eq!(parse_rates(&b), vec![768_000.0, 384_000.0, 256_000.0]);
        // A trailing partial word is dropped, not misread.
        b.push(0x01);
        assert_eq!(parse_rates(&b).len(), 3);
        // A zero rate is not a rate.
        assert_eq!(parse_rates(&0u32.to_le_bytes()), Vec::<f64>::new());

        let mut a = Vec::new();
        for s in [0.0f32, 6.0, 12.0] {
            a.extend_from_slice(&s.to_le_bytes());
        }
        assert_eq!(parse_att_steps(&a), vec![0.0, 6.0, 12.0]);
        // Garbage that is not a finite non-negative dB figure is dropped.
        a.extend_from_slice(&f32::NAN.to_le_bytes());
        a.extend_from_slice(&(-1.0f32).to_le_bytes());
        assert_eq!(parse_att_steps(&a).len(), 3);

        assert_eq!(parse_count(&9u32.to_le_bytes()), Some(9));
        assert_eq!(parse_count(&[1, 2, 3]), None);
    }

    #[test]
    fn the_board_id_reply_splits_into_a_part_id_and_a_serial() {
        let mut b = Vec::new();
        b.extend_from_slice(&2u32.to_le_bytes());
        b.extend_from_slice(&0x0123_4567u32.to_le_bytes());
        b.extend_from_slice(&0x89ab_cdefu32.to_le_bytes());
        b.extend_from_slice(&[0; 8]);
        assert_eq!(parse_board_id(&b), Some((2, 0x0123_4567_89ab_cdef)));
        assert_eq!(parse_board_id(&b[..19]), None);
    }

    /// A serial is used to pin one receiver out of several, so a half-parsed
    /// one would pin the wrong device.
    #[test]
    fn the_serial_parses_only_the_exact_descriptor_shape() {
        assert_eq!(
            serial_from_iserial("AIRSPYHF SN:0123456789ABCDEF").as_deref(),
            Some("0123456789ABCDEF")
        );
        // Lower-case hex is normalised, so a pinned serial matches whatever
        // case the firmware happens to use.
        assert_eq!(
            serial_from_iserial("AIRSPYHF SN:0123456789abcdef").as_deref(),
            Some("0123456789ABCDEF")
        );
        for bad in [
            "",
            "AIRSPYHF SN:",
            "AIRSPYHF SN:0123456789ABCDE",   // 15 digits
            "AIRSPYHF SN:0123456789ABCDEF0", // 17
            "AIRSPYHF SN:0123456789ABCDEG",  // not hex
            "AIRSPY SN:0123456789ABCDEF",
            "0123456789ABCDEF",
        ] {
            assert_eq!(serial_from_iserial(bad), None, "{bad:?} must not parse");
        }

        assert!(serial_matches(" 0123456789abcdef ", Some("0123456789ABCDEF")));
        assert!(!serial_matches("0123456789ABCDEF", Some("FEDCBA9876543210")));
        assert!(!serial_matches("0123456789ABCDEF", None));
    }

    #[test]
    fn the_filter_gain_undoes_the_reported_gain() {
        assert_eq!(filter_gain_from_db(&[0]), 1.0);
        // 6 dB of firmware gain is undone by scaling to a half.
        assert!((filter_gain_from_db(&[6]) - 0.501_187).abs() < 1e-5);
        assert!(filter_gain_from_db(&[12]) < filter_gain_from_db(&[6]));
        // Absent or absurd: unity, never a scale factor that makes a working
        // receiver look dead.
        assert_eq!(filter_gain_from_db(&[]), 1.0);
        assert_eq!(filter_gain_from_db(&[200]), 1.0);
    }

    #[test]
    fn the_bias_tee_value_sign_extends_through_wvalue() {
        assert_eq!(bias_tee_wvalue(-1), 0xffff);
        assert_eq!(bias_tee_wvalue(0), 0x0000);
        assert_eq!(bias_tee_wvalue(1), 0x0001);
        assert_eq!(bias_tee_wvalue(i8::MIN), 0xff80);
    }

    /// Every request added after the product shipped has to be allowed to fail.
    /// Getting this list wrong turns an older receiver into one that refuses to
    /// open.
    #[test]
    fn only_the_later_requests_are_optional() {
        for r in [
            Request::GetSampleRateArchitectures,
            Request::GetFilterGain,
            Request::GetFreqDelta,
            Request::GetAttSteps,
            Request::GetBiasTeeCount,
            Request::GetBiasTeeName,
            Request::SetBiasTee,
        ] {
            assert!(r.is_optional(), "{r:?}");
            assert!(r.code() >= 14, "{r:?}");
        }
        for r in [
            Request::ReceiverMode,
            Request::SetFreq,
            Request::GetSampleRates,
            Request::SetSampleRate,
            Request::SerialBoardId,
            Request::GetVersionString,
            Request::SetAgc,
            Request::SetAtt,
            Request::SetLna,
        ] {
            assert!(!r.is_optional(), "{r:?} is what makes the receiver work");
        }
        assert_eq!(Request::SetFreq.code(), 2);
        assert_eq!(Request::SetBiasTee.code(), 22);
    }

    #[test]
    fn ascii_replies_stop_at_the_nul() {
        assert_eq!(parse_ascii(b"R3.0.7-CD\0\0\0\0"), "R3.0.7-CD");
        assert_eq!(parse_ascii(b"R3.0.7-CD"), "R3.0.7-CD");
        assert_eq!(parse_ascii(b"\0junk"), "");
        assert_eq!(parse_ascii(b""), "");
    }
}
