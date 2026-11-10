#!/usr/bin/env python3
"""Fit the moon table in `src/planets.rs`, and refresh the ephemeris fixture.

Two jobs, both against JPL Horizons (public domain):

1. **Fit** each major moon's circular orbit — semi-major axis, sidereal period,
   and the orientation and phase of its orbit plane in the J2000 ecliptic
   frame. `planets::MOONS` is a transcription of what this prints, so a moon is
   never placed by a number somebody remembered.

2. **Sample** the planets and moons at dates spanning the era the view is used
   in, and write `tests/fixtures/horizons.json`. `planets.rs`'s tests replay
   those samples, which is what keeps the Keplerian element table honest: a
   mistyped digit moves a planet by degrees and the fixture catches it.

    python3 fit_bodies.py            # refit, rewrite the fixture, print the table

Network: one Horizons request per body per window; a full run is a few dozen.
"""

import json
import math
import os
import sys
import time
import urllib.parse
import urllib.request

import numpy as np

API = "https://ssd.jpl.nasa.gov/api/horizons.api"
J2000 = 2451545.0

# (name, Horizons id, centre, nominal period in days) for every moon the view
# draws. Periods are only used to size the fit windows; the fit measures them.
MOONS = [
    ("Phobos", "401", "500@499", 0.31891),
    ("Deimos", "402", "500@499", 1.26244),
    ("Io", "501", "500@599", 1.769138),
    ("Europa", "502", "500@599", 3.551181),
    ("Ganymede", "503", "500@599", 7.154553),
    ("Callisto", "504", "500@599", 16.689018),
    ("Enceladus", "602", "500@699", 1.370218),
    ("Tethys", "603", "500@699", 1.887802),
    ("Dione", "604", "500@699", 2.736915),
    ("Rhea", "605", "500@699", 4.517500),
    ("Titan", "606", "500@699", 15.945421),
    ("Iapetus", "608", "500@699", 79.3215),
    ("Miranda", "705", "500@799", 1.413479),
    ("Ariel", "701", "500@799", 2.520379),
    ("Umbriel", "702", "500@799", 4.144177),
    ("Titania", "703", "500@799", 8.705872),
    ("Oberon", "704", "500@799", 13.463239),
    ("Triton", "801", "500@899", 5.876854),
]

# The planets, by system barycentre — which is what the JPL Keplerian element
# set is defined for.
PLANETS = [
    ("Mercury", "1"),
    ("Venus", "2"),
    ("Earth", "3"),
    ("Mars", "4"),
    ("Jupiter", "5"),
    ("Saturn", "6"),
    ("Uranus", "7"),
    ("Neptune", "8"),
]

# The two windows the period is measured across. Wide enough that a period
# error of one part in a million shows up, recent enough that the slow
# precession of the outer moons' orbit planes is fitted where it matters.
WINDOW_A = "2010-01-01"
WINDOW_B = "2040-01-01"


def horizons(command, center, start, stop, step):
    """Geometric position vectors, km, in the J2000 ecliptic frame.

    Returns an (N, 4) array of (JD_TDB, x, y, z).
    """
    q = {
        "format": "text",
        "COMMAND": f"'{command}'",
        "OBJ_DATA": "'NO'",
        "MAKE_EPHEM": "'YES'",
        "EPHEM_TYPE": "'VECTORS'",
        "CENTER": f"'{center}'",
        "REF_PLANE": "'ECLIPTIC'",
        "VEC_TABLE": "'1'",
        "OUT_UNITS": "'KM-S'",
        "START_TIME": f"'{start}'",
        "STOP_TIME": f"'{stop}'",
        "STEP_SIZE": f"'{step}'",
        "CSV_FORMAT": "'YES'",
    }
    url = API + "?" + urllib.parse.urlencode(q)
    for attempt in range(4):
        try:
            with urllib.request.urlopen(url, timeout=180) as r:
                text = r.read().decode()
            break
        except Exception as e:  # transient API hiccups are common enough
            print(f"  retry {attempt}: {e}", file=sys.stderr)
            time.sleep(3 * (attempt + 1))
    else:
        raise SystemExit(f"Horizons failed for {command}")

    body = text.split("$$SOE")[1].split("$$EOE")[0]
    rows = []
    for line in body.strip().splitlines():
        f = [c.strip() for c in line.split(",")]
        rows.append([float(f[0]), float(f[2]), float(f[3]), float(f[4])])
    return np.array(rows)


