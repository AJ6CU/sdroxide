//! Hand-written bindings for LimeSuite's C API, and its LimeRFE half.
//!
//! The library is loaded with dlopen at runtime — nothing is linked at build
//! time, so this crate builds and ships everywhere and merely finds LimeSuite
//! missing where it is not installed.
//!
//! # Layout
//!
//! Every struct below mirrors the 23.11 headers field for field, and the
//! `const` asserts at the bottom pin the sizes and offsets to values **measured
//! with a C `offsetof` program** compiled against `/usr/include/lime/`. A
//! silent layout drift would not crash — it would put a FIFO size into a
//! latency setting and produce a stream that merely does not work — so any
//! mismatch has to be a compile error instead.
//!
//! # C enums
//!
//! C enums are deliberately *not* Rust enums here, for the same reason they are
//! not in `sdroxide-sdrplay`: values produced by foreign code are whatever they
//! are, and an unexpected one transmuted into a Rust enum is undefined
//! behaviour. Plain integers with named constants cannot be invalid.
//!
//! # Two tiers of symbol
//!
//! The `LMS_*` calls are resolved eagerly: a library missing one is the wrong
//! library and finding out at load time beats finding out mid-stream. The
//! `RFE_*` calls are resolved *optionally*, because LimeRFE support arrived in
//! LimeSuite 20.01 and an older library must still be able to deliver I/Q.
//!
//! # Calling convention
//!
//! `LMS_GetLibraryVersion`, `LMS_RegisterLogHandler`, `RFE_ConfigureState` and
//! `RFE_GetState` are declared in the headers without LimeSuite's `CALL_CONV`
//! macro. That is invisible on x86-64 and aarch64, which have one convention; a
//! 32-bit Windows port would have to re-check them.

#![allow(dead_code)]

use std::ffi::{CStr, c_char, c_int, c_void};

/// `lms_device_t*` — an opaque device handle owned by LimeSuite.
pub type Device = *mut c_void;
/// `rfe_dev_t*` — likewise for a LimeRFE.
pub type RfeDev = *mut c_void;

/// `lms_info_str_t`, a fixed 256-byte buffer.
pub const INFO_STR_LEN: usize = 256;
/// `lms_name_t`, a fixed 16-byte buffer.
pub const NAME_LEN: usize = 16;

/// Almost every call returns this: 0 for success, -1 for failure, and the
/// reason is fetched separately with `LMS_GetLastErrorMessage`.
pub const OK: c_int = 0;

/// `lms_stream_t::dataFmt` — what the host sees.
pub const FMT_F32: c_int = 0;
pub const FMT_I16: c_int = 1;
pub const FMT_I12: c_int = 2;

/// `lms_stream_t::linkFmt` — what goes over the wire.
pub const LINK_FMT_DEFAULT: c_int = 0;
pub const LINK_FMT_I16: c_int = 1;
pub const LINK_FMT_I12: c_int = 2;

/// `LMS_Calibrate`'s flags argument: the headers say "normally should be 0".
pub const CAL_FLAGS_NONE: u32 = 0;

/// `lms_range_t`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Range {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

/// `lms_stream_t`. `handle` is written by `LMS_SetupStream` and must not be
/// touched afterwards.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StreamT {
    pub handle: usize,
    pub is_tx: bool,
    pub channel: u32,
    pub fifo_size: u32,
    pub throughput_vs_latency: f32,
    pub data_fmt: c_int,
    pub link_fmt: c_int,
}

impl StreamT {
    pub fn zeroed() -> StreamT {
        StreamT {
            handle: 0,
            is_tx: false,
            channel: 0,
            fifo_size: 0,
            throughput_vs_latency: 0.5,
            data_fmt: FMT_F32,
            link_fmt: LINK_FMT_DEFAULT,
        }
    }
}

/// `lms_stream_meta_t`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StreamMetaT {
    pub timestamp: u64,
    pub wait_for_timestamp: bool,
    pub flush_partial_packet: bool,
}

/// `lms_stream_status_t`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct StreamStatusT {
    pub active: bool,
    pub fifo_filled_count: u32,
    pub fifo_size: u32,
    pub underrun: u32,
    pub overrun: u32,
    pub dropped_packets: u32,
    pub sample_rate: f64,
    pub link_rate: f64,
    pub timestamp: u64,
}

