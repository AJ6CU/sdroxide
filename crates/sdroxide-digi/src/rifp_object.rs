//! The RIFP *object*: the encoded image a session carries, and the JSON
//! manifest that describes it (draft-dulaunoy-rifp-00 §7).
//!
//! Everything here is grayscale on purpose. The manifest's `bits_per_pixel` is
//! defined as the "nominal decoded grayscale depth", and the vendor raster
//! media type is a packed grayscale raster — colour has no place in the model,
//! so a picture is converted to luma and quantised before it is encoded.

use std::io::Cursor;

use image::{ImageFormat, ImageReader};
use sdroxide_types::RifpEncoding;
use sha2::{Digest, Sha256};

use sdroxide_dsp::rifp::crc32;

/// Largest object a receiver will reassemble.
pub const MAX_OBJECT_BYTES: u32 = 4 * 1024 * 1024;
/// Largest decoded picture a receiver will build.
pub const MAX_PIXELS: u32 = 4_000_000;
/// Largest side a receiver will accept.
pub const MAX_SIDE: u16 = 4096;
/// Largest manifest a receiver will parse.
pub const MAX_MANIFEST_BYTES: usize = 16 * 1024;

/// A picture encoded for transmission.
pub struct Encoded {
    pub encoding: RifpEncoding,
    pub bytes: Vec<u8>,
    pub width: u16,
    pub height: u16,
    /// The depth the manifest declares — the bilevel facsimile encodings force
    /// this to 1 whatever was asked for.
    pub bits_per_pixel: u8,
    /// Size of the unencoded packed raster, for the manifest's `raw_size`.
    pub raw_size: u32,
    /// The quantised grayscale the object was built from, so a sender can show
    /// exactly what it put on the air.
    pub gray: Vec<u8>,
}

/// Quantise `gray` (w·h luma bytes, modified in place) to `bits_per_pixel`,
/// optionally with Floyd–Steinberg error diffusion. Values come back expanded
/// to the full 0..=255 range, which is what both the raster packer and the
/// PNG/JPEG encoders want.
pub fn quantize(gray: &mut [u8], w: u16, h: u16, bits_per_pixel: u8, dither: bool) {
    let (w, h) = (w as usize, h as usize);
    if bits_per_pixel >= 8 || gray.len() < w * h {
        return;
    }
    let levels = ((1u16 << bits_per_pixel) - 1) as f32;
    if !dither {
        for p in gray.iter_mut() {
            let q = (*p as f32 * levels / 255.0).round().clamp(0.0, levels);
            *p = (q * 255.0 / levels).round() as u8;
        }
        return;
    }
    // Error diffusion needs headroom on both sides of the range, so the working
    // buffer is float; the errors are pushed forward onto pixels not yet
    // quantised, exactly as the reference implementation's Pillow call does.
    let mut buf: Vec<f32> = gray.iter().map(|&p| p as f32).collect();
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let old = buf[i];
            let q = (old * levels / 255.0).round().clamp(0.0, levels);
            let new = (q * 255.0 / levels).round();
            buf[i] = new;
            let err = old - new;
            let mut spread = |dx: usize, dy: usize, right: bool, factor: f32| {
                let nx = if right { x + dx } else { x.wrapping_sub(dx) };
                let ny = y + dy;
                if nx < w && ny < h {
                    buf[ny * w + nx] += err * factor;
                }
            };
            spread(1, 0, true, 7.0 / 16.0);
            spread(1, 1, false, 3.0 / 16.0);
            spread(0, 1, true, 5.0 / 16.0);
            spread(1, 1, true, 1.0 / 16.0);
        }
    }
    for (p, v) in gray.iter_mut().zip(buf) {
        *p = v.round().clamp(0.0, 255.0) as u8;
    }
}

/// Pack quantised grayscale into the vendor raster: row-major, most
/// significant sample first, no per-row padding — only the complete pixel
/// sequence is padded to an octet boundary (§7.1).
pub fn pack_raster(gray: &[u8], bits_per_pixel: u8) -> Vec<u8> {
    if bits_per_pixel == 8 {
        return gray.to_vec();
    }
    let levels = ((1u16 << bits_per_pixel) - 1) as f32;
    let mut out = Vec::with_capacity(raster_len(gray.len() as u32, bits_per_pixel) as usize);
    let mut acc = 0u8;
    let mut used = 0u8;
    for &p in gray {
        let q = (p as f32 * levels / 255.0).round().clamp(0.0, levels) as u8;
        acc = (acc << bits_per_pixel) | q;
        used += bits_per_pixel;
        if used == 8 {
            out.push(acc);
            acc = 0;
            used = 0;
        }
    }
    if used > 0 {
        out.push(acc << (8 - used));
    }
    out
}

