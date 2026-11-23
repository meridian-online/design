//! The step mark as data: the shape and ink of each run state.
//!
//! The step row, the thin step mark, the run status and the graph renderer all
//! read this one table, so a state looks the same wherever it is drawn. Shape
//! tells the states apart without colour; ink says it again with colour.
//!
//! This module states shape and ink only. The mark's size and stroke are
//! geometry and live in [`crate::control`]: [`STEP_MARK`](crate::control::STEP_MARK)
//! across, drawn with a [`STEP_MARK_STROKE`](crate::control::STEP_MARK_STROKE)
//! stroke, in a [`STEP_MARK_CELL`](crate::control::STEP_MARK_CELL) cell.
//!
//! The status inks (ok, failed, stale) are the one [`STATUS`] set in both
//! modes, as the emitter writes them. Not run and running follow the theme.

use crate::colour::Rgba;
use crate::semantic::semantic;
use crate::viz::STATUS;

/// A run state of a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepMark {
    NotRun,
    Running,
    Ok,
    Failed,
    Stale,
}

/// Every state, in the order the spec lists them.
pub const STEP_MARKS: [StepMark; 5] = [
    StepMark::NotRun,
    StepMark::Running,
    StepMark::Ok,
    StepMark::Failed,
    StepMark::Stale,
];

/// The outline a state is drawn with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkShape {
    /// An open ring.
    Ring,
    /// A ring whose right half is filled. The shape itself carries that half
    /// fill, so [`MarkGlyph::fill`] is `None` for it.
    HalfDisc,
    /// A solid disc.
    Disc,
    /// A solid diamond.
    Diamond,
    /// A ring with a dot at its centre.
    RingWithCentre,
}

/// How to draw one state: its shape, the ink of its outline, and the fill the
/// whole shape takes when it is solid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkGlyph {
    pub shape: MarkShape,
    pub ink: Rgba,
    /// `Some(ink)` for the disc and the diamond; `None` for every other shape.
    pub fill: Option<Rgba>,
}

/// The shape and ink of `mark` in the light or dark theme.
pub fn glyph(mark: StepMark, dark: bool) -> MarkGlyph {
    let s = semantic(dark);
    let (shape, ink) = match mark {
        StepMark::NotRun => (MarkShape::Ring, s.borders.control),
        StepMark::Running => (MarkShape::HalfDisc, s.borders.focus),
        StepMark::Ok => (MarkShape::Disc, STATUS.good),
        StepMark::Failed => (MarkShape::Diamond, STATUS.critical),
        StepMark::Stale => (MarkShape::RingWithCentre, STATUS.warning),
    };
    let fill = match shape {
        MarkShape::Disc | MarkShape::Diamond => Some(ink),
        _ => None,
    };
    MarkGlyph { shape, ink, fill }
}

/// The state word a screen reader announces.
pub fn word(mark: StepMark) -> &'static str {
    match mark {
        StepMark::NotRun => "not run",
        StepMark::Running => "running",
        StepMark::Ok => "ok",
        StepMark::Failed => "failed",
        StepMark::Stale => "stale",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::semantic;
    use crate::viz::STATUS;

    #[test]
    fn every_state_has_its_own_shape() {
        let shapes: Vec<MarkShape> = STEP_MARKS.iter().map(|m| glyph(*m, false).shape).collect();
        assert_eq!(
            shapes,
            [
                MarkShape::Ring,
                MarkShape::HalfDisc,
                MarkShape::Disc,
                MarkShape::Diamond,
                MarkShape::RingWithCentre
            ]
        );
    }

    #[test]
    fn each_ink_is_the_named_token_in_both_modes() {
        for dark in [false, true] {
            let s = semantic(dark);
            assert_eq!(glyph(StepMark::NotRun, dark).ink, s.borders.control);
            assert_eq!(glyph(StepMark::Running, dark).ink, s.borders.focus);
            assert_eq!(glyph(StepMark::Ok, dark).ink, STATUS.good);
            assert_eq!(glyph(StepMark::Failed, dark).ink, STATUS.critical);
            assert_eq!(glyph(StepMark::Stale, dark).ink, STATUS.warning);
        }
    }

    #[test]
    fn only_the_disc_and_the_diamond_are_filled() {
        for dark in [false, true] {
            for m in STEP_MARKS {
                let g = glyph(m, dark);
                match g.shape {
                    MarkShape::Disc | MarkShape::Diamond => assert_eq!(g.fill, Some(g.ink)),
                    _ => assert_eq!(g.fill, None),
                }
            }
        }
    }

    #[test]
    fn words_are_the_state_words() {
        let w: Vec<&str> = STEP_MARKS.iter().map(|m| word(*m)).collect();
        assert_eq!(w, ["not run", "running", "ok", "failed", "stale"]);
    }
}
