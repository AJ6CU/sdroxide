#!/usr/bin/env python3
"""Fit the small-body table in `src/smallbody.rs`, and write its test fixture.

Three JPL services, all public domain:

1. **SBDB close-approach API** picks which near-Earth asteroids belong in the
   view at all. "Relevant within the next fifty years" is a query, not an
   opinion: everything that passes inside 0.02 AU of the Earth between now and
   2076 and is big enough to matter. The date and distance printed beside each
   body are that query's answer, so the caption cannot drift from the fact.

2. **Horizons ELEMENTS** gives osculating elements at the middle of the window,
   which seeds the fit.

3. **Horizons VECTORS** gives the truth the fit is measured against: geometric
   positions across 2026–2076, which also become
   `tests/fixtures/smallbodies.json`.

The model is a *chain* of Keplerian ellipses. One ellipse per body is what
`planets.rs` uses, and for a dwarf planet it is superb — Eris comes out at
0.003° across the whole fifty years. It falls apart for anything that crosses a
planet's path: fitted over the window in one piece, Encke lands 54° from where
it really is and Apophis's 2029 Earth encounter throws it 3°. So the window is
bisected until every piece holds, each piece is fitted over a slightly widened
span, and the two either side of a boundary are cross-faded so nothing jumps
when the clock is scrubbed through it.

What that costs is *measured* — through the same blend the renderer uses, not
against the individual fits — and printed as each body's `fit_error_deg`, which
the Rust tests then assert against.

    python3 fit_smallbodies.py            # refit, rewrite the fixture, print the table

Network: two requests per body; a full run is a hundred-odd and takes a few
minutes.
"""

import json
import math
import os
import sys
import time
import urllib.parse
import urllib.request

import numpy as np
from scipy.optimize import least_squares

HORIZONS = "https://ssd.jpl.nasa.gov/api/horizons.api"
CAD = "https://ssd-api.jpl.nasa.gov/cad.api"
SBDB = "https://ssd-api.jpl.nasa.gov/sbdb.api"
J2000 = 2451545.0
AU_KM = 149597870.7
# Gaussian gravitational constant, deg/day at 1 AU — the mean motion a fitted
# `n` is compared against to prove it stayed physical.
K_DEG = 0.9856076686

# The window the view is about: today to fifty years out, rounded to whole
# years so the captions read cleanly.
WIN_START = "2026-01-01"
WIN_END = "2076-01-01"

# How good the chain of ellipses has to be before the tool stops bisecting,
# degrees of heliocentric direction. A tenth of a degree is a twentieth of the
# Moon's apparent width seen from the Earth — far below anything this view can
# draw, and comfortably below the width of the dot each body is rendered as.
TOL_DEG = 0.10
# Floor on a piece's length, so a body that simply cannot be fitted (a comet
# during a Jupiter encounter) terminates instead of subdividing forever. Also
# keeps every piece comfortably longer than two cross-fades, without which the
# blend weights either side of a boundary would stop summing to one.
MIN_SEGMENT_D = 10.0
# Ceiling on the table: a body that has not converged by here publishes the
# error it actually reached instead of buying another halving with fifty more
# rows of numbers nobody can check.
MAX_SEGMENTS = 24
# How far the fitted mean motion may stray from the one Kepler's third law gives
# for the fitted semi-major axis. Some slack absorbs the along-track drift a
# perturbed orbit accumulates; too much lets the fit trade physics for residual,
# and lets `a` and `n` slide against each other on an arc shorter than one
# revolution.
MAX_N_DRIFT = 0.02
# How long the cross-fade at a boundary lasts, days. Each piece is fitted over
# its own span widened by this much, so both sides of a boundary are fitted
# where they are blended rather than extrapolated. Must stay well under half of
# `MIN_SEGMENT_D`, and `smallbody.rs` carries the same number.
BLEND_D = 0.5

