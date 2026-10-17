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
/// like hex_tokens, plus #rgb shorthand (only used inside color strings)
fn hex_tokens_short(s: &str) -> Vec<(usize, usize)> {
    let mut out = hex_tokens(s);
    let b = s.as_bytes();
    let mut i = 0;
    while i + 4 <= b.len() {
        let covered = out.iter().any(|&(x, y)| i >= x && i < y);
        if !covered && b[i] == b'#' && b[i + 1..i + 4].iter().all(u8::is_ascii_hexdigit)
            && !b.get(i + 4).is_some_and(|c| c.is_ascii_alphanumeric())
        {
            out.push((i, i + 4));
            i += 4;
        } else {
            i += 1;
        }
    }
    out.sort();
    out
}
fn norm_hex(h: &str) -> String {
    expand3(h).unwrap_or_else(|| h.to_lowercase()).to_lowercase()
}
/// hex tokens that are not inside an inline <...> color tag
fn hex_outside_tags(s: &str) -> Vec<(usize, usize)> {
    let tags = tag_specs(s);
    hex_tokens(s).into_iter().filter(|&(a, _)| !tags.iter().any(|&(x, y)| a >= x && a < y)).collect()
}
fn replace_hex(s: &str, f: &mut dyn FnMut(&str) -> String) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    for (a, b) in hex_tokens_short(s) {
        out.push_str(&s[last..a]);
        out.push_str(&f(&norm_hex(&s[a..b])));
        last = b;
    }
    out.push_str(&s[last..]);
    out
}
/// replace_hex for strings that are not color keys: only #rrggbb outside inline tags
fn replace_hex_plain(s: &str, f: &mut dyn FnMut(&str) -> String) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    for (a, b) in hex_outside_tags(s) {
        out.push_str(&s[last..a]);
        out.push_str(&f(&s[a..b].to_lowercase()));
        last = b;
    }
    out.push_str(&s[last..]);
    out
}

