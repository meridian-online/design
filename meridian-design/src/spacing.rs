//! Spacing, radii, and the row-height ladder. Logical pixels as plain floats
//! — GPUI's `px()` and Masonry's `Length` agree on the unit (ADR 0003).
//!
//! Dense by default, no density modes (ADR 0005): a *ladder* of row heights
//! replaces a compact/comfortable toggle — pick the rung that fits the
//! surface, never invent a height.

/// Hot dense lists (leaderboard-class rows).
pub const ROW_DENSE: f32 = 22.0;

/// Virtualised data grids.
pub const ROW_GRID: f32 = 24.0;

/// Preview tables.
pub const ROW_PREVIEW: f32 = 36.0;

/// Comfortable lists (settings, navigation).
pub const ROW_COMFORTABLE: f32 = 46.0;

// Spacing steps and corner radii land in Phase 1, as a small named set
// (`Base04`-style) rather than ad-hoc paddings — the Zed pattern.
