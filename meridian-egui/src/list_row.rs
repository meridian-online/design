//! The one list-row treatment.
//!
//! Every "row of things you can hover, select, and click" — picker matches,
//! navigation lists, palette results — draws through [`list_row`], which
//! paints hover/selected/pressed state **from the semantic row tokens only**.
//! The function takes no colour and no height in pixels: the height is a rung
//! of the row ladder ([`RowHeight`]) and the state colours come from
//! [`meridian_design::semantic`], so a hand-rolled row background cannot be
//! expressed through this API at all. That is deliberate — the audit behind
//! ADR 0011 found rows re-inventing their own accent at the call site, and the
//! durable fix is an API with no colour parameter to misuse.
//!
//! Hover-revealed affordances are **visibility-based**: the content closure
//! receives the row's [`RowState`] and simply does not add the affordance
//! widgets when the row is not hovered. Nothing is drawn at zero opacity —
//! a transparent widget still sits in the hit-test order and steals hover from
//! the row underneath it, which is the failure mode this closure shape exists
//! to rule out. The *row* is the hover sensor (its full rect is interacted
//! before content is laid out), so affordances appearing on hover cannot
//! flicker: the pointer is over the row whether or not the affordance exists
//! yet.

use egui::{Sense, StrokeKind, UiBuilder};
use meridian_design::{semantic, Role};

use crate::theme::to_color32;
use crate::tokens::Tokens;
use crate::MeridianUi;

/// A rung of the row-height ladder. There is no pixel parameter — a surface
/// picks a rung, never invents a height (the density stance in the token
/// crate's spacing module).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowHeight {
    /// Hot dense lists (leaderboard-class rows).
    Dense,
    /// Virtualised data grids — the default for picker matches.
    Grid,
    /// Preview tables.
    Preview,
    /// Comfortable lists (settings, navigation).
    Comfortable,
}

impl RowHeight {
    /// The rung's height in logical pixels, resolved from the shared tokens.
    #[must_use]
    pub fn px(self, tokens: &Tokens) -> f32 {
        tokens.rows[match self {
            RowHeight::Dense => 0,
            RowHeight::Grid => 1,
            RowHeight::Preview => 2,
            RowHeight::Comfortable => 3,
        }]
    }
}

/// Configuration for one [`list_row`]: which rung, and whether the row is the
/// current (persistent) selection. Nothing else is configurable — treatment
/// is the tokens' job.
#[derive(Clone, Copy, Debug)]
pub struct ListRow {
    height: RowHeight,
    selected: bool,
}

impl ListRow {
    /// A row at `height`, unselected.
    #[must_use]
    pub fn new(height: RowHeight) -> Self {
        Self {
            height,
            selected: false,
        }
    }

    /// Mark the row as the persistent selection.
    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

/// The interaction state a row's content closure sees while laying itself
/// out — the basis for visibility-based hover affordances.
#[derive(Clone, Copy, Debug)]
pub struct RowState {
    /// The pointer is over the row (anywhere in its rect, including over
    /// child widgets).
    pub hovered: bool,
    /// The row is the persistent selection.
    pub selected: bool,
    /// The primary button is down on the row.
    pub pressed: bool,
}

/// What [`list_row`] hands back: the row's own response (the click target),
/// the state it drew in, and the content closure's return value.
pub struct ListRowResponse<R> {
    /// The full-row interaction response. A click on a child widget inside
    /// the row is *not* reported here — egui resolves the hit to the child —
    /// so "row activated" and "row's button pressed" stay distinct.
    pub response: egui::Response,
    /// The state the row drew in this frame.
    pub state: RowState,
    /// Whatever the content closure returned.
    pub inner: R,
}

impl<R> ListRowResponse<R> {
    /// The row itself was clicked (not a child widget inside it).
    #[must_use]
    pub fn clicked(&self) -> bool {
        self.response.clicked()
    }
}

/// Draw one row: allocate the full available width at the rung's height,
/// paint hover/selected/pressed treatment from the semantic row tokens, then
/// lay out `content` vertically centred inside it with ladder padding.
///
/// State precedence: selected > pressed > hovered > nothing. Selected rows
/// additionally carry the token `selected_border` hairline, so selection
/// survives losing the pointer.
pub fn list_row<R>(
    ui: &mut egui::Ui,
    row: ListRow,
    content: impl FnOnce(&mut egui::Ui, RowState) -> R,
) -> ListRowResponse<R> {
    let t = ui.tokens();
    let dark = ui.visuals().dark_mode;
    let sem = semantic(dark);

    let height = row.height.px(t);
    let desired = egui::vec2(ui.available_width(), height);
    let (rect, response) = ui.allocate_exact_size(desired, Sense::click());

    let state = RowState {
        hovered: response.contains_pointer(),
        selected: row.selected,
        pressed: response.is_pointer_button_down_on(),
    };

    if ui.is_rect_visible(rect) {
        let rows = &sem.rows;
        let fill = if state.selected {
            Some(rows.selected_background)
        } else if state.pressed {
            // The rows vocabulary has no pressed slot; the neutral role's
            // active state is the token for "a chrome-wearing control, held".
            Some(sem.role(Role::Neutral).background.active)
        } else if state.hovered {
            Some(rows.hover_background)
        } else {
            None
        };
        if let Some(fill) = fill {
            ui.painter()
                .rect_filled(rect, t.radius_control, to_color32(fill));
        }
        if state.selected {
            ui.painter().rect_stroke(
                rect,
                t.radius_control,
                egui::Stroke::new(1.0, to_color32(rows.selected_border)),
                StrokeKind::Inside,
            );
        }
    }

    let content_rect = rect.shrink2(egui::vec2(t.space[4], 0.0));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(content_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let inner = content(&mut child, state);

    ListRowResponse {
        response,
        state,
        inner,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TOKENS;

    #[test]
    fn every_rung_resolves_to_its_ladder_value() {
        assert_eq!(RowHeight::Dense.px(&TOKENS), TOKENS.rows[0]);
        assert_eq!(RowHeight::Grid.px(&TOKENS), TOKENS.rows[1]);
        assert_eq!(RowHeight::Preview.px(&TOKENS), TOKENS.rows[2]);
        assert_eq!(RowHeight::Comfortable.px(&TOKENS), TOKENS.rows[3]);
    }

    #[test]
    fn a_row_lays_out_and_reports_state() {
        let mut saw_state: Option<RowState> = None;
        egui::__run_test_ui(|ui| {
            let r = list_row(ui, ListRow::new(RowHeight::Grid).selected(true), |ui, s| {
                ui.label("row");
                s
            });
            saw_state = Some(r.inner);
            assert!(r.state.selected);
        });
        assert!(saw_state.expect("content closure must run").selected);
    }
}