# Curated bodies: dwarf planets, the large main-belt asteroids anyone can name,
# and the mission targets. Close-approach objects are *added* to this by the CAD
# query below rather than listed here.
#
# (name, designation, Horizons command, class, radius_km, why)
CURATED = [
    ("Pluto", "134340 Pluto", "134340;", "Dwarf", 1188.3,
     "Largest Kuiper-belt body; New Horizons flew past in July 2015"),
    ("Ceres", "1 Ceres", "1;", "Dwarf", 469.7,
     "Largest main-belt body and the only dwarf planet inside Neptune; Dawn orbited it 2015-18"),
    ("Eris", "136199 Eris", "136199;", "Dwarf", 1163.0,
     "More massive than Pluto — the discovery that ended Pluto's planethood"),
    ("Haumea", "136108 Haumea", "136108;", "Dwarf", 780.0,
     "Spins in under four hours, which has pulled it into an egg, and carries a ring"),
    ("Makemake", "136472 Makemake", "136472;", "Dwarf", 715.0,
     "Brightest Kuiper-belt object after Pluto"),

    ("Vesta", "4 Vesta", "4;", "Asteroid", 262.7,
     "Brightest asteroid — the only one ever visible to the naked eye; Dawn orbited it in 2011"),
    ("Pallas", "2 Pallas", "2;", "Asteroid", 255.5,
     "Third-largest main-belt body, on an orbit tilted 35 degrees out of the ecliptic"),
    ("Psyche", "16 Psyche", "16;", "Asteroid", 111.0,
     "Metal-rich remnant core; NASA's Psyche arrives in 2029"),
    ("Eros", "433 Eros", "433;", "Asteroid", 8.42,
     "Largest near-Earth asteroid; NEAR Shoemaker landed on it in 2001"),
    ("Phaethon", "3200 Phaethon", "3200;", "Asteroid", 2.72,
     "Parent of the Geminid meteor shower; sheds a dust tail at perihelion, DESTINY+ target"),
    ("Didymos", "65803 Didymos", "65803;", "Asteroid", 0.39,
     "DART struck its moon Dimorphos in 2022; ESA's Hera surveys the result from 2026"),
    ("Ryugu", "162173 Ryugu", "162173;", "Asteroid", 0.448,
     "Hayabusa2 returned 5 g of it in December 2020"),
    ("Itokawa", "25143 Itokawa", "25143;", "Asteroid", 0.165,
     "First asteroid ever sampled — Hayabusa, 2005"),
    ("Patroclus", "617 Patroclus", "617;", "Asteroid", 51.0,
     "Jupiter Trojan, and a binary of two near-equal bodies; Lucy's final flyby in 2033"),
    ("Eurybates", "3548 Eurybates", "3548;", "Asteroid", 31.9,
     "Jupiter Trojan with its own satellite; Lucy flies past in August 2027"),
    ("2024 YR4", "2024 YR4", "2024 YR4;", "Asteroid", 0.03,
     "Ruled out for Earth, but still has a few per cent chance of hitting the Moon on 22 Dec 2032"),
    ("Apophis", "99942 Apophis", "99942;", "Asteroid", 0.185,
     "Passes inside the geostationary belt — the closest approach by anything this size "
     "in recorded history, and naked-eye visible from Europe and Africa"),
    ("Bennu", "101955 Bennu", "101955;", "Asteroid", 0.245,
     "OSIRIS-REx returned 122 g of it in September 2023"),

    # Comets: the periodic ones with a perihelion inside the window. The tool
    # checks that claim per comet and drops any that fails it.
    ("Halley", "1P/Halley", "DES=1P;CAP;", "Comet", 5.5,
     "The comet; next perihelion 28 July 2061, its first since 1986"),
    ("Encke", "2P/Encke", "DES=2P;CAP;", "Comet", 2.4,
     "Shortest period of any known comet at 3.3 years; parent of the Taurids"),
    ("Tuttle", "8P/Tuttle", "DES=8P;CAP;", "Comet", 2.3,
     "Parent of the Ursids"),
    ("Tempel 1", "9P/Tempel 1", "DES=9P;CAP;", "Comet", 3.0,
     "Deep Impact fired a projectile into it in 2005"),
    ("Borrelly", "19P/Borrelly", "DES=19P;CAP;", "Comet", 2.4,
     "Deep Space 1 photographed its nucleus in 2001"),
    ("Giacobini-Zinner", "21P/Giacobini-Zinner", "DES=21P;CAP;", "Comet", 1.0,
     "Parent of the Draconids; first comet ever visited, by ICE in 1985"),
    ("Wirtanen", "46P/Wirtanen", "DES=46P;CAP;", "Comet", 0.6,
     "Hyperactive for its size, and passes close enough to be a naked-eye object"),
    ("Tempel-Tuttle", "55P/Tempel-Tuttle", "DES=55P;CAP;", "Comet", 1.8,
     "Parent of the Leonids, whose storms follow its 33-year return"),
    ("Churyumov-Gerasimenko", "67P/Churyumov-Gerasimenko", "DES=67P;CAP;", "Comet", 1.65,
     "Rosetta orbited it for two years and landed Philae on it in 2014"),
    ("Schwassmann-Wachmann 3", "73P/Schwassmann-Wachmann 3", "DES=73P;CAP;NOFRAG;", "Comet", 0.55,
     "Broke into dozens of fragments in 1995 and is still coming apart"),
    ("Wild 2", "81P/Wild 2", "DES=81P;CAP;", "Comet", 1.98,
     "Stardust flew through its coma and returned the dust in 2006"),
    ("Machholz 1", "96P/Machholz 1", "DES=96P;CAP;", "Comet", 3.2,
     "Passes 0.12 AU from the Sun — closer than any other short-period comet"),
    ("Hartley 2", "103P/Hartley 2", "DES=103P;CAP;", "Comet", 0.58,
     "EPOXI found jets of CO2 blasting ice out of it in 2010"),
]

