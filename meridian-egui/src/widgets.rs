//! The shared chrome primitives: the field row, the pane header, the status
//! pill, the focus ring, and the slider colour pair.
//!
//! These are the widgets small enough to be *system*, not application: every
//! Meridian egui surface draws them the same way, from the same tokens. They
//! read their mode from the installed visuals ([`mode_of`]), so a context
//! themed by [`crate::theme::apply`] needs no extra argument threading — and
//! `apply` (or at least [`crate::theme::install_fonts`]) **must** have run,
//! because the pane header resolves the `Inter-Medium` family the theme
//! installs.
//!
//! Alignment: [`field_row`] participates in the two-pass shared column of
//! [`crate::align::align_scope`] when drawn inside one. Outside a scope it
//! degrades to alignment-by-convention (each row splits after its own label)
//! — supported, but a panel of rows should be wrapped in a scope.

use egui::{Align, Layout, Response, RichText, Sense, StrokeKind, UiBuilder};
use meridian_design::chrome::{OverlayTokens, OVERLAY_DARK, OVERLAY_LIGHT};
use meridian_design::semantic::{semantic, Role};
use meridian_design::typography::UI_SIZE;
use meridian_design::{control, focus};

use crate::align::LayoutInfo;
use crate::icons::Icon;
use crate::theme::to_color32;
use crate::{MeridianUi, Mode};

/// The [`Mode`] a `Ui` is themed in, read from its installed visuals — the
/// bridge back from egui state to the token accessors' `dark` argument.
#[must_use]
pub fn mode_of(ui: &egui::Ui) -> Mode {
    if ui.visuals().dark_mode {
        Mode::Dark
    } else {
        Mode::Light
    }
}

/// The mode's overlay tokens (the GPUI-era "never in the data scene" group:
/// brush wash, brush border, focus ring).
fn overlay(mode: Mode) -> &'static OverlayTokens {
    if mode.is_dark() {
        &OVERLAY_DARK
    } else {
        &OVERLAY_LIGHT
    }
}

/// The medium-weight family for titles once the Meridian fonts are installed.
///
/// `install_fonts` takes effect on the *next* frame (egui applies `set_fonts`
/// at frame boundaries), so the very first frame after `theme::apply` would
/// panic on the named family. Falling back to the default proportional face
/// for that one frame keeps the primitive safe to draw unconditionally.
fn medium_family(ui: &egui::Ui) -> egui::FontFamily {
    let family = egui::FontFamily::Name("Inter-Medium".into());
    if ui.fonts(|fonts| fonts.definitions().families.contains_key(&family)) {
        family
    } else {
        egui::FontFamily::Proportional
    }
}

/// One inspector field: a muted uppercase label, the value beside it in the
/// panel's shared column, and an optional one-line explainer under the value.
///
/// `mono` picks the face the *value* is set in — true for a value the reader
/// compares character by character (an address, a hash), false for prose.
///
/// Inside an [`crate::align::align_scope`], every `field_row` of the panel
/// splits at one shared x — the two-pass accumulator at work. Outside a
/// scope, the row splits after its own label.
pub fn field_row(
    ui: &mut egui::Ui,
    label: &str,
    value: &str,
    explain: Option<&str>,
    mono: bool,
) -> Response {
    let sem = semantic(mode_of(ui).is_dark());
    let muted = to_color32(sem.text.muted);
    let tokens = ui.tokens();
    let column_gap = tokens.control_gap;
    let info = LayoutInfo::current(ui.ctx());
    let left_x = info
        .as_ref()
        .map_or_else(|| ui.max_rect().left(), LayoutInfo::left_x);

    // The scope zeroes vertical item spacing (rows own their air), so the row
    // brings its own leading.
    ui.add_space(tokens.space[2]);

    let mut split_x = left_x;
    let row = ui.horizontal(|ui| {
        let label_response = ui.label(RichText::new(label.to_uppercase()).color(muted));

        // Register what this row's label wants; consume what the whole scope
        // agreed on last frame.
        let desired = (label_response.rect.right() - left_x) + column_gap;
        if let Some(info) = &info {
            info.register_desired_left_column_width(ui, desired);
        }
        split_x = match info.as_ref().and_then(LayoutInfo::left_column_width) {
            Some(width) => left_x + width,
            None => label_response.rect.right() + column_gap,
        };

        let pad = split_x - ui.cursor().min.x;
        if pad > 0.0 {
            ui.add_space(pad);
        }

        let text = RichText::new(value).color(to_color32(sem.text.primary));
        let text = if mono { text.monospace() } else { text };
        let value_response = ui.label(text);
        if let Some(info) = &info {
            // The item width rerun's cap heuristic wants is the full row span
            // (its list items fill the row), not the content's extent — so
            // register out to the row's right edge, or further if the value
            // overflows it.
            let row_extent =
                (value_response.rect.right() - left_x).max(ui.max_rect().right() - left_x);
            info.register_max_item_width(ui, row_extent);
        }
    });

    let mut response = row.response;
    if let Some(explain) = explain {
        let explain_row = ui.horizontal(|ui| {
            let pad = split_x - ui.cursor().min.x;
            if pad > 0.0 {
                ui.add_space(pad);
            }
            ui.label(RichText::new(explain).color(muted));
        });
        response = response.union(explain_row.response);
    }
    response
}

