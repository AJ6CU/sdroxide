//! Parsing and uploading the Cypress FX3 RAM image.
//!
//! The RX-888's FX3 has no populated boot EEPROM, so on every plug-in it comes
//! up as `04b4:00f3` running the mask-ROM bootloader, with no radio behaviour of
//! any kind. The host downloads a firmware image into its RAM over EP0 and tells
//! it to jump; the device then drops off the bus and re-appears as `04b4:00f1`.
//! Everything else this crate does depends on that having happened first.
//!
//! Written against the Cypress `.img` layout as implemented by libusb's
//! `examples/ezusb.c` (`fx3_load_ram`), and checked against the vendored image —
//! [`tests::the_bundled_image_parses`] parses the real artifact rather than only
//! a synthetic one, so a bad or truncated vendored file fails the build's tests
//! rather than the operator's receiver.
//!
//! Nothing here can brick the device: the image goes to RAM, never to flash, so
//! a failed or interrupted upload is undone by a power cycle.

use std::time::Duration;

use nusb::MaybeFuture;
use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient};

use crate::error::{Error, Result};

/// The firmware image shipped with sdroxide. See `firmware/PROVENANCE.md`.
pub const BUNDLED: &[u8] = include_bytes!("../firmware/SDDC_FX3.img");

/// The bootloader's one vendor request: read/write internal memory.
const RW_INTERNAL: u8 = 0xa0;

/// The bootloader accepts at most 4 KiB of payload per request.
const MAX_CHUNK: usize = 4096;

/// Generous — the boot ROM answers in microseconds — but a wedged device must
/// not hang the caller forever.
const CTRL_TIMEOUT: Duration = Duration::from_millis(1000);

/// An upper bound on a sane image, so a corrupt length field allocates nothing.
/// The FX3 has 512 KiB of usable RAM; the real image is ~128 KiB.
const MAX_IMAGE: usize = 512 * 1024;

/// One `{address, payload}` record from the image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Section<'a> {
    pub addr: u32,
    pub data: &'a [u8],
}

/// A parsed, checksum-verified FX3 RAM image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image<'a> {
    pub sections: Vec<Section<'a>>,
    /// Where the bootloader is told to jump once every section is written.
    pub entry: u32,
}

