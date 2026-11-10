//! Cloud cover, and the thunderstorms inside it.
//!
//! The source is NOAA/NESDIS's Global Mosaic of Geostationary Satellite
//! Imagery, served through nowCOAST's WMS: GOES-East and GOES-West, both
//! Meteosats and Himawari, stitched into one picture of the whole planet every
//! hour. No key, no registration, no quota — it is a public service of a public
//! agency, and one `GetMap` returns the entire world in a single PNG.
//!
//! Two channels are fetched, because neither alone is the whole truth.
//!
//! * **Longwave infrared** is the backbone. It works in the dark, and what it
//!   measures is how *cold* a cloud top is — which, through the lapse rate of
//!   the atmosphere, is how *high* it stands. That is the number that makes
//!   this layer a volume rather than a picture: the renderer is handed a height
//!   field taken from data, so a thunderhead towers because it really is
//!   fifteen kilometres tall and the stratus beside it lies flat because it
//!   really is one.
//! * **Visible** is a correction, and only on the sunlit half. Low warm cloud
//!   is nearly invisible to infrared — the top of a marine stratus deck is
//!   within a few kelvin of the sea under it — and blindingly obvious in
//!   visible light. Where the Sun is up, that channel fills in what the first
//!   one cannot see.
//!
//! **The trap.** Brightness in the infrared picture is not cloud. The winter
//! hemisphere is bright because the *ground* is cold, and the Sahara is dark
//! because the ground is hot; thresholding raw brightness would bury the
//! southern hemisphere under imaginary cloud every July. So every reading here
//! is taken against a locally estimated clear-sky background, and cloud is only
//! what stands above the ground it is over. The cost of that choice is at the
//! other end: an overcast broader than the window used to estimate it sets its
//! own background and so reads thinner than it is. A cloud field is a
//! difference, and a difference needs something to be different from.
//!
//! The lightning is not fetched at all, because no free worldwide feed of
//! individual strikes exists to fetch. What is fetched is where the deep
//! convection is: cold-top area is the oldest satellite proxy there is for
//! flash rate, and it is a good one. [`ConvCell`] carries those storms and how
//! hard each is working, and the renderer strikes inside them.

use serde::{Deserialize, Serialize};

/// nowCOAST's satellite WMS. Time-enabled, hourly, global.
pub const WMS: &str = "https://nowcoast.noaa.gov/geoserver/satellite/wms";

/// How far from the equator a ring of geostationary satellites gets any picture
/// at all. Poleward of this the mosaic is transparent, and so is this layer:
/// blank is the truth there, and it is where the aurora lives anyway.
pub const DATA_LAT: f32 = 72.75;

/// Grid width: one column per ⅓ degree of longitude.
pub const GRID_W: usize = 1024;
/// Grid height, pole to pole, at the same ⅓ degree. This is exactly the size
/// the images are requested at, so the grid *is* the picture — no resampling,
/// and the layout is already the equirectangular one the land mask and the
/// auroral oval use, which is the sphere mesh's own UVs.
pub const GRID_H: usize = 512;

/// Image size asked of the WMS: exactly the grid, pole to pole, so nothing is
/// resampled on the way in and the picture lands on the sphere's own UVs. 280 kB
/// a channel; half this is visibly coarse on a globe filling the window, twice
/// it is 830 kB for detail a sphere a few hundred pixels across cannot show.
pub const REQ_W: u32 = GRID_W as u32;
pub const REQ_H: u32 = GRID_H as u32;

/// The brightness ramp the mosaic is styled with, as brightness temperature.
/// The product is a picture rather than a calibrated field, so this is a fit to
/// the standard infrared enhancement rather than something published: black is
/// a hot desert surface, white is the top of a tropical storm.
const BT_HOT_K: f32 = 330.0;
const BT_COLD_K: f32 = 180.0;

/// Environmental lapse rate. Turns "this cloud top is 60 K colder than the
/// ground around it" into "this cloud top is nine kilometres up".
const LAPSE_K_PER_KM: f32 = 6.5;

/// Ceiling on cloud-top height. The tropical tropopause is around 17 km and
/// only the most violent overshooting tops pass it; anything the arithmetic
/// puts above this is noise in the styling, not a taller cloud.
pub const TOP_MAX_KM: f32 = 18.0;

/// How much the grid is shrunk before the clear-sky background is estimated on
/// it. Eight cells is 2.8°, finer than any surface feature the background needs
/// to follow and coarse enough to make the filtering below free.
const BG_SHRINK: usize = 8;
/// Radius of the opening below, in shrunken cells: ±11°, so the window is a
/// little over 22° across. Wide enough to swallow a mesoscale storm whole,
/// narrow enough that the background still separates a hot desert from the cool
/// sea beside it.
const BG_RADIUS: usize = 4;