/// strings of a dict that are not color keys (templates with inline <fg,bg> color tags, ...)
fn other_strings(m: &mut Map<String, Value>) -> Vec<&mut String> {
    let mut out = Vec::new();
    for (k, v) in m.iter_mut() {
        if COLOR_KEYS.contains(&k.as_str()) || TEMPLATE_KEYS.contains(&k.as_str()) { continue }
        match v {
            Value::String(s) => out.push(s),
            Value::Array(a) => for x in a { if let Value::String(s) = x { out.push(s) } },
            _ => {}
        }
    }
    out
}
/// inline color tags like <#ffffff,p:name>: byte ranges of the spec between < and >
fn tag_specs(s: &str) -> Vec<(usize, usize)> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'<' {
            if let Some(j) = s[i + 1..].find('>') {
                let spec = &s[i + 1..i + 1 + j];
                if !spec.is_empty() && (spec.contains('#') || spec.contains("p:"))
                    && spec.bytes().all(|c| c.is_ascii_alphanumeric() || b"#_:,-".contains(&c))
                {
                    out.push((i + 1, i + 1 + j));
                    i += j + 1;
                }
            }
        }
        i += 1;
    }
    out
}
/// rebuild s with every tag spec passed through f(part, is_foreground, has_background)
fn map_tags(s: &str, f: &mut dyn FnMut(&str, bool, bool) -> String) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    for (a, b) in tag_specs(s) {
        out.push_str(&s[last..a]);
        let parts: Vec<&str> = s[a..b].split(',').collect();
        let mapped: Vec<String> = parts.iter().enumerate().map(|(i, p)| f(p, i == 0, parts.len() > 1)).collect();
        out.push_str(&mapped.join(","));
        last = b;
    }
    out.push_str(&s[last..]);
    out
}
/// "#abc" -> "#aabbcc"
fn expand3(s: &str) -> Option<String> {
    (s.len() == 4 && s.starts_with('#') && s[1..].bytes().all(|b| b.is_ascii_hexdigit()))
        .then(|| format!("#{0}{0}{1}{1}{2}{2}", &s[1..2], &s[2..3], &s[3..4]))
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
    let raw: BTreeMap<String, String> = theme
        .get("palette")
        .and_then(Value::as_object)
        .map(|o| o.iter().filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string()))).collect())
        .unwrap_or_default();
    // palette entries may alias each other ("closer": "p:os")
    let own: BTreeMap<String, String> = raw
        .keys()
        .filter_map(|k| {
            let mut v = raw[k].as_str();
            for _ in 0..8 {
                match v.strip_prefix("p:") {
                    Some(n) => v = raw.get(n)?.as_str(),
                    None => break,
                }
            }
            let v = if v.len() == 4 && v.starts_with('#') && v[1..].bytes().all(|b| b.is_ascii_hexdigit()) {
                format!("#{0}{0}{1}{1}{2}{2}", &v[1..2], &v[2..3], &v[3..4])
            } else {
                v.to_string()
            };
            (v.len() == 7 && v.starts_with('#')).then(|| (k.clone(), v))
        })
        .collect();
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
        for (_, s) in color_strings(m) {
            if let Some(h) = expand3(s.trim()) { *s = h }
        }
        for s in other_strings(m) {
            if s.contains("p:") {
                let mut out = String::new();
                let mut rest = s.as_str();
                while let Some(i) = rest.find("p:") {
                    let tail = &rest[i + 2..];
                    let end = tail.find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-')).unwrap_or(tail.len());
                    let prev_alpha = rest[..i].chars().last().is_some_and(|c| c.is_alphanumeric());
                    match own.get(&tail[..end]) {
                        Some(h) if !prev_alpha && end > 0 => { out.push_str(&rest[..i]); out.push_str(h) }
                        _ => out.push_str(&rest[..i + 2 + end]),
                    }
                    rest = &tail[end..];
                }
                out.push_str(rest);
                *s = out;
            }
            if s.contains('<') {
                *s = map_tags(s, &mut |p, _, _| expand3(p).unwrap_or_else(|| p.to_string()));
            }
        }
    });

    // 2. collect: usage counts and fg/bg groups per dict
    let mut use_count: BTreeMap<String, usize> = BTreeMap::new();
    let mut groups: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    visit_dicts(&mut theme, &mut |m| {
        let (mut fg, mut bg) = (Vec::new(), Vec::new());
        for (is_fg, s) in color_strings(m) {
            for (a, b) in hex_tokens_short(s) {
                let h = norm_hex(&s[a..b]);
                *use_count.entry(h.clone()).or_default() += 1;
                let list = if is_fg { &mut fg } else { &mut bg };
                if !list.contains(&h) { list.push(h) }
            }
        }
        for s in other_strings(m) {
            for (a, b) in hex_outside_tags(s) {
                let h = s[a..b].to_lowercase();
                *use_count.entry(h.clone()).or_default() += 1;
                groups.push((vec![h], bg.clone()));
            }
            for (a, b) in tag_specs(s) {
                let parts: Vec<&str> = s[a..b].split(',').collect();
                let (mut tfg, mut tbg) = (Vec::new(), Vec::new());
                for (i, p) in parts.iter().enumerate() {
                    if hex_tokens(p).len() == 1 && p.len() == 7 {
                        let h = p.to_lowercase();
                        *use_count.entry(h.clone()).or_default() += 1;
                        (if i == 0 { &mut tfg } else { &mut tbg }).push(h);
                    }
                }
                // a tag without its own background sits on the segment background
                if parts.len() == 1 || tbg.is_empty() { tbg = bg.clone() }
                if !tfg.is_empty() || !tbg.is_empty() { groups.push((tfg, tbg)) }
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

    let mut neutral_of: BTreeMap<(String, bool), String> = BTreeMap::new();
    let ladder: Vec<(f64, &str)> = NEUTRALS.iter().filter_map(|n| pal.get(*n).map(|h| (lch(h).0, *n))).collect();
    let chromatic: Vec<&String> = use_count.keys().filter(|h| lch(h).1 >= 0.04).collect();
    let term_bg = get("background", "#000000");
    for (fg, bg) in &groups {
        for (list, is_fg) in [(fg, true), (bg, false)] {
            for h in list {
                // a grey used as text on the bare terminal background must stay readable on it
                let unbound = is_fg && bg.is_empty();
                if lch(h).1 >= 0.04 || ladder.is_empty() || neutral_of.contains_key(&(h.clone(), unbound)) { continue }
                let l = lch(h).0;
                let by_l = |a: &&(f64, &str), b: &&(f64, &str)| (a.0 - l).abs().total_cmp(&(b.0 - l).abs());
                let role = if unbound {
                    let target = contrast(h, &term_bg).clamp(3.5, 4.5);
                    let ok: Vec<&(f64, &str)> = ladder.iter().filter(|r| contrast(&pal[r.1], &term_bg) >= target).collect();
                    match ok.iter().copied().min_by(by_l) {
                        Some(r) => r.1,
                        None => ladder.iter().max_by(|a, b| contrast(&pal[a.1], &term_bg).total_cmp(&contrast(&pal[b.1], &term_bg))).unwrap().1,
                    }
                } else {
                    ladder.iter().min_by(by_l).unwrap().1
                };
                neutral_of.insert((h.clone(), unbound), role.to_string());
            }
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
                if lch(h).1 < 0.04 || slot_of.contains_key(&(h.clone(), is_fg)) { continue }
                let (l, c, hue) = lch(h);
                let red = hue_dist(hue, 29.0) < 22.0 && c > 0.08;
                let (anchor, dh) = if red { (Anchor::Error, signed_hue(29.0, hue) * spread) } else { (Anchor::Primary, signed_hue(h0, hue) * spread) };
                ctx.slots.push(Slot { name: format!("pm{}", ctx.slots.len()), anchor, l, crel: (c / c0).clamp(0.2, 2.5), dh, dl: 0.0 });
                slot_of.insert((h.clone(), is_fg), ctx.slots.len() - 1);
            }
        }
    }
    let make_ref = |h: &String, is_fg: bool, unbound: bool| -> Ref {
        match neutral_of.get(&(h.clone(), is_fg && unbound)) {
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
                pairs.push(Pair { fg: make_ref(f, true, bg.is_empty()), bg: b.map(|b| make_ref(b, false, false)).unwrap_or(Ref::Bg), target });
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
    let lookup = |h: &str, is_fg: bool, unbound: bool| -> Option<String> {
        let h = h.to_string();
        if let Some(n) = neutral_of.get(&(h.clone(), is_fg && unbound)).or_else(|| neutral_of.get(&(h.clone(), false))).or_else(|| neutral_of.get(&(h.clone(), true))) {
            return Some(format!("p:{n}"));
        }
        let i = slot_of.get(&(h.clone(), is_fg)).or_else(|| slot_of.get(&(h, !is_fg)))?;
        Some(format!("p:{}", ctx.slots[*i].name))
    };
    visit_dicts(&mut theme, &mut |m| {
        let mut strings = color_strings(m);
        let unbound = !strings.iter().any(|(is_fg, s)| !*is_fg && !hex_tokens_short(s).is_empty());
        for (is_fg, s) in strings.iter_mut() {
            let is_fg = *is_fg;
            **s = replace_hex(s, &mut |h| lookup(h, is_fg, unbound).unwrap_or_else(|| h.to_string()));
        }
        drop(strings);
        for s in other_strings(m) {
            if s.contains('#') { *s = replace_hex_plain(s, &mut |h| lookup(h, true, unbound).unwrap_or_else(|| h.to_string())) }
            if !s.contains('<') { continue }
            *s = map_tags(s, &mut |p, first, has_bg| {
                if p.len() != 7 || hex_tokens(p).len() != 1 { return p.to_string() }
                lookup(&p.to_lowercase(), first, unbound && !has_bg).unwrap_or_else(|| p.to_string())
            });
        }
    });
    let mut palette = Map::new();
    for (k, v) in &pal { palette.insert(k.clone(), Value::String(v.clone())); }
    for s in &ctx.slots { palette.insert(s.name.clone(), Value::String(ctx.slot_hex(s))); }
    // own entries that are not plain hex (named ANSI colors, ...) stay, so refs to them keep working
    for (k, v) in &raw {
        if !own.contains_key(k) && !palette.contains_key(k) { palette.insert(k.clone(), Value::String(v.clone())); }
    }
    if let Value::Object(m) = &mut theme { m.insert("palette".into(), Value::Object(palette)); }

    if args.iter().any(|a| a == "--stats") {
        eprintln!(
            "colors={} neutral={} slots={} pairs={} weak_before={} weak_after={}",
            use_count.len(), neutral_of.len(), ctx.slots.len(), pairs_total, pairs_fixed, pairs_left
        );
    }
    println!("{}", serde_json::to_string_pretty(&theme).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_six_digit_hex_only() {
        assert_eq!(hex_tokens("a #aabbcc b #aabbccd c").len(), 1);
        assert_eq!(hex_tokens_short("{{ if x }}#fff{{ end }}").len(), 1);
        assert_eq!(norm_hex("#FfA"), "#ffffaa");
    }

    #[test]
    fn tag_specs_ignore_plain_tags() {
        let s = "<b>x</b> <#fff,p:surface>y</> <p:a>z</>";
        assert_eq!(tag_specs(s).len(), 2);
    }

    #[test]
    fn oklch_round_trips_srgb() {
        for h in ["#ff8000", "#3366ff", "#121318", "#ffffff", "#000000", "#b9c3ff"] {
            let (l, c, hue) = lch(h);
            let back = parse_hex(&to_hex(l, c, hue));
            let orig = parse_hex(h);
            assert!(back.iter().zip(orig).all(|(a, b)| (a - b).abs() < 0.01), "{h}");
        }
    }

    #[test]
    fn out_of_gamut_chroma_is_reduced() {
        let h = to_hex(0.9, 0.4, 200.0);
        assert_eq!(h.len(), 7);
        assert!(lch(&h).1 < 0.4);
    }

    #[test]
    fn contrast_extremes() {
        assert!((contrast("#000000", "#ffffff") - 21.0).abs() < 0.01);
        assert!((contrast("#777777", "#777777") - 1.0).abs() < 1e-9);
    }
}
