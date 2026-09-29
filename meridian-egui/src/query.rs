//! The single-line query input every picker shares.
//!
//! One prompt glyph, one height (a control-ladder rung), one rule under the
//! input separating query from results. The audit behind ADR 0011 found this
//! row hand-rolled several times with prompt glyphs that disagreed;
//! [`query_line`] is the one copy, and [`PROMPT_GLYPH`] is the one glyph.

use egui::{Align, CornerRadius, Layout, Rect, RichText, Sense, TextEdit, UiBuilder};
use meridian_design::focus::RING_WIDTH;
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

/// The thickness of the rule under the query while it does not have the keys.
/// The same hairline the row's separator always was, now the field's own rule.
const FIELD_RULE: f32 = 1.0;

/// Draw the query row: prompt glyph in muted ink, a frameless single-line
/// text edit filling the remaining width at the large control-ladder height,
/// all on the sunken fill, over one rule that separates the query from whatever
/// list follows.
///
/// The field is the sunken fill over a 1px rule in the control border ink; while
/// the text edit has the keys the rule is [`RING_WIDTH`] thick in the focus ink,
/// drawn up into the fill so the row's height does not change with focus.
pub fn query_line(ui: &mut egui::Ui, query: &mut String, placeholder: &str) -> QueryLineResponse {
    let t = ui.tokens();
    let dark = ui.visuals().dark_mode;
    let sem = semantic(dark);

    // The field is the row at its rung and the rule's own strip beneath it: the
    // hairline that always separated the query from the results, so nothing
    // below the field moves.
    let height = t.control_heights[3];
    let (field, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height + FIELD_RULE),
        Sense::hover(),
    );
    let row = Rect::from_min_size(field.min, egui::vec2(field.width(), height));

    // The fill goes down first so the text draws over it; the rule goes down
    // last, once the edit has said whether it has the keys.
    if ui.is_rect_visible(field) {
        ui.painter()
            .rect_filled(field, CornerRadius::ZERO, to_color32(sem.surfaces.sunken));
    }

    let mut row_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(row)
            .layout(Layout::left_to_right(Align::Center)),
    );
    row_ui.add_space(t.space[4]);
    row_ui.label(RichText::new(PROMPT_GLYPH).color(to_color32(sem.text.muted)));
    row_ui.add_space(t.icon_label_gap);
    let response = row_ui.add(
        TextEdit::singleline(query)
            .hint_text(placeholder)
            .frame(egui::Frame::NONE)
            .vertical_align(Align::Center)
            .desired_width(f32::INFINITY),
    );

    if ui.is_rect_visible(field) {
        let (width, ink) = if response.has_focus() {
            (RING_WIDTH, sem.borders.focus)
        } else {
            (FIELD_RULE, sem.borders.control)
        };
        let rule = Rect::from_min_max(egui::pos2(field.left(), field.bottom() - width), field.max);
        ui.painter()
            .rect_filled(rule, CornerRadius::ZERO, to_color32(ink));
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
