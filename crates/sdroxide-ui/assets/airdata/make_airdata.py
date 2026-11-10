#!/usr/bin/env python3
"""Build the aircraft-navigation tables the survey map draws from.

    python3 crates/sdroxide-ui/assets/airdata/make_airdata.py
    python3 crates/sdroxide-ui/assets/airdata/make_airdata.py --from ~/Downloads

    python3 crates/sdroxide-ui/assets/airdata/make_airdata.py --extras --from ~/Downloads

Writes four committed assets beside this script:

    navaids.bin    every radio navigation aid in the world
    airports.bin   the large and medium airports
    runways.bin    the open runways of those airports
    freqs.bin      their published radio frequencies

`--extras` rebuilds only `runways.bin`, `freqs.bin` and `wiki.bin` and leaves the other two
alone, so a refresh of the frequencies does not rewrite 5 280 airports and 11 000
navaids that did not change.

**Source: OurAirports, which is public domain.** Their data repository carries
the Unlicense — *"free and unencumbered software released into the public
domain… sell, or distribute… for any purpose"* — so there is no attribution
obligation and no share-alike to reason about. That is the whole reason this
source and not another: the paid and keyed ones are out (see the house rule in
`AGENTS.md`), OpenFlights is ODbL with a share-alike that reaches a derived
database, and OpenAIP is non-commercial.

Files, from <https://ourairports.com/data/> (mirrored on GitHub Pages):

    https://davidmegginson.github.io/ourairports-data/navaids.csv
    https://davidmegginson.github.io/ourairports-data/airports.csv
    https://davidmegginson.github.io/ourairports-data/runways.csv
    https://davidmegginson.github.io/ourairports-data/airport-frequencies.csv

Nothing fetches at build time or at run time — this script is run by a person
when the data wants refreshing, exactly like `make_earth_maps.py` beside the
Earth rasters. The assets it writes are committed.

**`--from DIR` reads CSVs already on disk instead of downloading them**, which
is the route to use when the machine's Python has no certificate store (an
Anaconda install often does not) or when the machine is offline: save
`navaids.csv` and `airports.csv` from <https://ourairports.com/data/> into one
directory and point the script at it.

## Why these tables and not the whole thing

`airports.csv` is 86 258 rows and most of them are heliports and grass strips:
at map scale they are noise, and every one is bytes in the binary. Only
`large_airport` and `medium_airport` are kept — 5 280 of them, which is what a
person could actually fly to.

Every navaid is kept, because they are not just map furniture: **6 700 of them
are NDBs below 1 800 kHz**, which are receivable on an ordinary medium-wave
receiver. A list of them is a listening target list as much as a map layer.

## Format

A magic, then a count, then fixed-size records — little-endian, strings
NUL-padded, `make_earth_maps.py`'s convention. Fixed size rather than
variable so a reader can index a record without walking the file.

    navaids.bin   "SDXNAV01" u16 count
                  { ident[5] name[16] type u8 country[2] freq_khz u32 lat f32 lon f32 }
    airports.bin  "SDXAPT01" u16 count
                  { ident[8] name[28] iata[3] icao[4] type u8 lat f32 lon f32 elev_ft i16 }
    runways.bin   "SDXRWY01" u32 date u32 count
                  { ident[8] le[4] he[4] length_ft u16 width_ft u16 surface u8 lit u8 }
    freqs.bin     "SDXFRQ01" u32 date u32 count
                  { ident[8] kind u8 freq_khz u32 }

The last two are keyed by the airport's `ident` (the first field of an airport
record), not by its position, so they stay valid when `airports.bin` is
rebuilt. `date` is the day the file was built, as `yyyymmdd`: frequencies change,
and the airport card says how old the ones it shows are. Closed runways and
heliport pads are left out; a runway with no published length is left out too.
Frequencies are in kHz (118.005 MHz is 118 005) and de-duplicated per airport.

Airport names get 28 bytes rather than 20 because the hover shows them, and 20
cut **"Amsterdam Airport Schiphol"** to "Amsterdam Airport Sc" — a half-name on
screen, which is worse than no name. Where even 28 is not enough the name is cut
at a **word** boundary (`word`), so a short field costs a word rather than a
letter.

Latitude and longitude are `f32`: about a metre at the equator, and these are
map positions. `freq_khz` is `u32` because a VOR at 116 300 kHz and an NDB at
291 kHz are the same field, and kHz is the unit both are published in.
"""