# How many close-approach objects to add on top of the curated list, and the
# query that ranks them.
CAD_LIMIT = 10
CAD_DIST_AU = 0.02
CAD_H_MAX = 22.0


def http_json(url):
    for attempt in range(4):
        try:
            with urllib.request.urlopen(url, timeout=180) as r:
                return json.loads(r.read().decode())
        except Exception as e:
            print(f"  retry {attempt}: {e}", file=sys.stderr)
            time.sleep(3 * (attempt + 1))
    raise SystemExit(f"request failed: {url}")


def horizons(params):
    q = {"format": "text", "OBJ_DATA": "'NO'", "MAKE_EPHEM": "'YES'",
         "CENTER": "'500@10'", "REF_PLANE": "'ECLIPTIC'", "CSV_FORMAT": "'YES'"}
    q.update(params)
    url = HORIZONS + "?" + urllib.parse.urlencode(q)
    for attempt in range(4):
        try:
            with urllib.request.urlopen(url, timeout=300) as r:
                text = r.read().decode()
            if "$$SOE" in text:
                return text
            print(f"  Horizons rejected {params.get('COMMAND')}:\n{text[:600]}", file=sys.stderr)
            return None
        except Exception as e:
            print(f"  retry {attempt}: {e}", file=sys.stderr)
            time.sleep(3 * (attempt + 1))
    raise SystemExit(f"Horizons failed for {params.get('COMMAND')}")


def vectors(command, start, stop, step):
    """(N, 4) of (JD_TDB, x, y, z) in km, J2000 ecliptic, heliocentric."""
    text = horizons({"COMMAND": f"'{command}'", "EPHEM_TYPE": "'VECTORS'",
                     "VEC_TABLE": "'1'", "OUT_UNITS": "'KM-S'",
                     "START_TIME": f"'{start}'", "STOP_TIME": f"'{stop}'",
                     "STEP_SIZE": f"'{step}'"})
    if text is None:
        return None
    rows = []
    for line in text.split("$$SOE")[1].split("$$EOE")[0].strip().splitlines():
        f = [c.strip() for c in line.split(",")]
        rows.append([float(f[0]), float(f[2]), float(f[3]), float(f[4])])
    return np.array(rows)


