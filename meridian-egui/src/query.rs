//! The single-line query input every picker shares.
//!
//! One prompt glyph, one height (a control-ladder rung), one hairline under
//! the input separating query from results. The audit behind ADR 0011 found
//! this row hand-rolled several times with prompt glyphs that disagreed;
//! [`query_line`] is the one copy, and [`PROMPT_GLYPH`] is the one glyph.

use egui::{Align, Layout, RichText, Sense, TextEdit};
use meridian_design::semantic;

use crate::theme::to_color32;
use crate::MeridianUi;

/// The one query prompt glyph. Public so a caller (or a test) can assert
/// against it rather than restating it.
pub const PROMPT_GLYPH: &str = "›";

/// What [`query_line`] hands back.
pub struct QueryLineResponse {
    /// The text edit's response — request focus on it to focus the query.
    pub response: egui::Response,
    /// The query string changed this frame.
    pub changed: bool,
}

/// Draw the query row: prompt glyph in muted ink, a frameless single-line
/// text edit filling the remaining width at the large control-ladder height,
/// and the hairline rule under it that separates the query from whatever
/// list follows.
pub fn query_line(ui: &mut egui::Ui, query: &mut String, placeholder: &str) -> QueryLineResponse {
    let t = ui.tokens();
    let dark = ui.visuals().dark_mode;
    let sem = semantic(dark);

    let height = t.control_heights[3];
    let response = ui
        .allocate_ui_with_layout(
            egui::vec2(ui.available_width(), height),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.add_space(t.space[4]);
                ui.label(RichText::new(PROMPT_GLYPH).color(to_color32(sem.text.muted)));
                ui.add_space(t.icon_label_gap);
                ui.add(
                    TextEdit::singleline(query)
                        .hint_text(placeholder)
                        .frame(egui::Frame::NONE)
                        .vertical_align(Align::Center)
                        .desired_width(f32::INFINITY),
                )
            },
        )
        .inner;

    // The rule between query and results — a single structural edge, so it
    // takes the subtle border step rather than the repeated-divider step.
    let (line_rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), Sense::hover());
    if ui.is_rect_visible(line_rect) {
        ui.painter().hline(
            line_rect.x_range(),
            line_rect.center().y,
            egui::Stroke::new(1.0, to_color32(sem.borders.subtle)),
        );
    }

    QueryLineResponse {
        changed: response.changed(),
        response,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable;
    use egui_kittest::Harness;

    #[test]
    fn typing_into_a_focused_query_line_reports_changed() {
        struct S {
            query: String,
            changed: bool,
        }
        let mut harness = Harness::new_ui_state(
            |ui, s: &mut S| {
                crate::theme::apply(ui.ctx(), crate::Mode::Light);
                let r = query_line(ui, &mut s.query, "Search…");
                r.response.request_focus();
                s.changed |= r.changed;
            },
            S {
                query: String::new(),
                changed: false,
            },
        );
        harness.run();
        harness.event(egui::Event::Text("abc".to_owned()));
        harness.run();
        assert_eq!(harness.state().query, "abc");
        assert!(harness.state().changed);
    }

    #[test]
    fn the_one_prompt_glyph_is_part_of_the_chrome() {
        // The placeholder itself is egui hint text (not an accessibility
        // node), so what this asserts is the chrome the primitive owns: the
        // single prompt glyph, present without any call-site choice.
        let mut query = String::new();
        let mut harness = Harness::new_ui(|ui| {
            crate::theme::apply(ui.ctx(), crate::Mode::Light);
            query_line(ui, &mut query, "Jump to…");
        });
        harness.run();
        harness.get_by_label(PROMPT_GLYPH);
    }
}
