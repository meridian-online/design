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
//! **The chrome is square** (`guidelines/chrome.md`, ADR 0013). Every rung a
//! chrome surface reads is `0`: a rule and a bar do the work a corner used to
//! do. The rungs keep their names because consumers read them in their own
//! painters, so a consumer that reads a rung goes square with no edit, and a
//! rung is where a later look would put a value back. What stays round is a
//! mark that holds no icon and no word — a dot, a ring, a spinner, a
//! slider's thumb — and it reads [`FULL`].

/// `0` — square. Table cells, grid rules, anything that tiles.
pub const NONE: f32 = 0.0;

/// `0` — inline chips, badges, tags, swatches, key hints. Square, as the
/// rest of the chrome is; a key hint reads as a keycap by its foot
/// ([`crate::control::KEYCAP_FOOT_WIDTH`]), not by a corner.
pub const CHIP: f32 = 0.0;

/// `0` — buttons, inputs, selects, menu items, list rows. The desktop
/// theme's widget states read this value.
pub const CONTROL: f32 = 0.0;

/// `0` — panels, cards, popovers, modals, docked containers. A floating card
/// is told from the plane by its rule and its hard shadow
/// ([`crate::elevation`]), not by a corner.
pub const PANEL: f32 = 0.0;

/// A radius large enough to fully round anything on the control ladder — a
/// dot, a ring, a spinner, a slider's thumb. Consumers clamp to half the
/// shorter side. The one radius in the system that is not `0`.
pub const FULL: f32 = 9999.0;

/// Every rung a chrome surface reads, excluding [`FULL`] (which is a
/// sentinel for a round mark, not a rung of the chrome).
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

    /// The ruling, pinned rung by rung: every rung the chrome reads is
    /// square. A rung that comes back rounded is a change of look, and it
    /// has to come through this test and the decision record that says so.
    #[test]
    fn every_chrome_rung_is_square() {
        for (name, r) in [
            ("NONE", NONE),
            ("CHIP", CHIP),
            ("CONTROL", CONTROL),
            ("PANEL", PANEL),
        ] {
            assert_eq!(r, 0.0, "radius::{name} is {r}, and the chrome is square");
        }
        assert_eq!(RADII, [NONE, CHIP, CONTROL, PANEL]);
    }

    /// `FULL` is the one round radius, and it must still round anything the
    /// control ladder can draw: a dot or a slider's thumb clamped to half its
    /// shorter side has to reach a full circle at the tallest control.
    #[test]
    fn only_a_round_mark_is_round_and_it_rounds_the_tallest_control() {
        const { assert!(FULL >= crate::control::HEIGHT_LG / 2.0) };
        assert!(RADII.iter().all(|&r| r < FULL));
    }

    #[test]
    fn a_square_shape_keeps_a_square_outline_and_a_round_one_stays_concentric() {
        for r in RADII {
            for offset in [0.0, 1.0, 2.0] {
                assert_eq!(outer(r, offset), 0.0);
            }
        }
        assert_eq!(outer(FULL, 2.0), FULL + 2.0);
    }
}
