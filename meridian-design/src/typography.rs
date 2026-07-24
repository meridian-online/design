//! Type ramp (ADR 0005, gate resolved 2026-07-16). The Phase 2 render test —
//! side-by-side 11px `tnum` tables in a real Brightfield window — FAILED for
//! Geist (Inter read better at 11px; harness: brightfield branch
//! `design/0002-font-gate`, evidence `validation/font-gate-2026-07-16.png`).
//! The named fallback locked in. Sizes are logical pixels.

/// The UI sans. Embed the upstream rsms builds, never a stripped webfont
/// build.
pub const SANS_FAMILY: &str = "Inter";

/// The data/code mono — editor, table cells, tick labels. JetBrains Mono
/// ships coding ligatures ON by default (`calt`) — Meridian turns them off
/// on every data/editor surface via [`CALT_OFF`].
pub const MONO_FAMILY: &str = "JetBrains Mono";

/// Brand display face — marketing/web display surfaces ONLY, never inside
/// the apps (ADR 0005).
pub const DISPLAY_FAMILY: &str = "Anybody";

/// Dense-by-default UI base size (the Rill posture: 12px body).
pub const UI_SIZE: f32 = 12.0;

/// Chart axis/tick labels sit one step below UI body.
pub const CHART_LABEL_SIZE: f32 = 11.0;

/// An OpenType feature setting as a tag + value, so it maps to CSS
/// `font-feature-settings` and any OpenType-aware shaper alike.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontFeature {
    pub tag: [u8; 4],
    pub value: u16,
}

/// Tabular figures — mandatory wherever numbers align (tables, axis ticks),
/// applied at table scope, not globally.
pub const TNUM: FontFeature = FontFeature {
    tag: *b"tnum",
    value: 1,
};

/// Slashed zero — on wherever `TNUM` is on (0/O disambiguation at 11px).
pub const ZERO: FontFeature = FontFeature {
    tag: *b"zero",
    value: 1,
};

/// Ligatures off — REQUIRED alongside `MONO_FAMILY` (JetBrains Mono defaults
/// its coding ligatures on; `=>` must read as two glyphs in data surfaces).
pub const CALT_OFF: FontFeature = FontFeature {
    tag: *b"calt",
    value: 0,
};
