//! Focus ring geometry.
//!
//! Focus is the one interaction state a keyboard-first tool cannot get from
//! its framework for free. The immediate-mode desktop framework (0.35) models
//! widget appearance as exactly five buckets — `noninteractive`, `inactive`,
//! `hovered`, `active`, `open` — and **none of them is focus**; its
//! `Widgets::style` resolver folds `response.has_focus()` into the *pressed*
//! bucket, so a keyboard-focused control and a mouse-held control are
//! indistinguishable by default. (Verified against the 0.35 source; the one
//! focus-related knob it does ship, `show_focused_widget`, defaults to off and
//! is a debug aid, not a design token.) A Meridian shell therefore paints its
//! own ring, and the ring's geometry has to live here so every surface paints
//! the same one.
//!
//! **The ring is a rule inside the control's edge** (`guidelines/chrome.md`,
//! ADR 0013): [`RING_WIDTH`] wide in the focus ink, square, drawn over the
//! control's own fill. Drawn inside, it cannot be clipped by a parent or
//! overlap a neighbour, so a layout reserves no room for it ([`RING_BLEED`]
//! is `0`) and there is one ring rather than an outset one and an inset
//! exception. On a solid fill the focus ink can be lost — on the accent's
//! own it measures under 3:1, and the gate holds it there — so the ring is
//! [`RING_WIDTH_ON_SOLID`] wide in the on-solid ink, [`RING_INSET`] in from
//! the edge. `tests/chrome_gate.rs` measures both inks against every fill
//! they can land on.
//!
//! Colour comes from the semantic layer, not this module:
//! `semantic::Borders::focus` is Maritime — the accent is reserved for
//! interaction and focus is the purest case of it (`guidelines/identity.md`)
//! — and `semantic::Text::on_solid` is the ink on a solid.
//!
//! Focus is feedback, so it lands next frame with no animation
//! (`guidelines/speed.md`): a ring that fades in is a ring that lies about
//! when the control became focused.

use crate::radius;

/// `2` — ring stroke width. One logical pixel disappears against a hairline
/// border on a dense surface; two reads as deliberate at any scale factor.
pub const RING_WIDTH: f32 = 2.0;

/// `0` — gap between the control's edge and the ring. The ring's outer edge
/// is the control's edge: it is drawn inside, over the control's fill.
pub const RING_OFFSET: f32 = 0.0;

/// `1` — inset of the ring on a solid fill from the control's edge. The
/// solid fill shows as a one-pixel line outside the ring, so the ring reads
/// as a mark on the control rather than as a second border.
pub const RING_INSET: f32 = 1.0;

/// `1` — ring stroke width on a solid fill, in the on-solid ink. One pixel
/// is enough there: the on-solid ink holds far more contrast against the
/// fill than the focus ink holds against a grey.
pub const RING_WIDTH_ON_SOLID: f32 = 1.0;

/// Radius for the ring around a control of radius `control_radius`, so the
/// ring and the control stay concentric instead of visibly diverging at the
/// corners. A square control keeps a square ring, which in the square look is
/// every control.
pub fn ring_radius(control_radius: f32) -> f32 {
    radius::outer(control_radius, RING_OFFSET)
}

/// Radius for the ring drawn [`RING_INSET`] inside a control of radius
/// `control_radius`. Never goes negative: a tight corner flattens to square
/// rather than inverting.
pub fn inset_ring_radius(control_radius: f32) -> f32 {
    (control_radius - RING_INSET).max(0.0)
}

/// `0` — space the ring needs outside the control's box, which a layout
/// must reserve so the ring is not clipped by a parent's bounds. The ring is
/// drawn inside the edge, so there is none.
pub const RING_BLEED: f32 = 0.0;

#[cfg(test)]
mod tests {
    use super::*;

    /// The ruling's geometry, pinned: 2 wide, flush on the edge, and no room
    /// outside the box. `RING_BLEED` is stated rather than derived, so this
    /// is the test that says what it must be.
    #[test]
    fn the_ring_is_a_two_pixel_rule_inside_the_edge() {
        assert_eq!(RING_WIDTH, 2.0);
        assert_eq!(RING_OFFSET, 0.0);
        assert_eq!(RING_BLEED, 0.0);
    }

    /// On a solid fill the ring is one pixel, one pixel in: it must sit
    /// wholly inside the control, which a ring wider than its inset could not
    /// do on a control narrower than both together.
    #[test]
    fn the_ring_on_a_solid_is_one_pixel_one_pixel_in() {
        assert_eq!(RING_WIDTH_ON_SOLID, 1.0);
        assert_eq!(RING_INSET, 1.0);
        const { assert!(RING_INSET + RING_WIDTH_ON_SOLID <= crate::control::HEIGHT_XS / 2.0) };
    }

    #[test]
    fn a_square_control_keeps_a_square_ring_and_a_round_one_a_round_ring() {
        for r in radius::RADII {
            assert_eq!(ring_radius(r), 0.0);
            assert_eq!(inset_ring_radius(r), 0.0);
        }
        assert_eq!(ring_radius(radius::FULL), radius::FULL + RING_OFFSET);
        assert_eq!(
            inset_ring_radius(radius::FULL),
            radius::FULL - RING_INSET
        );
    }
}