/// `lms_dev_info_t`. Returned as a pointer into LimeSuite's own storage, which
/// is freed when the device closes — so everything wanted from it is copied out
/// while the device is still open.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DevInfoT {
    pub device_name: [c_char; 32],
    pub expansion_name: [c_char; 32],
    pub firmware_version: [c_char; 16],
    pub hardware_version: [c_char; 16],
    pub protocol_version: [c_char; 16],
    pub board_serial_number: u64,
    pub gateware_version: [c_char; 16],
    pub gateware_target_board: [c_char; 32],
}

/// `rfe_boardState` — nine bytes, no padding.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RfeBoardState {
    pub channel_id_rx: c_char,
    pub channel_id_tx: c_char,
    pub sel_port_rx: c_char,
    pub sel_port_tx: c_char,
    pub mode: c_char,
    pub notch_on_off: c_char,
    pub att_value: c_char,
    pub enable_swr: c_char,
    pub source_swr: c_char,
}

/// Read a fixed-size C string field into a Rust `String`, stopping at the first
/// NUL and tolerating a field that is not NUL-terminated at all.
pub fn c_field(buf: &[c_char]) -> String {
    let bytes: Vec<u8> = buf.iter().take_while(|&&c| c != 0).map(|&c| c as u8).collect();
    String::from_utf8_lossy(&bytes).trim().to_string()
}

/// The log levels LimeSuite reports. Its own `LMS_LOG_*`.
pub const LOG_CRITICAL: c_int = 0;
pub const LOG_ERROR: c_int = 1;
pub const LOG_WARNING: c_int = 2;
pub const LOG_INFO: c_int = 3;
pub const LOG_DEBUG: c_int = 4;

pub type LogHandler = unsafe extern "C" fn(c_int, *const c_char);

pub struct Api {
    /// The loaded library, kept for the life of the process. Never unloaded:
    /// the log-handler pointer handed to LimeSuite must stay valid.
    _lib: libloading::Library,

    // --- identity and enumeration ---
    pub get_library_version: unsafe extern "C" fn() -> *const c_char,
    pub get_last_error_message: unsafe extern "C" fn() -> *const c_char,
    pub register_log_handler: unsafe extern "C" fn(Option<LogHandler>),
    pub get_device_list: unsafe extern "C" fn(*mut [c_char; INFO_STR_LEN]) -> c_int,
    pub open: unsafe extern "C" fn(*mut Device, *const c_char, *mut c_void) -> c_int,
    pub close: unsafe extern "C" fn(Device) -> c_int,
    pub get_device_info: unsafe extern "C" fn(Device) -> *const DevInfoT,

    // --- configuration ---
    pub init: unsafe extern "C" fn(Device) -> c_int,
    pub reset: unsafe extern "C" fn(Device) -> c_int,
    pub get_num_channels: unsafe extern "C" fn(Device, bool) -> c_int,
    pub enable_channel: unsafe extern "C" fn(Device, bool, usize, bool) -> c_int,
    pub set_sample_rate: unsafe extern "C" fn(Device, f64, usize) -> c_int,
    pub get_sample_rate: unsafe extern "C" fn(Device, bool, usize, *mut f64, *mut f64) -> c_int,
    pub get_sample_rate_range: unsafe extern "C" fn(Device, bool, *mut Range) -> c_int,
    pub set_lo_frequency: unsafe extern "C" fn(Device, bool, usize, f64) -> c_int,
    pub get_lo_frequency: unsafe extern "C" fn(Device, bool, usize, *mut f64) -> c_int,
    pub get_lo_frequency_range: unsafe extern "C" fn(Device, bool, *mut Range) -> c_int,
    pub get_antenna_list:
        unsafe extern "C" fn(Device, bool, usize, *mut [c_char; NAME_LEN]) -> c_int,
    pub set_antenna: unsafe extern "C" fn(Device, bool, usize, usize) -> c_int,
    pub get_antenna: unsafe extern "C" fn(Device, bool, usize) -> c_int,
    pub set_gain_db: unsafe extern "C" fn(Device, bool, usize, u32) -> c_int,
    pub get_gain_db: unsafe extern "C" fn(Device, bool, usize, *mut u32) -> c_int,
    pub set_lpf_bw: unsafe extern "C" fn(Device, bool, usize, f64) -> c_int,
    pub get_lpf_bw: unsafe extern "C" fn(Device, bool, usize, *mut f64) -> c_int,
    pub get_lpf_bw_range: unsafe extern "C" fn(Device, bool, *mut Range) -> c_int,
    pub calibrate: unsafe extern "C" fn(Device, bool, usize, f64, u32) -> c_int,
    pub get_chip_temperature: unsafe extern "C" fn(Device, usize, *mut f64) -> c_int,

