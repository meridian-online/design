//! Conformance: emitted artefacts are pinned exactly. If an intentional token
//! change lands, regenerate the snapshot in the same commit — a diff here in
//! CI means a consumer would have drifted.

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
