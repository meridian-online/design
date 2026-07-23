//! Control heights, the icon-size ladder, and the row *binding*: the coherent
//! set of dimensions a row and everything inside it must share.
//!
//! The problem this solves is not "what height is a button" — it is that a
//! row height, the control inside it, that control's icon, its text size and
//! its padding were each chosen in a different file and agreed only by
//! coincidence. [`binding`] returns all of them together, derived from one
//! rung of the row ladder (`guidelines/density.md`), so a row and its
//! contents agree *by construction*.
//!
//! Text size never varies across rungs: 12px UI is the dense-by-default base
//! (ADR 0005) and a taller row buys air, not bigger type. Chart labels sit one
//! step below at 11px ([`crate::typography::CHART_LABEL_SIZE`]) and are not a
//! control size.
//!
//! # Pointer targets — stated, not discovered
//!
//! The dense rung is a 20px row holding an 18px control. WCAG 2.2 AA 2.5.8
//! (Target Size, Minimum) asks for 24×24 CSS px. **The dense rung does not
//! meet that, and at 20px pitch the spacing exception no longer covers it
//! either**: a 24px circle centred on one row's control reaches 12px past the
//! row centre, and the neighbouring row's control begins 11px away — the
//! circle overlaps the neighbouring target itself, not merely its circle.
//! (At the previous 22px pitch the circle cleared the neighbouring control by
//! 1px; the drop to 20 spends that margin.) The grid rung (24px) is the
//! smallest that conforms by itself.
//!
//! Meridian ships the dense rung anyway, as a deliberate and documented
//! trade, on these grounds:
//!
//! 1. The dense rung is for scanning surfaces where the full-width row is the
//!    only pointer target. It must **not** carry inline pointer controls —
//!    two icon buttons packed side by side in a dense row is exactly the case
//!    a reviewer should reject — and the ladder gives a conforming rung
//!    (grid, 24px) to move to, so the choice is visible, never accidental.
//! 2. Every action reachable by pointer in a dense row is also reachable by
//!    keyboard, and the app is keyboard-first. That is a real mitigation for
//!    users with limited dexterity — but it is a mitigation, not a 2.5.8
//!    exception, and this doc does not claim the keyboard path discharges
//!    the criterion.
//! 3. Density is a product value (`guidelines/density.md`): a surface that
//!    needs conforming pointer targets picks a taller rung; the ladder does
//!    not grow a mode.
//!
//! Use the dense rung for scanning surfaces; use the grid rung or taller
//! anywhere a row carries its own inline controls or must meet 2.5.8 on its
//! own.

use crate::spacing::{ROW_COMFORTABLE, ROW_DENSE, ROW_GRID, ROW_PREVIEW};
use crate::typography::UI_SIZE;

/// `18` — controls inside a dense row. Also, coincidentally, the stock
/// control height of the immediate-mode desktop framework (`interact_size.y`
/// is `18.0` in 0.35) — its stock *width* floor of `40.0` and its stock
/// item spacing are the parts an adapter must override, not the height.
pub const HEIGHT_XS: f32 = 18.0;
/// `20` — controls inside a data-grid row; the default toolbar control.
pub const HEIGHT_SM: f32 = 20.0;
/// `24` — controls on a preview/detail surface; the smallest height that
/// satisfies the 24px pointer-target minimum by itself.
pub const HEIGHT_MD: f32 = 24.0;
/// `30` — controls on comfortable surfaces: settings forms, modal footers,
/// primary actions.
pub const HEIGHT_LG: f32 = 30.0;

/// The control-height ladder in ascending order.
pub const HEIGHTS: [f32; 4] = [HEIGHT_XS, HEIGHT_SM, HEIGHT_MD, HEIGHT_LG];

/// `12` — inline glyph inside dense text (a sort caret, a NULL dot).
pub const ICON_XS: f32 = 12.0;
/// `14` — the dense-row and grid-row icon.
pub const ICON_SM: f32 = 14.0;
/// `16` — preview/detail rows, toolbar buttons with visible labels.
pub const ICON_MD: f32 = 16.0;
/// `20` — comfortable rows, navigation rails.
pub const ICON_LG: f32 = 20.0;
/// `24` — the icon set's native grid (2px stroke at 1:1, ADR 0009). Empty
/// states and feature affordances only; at any smaller size the same glyph is
/// scaled down, stroke width stays a live attribute.
pub const ICON_XL: f32 = 24.0;

/// The icon ladder in ascending order.
pub const ICONS: [f32; 5] = [ICON_XS, ICON_SM, ICON_MD, ICON_LG, ICON_XL];