impl<'a> Image<'a> {
    /// Parse and fully validate an image, including its checksum.
    ///
    /// Validation is strict on purpose. The alternative to rejecting a bad image
    /// here is uploading nonsense to the device and then trying to explain why a
    /// receiver that enumerated perfectly well produces no samples.
    pub fn parse(bytes: &'a [u8]) -> Result<Image<'a>> {
        let bad = |m: String| Error::BadFirmware(m);

        if bytes.len() < 8 {
            return Err(bad(format!("only {} bytes long", bytes.len())));
        }
        if &bytes[0..2] != b"CY" {
            return Err(bad(format!(
                "no \"CY\" signature (starts with {:02x?}) — is this really an FX3 .img?",
                &bytes[0..2]
            )));
        }
        // bytes[2] is imagectl (bit 0 selects the image type family); bytes[3]
        // is the image type, 0xB0 for a normal firmware image. The bootloader
        // only executes 0xB0, so anything else would upload and never run.
        let image_type = bytes[3];
        if image_type != 0xb0 {
            return Err(bad(format!(
                "image type is 0x{image_type:02x}, not 0xb0 (normal firmware)"
            )));
        }

        let mut off = 4usize;
        let mut checksum: u32 = 0;
        let mut sections = Vec::new();
        let mut total = 0usize;

        let read_u32 = |off: usize| -> Option<u32> {
            let b = bytes.get(off..off + 4)?;
            Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };

        let entry = loop {
            let words = read_u32(off).ok_or_else(|| bad("truncated section header".into()))?;
            let addr = read_u32(off + 4).ok_or_else(|| bad("truncated section header".into()))?;
            off += 8;

            // A zero-length section is the terminator, and its address field is
            // the program entry point rather than a load address.
            if words == 0 {
                break addr;
            }

            let len = (words as usize)
                .checked_mul(4)
                .filter(|n| *n <= MAX_IMAGE)
                .ok_or_else(|| bad(format!("section claims {words} words, which is absurd")))?;
            total += len;
            if total > MAX_IMAGE {
                return Err(bad(format!("image payload exceeds {MAX_IMAGE} bytes")));
            }

            let data = bytes
                .get(off..off + len)
                .ok_or_else(|| bad(format!("section at 0x{addr:08x} runs past the end")))?;
            off += len;

            // The checksum is over 32-bit words, so it must be accumulated from
            // the words and not from the bytes.
            for w in data.chunks_exact(4) {
                checksum = checksum.wrapping_add(u32::from_le_bytes([w[0], w[1], w[2], w[3]]));
            }
            sections.push(Section { addr, data });
        };

        if sections.is_empty() {
            return Err(bad("contains no sections".into()));
        }

        let expected =
            read_u32(off).ok_or_else(|| bad("truncated: no trailing checksum".into()))?;
        if checksum != expected {
            return Err(bad(format!(
                "checksum mismatch: computed 0x{checksum:08x}, image says 0x{expected:08x}"
            )));
        }

        Ok(Image { sections, entry })
    }

    /// Total payload bytes across all sections.
    pub fn payload_len(&self) -> usize {
        self.sections.iter().map(|s| s.data.len()).sum()
    }
}

/// Write every section into the device's RAM and jump to the entry point.
///
/// Each chunk is read back and compared, exactly as `fx3_load_ram` does. That
/// doubles the EP0 traffic for an upload that takes well under a second, and it
/// is worth it: a marginal cable corrupts the image silently, and the resulting
/// symptom — firmware that enumerates but misbehaves — is far harder to
/// diagnose than "verify failed at 0x40013000".
///
/// The caller must have claimed the bootloader's interface. On return the device
/// is on its way off the bus; use [`crate::usb::wait_for_programmed`] to pick it
/// up again under its new id.
pub fn upload(iface: &nusb::Interface, image: &Image<'_>) -> Result<()> {
    for section in &image.sections {
        let mut addr = section.addr;
        for chunk in section.data.chunks(MAX_CHUNK) {
            write_ram(iface, addr, chunk)?;

            let echo = read_ram(iface, addr, chunk.len())?;
            if echo != chunk {
                let at = echo.iter().zip(chunk).position(|(a, b)| a != b).unwrap_or(0);
                return Err(Error::BadFirmware(format!(
                    "read-back mismatch at 0x{:08x}: wrote 0x{:02x}, read 0x{:02x} \
                     — this usually means a failing USB cable or port",
                    addr as usize + at,
                    chunk[at],
                    echo[at],
                )));
            }
            addr += chunk.len() as u32;
        }
        tracing::debug!("FX3: loaded {} bytes at 0x{:08x}", section.data.len(), section.addr);
    }

    jump(iface, image.entry)
}

/// Tell the bootloader to start executing at `entry`.
///
/// The device disconnects as it does so, which means the status stage may never
/// come back. A transport-level failure here is the *expected* outcome, not an
/// error — libusb's loader tolerates `LIBUSB_ERROR_IO` for the same reason.
fn jump(iface: &nusb::Interface, entry: u32) -> Result<()> {
    tracing::info!("FX3: jumping to program entry 0x{entry:08x}");
    match iface
        .control_out(
            ControlOut {
                control_type: ControlType::Vendor,
                recipient: Recipient::Device,
                request: RW_INTERNAL,
                value: (entry & 0xffff) as u16,
                index: (entry >> 16) as u16,
                data: &[],
            },
            CTRL_TIMEOUT,
        )
        .wait()
    {
        Ok(()) => Ok(()),
        Err(e) => {
            tracing::debug!("FX3: jump returned {e} (expected — the device is leaving the bus)");
            Ok(())
        }
    }
}

fn write_ram(iface: &nusb::Interface, addr: u32, data: &[u8]) -> Result<()> {
    iface
        .control_out(
            ControlOut {
                control_type: ControlType::Vendor,
                recipient: Recipient::Device,
                request: RW_INTERNAL,
                value: (addr & 0xffff) as u16,
                index: (addr >> 16) as u16,
                data,
            },
            CTRL_TIMEOUT,
        )
        .wait()
        .map_err(|source| Error::Transfer { op: "firmware write", source })
}

fn read_ram(iface: &nusb::Interface, addr: u32, len: usize) -> Result<Vec<u8>> {
    iface
        .control_in(
            ControlIn {
                control_type: ControlType::Vendor,
                recipient: Recipient::Device,
                request: RW_INTERNAL,
                value: (addr & 0xffff) as u16,
                index: (addr >> 16) as u16,
                length: len as u16,
            },
            CTRL_TIMEOUT,
        )
        .wait()
        .map_err(|source| Error::Transfer { op: "firmware read-back", source })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a well-formed image the same way the Cypress tooling does, so the
    /// parser is tested against the format rather than against itself.
    fn build(sections: &[(u32, &[u8])], entry: u32, corrupt_checksum: bool) -> Vec<u8> {
        let mut out = vec![b'C', b'Y', 0x1c, 0xb0];
        let mut sum: u32 = 0;
        for (addr, data) in sections {
            assert_eq!(data.len() % 4, 0, "sections are whole words");
            out.extend_from_slice(&((data.len() / 4) as u32).to_le_bytes());
            out.extend_from_slice(&addr.to_le_bytes());
            out.extend_from_slice(data);
            for w in data.chunks_exact(4) {
                sum = sum.wrapping_add(u32::from_le_bytes([w[0], w[1], w[2], w[3]]));
            }
        }
        out.extend_from_slice(&0u32.to_le_bytes()); // terminator
        out.extend_from_slice(&entry.to_le_bytes());
        out.extend_from_slice(&sum.wrapping_add(corrupt_checksum as u32).to_le_bytes());
        out
    }

    #[test]
    fn round_trips_a_synthetic_image() {
        let a: Vec<u8> = (0..64u8).collect();
        let b: Vec<u8> = (0..128u8).rev().collect();
        let raw = build(&[(0x0000_0100, &a), (0x4000_3000, &b)], 0x4000_e8bc, false);

        let img = Image::parse(&raw).expect("valid image parses");
        assert_eq!(img.entry, 0x4000_e8bc);
        assert_eq!(img.sections.len(), 2);
        assert_eq!(img.sections[0], Section { addr: 0x0000_0100, data: &a[..] });
        assert_eq!(img.sections[1], Section { addr: 0x4000_3000, data: &b[..] });
        assert_eq!(img.payload_len(), 192);
    }

    #[test]
    fn a_wrong_checksum_is_rejected() {
        let raw = build(&[(0x4000_3000, &[1, 2, 3, 4])], 0x4000_0000, true);
        let e = Image::parse(&raw).unwrap_err().to_string();
        assert!(e.contains("checksum mismatch"), "{e}");
    }

    #[test]
    fn a_missing_cy_signature_is_rejected() {
        let mut raw = build(&[(0x4000_3000, &[1, 2, 3, 4])], 0x4000_0000, false);
        raw[0] = b'X';
        assert!(Image::parse(&raw).unwrap_err().to_string().contains("CY"));
    }

    #[test]
    fn a_non_firmware_image_type_is_rejected() {
        let mut raw = build(&[(0x4000_3000, &[1, 2, 3, 4])], 0x4000_0000, false);
        raw[3] = 0xb2;
        assert!(Image::parse(&raw).unwrap_err().to_string().contains("0xb2"));
    }

    #[test]
    fn a_truncated_section_is_rejected_rather_than_read_past_the_end() {
        let raw = build(&[(0x4000_3000, &[1, 2, 3, 4, 5, 6, 7, 8])], 0x4000_0000, false);
        // Drop the checksum, the entry, the terminator and half the payload.
        let e = Image::parse(&raw[..raw.len() - 16]).unwrap_err().to_string();
        assert!(e.contains("runs past the end") || e.contains("truncated"), "{e}");
    }

    #[test]
    fn an_absurd_section_length_allocates_nothing() {
        let mut raw = build(&[(0x4000_3000, &[1, 2, 3, 4])], 0x4000_0000, false);
        raw[4..8].copy_from_slice(&0xffff_ffffu32.to_le_bytes());
        assert!(Image::parse(&raw).is_err());
    }

    /// The vendored artifact is a binary that no reviewer will read; this pins
    /// its exact shape so a bad re-download cannot slip through. The values come
    /// from parsing the real file — see `firmware/PROVENANCE.md`.
    #[test]
    fn the_bundled_image_parses() {
        let img = Image::parse(BUNDLED).expect("the vendored firmware is a valid FX3 image");
        assert_eq!(img.entry, 0x4000_e8bc);
        assert_eq!(
            img.sections.iter().map(|s| (s.addr, s.data.len())).collect::<Vec<_>>(),
            vec![
                (0x0000_0100, 9088),
                (0x4000_3000, 65536),
                (0x4001_3000, 45016),
                (0x4003_0000, 9056),
            ]
        );
        assert_eq!(img.payload_len(), 128_696);
        assert_eq!(BUNDLED.len(), 128_744);
    }

    /// Chunking is where an off-by-one costs a corrupted upload, so assert the
    /// split the uploader will actually perform on the real image.
    #[test]
    fn the_bundled_image_chunks_into_whole_4k_writes() {
        let img = Image::parse(BUNDLED).unwrap();
        for s in &img.sections {
            let chunks: Vec<usize> = s.data.chunks(MAX_CHUNK).map(<[u8]>::len).collect();
            assert_eq!(chunks.iter().sum::<usize>(), s.data.len());
            assert!(chunks.iter().all(|n| *n <= MAX_CHUNK));
            // Only the final chunk of a section may be short.
            assert!(chunks[..chunks.len() - 1].iter().all(|n| *n == MAX_CHUNK));
        }
    }
}
