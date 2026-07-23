//! Input-widget chrome — the canonical geometry and ink for the dashboard's
//! composition-level input widgets (slider, menu, radio), stated once.
//!
//! Until now these values lived as twin constant sets in the consumer, "kept
//! in sync by comment" across its two render paths — a headless Vello scene
//! (`brightfield-render/src/scene.rs`, `asset_scene.rs`) and a live windowed
//! shim (`brightfield-ui/src/slider_element.rs`, `menu_element.rs`), with the
//! layout half in `brightfield-spec/src/layout.rs`. A comment is not a
//! contract: this module is the single definition point, so the resting
//! raster and the live widget can only agree *by construction*. The consumer
//! repoints both twins here when its shell cutover lands; until then the
//! values below are pinned equal to what both twins ship today.
//!
//! Geometry is logical pixels as plain `f32` (ADR 0003); convert at the
//! call site if a scene wants `f64`. Row heights are rungs of the published
//! ladder ([`crate::spacing`]) and the label size is the 12px UI base
//! ([`crate::typography::UI_SIZE`]) — both pinned by tests below, so a ladder
//! change is a visible decision here, never silent drift.

use crate::chrome::{INK_DARK, INK_LIGHT};
use crate::colour::Rgba;
use crate::scales::{GRAY_DARK, GRAY_LIGHT};
use crate::spacing::{ROW_DENSE, ROW_GRID};
use crate::typography::UI_SIZE;

// ---------------------------------------------------------------------------
// Geometry
// ---------------------------------------------------------------------------

/// Default width of a composition-level input widget (slider / menu), the
/// box the layout reserves beside the plots.
pub const INPUT_WIDTH: f32 = 200.0;

/// Default height of a composition-level input widget's resting box.
pub const INPUT_HEIGHT: f32 = 32.0;

/// Slider thumb radius. The track is inset by this on each end so the thumb
/// never leaves the widget bounds at min/max.
pub const SLIDER_THUMB_RADIUS: f32 = 7.0;

/// Slider track bar thickness (the rounded horizontal bar the thumb rides).
pub const SLIDER_TRACK_THICKNESS: f32 = 4.0;

/// Label text size inside every input widget — the menu's value, the open
/// list's options, a radio row's label. Identical to the UI base size: widget
/// text is UI text, not chart annotation (chart labels sit at 11px).
pub const WIDGET_TEXT_SIZE: f32 = UI_SIZE;

/// Vertical offset from a row's centre down to the label BASELINE at
/// [`WIDGET_TEXT_SIZE`] (approximately half the cap height). Only a renderer
/// that places raw glyph baselines needs it (a retained-mode host centres
/// text boxes instead); stated here so a second baseline-placing surface
/// (e.g. an asset-graph card label) nudges by the same amount.
pub const WIDGET_BASELINE_NUDGE: f32 = 4.0;

/// Height of one option row in an open menu's popup list — the grid rung of
/// the row ladder (24px, the smallest rung that is a self-sufficient pointer
/// target: the popup is pointer-first chrome).
pub const MENU_OPTION_ROW_HEIGHT: f32 = ROW_GRID;

/// Height of one option row of a `radio`-presented input — the dense rung
/// (20px): radio rows rest in the composition like a dense list, and every
/// row is also keyboard-reachable.
pub const RADIO_ROW_HEIGHT: f32 = ROW_DENSE;

/// Vertical chrome padding a radio-presented input adds around its rows:
/// `height = RADIO_ROW_HEIGHT * options + RADIO_CHROME_PAD`, half above the
/// first row and half below the last.
pub const RADIO_CHROME_PAD: f32 = 10.0;

// ---------------------------------------------------------------------------
// Ink
// ---------------------------------------------------------------------------

/// The ink roles an input widget wears. One struct per theme, mirroring
/// [`crate::chrome::InkTokens`]; every field is a reference to an existing
/// chrome/scale token — this module maps roles, it never invents a colour.
pub struct WidgetInk {
    /// Widget body fill — the chart surface, so a resting widget sits on the
    /// canvas as quietly as a plot does.
    pub fill: Rgba,
    /// Resting border / outline hairline — warm gray step 5.
    pub border: Rgba,
    /// Label text — primary ink.
    pub label: Rgba,
    /// Secondary affordances: the menu chevron, a radio ring outline — muted
    /// ink, present but not competing with the value.
    pub affordance: Rgba,
    /// Active/selected marks: the picked option's check, the selected radio
    /// dot — the focus ink (Maritime), the one accent input chrome may wear.
    pub active: Rgba,
    /// Hover wash for an open menu's option rows — warm gray step 2.
    /// Live-host-side only: a resting raster never shows hover, so the
    /// headless twin must not paint it.
    pub hover: Rgba,
    /// Slider track bar — deliberately the same gray as [`Self::border`]
    /// (one hairline gray per widget family), pinned equal by test.
    pub slider_track: Rgba,
    /// Slider thumb — deliberately the same ink as [`Self::active`] (the
    /// thumb IS the widget's active mark), pinned equal by test.
    pub slider_thumb: Rgba,
}

