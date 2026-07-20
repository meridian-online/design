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
//! Colour comes from the chrome layer, not this module: `chrome::OVERLAY_*`
//! carries `focus_ring`, which is Maritime — the accent is reserved for
//! interaction and focus is the purest case of it (`guidelines/identity.md`).
//!
//! Focus is feedback, so it lands next frame with no animation
//! (`guidelines/speed.md`): a ring that fades in is a ring that lies about
//! when the control became focused.

use crate::radius;

/// `2` — ring stroke width. One logical pixel disappears against a hairline
/// border on a dense surface; two reads as deliberate at any scale factor.
pub const RING_WIDTH: f32 = 2.0;

/// `1` — gap between the control's edge and the ring, for the *outset* ring
/// (the default: buttons, inputs, chips — anything with room around it).
pub const RING_OFFSET: f32 = 1.0;

/// `1` — inset of the ring from the control's edge, for the *inset* ring —
/// used where an outset ring would be clipped or would overlap a neighbour:
/// full-bleed list rows, table cells, the first control in a flush toolbar.
/// The inset ring is the exception; prefer the outset ring.
pub const RING_INSET: f32 = 1.0;

/// Radius for an outset ring around a control of radius `control_radius`, so
/// the ring and the control stay concentric instead of visibly diverging at
/// the corners. A square control keeps a square ring.
pub fn ring_radius(control_radius: f32) -> f32 {
    radius::outer(control_radius, RING_OFFSET)
}

/// Radius for an inset ring drawn inside a control of radius
/// `control_radius`. Never goes negative: a tight corner flattens to square
/// rather than inverting.
pub fn inset_ring_radius(control_radius: f32) -> f32 {
    (control_radius - RING_INSET).max(0.0)
}

/// Total space an outset ring needs outside the control's box — what a layout
/// must reserve so the ring is not clipped by a parent's bounds.
pub const RING_BLEED: f32 = RING_OFFSET + RING_WIDTH;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outset_ring_stays_concentric_and_square_stays_square() {
        assert_eq!(ring_radius(radius::CONTROL), radius::CONTROL + RING_OFFSET);
        assert_eq!(ring_radius(radius::NONE), 0.0);
    }

    #[test]
    fn inset_ring_never_inverts() {
        assert_eq!(inset_ring_radius(radius::NONE), 0.0);
        assert_eq!(
            inset_ring_radius(radius::CONTROL),
            radius::CONTROL - RING_INSET
        );
    }

    #[test]
    fn bleed_is_what_a_layout_must_reserve() {
        assert_eq!(RING_BLEED, 3.0);
    }
}