def elements(command, start, stop, step):
    """Osculating elements across the window, as (N, 7) rows.

    Columns are (JD, a, e, incl, node, peri, M0-at-J2000) — the same seven the
    fit works in, so the row nearest an arc's midpoint drops straight in as its
    starting point. That matters more than it sounds: an arc covering a fraction
    of a revolution is fitted almost as well by a whole family of ellipses, and
    without a seed that is already the physical one the fit settles on whichever
    member of the family it happened to start nearest. The positions come out
    fine either way; `a`, `e` and the perihelion do not, and those are what the
    info card quotes and the orbit ring is drawn from.
    """
    text = horizons({"COMMAND": f"'{command}'", "EPHEM_TYPE": "'ELEMENTS'",
                     "OUT_UNITS": "'AU-D'", "START_TIME": f"'{start}'",
                     "STOP_TIME": f"'{stop}'", "STEP_SIZE": f"'{step}'"})
    if text is None:
        return None
    rows = []
    for line in text.split("$$SOE")[1].split("$$EOE")[0].strip().splitlines():
        f = [c.strip() for c in line.split(",")]
        # Horizons CSV element order: JDTDB, date, EC, QR, IN, OM, W, Tp, N, MA, TA, A, AD, PR
        jd, e, incl, node, peri = float(f[0]), float(f[2]), float(f[4]), float(f[5]), float(f[6])
        n, ma, a = float(f[8]), float(f[9]), float(f[11])
        rows.append([jd, a, e, incl, node, peri, (ma - n * (jd - J2000)) % 360.0])
    return np.array(rows)


def mean_motion(a, dn):
    """Kepler's third law, plus the small correction the fit is allowed."""
    return K_DEG / a ** 1.5 * (1.0 + dn)


def kepler_xyz(p, jd):
    """The model `smallbody.rs` implements: a Keplerian ellipse at mean motion n.

    `p` is (a, e, incl, node, peri, m0, dn) — the last being the fractional
    correction to Kepler's own mean motion. Returns AU in the J2000 ecliptic.
    """
    a, e, incl, node, peri, m0, dn = p
    m = np.radians((m0 + mean_motion(a, dn) * (jd - J2000)) % 360.0)
    ea = m + e * np.sin(m)
    for _ in range(24):
        d = (m - (ea - e * np.sin(ea))) / (1.0 - e * np.cos(ea))
        ea = ea + d
    px = a * (np.cos(ea) - e)
    py = a * math.sqrt(max(1.0 - e * e, 0.0)) * np.sin(ea)
    cw, sw = math.cos(math.radians(peri)), math.sin(math.radians(peri))
    co, so = math.cos(math.radians(node)), math.sin(math.radians(node))
    ci, si = math.cos(math.radians(incl)), math.sin(math.radians(incl))
    return np.stack([
        (cw * co - sw * so * ci) * px + (-sw * co - cw * so * ci) * py,
        (cw * so + sw * co * ci) * px + (-sw * so + cw * co * ci) * py,
        (sw * si) * px + (cw * si) * py,
    ], axis=-1)


def fit(seed, jd, pos_au):
    """Least-squares the seven parameters against Horizons, scale-free."""
    scale = np.linalg.norm(pos_au, axis=1)[:, None]

    def resid(p):
        return ((kepler_xyz(p, jd) - pos_au) / scale).ravel()

    lo = [1e-4, 0.0, -180.0, -1080.0, -1080.0, -1080.0, -MAX_N_DRIFT]
    hi = [1e4, 0.999, 180.0, 1080.0, 1080.0, 1080.0, MAX_N_DRIFT]
    p = np.clip(np.array(seed, dtype=float), lo, hi)
    out = least_squares(resid, p, bounds=(lo, hi), xtol=1e-14, ftol=1e-14, max_nfev=8000)
    return out.x


def errors(model, pos_au):
    cos = (model * pos_au).sum(axis=1) / (
        np.linalg.norm(model, axis=1) * np.linalg.norm(pos_au, axis=1))
    ang = np.degrees(np.arccos(np.clip(cos, -1, 1)))
    radial = np.abs(np.linalg.norm(model, axis=1) - np.linalg.norm(pos_au, axis=1)) \
        / np.linalg.norm(pos_au, axis=1)
    return ang.max(), radial.max()


def smoothstep(t):
    t = np.clip(t, 0.0, 1.0)
    return t * t * (3.0 - 2.0 * t)


