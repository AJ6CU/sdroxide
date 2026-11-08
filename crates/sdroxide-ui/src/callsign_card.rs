//! The QRZ card: what the station knows about a callsign, shown where the
//! pointer rests on one — in the decode lists, over the waterfall's boxes, in
//! the spot lists.
//!
//! **Nothing here calls QRZ.** The lookup runs on the station
//! (`sdroxide-net`), through the ordinary `Command::LookupCallsign` →
//! `CallsignResult` path, so a browser and a remote desktop get the same card
//! from the same single QRZ session. This module only remembers the answers,
//! asks for the ones it lacks, and draws them.
//!
//! The drawing sites are many and some of them (the waterfall's painter-drawn
//! boxes, the free functions that build a hover card) have no handle on the
//! app, so the state lives in a [`Hub`] kept in egui's own data store, one per
//! context: the app feeds it (active or not, results) and drains its requests
//! once a frame; the sites only call [`section`] or [`tooltip`].

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use eframe::egui::{self, Color32, RichText};
use sdroxide_types::CallsignInfo;

use crate::i18n::tr;

/// Seconds before a callsign that was asked for and never answered is asked for
/// again (a lookup the station dropped as one too many, or a failed login).
const RETRY_S: f64 = 20.0;
/// Cards kept on screen's side. The station keeps its own, for a day.
const KEEP: usize = 300;
const PHOTO_PT: f32 = 64.0;

struct Entry {
    info: CallsignInfo,
    tex: Option<egui::TextureHandle>,
    /// Decoding the photo was tried (and either worked or did not): once.
    tex_tried: bool,
}

#[derive(Default)]
struct State {
    active: bool,
    cache: HashMap<String, Entry>,
    /// `call -> egui time it was asked for`.
    asked: HashMap<String, f64>,
    requests: Vec<String>,
}

/// The card state of one egui context.
#[derive(Clone, Default)]
struct Hub(Arc<Mutex<State>>);

fn hub(ctx: &egui::Context) -> Hub {
    ctx.data_mut(|d| d.get_temp_mut_or_insert_with(egui::Id::new("qrz-card-hub"), Hub::default).clone())
}

/// Switch the card on or off for this frame
/// ([`sdroxide_types::NetworkConfig::qrz_card_active`]). Off, nothing is asked
/// for and nothing is drawn.
pub fn set_active(ctx: &egui::Context, active: bool) {
    let h = hub(ctx);
    if let Ok(mut s) = h.0.lock() {
        s.active = active;
        if !active {
            s.requests.clear();
        }
    }
}

pub fn is_active(ctx: &egui::Context) -> bool {
    hub(ctx).0.lock().map(|s| s.active).unwrap_or(false)
}

/// Whether a card would be drawn for `raw` — the card is on and `raw` can be a
/// callsign. A site that opens a hover of its own for the card asks first, so
/// that an operator without QRZ never meets an empty tooltip.
pub fn applies(ctx: &egui::Context, raw: &str) -> bool {
    is_active(ctx) && base_call(raw).is_some()
}

/// Remember a lookup result (also an empty one: "QRZ does not know this call").
/// Returns whether the card had asked for it, so the caller can keep a result
/// nobody typed a request for out of the net log.
pub fn store(ctx: &egui::Context, info: &CallsignInfo) -> bool {
    let h = hub(ctx);
    let Ok(mut s) = h.0.lock() else { return false };
    let key = info.call.to_ascii_uppercase();
    let asked = s.asked.remove(&key).is_some();
    if !s.active && !asked {
        return false;
    }
    if s.cache.len() >= KEEP {
        s.cache.clear();
    }
    s.cache.insert(key, Entry { info: info.clone(), tex: None, tex_tried: false });
    asked
}

/// The callsigns the screen wants looked up, once each.
pub fn take_requests(ctx: &egui::Context) -> Vec<String> {
    let h = hub(ctx);
    h.0.lock().map(|mut s| std::mem::take(&mut s.requests)).unwrap_or_default()
}