/// Cloud-top height at which a tower is deep enough to be making lightning.
/// Below this is weather; above it is a thunderstorm.
const CONVECTION_KM: f32 = 10.5;
/// How many storms are kept. Sixty-four is more than are ever on the visible
/// half of the globe at once, and the list is sorted so the ones that are
/// dropped are always the weakest.
pub const MAX_CELLS: usize = 64;
/// Coarse grid the storms are found on: 1.4°, about 150 km, which is the scale
/// of a single mesoscale system.
const CELL_STRIDE: usize = 4;

/// Which channel of the mosaic.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Band {
    /// Longwave infrared, ~11 µm. Day and night; cold means high.
    Longwave,
    /// Visible. Only half the planet at a time, but it sees low cloud.
    Visible,
}

impl Band {
    pub fn layer(self) -> &'static str {
        match self {
            Band::Longwave => "global_longwave_imagery_mosaic",
            Band::Visible => "global_visible_imagery_mosaic",
        }
    }

    pub fn cache_name(self) -> &'static str {
        match self {
            Band::Longwave => "clouds-lir.png",
            Band::Visible => "clouds-vis.png",
        }
    }

    /// One `GetMap` for the whole world.
    ///
    /// `TIME` is deliberately left off: the server then serves its newest hour,
    /// which is exactly what is wanted and saves having to guess how far behind
    /// real time the mosaic currently is. It says which hour it chose in a
    /// header — see [`frame_unix`].
    ///
    /// `TRANSPARENT=true` is not decoration. Without it the polar caps come
    /// back as solid white, and white in an infrared picture means a cloud top
    /// at the tropopause: an anvil over each pole, all day, every day. With it
    /// they arrive with an alpha of zero and can be told apart from a reading.
    pub fn url(self) -> String {
        format!(
            "{WMS}?SERVICE=WMS&VERSION=1.3.0&REQUEST=GetMap\
             &LAYERS={layer}&STYLES=reflectance&CRS=CRS:84\
             &BBOX=-180,-90,180,90&WIDTH={w}&HEIGHT={h}\
             &FORMAT=image/png&TRANSPARENT=true",
            layer = self.layer(),
            w = REQ_W,
            h = REQ_H,
        )
    }
}

/// The hour the mosaic is *of*, out of GeoServer's `Warning` header.
///
/// This is the only place the frame's own time appears, and it matters: the
/// global mosaics are published hourly and run over an hour behind the clock,
/// so calling the moment of the fetch the moment of the observation would
/// overstate freshness by more than the refresh interval. The header reads
///
/// ```text
/// Warning: 99 Default value used: time=2026-07-29T18:00:00.000Z ISO8601 (satellite:global_longwave_imagery_mosaic)
/// ```
///
/// and says `Nearest value used` instead when a `TIME` was asked for.
pub fn frame_unix(warning: &str) -> Option<i64> {
    let rest = warning.split("time=").nth(1)?;
    crate::timefmt::parse_unix(rest.split_whitespace().next()?)
}

/// One decoded channel.
///
/// `excess` is brightness above the local clear-sky background, 0–255, laid out
/// row-major from the north pole down and from 180° west eastward — the same
/// equirectangular convention as the land mask and the auroral oval, so the
/// renderer addresses it with the sphere's own UVs. Anywhere no satellite looks
/// is zero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plane {
    pub band: Band,
    /// The hour the picture is *of*. See [`frame_unix`].
    pub frame_unix: i64,
    /// When it was fetched, which is over an hour later.
    pub fetched_unix: i64,
    pub excess: Vec<u8>,
}

/// The frame time to assume for a picture recovered from disk, where the header
/// that carried it is long gone.
///
/// The mosaics are published on the hour and were already an hour or so old
/// when they were fetched, so the hour before the fetch is the safe guess: it
/// is right most of the time and, when it is wrong, it is wrong in the
/// direction of calling the picture older than it is. Overstating freshness is
/// the failure that matters.
pub fn assumed_frame_unix(fetched_unix: i64) -> i64 {
    fetched_unix.div_euclid(3600) * 3600 - 3600
}

/// A thunderstorm: where it is, how tall, and how hard it is working.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConvCell {
    pub lat: f32,
    pub lon: f32,
    /// Radius of a circle with the storm's area, degrees. Flashes are scattered
    /// within it rather than all striking the centre.
    pub radius_deg: f32,
    /// Height of the tallest top in it, kilometres.
    pub top_km: f32,
    /// Flashes per second, from cold-top area and how far the tops overshoot.
    pub flash_rate: f32,
}

