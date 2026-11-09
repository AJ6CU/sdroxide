//! Not a check: turns the three source pictures (a dark symbol on a light
//! ground) into the vector data in `data.rs`, so the map draws them as
//! filled triangles that stay sharp at any size, with no picture shipped.
//!
//! `ICON_SRC=<dir holding aircraft.png airport.png vor.png>
//!  cargo test -p sdroxide-ui --lib trace_icons -- --ignored`
//!
//! Steps per picture: threshold to a mask; follow the edges between filled and
//! empty pixels into closed loops; drop specks and holes; simplify each loop
//! (Douglas-Peucker); scale everything to ±1 about the centre of the bounding
//! box (y down, nose or top up); split into triangles by ear clipping.

use std::collections::HashMap;
use std::fmt::Write as _;

type P = (f32, f32);

/// The closed loops of the filled pixels, each a list of corner points.
fn loops(mask: &[bool], w: usize, h: usize) -> Vec<Vec<P>> {
    let at = |x: i64, y: i64| {
        x >= 0
            && y >= 0
            && (x as usize) < w
            && (y as usize) < h
            && mask[y as usize * w + x as usize]
    };
    // Edges keep the filled pixel on their right (clockwise in screen space).
    let mut next: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    for y in 0..h as i64 {
        for x in 0..w as i64 {
            if !at(x, y) {
                continue;
            }
            if !at(x, y - 1) {
                next.entry((x, y)).or_default().push((x + 1, y));
            }
            if !at(x + 1, y) {
                next.entry((x + 1, y)).or_default().push((x + 1, y + 1));
            }
            if !at(x, y + 1) {
                next.entry((x + 1, y + 1)).or_default().push((x, y + 1));
            }
            if !at(x - 1, y) {
                next.entry((x, y + 1)).or_default().push((x, y));
            }
        }
    }
    let mut out = Vec::new();
    loop {
        let Some(&start) = next.iter().find(|(_, v)| !v.is_empty()).map(|(k, _)| k) else { break };
        let mut pts = vec![start];
        let mut cur = start;
        loop {
            let Some(to) = next.get_mut(&cur).and_then(|v| v.pop()) else { break };
            if to == start {
                break;
            }
            pts.push(to);
            cur = to;
        }
        out.push(pts.iter().map(|&(x, y)| (x as f32, y as f32)).collect());
    }
    out
}

fn area(p: &[P]) -> f32 {
    let mut a = 0.0;
    for i in 0..p.len() {
        let (x1, y1) = p[i];
        let (x2, y2) = p[(i + 1) % p.len()];
        a += x1 * y2 - x2 * y1;
    }
    a * 0.5
}

fn dist_to_segment(p: P, a: P, b: P) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len2 = dx * dx + dy * dy;
    let t = if len2 == 0.0 {
        0.0
    } else {
        (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / len2).clamp(0.0, 1.0)
    };
    ((p.0 - a.0 - t * dx).powi(2) + (p.1 - a.1 - t * dy).powi(2)).sqrt()
}

fn dp(pts: &[P], eps: f32) -> Vec<P> {
    if pts.len() < 3 {
        return pts.to_vec();
    }
    let (a, b) = (pts[0], pts[pts.len() - 1]);
    let (mut far, mut at) = (0.0, 0);
    for (i, &p) in pts.iter().enumerate().take(pts.len() - 1).skip(1) {
        let d = dist_to_segment(p, a, b);
        if d > far {
            (far, at) = (d, i);
        }
    }
    if far <= eps {
        return vec![a, b];
    }
    let mut left = dp(&pts[..=at], eps);
    let right = dp(&pts[at..], eps);
    left.pop();
    left.extend(right);
    left
}

/// A closed loop, simplified: split at two far-apart corners so the open-line
/// algorithm can take each half.
fn simplify(p: &[P], eps: f32) -> Vec<P> {
    let n = p.len();
    let (mut best, mut ia, mut ib) = (0.0, 0, n / 2);
    let step = (n / 300).max(1);
    for i in (0..n).step_by(step) {
        for j in ((i + 1)..n).step_by(step) {
            let d = (p[i].0 - p[j].0).powi(2) + (p[i].1 - p[j].1).powi(2);
            if d > best {
                (best, ia, ib) = (d, i, j);
            }
        }
    }
    let first: Vec<P> = p[ia..=ib].to_vec();
    let mut second: Vec<P> = p[ib..].to_vec();
    second.extend_from_slice(&p[..=ia]);
    let mut a = dp(&first, eps);
    let b = dp(&second, eps);
    a.pop();
    a.extend(b);
    a.pop();
    a
}

