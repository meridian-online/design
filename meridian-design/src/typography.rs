//! Type ramp (ADR 0005). Families are GATED on the Phase 2 render test — an
//! 11px `tnum` table in Brightfield must pass an eyeball before these lock;
//! the named fallback is Inter + JetBrains Mono. Sizes are logical pixels.

/// The UI sans. Embed the upstream Geist builds, never the Google Fonts
/// build (it strips stylistic sets).
pub const SANS_FAMILY: &str = "Geist";

/// The data/code mono — editor, table cells, tick labels. Coding ligatures
/// are off by default upstream (opt-in ss11); keep them off.
pub const MONO_FAMILY: &str = "Geist Mono";

/// Brand display face — marketing/web display surfaces ONLY, never inside
/// the apps (ADR 0005).
pub const DISPLAY_FAMILY: &str = "Anybody";

/// Dense-by-default UI base size (the Rill posture: 12px body).
pub const UI_SIZE: f32 = 12.0;

/// Chart axis/tick labels sit one step below UI body.
pub const CHART_LABEL_SIZE: f32 = 11.0;

/// An OpenType feature setting, shaped as Parley's `FontFeature` (tag +
/// value) so it maps to CSS `font-feature-settings`, gpui, and Masonry alike.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontFeature {
    pub tag: [u8; 4],
    pub value: u16,
}

/// Tabular figures — mandatory wherever numbers align (tables, axis ticks),
/// applied at table scope, not globally.
pub const TNUM: FontFeature = FontFeature { tag: *b"tnum", value: 1 };

/// Slashed zero — on wherever `TNUM` is on (0/O disambiguation at 11px).
pub const ZERO: FontFeature = FontFeature { tag: *b"zero", value: 1 };