/// What the renderer is handed: an equirectangular cloud field and the storms in
/// it.
#[derive(Clone, Debug, PartialEq)]
pub struct CloudField {
    /// The hour the infrared picture this was built from is of — not when it
    /// was fetched, which is over an hour later.
    pub frame_unix: i64,
    /// Optical thickness, 0–255. Same layout as [`Plane::excess`].
    pub opacity: Vec<u8>,
    /// Cloud-top height, 0–255 over 0..[`TOP_MAX_KM`]. Same layout.
    pub top: Vec<u8>,
    /// Deep convection, strongest first.
    pub cells: Vec<ConvCell>,
    /// Whether the visible channel contributed. False at once means either the
    /// fetch failed or it has not landed yet, and the picture is infrared only.
    pub has_visible: bool,
}

impl CloudField {
    pub fn is_empty(&self) -> bool {
        self.opacity.len() != GRID_W * GRID_H
    }

    /// Optical thickness at a point, 0..1. For the panels, not the renderer.
    pub fn opacity_at(&self, lat: f32, lon: f32) -> f32 {
        match self.index_of(lat, lon) {
            Some(i) => self.opacity[i] as f32 / 255.0,
            None => 0.0,
        }
    }

    fn index_of(&self, lat: f32, lon: f32) -> Option<usize> {
        if self.is_empty() || !(-90.0..=90.0).contains(&lat) {
            return None;
        }
        let row = (((90.0 - lat) / 180.0) * GRID_H as f32) as usize;
        let col = ((lon + 180.0).rem_euclid(360.0) / 360.0 * GRID_W as f32) as usize;
        Some(row.min(GRID_H - 1) * GRID_W + col.min(GRID_W - 1))
    }
}

/// Latitude of the centre of grid row `row`, degrees.
pub fn row_lat(row: usize) -> f32 {
    90.0 - (row as f32 + 0.5) * 180.0 / GRID_H as f32
}

/// Longitude of the centre of grid column `col`, degrees east.
pub fn col_lon(col: usize) -> f32 {
    -180.0 + (col as f32 + 0.5) * 360.0 / GRID_W as f32
}

/// Decode one channel's PNG and reduce it to brightness above local clear sky.
///
/// Returns `None` for anything that is not a decodable image — an HTML error
/// page from the WMS included — so a bad fetch leaves the previous picture on
/// the globe rather than replacing it with nothing.
pub fn parse_plane(band: Band, png: &[u8], frame_unix: i64, fetched_unix: i64) -> Option<Plane> {
    let img = image::load_from_memory(png)
        .map_err(|e| tracing::warn!("cloud {band:?} decode failed: {e}"))
        .ok()?;
    let src = img.to_luma_alpha8();
    let (sw, sh) = (src.width() as usize, src.height() as usize);
    if sw == 0 || sh == 0 {
        return None;
    }

    // At the size actually requested both mappings are the identity; the
    // arithmetic is here so a smaller test image, or a WMS that one day serves
    // something else, still lands in the right place rather than skewed.
    let mut value = vec![0u8; GRID_W * GRID_H];
    let mut seen = vec![false; GRID_W * GRID_H];
    for row in 0..GRID_H {
        let sy = (row * sh / GRID_H).min(sh - 1);
        for col in 0..GRID_W {
            let sx = (col * sw / GRID_W).min(sw - 1);
            let px = src.get_pixel(sx as u32, sy as u32).0;
            // Alpha zero is the WMS saying "no satellite sees this", which is
            // not the same as "no cloud here" and must not become a reading.
            // Without it the polar caps arrive as solid white and every frame
            // grows an anvil over each pole.
            if px[1] < 8 {
                continue;
            }
            let i = row * GRID_W + col;
            value[i] = px[0];
            seen[i] = true;
        }
    }
    // The mosaic has a one-pixel transparent seam down the antimeridian, where
    // its two edges meet. Left alone that is a hairline of missing weather
    // running pole to pole, so a hole one pixel wide is closed from the columns
    // either side of it — which wrap, because longitude does.
    close_seams(&mut value, &mut seen);

    let bg = clear_sky_background(&value, &seen);
    let mut excess = vec![0u8; GRID_W * GRID_H];
    for i in 0..excess.len() {
        if seen[i] {
            excess[i] = value[i].saturating_sub(bg[i]);
        }
    }
    Some(Plane { band, frame_unix, fetched_unix, excess })
}