/// A pane's header band: optional leading icon, a medium-weight title, and a
/// bottom hairline over the header surface. Returns the band's response.
pub fn pane_header(ui: &mut egui::Ui, icon: Option<&Icon>, title: &str) -> Response {
    pane_header_with(ui, icon, title, |_| {})
}

/// [`pane_header`] with a right-aligned trailing slot (close buttons, a mode
/// toggle). The slot's `Ui` lays out right-to-left inside the band.
pub fn pane_header_with(
    ui: &mut egui::Ui,
    icon: Option<&Icon>,
    title: &str,
    trailing: impl FnOnce(&mut egui::Ui),
) -> Response {
    let sem = semantic(mode_of(ui).is_dark());
    let tokens = ui.tokens();
    // The smallest rung that is a self-sufficient pointer target.
    let height = control::HEIGHT_MD;

    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), height), Sense::hover());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        painter.rect_filled(rect, 0, to_color32(sem.surfaces.header));
        painter.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            egui::Stroke::new(1.0, to_color32(sem.borders.subtle)),
        );

        let inner = rect.shrink2(egui::vec2(tokens.space[3], 0.0));
        let mut left = ui.new_child(
            UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::left_to_right(Align::Center)),
        );
        left.spacing_mut().item_spacing.x = tokens.icon_label_gap;
        if let Some(icon) = icon {
            icon.show(&mut left, control::ICON_SM, to_color32(sem.text.secondary));
        }
        let family = medium_family(&left);
        left.label(
            RichText::new(title)
                .family(family)
                .size(UI_SIZE)
                .color(to_color32(sem.text.primary)),
        );

        let mut right = ui.new_child(
            UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::right_to_left(Align::Center)),
        );
        trailing(&mut right);
    }
    response
}

/// A status pill: icon + label on a role-coloured chip. Generic on purpose —
/// the *vocabulary* (which states exist and which [`Role`] each wears) belongs
/// to the application; this widget only guarantees the redundancy rule: icon
/// and label always travel together, so colour is never the only signal.
pub fn status_pill(ui: &mut egui::Ui, icon: &Icon, label: &str, role: Role) -> Response {
    let sem = semantic(mode_of(ui).is_dark());
    let colours = sem.role(role);
    let ink = to_color32(colours.foreground.base);
    let tokens = ui.tokens();

    let height = control::HEIGHT_XS;
    let icon_size = control::ICON_XS;
    let pad_x = tokens.space[2];
    let gap = tokens.icon_label_gap;

    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        egui::FontId::new(UI_SIZE, egui::FontFamily::Proportional),
        ink,
    );
    let width = pad_x + icon_size + gap + galley.size().x + pad_x;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
    // The text is painted, not laid out as a child widget, so the pill carries
    // its label into the accessibility tree itself.
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));

    if ui.is_rect_visible(rect) {
        let radius = egui::CornerRadius::same(tokens.radius_chip.round() as u8);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, to_color32(colours.background.base));
        painter.rect_stroke(
            rect,
            radius,
            egui::Stroke::new(1.0, to_color32(colours.border.base)),
            StrokeKind::Inside,
        );

        let icon_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + pad_x, rect.center().y - icon_size / 2.0),
            egui::vec2(icon_size, icon_size),
        );
        icon.paint(painter, icon_rect, ink);
        painter.galley(
            egui::pos2(
                icon_rect.right() + gap,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            ink,
        );
    }
    response
}

