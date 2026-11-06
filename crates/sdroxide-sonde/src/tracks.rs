//! The book of sondes heard: one [`SondeTrack`] per serial number, updated
//! frame by frame. Pure data handling, no threads, so it is tested directly.

use sdroxide_types::{SONDE_MAX_TRACKS, SONDE_TRACK_DEPTH, SondePoint, SondeTrack};
use crate::model::SondeFix;

/// Every sonde heard since the lane started, most recently heard first.
#[derive(Debug, Default)]
pub struct TrackBook {
    sondes: Vec<SondeTrack>,
}

impl TrackBook {
    /// Fold one decoded frame in. `freq_hz` is the channel it was heard on and
    /// `unix` the time it was heard.
    pub fn push(&mut self, f: &SondeFix, freq_hz: f64, unix: i64) {
        let idx = match self.sondes.iter().position(|s| s.serial == f.serial) {
            Some(i) => i,
            None => {
                self.sondes.insert(
                    0,
                    SondeTrack {
                        model: f.model.name().to_string(),
                        serial: f.serial.clone(),
                        freq_hz,
                        frame: 0,
                        lat: None,
                        lon: None,
                        alt_m: None,
                        speed_ms: None,
                        course_deg: None,
                        climb_ms: None,
                        num_sv: None,
                        battery_v: None,
                        last_unix: unix,
                        frames: 0,
                        track: Vec::new(),
                    },
                );
                self.sondes.truncate(SONDE_MAX_TRACKS);
                0
            }
        };
        let s = &mut self.sondes[idx];
        s.freq_hz = freq_hz;
        s.frame = f.frame_num;
        s.battery_v = f.battery_v.filter(|v| v.is_finite() && *v > 0.0);
        s.last_unix = unix;
        s.frames = s.frames.saturating_add(1);
        if let Some(p) = f.pos.as_ref().filter(|p| valid_fix(p.lat, p.lon)) {
            s.lat = Some(p.lat);
            s.lon = Some(p.lon);
            s.alt_m = Some(p.alt_m as f32);
            s.speed_ms = Some(p.speed_ms as f32);
            s.course_deg = Some(p.course_deg as f32);
            s.climb_ms = Some(p.climb_ms as f32);
            s.num_sv = Some(p.num_sv);
            // One point per second; a repeat of the same second adds nothing.
            let same = s.track.last().is_some_and(|l| l.unix == unix);
            if !same {
                if s.track.len() >= SONDE_TRACK_DEPTH {
                    thin(&mut s.track);
                }
                s.track.push(SondePoint { lat: p.lat, lon: p.lon, alt_m: p.alt_m as f32, unix });
            }
        }
        // Most recently heard first.
        if idx != 0 {
            let s = self.sondes.remove(idx);
            self.sondes.insert(0, s);
        }
    }

    pub fn snapshot(&self) -> Vec<SondeTrack> {
        self.sondes.clone()
    }
}

/// A GPS fix that is a place on Earth. A sonde without a lock sends zeros (or
/// the geocentre, which converts to nonsense), and neither belongs on a map.
fn valid_fix(lat: f64, lon: f64) -> bool {
    lat.is_finite()
        && lon.is_finite()
        && lat.abs() <= 90.0
        && lon.abs() <= 180.0
        && !(lat.abs() < 1e-6 && lon.abs() < 1e-6)
}

/// Drop every other point (keeping the newest), so a long flight keeps its
/// whole shape in bounded memory instead of losing its start.
fn thin(track: &mut Vec<SondePoint>) {
    let last = track.len().saturating_sub(1);
    let mut i = 0;
    track.retain(|_| {
        let keep = i % 2 == 0 || i == last;
        i += 1;
        keep
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::SondeModel;
    use xng_mode_sonde::GpsPos;

    fn frame(serial: &str, n: u16, lat: f64, lon: f64, alt: f64) -> SondeFix {
        SondeFix {
            model: SondeModel::Rs41,
            serial: serial.to_string(),
            frame_num: u32::from(n),
            battery_v: Some(2.7),
            pos: Some(GpsPos {
                lat,
                lon,
                alt_m: alt,
                speed_ms: 4.0,
                course_deg: 90.0,
                climb_ms: 5.0,
                num_sv: 9,
            }),
        }
    }

    #[test]
    fn frames_of_one_serial_build_one_track() {
        let mut b = TrackBook::default();
        for i in 0..5u16 {
            let lat = 48.0 + f64::from(i) * 1e-3;
            b.push(&frame("N1", i, lat, 2.0, 100.0 * f64::from(i)), 403e6, 10 + i64::from(i));
        }
        let s = b.snapshot();
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].frames, 5);
        assert_eq!(s[0].track.len(), 5);
        assert_eq!(s[0].frame, 4);
        assert_eq!(s[0].alt_m, Some(400.0));
        assert_eq!(s[0].climb_ms, Some(5.0));
    }

    #[test]
    fn the_latest_heard_sonde_comes_first_and_the_book_is_bounded() {
        let mut b = TrackBook::default();
        b.push(&frame("A", 1, 48.0, 2.0, 1.0), 403e6, 1);
        b.push(&frame("B", 1, 49.0, 2.0, 1.0), 403e6, 2);
        b.push(&frame("A", 2, 48.0, 2.0, 1.0), 403e6, 3);
        assert_eq!(b.snapshot()[0].serial, "A");
        for i in 0..20 {
            b.push(&frame(&format!("S{i}"), 1, 48.0, 2.0, 1.0), 403e6, 10 + i);
        }
        assert_eq!(b.snapshot().len(), SONDE_MAX_TRACKS);
    }

    #[test]
    fn a_frame_without_a_real_fix_adds_no_point() {
        let mut b = TrackBook::default();
        b.push(&frame("N1", 1, 0.0, 0.0, 0.0), 403e6, 1);
        let s = b.snapshot();
        assert_eq!(s[0].frames, 1, "the sonde was heard");
        assert!(s[0].lat.is_none() && s[0].track.is_empty(), "but it has no place yet");
    }

    #[test]
    fn a_long_flight_is_thinned_not_truncated() {
        let mut b = TrackBook::default();
        for i in 0..(SONDE_TRACK_DEPTH as i64 + 10) {
            b.push(&frame("N1", (i % 60000) as u16, 48.0, 2.0 + i as f64 * 1e-5, 1.0), 403e6, i);
        }
        let t = &b.snapshot()[0].track;
        assert!(t.len() <= SONDE_TRACK_DEPTH);
        assert_eq!(t[0].unix, 0, "the launch point is kept");
        assert_eq!(t.last().unwrap().unix, SONDE_TRACK_DEPTH as i64 + 9, "and the newest");
    }
}
