#!/usr/bin/env python3
"""Convert an SDR++ bandplan JSON file to sdroxide's bandplan.json format.

SDR++ bandplans are a flat list of bands with name/type/start/end (Hz).
sdroxide bandplans are region-based (R1/R2/R3) with amateur Band entries
and sub-segments (Cw/Phone/Digi/All) with optional labels.

Usage:
    python3 sdrpp_to_bandplan.py input.json output.json

The converter maps SDR++ band types to segment kinds. Amateur bands are
identified by frequency overlap with known amateur allocations. All three
regions get the same plan (SDR++ has no region concept).
"""

import json
import sys
import argparse

TYPE_TO_KIND = {
    "amateur": "All",
    "broadcast": "Phone",
    "aviation": "Phone",
    "marine": "Phone",
    "military": "Phone",
    "satellite": "Digi",
    "other": "All",
    "utility": "All",
    "cellular": "All",
    "ism": "All",
}

# Amateur band frequency ranges (Hz) -> sdroxide Band enum
AMATEUR_BANDS = [
    (135700, 137800, "M2200"),  # 2200m (not in our enum, skip)
    (472000, 479000, None),  # 630m (not in our enum)
    (1810000, 2000000, "M160"),
    (3500000, 3800000, "M80"),
    (5351500, 5366500, "M60"),
    (7000000, 7200000, "M40"),
    (10100000, 10150000, "M30"),
    (14000000, 14350000, "M20"),
    (18068000, 18168000, "M17"),
    (21000000, 21450000, "M15"),
    (24890000, 24990000, "M12"),
    (28000000, 29700000, "M10"),
    (50000000, 52000000, "M6"),
    (70000000, 70500000, "M4"),
    (144000000, 146000000, "M2"),
    (430000000, 440000000, "M70"),
    (1240000000, 1300000000, "Cm23"),
    (2320000000, 2450000000, "Cm13"),
    (3400000000, 3475000000, "Cm9"),
    (5650000000, 5850000000, "Cm6"),
    (10000000000, 10500000000, "Cm3"),
    (220000000, 225000000, "M125"),
    (902000000, 928000000, "Cm33"),
]

# Listening bands by frequency
LISTENING_BANDS = [
    (148500, 283500, "Lw"),
    (526500, 1606500, "Mw"),
    (530000, 1700000, "Mw"),
    (2300000, 26100000, None),  # SW - too broad, skip as band
    (87500000, 108000000, "Fm"),
]

README = [
    "Converted from SDR++ bandplan.",
    "Edit, save, then press RELOAD BAND PLAN on the General tab.",
]


def hz_to_mhz(hz):
    return round(hz / 1_000_000.0, 4)


def identify_band(start, end):
    """Identify sdroxide Band from frequency range."""
    for lo, hi, band in AMATEUR_BANDS:
        if abs(start - lo) < 1000 and abs(end - hi) < 1000:
            return band
    for lo, hi, band in LISTENING_BANDS:
        if band and abs(start - lo) < 1000 and abs(end - hi) < 1000:
            return band
    return None


def convert(sdrpp):
    bands = sdrpp.get("bands", [])

    amateur_bands = {}
    segments = []

    for b in bands:
        name = b.get("name", "")
        btype = b.get("type", "other")
        start = b.get("start", 0)
        end = b.get("end", 0)
        if start >= end:
            continue

        band_enum = identify_band(start, end)

        if band_enum is not None:
            amateur_bands[band_enum] = (start, end)

        kind = TYPE_TO_KIND.get(btype, "All")
        segments.append({
            "lo_mhz": hz_to_mhz(start),
            "hi_mhz": hz_to_mhz(end),
            "kind": kind,
            "label": name,
        })

    band_list = []
    for band_name, (start, end) in amateur_bands.items():
        band_list.append({
            "band": band_name,
            "lo_mhz": hz_to_mhz(start),
            "hi_mhz": hz_to_mhz(end),
        })

    segments.sort(key=lambda s: s["lo_mhz"])

    plan = {
        "readme": README,
        "region1": {
            "bands": [dict(b) for b in band_list],
            "segments": [dict(s) for s in segments],
        },
        "region2": {
            "bands": [dict(b) for b in band_list],
            "segments": [dict(s) for s in segments],
        },
        "region3": {
            "bands": [dict(b) for b in band_list],
            "segments": [dict(s) for s in segments],
        },
    }

    return plan


def main():
    parser = argparse.ArgumentParser(description="Convert SDR++ bandplan to sdroxide format")
    parser.add_argument("input", help="SDR++ bandplan JSON file")
    parser.add_argument("output", help="Output sdroxide bandplan JSON file")
    args = parser.parse_args()

    with open(args.input) as f:
        sdrpp = json.load(f)

    plan = convert(sdrpp)

    with open(args.output, 'w') as f:
        json.dump(plan, f, indent=2)
        f.write('\n')

    print("Converted %d SDR++ bands to %d sdroxide bands and %d segments." % (
        len(sdrpp.get('bands', [])),
        len(plan['region1']['bands']),
        len(plan['region1']['segments'])))


if __name__ == "__main__":
    main()
