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

use std::sync::Arc;

use egui::{
    Align, Color32, FontFamily, FontId, FontSelection, Galley, Margin, RichText, Sense, WidgetText,
};
use meridian_design::semantic;
use meridian_design::typography::CHART_LABEL_SIZE;

use crate::theme::to_color32;
use crate::MeridianUi;

/// The keycap ink: the same muted secondary the verb beside a chip uses.
fn chip_ink(ui: &egui::Ui) -> Color32 {
    to_color32(semantic(ui.visuals().dark_mode).text.secondary)
}

/// The chip's box: sunken fill, hairline border, chip radius, spacing-ladder
/// padding. Every geometry and colour comes from a token — there is nothing to
/// tune at the call site, which is the point.
fn chip_frame(ui: &egui::Ui) -> egui::Frame {
    let t = ui.tokens();
    let sem = semantic(ui.visuals().dark_mode);
    egui::Frame::new()
        .fill(to_color32(sem.surfaces.sunken))
        .stroke(egui::Stroke::new(1.0, to_color32(sem.borders.default_)))
        .corner_radius(t.radius_chip)
        .inner_margin(Margin::symmetric(t.space[2] as i8, t.space[1] as i8))
}

/// The keystroke laid out in the keycap's monospace ink: one section, no
/// wrapping, and a vertical alignment of its own rather than the containing
/// layout's, so the galley is a function of the string and the tokens and of
/// nothing about the space the caller happens to have spare.
fn chip_galley(ui: &egui::Ui, keystroke: &str) -> Arc<Galley> {
    let job = WidgetText::from(
        RichText::new(keystroke)
            .font(FontId::new(CHART_LABEL_SIZE, FontFamily::Monospace))
            .color(chip_ink(ui)),
    )
    .into_layout_job(ui.style(), FontSelection::Default, Align::Min);
    ui.painter().layout_job(Arc::unwrap_or_clone(job))
}

/// How tall a [`key_chip`] draws for `keystroke`: its galley plus the chip's
/// own padding and hairline.
///
/// A caller laying out a row of chips needs this **before** it draws them. A
/// row that instead takes the space left over hands each chip a column, and
/// egui's cross-centred horizontal layout grows a child to fill what it is
/// handed — see [`key_chip`].
pub(crate) fn chip_height(ui: &egui::Ui, keystroke: &str) -> f32 {
    chip_galley(ui, keystroke).size().y + chip_frame(ui).total_margin().sum().y
}

/// A keystroke rendered as a keycap chip: monospace label on the sunken
/// surface with a hairline border, chip radius, and spacing-ladder padding.
/// Every geometry and colour comes from a token — there is nothing to tune at
/// the call site, which is the point.
///
/// The chip's size is settled from its galley and the frame's own margins
/// before any space is claimed, which is what keeps a keycap keycap-sized in
/// every layout. Measuring it the other way round — letting
/// [`egui::Frame::show`] report whatever its content ui used — is not
/// content-driven inside a cross-centred horizontal layout: egui grows a
/// child's frame to `available_rect.height()` there
/// (`Layout::next_frame_ignore_wrap`) and folds that frame into the ui's
/// `min_rect` (`Placer::advance_after_rects`), so the chip would take the
/// whole height the caller had spare.
///
/// Returns the chip's [`egui::Response`] so a caller can hang a tooltip or
/// hover behaviour off it.
pub fn key_chip(ui: &mut egui::Ui, keystroke: &str) -> egui::Response {
    let frame = chip_frame(ui);
    let galley = chip_galley(ui, keystroke);
    let margin = frame.total_margin();

    let (rect, response) = ui.allocate_exact_size(galley.size() + margin.sum(), Sense::hover());
    let content_rect = rect - margin;

    if ui.is_rect_visible(rect) {
        ui.painter().add(frame.paint(content_rect));
        ui.painter().galley(content_rect.min, galley, chip_ink(ui));
    }

    let enabled = ui.is_enabled();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, enabled, keystroke));
    response
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
