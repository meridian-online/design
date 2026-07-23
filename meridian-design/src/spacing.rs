//! Spacing and the row-height ladder. Logical pixels as plain floats — every
//! target framework agrees on the unit (ADR 0003). Corner radii moved to
//! [`crate::radius`] when they landed, so a consumer asking "what radius?"
//! finds one module with one answer.
//!
//! Dense by default, no density modes (ADR 0005, `guidelines/density.md`): a
//! *ladder* of row heights replaces a compact/comfortable toggle — pick the
//! rung that fits the surface, never invent a height.
//!
//! **Single column, deliberately.** There is no `Density` parameter and no
//! multiplier here. `guidelines/density.md` is explicit that "a mode
//! multiplies every component spec by two and the default still has to be
//! chosen" — so the default is the only column. A surface that needs more
//! room picks a taller rung; it does not scale the ladder.

/// Hot dense lists (leaderboard-class rows) and the unified data table —
/// scanning surfaces where the row itself is the only pointer target.
pub const ROW_DENSE: f32 = 20.0;

/// Rows that carry their own inline pointer controls (menu option rows,
/// pickers) — the smallest rung that is a self-sufficient pointer target.
pub const ROW_GRID: f32 = 24.0;

/// Preview tables.
pub const ROW_PREVIEW: f32 = 36.0;

/// Comfortable lists (settings, navigation).
pub const ROW_COMFORTABLE: f32 = 46.0;

/// The row ladder in order, for consumers that need to snap an arbitrary
/// height onto a legal rung (see [`nearest_row`]).
pub const ROWS: [f32; 4] = [ROW_DENSE, ROW_GRID, ROW_PREVIEW, ROW_COMFORTABLE];

/// Snap an arbitrary height to the nearest legal rung. Exists so a consumer
/// porting a hard-coded height has a mechanical answer instead of inventing a
/// fifth row height. Exact ties round **up**: a row that is a hair too tall
/// costs density, a row that is a hair too short costs a pointer target.
pub fn nearest_row(px: f32) -> f32 {
    let mut best = ROWS[0];
    for r in ROWS {
        if (r - px).abs() <= (best - px).abs() {
            best = r;
        }
    }
    best
}

/// The base unit the ladder is built from.
///
/// **The grid is the half-step, not the base**, and calling this a "base-4
/// ladder" overstates it. Every step is a multiple of `BASE / 2` = 2 px;
/// most are also multiples of 4, but [`SPACE_1`] (2) and [`SPACE_3`] (6) are
/// not, and they earn their place — 2 px is the only gap that reads as
/// "touching but not welded" inside a 20 px row, and 6 px is the
/// icon-to-label gap the icon guideline asks for. So: **2 px grid, 4 px
/// rhythm.** Nothing off the 2 px grid is a legal gap, which is what
/// `ladder_ascends_and_stays_on_the_half_base_grid` enforces.
pub const BASE: f32 = 4.0;

/// `0` — flush. Named so "no gap" reads as a decision, not a missing value.
pub const SPACE_0: f32 = 0.0;
/// `2` — hairline gap: glyph-to-rule inside a dense control, chip internals.
pub const SPACE_1: f32 = 2.0;
/// `4` — the base unit: vertical padding inside a grid row.
pub const SPACE_2: f32 = 4.0;
/// `6` — icon-to-label, inline control gaps.
pub const SPACE_3: f32 = 6.0;
/// `8` — horizontal cell padding, control-to-control in a toolbar.
pub const SPACE_4: f32 = 8.0;
/// `12` — panel padding, gap between grouped controls.
pub const SPACE_5: f32 = 12.0;
/// `16` — gap between sections inside a panel; modal body padding.
pub const SPACE_6: f32 = 16.0;
/// `24` — gap between panels on the working plane.
pub const SPACE_7: f32 = 24.0;
/// `32` — page-level gutter; empty-state block spacing.
pub const SPACE_8: f32 = 32.0;
/// `48` — the largest sanctioned gap (empty-state breathing room).
pub const SPACE_9: f32 = 48.0;

/// The ladder in order, for iteration or snapping.
pub const SPACE: [f32; 10] = [
    SPACE_0, SPACE_1, SPACE_2, SPACE_3, SPACE_4, SPACE_5, SPACE_6, SPACE_7, SPACE_8, SPACE_9,
];

/// Snap an arbitrary gap to the nearest ladder step (same intent as
/// [`nearest_row`], ties round up).
pub fn nearest_space(px: f32) -> f32 {
    let mut best = SPACE[0];
    for s in SPACE {
        if (s - px).abs() <= (best - px).abs() {
            best = s;
        }
    }
    best
}

// ---------------------------------------------------------------------------
// Purposeful aliases — the named uses. Each is an alias of a ladder step,
// never a new value, so "is this on the ladder?" stays mechanically checkable.
// ---------------------------------------------------------------------------

/// Padding inside a panel or card body.
pub const PANEL_PADDING: f32 = SPACE_5;
/// Gap between sections within one panel.
pub const SECTION_GAP: f32 = SPACE_6;
/// Gap between two panels on the working plane.
pub const PANE_GAP: f32 = SPACE_7;
/// Gap between an icon and the label it belongs to (`guidelines/icons.md`).
pub const ICON_LABEL_GAP: f32 = SPACE_3;
/// Gap between sibling controls in a toolbar row.
pub const CONTROL_GAP: f32 = SPACE_4;
/// Padding inside a modal body.
pub const MODAL_PADDING: f32 = SPACE_6;

/// Modal card width — the narrow rung (confirmations, single-field prompts).
///
/// Two rungs exist because a desktop audit found one conceptual card
/// re-declared at four different widths that agreed with nothing. A modal is
/// either narrow or default; there is no third width.
pub const MODAL_WIDTH_NARROW: f32 = 480.0;
/// Modal card width — the default rung (forms, pickers, detail cards).
pub const MODAL_WIDTH_DEFAULT: f32 = 560.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ladder_ascends_and_stays_on_the_half_base_grid() {
        for w in SPACE.windows(2) {
            assert!(w[1] > w[0], "spacing ladder must ascend");
        }
        for s in SPACE {
            assert_eq!(s % (BASE / 2.0), 0.0, "{s} is off the 2px half-base grid");
        }
        // The claim is "2px grid, 4px rhythm" — not "base-4 ladder". Pin the
        // two steps that are deliberately off 4 so the looser wording stays
        // honest: if either moved onto 4, the half-step would no longer be
        // load-bearing and the grid should tighten to 4.
        let off_base: Vec<f32> = SPACE.iter().copied().filter(|s| s % BASE != 0.0).collect();
        assert_eq!(off_base, vec![SPACE_1, SPACE_3]);
    }

    #[test]
    fn aliases_are_ladder_steps_never_new_values() {
        for v in [
            PANEL_PADDING,
            SECTION_GAP,
            PANE_GAP,
            ICON_LABEL_GAP,
            CONTROL_GAP,
            MODAL_PADDING,
        ] {
            assert!(SPACE.contains(&v), "{v} is not a ladder step");
        }
    }

    #[test]
    fn snapping_lands_on_legal_values() {
        assert_eq!(nearest_row(23.0), ROW_GRID);
        assert_eq!(nearest_row(40.0), ROW_PREVIEW);
        assert_eq!(nearest_space(5.0), SPACE_3);
        assert_eq!(nearest_space(20.0), SPACE_7);
    }
}
