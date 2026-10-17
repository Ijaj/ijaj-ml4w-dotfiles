//! promptmap: re-color an oh-my-posh theme from the wallpaper palette.
//!
//! Every hardcoded hex color in the source theme is replaced by a `p:<name>` palette
//! reference, and the theme gets a `palette` block built from the current wallpaper palette:
//!  - greys become the palette's own neutral roles (surface, on_surface, ...)
//!  - colored hexes become derived accent slots: the theme's hue relationships are rotated
//!    onto the wallpaper's primary hue (reds go to the error hue), chroma follows the
//!    wallpaper's vividness, lightness is kept from the source theme
//!  - fg/bg pairs that lose readability are nudged until they pass a contrast target
//!
//! usage: promptmap <theme.omp.json> --palette <colors.css> [--spread 0.85] [--stats]
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::{env, fs, process};

// ---- color math -------------------------------------------------------------------------

fn parse_hex(h: &str) -> [f64; 3] {
    let v = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap_or(0) as f64 / 255.0;
    [v(1), v(3), v(5)]
}
fn to_lin(c: f64) -> f64 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}
fn from_lin(c: f64) -> f64 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
}
fn oklab(hex: &str) -> [f64; 3] {
    let [r, g, b] = parse_hex(hex).map(to_lin);
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    [
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    ]
}
/// (lightness, chroma, hue in degrees)
fn lch(hex: &str) -> (f64, f64, f64) {
    let [l, a, b] = oklab(hex);
    (l, a.hypot(b), b.atan2(a).to_degrees().rem_euclid(360.0))
}
fn lin_rgb(l: f64, c: f64, h: f64) -> [f64; 3] {
    let (a, b) = (c * h.to_radians().cos(), c * h.to_radians().sin());
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
    let (l3, m3, s3) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
    [
        4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3,
        -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3,
        -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3,
    ]
}
/// OKLCH to hex; chroma is reduced until the color fits sRGB
fn to_hex(l: f64, c: f64, h: f64) -> String {
    let fits = |c: f64| lin_rgb(l, c, h).iter().all(|v| (-0.0005..=1.0005).contains(v));
    let mut c = c.max(0.0);
    if !fits(c) {
        let (mut lo, mut hi) = (0.0, c);
        for _ in 0..24 {
            let mid = (lo + hi) / 2.0;
            if fits(mid) { lo = mid } else { hi = mid }
        }
        c = lo;
    }
    let [r, g, b] = lin_rgb(l, c, h).map(from_lin);
    format!("#{:02x}{:02x}{:02x}", (r * 255.0).round() as u8, (g * 255.0).round() as u8, (b * 255.0).round() as u8)
}
fn luminance(hex: &str) -> f64 {
    let [r, g, b] = parse_hex(hex).map(to_lin);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
fn contrast(a: &str, b: &str) -> f64 {
    let (x, y) = (luminance(a), luminance(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}
fn hue_dist(a: f64, b: f64) -> f64 {
    let d = (a - b).abs() % 360.0;
    d.min(360.0 - d)
}
fn signed_hue(from: f64, to: f64) -> f64 {
    (to - from + 540.0).rem_euclid(360.0) - 180.0
}

// ---- palette ------------------------------------------------------------------------------

fn load_palette(path: &str) -> BTreeMap<String, String> {
    let text = fs::read_to_string(path).unwrap_or_else(|e| die(&format!("palette {path}: {e}")));
    let mut pal = BTreeMap::new();
    for line in text.lines() {
        let Some(rest) = line.trim().strip_prefix("@define-color ") else { continue };
        let mut it = rest.split_whitespace();
        let (Some(name), Some(val)) = (it.next(), it.next()) else { continue };
        let val = val.trim_end_matches(';');
        if val.len() >= 7 && val.starts_with('#') {
            pal.insert(name.to_string(), val[..7].to_lowercase());
        }
    }
    if pal.is_empty() { die("no colors found in palette file") }
    pal
}
fn die(msg: &str) -> ! {
    eprintln!("promptmap: {msg}");
    process::exit(1)
}

const NEUTRALS: [&str; 12] = [
    "surface_container_lowest", "surface", "surface_container_low", "surface_container",
    "surface_container_high", "surface_container_highest", "surface_bright", "outline_variant",
    "outline", "on_surface_variant", "on_surface", "inverse_surface",
];

// ---- theme walking --------------------------------------------------------------------------

const COLOR_KEYS: [&str; 2] = ["foreground", "background"];
const TEMPLATE_KEYS: [&str; 2] = ["foreground_templates", "background_templates"];

/// every color-bearing string of a dict: (is_foreground, string)
fn color_strings(m: &mut Map<String, Value>) -> Vec<(bool, &mut String)> {
    let mut out = Vec::new();
    for (k, v) in m.iter_mut() {
        let fg = k.starts_with("foreground");
        if COLOR_KEYS.contains(&k.as_str()) {
            if let Value::String(s) = v { out.push((fg, s)) }
        } else if TEMPLATE_KEYS.contains(&k.as_str()) {
            if let Value::Array(a) = v {
                for x in a { if let Value::String(s) = x { out.push((fg, s)) } }
            }
        }
    }
    out
}
fn visit_dicts(v: &mut Value, f: &mut dyn FnMut(&mut Map<String, Value>)) {
    match v {
        Value::Object(m) => {
            f(m);
            for x in m.values_mut() { visit_dicts(x, f) }
        }
        Value::Array(a) => for x in a { visit_dicts(x, f) },
        _ => {}
    }
}
/// byte ranges of #rrggbb tokens
fn hex_tokens(s: &str) -> Vec<(usize, usize)> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 7 <= b.len() {
        if b[i] == b'#' && b[i + 1..i + 7].iter().all(u8::is_ascii_hexdigit)
            && !b.get(i + 7).is_some_and(u8::is_ascii_hexdigit)
        {
            out.push((i, i + 7));
            i += 7;
        } else {
            i += 1;
        }
    }
    out
}
fn replace_hex(s: &str, f: &mut dyn FnMut(&str) -> String) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    for (a, b) in hex_tokens(s) {
        out.push_str(&s[last..a]);
        out.push_str(&f(&s[a..b].to_lowercase()));
        last = b;
    }
    out.push_str(&s[last..]);
    out
}

// ---- mapping --------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Anchor { Primary, Error }

struct Slot {
    name: String,
    anchor: Anchor,
    l: f64,
    crel: f64,
    dh: f64,
    dl: f64,
}

#[derive(Clone)]
enum Ref { Role(String), Slot(usize), Bg }

struct Pair { fg: Ref, bg: Ref, target: f64 }

struct Ctx<'a> {
    pal: &'a BTreeMap<String, String>,
    slots: Vec<Slot>,
    hp: f64,
    he: f64,
    cp: f64,
}
impl Ctx<'_> {
    fn slot_hex(&self, s: &Slot) -> String {
        let h = (if s.anchor == Anchor::Primary { self.hp } else { self.he }) + s.dh;
        to_hex((s.l + s.dl).clamp(0.02, 0.98), s.crel * self.cp, h.rem_euclid(360.0))
    }
    fn hex(&self, r: &Ref) -> String {
        match r {
            Ref::Role(n) => self.pal[n].clone(),
            Ref::Slot(i) => self.slot_hex(&self.slots[*i]),
            Ref::Bg => self.pal.get("background").cloned().unwrap_or_else(|| "#000000".into()),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let opt = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    let theme_path = args.first().filter(|a| !a.starts_with("--")).unwrap_or_else(|| die("usage: promptmap <theme.omp.json> --palette <colors.css>"));
    let pal_path = opt("--palette").unwrap_or_else(|| die("--palette <colors.css> is required"));
    let spread: f64 = opt("--spread").and_then(|s| s.parse().ok()).unwrap_or(0.85);
    let pal = load_palette(&pal_path);
    let text = fs::read_to_string(theme_path).unwrap_or_else(|e| die(&format!("{theme_path}: {e}")));
    let mut theme: Value = serde_json::from_str(&text).unwrap_or_else(|e| die(&format!("{theme_path}: {e}")));

    // 1. inline the theme's own palette refs, then drop its palette
    let own: BTreeMap<String, String> = theme
        .get("palette")
        .and_then(Value::as_object)
        .map(|o| o.iter().filter_map(|(k, v)| v.as_str().filter(|s| s.starts_with('#')).map(|s| (k.clone(), s.to_string()))).collect())
        .unwrap_or_default();
    if let Value::Object(m) = &mut theme {
        m.remove("palette");
        m.remove("palettes");
    }
    visit_dicts(&mut theme, &mut |m| {
        for (_, s) in color_strings(m) {
            if !s.contains("p:") { continue }
            let mut out = String::new();
            let mut rest = s.as_str();
            while let Some(i) = rest.find("p:") {
                out.push_str(&rest[..i]);
                let tail = &rest[i + 2..];
                let end = tail.find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-')).unwrap_or(tail.len());
                match own.get(&tail[..end]) {
                    Some(h) => out.push_str(h),
                    None => out.push_str(&rest[i..i + 2 + end]),
                }
                rest = &tail[end..];
            }
            out.push_str(rest);
            *s = out;
        }
    });

    // 2. collect: usage counts and fg/bg groups per dict
    let mut use_count: BTreeMap<String, usize> = BTreeMap::new();
    let mut groups: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    visit_dicts(&mut theme, &mut |m| {
        let (mut fg, mut bg) = (Vec::new(), Vec::new());
        for (is_fg, s) in color_strings(m) {
            for (a, b) in hex_tokens(s) {
                let h = s[a..b].to_lowercase();
                *use_count.entry(h.clone()).or_default() += 1;
                let list = if is_fg { &mut fg } else { &mut bg };
                if !list.contains(&h) { list.push(h) }
            }
        }
        if !fg.is_empty() || !bg.is_empty() { groups.push((fg, bg)) }
    });

    // 3. classify source colors
    let get = |n: &str, d: &str| pal.get(n).cloned().unwrap_or_else(|| d.to_string());
    let primary = get("primary", "#b9c3ff");
    let (_, cprim, hp) = lch(&primary);
    let (_, ctert, _) = lch(&get("tertiary", &primary));
    let (_, _, he) = lch(&get("error", "#ffb4ab"));
    let cp = (cprim.max(ctert) * 1.25).clamp(0.05, 0.22);

    let mut neutral_of: BTreeMap<String, String> = BTreeMap::new();
    let ladder: Vec<(f64, &str)> = NEUTRALS.iter().filter_map(|n| pal.get(*n).map(|h| (lch(h).0, *n))).collect();
    let chromatic: Vec<&String> = use_count.keys().filter(|h| lch(h).1 >= 0.04).collect();
    for h in use_count.keys() {
        if lch(h).1 < 0.04 && !ladder.is_empty() {
            let l = lch(h).0;
            let n = ladder.iter().min_by(|a, b| (a.0 - l).abs().total_cmp(&(b.0 - l).abs())).unwrap().1;
            neutral_of.insert(h.clone(), n.to_string());
        }
    }
    // dominant hue: most used chromatic color, averaged with its 30-degree neighbours
    let (mut h0, mut c0) = (0.0, 0.1);
    let non_red: Vec<&&String> = chromatic.iter().filter(|h| hue_dist(lch(h).2, 29.0) >= 22.0).collect();
    let pool: Vec<&&String> = if non_red.is_empty() { chromatic.iter().collect() } else { non_red };
    if let Some(top) = pool.iter().max_by_key(|h| use_count[***h]) {
        let ht = lch(top).2;
        let (mut sx, mut sy, mut sc, mut w) = (0.0, 0.0, 0.0, 0.0);
        for h in pool.iter().filter(|h| hue_dist(lch(h).2, ht) < 30.0) {
            let (_, c, hh) = lch(h);
            let wt = use_count[h.as_str()] as f64;
            sx += hh.to_radians().cos() * wt;
            sy += hh.to_radians().sin() * wt;
            sc += c * wt;
            w += wt;
        }
        h0 = sy.atan2(sx).to_degrees().rem_euclid(360.0);
        c0 = (sc / w).max(0.04);
    }

    // 4. slots: one per (hex, fg/bg usage)
    let mut ctx = Ctx { pal: &pal, slots: Vec::new(), hp, he, cp };
    let mut slot_of: BTreeMap<(String, bool), usize> = BTreeMap::new();
    for (fg, bg) in &groups {
        for (list, is_fg) in [(fg, true), (bg, false)] {
            for h in list {
                if neutral_of.contains_key(h) || slot_of.contains_key(&(h.clone(), is_fg)) { continue }
                let (l, c, hue) = lch(h);
                let red = hue_dist(hue, 29.0) < 22.0 && c > 0.08;
                let (anchor, dh) = if red { (Anchor::Error, signed_hue(29.0, hue) * spread) } else { (Anchor::Primary, signed_hue(h0, hue) * spread) };
                ctx.slots.push(Slot { name: format!("pm{}", ctx.slots.len()), anchor, l, crel: (c / c0).clamp(0.2, 2.5), dh, dl: 0.0 });
                slot_of.insert((h.clone(), is_fg), ctx.slots.len() - 1);
            }
        }
    }
    let make_ref = |h: &String, is_fg: bool| -> Ref {
        match neutral_of.get(h) {
            Some(n) => Ref::Role(n.clone()),
            None => Ref::Slot(slot_of[&(h.clone(), is_fg)]),
        }
    };

    // 5. readability pairs (target follows the source theme, within 3.5..4.5)
    let mut pairs = Vec::new();
    for (fg, bg) in &groups {
        for f in fg {
            let bgs: Vec<Option<&String>> = if bg.is_empty() { vec![None] } else { bg.iter().map(Some).collect() };
            for b in bgs {
                let bhex = b.cloned().unwrap_or_else(|| ctx.hex(&Ref::Bg));
                let target = contrast(f, &bhex).clamp(3.5, 4.5);
                pairs.push(Pair { fg: make_ref(f, true), bg: b.map(|b| make_ref(b, false)).unwrap_or(Ref::Bg), target });
            }
        }
    }
    let mut pairs_total = 0;
    let mut pairs_fixed = 0;
    let mut pairs_left = 0;
    for p in &pairs {
        pairs_total += 1;
        if contrast(&ctx.hex(&p.fg), &ctx.hex(&p.bg)) < p.target - 0.1 { pairs_fixed += 1 }
    }
    for _ in 0..40 {
        let mut changed = false;
        for p in &pairs {
            let (fh, bh) = (ctx.hex(&p.fg), ctx.hex(&p.bg));
            if contrast(&fh, &bh) >= p.target - 0.1 { continue }
            // nudge the accent slot (prefer the foreground) away from the other color's luminance
            let (idx, other) = match (&p.fg, &p.bg) {
                (Ref::Slot(i), _) if ctx.slots[*i].dl.abs() < 0.45 => (*i, bh),
                (_, Ref::Slot(i)) if ctx.slots[*i].dl.abs() < 0.45 => (*i, fh),
                _ => continue,
            };
            let dir = if luminance(&other) > 0.18 { -1.0 } else { 1.0 };
            ctx.slots[idx].dl += 0.025 * dir;
            changed = true;
        }
        if !changed { break }
    }
    for p in &pairs {
        if contrast(&ctx.hex(&p.fg), &ctx.hex(&p.bg)) < p.target - 0.1 { pairs_left += 1 }
    }

    // 6. rewrite colors as palette refs
    visit_dicts(&mut theme, &mut |m| {
        for (is_fg, s) in color_strings(m) {
            *s = replace_hex(s, &mut |h| match neutral_of.get(h) {
                Some(n) => format!("p:{n}"),
                None => format!("p:{}", ctx.slots[slot_of[&(h.to_string(), is_fg)]].name),
            });
        }
    });
    let mut palette = Map::new();
    for (k, v) in &pal { palette.insert(k.clone(), Value::String(v.clone())); }
    for s in &ctx.slots { palette.insert(s.name.clone(), Value::String(ctx.slot_hex(s))); }
    if let Value::Object(m) = &mut theme { m.insert("palette".into(), Value::Object(palette)); }

    if args.iter().any(|a| a == "--stats") {
        eprintln!(
            "colors={} neutral={} slots={} pairs={} weak_before={} weak_after={}",
            use_count.len(), neutral_of.len(), ctx.slots.len(), pairs_total, pairs_fixed, pairs_left
        );
    }
    println!("{}", serde_json::to_string_pretty(&theme).unwrap());
}
