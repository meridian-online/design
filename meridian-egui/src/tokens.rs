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

use meridian_design::{control, radius, spacing};

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

    /// The row-height ladder (dense → comfortable).
    pub rows: [f32; 4],
    /// The control-height ladder (xs → lg).
    pub control_heights: [f32; 4],

    /// Inline chips, badges, tags, swatches.
    pub radius_chip: f32,
    /// The default control radius: buttons, inputs, selects, list rows.
    pub radius_control: f32,
    /// Containers: panels, cards, popovers, modals, docked regions.
    pub radius_panel: f32,

    /// Modal card width — the narrow rung (confirmations, single-field prompts).
    pub modal_width_narrow: f32,
    /// Modal card width — the default rung (forms, pickers, detail cards).
    pub modal_width_default: f32,
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

    rows: spacing::ROWS,
    control_heights: control::HEIGHTS,

    radius_chip: radius::CHIP,
    radius_control: radius::CONTROL,
    radius_panel: radius::PANEL,

    modal_width_narrow: spacing::MODAL_WIDTH_NARROW,
    modal_width_default: spacing::MODAL_WIDTH_DEFAULT,
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
        assert_eq!(TOKENS.rows, spacing::ROWS);
        assert_eq!(TOKENS.control_heights, control::HEIGHTS);
        assert_eq!(TOKENS.radius_control, radius::CONTROL);
        assert_eq!(TOKENS.radius_panel, radius::PANEL);
        assert_eq!(TOKENS.modal_width_default, spacing::MODAL_WIDTH_DEFAULT);
    }

    #[test]
    fn get_returns_the_shared_instance() {
        assert!(std::ptr::eq(Tokens::get(), &TOKENS));
    }
}
