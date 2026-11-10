#!/usr/bin/env python3
"""Turn the published spacecraft map data into the planet textures.

All come out 2048×1024 equirectangular, east longitude −180°…180° with 0° in
the centre and latitude +90°…−90° top to bottom — the sphere mesh's own UV
convention, so the shader indexes them directly.

    mars.jpg     MDIM 2.1, the USGS Viking mosaic tied to the MOLA control
                 network and colourised by NASA Ames, served at exactly the
                 grid this wants by the USGS planetary WMS. Requested at 2× and
                 filtered down, which is what keeps the small craters from
                 aliasing into noise.
    jupiter.jpg  PIA07782, Cassini's global map of Jupiter (Dec 2000). Already
                 a clean cylindrical map; only re-projected and resized.
    saturn.jpg   The Cassini ISS RGB global map of Saturn (2011-08-11) from the
                 PDS Atmospheres node. Saturn's own rings hide a band of it,
                 and the poles were never in view, so those latitudes are
                 interpolated from the rows either side — which is defensible
                 exactly because Saturn is zonal: whatever is at 20°N is at
                 20°N all the way round.

Sources (all public domain, USGS Astrogeology and
NASA/JPL-Caltech/Space Science Institute):

    https://planetarymaps.usgs.gov/cgi-bin/mapserv?map=/maps/mars/
        mars_simp_cyl.map (WMS layer MDIM21_color)
    https://images-assets.nasa.gov/image/PIA07782/PIA07782~orig.jpg
    https://atmos.nmsu.edu/PDS/data/PDS4/co_iss_global-maps/data_derived/
        Cassini_ISS_RGB_Saturn_global_color_map_original.fits

    python3 make_body_maps.py
"""

import os
import sys
import urllib.request

import numpy as np
from PIL import Image

W, H = 2048, 1024
CACHE = os.environ.get("BODY_MAP_CACHE", "/tmp/bodymaps")

JUPITER = "https://images-assets.nasa.gov/image/PIA07782/PIA07782~orig.jpg"
SATURN = (
    "https://atmos.nmsu.edu/PDS/data/PDS4/co_iss_global-maps/data_derived/"
    "Cassini_ISS_RGB_Saturn_global_color_map_original.fits"
)
# The WMS renders whatever grid is asked for, so the projection, the longitude
# origin and the size are all settled by the request — nothing to re-project
# afterwards. 2× the texture, then a Lanczos step down.
MARS = (
    "https://planetarymaps.usgs.gov/cgi-bin/mapserv?"
    "map=/maps/mars/mars_simp_cyl.map&SERVICE=WMS&VERSION=1.1.1&REQUEST=GetMap"
    "&LAYERS=MDIM21_color&STYLES=&SRS=EPSG:4326&BBOX=-180,-90,180,90"
    f"&WIDTH={W * 2}&HEIGHT={H * 2}&FORMAT=image/png"
)


def fetch(name: str, url: str) -> str:
    os.makedirs(CACHE, exist_ok=True)
    path = os.path.join(CACHE, name)
    if not os.path.exists(path):
        print(f"fetching {url}", file=sys.stderr)
        urllib.request.urlretrieve(url, path)
    return path


def read_fits(path: str):
    """The one FITS this script needs: a 3-plane float32 cube."""
    with open(path, "rb") as f:
        header = b""
        while True:
            block = f.read(2880)
            header += block
            if b"END     " in block:
                break
        cards = dict()
        for i in range(0, len(header), 80):
            card = header[i : i + 80].decode("ascii", "replace")
            if "=" in card:
                k, v = card.split("=", 1)
                cards[k.strip()] = v.split("/")[0].strip()
        w, h, planes = (int(cards[f"NAXIS{i}"]) for i in (1, 2, 3))
        assert cards["BITPIX"] == "-32", cards["BITPIX"]
        data = np.frombuffer(f.read(w * h * planes * 4), dtype=">f4")
    return data.reshape(planes, h, w).astype(np.float32)


def fill_gaps(rgb: np.ndarray, valid: np.ndarray) -> np.ndarray:
    """Interpolate the latitudes Cassini could not see.

    Missing rows are filled from the nearest rows that are not, and rows past
    the last valid one — the poles, on both planets — repeat it. Sound only
    because both bodies are zonal: what is at 20°N is at 20°N all the way
    round, so a filled row is a real latitude's colour rather than a guess.
    """
    idx = np.flatnonzero(valid)
    if idx.size == 0:
        raise SystemExit("the whole map is empty")
    out = rgb.copy()
    for row in np.flatnonzero(~valid):
        before = idx[idx < row]
        after = idx[idx > row]
        if before.size and after.size:
            a, b = before[-1], after[0]
            t = (row - a) / (b - a)
            out[row] = rgb[a] * (1.0 - t) + rgb[b] * t
        else:
            out[row] = rgb[before[-1] if before.size else after[0]]
    return out