fn cross(o: P, a: P, b: P) -> f32 {
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

/// Ear clipping of one simple polygon, clockwise on screen (positive area).
fn ears(poly: &[P], base: u32, tris: &mut Vec<u32>) {
    let mut idx: Vec<usize> = (0..poly.len()).collect();
    let inside = |p: P, a: P, b: P, c: P| {
        cross(a, b, p) >= 0.0 && cross(b, c, p) >= 0.0 && cross(c, a, p) >= 0.0
    };
    while idx.len() > 3 {
        let n = idx.len();
        let mut cut = None;
        for k in 0..n {
            let (a, b, c) = (idx[(k + n - 1) % n], idx[k], idx[(k + 1) % n]);
            if cross(poly[a], poly[b], poly[c]) <= 0.0 {
                continue;
            }
            let clear = idx
                .iter()
                .filter(|&&i| i != a && i != b && i != c)
                .all(|&i| !inside(poly[i], poly[a], poly[b], poly[c]));
            if clear {
                cut = Some((k, a, b, c));
                break;
            }
        }
        // A degenerate loop with no ear: take the first corner so it ends.
        let (k, a, b, c) = cut.unwrap_or((0, idx[n - 1], idx[0], idx[1]));
        tris.extend([base + a as u32, base + b as u32, base + c as u32]);
        idx.remove(k);
    }
    if idx.len() == 3 {
        tris.extend(idx.iter().map(|&i| base + i as u32));
    }
}

/// Vertices, triangle indices and the number of vertices of each loop.
type Traced = (Vec<P>, Vec<u32>, Vec<u32>);

fn trace(path: &std::path::Path, eps: f32, min_area: f32) -> Traced {
    let img = image::open(path).unwrap().to_rgba8();
    let (w, h) = (img.width() as usize, img.height() as usize);
    let mask: Vec<bool> = img
        .pixels()
        .map(|p| {
            let a = f32::from(p[3]) / 255.0;
            let lum = (0.299 * f32::from(p[0]) + 0.587 * f32::from(p[1]) + 0.114 * f32::from(p[2]))
                * a
                + 255.0 * (1.0 - a);
            lum < 128.0
        })
        .collect();
    let kept: Vec<Vec<P>> = loops(&mask, w, h)
        .into_iter()
        .filter(|l| area(l) > min_area)
        .map(|l| simplify(&l, eps))
        .collect();
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for &(x, y) in kept.iter().flatten() {
        (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
    }
    let (cx, cy, half) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0, ((x1 - x0).max(y1 - y0)) / 2.0);
    let mut verts = Vec::new();
    let mut tris = Vec::new();
    let mut lens = Vec::new();
    for l in &kept {
        let poly: Vec<P> = l.iter().map(|&(x, y)| ((x - cx) / half, (y - cy) / half)).collect();
        ears(&poly, verts.len() as u32, &mut tris);
        lens.push(poly.len() as u32);
        verts.extend(poly);
    }
    (verts, tris, lens)
}

#[test]
#[ignore]
fn trace_icons() {
    let dir = std::path::PathBuf::from(std::env::var("ICON_SRC").expect("ICON_SRC"));
    let mut out = String::from(
        "//! Generated by `trace_icons` (see `trace.rs`) from the three source\n\
         //! pictures; do not edit by hand. Coordinates are ±1 about the centre,\n\
         //! y down, the nose (or the top) up; `*_TRIS` index `*_VERTS` in threes.\n\n",
    );
    for (name, file, eps, min_area) in [
        ("AIRCRAFT", "aircraft.png", 1.2, 60.0),
        ("AIRPORT", "airport.png", 1.2, 60.0),
        ("VOR", "vor.png", 1.2, 60.0),
    ] {
        let (v, t, lens) = trace(&dir.join(file), eps, min_area);
        writeln!(out, "pub const {name}_VERTS: &[[f32; 2]] = &[").unwrap();
        for chunk in v.chunks(6) {
            let row: Vec<String> = chunk.iter().map(|(x, y)| format!("[{x:.4}, {y:.4}]")).collect();
            writeln!(out, "    {},", row.join(", ")).unwrap();
        }
        writeln!(out, "];\npub const {name}_TRIS: &[u32] = &[").unwrap();
        for chunk in t.chunks(12) {
            let row: Vec<String> = chunk.iter().map(u32::to_string).collect();
            writeln!(out, "    {},", row.join(", ")).unwrap();
        }
        let lens: Vec<String> = lens.iter().map(u32::to_string).collect();
        writeln!(out, "];\npub const {name}_LOOPS: &[u32] = &[{}];\n", lens.join(", ")).unwrap();
        eprintln!("{name}: {} vertices, {} triangles", v.len(), t.len() / 3);
    }
    let target = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/adsb_icons/data.rs");
    std::fs::write(target, out).unwrap();
}