/// Fill single-column gaps from their neighbours.
fn close_seams(value: &mut [u8], seen: &mut [bool]) {
    // Against a snapshot, so filling one column cannot make the next one look
    // fillable and smear a wide gap shut a pixel at a time.
    let was = seen.to_vec();
    for row in 0..GRID_H {
        for col in 0..GRID_W {
            let i = row * GRID_W + col;
            if was[i] {
                continue;
            }
            let l = row * GRID_W + (col + GRID_W - 1) % GRID_W;
            let r = row * GRID_W + (col + 1) % GRID_W;
            if was[l] && was[r] {
                value[i] = ((value[l] as u16 + value[r] as u16) / 2) as u8;
                seen[i] = true;
            }
        }
    }
}

/// Estimate what the picture would look like with the clouds taken out.
///
/// A greyscale *opening* — shrink the picture, take a minimum over a wide
/// window, then take a maximum back over the same window — on a coarse copy of
/// the grid, interpolated back up.
///
/// The choice of an opening rather than the obvious low percentile is the whole
/// point. Both remove clouds; only the opening leaves a *sloping* surface
/// alone. Under a percentile, the smooth pole-to-equator temperature gradient
/// comes out biased by a fixed fraction of its slope, which is a thin haze
/// painted over the entire planet that no amount of threshold tuning removes.
/// A minimum followed by a maximum reproduces any ramp wider than the window
/// exactly, and deletes anything brighter and narrower than it — which is the
/// definition of a cloud sitting on a background.
///
/// What it cannot do is find the background under an overcast broader than the
/// window; a frontal band a third of a hemisphere wide sets its own clear sky
/// and reads thinner than it is. That is the price of measuring cloud as a
/// difference, and there is nothing in a single picture to pay it with.
fn clear_sky_background(value: &[u8], seen: &[bool]) -> Vec<u8> {
    let (bw, bh) = (GRID_W / BG_SHRINK, GRID_H / BG_SHRINK);

    // Shrink by minimum. The first stage of the erosion, for free.
    let mut small = vec![255u8; bw * bh];
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            let i = y * GRID_W + x;
            // Somewhere no satellite sees stays at the top of the range, so the
            // minimum below steps over it instead of reading it as scorching
            // ground and inventing cloud along the edge of coverage.
            if seen[i] {
                let c = (y / BG_SHRINK) * bw + x / BG_SHRINK;
                small[c] = small[c].min(value[i]);
            }
        }
    }

    let eroded = separable(&small, bw, bh, u8::min);
    let opened = separable(&eroded, bw, bh, u8::max);

    // Back up to full size, bilinearly, sampling from cell centres so the
    // result is a smooth surface rather than a chequerboard. Longitude wraps;
    // latitude clamps.
    let mut out = vec![0u8; GRID_W * GRID_H];
    for y in 0..GRID_H {
        let fy = (y as f32 / BG_SHRINK as f32 - 0.5).clamp(0.0, bh as f32 - 1.0);
        let (y0, ty) = (fy.floor() as usize, fy.fract());
        let y1 = (y0 + 1).min(bh - 1);
        for x in 0..GRID_W {
            let fx = x as f32 / BG_SHRINK as f32 - 0.5;
            let x0 = fx.div_euclid(1.0).rem_euclid(bw as f32) as usize;
            let x1 = (x0 + 1) % bw;
            let tx = fx.rem_euclid(1.0);
            let top = lerp(opened[y0 * bw + x0], opened[y0 * bw + x1], tx);
            let bot = lerp(opened[y1 * bw + x0], opened[y1 * bw + x1], tx);
            out[y * GRID_W + x] = (top * (1.0 - ty) + bot * ty).clamp(0.0, 255.0) as u8;
        }
    }
    out
}

/// A square min or max filter of radius [`BG_RADIUS`], run as two passes.
///
/// Separable because a square window is: the extreme over a box is the extreme
/// along each row of the extremes down each column. Two O(n·r) passes instead of
/// one O(n·r²) one, on a grid small enough that either would do.
fn separable(src: &[u8], w: usize, h: usize, pick: fn(u8, u8) -> u8) -> Vec<u8> {
    let r = BG_RADIUS as isize;
    let mut mid = vec![0u8; w * h];
    for y in 0..h {
        for x in 0..w {
            let mut acc = src[y * w + x];
            for d in -r..=r {
                // Longitude wraps: a storm on the date line is one storm.
                let nx = (x as isize + d).rem_euclid(w as isize) as usize;
                acc = pick(acc, src[y * w + nx]);
            }
            mid[y * w + x] = acc;
        }
    }
    let mut out = vec![0u8; w * h];
    for y in 0..h {
        for x in 0..w {
            let mut acc = mid[y * w + x];
            for d in -r..=r {
                let ny = (y as isize + d).clamp(0, h as isize - 1) as usize;
                acc = pick(acc, mid[ny * w + x]);
            }
            out[y * w + x] = acc;
        }
    }
    out
}

