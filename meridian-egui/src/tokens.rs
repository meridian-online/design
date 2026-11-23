//! The geometry a UI surface reaches for, aggregated behind `ui.tokens()`.
//!
//! `meridian-design` exposes its box model as modules of named constants —
//! [`spacing`](meridian_design::spacing), [`radius`](meridian_design::radius),
//! [`control`](meridian_design::control). That is the right shape for the source
//! of truth, but a call site that wants "the control gap" should not have to
//! remember which module it lives in or thread a bundle through its signature.
//! [`Tokens`] gathers the values a surface actually reaches for into one plain
//! struct, and [`MeridianUi::tokens`](crate::MeridianUi::tokens) hands back a
//! `&'static` reference to the single [`TOKENS`] instance — the values are
//! mode-independent constants, so one shared instance is all there is.
//!
//! It is a *convenience view*, not a second source: every field is initialised
//! straight from a `meridian-design` constant, never a literal, so the two
//! cannot drift.

use meridian_design::{control, radius, spacing, typography};

/// The Meridian box-model tokens a UI surface reaches for, in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tokens {
    /// The full spacing ladder (`SPACE_0`..`SPACE_9`), for snapping or iteration.
    pub space: [f32; 10],
    /// Padding inside a panel or card body.
    pub panel_padding: f32,
    /// Gap between sections within one panel.
    pub section_gap: f32,
    /// Gap between two panels on the working plane.
    pub pane_gap: f32,
    /// Gap between an icon and the label it belongs to.
    pub icon_label_gap: f32,
    /// Gap between sibling controls in a toolbar row.
    pub control_gap: f32,
    /// Padding inside a modal body.
    pub modal_padding: f32,
    /// Horizontal padding between a chip's edge and its contents — the status
    /// pill's capsule inset and the keycap chip's inner margin.
    pub chip_padding_x: f32,

    /// The row-height ladder (dense → comfortable).
    pub rows: [f32; 4],
    /// The control-height ladder (xs → lg).
    pub control_heights: [f32; 4],

    /// Inline chips, badges, tags, swatches.
    pub radius_chip: f32,
    /// The default control radius: buttons, text fields, selects. List rows are square and read `radius::NONE`.
    pub radius_control: f32,
    /// Containers: panels, cards, popovers, modals, docked regions.
    pub radius_panel: f32,

    /// Modal card width — the narrow rung (confirmations, single-field prompts).
    pub modal_width_narrow: f32,
    /// Modal card width — the default rung (forms, pickers, detail cards).
    pub modal_width_default: f32,
    /// Popover width.
    pub popover_width: f32,
    /// Height of a band: a section header or column-header strip.
    pub band_height: f32,

    /// Height of a keycap standing alone.
    pub keycap_height: f32,
    /// Height of a keycap inside a control.
    pub keycap_height_in_control: f32,
    /// Gap between a keycap inside a control and the control's edge.
    pub keycap_inset_in_control: f32,
    /// Width of the gutter lane at a row's leading edge.
    pub gutter_lane: f32,
    /// The cell a step mark sits in.
    pub step_mark_cell: f32,
    /// The drawn size of a step mark.
    pub step_mark: f32,
    /// Line weight of an outlined step mark.
    pub step_mark_stroke: f32,
    /// Width of the timeline column in a step row.
    pub timeline_width: f32,
    /// Height of a bar in the timeline column.
    pub timeline_bar_height: f32,
    /// Width of the duration column in a step row.
    pub duration_width: f32,
    /// Diameter of a status dot.
    pub dot: f32,

    /// Size of a section label.
    pub label_size: f32,
    /// Letter-spacing of a section label, in em.
    pub label_tracking_em: f32,
    /// Whether a section label is set in capitals.
    pub label_uppercase: bool,
}