def evaluate(chain, blend_d, jd):
    """The chain, exactly as `smallbody.rs` evaluates it — cross-fade included.

    `chain` is [(start_jd, end_jd, params)] in order, covering the window
    without gaps. Inside `blend_d` of a boundary the two pieces are mixed, which
    is what stops a body from jumping as the clock crosses one.
    """
    jd = np.atleast_1d(jd)
    out = np.zeros((len(jd), 3))
    for i, (s, e, p) in enumerate(chain):
        # This piece owns [s, e), and reaches `blend_d` into its neighbours.
        lo = s - blend_d if i > 0 else -math.inf
        hi = e + blend_d if i < len(chain) - 1 else math.inf
        sel = (jd >= lo) & (jd < hi)
        if not sel.any():
            continue
        w = np.ones(sel.sum())
        if i > 0:
            w *= smoothstep((jd[sel] - (s - blend_d)) / (2.0 * blend_d))
        if i < len(chain) - 1:
            w *= 1.0 - smoothstep((jd[sel] - (e - blend_d)) / (2.0 * blend_d))
        out[sel] += kepler_xyz(p, jd[sel]) * w[:, None]
    return out


def segment(osc, jd, pos_au, blend_d):
    """Subdivide the window until a chain of ellipses holds it to `TOL_DEG`."""
    def residuals(lo, hi, p):
        sel = (jd >= lo) & (jd <= hi)
        if not sel.any():
            return sel, np.zeros(0)
        model = kepler_xyz(p, jd[sel])
        cos = (model * pos_au[sel]).sum(axis=1) / (
            np.linalg.norm(model, axis=1) * np.linalg.norm(pos_au[sel], axis=1))
        return sel, np.degrees(np.arccos(np.clip(cos, -1, 1)))

    def seed_at(mid):
        """Horizons' own osculating elements nearest `mid`, as a fit seed."""
        row = osc[int(np.argmin(np.abs(osc[:, 0] - mid)))]
        return [row[1], row[2], row[3], row[4], row[5], row[6], 0.0]

    def fit_span(lo, hi):
        # Fitted over the span widened by the blend, so a piece is never asked
        # to extrapolate into the region where it is still contributing.
        sel = (jd >= lo - blend_d) & (jd <= hi + blend_d)
        if sel.sum() < 16:
            sel = (jd >= lo - 8 * blend_d) & (jd <= hi + 8 * blend_d)
        return fit(seed_at(0.5 * (lo + hi)), jd[sel], pos_au[sel])

    lo0, hi0 = jd[0], jd[-1]
    chain = [(lo0, hi0, fit_span(lo0, hi0))]
    while len(chain) < MAX_SEGMENTS:
        # Split whichever piece is worst, so the effort lands where it is needed
        # — a comet is well behaved for decades and hopeless for one year of it.
        scored = []
        for s, e, p in chain:
            sel, ang = residuals(s, e, p)
            splittable = e - s > 2 * MIN_SEGMENT_D and len(ang) > 0
            scored.append((ang.max() if splittable else 0.0, sel, ang))
        k = int(np.argmax([x[0] for x in scored]))
        if scored[k][0] <= TOL_DEG:
            break
        s, e, p = chain[k]
        # Cut where the error actually lives — at the median of its mass, so a
        # localised event (an encounter that steps the orbit at one instant)
        # gets a boundary put on it instead of being bisected towards over and
        # over. Clamped to the middle of the piece, because error that simply
        # grows with time peaks at an end, and cutting there would shave a
        # sliver off and leave the problem untouched.
        _, sel, ang = scored[k]
        mass = np.cumsum(ang ** 2)
        cut = float(jd[sel][int(np.searchsorted(mass, mass[-1] * 0.5))])
        cut = min(max(cut, s + MIN_SEGMENT_D), e - MIN_SEGMENT_D)
        chain[k:k + 1] = [(s, cut, fit_span(s, cut)), (cut, e, fit_span(cut, e))]
    return chain


def jd_of(date):
    y, m, d = (int(x) for x in date.split("-"))
    a = (14 - m) // 12
    y2, m2 = y + 4800 - a, m + 12 * a - 3
    jdn = d + (153 * m2 + 2) // 5 + 365 * y2 + y2 // 4 - y2 // 100 + y2 // 400 - 32045
    return jdn - 0.5