    // --- streaming ---
    pub setup_stream: unsafe extern "C" fn(Device, *mut StreamT) -> c_int,
    pub destroy_stream: unsafe extern "C" fn(Device, *mut StreamT) -> c_int,
    pub start_stream: unsafe extern "C" fn(*mut StreamT) -> c_int,
    pub stop_stream: unsafe extern "C" fn(*mut StreamT) -> c_int,
    pub recv_stream:
        unsafe extern "C" fn(*mut StreamT, *mut c_void, usize, *mut StreamMetaT, u32) -> c_int,
    pub send_stream:
        unsafe extern "C" fn(*mut StreamT, *const c_void, usize, *const StreamMetaT, u32) -> c_int,
    pub get_stream_status: unsafe extern "C" fn(*mut StreamT, *mut StreamStatusT) -> c_int,

    // --- LimeRFE, optional: it arrived in LimeSuite 20.01 ---
    pub rfe_open: Option<unsafe extern "C" fn(*const c_char, Device) -> RfeDev>,
    pub rfe_close: Option<unsafe extern "C" fn(RfeDev)>,
    pub rfe_get_info: Option<unsafe extern "C" fn(RfeDev, *mut u8) -> c_int>,
    pub rfe_reset: Option<unsafe extern "C" fn(RfeDev) -> c_int>,
    pub rfe_configure_state: Option<unsafe extern "C" fn(RfeDev, RfeBoardState) -> c_int>,
    pub rfe_get_state: Option<unsafe extern "C" fn(RfeDev, *mut RfeBoardState) -> c_int>,
    pub rfe_mode: Option<unsafe extern "C" fn(RfeDev, c_int) -> c_int>,
    pub rfe_read_adc: Option<unsafe extern "C" fn(RfeDev, c_int, *mut c_int) -> c_int>,
    pub rfe_fan: Option<unsafe extern "C" fn(RfeDev, c_int) -> c_int>,
}

/// Library names to try, most specific first.
///
/// The SONAME on this machine's build is `libLimeSuite.so.23.11-1`, which is
/// not a shape that can be guessed from a version number — so the plain
/// unversioned name is what actually does the work where the development
/// package is installed, and the versioned entries cover a runtime-only install
/// where only the SONAME symlink exists.
#[cfg(target_os = "linux")]
fn lib_candidates() -> Vec<std::ffi::OsString> {
    let mut out: Vec<std::ffi::OsString> = vec!["libLimeSuite.so".into()];
    for major in ["23", "22", "21", "20"] {
        out.push(format!("libLimeSuite.so.{major}").into());
    }
    for v in ["23.11-1", "22.09-1", "20.10-1"] {
        out.push(format!("libLimeSuite.so.{v}").into());
    }
    out
}

#[cfg(target_os = "macos")]
fn lib_candidates() -> Vec<std::ffi::OsString> {
    [
        "libLimeSuite.dylib",
        "/usr/local/lib/libLimeSuite.dylib",
        "/opt/homebrew/lib/libLimeSuite.dylib",
    ]
    .iter()
    .map(Into::into)
    .collect()
}

/// On Windows nearly everybody has LimeSuite because PothosSDR installed it,
/// and that bundle's `bin` directory is not on anyone's DLL search path — so
/// the explicit guesses are the load-bearing entries here, exactly as the
/// registry lookup is for the SDRplay backend.
#[cfg(target_os = "windows")]
fn lib_candidates() -> Vec<std::ffi::OsString> {
    use std::path::PathBuf;
    let mut out: Vec<std::ffi::OsString> = vec!["LimeSuite.dll".into()];
    for var in ["ProgramFiles", "ProgramFiles(x86)"] {
        if let Some(pf) = std::env::var_os(var) {
            let pf = PathBuf::from(pf);
            out.push(pf.join("PothosSDR").join("bin").join("LimeSuite.dll").into_os_string());
            out.push(pf.join("LimeSuite").join("bin").join("LimeSuite.dll").into_os_string());
        }
    }
    out
}

