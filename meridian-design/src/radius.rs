//! Corner radii — one definition, every consumer.
//!
//! **A small corner returns** (`guidelines/chrome.md`, ADR 0013). A chip
//! rounds by [`CHIP`] (2), a control by [`CONTROL`] (3), a card by [`PANEL`]
//! (3). What stays square is a row and its fills, every bar, every hairline,
//! data marks, and a band that runs edge to edge, and it reads [`NONE`], never
//! a rounding rung: a square shape that reads a rung rounds the day the rung
//! changes. What stays round is a mark that holds no icon and no word — a
//! dot, a ring, a spinner, a slider's thumb — and it reads [`FULL`].

/// `0` — square. Rows and their hover, cursor and selection fills, bars,
/// hairlines, table cells, grid rules, bands that run edge to edge, anything
/// that tiles.
pub const NONE: f32 = 0.0;

/// `2` — the small things: a keycap, a status pill, a timeline bar, a tab's
/// close control. A keycap still reads as one by its foot
/// ([`crate::control::KEYCAP_FOOT_WIDTH`]) as well as by its corner.
pub const CHIP: f32 = 2.0;

/// `3` — buttons, text fields, segmented controls, and the focus ring that
/// follows them. The desktop theme's widget states read this value.
pub const CONTROL: f32 = 3.0;

/// `3` — popovers, menus, modal cards, banners, toasts, an inspector or step
/// list drawn as a card. A floating card is also told from the plane by its
/// rule and its hard shadow ([`crate::elevation`]).
pub const PANEL: f32 = 3.0;

/// A radius large enough to fully round anything on the control ladder — a
/// dot, a ring, a spinner, a slider's thumb. Consumers clamp to half the
/// shorter side.
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

    /// The ruling, pinned rung by rung, written as numbers rather than read
    /// back from the constants. A rung that moves is a change of look, and it
    /// has to come through this test and the decision record that says so.
    #[test]
    fn the_chrome_rungs_are_two_three_and_three() {
        assert_eq!(NONE, 0.0);
        assert_eq!(CHIP, 2.0);
        assert_eq!(CONTROL, 3.0);
        assert_eq!(PANEL, 3.0);
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
        for offset in [0.0, 1.0, 2.0] {
            assert_eq!(outer(NONE, offset), 0.0);
        }
        for r in [CHIP, CONTROL, PANEL] {
            for offset in [0.0, 1.0, 2.0] {
                assert_eq!(outer(r, offset), r + offset);
            }
        }
        assert_eq!(outer(FULL, 2.0), FULL + 2.0);
    }
}