/// The single shared token instance. Every field is a `meridian-design`
/// constant; nothing here is a fresh value.
///
/// A `static`, not a `const`, deliberately: `ui.tokens()` promises "the same
/// instance every time", and a `const` can be promoted to a distinct anonymous
/// static at each `&TOKENS`, so two callers could hold references that compare
/// unequal by address. One `static` has one address, which is what makes the
/// promise true.
pub static TOKENS: Tokens = Tokens {
    space: spacing::SPACE,
    panel_padding: spacing::PANEL_PADDING,
    section_gap: spacing::SECTION_GAP,
    pane_gap: spacing::PANE_GAP,
    icon_label_gap: spacing::ICON_LABEL_GAP,
    control_gap: spacing::CONTROL_GAP,
    modal_padding: spacing::MODAL_PADDING,
    chip_padding_x: spacing::CHIP_PADDING_X,

    rows: spacing::ROWS,
    control_heights: control::HEIGHTS,

    radius_chip: radius::CHIP,
    radius_control: radius::CONTROL,
    radius_panel: radius::PANEL,

    modal_width_narrow: spacing::MODAL_WIDTH_NARROW,
    modal_width_default: spacing::MODAL_WIDTH_DEFAULT,
    popover_width: spacing::POPOVER_WIDTH,
    band_height: spacing::BAND_HEIGHT,

    keycap_height: control::KEYCAP_HEIGHT,
    keycap_height_in_control: control::KEYCAP_HEIGHT_IN_CONTROL,
    keycap_inset_in_control: control::KEYCAP_INSET_IN_CONTROL,
    gutter_lane: control::GUTTER_LANE,
    step_mark_cell: control::STEP_MARK_CELL,
    step_mark: control::STEP_MARK,
    step_mark_stroke: control::STEP_MARK_STROKE,
    timeline_width: control::TIMELINE_WIDTH,
    timeline_bar_height: control::TIMELINE_BAR_HEIGHT,
    duration_width: control::DURATION_WIDTH,
    dot: control::DOT,

    label_size: typography::LABEL_SIZE,
    label_tracking_em: typography::LABEL_TRACKING_EM,
    label_uppercase: typography::LABEL_UPPERCASE,
};

impl Tokens {
    /// The shared token instance. Equivalent to `&TOKENS`; provided so callers
    /// that hold neither a `Ui` nor a `Context` can still reach the tokens.
    #[must_use]
    pub fn get() -> &'static Tokens {
        &TOKENS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every field mirrors its `meridian-design` source exactly — the view is
    /// not a place a value can be edited into disagreement with the tokens.
    #[test]
    fn tokens_mirror_the_design_constants() {
        assert_eq!(TOKENS.space, spacing::SPACE);
        assert_eq!(TOKENS.control_gap, spacing::CONTROL_GAP);
        assert_eq!(TOKENS.icon_label_gap, spacing::ICON_LABEL_GAP);
        assert_eq!(TOKENS.chip_padding_x, spacing::CHIP_PADDING_X);
        assert_eq!(TOKENS.rows, spacing::ROWS);
        assert_eq!(TOKENS.control_heights, control::HEIGHTS);
        assert_eq!(TOKENS.radius_control, radius::CONTROL);
        assert_eq!(TOKENS.radius_panel, radius::PANEL);
        assert_eq!(TOKENS.modal_width_default, spacing::MODAL_WIDTH_DEFAULT);
        assert_eq!(TOKENS.popover_width, spacing::POPOVER_WIDTH);
        assert_eq!(TOKENS.band_height, spacing::BAND_HEIGHT);
        assert_eq!(TOKENS.keycap_height, control::KEYCAP_HEIGHT);
        assert_eq!(
            TOKENS.keycap_height_in_control,
            control::KEYCAP_HEIGHT_IN_CONTROL
        );
        assert_eq!(
            TOKENS.keycap_inset_in_control,
            control::KEYCAP_INSET_IN_CONTROL
        );
        assert_eq!(TOKENS.gutter_lane, control::GUTTER_LANE);
        assert_eq!(TOKENS.step_mark_cell, control::STEP_MARK_CELL);
        assert_eq!(TOKENS.step_mark, control::STEP_MARK);
        assert_eq!(TOKENS.step_mark_stroke, control::STEP_MARK_STROKE);
        assert_eq!(TOKENS.timeline_width, control::TIMELINE_WIDTH);
        assert_eq!(TOKENS.timeline_bar_height, control::TIMELINE_BAR_HEIGHT);
        assert_eq!(TOKENS.duration_width, control::DURATION_WIDTH);
        assert_eq!(TOKENS.dot, control::DOT);
        assert_eq!(TOKENS.label_size, typography::LABEL_SIZE);
        assert_eq!(TOKENS.label_tracking_em, typography::LABEL_TRACKING_EM);
        assert_eq!(TOKENS.label_uppercase, typography::LABEL_UPPERCASE);
    }

    #[test]
    fn get_returns_the_shared_instance() {
        assert!(std::ptr::eq(Tokens::get(), &TOKENS));
    }
}