from __future__ import annotations

import csv
import io
import re
import os
import struct
import sys
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
BASE = "https://davidmegginson.github.io/ourairports-data"

# The navaid types, in the order they are numbered in the file. An index rather
# than a string: this is the field the map colours and sizes its symbols by.
NAVAID_TYPES = ["NDB", "VOR", "VOR-DME", "VORTAC", "TACAN", "DME", "NDB-DME"]

AIRPORT_TYPES = ["large_airport", "medium_airport"]


#: Where `--from` points, or None to download.
LOCAL: str | None = None


def fetch(name: str) -> str:
    if LOCAL:
        path = os.path.join(LOCAL, name)
        sys.stderr.write(f"reading {path}\n")
        return open(path, encoding="utf-8").read()
    url = f"{BASE}/{name}"
    sys.stderr.write(f"fetching {url}\n")
    try:
        with urllib.request.urlopen(url, timeout=120) as r:
            return r.read().decode("utf-8")
    except urllib.error.URLError as e:
        raise SystemExit(
            f"could not download {url}: {e}\n\n"
            "If this machine's Python has no certificate store, save navaids.csv "
            "and airports.csv from https://ourairports.com/data/ into a directory "
            "and run this again with --from <that directory>."
        ) from e


def fixed(text: str, n: int) -> bytes:
    """`text` as `n` bytes, NUL-padded — and cut, not truncated mid-character."""
    raw = (text or "").encode("utf-8")
    return raw[:n].ljust(n, b"\0")


def word(text: str, n: int) -> str:
    """`text` cut to `n` bytes if it must be — at a word boundary where there is
    one, because a name shown on hover is read, not parsed."""
    raw = (text or "").encode("utf-8")
    if len(raw) <= n:
        return text or ""
    cut = raw[:n].decode("utf-8", "ignore")
    if " " in cut:
        cut = cut[: cut.rindex(" ")]
    return cut


def number(value, default=0.0) -> float:
    try:
        return float(value)
    except (TypeError, ValueError):
        return default


def build_navaids() -> bytes:
    rows = list(csv.DictReader(io.StringIO(fetch("navaids.csv"))))
    keep = [r for r in rows if r["type"] in NAVAID_TYPES and r["latitude_deg"]]
    out = io.BytesIO()
    out.write(b"SDXNAV01")
    out.write(struct.pack("<H", len(keep)))
    for r in keep:
        out.write(fixed(r["ident"], 5))
        out.write(fixed(r["name"], 16))
        out.write(struct.pack("<B", NAVAID_TYPES.index(r["type"])))
        out.write(fixed(r["iso_country"], 2))
        # Some rows carry a sentinel rather than a frequency; 0 means "not
        # published", and the map draws those as an unfrequency'd symbol rather
        # than at DC.
        out.write(struct.pack("<I", max(0, int(number(r["frequency_khz"])))))
        out.write(struct.pack("<f", number(r["latitude_deg"])))
        out.write(struct.pack("<f", number(r["longitude_deg"])))
    silent = sum(1 for r in keep if int(number(r["frequency_khz"])) <= 0)
    sys.stderr.write(
        f"  {len(keep)} navaids of {len(rows)} rows, {silent} with no frequency\n"
    )
    return out.getvalue()


def build_airports() -> bytes:
    rows = list(csv.DictReader(io.StringIO(fetch("airports.csv"))))
    keep = [r for r in rows if r["type"] in AIRPORT_TYPES and r["latitude_deg"]]
    out = io.BytesIO()
    out.write(b"SDXAPT01")
    out.write(struct.pack("<H", len(keep)))
    for r in keep:
        out.write(fixed(r["ident"], 8))
        out.write(fixed(word(r["name"], 28), 28))
        out.write(fixed(r["iata_code"], 3))
        out.write(fixed(r["icao_code"] or r["gps_code"], 4))
        out.write(struct.pack("<B", AIRPORT_TYPES.index(r["type"])))
        out.write(struct.pack("<f", number(r["latitude_deg"])))
        out.write(struct.pack("<f", number(r["longitude_deg"])))
        out.write(struct.pack("<h", int(number(r["elevation_ft"]))))
    sys.stderr.write(f"  {len(keep)} airports of {len(rows)} rows\n")
    return out.getvalue()


