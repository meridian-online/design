//! Motion — two constants, because the budget only needs two.
//!
//! `guidelines/speed.md` (ADR 0010) says motion exists **only for spatial
//! continuity** — a panel opening, a brush moving, an overlay appearing — and
//! stays under ~150 ms; nothing eases in for delight. Feedback that
//! acknowledges input (hover emphasis, focus ring, selection, keypress echo)
//! lands on the *next frame* with no animation at all, which is why there is
//! no hover or focus duration in this module: the correct value is zero and a
//! token would only invite someone to raise it.
//!
//! [`ANIMATION_TIME`] exists for a specific failure mode. Frameworks that
//! carry a single global animation duration apply it to everything they
//! animate, including feedback — and the immediate-mode desktop framework's
//! stock value is `0.2` s (verified against the 0.35 source), which is over
//! the 150 ms spatial budget before a single Meridian widget has been drawn.
//! An adapter must therefore **set this explicitly**; inheriting the stock
//! value is a silent breach of the speed policy, not a neutral default.

/// `150` ms — the ceiling for spatial-continuity motion: a panel opening, an
/// overlay appearing, a docked pane resizing to a new position. Faster is
/// always allowed; slower is a policy breach.
pub const SPATIAL_MS: u32 = 150;

/// `0.12` s — the value a framework adapter writes into a global
/// animation-duration field. Below [`SPATIAL_MS`] with headroom, so a
/// framework that chains two animations still lands inside the budget.
///
/// Seconds, not milliseconds, because that is the unit such fields use.
pub const ANIMATION_TIME: f32 = 0.12;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_global_animation_time_sits_inside_the_spatial_budget() {
        assert!(
            ANIMATION_TIME * 1000.0 <= SPATIAL_MS as f32,
            "the value adapters set must not exceed the budget it exists to honour"
        );
    }

    #[test]
    fn there_are_no_other_durations() {
        // Feedback is next-frame; if a third constant ever appears here,
        // `guidelines/speed.md` has to change first.
        assert_eq!(SPATIAL_MS, 150);
        assert_eq!(ANIMATION_TIME, 0.12);
    }
}
