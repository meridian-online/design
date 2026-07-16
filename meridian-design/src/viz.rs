//! Chart palettes (ADR 0006, values approved 2026-07-16). Designed in OKLCH
//! against the Meridian surfaces and gated by `tests/palette_gate.rs` — the
//! Rust port of the palette validator. Regenerate via `validation/`, never
//! hand-edit.
//!
//! Portability contract: these are renderer DEFAULTS (zero spec surface).
//! Specs that must pin colours portably carry explicit `colorDomain`/
//! `colorRange` literals — never a scheme name vanilla Mosaic would reject.

use crate::colour::Rgba;

/// The "Harbour" categorical order: blue, gold, teal, red, violet, orange,
/// plum, green. The ORDER is the colourblind-safety mechanism (chosen by
/// exhaustive search for maximum adjacent CVD dE jointly across modes) and is
/// therefore data, never cosmetic. Assign in fixed order; fold a 9th series
/// into "Other". The first FOUR slots validate all-pairs (scatter,
/// choropleth); past four, fold, facet, or label directly. Light mode slots
/// 2/3/6 (gold/teal/orange) sit below 3:1 on the surface — the relief rule
/// applies (visible direct labels or table view).
pub const CATEGORICAL_LIGHT: [Rgba; 8] = [
    Rgba::from_u8(0x00, 0x83, 0xc4, 0xff), // #0083c4
    Rgba::from_u8(0xcc, 0xa1, 0x24, 0xff), // #cca124
    Rgba::from_u8(0x31, 0xaa, 0x8c, 0xff), // #31aa8c
    Rgba::from_u8(0xc1, 0x3c, 0x3b, 0xff), // #c13c3b
    Rgba::from_u8(0x6a, 0x55, 0xb8, 0xff), // #6a55b8
    Rgba::from_u8(0xdd, 0x7b, 0x2b, 0xff), // #dd7b2b
    Rgba::from_u8(0xbf, 0x62, 0xad, 0xff), // #bf62ad
    Rgba::from_u8(0x47, 0x94, 0x4c, 0xff), // #47944c
];

pub const CATEGORICAL_DARK: [Rgba; 8] = [
    Rgba::from_u8(0x0b, 0x89, 0xc8, 0xff), // #0b89c8
    Rgba::from_u8(0xb6, 0x8f, 0x1b, 0xff), // #b68f1b
    Rgba::from_u8(0x31, 0xaa, 0x8c, 0xff), // #31aa8c
    Rgba::from_u8(0xc7, 0x4b, 0x47, 0xff), // #c74b47
    Rgba::from_u8(0x7b, 0x69, 0xc6, 0xff), // #7b69c6
    Rgba::from_u8(0xb4, 0x5c, 0x03, 0xff), // #b45c03
    Rgba::from_u8(0xb5, 0x5f, 0xa4, 0xff), // #b55fa4
    Rgba::from_u8(0x47, 0x94, 0x4c, 0xff), // #47944c
];

/// Meridian blue-240 sequential ramp, steps 100..=700 — an OPT-IN named
/// scheme ("meridian"); the DEFAULT sequential scheme remains viridis
/// (Hugh's call, 2026-07-16). Ordinal use: light no lighter than index 3
/// (step 250), dark no darker than index 10 (step 600).
pub const SEQUENTIAL_MERIDIAN: [Rgba; 13] = [
    Rgba::from_u8(0xc6, 0xe4, 0xfb, 0xff), // #c6e4fb
    Rgba::from_u8(0xa6, 0xd7, 0xfa, 0xff), // #a6d7fa
    Rgba::from_u8(0x87, 0xc8, 0xf6, 0xff), // #87c8f6
    Rgba::from_u8(0x69, 0xba, 0xf0, 0xff), // #69baf0
    Rgba::from_u8(0x4d, 0xaa, 0xe6, 0xff), // #4daae6
    Rgba::from_u8(0x35, 0x9b, 0xd9, 0xff), // #359bd9
    Rgba::from_u8(0x23, 0x8c, 0xc7, 0xff), // #238cc7
    Rgba::from_u8(0x1d, 0x7c, 0xb2, 0xff), // #1d7cb2
    Rgba::from_u8(0x21, 0x6d, 0x9b, 0xff), // #216d9b
    Rgba::from_u8(0x28, 0x5e, 0x81, 0xff), // #285e81
    Rgba::from_u8(0x2d, 0x4f, 0x67, 0xff), // #2d4f67
    Rgba::from_u8(0x27, 0x41, 0x54, 0xff), // #274154
    Rgba::from_u8(0x1b, 0x35, 0x46, 0xff), // #1b3546
];