def fit_plane(pos):
    """Orbit-plane normal (unit) from a set of position vectors."""
    # The smallest singular vector of the position matrix is the plane normal.
    _, _, vt = np.linalg.svd(pos)
    n = vt[2]
    # Orient it along the angular momentum, so prograde motion is +θ about it
    # and a retrograde moon (Triton) comes out with an inclination past 90°.
    h = np.cross(pos[:-1], pos[1:]).sum(axis=0)
    if np.dot(n, h) < 0:
        n = -n
    return n / np.linalg.norm(n)


def in_plane_basis(n):
    """Ascending node longitude, and the in-plane axes θ is measured from."""
    node = math.degrees(math.atan2(n[0], -n[1])) % 360.0
    u = np.array([math.cos(math.radians(node)), math.sin(math.radians(node)), 0.0])
    return node, u, np.cross(n, u)


def fit_moon(name, ident, center, period):
    """Fit (a, period, inclination, node, phase at J2000) to Horizons."""
    # Four revolutions per window at 64 samples each: enough to pin the phase
    # far below the residual the circular model leaves anyway.
    # Horizons takes whole units only, so the step is in minutes.
    step = f"{max(round(period * 4 / 64 * 1440), 1)} m"
    win = []
    for start in (WINDOW_A, WINDOW_B):
        stop = date_of(jd_of(start) + period * 4)
        win.append(horizons(ident, center, start, stop, step))
    rows = np.vstack(win)
    jd, pos = rows[:, 0], rows[:, 1:]

    n = fit_plane(pos)
    incl = math.degrees(math.acos(max(-1.0, min(1.0, n[2]))))
    node, u, v = in_plane_basis(n)
    a = np.linalg.norm(pos, axis=1).mean()

    # Rate within each window, then the exact number of revolutions between
    # them — that is what makes the period good to a part in 1e7. Each window's
    # phases are unwrapped locally; only the whole-turn count between the two
    # is ambiguous, and the local rate resolves it with enormous margin.
    def line(rows_i):
        j, p = rows_i[:, 0], rows_i[:, 1:]
        th = np.unwrap(np.arctan2(p @ v, p @ u)) * 180.0 / math.pi
        k, c = np.polyfit(j - J2000, th, 1)
        return k, k * (j[0] - J2000) + c, j[0]

    _, th_a, jd_a = line(win[0])
    _, th_b, jd_b = line(win[1])
    dt = jd_b - jd_a
    delta = (th_b - th_a) % 360.0

    # How many whole turns fit in the baseline is the one ambiguous quantity,
    # and it cannot be resolved from the data: every candidate reproduces both
    # windows exactly by construction. So it comes from the published period,
    # which is good to about a part in a million — Phobos, the fastest moon
    # here, makes 34 000 revolutions across the baseline and even that leaves
    # the count unambiguous by three orders of magnitude.
    turns = round((360.0 / period * dt - delta) / 360.0)
    rate = (delta + 360.0 * turns) / dt
    p_fit = 360.0 / rate
    # The nominal period only has to land within a quarter turn over the whole
    # baseline for the count to be unambiguous; assert exactly that, so a bad
    # nominal is caught and a merely-improved period is not.
    assert abs(p_fit / period - 1.0) * turns < 0.25, (
        f"{name}: fitted period {p_fit} is too far from the nominal {period} — "
        "the turn count is not resolvable"
    )
    l0 = (th_a - rate * (jd_a - J2000)) % 360.0

    # Residual of the fitted circle against the samples: the honest accuracy.
    model = np.array([moon_pos(a, p_fit, incl, node, l0, j) for j in jd])
    err = np.linalg.norm(model - pos, axis=1)
    ang = np.degrees(
        np.arccos(np.clip((model * pos).sum(axis=1) / (np.linalg.norm(model, axis=1) * np.linalg.norm(pos, axis=1)), -1, 1))
    )
    return dict(
        name=name,
        a_km=a,
        period_d=p_fit,
        incl=incl,
        node=node,
        l0=l0,
        max_km=err.max(),
        max_deg=ang.max(),
    )


