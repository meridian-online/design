//! Corner radii — one definition, every consumer.
//!
//! Before this module the system had two radii that existed *only* as literal
//! numbers inside the desktop theme emitter (`"radius": 6` and
//! `"radius.lg": 8`), invisible to any Rust consumer, while the web declared
//! its own `--radius` independently. Same intent, three declarations, no
//! shared source. [`CONTROL`] and [`PANEL`] are those two values, promoted:
//! the emitter now reads them, and `emit::tokens_css` publishes them so the
//! web can stop re-declaring.
//!
//! Radii are quiet (`guidelines/identity.md`): rounding is there to soften a
//! hit target, not to decorate. Nothing in the chrome is rounder than
//! [`PANEL`] except deliberate pills.

/// `0` — square. Table cells, grid rules, anything that tiles: rounding a
/// tiled edge shows the surface through the corner and reads as a mistake.
pub const NONE: f32 = 0.0;

/// `3` — inline chips, badges, tags, swatches. Half [`CONTROL`], so a chip
/// inside a control reads as nested rather than concentric-by-accident.
pub const CHIP: f32 = 3.0;

/// `6` — the default: buttons, inputs, selects, menu items, list rows with a
/// selection wash. This is the value the desktop theme's `radius` carries.
pub const CONTROL: f32 = 6.0;

/// `8` — panels, cards, popovers, modals, docked containers. This is the
/// value the desktop theme's `radius.lg` carries.
pub const PANEL: f32 = 8.0;

/// A radius large enough to fully round any control on the ladder — pills,
/// avatars, toggle tracks. Consumers clamp to half the shorter side.
pub const FULL: f32 = 9999.0;

/// The ladder in ascending order, excluding [`FULL`] (which is a sentinel,
/// not a step).
pub const RADII: [f32; 4] = [NONE, CHIP, CONTROL, PANEL];

/// Radius of a shape drawn *outside* another by `offset` px, so the two
/// curves stay concentric instead of visibly diverging at the corner. Used by
/// [`crate::focus::ring_radius`]; exposed here because any outline, glow, or
/// drop-shadow silhouette needs the same arithmetic.
pub fn outer(inner: f32, offset: f32) -> f32 {
    if inner <= 0.0 {
        // A square shape keeps a square outline: the eye reads a rounded ring
        // around a square control as a rendering bug.
        0.0
    } else {
        inner + offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ladder_ascends() {
        for w in RADII.windows(2) {
            assert!(w[1] > w[0], "radius ladder must ascend");
        }
    }

    #[test]
    fn chip_is_half_a_control_and_panel_is_the_only_thing_rounder() {
        assert_eq!(CHIP * 2.0, CONTROL);
        // `PANEL > CONTROL` was asserted here, but as a comparison of two
        // constants it could not fail at runtime — it documented, it did not
        // gate (and clippy said so). `ladder_ascends` already covers the
        // ordering over `RADII`; what is worth gating here is *position*:
        // PANEL is the top of the ladder and CONTROL is directly below it, so
        // a rounder step or a reshuffle forces a decision instead of sliding
        // in unnoticed.
        assert_eq!(*RADII.last().unwrap(), PANEL);
        assert_eq!(RADII[RADII.len() - 2], CONTROL);
    }

    #[test]
    fn concentric_outer_keeps_square_corners_square() {
        assert_eq!(outer(NONE, 2.0), 0.0);
        assert_eq!(outer(CONTROL, 1.0), 7.0);
    }
}