/// Unpack the vendor raster back to one luma byte per pixel.
pub fn unpack_raster(
    data: &[u8],
    width: u16,
    height: u16,
    bits_per_pixel: u8,
) -> Result<Vec<u8>, String> {
    let pixels = width as usize * height as usize;
    let want = raster_len(pixels as u32, bits_per_pixel) as usize;
    if data.len() < want {
        return Err(format!("truncated raster: {} of {want} octets", data.len()));
    }
    let levels = ((1u16 << bits_per_pixel) - 1) as f32;
    let mask = ((1u16 << bits_per_pixel) - 1) as u8;
    let mut out = Vec::with_capacity(pixels);
    'outer: for &byte in data {
        let mut shift = 8i8 - bits_per_pixel as i8;
        while shift >= 0 {
            let q = (byte >> shift) & mask;
            out.push((q as f32 * 255.0 / levels).round() as u8);
            if out.len() == pixels {
                break 'outer;
            }
            shift -= bits_per_pixel as i8;
        }
    }
    Ok(out)
}

/// Octets a packed raster of `pixels` samples occupies.
pub fn raster_len(pixels: u32, bits_per_pixel: u8) -> u32 {
    (pixels * bits_per_pixel as u32).div_ceil(8)
}

/// Byte-oriented run-length coding: `(count 1..=255, value)` pairs (§7.1).
pub fn rle8_encode(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let value = data[i];
        let mut run = 1;
        while i + run < data.len() && data[i + run] == value && run < 255 {
            run += 1;
        }
        out.push(run as u8);
        out.push(value);
        i += run;
    }
    out
}

/// Inverse of [`rle8_encode`]. A zero run length is invalid (§7.1), and the
/// output is bounded so a hostile stream cannot expand without limit.
pub fn rle8_decode(data: &[u8], max_out: usize) -> Result<Vec<u8>, String> {
    if !data.len().is_multiple_of(2) {
        return Err("odd-length RLE8 payload".into());
    }
    let mut out = Vec::new();
    for pair in data.chunks_exact(2) {
        let (count, value) = (pair[0], pair[1]);
        if count == 0 {
            return Err("zero run length in RLE8 payload".into());
        }
        if out.len() + count as usize > max_out {
            return Err("RLE8 payload expands past the declared raster".into());
        }
        out.extend(std::iter::repeat_n(value, count as usize));
    }
    Ok(out)
}

/// Encode a picture for transmission. [`RifpEncoding::Auto`] tries every
/// encoding this build can produce and keeps the smallest — lossy JPEG
/// excluded, as in the reference implementation, because "smallest" is a poor
/// reason to throw detail away without being asked.
pub fn encode_object(
    rgb: &[u8],
    width: u16,
    height: u16,
    encoding: RifpEncoding,
    bits_per_pixel: u8,
    dither: bool,
) -> Result<Encoded, String> {
    let bpp = match bits_per_pixel {
        1 | 2 | 4 | 8 => bits_per_pixel,
        _ => return Err(format!("unsupported grayscale depth {bits_per_pixel}")),
    };
    // The facsimile encodings are bilevel by definition.
    let bpp = if encoding.is_bilevel() { 1 } else { bpp };
    let pixels = width as usize * height as usize;
    if rgb.len() < pixels * 3 {
        return Err("transmit image is smaller than its declared size".into());
    }
    let mut gray: Vec<u8> = rgb[..pixels * 3]
        .chunks_exact(3)
        .map(|p| {
            // Rec. 601 luma, matching the reference implementation's PIL "L".
            (0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32).round() as u8
        })
        .collect();
    quantize(&mut gray, width, height, bpp, dither);
    let raster = pack_raster(&gray, bpp);
    let raw_size = raster.len() as u32;

    let build = |enc: RifpEncoding| -> Result<Vec<u8>, String> {
        match enc {
            RifpEncoding::Raw => Ok(raster.clone()),
            RifpEncoding::Rle8 => Ok(rle8_encode(&raster)),
            RifpEncoding::Zlib => Ok(zlib_compress(&raster)),
            RifpEncoding::Png => encode_image(&gray, width, height, ImageFormat::Png),
            RifpEncoding::Jpeg => encode_image(&gray, width, height, ImageFormat::Jpeg),
            RifpEncoding::Group4 => encode_group4(&gray, width, height),
            RifpEncoding::Group3 => {
                Err("Group 3 is receive-only in this build; pick Group 4".into())
            }
            RifpEncoding::Auto => unreachable!("Auto is resolved by the caller"),
        }
    };

    let (encoding, bytes) = if encoding == RifpEncoding::Auto {
        let mut candidates = vec![RifpEncoding::Png, RifpEncoding::Zlib, RifpEncoding::Rle8];
        if bpp == 1 {
            candidates.insert(0, RifpEncoding::Group4);
        }
        let mut best: Option<(RifpEncoding, Vec<u8>)> = None;
        let mut failures = Vec::new();
        for candidate in candidates {
            match build(candidate) {
                Ok(bytes) => {
                    if best.as_ref().is_none_or(|(_, b)| bytes.len() < b.len()) {
                        best = Some((candidate, bytes));
                    }
                }
                Err(e) => failures.push(format!("{}: {e}", candidate.label())),
            }
        }
        best.ok_or_else(|| format!("no encoding succeeded ({})", failures.join("; ")))?
    } else {
        (encoding, build(encoding)?)
    };

    Ok(Encoded { encoding, bytes, width, height, bits_per_pixel: bpp, raw_size, gray })
}

