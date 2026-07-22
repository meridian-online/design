//! Keystroke chips and action tooltips.
//!
//! Two tiny primitives with one purpose: put the keyboard on the surface. A
//! [`key_chip`] renders a keystroke as a small keycap-styled chip; a
//! [`tooltip_for_action`] attaches a hover tooltip pairing an action's name
//! with its keystroke chip, so every control can advertise its shortcut the
//! same way.
//!
//! Both take the keystroke as a **caller-provided string**. Which key maps to
//! which verb is application state — a keystroke registry is host-side
//! information architecture (ADR 0011) — so this module renders whatever it
//! is handed and holds no bindings of its own.

use egui::{FontFamily, FontId, Label, Margin, RichText};
use meridian_design::semantic;
use meridian_design::typography::CHART_LABEL_SIZE;

use crate::theme::to_color32;
use crate::MeridianUi;

/// A keystroke rendered as a keycap chip: monospace label on the sunken
/// surface with a hairline border, chip radius, and spacing-ladder padding.
/// Every geometry and colour comes from a token — there is nothing to tune at
/// the call site, which is the point.
///
/// Returns the chip's [`egui::Response`] so a caller can hang a tooltip or
/// hover behaviour off it.
pub fn key_chip(ui: &mut egui::Ui, keystroke: &str) -> egui::Response {
    let t = ui.tokens();
    let dark = ui.visuals().dark_mode;
    let sem = semantic(dark);

    let text = RichText::new(keystroke)
        .font(FontId::new(CHART_LABEL_SIZE, FontFamily::Monospace))
        .color(to_color32(sem.text.secondary));

    egui::Frame::new()
        .fill(to_color32(sem.surfaces.sunken))
        .stroke(egui::Stroke::new(1.0, to_color32(sem.borders.default_)))
        .corner_radius(t.radius_chip)
        .inner_margin(Margin::symmetric(t.space[2] as i8, t.space[1] as i8))
        .show(ui, |ui| {
            ui.add(Label::new(text).selectable(false));
        })
        .response
}

/// Attach a tooltip to `response` naming an action and (optionally) its
/// keystroke as a [`key_chip`]. The one tooltip treatment every surface uses:
/// action name in body ink, chip trailing.
///
/// The keystroke is a caller-provided string for the same reason [`key_chip`]
/// takes one — the binding registry stays application-side.
pub fn tooltip_for_action(
    response: egui::Response,
    action: &str,
    keystroke: Option<&str>,
) -> egui::Response {
    response.on_hover_ui(|ui| {
        ui.horizontal(|ui| {
            ui.label(action);
            if let Some(k) = keystroke {
                ui.add_space(ui.tokens().icon_label_gap);
                key_chip(ui, k);
            }
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable;
    use egui_kittest::Harness;

    #[test]
    fn chip_renders_its_keystroke_as_a_queryable_node() {
        let mut harness = Harness::new_ui(|ui| {
            crate::theme::apply(ui.ctx(), crate::Mode::Light);
            key_chip(ui, "Esc");
        });
        harness.run();
        harness.get_by_label("Esc");
    }

    #[test]
    fn tooltip_appears_on_hover_with_action_and_keystroke() {
        let mut harness = Harness::new_ui(|ui| {
            crate::theme::apply(ui.ctx(), crate::Mode::Light);
            let r = ui.button("Run");
            tooltip_for_action(r, "Run the pipeline", Some("⌘R"));
        });
        harness.run();
        assert!(harness.query_by_label("Run the pipeline").is_none());
        harness.get_by_label("Run").hover();
        harness.run();
        harness.get_by_label("Run the pipeline");
        harness.get_by_label("⌘R");
    }
}