/// The callsign QRZ is asked about for what is written on screen: `F4ABC/P` and
/// `EA8/F4ABC` are both the operator behind `F4ABC`, and a hashed `<...>` or
/// free text is nobody. `None` when it cannot be a callsign.
pub fn base_call(raw: &str) -> Option<String> {
    let t = raw.trim().trim_matches(|c| c == '<' || c == '>').to_ascii_uppercase();
    if t.is_empty() || !t.chars().all(|c| c.is_ascii_alphanumeric() || c == '/') {
        return None;
    }
    let best = t.split('/').filter(|p| !p.is_empty()).max_by_key(|p| p.len())?;
    let ok = best.len() >= 3 && best.chars().any(|c| c.is_ascii_digit());
    ok.then(|| best.to_string())
}

/// The card, as a section of a hover that is already open (the decode list's
/// station card), or the whole of one ([`tooltip`]). Draws nothing when the
/// card is off or `raw` is no callsign; otherwise draws the record, or "…"
/// while it is on its way — and asks for it.
pub fn section(ui: &mut egui::Ui, raw: &str) {
    let ctx = ui.ctx().clone();
    let Some(call) = base_call(raw) else { return };
    let h = hub(&ctx);
    let now = ctx.input(|i| i.time);
    // Look under the lock, draw outside it.
    let found = {
        let Ok(mut s) = h.0.lock() else { return };
        if !s.active {
            return;
        }
        if s.cache.contains_key(&call) {
            // Decode the photo once, here, where the context is at hand.
            let e = s.cache.get_mut(&call).expect("checked");
            if !e.tex_tried {
                e.tex_tried = true;
                e.tex = e.info.photo.as_deref().and_then(|b| load_photo(&ctx, &call, b));
            }
            let e = &s.cache[&call];
            Some((e.info.clone(), e.tex.clone()))
        } else {
            let due = s.asked.get(&call).is_none_or(|t| now - *t >= RETRY_S);
            if due {
                s.asked.insert(call.clone(), now);
                s.requests.push(call.clone());
            }
            None
        }
    };
    match found {
        Some((info, tex)) => card(ui, &call, &info, tex.as_ref()),
        None => {
            ui.separator();
            ui.label(RichText::new(tr!("qrzcard-loading")).italics().color(crate::theme::gray(130)));
            // Keep painting so the card appears the moment the answer lands.
            ctx.request_repaint_after(std::time::Duration::from_millis(250));
        }
    }
}

/// A tooltip at the pointer with the card in it, for a painter-drawn box that
/// has no widget of its own to hang a hover on: pass the response of the
/// widget the box was drawn on, and say whether the pointer is on the box.
pub fn tooltip(resp: &egui::Response, hovered: bool, raw: &str) {
    // Not while the waterfall is being dragged or zoomed: the card would follow
    // the pointer across the whole band.
    if !hovered || resp.dragged() || !is_active(&resp.ctx) || base_call(raw).is_none() {
        return;
    }
    egui::Tooltip::for_widget(resp).at_pointer().gap(12.0).show(|ui| {
        ui.set_max_width(300.0);
        section(ui, raw);
    });
}

fn load_photo(ctx: &egui::Context, call: &str, bytes: &[u8]) -> Option<egui::TextureHandle> {
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let color = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
    Some(ctx.load_texture(format!("qrz-photo-{call}"), color, egui::TextureOptions::LINEAR))
}

