//! The LimeRFE over its own micro-USB port.
//!
//! Pure Rust, and deliberately so rather than calling LimeSuite's
//! `RFE_Open(port, NULL)`: this is the transport that works on a machine with
//! no LimeSuite installed at all, which is what lets a LimeRFE sit in front of
//! something that is not a LimeSDR. It also means the timeouts are ours —
//! `RFE_Open` blocks for up to two seconds doing its own handshake, on
//! whichever thread called it, with no way to give up early.
//!
//! The exchange is strict request/response with no framing to resynchronise
//! from — replies carry the echoed command in `buf[0]` but LimeSuite never
//! checks it, so neither does this. What keeps the link in step instead is
//! flushing the input whenever a transaction fails: a reply that arrived late
//! is not evidence about the next request.

use std::io::{Read, Write};
use std::time::{Duration, Instant};

use sdroxide_types::{RFE_BAUD, RfeMode};

use crate::error::{Error, Result};
use crate::frame::{self, Cmd, HELLO_INTERVAL_MS, MAX_HELLO_ATTEMPTS, RfeInfo, RfeState};
use crate::transport::RfeTransport;

/// How long to wait for the board's answer. Generous next to the ~33 ms sixteen
/// bytes each way take at 9600 baud, because the board answers after it has
/// finished throwing its relays.
const REPLY_TIMEOUT: Duration = Duration::from_millis(600);

/// The serial read timeout. Short, so a partial reply is noticed and retried
/// inside [`REPLY_TIMEOUT`] rather than blocking to the end of it.
const READ_TIMEOUT: Duration = Duration::from_millis(50);

/// One exchange, near enough. 16 bytes each way at 9600 8N1 is ~33 ms of wire
/// time; the relays add the rest.
const ROUND_TRIP: Duration = Duration::from_millis(45);

pub struct SerialTransport {
    port: Box<dyn serialport::SerialPort>,
    path: String,
    info: RfeInfo,
}

impl SerialTransport {
    /// Open the port and shake hands.
    ///
    /// The handshake is not ceremony. The board's microcontroller can still be
    /// booting when the CDC port enumerates, and an FTDI bridge will happily
    /// hand over a port with nothing behind it — so ten tries 200 ms apart is
    /// LimeSuite's own patience, and it is what tells a LimeRFE apart from the
    /// several other things that present the same generic USB-serial id.
    ///
    /// It is also **not a framed command**: one byte out, the same byte back.
    /// Sending a 16-byte frame here is answered with a single byte, and waiting
    /// for the other fifteen simply times out — which is what the first field
    /// report of this backend turned out to be.
    pub fn open(path: &str) -> Result<SerialTransport> {
        let port = serialport::new(path, RFE_BAUD)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .timeout(READ_TIMEOUT)
            .open()
            .map_err(|e| Error::Open {
                path: path.to_string(),
                source: std::io::Error::other(e),
            })?;

        let mut t = SerialTransport {
            port,
            path: path.to_string(),
            info: RfeInfo { firmware: 0, hardware: 0 },
        };

        let hello = frame::hello();
        for attempt in 0..MAX_HELLO_ATTEMPTS {
            // Anything the bridge buffered before we arrived is not an answer
            // to a question we asked.
            t.flush_input();
            if t.port.write_all(hello.wire()).is_ok() && t.port.flush().is_ok() {
                // LimeSuite waits the full interval before reading rather than
                // polling, and the board is slow enough to need it.
                std::thread::sleep(Duration::from_millis(HELLO_INTERVAL_MS));
                let mut byte = [0u8; 1];
                if let Ok(1) = t.port.read(&mut byte)
                    && frame::hello_answered(&byte)
                {
                    t.info = t.info()?;
                    tracing::info!(
                        "LimeRFE on {path}: firmware {}, hardware {} (answered hello on attempt \
                         {})",
                        t.info.firmware,
                        t.info.hardware,
                        attempt + 1
                    );
                    return Ok(t);
                }
            }
            if attempt + 1 < MAX_HELLO_ATTEMPTS {
                std::thread::sleep(Duration::from_millis(HELLO_INTERVAL_MS));
            }
        }
        Err(Error::NoAnswer { path: path.to_string() })
    }

    /// Drop anything sitting in the input buffer.
    ///
    /// Called before the handshake and after every failure. Without it a reply
    /// that arrived after its deadline becomes the answer to the *next*
    /// request, and the link stays one transaction out of step for as long as
    /// it runs.
    fn flush_input(&mut self) {
        let _ = self.port.clear(serialport::ClearBuffer::Input);
    }

    /// Send one command and read exactly the reply it expects.
    fn txn(&mut self, cmd: Cmd) -> Result<Vec<u8>> {
        match self.txn_inner(&cmd) {
            Ok(reply) => Ok(reply),
            Err(e) => {
                // Whatever state the link is in, it is not one to carry into
                // the next request.
                self.flush_input();
                Err(e)
            }
        }
    }

    fn txn_inner(&mut self, cmd: &Cmd) -> Result<Vec<u8>> {
        self.port.write_all(cmd.wire())?;
        self.port.flush()?;

        let deadline = Instant::now() + REPLY_TIMEOUT;
        let mut buf = vec![0u8; cmd.reply_len()];
        let mut have = 0usize;
        while have < buf.len() {
            if Instant::now() >= deadline {
                return Err(Error::ShortReply { want: cmd.reply_len(), got: have });
            }
            match self.port.read(&mut buf[have..]) {
                Ok(0) => {}
                Ok(n) => have += n,
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                Err(e) => return Err(e.into()),
            }
        }
        cmd.check(&buf)?;
        Ok(buf)
    }
}

impl RfeTransport for SerialTransport {
    fn info(&mut self) -> Result<RfeInfo> {
        let reply = self.txn(frame::get_info())?;
        frame::decode_info(&reply)
    }

    fn configure(&mut self, state: RfeState) -> Result<()> {
        self.txn(frame::config(&state)).map(|_| ())
    }

    fn set_mode(&mut self, mode: RfeMode) -> Result<()> {
        self.txn(frame::mode(mode)).map(|_| ())
    }

    fn set_fan(&mut self, on: bool) -> Result<()> {
        self.txn(frame::fan(on)).map(|_| ())
    }

    fn round_trip(&self) -> Duration {
        ROUND_TRIP
    }

    fn describe(&self) -> String {
        format!(
            "LimeRFE on {} (firmware {}, hardware {})",
            self.path, self.info.firmware, self.info.hardware
        )
    }
}