/// Draw the keyboard focus ring around `rect` — the **one** focus/selection
/// treatment. Colour is the overlay `focus_ring` token (Maritime: focus is
/// the purest "accent as a verb"); geometry is the token set in
/// [`meridian_design::focus`] — an outset ring at [`focus::RING_OFFSET`],
/// [`focus::RING_WIDTH`] thick, concentric with the control's corners.
///
/// `control_radius` is the corner radius of the control the ring surrounds
/// (usually `ui.tokens().radius_control`). Layouts that clip hard at the
/// control's box must reserve [`focus::RING_BLEED`] or use an inset treatment.
pub fn focus_ring(ui: &egui::Ui, rect: egui::Rect, control_radius: f32) {
    let colour = overlay(mode_of(ui)).focus_ring;
    // `StrokeKind::Outside` keeps the whole stroke outside the given rect, so
    // expanding by the offset alone puts the ring's inner edge exactly
    // RING_OFFSET off the control.
    ui.painter().rect_stroke(
        rect.expand(focus::RING_OFFSET),
        egui::CornerRadius::same(focus::ring_radius(control_radius).round() as u8),
        egui::Stroke::new(focus::RING_WIDTH, to_color32(colour)),
        StrokeKind::Outside,
    );
}

/// Draw the focus ring around a widget iff it has keyboard focus — the call a
/// widget adds after `interact`. Uses the control corner radius.
pub fn focus_ring_for(ui: &egui::Ui, response: &Response) {
    if response.has_focus() {
        focus_ring(ui, response.rect, ui.tokens().radius_control);
    }
}

/// The slider colour pair, resolved to egui colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliderColours {
    /// The rail — `feedback.slider_track`.
    pub track: egui::Color32,
    /// The grab — `feedback.slider_thumb` (the Maritime solid: a slider is an
    /// interaction, and the accent is a verb).
    pub thumb: egui::Color32,
}

/// The semantic `feedback.slider_track` / `feedback.slider_thumb` tokens for
/// `mode`, through the one colour boundary ([`to_color32`]). egui's `Visuals`
/// has no slider slots, so any surface drawing a slider takes its colours from
/// here rather than restating scale steps.
#[must_use]
pub fn slider_colours(mode: Mode) -> SliderColours {
    let feedback = &semantic(mode.is_dark()).feedback;
    SliderColours {
        track: to_color32(feedback.slider_track),
        thumb: to_color32(feedback.slider_thumb),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use meridian_design::scales::{GRAY_DARK, GRAY_LIGHT, MARITIME_DARK, MARITIME_LIGHT};

    /// The helper mirrors the semantic tokens exactly (same scale indices the
    /// semantic layer assigns), in both modes.
    #[test]
    fn slider_colours_mirror_the_feedback_tokens() {
        let light = slider_colours(Mode::Light);
        assert_eq!(light.track, to_color32(GRAY_LIGHT[4]));
        assert_eq!(light.thumb, to_color32(MARITIME_LIGHT[9]));

        let dark = slider_colours(Mode::Dark);
        assert_eq!(dark.track, to_color32(GRAY_DARK[4]));
        assert_eq!(dark.thumb, to_color32(MARITIME_DARK[9]));
    }

    /// `mode_of` reads the installed visuals.
    #[test]
    fn mode_follows_the_installed_visuals() {
        let ctx = egui::Context::default();
        for (visuals, expected) in [
            (egui::Visuals::light(), Mode::Light),
            (egui::Visuals::dark(), Mode::Dark),
        ] {
            ctx.set_visuals(visuals);
            let _ = ctx.run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    assert_eq!(mode_of(ui), expected);
                });
            });
        }
    }
}