/// Decode a complete object back to one luma byte per pixel.
pub fn decode_object(
    payload: &[u8],
    encoding: RifpEncoding,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
) -> Result<Vec<u8>, String> {
    let pixels = width as usize * height as usize;
    match encoding {
        RifpEncoding::Png => decode_image(payload, ImageFormat::Png, width, height),
        RifpEncoding::Jpeg => decode_image(payload, ImageFormat::Jpeg, width, height),
        RifpEncoding::Group4 => decode_image(payload, ImageFormat::Tiff, width, height),
        RifpEncoding::Group3 => {
            Err("Group 3 facsimile is not decodable by this build (Group 4, PNG, JPEG and the \
             raster encodings are)"
                .into())
        }
        RifpEncoding::Raw => unpack_raster(payload, width, height, bits_per_pixel),
        RifpEncoding::Rle8 => {
            let want = raster_len(pixels as u32, bits_per_pixel) as usize;
            let raster = rle8_decode(payload, want)?;
            unpack_raster(&raster, width, height, bits_per_pixel)
        }
        RifpEncoding::Zlib => {
            let want = raster_len(pixels as u32, bits_per_pixel) as usize;
            let raster = zlib_decompress(payload, want)?;
            unpack_raster(&raster, width, height, bits_per_pixel)
        }
        RifpEncoding::Auto => Err("no content encoding in the manifest".into()),
    }
}

fn zlib_compress(data: &[u8]) -> Vec<u8> {
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write;
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
    enc.write_all(data).expect("write to Vec cannot fail");
    enc.finish().expect("flush to Vec cannot fail")
}

fn zlib_decompress(data: &[u8], max_out: usize) -> Result<Vec<u8>, String> {
    use flate2::write::ZlibDecoder;
    use std::io::Write;
    // Bounded: a decompression bomb must not be able to allocate its way
    // through the machine (§12).
    let mut dec = ZlibDecoder::new(Vec::with_capacity(max_out.min(1 << 20)));
    dec.write_all(data).map_err(|e| format!("zlib: {e}"))?;
    let out = dec.finish().map_err(|e| format!("zlib: {e}"))?;
    if out.len() > max_out {
        return Err("zlib payload expands past the declared raster".into());
    }
    Ok(out)
}

fn encode_image(gray: &[u8], w: u16, h: u16, format: ImageFormat) -> Result<Vec<u8>, String> {
    let img = image::GrayImage::from_raw(w as u32, h as u32, gray.to_vec())
        .ok_or("grayscale buffer does not match the image size")?;
    let mut out = Cursor::new(Vec::new());
    image::DynamicImage::ImageLuma8(img)
        .write_to(&mut out, format)
        .map_err(|e| format!("{format:?} encode: {e}"))?;
    Ok(out.into_inner())
}

