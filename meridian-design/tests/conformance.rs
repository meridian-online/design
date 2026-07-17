//! Conformance: emitted artefacts are pinned exactly. If an intentional token
//! change lands, regenerate the snapshot in the same commit — a diff here in
//! CI means a consumer would have drifted.

#[test]
fn theme_config_matches_snapshots() {
    use meridian_design::emit::{theme_config, ThemeMode};
    assert_eq!(
        theme_config(ThemeMode::Light),
        include_str!("snapshots/theme-light.json"),
        "regenerate with: cargo run --example dump_theme light"
    );
    assert_eq!(
        theme_config(ThemeMode::Dark),
        include_str!("snapshots/theme-dark.json"),
        "regenerate with: cargo run --example dump_theme dark"
    );
}

#[test]
fn bundled_fonts_are_present_and_nonempty() {
    for bytes in meridian_design::fonts::ALL {
        assert!(bytes.len() > 100_000, "a bundled font file looks truncated");
    }
}

#[test]
fn tokens_css_matches_snapshot() {
    let expected = include_str!("snapshots/tokens.css");
    assert_eq!(
        meridian_design::emit::tokens_css(),
        expected,
        "emit::tokens_css() no longer matches tests/snapshots/tokens.css — \
         regenerate the snapshot if the change is intentional"
    );
}