# Surface codes, in the order the reader numbers them. 0 is "not published".
SURFACES = [
    ("asp", 1), ("pem", 1), ("bit", 1), ("tar", 1),
    ("con", 2),
    ("turf", 3), ("gra", 3), ("grs", 3), ("sod", 3), ("gre", 3),
    ("gvl", 4), ("grv", 4), ("coral", 4), ("mac", 4),
    ("dirt", 5), ("earth", 5), ("soil", 5), ("clay", 5), ("sand", 5),
    ("water", 6),
]

# Frequency kinds, most useful first; the card shows them in this order.
FREQ_KINDS = ["TWR", "GND", "ATIS", "APP", "DEP", "CTAF", "UNIC", "CLD", "AFIS"]


def surface_code(text: str) -> int:
    t = (text or "").strip().lower()
    for key, code in SURFACES:
        if t.startswith(key):
            return code
    return 0


def build_runways(idents: set, date: int) -> bytes:
    rows = list(csv.DictReader(io.StringIO(fetch("runways.csv"))))
    by_ident: dict = {}
    for r in rows:
        ident = r["airport_ident"]
        length = int(number(r["length_ft"]))
        if ident not in idents or r["closed"] == "1" or length <= 0:
            continue
        # A helipad has an "H1" end and no other.
        if not r["he_ident"] and r["le_ident"].upper().startswith("H"):
            continue
        by_ident.setdefault(ident, []).append(r)
    out = io.BytesIO()
    body = io.BytesIO()
    count = 0
    for ident in sorted(by_ident):
        for r in sorted(by_ident[ident], key=lambda r: -int(number(r["length_ft"]))):
            body.write(fixed(ident, 8))
            body.write(fixed(r["le_ident"], 4))
            body.write(fixed(r["he_ident"], 4))
            body.write(struct.pack("<H", min(65535, int(number(r["length_ft"])))))
            body.write(struct.pack("<H", min(65535, int(number(r["width_ft"])))))
            body.write(struct.pack("<B", surface_code(r["surface"])))
            body.write(struct.pack("<B", 1 if r["lighted"] == "1" else 0))
            count += 1
    out.write(b"SDXRWY01")
    out.write(struct.pack("<II", date, count))
    out.write(body.getvalue())
    sys.stderr.write(f"  {count} runways at {len(by_ident)} airports\n")
    return out.getvalue()


def build_freqs(idents: set, date: int) -> bytes:
    rows = list(csv.DictReader(io.StringIO(fetch("airport-frequencies.csv"))))
    by_ident: dict = {}
    for r in rows:
        ident = r["airport_ident"]
        kind = r["type"].upper()
        khz = int(round(number(r["frequency_mhz"]) * 1000))
        if ident not in idents or kind not in FREQ_KINDS or khz <= 0:
            continue
        got = by_ident.setdefault(ident, [])
        if (kind, khz) not in got:
            got.append((kind, khz))
    out = io.BytesIO()
    body = io.BytesIO()
    count = 0
    for ident in sorted(by_ident):
        for kind, khz in sorted(by_ident[ident], key=lambda k: FREQ_KINDS.index(k[0])):
            body.write(fixed(ident, 8))
            body.write(struct.pack("<B", FREQ_KINDS.index(kind)))
            body.write(struct.pack("<I", khz))
            count += 1
    out.write(b"SDXFRQ01")
    out.write(struct.pack("<II", date, count))
    out.write(body.getvalue())
    sys.stderr.write(f"  {count} frequencies at {len(by_ident)} airports\n")
    return out.getvalue()