fn decode_image(
    payload: &[u8],
    format: ImageFormat,
    width: u16,
    height: u16,
) -> Result<Vec<u8>, String> {
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_SIDE as u32);
    limits.max_image_height = Some(MAX_SIDE as u32);
    limits.max_alloc = Some(64 * 1024 * 1024);
    let mut reader = ImageReader::with_format(Cursor::new(payload), format);
    reader.limits(limits);
    let img = reader.decode().map_err(|e| format!("{format:?} decode: {e}"))?;
    if img.width() != width as u32 || img.height() != height as u32 {
        return Err(format!(
            "decoded {}×{} does not match the manifest's {width}×{height}",
            img.width(),
            img.height()
        ));
    }
    Ok(img.to_luma8().into_raw())
}

/// CCITT Group 4 (T.6) in a minimal TIFF wrapper, as `image/tiff` +
/// `ccitt-group4`. Pixels at or above mid-grey are white.
fn encode_group4(gray: &[u8], w: u16, h: u16) -> Result<Vec<u8>, String> {
    use fax::{Color, VecWriter, encoder::Encoder};
    let (wi, hi) = (w as usize, h as usize);
    if gray.len() < wi * hi {
        return Err("grayscale buffer does not match the image size".into());
    }
    let mut encoder = Encoder::new(VecWriter::new());
    for y in 0..hi {
        let row = &gray[y * wi..(y + 1) * wi];
        let pels = row.iter().map(|&p| if p >= 128 { Color::White } else { Color::Black });
        encoder.encode_line(pels, w as u32).map_err(|_| "Group 4 encode failed".to_string())?;
    }
    let writer = encoder.finish().map_err(|_| "Group 4 encode failed".to_string())?;
    Ok(fax::tiff::wrap(&writer.finish(), w as u32, h as u32))
}

/// SHA-256 of the complete object (§8).
pub fn sha256_hex(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// SHA-256 of the complete object, as the 32 raw octets an END frame carries.
pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// CRC-32 of the complete object (§8).
pub fn object_crc32(data: &[u8]) -> u32 {
    crc32(data)
}

// ───────────────────────────── the manifest ─────────────────────────────

/// The manifest as it travels: a JSON object with the fourteen required fields
/// of §7 plus the optional ones. Unknown members are ignored, as the draft
/// requires.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    pub protocol: String,
    pub protocol_version: String,
    /// Session ID as exactly sixteen lowercase hexadecimal digits.
    pub session_id: String,
    pub filename: String,
    pub media_type: String,
    pub content_encoding: String,
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u32,
    pub encoded_size: u64,
    pub payload_crc32: u32,
    pub payload_sha256: String,
    pub chunk_size: u32,
    pub chunk_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radio_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
}

impl Manifest {
    /// Serialise exactly as the reference implementation does: compact, keys
    /// sorted, ASCII only — so two senders describing the same object produce
    /// the same octets and a receiver's duplicate check succeeds.
    pub fn to_json(&self) -> Vec<u8> {
        let value = serde_json::to_value(self).expect("manifest is plain data");
        // serde_json's Map is a BTreeMap unless `preserve_order` is on, so
        // re-serialising the value sorts the keys.
        let text = serde_json::to_string(&value).expect("manifest is plain data");
        escape_non_ascii(&text).into_bytes()
    }