/// Diverging pair: Maritime blue <-> brick red (Hugh's call, 2026-07-16),
/// neutral warm-gray midpoint so "nothing" reads as nothing.
/// Blue arm runs pole -> lightest; compose full ramp as
/// blue arm + mid (per mode) + red arm.
pub const DIVERGING_BLUE_ARM: [Rgba; 5] = [
    Rgba::from_u8(0x00, 0x53, 0x89, 0xff), // #005389
    Rgba::from_u8(0x00, 0x70, 0xa9, 0xff), // #0070a9
    Rgba::from_u8(0x47, 0x8e, 0xbc, 0xff), // #478ebc
    Rgba::from_u8(0x79, 0xab, 0xcf, 0xff), // #79abcf
    Rgba::from_u8(0xa8, 0xc9, 0xe2, 0xff), // #a8c9e2
];

/// Red arm runs lightest -> pole.
pub const DIVERGING_RED_ARM: [Rgba; 5] = [
    Rgba::from_u8(0xe3, 0xb8, 0xb4, 0xff), // #e3b8b4
    Rgba::from_u8(0xd0, 0x93, 0x8d, 0xff), // #d0938d
    Rgba::from_u8(0xbb, 0x6d, 0x67, 0xff), // #bb6d67
    Rgba::from_u8(0xa5, 0x47, 0x43, 0xff), // #a54743
    Rgba::from_u8(0x8d, 0x1a, 0x1e, 0xff), // #8d1a1e
];

/// Diverging midpoints (warm gray step 2 light / step 4 dark).
pub const DIVERGING_MID_LIGHT: Rgba = Rgba::from_u8(0xf4, 0xf3, 0xf2, 0xff);
pub const DIVERGING_MID_DARK: Rgba = Rgba::from_u8(0x2a, 0x29, 0x28, 0xff);

/// Status colours — reserved (never reused as series), fixed across modes,
/// always paired with icon + label, never colour alone. Warning takes dark
/// text on badges (bright-scale caveat).
pub struct Status {
    pub good: Rgba,
    pub warning: Rgba,
    pub serious: Rgba,
    pub critical: Rgba,
}
pub const STATUS: Status = Status {
    good: Rgba::from_u8(0x21, 0x8a, 0x45, 0xff),     // #218a45
    warning: Rgba::from_u8(0xda, 0x95, 0x0b, 0xff),  // #da950b
    serious: Rgba::from_u8(0xdb, 0x70, 0x3b, 0xff),  // #db703b
    critical: Rgba::from_u8(0xc9, 0x30, 0x2d, 0xff), // #c9302d
};

/// NULL ink — a warm-gray cell for NULL values that cannot impersonate any
/// scheme value (closes the NULL-reads-as-high bug). Deliberately BELOW the
/// series chroma floor so it never reads as data.
pub const NULL_INK_LIGHT: Rgba = Rgba::from_u8(0xdc, 0xda, 0xd8, 0xff);
pub const NULL_INK_DARK: Rgba = Rgba::from_u8(0x32, 0x30, 0x2f, 0xff);

/// Default single-mark colour = categorical slot 1 (replaces Tableau10
/// steel blue in Brightfield, Phase 4 PR B).
pub const MARK_DEFAULT_LIGHT: Rgba = CATEGORICAL_LIGHT[0];
pub const MARK_DEFAULT_DARK: Rgba = CATEGORICAL_DARK[0];
