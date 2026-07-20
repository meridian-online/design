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

/// The snapshot above pins the emitter against its own past output, so it can
/// only catch a change — never a *missing* one. It therefore did not notice
/// that the emitter restated the chrome anchors as literal hexes instead of
/// reading `chrome.rs`: editing a chrome token changed nothing downstream and
/// every test still passed.
///
/// This test closes that hole by deriving the expectation from `chrome.rs`
/// rather than from the snapshot. Change a chrome token and it fails until the
/// emitter propagates the change.
#[test]
fn theme_config_reads_chrome_tokens_rather_than_restating_them() {
    use meridian_design::chrome::{INK_DARK, INK_LIGHT};
    use meridian_design::emit::{theme_config, ThemeMode};

    for (mode, ink) in [(ThemeMode::Light, &INK_LIGHT), (ThemeMode::Dark, &INK_DARK)] {
        let json = theme_config(mode);
        for (key, expected) in [
            ("background", ink.page.hex()),
            ("title_bar.background", ink.page.hex()),
            ("status_bar.background", ink.page.hex()),
            ("tiles.background", ink.page.hex()),
            ("list.background", ink.surface.hex()),
            ("table.background", ink.surface.hex()),
            ("popover.background", ink.surface.hex()),
            ("tab.active.background", ink.surface.hex()),
            ("foreground", ink.ink_primary.hex()),
            ("caret", ink.ink_primary.hex()),
        ] {
            let needle = format!("\"{key}\": \"{expected}\"");
            assert!(
                json.contains(&needle),
                "{mode:?}: {key} does not track chrome.rs (expected {expected})"
            );
        }
        // On-solid ink is the light surface colour in both modes…
        assert!(json.contains(&format!(
            "\"primary.foreground\": \"{}\"",
            INK_LIGHT.surface.hex()
        )));
        // …and bright-solid ink is the light primary ink (ADR 0007).
        assert!(json.contains(&format!(
            "\"warning.foreground\": \"{}\"",
            INK_LIGHT.ink_primary.hex()
        )));
    }
}

/// The corner radii the desktop theme carries are the crate's radii, not two
/// numbers that happen to agree with them.
#[test]
fn theme_config_reads_the_radius_module() {
    use meridian_design::emit::{theme_config, ThemeMode};
    use meridian_design::radius;

    for mode in [ThemeMode::Light, ThemeMode::Dark] {
        let json = theme_config(mode);
        assert!(json.contains(&format!("\"radius\": {}", radius::CONTROL as u32)));
        assert!(json.contains(&format!("\"radius.lg\": {}", radius::PANEL as u32)));
    }
}

/// The CSS artefact carries the non-colour layer too, so the web can stop
/// re-declaring its own radius and spacing scales.
#[test]
fn tokens_css_carries_the_geometry_layer() {
    use meridian_design::{control, focus, motion, radius, spacing, typography};

    let css = meridian_design::emit::tokens_css();
    for (name, value) in [
        (
            "--m-radius-control",
            format!("{}px", radius::CONTROL as u32),
        ),
        ("--m-radius-panel", format!("{}px", radius::PANEL as u32)),
        ("--m-space-4", format!("{}px", spacing::SPACE_4 as u32)),
        ("--m-row-dense", format!("{}px", spacing::ROW_DENSE as u32)),
        ("--m-control-sm", format!("{}px", control::HEIGHT_SM as u32)),
        ("--m-icon-sm", format!("{}px", control::ICON_SM as u32)),
        (
            "--m-focus-ring-width",
            format!("{}px", focus::RING_WIDTH as u32),
        ),
        ("--m-motion-spatial", format!("{}ms", motion::SPATIAL_MS)),
        (
            "--m-motion-animation-time",
            format!("{}s", motion::ANIMATION_TIME),
        ),
        (
            "--m-font-size-ui",
            format!("{}px", typography::UI_SIZE as u32),
        ),
    ] {
        assert!(
            css.contains(&format!("{name}: {value};")),
            "{name} missing or wrong in tokens.css"
        );
    }
    // Shadows exist only above the working plane — two, never four.
    assert!(css.contains("--m-shadow-overlay:"));
    assert!(css.contains("--m-shadow-modal:"));
    assert!(!css.contains("--m-shadow-raised"));
    assert!(!css.contains("--m-shadow-flat"));
}

/// Every semantic slot reaches the web artefact, in both modes.
#[test]
fn tokens_css_carries_the_semantic_layer_in_both_modes() {
    let css = meridian_design::emit::tokens_css();
    let (light, dark) = css.split_once("\n.dark {").expect("two mode blocks");
    for block in [light, dark] {
        for name in [
            "--m-surface-app",
            "--m-surface-raised",
            "--m-surface-overlay",
            "--m-border-subtle",
            "--m-border-control",
            "--m-border-focus",
            "--m-text-primary",
            "--m-text-on-solid",
            "--m-accent-bg",
            "--m-accent-bg-hover",
            "--m-accent-fg-disabled",
            "--m-danger-border-focus",
            "--m-rows-selected-bg",
            "--m-scrollbar-thumb",
            "--m-popover-bg",
        ] {
            assert!(
                block.contains(&format!("{name}:")),
                "{name} missing from a mode block"
            );
        }
    }
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