/// Every dimension a row and its contents must agree on.
///
/// Invariant, asserted in tests: `control + 2 * pad_y == row`. A control
/// therefore never overflows its row and the vertical rhythm is a consequence
/// of the rung, not of per-call-site arithmetic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Binding {
    /// The row rung this binding belongs to (`spacing::ROW_*`).
    pub row: f32,
    /// Height of a control sitting inside that row.
    pub control: f32,
    /// Icon size inside that control.
    pub icon: f32,
    /// Text size inside that control (always the 12px UI base).
    pub text: f32,
    /// Horizontal padding between the row edge and its first content.
    pub pad_x: f32,
    /// Vertical padding above and below the control inside the row.
    pub pad_y: f32,
}

const DENSE: Binding = Binding {
    row: ROW_DENSE,
    control: HEIGHT_XS,
    icon: ICON_SM,
    text: UI_SIZE,
    pad_x: 6.0,
    pad_y: 1.0,
};

const GRID: Binding = Binding {
    row: ROW_GRID,
    control: HEIGHT_SM,
    icon: ICON_SM,
    text: UI_SIZE,
    pad_x: 8.0,
    pad_y: 2.0,
};

const PREVIEW: Binding = Binding {
    row: ROW_PREVIEW,
    control: HEIGHT_MD,
    icon: ICON_MD,
    text: UI_SIZE,
    pad_x: 8.0,
    pad_y: 6.0,
};

const COMFORTABLE: Binding = Binding {
    row: ROW_COMFORTABLE,
    control: HEIGHT_LG,
    icon: ICON_LG,
    text: UI_SIZE,
    pad_x: 12.0,
    pad_y: 8.0,
};

/// Every binding, in row order.
pub const BINDINGS: [Binding; 4] = [DENSE, GRID, PREVIEW, COMFORTABLE];

/// The coherent dimension set for a row rung.
///
/// `row` must be a rung of the ladder (`spacing::ROWS`); anything else snaps
/// to the grid rung rather than inventing a fifth geometry — an unknown height
/// is a bug in the caller, and silently honouring it is how a shadow ladder
/// starts.
pub const fn binding(row: f32) -> Binding {
    if row <= ROW_DENSE {
        DENSE
    } else if row <= ROW_GRID {
        GRID
    } else if row <= ROW_PREVIEW {
        PREVIEW
    } else if row <= ROW_COMFORTABLE {
        COMFORTABLE
    } else {
        GRID
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spacing::{ROWS, SPACE};

    #[test]
    fn control_plus_padding_fills_its_row_exactly() {
        for b in BINDINGS {
            assert_eq!(
                b.control + 2.0 * b.pad_y,
                b.row,
                "binding for row {} does not close",
                b.row
            );
        }
    }

    #[test]
    fn every_binding_value_is_on_a_published_ladder() {
        for b in BINDINGS {
            assert!(ROWS.contains(&b.row), "row {} is off the ladder", b.row);
            assert!(
                HEIGHTS.contains(&b.control),
                "control {} is off the ladder",
                b.control
            );
            assert!(ICONS.contains(&b.icon), "icon {} is off the ladder", b.icon);
            assert!(
                SPACE.contains(&b.pad_x),
                "pad_x {} is off the ladder",
                b.pad_x
            );
            // pad_y is not a free gap: it is the residual of two ladder
            // values, `(row - control) / 2`. What must sit on the gap ladder
            // is the *total* vertical slack — the dense rung's per-side 1px
            // is half of SPACE_1, not an invented step.
            assert!(
                SPACE.contains(&(2.0 * b.pad_y)),
                "total vertical slack {} is off the ladder",
                2.0 * b.pad_y
            );
            assert_eq!(b.text, UI_SIZE, "text size never varies by rung");
        }
    }

    #[test]
    fn icons_fit_inside_their_controls() {
        for b in BINDINGS {
            assert!(
                b.icon < b.control,
                "icon overflows its control at row {}",
                b.row
            );
        }
    }

    #[test]
    fn lookup_returns_the_rung_and_snaps_the_unknown() {
        assert_eq!(binding(ROW_DENSE), DENSE);
        assert_eq!(binding(ROW_GRID), GRID);
        assert_eq!(binding(ROW_PREVIEW), PREVIEW);
        assert_eq!(binding(ROW_COMFORTABLE), COMFORTABLE);
        assert_eq!(binding(999.0), GRID);
    }

    #[test]
    fn ladders_ascend() {
        for w in HEIGHTS.windows(2) {
            assert!(w[1] > w[0]);
        }
        for w in ICONS.windows(2) {
            assert!(w[1] > w[0]);
        }
    }
}