/// Light-theme widget ink — the set both consumer twins ship today.
pub const WIDGET_INK_LIGHT: WidgetInk = WidgetInk {
    fill: INK_LIGHT.surface,
    border: GRAY_LIGHT[4],
    label: INK_LIGHT.ink_primary,
    affordance: INK_LIGHT.ink_muted,
    active: INK_LIGHT.focus,
    hover: GRAY_LIGHT[1],
    slider_track: GRAY_LIGHT[4],
    slider_thumb: INK_LIGHT.focus,
};

/// Dark-theme widget ink — the same role mapping over the dark anchors.
/// No consumer reads it yet (the desktop shell is light-only today); it
/// exists so the dark theme arrives as a data change, not a redesign.
pub const WIDGET_INK_DARK: WidgetInk = WidgetInk {
    fill: INK_DARK.surface,
    border: GRAY_DARK[4],
    label: INK_DARK.ink_primary,
    affordance: INK_DARK.ink_muted,
    active: INK_DARK.focus,
    hover: GRAY_DARK[1],
    slider_track: GRAY_DARK[4],
    slider_thumb: INK_DARK.focus,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spacing::ROWS;

    /// The geometry both consumer twins ship today, pinned numerically: a
    /// change here is a deliberate re-baseline of the consumer's rendered
    /// output, never an accident of editing a shared upstream constant.
    #[test]
    fn geometry_matches_the_shipped_twins() {
        assert_eq!(INPUT_WIDTH, 200.0);
        assert_eq!(INPUT_HEIGHT, 32.0);
        assert_eq!(SLIDER_THUMB_RADIUS, 7.0);
        assert_eq!(SLIDER_TRACK_THICKNESS, 4.0);
        assert_eq!(WIDGET_TEXT_SIZE, 12.0);
        assert_eq!(WIDGET_BASELINE_NUDGE, 4.0);
        assert_eq!(MENU_OPTION_ROW_HEIGHT, 24.0);
        assert_eq!(RADIO_ROW_HEIGHT, 20.0);
        assert_eq!(RADIO_CHROME_PAD, 10.0);
    }

    /// Widget rows are rungs of the published ladder and widget text is the
    /// UI base size — the module derives from the system, so a ladder or
    /// type-ramp change surfaces here as a failing pin, not as silent drift.
    #[test]
    fn rows_and_text_come_from_the_system() {
        assert!(ROWS.contains(&MENU_OPTION_ROW_HEIGHT));
        assert!(ROWS.contains(&RADIO_ROW_HEIGHT));
        assert_eq!(WIDGET_TEXT_SIZE, UI_SIZE);
    }

    /// The slider wears the widget family's own ink: its track is the border
    /// gray and its thumb is the active ink, in both themes. Stated as
    /// separate fields (the twins name them separately) but pinned equal so
    /// the pairs cannot drift apart.
    #[test]
    fn slider_ink_is_the_widget_ink() {
        for ink in [&WIDGET_INK_LIGHT, &WIDGET_INK_DARK] {
            assert_eq!(ink.slider_track, ink.border);
            assert_eq!(ink.slider_thumb, ink.active);
        }
    }

    /// The light set reads the chrome/scale tokens, not restated literals —
    /// editing a chrome token must move widget ink (the silent-no-op class
    /// an emitter once shipped).
    #[test]
    fn light_ink_reads_the_tokens() {
        assert_eq!(WIDGET_INK_LIGHT.fill, INK_LIGHT.surface);
        assert_eq!(WIDGET_INK_LIGHT.border, GRAY_LIGHT[4]);
        assert_eq!(WIDGET_INK_LIGHT.label, INK_LIGHT.ink_primary);
        assert_eq!(WIDGET_INK_LIGHT.affordance, INK_LIGHT.ink_muted);
        assert_eq!(WIDGET_INK_LIGHT.active, INK_LIGHT.focus);
        assert_eq!(WIDGET_INK_LIGHT.hover, GRAY_LIGHT[1]);
    }

    /// The thumb fits the resting box: a full thumb diameter (plus a pixel
    /// of breathing room each side) never exceeds the default input height.
    /// Constant inputs, so the invariant is checked at compile time.
    #[test]
    fn thumb_fits_the_input_box() {
        const { assert!(2.0 * SLIDER_THUMB_RADIUS + 2.0 <= INPUT_HEIGHT) }
        const { assert!(SLIDER_TRACK_THICKNESS < 2.0 * SLIDER_THUMB_RADIUS) }
    }
}