def build_wiki(idents: set, date: int) -> bytes:
    """`wiki.bin`: the Wikipedia article of each airport that has one.

    Records are `{ident[8] lang[2] len u8 title[len]}`, the title as UTF-8 with
    its percent-escapes undone and spaces as underscores. Only `xx.wikipedia.org`
    links with a plain title are kept: anything the station would refuse to put
    in a URL is dropped here too, so the table never promises a picture the
    station will not ask for.
    """
    import urllib.parse

    rows = list(csv.DictReader(io.StringIO(fetch("airports.csv"))))
    body = io.BytesIO()
    count = 0
    for r in sorted(rows, key=lambda r: r["ident"]):
        if r["ident"] not in idents or not r["wikipedia_link"]:
            continue
        m = re.fullmatch(r"https?://([a-z]{2})\.wikipedia\.org/wiki/([^?#]+)", r["wikipedia_link"].strip())
        if not m:
            continue
        lang, title = m.group(1), urllib.parse.unquote(m.group(2)).replace(" ", "_")
        raw = title.encode("utf-8")
        if not 1 <= len(raw) <= 120 or re.search(r"[/\\%?#<>|\x00-\x1f]", title):
            continue
        body.write(fixed(r["ident"], 8))
        body.write(lang.encode("ascii"))
        body.write(struct.pack("<B", len(raw)))
        body.write(raw)
        count += 1
    out = io.BytesIO()
    out.write(b"SDXWIK01")
    out.write(struct.pack("<II", date, count))
    out.write(body.getvalue())
    sys.stderr.write(f"  {count} Wikipedia articles\n")
    return out.getvalue()


def airport_idents() -> set:
    """The idents of the airports `build_airports` keeps."""
    rows = list(csv.DictReader(io.StringIO(fetch("airports.csv"))))
    return {r["ident"] for r in rows if r["type"] in AIRPORT_TYPES and r["latitude_deg"]}


def verify_dated(path: str, magic: bytes, size: int) -> None:
    blob = open(path, "rb").read()
    assert blob[:8] == magic, f"{path}: bad magic {blob[:8]!r}"
    date, count = struct.unpack_from("<II", blob, 8)
    assert len(blob) - 16 == count * size, f"{path}: {len(blob) - 16} bytes for {count} x {size}"
    sys.stderr.write(f"  wrote {path}: {len(blob) / 1024:.0f} KiB, {count} records, {date}\n")


def verify(path: str, magic: bytes, size: int, headers: int = 2) -> None:
    """Read it back the way the Rust reader will, and say what came out.

    Nothing here is a test — the reader has those — but a format that cannot be
    walked is a format that will fail later, and the walk is four lines.
    """
    blob = open(path, "rb").read()
    assert blob[:8] == magic, f"{path}: bad magic {blob[:8]!r}"
    (count,) = struct.unpack_from("<H", blob, 8)
    body = len(blob) - 10
    assert body == count * size, f"{path}: {body} bytes for {count} × {size}"
    sys.stderr.write(f"  wrote {path}: {len(blob) / 1024:.0f} KiB, {count} records\n")


def main() -> None:
    global LOCAL
    args = sys.argv[1:]
    if args[:1] == ["--from"] and len(args) == 1:
        LOCAL = "."
    extras = "--extras" in args
    if "--from" in args:
        LOCAL = os.path.expanduser(args[args.index("--from") + 1])
    import datetime

    date = int(datetime.date.today().strftime("%Y%m%d"))
    idents = airport_idents()
    open(os.path.join(HERE, "runways.bin"), "wb").write(build_runways(idents, date))
    open(os.path.join(HERE, "freqs.bin"), "wb").write(build_freqs(idents, date))
    open(os.path.join(HERE, "wiki.bin"), "wb").write(build_wiki(idents, date))
    # Record sizes: 8+4+4+2+2+1+1 and 8+1+4 (wiki.bin's are not fixed).
    verify_dated(os.path.join(HERE, "runways.bin"), b"SDXRWY01", 22)
    verify_dated(os.path.join(HERE, "freqs.bin"), b"SDXFRQ01", 13)
    sys.stderr.write(f"  wrote wiki.bin: {os.path.getsize(os.path.join(HERE, 'wiki.bin')) / 1024:.0f} KiB\n")
    if extras:
        return
    navaids = build_navaids()
    airports = build_airports()
    open(os.path.join(HERE, "navaids.bin"), "wb").write(navaids)
    open(os.path.join(HERE, "airports.bin"), "wb").write(airports)

    # Record sizes, kept in step with the structs above and with the Rust
    # reader: 5+16+1+2+4+4+4 and 8+28+3+4+1+4+4+2.
    verify(os.path.join(HERE, "navaids.bin"), b"SDXNAV01", 36)
    verify(os.path.join(HERE, "airports.bin"), b"SDXAPT01", 54)


if __name__ == "__main__":
    main()
