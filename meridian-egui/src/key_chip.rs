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
    Align, Color32, CornerRadius, FontFamily, FontId, FontSelection, Galley, Rect, RichText, Sense,
    Vec2, WidgetText,
};
use meridian_design::control::KEYCAP_FOOT_WIDTH;
use meridian_design::semantic;
use meridian_design::typography::CHART_LABEL_SIZE;

use crate::theme::to_color32;
use crate::widgets::optically_centred_galley_top;
use crate::MeridianUi;

/// The keycap ink: the same muted secondary the verb beside a chip uses.
fn chip_ink(ui: &egui::Ui) -> Color32 {
    to_color32(semantic(ui.visuals().dark_mode).text.secondary)
}

/// The rule on a keycap's top and sides. The foot under the face is
/// [`KEYCAP_FOOT_WIDTH`] and is the bottom edge, so it is not one of the sides.
const KEYCAP_SIDE_RULE: f32 = 1.0;

/// What the chip adds around its keystroke's galley: the spacing-ladder padding
/// and the rule on each edge — a hairline on the top and both sides, the foot
/// under the face. Every geometry comes from a token — there is nothing to tune
/// at the call site, which is the point.
///
/// The chip's box is the galley plus this, and it is the same box the chip drew
/// when it was one frame with one hairline all round: the foot takes one more
/// pixel out of the bottom than that hairline did, from the padding rather than
/// from the box, so a row that reserves [`chip_width`] and [`chip_height`] does
/// not move.
fn chip_extra(ui: &egui::Ui) -> Vec2 {
    let t = ui.tokens();
    Vec2::new(
        2.0 * (t.chip_padding_x + KEYCAP_SIDE_RULE),
        2.0 * (t.space[1] + KEYCAP_SIDE_RULE),
    )
}

/// The face of a keycap drawn in `rect`: the fill inside its top and side
/// hairlines and above its foot. The keystroke is centred on it.
fn keycap_face(rect: Rect) -> Rect {
    Rect::from_min_max(
        rect.min + Vec2::splat(KEYCAP_SIDE_RULE),
        rect.max - Vec2::new(KEYCAP_SIDE_RULE, KEYCAP_FOOT_WIDTH),
    )
}

