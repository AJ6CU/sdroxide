//! **Look at the survey ground, in each of its styles.**
//!
//! `the_painted_ground_puts_land_where_the_land_is` proves a place on the Earth
//! came out as land, and that the styles differ. Neither can prove the thing
//! reads as a *map* — that is what an eye is for, and the whole point of the
//! change is that the ADS-B picture stops being a stipple (r/ShortwavePlus,
//! 2026-10-17: *"the map is dismal"*, and a moderator asking for *"a choice
//! between nav maps and regular terrain/road maps"*).
//!
//! A committed probe, not scratch: it draws the same image the air map does,
//! through the same code and the same style derivation, so the next person who
//! changes the ground can look at it before believing a test.
//!
//! ```text
//! cargo run --release -p sdroxide-ui --example survey_ground_probe
//! ```

use image::{Rgb, RgbImage};
use sdroxide_types::MapStyle;
use sdroxide_ui::{ground_image, theme};

fn main() {
    // Two views. The wide one has land, sea, borders and rivers in frame — the
    // map this has always been. The close one is the *detail*: internal
    // boundaries from a 40° span and roads from 8°, so a 6° view over the Low
    // Countries is where the new layers either read as a place or as a smudge.
    for (tag, view) in [("", (50.0, 5.0, 60.0, 36.0)), ("-close", (51.0, 5.0, 6.0, 3.6))] {
        for style in MapStyle::ALL {
            let map = theme::map_styled(style);
            let img = ground_image(view, (900, 540), &map, style);
            let mut png = RgbImage::new(img.size[0] as u32, img.size[1] as u32);
            for (i, px) in img.pixels.iter().enumerate() {
                let (x, y) = (i as u32 % img.size[0] as u32, i as u32 / img.size[0] as u32);
                png.put_pixel(x, y, Rgb([px.r(), px.g(), px.b()]));
            }
            let name = style.label().to_lowercase();
            let path = format!("/tmp/opencode/ground-{name}{tag}.png");
            match png.save(&path) {
                Ok(()) => println!("wrote {path}"),
                Err(e) => println!("could not write {path}: {e}"),
            }
        }
    }
    println!(
        "\nLook at them: radar is the dark ground the picture is read against, \
         terrain lifts the land, chart is the pale marked-up one with its \
         graticule. The `-close` pair is the one to check the detail layers on."
    );
}
