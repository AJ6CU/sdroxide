//! One aircraft's kept track as a KML file, for Google Earth and the like.
//!
//! Pure text: the positions of a [`Journal`] and nothing else — the address,
//! the callsign and the registration name the file and the line, and that is
//! all. The track is cut where the radio lost the aircraft for a while, so two
//! passes are not joined by a straight line across the map.

use crate::export::iso_utc;
use crate::{Journal, Point};

/// A silence longer than this starts a new segment.
pub const GAP_S: i64 = 600;

const FT_TO_M: f64 = 0.3048;

/// The file name: `aircraft-4840D6-20261025.kml`.
pub fn file_name(j: &Journal, now: i64) -> String {
    format!("aircraft-{:06X}-{}.kml", j.icao & 0xff_ffff, iso_utc(now)[..10].replace('-', ""))
}

/// The positions in time order, cut at each silence longer than [`GAP_S`].
pub fn segments(j: &Journal) -> Vec<Vec<Point>> {
    let mut points = j.points.clone();
    points.sort_by_key(Point::time);
    let mut out: Vec<Vec<Point>> = Vec::new();
    for p in points {
        match out.last_mut() {
            Some(seg) if p.time() - seg.last().map_or(0, Point::time) <= GAP_S => seg.push(p),
            _ => out.push(vec![p]),
        }
    }
    out
}

/// Text made safe to put between tags.
fn xml(raw: &str) -> String {
    raw.chars()
        .filter(|c| !c.is_control() || matches!(c, '\n' | '\t'))
        .flat_map(|c| match c {
            '&' => "&amp;".chars().collect::<Vec<_>>(),
            '<' => "&lt;".chars().collect(),
            '>' => "&gt;".chars().collect(),
            '"' => "&quot;".chars().collect(),
            c => vec![c],
        })
        .collect()
}

/// The track as a KML document, or `None` when no position is kept.
pub fn track(j: &Journal) -> Option<String> {
    let segs = segments(j);
    if segs.is_empty() {
        return None;
    }
    let name = match j.callsign() {
        Some(c) => format!("{c} ({:06X})", j.icao & 0xff_ffff),
        None => format!("{:06X}", j.icao & 0xff_ffff),
    };
    let mut s = String::new();
    s.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    s.push_str(
        "<kml xmlns=\"http://www.opengis.net/kml/2.2\" \
         xmlns:gx=\"http://www.google.com/kml/ext/2.2\">\n<Document>\n",
    );
    s.push_str(&format!("<name>{}</name>\n", xml(&name)));
    if let Some(r) = j.registration() {
        s.push_str(&format!("<description>{}</description>\n", xml(r)));
    }
    s.push_str("<Style id=\"t\"><LineStyle><color>ff00a5ff</color><width>3</width></LineStyle></Style>\n");
    for (n, seg) in segs.iter().enumerate() {
        s.push_str(&format!("<Placemark><name>{}</name><styleUrl>#t</styleUrl>\n", n + 1));
        s.push_str("<gx:Track><altitudeMode>absolute</altitudeMode>\n");
        for p in seg {
            s.push_str(&format!("<when>{}</when>\n", iso_utc(p.time())));
        }
        for p in seg {
            let alt = if p.on_ground() {
                0.0
            } else {
                f64::from(p.altitude_ft().unwrap_or(0)) * FT_TO_M
            };
            s.push_str(&format!("<gx:coord>{:.5} {:.5} {:.0}</gx:coord>\n", p.lon(), p.lat(), alt));
        }
        s.push_str("</gx:Track></Placemark>\n");
    }
    s.push_str("</Document>\n</kml>\n");
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn j(points: Vec<Point>) -> Journal {
        Journal { icao: 0x4840D6, points, ..Journal::default() }
    }

    #[test]
    fn no_position_no_file() {
        assert_eq!(track(&j(vec![])), None);
    }

    #[test]
    fn a_track_holds_time_longitude_latitude_and_metres() {
        let k = track(&j(vec![
            Point(1_000_000_000, 52.3, 4.76, Some(35_000), false),
            Point(1_000_000_010, 52.31, 4.77, None, true),
        ]))
        .unwrap();
        assert!(k.contains("<when>2001-09-09T01:46:40Z</when>"), "{k}");
        assert!(k.contains("<gx:coord>4.76000 52.30000 10668</gx:coord>"), "{k}");
        assert!(k.contains("<gx:coord>4.77000 52.31000 0</gx:coord>"), "on the ground: 0 m");
        assert!(k.contains("<name>4840D6</name>"));
    }

    #[test]
    fn a_long_silence_cuts_the_track_in_two() {
        let segs = segments(&j(vec![
            Point(5_000, 1.0, 1.0, None, false),
            Point(1_000, 1.0, 1.0, None, false),
            Point(1_000 + GAP_S, 1.0, 1.0, None, false),
            Point(1_000 + GAP_S + 1 + 3_000, 1.0, 1.0, None, false),
        ]));
        let lens: Vec<usize> = segs.iter().map(Vec::len).collect();
        assert_eq!(lens, vec![2, 2], "sorted first, then cut");
    }

    #[test]
    fn the_names_are_escaped() {
        let mut journal = j(vec![Point(1, 0.0, 0.0, None, false)]);
        journal.callsigns.push(crate::Seen { text: "A<&>\"B".into(), first: 1, last: 1 });
        let k = track(&journal).unwrap();
        assert!(k.contains("A&lt;&amp;&gt;&quot;B (4840D6)"), "{k}");
        assert!(!k.contains("A<&"));
    }

    #[test]
    fn the_file_is_named_by_address_and_day() {
        assert_eq!(file_name(&j(vec![]), 1_000_000_000), "aircraft-4840D6-20010909.kml");
    }
}