def moon_pos(a, period, incl, node, l0, jd):
    """The model `planets.rs` implements, for checking the fit."""
    i, o = math.radians(incl), math.radians(node)
    n = np.array([math.sin(i) * math.sin(o), -math.sin(i) * math.cos(o), math.cos(i)])
    u = np.array([math.cos(o), math.sin(o), 0.0])
    v = np.cross(n, u)
    th = math.radians(l0 + 360.0 * (jd - J2000) / period)
    return a * (math.cos(th) * u + math.sin(th) * v)


def jd_of(date):
    y, m, d = (int(x) for x in date.split("-"))
    # Fliegel–Van Flandern, at 00:00 UT.
    a = (14 - m) // 12
    y2 = y + 4800 - a
    m2 = m + 12 * a - 3
    jdn = d + (153 * m2 + 2) // 5 + 365 * y2 + y2 // 4 - y2 // 100 + y2 // 400 - 32045
    return jdn - 0.5


def date_of(jd):
    """Inverse of `jd_of`, rounded up to the next whole day."""
    jdn = int(math.ceil(jd + 0.5))
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


def fixture():
    """Reference positions for the Rust tests, over the era the view is used."""
    out = {"note": "JPL Horizons geometric vectors, J2000 ecliptic, gigametres",
           "planets": {}, "moons": {}}
    for name, ident in PLANETS:
        rows = horizons(ident, "500@10", "2015-01-01", "2045-01-01", "700 d")
        out["planets"][name] = [[r[0]] + list(r[1:] / 1.0e6) for r in rows]
        print(f"  fixture {name}: {len(rows)} samples", file=sys.stderr)
    for name, ident, center, period in MOONS:
        # Irregular in phase relative to any of these orbits, so the samples do
        # not all land on the same point of the orbit.
        rows = horizons(ident, center, "2024-03-07", "2032-03-07", "271 d")
        out["moons"][name] = [[r[0]] + list(r[1:] / 1.0e6) for r in rows]
        print(f"  fixture {name}: {len(rows)} samples", file=sys.stderr)
    return out


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    fits = []
    for name, ident, center, period in MOONS:
        print(f"fitting {name}", file=sys.stderr)
        fits.append(fit_moon(name, ident, center, period))

    print("\n// name, a (Gm), period (d), inclination, node, L0 — see tools/fit_bodies.py")
    for f in fits:
        print(
            '    Moon {{ name: "{name}", a: {a:.6f}, period_d: {p:.7f}, '
            "incl_deg: {i:.4f}, node_deg: {n:.4f}, l0_deg: {l:.4f}, radius: 0.0 }},"
            "  // residual {km:.0f} km, {deg:.2f}°".format(
                name=f["name"],
                a=f["a_km"] / 1.0e6,
                p=f["period_d"],
                i=f["incl"],
                n=f["node"],
                l=f["l0"],
                km=f["max_km"],
                deg=f["max_deg"],
            )
        )

    fx = fixture()
    fx["fit"] = {f["name"]: {"max_km": f["max_km"], "max_deg": f["max_deg"]} for f in fits}
    path = os.path.join(here, "..", "tests", "fixtures", "horizons.json")
    with open(path, "w") as fh:
        json.dump(fx, fh, indent=0)
    print(f"wrote {os.path.relpath(path)}", file=sys.stderr)


if __name__ == "__main__":
    main()