def feather_seam(rgb: np.ndarray, half: int = 6) -> np.ndarray:
    """Blend across the map's own longitude seam.

    Both maps are mosaics that close on themselves at one meridian, and after
    re-centring that join lands in the middle of the texture where it reads as
    a scratch down the planet. Interpolating across a dozen columns removes it
    without touching anything else.
    """
    mid = rgb.shape[1] // 2
    lo, hi = mid - half - 1, mid + half + 1
    left, right = rgb[:, lo][:, None, :], rgb[:, hi][:, None, :]
    t = np.linspace(0.0, 1.0, hi - lo + 1)[None, :, None]
    rgb[:, lo : hi + 1] = left * (1.0 - t) + right * t
    return rgb


def to_texture(rgb: np.ndarray, median: float, contrast: float) -> Image.Image:
    """Tone-map to the brightness the eye expects, and resize.

    The published data is calibrated reflectance, which comes out washed out
    when shown directly: the mid-tone is placed deliberately instead, and the
    contrast lifted around it — the same thing every published version of these
    maps does, said out loud.
    """
    v = np.nan_to_num(rgb, nan=0.0)
    v = v * (median / max(float(np.median(v)), 1e-6))
    v = np.clip(0.5 + (v - 0.5) * contrast, 0.0, 1.0)
    im = Image.fromarray((v * 255.0 + 0.5).astype(np.uint8), "RGB")
    return im.resize((W, H), Image.LANCZOS)


def build_saturn() -> Image.Image:
    cube = read_fits(fetch("saturn_rgb.fits", SATURN))
    # (plane, row, col) -> (row, col, plane), rows south-to-north in the file.
    rgb = np.moveaxis(cube, 0, -1)
    # A row Cassini never saw is one with almost nothing lit in it: the band
    # the rings hide, and both poles.
    lit = np.nanmean(np.nanmax(rgb, axis=2), axis=1)
    rgb = fill_gaps(rgb, lit > 0.02)
    # The file runs 360°W…0°W left to right, i.e. 0°E…360°E; the texture wants
    # 0°E in the middle. And its first row is the south pole.
    rgb = feather_seam(np.roll(rgb, rgb.shape[1] // 2, axis=1))[::-1]
    # Saturn really is this bland — a mild stretch, not a false-colour one.
    return to_texture(rgb, median=0.70, contrast=1.35)


def build_mars() -> Image.Image:
    """MDIM 2.1, as the WMS renders it.

    Nothing to re-project: the request already asks for −180°…180° by −90°…90°
    on an equirectangular grid, which is the texture's layout. The only thing
    added is the stretch — the mosaic is a calibrated albedo product and comes
    out the colour of wet cardboard shown raw, while Mars through a telescope is
    the butterscotch this pushes it back towards. Restrained on purpose: the
    dark markings really are grey-blue, not black, and the caps really do blow
    out to white.
    """
    src = np.asarray(Image.open(fetch("mars_mdim21.png", MARS)).convert("RGB"), dtype=np.float32) / 255.0
    return to_texture(src, median=0.46, contrast=1.30)


def build_jupiter() -> Image.Image:
    src = np.asarray(Image.open(fetch("jupiter.jpg", JUPITER)).convert("RGB"), dtype=np.float32) / 255.0
    # The published map stops short of both poles and pads them with a flat
    # grey; a row with no variation across longitude is that padding.
    varies = src.std(axis=(1, 2)) > 0.012
    src = fill_gaps(src, varies)
    # Same longitude convention as the Saturn map: 0° at the left edge.
    src = feather_seam(np.roll(src, src.shape[1] // 2, axis=1))
    return to_texture(src, median=0.52, contrast=1.12)


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    builds = (("mars.jpg", build_mars), ("jupiter.jpg", build_jupiter), ("saturn.jpg", build_saturn))
    for name, build in builds:
        path = os.path.join(here, name)
        build().save(path, quality=88, optimize=True)
        print(f"{name}: {os.path.getsize(path) / 1024:.0f} kB", file=sys.stderr)


if __name__ == "__main__":
    main()