/// Paint the keycap's box in `rect`: the sunken fill, a 1px hairline in the
/// subtle border ink on the top and both sides, and a [`KEYCAP_FOOT_WIDTH`] foot
/// in the default border ink across the bottom — so it reads as a key by its
/// heavier foot rather than by a corner. One frame stroke cannot draw the two
/// widths, so the box is five square rects that do not overlap: the fill's
/// face and the four rules around it.
fn paint_keycap(ui: &egui::Ui, rect: Rect) {
    let sem = semantic(ui.visuals().dark_mode);
    let painter = ui.painter();
    let square = CornerRadius::ZERO;
    let side = to_color32(sem.borders.subtle);
    let foot = to_color32(sem.borders.default_);

    painter.rect_filled(rect, square, to_color32(sem.surfaces.sunken));

    let (left, right, top, bottom) = (rect.left(), rect.right(), rect.top(), rect.bottom());
    let foot_top = bottom - KEYCAP_FOOT_WIDTH;
    let rules = [
        // The top hairline, across the whole width.
        (
            Rect::from_min_max(rect.min, egui::pos2(right, top + KEYCAP_SIDE_RULE)),
            side,
        ),
        // The two sides, between the top hairline and the foot.
        (
            Rect::from_min_max(
                egui::pos2(left, top + KEYCAP_SIDE_RULE),
                egui::pos2(left + KEYCAP_SIDE_RULE, foot_top),
            ),
            side,
        ),
        (
            Rect::from_min_max(
                egui::pos2(right - KEYCAP_SIDE_RULE, top + KEYCAP_SIDE_RULE),
                egui::pos2(right, foot_top),
            ),
            side,
        ),
        // The foot, across the whole width.
        (
            Rect::from_min_max(egui::pos2(left, foot_top), rect.max),
            foot,
        ),
    ];
    for (rule, ink) in rules {
        painter.rect_filled(rule, square, ink);
    }
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
/// own padding, hairline and foot.
///
/// A caller laying out a row of chips needs this **before** it draws them. A
/// row that instead takes the space left over hands each chip a column, and
/// egui's cross-centred horizontal layout grows a child to fill what it is
/// handed — see [`key_chip`].
pub(crate) fn chip_height(ui: &egui::Ui, keystroke: &str) -> f32 {
    chip_galley(ui, keystroke).size().y + chip_extra(ui).y
}

/// How wide a [`key_chip`] draws for `keystroke`: its galley plus the chip's
/// own padding and hairline — the horizontal twin of [`chip_height`], and the
/// same claim on the other axis.
///
/// A caller laying text out beside a chip needs this **before** it draws
/// either. A row that instead adds the text first hands it the whole row's
/// width, because `egui::Label` reads `ui.available_width()` at the moment it
/// is added and a chip that has not been added yet has claimed nothing; the
/// chip then paints an opaque fill over the glyphs already on the canvas —
/// see [`crate::picker::Picker`]'s match list, which is why this exists.
pub(crate) fn chip_width(ui: &egui::Ui, keystroke: &str) -> f32 {
    chip_galley(ui, keystroke).size().x + chip_extra(ui).x
}

/// A keystroke rendered as a keycap chip: monospace label on the sunken
/// surface, a 1px hairline on its top and sides and a heavier foot under it —
/// see [`paint_keycap`] — with spacing-ladder padding and no corner. Every
/// geometry and colour comes from a token — there is nothing to tune at the
/// call site, which is the point.
///
/// The chip's size is settled from its galley and its own insets before any
/// space is claimed, which is what keeps a keycap keycap-sized in every layout.
/// Measuring it the other way round — laying the content out in a frame and
/// letting [`egui::Frame::show`] report whatever its content ui used — is not
/// content-driven inside a cross-centred horizontal layout: egui grows a
/// child's frame to `available_rect.height()` there
/// (`Layout::next_frame_ignore_wrap`) and folds that frame into the ui's
/// `min_rect` (`Placer::advance_after_rects`), so the chip would take the
/// whole height the caller had spare.
///
/// The keystroke is centred on the keycap's face — the fill above the foot and
/// inside the hairlines — and placed on the glyphs it inks, not on the font
/// box those glyphs were laid out in — [`optically_centred_galley_top`]. A
/// galley's box is metrics: one ascent above the baseline and one reserved
/// descent below it, the same height for every string in the face whatever the
/// string is, so centring it centres the metrics and leaves the ink wherever it
/// falls inside them. The bundled mono face reserves more room below its
/// baseline than it leaves above its ascenders, so its two boxes do not share a
/// centre for *any* string and every keystroke moved when this was corrected —
/// a keystroke with no descender down, one with a descender up.
///
/// Returns the chip's [`egui::Response`] so a caller can hang a tooltip or
/// hover behaviour off it.
pub fn key_chip(ui: &mut egui::Ui, keystroke: &str) -> egui::Response {
    let galley = chip_galley(ui, keystroke);

    let (rect, response) = ui.allocate_exact_size(galley.size() + chip_extra(ui), Sense::hover());

    if ui.is_rect_visible(rect) {
        paint_keycap(ui, rect);
        let face = keycap_face(rect);
        let top = optically_centred_galley_top(&galley, face.center().y);
        ui.painter().galley(
            egui::pos2(face.min.x + ui.tokens().chip_padding_x, top),
            galley,
            chip_ink(ui),
        );
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
        // And under the role a consumer resolves a chip by. The chip publishes
        // this node itself rather than getting one from an `egui::Label`, so
        // the role is a decision here and not a consequence.
        harness.get_by_role_and_label(egui::accesskit::Role::Label, "Esc");
    }

    /// The reservation a caller makes is the space the chip then takes.
    ///
    /// `chip_width` is not a second estimate of the chip's width — it is the
    /// same two terms `key_chip` allocates from, so a row that reserves it and
    /// a chip that fills it cannot disagree. Pinned across keystrokes of
    /// different lengths so a fix that happened to hold for one string is not
    /// mistaken for the relation.
    #[test]
    fn the_reserved_width_is_the_width_the_chip_allocates() {
        for keystroke in ["k", "Esc", "Enter", "\u{2318}\u{21e7}P"] {
            let mut measured = None;
            let mut drawn = None;
            {
                // The harness holds the closure, and the closure holds these
                // two; it has to go out of scope before they can be read.
                let mut harness = Harness::new_ui(|ui| {
                    crate::theme::apply(ui.ctx(), crate::Mode::Light);
                    measured = Some(chip_width(ui, keystroke));
                    drawn = Some(key_chip(ui, keystroke).rect.width());
                });
                harness.run();
            }
            let measured = measured.expect("the closure runs");
            let drawn = drawn.expect("the closure runs");
            assert!(
                (measured - drawn).abs() < 0.01,
                "{keystroke:?}: reserved {measured} pt, the chip took {drawn} pt"
            );
            assert!(drawn > 0.0, "{keystroke:?}: the chip took no width at all");
        }
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