impl Api {
    pub fn load() -> Result<Api, String> {
        let mut last = String::new();
        for name in lib_candidates() {
            match unsafe { libloading::Library::new(&name) } {
                Ok(lib) => return unsafe { Api::from_lib(lib) },
                Err(e) => last = e.to_string(),
            }
        }
        Err(format!(
            "LimeSuite was not found ({last}) — install it (Debian/Ubuntu: limesuite; Arch: \
             limesuite; macOS: brew install limesuite; Windows: the PothosSDR bundle), then \
             rescan"
        ))
    }

    unsafe fn from_lib(lib: libloading::Library) -> Result<Api, String> {
        macro_rules! sym {
            ($name:literal) => {
                *unsafe { lib.get(concat!($name, "\0").as_bytes()) }
                    .map_err(|e| format!("{} missing from LimeSuite: {e}", $name))?
            };
        }
        // The LimeRFE half: absent on LimeSuite older than 20.01, which is a
        // library that can still deliver I/Q perfectly well.
        macro_rules! opt {
            ($name:literal) => {
                unsafe { lib.get(concat!($name, "\0").as_bytes()) }.ok().map(|s| *s)
            };
        }
        Ok(Api {
            get_library_version: sym!("LMS_GetLibraryVersion"),
            get_last_error_message: sym!("LMS_GetLastErrorMessage"),
            register_log_handler: sym!("LMS_RegisterLogHandler"),
            get_device_list: sym!("LMS_GetDeviceList"),
            open: sym!("LMS_Open"),
            close: sym!("LMS_Close"),
            get_device_info: sym!("LMS_GetDeviceInfo"),

            init: sym!("LMS_Init"),
            reset: sym!("LMS_Reset"),
            get_num_channels: sym!("LMS_GetNumChannels"),
            enable_channel: sym!("LMS_EnableChannel"),
            set_sample_rate: sym!("LMS_SetSampleRate"),
            get_sample_rate: sym!("LMS_GetSampleRate"),
            get_sample_rate_range: sym!("LMS_GetSampleRateRange"),
            set_lo_frequency: sym!("LMS_SetLOFrequency"),
            get_lo_frequency: sym!("LMS_GetLOFrequency"),
            get_lo_frequency_range: sym!("LMS_GetLOFrequencyRange"),
            get_antenna_list: sym!("LMS_GetAntennaList"),
            set_antenna: sym!("LMS_SetAntenna"),
            get_antenna: sym!("LMS_GetAntenna"),
            set_gain_db: sym!("LMS_SetGaindB"),
            get_gain_db: sym!("LMS_GetGaindB"),
            set_lpf_bw: sym!("LMS_SetLPFBW"),
            get_lpf_bw: sym!("LMS_GetLPFBW"),
            get_lpf_bw_range: sym!("LMS_GetLPFBWRange"),
            calibrate: sym!("LMS_Calibrate"),
            get_chip_temperature: sym!("LMS_GetChipTemperature"),

            setup_stream: sym!("LMS_SetupStream"),
            destroy_stream: sym!("LMS_DestroyStream"),
            start_stream: sym!("LMS_StartStream"),
            stop_stream: sym!("LMS_StopStream"),
            recv_stream: sym!("LMS_RecvStream"),
            send_stream: sym!("LMS_SendStream"),
            get_stream_status: sym!("LMS_GetStreamStatus"),

            rfe_open: opt!("RFE_Open"),
            rfe_close: opt!("RFE_Close"),
            rfe_get_info: opt!("RFE_GetInfo"),
            rfe_reset: opt!("RFE_Reset"),
            rfe_configure_state: opt!("RFE_ConfigureState"),
            rfe_get_state: opt!("RFE_GetState"),
            rfe_mode: opt!("RFE_Mode"),
            rfe_read_adc: opt!("RFE_ReadADC"),
            rfe_fan: opt!("RFE_Fan"),

            _lib: lib,
        })
    }