fn lerp(a: u8, b: u8, t: f32) -> f32 {
    a as f32 * (1.0 - t) + b as f32 * t
}

/// Build the field the renderer draws from one or both channels.
///
/// Infrared alone is a complete picture. Visible alone is not — it is half a
/// planet, and it cannot tell how high anything is — so it only ever refines a
/// field the infrared has already established.
pub fn combine(ir: &Plane, vis: Option<&Plane>) -> CloudField {
    let n = GRID_W * GRID_H;
    let mut opacity = vec![0u8; n];
    let mut top = vec![0u8; n];

    // Where the Sun was when the visible picture was taken, so that channel is
    // only believed where there was light to see by. The frame's own hour, not
    // the hour it was fetched: those are twenty degrees of longitude apart, and
    // the terminator is the whole point of the test.
    let sub =
        vis.map(|v| crate::ephem::subsolar_point(crate::ephem::julian_day(v.frame_unix as f64)));

    let k_per_count = (BT_HOT_K - BT_COLD_K) / 255.0;
    for row in 0..GRID_H {
        let lat = row_lat(row);
        // A geostationary satellite sees the high latitudes at a grazing angle
        // through a great depth of atmosphere, and past about 60° what it
        // returns is more geometry than weather — NOAA quotes the mosaic's
        // coverage as 60° N to 60° S even though the grid runs further. So the
        // picture is faded out across that band rather than trusted to the edge
        // and then cut, which would draw a straight line of cloud around the
        // planet where the data stops.
        let edge = smoothstep(DATA_LAT - 0.25, 62.0, lat.abs());
        for col in 0..GRID_W {
            let i = row * GRID_W + col;
            let deficit_k = ir.excess[i] as f32 * k_per_count * edge;
            let top_km = (deficit_k / LAPSE_K_PER_KM).min(TOP_MAX_KM);

            // Optical thickness against depth of cloud, which is Beer's law: a
            // wisp of cirrus is a fifth of the way to opaque, a fair-weather
            // cumulus half, and a storm effectively all of it. A straight ramp
            // was tried first and turns the planet into a black-and-white
            // stencil — nearly every cloud on Earth is past the top of it.
            let mut op = 1.0 - (-(deficit_k - 2.0).max(0.0) / 20.0).exp();

            if let (Some(v), Some((slat, slon))) = (vis, sub) {
                let sun = cos_solar_zenith(lat, col_lon(col), slat as f32, slon as f32);
                let lit = smoothstep(0.12, 0.42, sun);
                // Only ever *adds*, and only into what the infrared left empty:
                // this is here to find the low deck the first channel is blind
                // to, not to argue with it about the tall cloud it saw.
                let low = smoothstep(0.10, 0.50, v.excess[i] as f32 / 255.0);
                op += (1.0 - op) * low * lit * 0.8;
            }

            opacity[i] = (op * 255.0).clamp(0.0, 255.0) as u8;
            top[i] = (top_km / TOP_MAX_KM * 255.0).clamp(0.0, 255.0) as u8;
        }
    }

    let cells = find_cells(&top);
    CloudField { frame_unix: ir.frame_unix, opacity, top, cells, has_visible: vis.is_some() }
}