    /// Parse and sanity-check a MANIFEST payload against the frame it arrived
    /// in. Every limit here exists because the sender is untrusted (§12).
    pub fn parse(payload: &[u8], session_id: u64, total: u32) -> Result<Manifest, String> {
        if payload.len() > MAX_MANIFEST_BYTES {
            return Err(format!("manifest of {} octets is too large", payload.len()));
        }
        let text = std::str::from_utf8(payload).map_err(|_| "manifest is not valid UTF-8")?;
        let m: Manifest = serde_json::from_str(text).map_err(|e| format!("manifest: {e}"))?;

        if m.protocol != "rifp" {
            return Err(format!("not a RIFP manifest (protocol {:?})", m.protocol));
        }
        if !m.protocol_version.starts_with("1.") {
            return Err(format!("unsupported manifest version {}", m.protocol_version));
        }
        if m.session_id != format!("{session_id:016x}") {
            return Err("manifest session ID does not match the frame header".into());
        }
        if m.chunk_count != total {
            return Err("manifest chunk count does not match the frame header".into());
        }
        if m.width == 0
            || m.height == 0
            || m.width > MAX_SIDE as u32
            || m.height > MAX_SIDE as u32
            || m.width * m.height > MAX_PIXELS
        {
            return Err(format!("image dimensions {}×{} exceed local limits", m.width, m.height));
        }
        if !matches!(m.bits_per_pixel, 1 | 2 | 4 | 8) {
            return Err(format!("unsupported grayscale depth {}", m.bits_per_pixel));
        }
        if m.encoded_size == 0 || m.encoded_size > MAX_OBJECT_BYTES as u64 {
            return Err(format!("object of {} octets exceeds local limits", m.encoded_size));
        }
        if m.chunk_size == 0 || m.chunk_size as usize > sdroxide_dsp::rifp::MAX_PAYLOAD_LEN {
            return Err(format!("implausible chunk size {}", m.chunk_size));
        }
        if m.chunk_count == 0 || m.chunk_count as u64 > m.encoded_size {
            return Err(format!("implausible chunk count {}", m.chunk_count));
        }
        if m.payload_sha256.len() != 64 || !m.payload_sha256.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err("manifest digest is not a 64-digit hex string".into());
        }
        Ok(m)
    }

    /// The encoding this manifest names, if it is one we know.
    pub fn encoding(&self) -> Option<RifpEncoding> {
        RifpEncoding::from_manifest(&self.media_type, &self.content_encoding)
    }

    /// The filename, reduced to something that can never escape a directory
    /// (§12: never use a received filename as an unrestricted path).
    pub fn safe_filename(&self) -> String {
        let base = self.filename.rsplit(['/', '\\']).next().unwrap_or_default();
        let cleaned: String = base
            .chars()
            .map(
                |c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') { c } else { '_' },
            )
            .collect();
        let trimmed = cleaned.trim_matches(['.', '_']).to_string();
        if trimmed.is_empty() { "image".to_string() } else { trimmed }
    }
}

/// Escape every non-ASCII character as `\uXXXX`, matching the reference
/// implementation's `ensure_ascii=True`. Keeps a manifest byte-identical
/// between senders and safe through anything that is not 8-bit clean.
fn escape_non_ascii(text: &str) -> String {
    if text.is_ascii() {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if ch.is_ascii() {
            out.push(ch);
        } else {
            let mut buf = [0u16; 2];
            for unit in ch.encode_utf16(&mut buf) {
                out.push_str(&format!("\\u{unit:04x}"));
            }
        }
    }
    out
}