    /// Whether this build can drive a LimeRFE through the SDR board.
    pub fn has_rfe(&self) -> bool {
        self.rfe_open.is_some()
            && self.rfe_configure_state.is_some()
            && self.rfe_mode.is_some()
            && self.rfe_close.is_some()
    }

    /// LimeSuite's own account of what just went wrong. Process-global and only
    /// meaningful straight after a failed call, which is where it is read.
    pub fn err_text(&self) -> String {
        let p = unsafe { (self.get_last_error_message)() };
        if p.is_null() {
            return "no further detail".into();
        }
        unsafe { CStr::from_ptr(p) }.to_string_lossy().trim().to_string()
    }

    pub fn version(&self) -> String {
        let p = unsafe { (self.get_library_version)() };
        if p.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(p) }.to_string_lossy().trim().to_string()
    }
}

// Layout pins — **measured** on x86_64 Linux against the 23.11 headers with an
// `offsetof` program, not derived from the source. See the module doc. A 32-bit
// port must re-measure: `lms_stream_t::handle` is a `size_t`.
#[cfg(target_pointer_width = "64")]
mod layout_asserts {
    use super::*;
    use std::mem::{align_of, offset_of, size_of};

    const _: () = assert!(size_of::<Range>() == 24);
    const _: () = assert!(offset_of!(Range, min) == 0);
    const _: () = assert!(offset_of!(Range, max) == 8);
    const _: () = assert!(offset_of!(Range, step) == 16);

    const _: () = assert!(size_of::<StreamT>() == 32);
    const _: () = assert!(offset_of!(StreamT, handle) == 0);
    const _: () = assert!(offset_of!(StreamT, is_tx) == 8);
    const _: () = assert!(offset_of!(StreamT, channel) == 12);
    const _: () = assert!(offset_of!(StreamT, fifo_size) == 16);
    const _: () = assert!(offset_of!(StreamT, throughput_vs_latency) == 20);
    const _: () = assert!(offset_of!(StreamT, data_fmt) == 24);
    const _: () = assert!(offset_of!(StreamT, link_fmt) == 28);

    const _: () = assert!(size_of::<StreamMetaT>() == 16);
    const _: () = assert!(offset_of!(StreamMetaT, timestamp) == 0);
    const _: () = assert!(offset_of!(StreamMetaT, wait_for_timestamp) == 8);
    const _: () = assert!(offset_of!(StreamMetaT, flush_partial_packet) == 9);

    const _: () = assert!(size_of::<StreamStatusT>() == 48);
    const _: () = assert!(offset_of!(StreamStatusT, active) == 0);
    const _: () = assert!(offset_of!(StreamStatusT, fifo_filled_count) == 4);
    const _: () = assert!(offset_of!(StreamStatusT, fifo_size) == 8);
    const _: () = assert!(offset_of!(StreamStatusT, underrun) == 12);
    const _: () = assert!(offset_of!(StreamStatusT, overrun) == 16);
    const _: () = assert!(offset_of!(StreamStatusT, dropped_packets) == 20);
    const _: () = assert!(offset_of!(StreamStatusT, sample_rate) == 24);
    const _: () = assert!(offset_of!(StreamStatusT, link_rate) == 32);
    const _: () = assert!(offset_of!(StreamStatusT, timestamp) == 40);

    const _: () = assert!(size_of::<DevInfoT>() == 168);
    const _: () = assert!(offset_of!(DevInfoT, expansion_name) == 32);
    const _: () = assert!(offset_of!(DevInfoT, firmware_version) == 64);
    const _: () = assert!(offset_of!(DevInfoT, hardware_version) == 80);
    const _: () = assert!(offset_of!(DevInfoT, protocol_version) == 96);
    const _: () = assert!(offset_of!(DevInfoT, board_serial_number) == 112);
    const _: () = assert!(offset_of!(DevInfoT, gateware_version) == 120);
    const _: () = assert!(offset_of!(DevInfoT, gateware_target_board) == 136);

    const _: () = assert!(size_of::<RfeBoardState>() == 9);
    const _: () = assert!(align_of::<RfeBoardState>() == 1);
    const _: () = assert!(offset_of!(RfeBoardState, source_swr) == 8);
}