/// Find the deep convection: connected runs of very cold tops.
///
/// Done on a coarse grid, because a thunderstorm is a hundred kilometres across
/// and finding it at a third of a degree would be counting pixels rather than
/// storms. Longitude wraps, so a system sitting on the date line stays one
/// system.
fn find_cells(top: &[u8]) -> Vec<ConvCell> {
    let (cw, ch) = (GRID_W / CELL_STRIDE, GRID_H / CELL_STRIDE);
    let thresh = (CONVECTION_KM / TOP_MAX_KM * 255.0) as u8;

    // Coarse grid holds the tallest top in each block: a storm is defined by
    // its peak, and averaging would dissolve it into the shield around it.
    let mut coarse = vec![0u8; cw * ch];
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            let c = (y / CELL_STRIDE) * cw + x / CELL_STRIDE;
            coarse[c] = coarse[c].max(top[y * GRID_W + x]);
        }
    }

    let mut visited = vec![false; cw * ch];
    let mut cells: Vec<ConvCell> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    for start in 0..cw * ch {
        if visited[start] || coarse[start] < thresh {
            continue;
        }
        // Flood fill. Longitude is summed as a unit vector so a storm straddling
        // ±180° averages to the date line rather than to the prime meridian.
        let (mut n, mut peak) = (0usize, 0u8);
        let (mut sx, mut sy, mut slat) = (0f64, 0f64, 0f64);
        stack.push(start);
        visited[start] = true;
        while let Some(c) = stack.pop() {
            let (cy, cx) = (c / cw, c % cw);
            n += 1;
            peak = peak.max(coarse[c]);
            let lat = row_lat(cy * CELL_STRIDE + CELL_STRIDE / 2);
            let lon = (col_lon(cx * CELL_STRIDE + CELL_STRIDE / 2) as f64).to_radians();
            slat += lat as f64;
            sx += lon.cos();
            sy += lon.sin();
            for (dy, dx) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let ny = cy as i32 + dy;
                if ny < 0 || ny >= ch as i32 {
                    continue;
                }
                let nx = (cx as i32 + dx).rem_euclid(cw as i32) as usize;
                let ni = ny as usize * cw + nx;
                if !visited[ni] && coarse[ni] >= thresh {
                    visited[ni] = true;
                    stack.push(ni);
                }
            }
        }

        let lat = (slat / n as f64) as f32;
        let lon = sy.atan2(sx).to_degrees() as f32;
        let top_km = peak as f32 / 255.0 * TOP_MAX_KM;

        // One coarse block is CELL_STRIDE grid cells square; its area shrinks
        // with the cosine of the latitude like every other cell on a sphere.
        let deg = CELL_STRIDE as f32 * 360.0 / GRID_W as f32;
        let km_per_deg = 111.0;
        let area_km2 = n as f32 * (deg * km_per_deg).powi(2) * lat.to_radians().cos().max(0.05);
        let radius_deg = (n as f32 / core::f32::consts::PI).sqrt() * deg;

        // Flash rate. A mesoscale system a few hundred kilometres across
        // flashes a couple of times a second; an isolated cell that has only
        // just reached the threshold flashes every several seconds. The
        // overshoot term is what separates the two, because a tower that has
        // punched well past the convection threshold is the one with the
        // updraught under it.
        let overshoot = 0.2 + 0.8 * smoothstep(CONVECTION_KM, 15.0, top_km);
        let flash_rate = (area_km2 / 400_000.0 * overshoot).clamp(0.04, 4.0);
        cells.push(ConvCell { lat, lon, radius_deg, top_km, flash_rate });
    }

    cells.sort_by(|a, b| b.flash_rate.total_cmp(&a.flash_rate));
    cells.truncate(MAX_CELLS);
    cells
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    if (b - a).abs() < f32::EPSILON {
        return if x >= b { 1.0 } else { 0.0 };
    }
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Cosine of the solar zenith angle: 1 with the Sun overhead, 0 at the horizon.
fn cos_solar_zenith(lat: f32, lon: f32, sub_lat: f32, sub_lon: f32) -> f32 {
    let (a, b) = (lat.to_radians(), sub_lat.to_radians());
    let h = (lon - sub_lon).to_radians();
    a.sin() * b.sin() + a.cos() * b.cos() * h.cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A test mosaic. `f` returns `None` where the WMS would send a transparent
    /// pixel — outside what the geostationary ring sees — and a brightness
    /// where it would send a reading.
    fn png_of(w: u32, h: u32, f: impl Fn(f32, f32) -> Option<u8>) -> Vec<u8> {
        let img = image::GrayAlphaImage::from_fn(w, h, |x, y| {
            let lat = 90.0 - (y as f32 + 0.5) * 180.0 / h as f32;
            let lon = -180.0 + (x as f32 + 0.5) * 360.0 / w as f32;
            match f(lat, lon) {
                Some(v) => image::LumaA([v, 255]),
                None => image::LumaA([255, 0]),
            }
        });
        let mut out = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageLumaA8(img)
            .write_to(&mut out, image::ImageFormat::Png)
            .expect("encode");
        out.into_inner()
    }

    /// Only where a satellite looks.
    fn covered(lat: f32, v: u8) -> Option<u8> {
        (lat.abs() <= DATA_LAT).then_some(v)
    }

    fn parse(band: Band, png: &[u8]) -> Plane {
        parse_plane(band, png, 1_784_937_600, 1_784_942_000).expect("parse")
    }

    /// The grid is the same equirectangular layout as the land mask and the
    /// auroral oval, and every consumer assumes it: row zero is the north pole,
    /// column zero is 180° west. Getting this wrong would put the weather on the
    /// wrong side of the planet, which is exactly the kind of mistake that looks
    /// plausible until someone who lives there notices.
    #[test]
    fn the_grid_runs_north_to_south_and_starts_at_the_date_line() {
        assert!(row_lat(0) > 89.0, "row 0 is not the north pole");
        assert!(row_lat(GRID_H - 1) < -89.0, "the last row is not the south pole");
        assert!(col_lon(0) < -179.0, "column 0 is not 180° west");
        assert!(col_lon(GRID_W - 1) > 179.0, "the last column is not 180° east");
    }

    /// Transparent means *unknown*, and unknown is not clear sky.
    ///
    /// This is the one the mosaic sets a trap for: with `TRANSPARENT` off, the
    /// polar caps arrive as solid white, and white in an infrared picture is a
    /// cloud top at the tropopause. The alpha channel is the only thing that
    /// separates "no cloud" from "nobody looked", and it has to be honoured.
    #[test]
    fn the_poles_stay_empty_because_nothing_looks_at_them() {
        let plane = parse(Band::Longwave, &png_of(256, 128, |lat, _| covered(lat, 255)));
        for row in 0..GRID_H {
            if row_lat(row).abs() <= DATA_LAT {
                continue;
            }
            for col in 0..GRID_W {
                assert_eq!(plane.excess[row * GRID_W + col], 0, "row {row} is outside coverage");
            }
        }
        let field = combine(&plane, None);
        assert_eq!(field.opacity_at(85.0, 0.0), 0.0, "cloud drawn over the north pole");
        assert_eq!(field.opacity_at(-85.0, 0.0), 0.0, "cloud drawn over the south pole");
    }

    /// The one that matters. A cold but *cloudless* winter hemisphere is bright
    /// in the infrared, and reading brightness as cloud would bury half the
    /// planet under weather that is not there. Only brightness above the local
    /// ground may count.
    #[test]
    fn a_uniformly_cold_but_clear_hemisphere_is_not_cloud() {
        // A July surface with no cloud at all: warm in the northern tropics,
        // cold towards the southern high latitudes, sloping smoothly between.
        // In the infrared that is a picture running from black to nearly white,
        // and none of it is weather.
        let png =
            png_of(256, 128, |lat, _| covered(lat, (120.0 - lat * 1.3).clamp(0.0, 255.0) as u8));
        let plane = parse(Band::Longwave, &png);
        let peak = plane.excess.iter().copied().max().unwrap_or(0);
        assert!(peak < 24, "a clear temperature gradient produced cloud (peak excess {peak})");

        let field = combine(&plane, None);
        let thickest = field.opacity.iter().copied().max().unwrap_or(0);
        assert!(thickest < 40, "a clear temperature gradient produced opacity {thickest}");
        assert!(field.cells.is_empty(), "a clear sky produced {} storms", field.cells.len());
    }

    /// A cold blob on a warm background is a thunderstorm, and it has to come
    /// out at the latitude and longitude it was put in at.
    #[test]
    fn a_cold_blob_becomes_a_storm_where_it_was_put() {
        let png = png_of(360, 180, |lat, lon| {
            let cold = (lon - 30.0).abs() < 6.0 && (lat - 15.0).abs() < 6.0;
            covered(lat, if cold { 250 } else { 30 })
        });
        let field = combine(&parse(Band::Longwave, &png), None);
        let cell = field.cells.first().expect("no storm found");
        assert!((cell.lat - 15.0).abs() < 3.0, "storm at {}° N", cell.lat);
        assert!((cell.lon - 30.0).abs() < 3.0, "storm at {}° E", cell.lon);
        assert!(cell.top_km > CONVECTION_KM, "storm top only {} km", cell.top_km);
        assert!(cell.flash_rate > 0.0, "a storm with no lightning");
        assert!(field.opacity_at(15.0, 30.0) > 0.5, "the storm is transparent");
        assert!(field.opacity_at(-40.0, -120.0) < 0.2, "cloud where there is none");
    }

    /// Longitude wraps, and a system sitting on the date line is one system.
    /// The flood fill is the subtlest arithmetic in the file and this is the
    /// case it gets wrong if the wrap is dropped.
    #[test]
    fn a_storm_on_the_date_line_stays_one_storm() {
        let png = png_of(360, 180, |lat, lon| {
            // Straddling ±180°, so it is split across both edges of the image.
            let dlon = (lon.abs() - 180.0).abs();
            covered(lat, if dlon < 6.0 && lat.abs() < 6.0 { 250 } else { 30 })
        });
        let field = combine(&parse(Band::Longwave, &png), None);
        assert_eq!(field.cells.len(), 1, "the date line cut a storm in two");
        let cell = field.cells[0];
        assert!(cell.lon.abs() > 174.0, "storm centred at {}° E, not on the date line", cell.lon);
        assert!(cell.lat.abs() < 3.0, "storm centred at {}° N", cell.lat);
    }

    /// The visible channel is only evidence where the Sun is up. At local
    /// midnight it says nothing, and "nothing" must not be read as "clear".
    #[test]
    fn the_visible_channel_only_counts_on_the_day_side() {
        // 2024-06-20T12:00:00Z: noon on the Greenwich meridian, midnight on the
        // date line. Infrared sees a uniform warm surface — no cloud at all.
        let frame = 1_718_884_800;
        let ir =
            parse_plane(Band::Longwave, &png_of(256, 128, |lat, _| covered(lat, 40)), frame, frame)
                .expect("ir");
        // Visible sees a bright deck on both meridians. Only the lit one is
        // believable; the other is a satellite looking at an unlit ocean.
        let vis = parse_plane(
            Band::Visible,
            // Both decks are kept well inside the window the clear-sky estimate
            // uses; one broader than that would set its own background and read
            // as clear, which is the documented limit of measuring cloud as a
            // difference and not what this test is about.
            &png_of(256, 128, |lat, lon| {
                let deck = lat.abs() < 6.0 && ((lon.abs() < 6.0) || (lon.abs() > 174.0));
                covered(lat, if deck { 230 } else { 20 })
            }),
            frame,
            frame,
        )
        .expect("vis");

        let field = combine(&ir, Some(&vis));
        assert!(field.has_visible);
        assert!(field.opacity_at(0.0, 0.0) > 0.3, "the sunlit deck was not picked up");
        assert!(
            field.opacity_at(0.0, 180.0) < 0.1,
            "a deck at local midnight was read out of the visible channel"
        );
        // And with no visible channel at all, the infrared answer is unchanged.
        assert!(combine(&ir, None).opacity_at(0.0, 0.0) < 0.1);
    }

    /// A visible frame from a different hour would put yesterday's daylight
    /// over today's cloud, so the field is built from the infrared frame's own
    /// time and the pairing is checked upstream — here, only that the field
    /// reports the frame it was built from rather than when it was fetched.
    #[test]
    fn the_field_reports_the_hour_it_is_of_not_when_it_arrived() {
        let png = png_of(64, 32, |lat, _| covered(lat, 40));
        let plane = parse_plane(Band::Longwave, &png, 1_784_937_600, 1_784_942_000).expect("parse");
        assert_eq!(combine(&plane, None).frame_unix, 1_784_937_600);
    }

    /// Rubbish in — an HTML error page, a truncated body — must leave the last
    /// good picture alone rather than blanking the globe.
    #[test]
    fn an_undecodable_body_is_refused() {
        assert!(parse_plane(Band::Longwave, b"<html>down for maintenance</html>", 0, 0).is_none());
        assert!(parse_plane(Band::Visible, &[], 0, 0).is_none());
    }

    /// The frame's own hour lives in a response header and nowhere else.
    #[test]
    fn the_frame_time_is_read_out_of_the_geoserver_warning() {
        // Both forms the service actually sends.
        assert_eq!(
            frame_unix(
                "99 Default value used: time=2026-07-29T18:00:00.000Z ISO8601 \
                 (satellite:global_longwave_imagery_mosaic)"
            ),
            Some(1_785_348_000)
        );
        assert_eq!(
            frame_unix("99 Nearest value used: time=2026-07-29T17:00:00.000Z ISO8601 (x)"),
            Some(1_785_344_400)
        );
        assert_eq!(frame_unix("99 Default value used: elevation=0 (x)"), None);
        assert_eq!(frame_unix(""), None);
    }

    /// Recovered from disk there is no header, so the hour before the fetch is
    /// assumed — never a later one, because overstating freshness is the
    /// failure that matters.
    #[test]
    fn a_cached_frame_is_never_called_fresher_than_it_is() {
        // 2026-07-29T19:14:53Z was fetched; the newest frame then was 18:00Z.
        let fetched = 1_785_352_493;
        assert_eq!(assumed_frame_unix(fetched), 1_785_348_000);
        assert!(assumed_frame_unix(fetched) < fetched);
    }

    /// Every request has to be a complete WMS GetMap for the whole world, with
    /// the transparency that separates "no cloud" from "nobody looked".
    #[test]
    fn the_request_asks_for_the_whole_world() {
        for band in [Band::Longwave, Band::Visible] {
            let url = band.url();
            assert!(url.starts_with(WMS), "{url}");
            assert!(url.contains("REQUEST=GetMap"), "{url}");
            assert!(url.contains(band.layer()), "{url}");
            assert!(url.contains("BBOX=-180,-90,180,90"), "{url}");
            assert!(url.contains("WIDTH=1024&HEIGHT=512"), "{url}");
            assert!(url.contains("FORMAT=image/png"), "{url}");
            assert!(url.contains("TRANSPARENT=true"), "{url}");
        }
        assert_ne!(Band::Longwave.url(), Band::Visible.url());
        assert_ne!(Band::Longwave.cache_name(), Band::Visible.cache_name());
    }
}
