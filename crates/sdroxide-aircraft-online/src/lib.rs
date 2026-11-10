//! The optional online half of an aircraft card: what adsbdb knows about the
//! airframe and the flight, and Planespotters' photo of it.
//!
//! Everything but [`fetch`] is a decision, never a side effect — which requests
//! a card needs, whether an answer is still good, what a reply means — so it
//! is tested without a network and compiles for wasm32, where the client reads
//! [`Info`] and nothing else. The rules it keeps:
//!
//! * no key, no account;
//! * only fixed hosts are ever asked, and only for an ICAO address, a callsign
//!   or a registration that has been cleaned first;
//! * answers are kept as JSON for at most 24 h (Planespotters' own limit),
//!   a failure for five minutes, and a photo is never fetched or kept — only
//!   the address Planespotters gave for it, unchanged, and the credit. The one
//!   exception, on the operator's instruction of 2026-10-27, is [`Relay::image`]:
//!   a browser cannot read the picture itself (the host sends no CORS header),
//!   so the station hands it over once, from memory, for the open card only.
//!
//! The [`Relay`] is what a station runs so that its clients do not each reach
//! the internet themselves: one question out per key however many clients ask,
//! a ceiling on questions per minute, and one cache for all of them.

mod cache;
mod model;
mod parse;
mod relay;
mod weather;
mod wikipedia;

#[cfg(feature = "fetch")]
pub mod fetch;

pub use cache::{BACKOFF_S, MAX_ENTRIES, OnlineCache, TTL_S};
pub use model::{
    Adsbdb, AircraftInfo, Airport, Image, Lookup, MAX_IMAGE, Photo, Reply, Request, Route,
    clean_callsign, clean_registration, image_ok, thumb_url_ok, user_agent,
};
pub use parse::{parse_adsbdb, parse_photo, reply_for};
pub use relay::{FORMAT_VERSION, Info, Outlook, Relay, Status, Upstream};
pub use weather::{Spot, WEATHER_VERSION, Weather, parse_forecast};
pub use wikipedia::{Article, image_url_ok, parse_summary};