def perihelion_dates(chain):
    """Every perihelion passage inside the window, from the piece that owns it."""
    out = []
    for s, e, p in chain:
        n, m0 = mean_motion(p[0], p[6]), p[5]
        # M = 0 at perihelion, so solve m0 + n(t − J2000) = 360k for integer k.
        k0 = math.floor((m0 + n * (s - J2000)) / 360.0)
        for k in range(k0, k0 + int((e - s) * n / 360.0) + 3):
            t = J2000 + (360.0 * k - m0) / n
            if s <= t < e:
                out.append(t)
    return sorted(out)


def ymd(jd):
    jdn = int(math.floor(jd + 0.5))
    a = jdn + 32044
    b = (4 * a + 3) // 146097
    c = a - 146097 * b // 4
    d2 = (4 * c + 3) // 1461
    e = c - 1461 * d2 // 4
    m = (5 * e + 2) // 153
    day = e - (153 * m + 2) // 5 + 1
    month = m + 3 - 12 * (m // 10)
    year = 100 * b + d2 - 4800 + m // 10
    return f"{year:04d}-{month:02d}-{day:02d}"


def close_approaches():
    """Near-Earth objects that pass closest inside the window, from the CAD API."""
    q = urllib.parse.urlencode({
        "date-min": WIN_START, "date-max": WIN_END, "dist-max": CAD_DIST_AU,
        "h-max": CAD_H_MAX, "sort": "dist", "fullname": "true",
    })
    d = http_json(CAD + "?" + q)
    seen, out = set(), []
    for row in d.get("data", []):
        z = dict(zip(d["fields"], row))
        des = z["des"]
        if des in seen:
            continue
        seen.add(des)
        km = float(z["dist"]) * AU_KM
        # Diameter from H, at the 0.14 albedo the CAD table's own H assumes.
        h = float(z["h"])
        radius_km = 0.5 * 1329.0 / math.sqrt(0.14) * 10.0 ** (-0.2 * h)
        out.append(dict(
            des=des, fullname=z["fullname"].strip(), cd=z["cd"], km=km,
            radius_km=radius_km,
        ))
        if len(out) >= CAD_LIMIT:
            break
    return out


def cad_caption(a):
    when = a["cd"].split()[0]              # "2029-Apr-13"
    y, mon, d = when.split("-")
    dist = f"{a['km']:,.0f} km".replace(",", " ")
    return f"Passes {dist} from the Earth on {d} {mon} {y}"


def short_name(fullname, des):
    """What to call it: `99942 Apophis (2004 MN4)` -> `Apophis`.

    An asteroid that has not earned a name is known by its provisional
    designation and nothing else, so `308635 (2005 YU55)` -> `2005 YU55`. The
    catalogue number is the one thing nobody says out loud.
    """
    core = fullname.split("(")[0].strip()
    parts = core.split(None, 1)
    if len(parts) == 2 and parts[0].isdigit():
        return parts[1]
    return des


def body_list():
    bodies = [dict(name=n, designation=d, command=c, cls=k, radius_km=r, why=w)
              for n, d, c, k, r, w in CURATED]
    have = {b["designation"].split()[0] for b in bodies} | {b["name"] for b in bodies}
    for a in close_approaches():
        name = short_name(a["fullname"], a["des"])
        if a["des"] in have or name in have:
            # Already curated (Apophis, Bennu): keep the curated caption but say
            # what the close approach is, since that is why it is in the window.
            for b in bodies:
                if b["name"] == name or b["designation"].split()[0] == a["des"]:
                    b["why"] = f"{b['why']}. {cad_caption(a)}"
            continue
        bodies.append(dict(
            name=name,
            # The catalogue's own full name, parenthesised provisional
            # designation and all: it is what a search for either half has to
            # match, and for an unnamed body it is all there is to go on.
            designation=a["fullname"],
            command=f"{a['des']};",
            cls="Asteroid",
            radius_km=a["radius_km"],
            # Its size is not measured, only inferred from how bright it is —
            # so say so, rather than let a rendered dot imply otherwise.
            why=f"{cad_caption(a)}. Roughly {2000 * a['radius_km']:.0f} m across, "
                "estimated from its brightness",
        ))
    return bodies


def process(b):
    # Two dozen samples per revolution, but never coarser than ten days: a
    # comet with a 0.12 AU perihelion turns through most of its orbit in a
    # fortnight, and a step scaled only to its period would step straight over
    # the one part of the path that is hard to fit.
    one = elements(b["command"], WIN_START, "2026-01-11", "10 d")
    if one is None:
        return None
    period = 365.25 * one[0][1] ** 1.5
    step = max(int(min(max(period / 24.0, 2.0), 10.0)), 1)

    rows = vectors(b["command"], WIN_START, WIN_END, f"{step} d")
    osc = elements(b["command"], WIN_START, WIN_END, f"{step} d")
    if rows is None or osc is None:
        return None
    jd, pos = rows[:, 0], rows[:, 1:] / AU_KM

    chain = segment(osc, jd, pos, BLEND_D)
    ang, radial = errors(evaluate(chain, BLEND_D, jd), pos)

    # How hard the fit had to lean on the mean-motion slack. At the bound it
    # means the arc wanted something Kepler would not give it, which is worth
    # seeing rather than assuming away.
    drift = max(abs(p[6]) for _, _, p in chain)

    b.update(chain=chain, ang=ang, radial=radial, drift=drift,
             peri=perihelion_dates(chain), jd=jd, pos=pos)
    return b


def rust_rows(b):
    a, e = b["chain"][0][2][0], b["chain"][0][2][1]
    head = (
        f'    small("{b["name"]}", "{b["designation"]}", Class::{b["cls"]}, '
        f'{b["radius_km"] / 1.0e6:.9f}, {b["ang"]:.3f}, &[\n'
    )
    rows = "".join(
        f"        arc({s:.1f}, {p[0]:.9f}, {p[1]:.9f}, {p[2]:.6f}, "
        f"{p[3]:.6f}, {p[4]:.6f}, {p[5] % 360.0:.6f}, {mean_motion(p[0], p[6]):.9f}),\n"
        for s, _, p in b["chain"]
    )
    peri = ""
    if b["peri"]:
        peri = "  perihelion " + ", ".join(ymd(t) for t in b["peri"][:4])
        if len(b["peri"]) > 4:
            peri += ", …"
    tail = (
        f'    ]),  // q {a * (1 - e):.3f} AU, {len(b["chain"])} arc(s), '
        f'radial {b["radial"] * 100:.3f}%{peri}\n'
    )
    return head + rows + tail


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    out = []
    for b in body_list():
        print(f"fitting {b['name']}", file=sys.stderr)
        r = process(b)
        if r is None:
            print(f"  !! skipped {b['name']}", file=sys.stderr)
            continue
        if r["cls"] == "Comet" and not r["peri"]:
            print(f"  !! {b['name']} has no perihelion in the window — dropped",
                  file=sys.stderr)
            continue
        print(f"  {len(r['chain'])} arcs, {r['ang']:.3f}° / {r['radial'] * 100:.3f}% radial, "
              f"n drift {r['drift']:.1%}", file=sys.stderr)
        out.append(r)

    print("// name, designation, class, radius (Gm), fit error; then one arc per line:")
    print("// start JD, a, e, incl, node, arg. perihelion, M0, mean motion")
    for b in out:
        print(rust_rows(b), end="")
    print("\n// why, in table order")
    for b in out:
        print(f'    // {b["name"]}: {b["why"]}')
    print(f"\n// {sum(len(b['chain']) for b in out)} arcs over {len(out)} bodies")

    fx = {"note": "JPL Horizons geometric vectors, J2000 ecliptic, AU",
          "window": [WIN_START, WIN_END], "bodies": {}}
    for b in out:
        # Every seventh sample, so the fixture is a spread of dates across the
        # window rather than the whole fit input — and one that does not land on
        # the same orbital phase every time.
        keep = list(range(0, len(b["jd"]), 7))
        fx["bodies"][b["name"]] = {
            "fit_error_deg": b["ang"],
            "samples": [[float(b["jd"][i])] + [float(x) for x in b["pos"][i]] for i in keep],
        }
    path = os.path.join(here, "..", "tests", "fixtures", "smallbodies.json")
    with open(path, "w") as fh:
        json.dump(fx, fh, indent=0)
    print(f"wrote {os.path.relpath(path)}", file=sys.stderr)


if __name__ == "__main__":
    main()