/// Format a Unix timestamp as the manifest's `created` (RFC 3339, UTC, whole
/// seconds).
pub fn rfc3339_utc(unix: i64) -> String {
    let (y, mo, d, h, mi, s) = sdroxide_types::utc_ymd_hms(unix);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

/// A fresh 64-bit session ID. The draft asks for a cryptographically secure
/// generator; a collision between two stations on the same channel would
/// silently merge two pictures.
pub fn new_session_id() -> u64 {
    let mut bytes = [0u8; 8];
    if getrandom::fill(&mut bytes).is_err() {
        // No entropy source is a situation with no good answer; a clock-derived
        // ID is still overwhelmingly likely to be unique on one channel.
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        bytes = nanos.rotate_left(17).to_be_bytes();
    }
    u64::from_be_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ramp(w: u16, h: u16) -> Vec<u8> {
        (0..w as usize * h as usize)
            .flat_map(|i| {
                let v = (i % 256) as u8;
                [v, v.wrapping_add(40), v.wrapping_sub(70)]
            })
            .collect()
    }

    #[test]
    fn raster_packs_and_unpacks_at_every_depth() {
        for bpp in [1u8, 2, 4, 8] {
            // 13×3 is deliberately not a whole number of octets at 2 or 4 bpp.
            let (w, h) = (13u16, 3u16);
            let mut gray: Vec<u8> = (0..(w as usize * h as usize)).map(|i| (i * 7) as u8).collect();
            quantize(&mut gray, w, h, bpp, false);
            let packed = pack_raster(&gray, bpp);
            assert_eq!(packed.len(), raster_len((w * h) as u32, bpp) as usize, "bpp {bpp}");
            let back = unpack_raster(&packed, w, h, bpp).unwrap();
            assert_eq!(back, gray, "bpp {bpp} did not survive the round trip");
        }
    }

    #[test]
    fn rle8_round_trip() {
        let data = [vec![0u8; 300], vec![1, 2, 3], vec![9; 7]].concat();
        let encoded = rle8_encode(&data);
        // 300 zeros cannot be one run: the count is a single octet.
        assert_eq!(&encoded[..4], &[255, 0, 45, 0]);
        assert_eq!(rle8_decode(&encoded, data.len()).unwrap(), data);
        assert!(rle8_decode(&encoded, data.len() - 1).is_err());
        assert!(rle8_decode(&[0, 5], 100).is_err(), "zero run length must be rejected");
        assert!(rle8_decode(&[1], 100).is_err(), "odd length must be rejected");
    }

    #[test]
    fn every_encoding_round_trips() {
        let (w, h) = (64u16, 48u16);
        let rgb = ramp(w, h);
        for (encoding, bpp) in [
            (RifpEncoding::Raw, 8),
            (RifpEncoding::Rle8, 4),
            (RifpEncoding::Zlib, 2),
            (RifpEncoding::Png, 8),
            (RifpEncoding::Group4, 1),
        ] {
            let enc = encode_object(&rgb, w, h, encoding, bpp, false)
                .unwrap_or_else(|e| panic!("{encoding:?}: {e}"));
            assert_eq!(enc.encoding, encoding);
            let back = decode_object(&enc.bytes, enc.encoding, w, h, enc.bits_per_pixel)
                .unwrap_or_else(|e| panic!("{encoding:?}: {e}"));
            assert_eq!(back, enc.gray, "{encoding:?} did not survive the round trip");
        }
    }

    #[test]
    fn auto_picks_the_smallest() {
        let (w, h) = (64u16, 48u16);
        // A flat picture: run-length and deflate both crush it, raw does not.
        let rgb = vec![200u8; w as usize * h as usize * 3];
        let auto = encode_object(&rgb, w, h, RifpEncoding::Auto, 4, false).unwrap();
        let raw = encode_object(&rgb, w, h, RifpEncoding::Raw, 4, false).unwrap();
        assert!(auto.bytes.len() < raw.bytes.len());
        assert_ne!(auto.encoding, RifpEncoding::Auto);
        assert!(!auto.encoding.is_lossy(), "Auto must not choose a lossy encoding");
    }

    #[test]
    fn manifest_round_trips_and_is_checked() {
        let m = Manifest {
            protocol: "rifp".into(),
            protocol_version: "1.0".into(),
            session_id: "0123456789abcdef".into(),
            filename: "weather-map.png".into(),
            media_type: "image/png".into(),
            content_encoding: "identity".into(),
            width: 128,
            height: 96,
            bits_per_pixel: 1,
            encoded_size: 246,
            payload_crc32: 188_765_321,
            payload_sha256: "0".repeat(64),
            chunk_size: 192,
            chunk_count: 2,
            raw_size: Some(1536),
            created: Some("2026-07-25T08:00:00Z".into()),
            radio_profile: Some("rifp-cpfsk-4800".into()),
            extensions: None,
        };
        let json = m.to_json();
        let text = std::str::from_utf8(&json).unwrap();
        assert!(text.starts_with(r#"{"bits_per_pixel":1,"chunk_count":2,"#), "{text}");
        assert!(!text.contains(' '), "manifest must be compact: {text}");

        let parsed = Manifest::parse(&json, 0x0123_4567_89ab_cdef, 2).unwrap();
        assert_eq!(parsed.encoding(), Some(RifpEncoding::Png));
        // The header is authoritative: a manifest that disagrees is refused.
        assert!(Manifest::parse(&json, 1, 2).is_err());
        assert!(Manifest::parse(&json, 0x0123_4567_89ab_cdef, 3).is_err());
    }

    #[test]
    fn filenames_cannot_escape() {
        let mut m = Manifest::parse(
            &Manifest {
                protocol: "rifp".into(),
                protocol_version: "1.0".into(),
                session_id: "0000000000000001".into(),
                filename: "../../etc/passwd".into(),
                media_type: "image/png".into(),
                content_encoding: "identity".into(),
                width: 8,
                height: 8,
                bits_per_pixel: 8,
                encoded_size: 10,
                payload_crc32: 0,
                payload_sha256: "a".repeat(64),
                chunk_size: 192,
                chunk_count: 1,
                raw_size: None,
                created: None,
                radio_profile: None,
                extensions: None,
            }
            .to_json(),
            1,
            1,
        )
        .unwrap();
        assert_eq!(m.safe_filename(), "passwd");
        m.filename = "  ".into();
        assert_eq!(m.safe_filename(), "image");
        m.filename = "C:\\windows\\system32\\cmd.exe".into();
        assert_eq!(m.safe_filename(), "cmd.exe");
    }

    #[test]
    fn rfc3339_matches_the_draft_example() {
        // The manifest example in §7.
        assert_eq!(rfc3339_utc(1_784_966_400), "2026-07-25T08:00:00Z");
    }
}