/// One line per fact QRZ gave; a fact it did not give has no line.
fn card(ui: &mut egui::Ui, call: &str, i: &CallsignInfo, tex: Option<&egui::TextureHandle>) {
    ui.separator();
    if i.is_empty() {
        ui.label(RichText::new(tr!("qrzcard-not-found")).italics().color(crate::theme::gray(130)));
        return;
    }
    let dim = crate::theme::gray(150);
    ui.horizontal_top(|ui| {
        // The picture, or a plain tile where QRZ has none, so the card does
        // not change shape from one operator to the next.
        let (rect, _) = ui.allocate_exact_size(egui::vec2(PHOTO_PT, PHOTO_PT), egui::Sense::hover());
        match tex {
            Some(t) => {
                egui::Image::new(t)
                    .fit_to_exact_size(rect.size())
                    .corner_radius(4.0)
                    .paint_at(ui, rect);
            }
            None => {
                ui.painter().rect_filled(rect, 4.0, crate::theme::gray(45));
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "?",
                    egui::FontId::proportional(24.0),
                    crate::theme::gray(95),
                );
            }
        }
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            ui.label(RichText::new(call).size(14.0).strong().color(crate::theme::TEXT_STRONG()));
            if let Some(n) = &i.name {
                ui.label(RichText::new(n).size(13.0).color(crate::theme::TEXT()));
            }
            let mut who: Vec<String> = Vec::new();
            if let Some(a) = i.age {
                who.push(tr!("qrzcard-years", n = a as i64));
            }
            if let Some(c) = &i.class {
                who.push(tr!("qrzcard-class", class = c.as_str()));
            }
            if !who.is_empty() {
                ui.label(RichText::new(who.join(" · ")).size(11.5).color(dim));
            }
        });
    });
    // Where they are.
    let mut place: Vec<&str> = Vec::new();
    for s in [i.addr1.as_deref(), i.qth.as_deref(), i.state.as_deref(), i.country.as_deref()]
        .into_iter()
        .flatten()
    {
        place.push(s);
    }
    if !place.is_empty() {
        ui.label(RichText::new(place.join(", ")).size(11.5).color(crate::theme::TEXT()));
    }
    let mut codes: Vec<String> = Vec::new();
    if let Some(g) = &i.grid {
        codes.push(g.clone());
    }
    if let Some(d) = i.dxcc {
        codes.push(format!("DXCC {d}"));
    }
    if let Some(z) = i.cq_zone {
        codes.push(format!("CQ {z}"));
    }
    if let Some(z) = i.itu_zone {
        codes.push(format!("ITU {z}"));
    }
    if !codes.is_empty() {
        ui.label(
            RichText::new(codes.join(" · ")).monospace().size(11.0).color(crate::theme::CYAN_DIM()),
        );
    }
    if let Some(m) = &i.email {
        ui.label(RichText::new(m).size(11.0).color(dim));
    }
    let mut qsl: Vec<String> = Vec::new();
    if i.qsl_lotw {
        qsl.push("LoTW".into());
    }
    if i.qsl_eqsl {
        qsl.push("eQSL".into());
    }
    if i.qsl_paper {
        qsl.push(tr!("qrzcard-qsl-paper"));
    }
    if let Some(v) = &i.qsl_via {
        qsl.push(tr!("qrzcard-qsl-via", mgr = v.as_str()));
    }
    if !qsl.is_empty() {
        ui.label(RichText::new(format!("QSL: {}", qsl.join(", "))).size(11.0).color(dim));
    }
    ui.label(RichText::new("QRZ.com").size(9.5).color(Color32::from_gray(110)));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One pass; the texture changes the text asks for are dropped on purpose.
    fn run(ctx: &egui::Context, f: impl FnMut(&mut egui::Ui)) {
        ctx.run_ui(egui::RawInput::default(), f).textures_delta.clear();
    }

    #[test]
    fn what_is_asked_of_qrz_is_the_operators_own_call() {
        assert_eq!(base_call("F4ABC").as_deref(), Some("F4ABC"));
        assert_eq!(base_call(" f4abc/p ").as_deref(), Some("F4ABC"));
        assert_eq!(base_call("EA8/F4ABC").as_deref(), Some("F4ABC"));
        assert_eq!(base_call("<F4ABC>").as_deref(), Some("F4ABC"));
        assert_eq!(base_call("CQ DX"), None, "text is nobody");
        assert_eq!(base_call("?"), None);
        assert_eq!(base_call(""), None);
        assert_eq!(base_call("ABC"), None, "no digit, no call");
    }

    #[test]
    fn off_asks_for_nothing_and_on_asks_once() {
        let ctx = egui::Context::default();
        set_active(&ctx, false);
        run(&ctx, |ui| section(ui, "F4ABC"));
        assert!(take_requests(&ctx).is_empty(), "the card is off: no request");

        set_active(&ctx, true);
        run(&ctx, |ui| {
            section(ui, "F4ABC");
            section(ui, "f4abc/p");
        });
        assert_eq!(take_requests(&ctx), vec!["F4ABC".to_string()], "once, for the base call");
    }

    #[test]
    fn a_stored_answer_stops_the_asking() {
        let ctx = egui::Context::default();
        set_active(&ctx, true);
        let mut info = CallsignInfo::for_call("F4ABC");
        info.name = Some("Jean Dupont".into());
        store(&ctx, &info);
        run(&ctx, |ui| section(ui, "F4ABC"));
        assert!(take_requests(&ctx).is_empty());
    }
}
