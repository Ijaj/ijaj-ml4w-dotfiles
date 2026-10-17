use std::process::Command;

const PALETTE: &str = "\
@define-color background #18120c;
@define-color primary #ffb59c;
@define-color tertiary #d6c68e;
@define-color error #ffb4ab;
@define-color surface #18120c;
@define-color surface_container #251e18;
@define-color surface_container_high #302922;
@define-color on_surface #ece0d8;
@define-color outline #a08d84;
";

const THEME: &str = r##"{
  "version": 4,
  "palette": { "white": "#f7f7f7", "alias": "p:white", "named": "red" },
  "blocks": [{ "type": "prompt", "segments": [
    { "type": "path", "foreground": "p:alias", "background": "#3366ff", "template": " <#fff>x</> <#ffcc00,#222222>y</> <p:white>z</> {{ .Path }} " },
    { "type": "git", "foreground": "#fff", "background": "#bd93f9",
      "background_templates": ["{{ if .Working.Changed }}#ff9248{{ end }}", "{{ if gt .Ahead 0 }}p:named{{ end }}"] },
    { "type": "text", "foreground": "#aaaaaa", "template": "t" },
    { "type": "text", "foreground": "red", "background": "transparent", "template": "n" }
  ]}]
}"##;

fn run(theme: &str) -> serde_json::Value {
    let dir = std::env::temp_dir().join(format!("promptmap-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let (t, p) = (dir.join("t.omp.json"), dir.join("colors.css"));
    std::fs::write(&t, theme).unwrap();
    std::fs::write(&p, PALETTE).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_promptmap")).arg(&t).arg("--palette").arg(&p).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    serde_json::from_slice(&out.stdout).unwrap()
}

#[test]
fn no_hex_left_outside_palette_and_all_refs_resolve() {
    let v = run(THEME);
    let palette = v["palette"].clone();
    let mut body = v.clone();
    body.as_object_mut().unwrap().remove("palette");
    let text = body.to_string();
    assert!(!text.contains('#'), "stray hex: {text}");
    let mut rest = text.as_str();
    while let Some(i) = rest.find("p:") {
        let tail = &rest[i + 2..];
        let end = tail.find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-')).unwrap_or(tail.len());
        assert!(palette.get(&tail[..end]).is_some(), "unresolved p:{}", &tail[..end]);
        rest = &tail[end..];
    }
}

#[test]
fn named_colors_and_aliases_survive() {
    let v = run(THEME);
    assert_eq!(v["blocks"][0]["segments"][3]["foreground"], "red");
    assert_eq!(v["palette"]["named"], "red");
    // alias of the theme's own palette resolved into an accent/neutral ref, not left dangling
    assert!(v["blocks"][0]["segments"][0]["foreground"].as_str().unwrap().starts_with("p:"));
}

#[test]
fn greys_become_palette_roles_and_accents_become_slots() {
    let v = run(THEME);
    let grey = v["blocks"][0]["segments"][2]["foreground"].as_str().unwrap();
    assert!(!grey.starts_with("p:pm"), "grey should map to a palette role, got {grey}");
    let accent = v["blocks"][0]["segments"][0]["background"].as_str().unwrap();
    assert!(accent.starts_with("p:pm"), "accent should be a derived slot, got {accent}");
}

#[test]
fn output_is_deterministic() {
    assert_eq!(run(THEME), run(THEME));
}
