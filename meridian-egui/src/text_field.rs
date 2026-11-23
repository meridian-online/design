//! The text field: a sunken fill over a rule, with an optional leading icon
//! and unit, and one message line beneath.
//!
//! The editing is egui's own `TextEdit` with its frame off, so text editing,
//! selection and input methods are not reimplemented. This draws what is
//! around it. The field fills the width it is given: 24 tall, or 20 with
//! [`TextField::dense`]. Its top corners are rounded at the control radius and
//! its bottom corners are square, over a rule along the foot: 1px in the
//! control border at rest and in the primary ink under the pointer; 2px in the
//! focus ink while it has the keys, and 2px in the danger border when it has an
//! error. An error wins over the keys. The rule is drawn inside the box, so
//! thickening it moves nothing.
//!
//! Beneath the field is one message line at label size: the help in the muted
//! ink, or the error in the red scale's step 11 when one is passed, which
//! replaces the help. The field's label is the caller's to lay out.
//!
//! An unavailable field ([`TextField::unavailable`]) keeps its place and its
//! fill, takes no focus and no typing, and rules 1px in the disabled border.
//! The error, when there is one, is the field's accessibility description.

use egui::epaint::text::{FontId, LayoutJob, TextFormat};
use egui::{
    Align, Color32, CornerRadius, Layout, Margin, Rect, Sense, Shape, TextEdit, UiBuilder, Vec2,
};
use meridian_design::control::{HEIGHT_MD, HEIGHT_SM, ICON_SM};
use meridian_design::scales::{RED_DARK, RED_LIGHT};
use meridian_design::semantic::{semantic, Role};
use meridian_design::typography::UI_SIZE;

use crate::icons::Icon;
use crate::theme::to_color32;
use crate::widgets::optically_centred_galley_top;
use crate::MeridianUi;

/// One text field's configuration, drawn by [`text_field`].
#[must_use]
pub struct TextField<'a> {
    text: &'a mut String,
    placeholder: Option<&'a str>,
    icon: Option<&'a Icon>,
    unit: Option<&'a str>,
    help: Option<&'a str>,
    error: Option<&'a str>,
    dense: bool,
    unavailable: bool,
}

impl<'a> TextField<'a> {
    /// A field editing `text`, 24 tall.
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            placeholder: None,
            icon: None,
            unit: None,
            help: None,
            error: None,
            dense: false,
            unavailable: false,
        }
    }

    /// The hint drawn while the field is empty.
    pub fn placeholder(mut self, text: &'a str) -> Self {
        self.placeholder = Some(text);
        self
    }

    /// An icon before the text, in the muted ink.
    pub fn icon(mut self, icon: &'a Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// A unit after the text (`"days"`), in the muted ink.
    pub fn unit(mut self, unit: &'a str) -> Self {
        self.unit = Some(unit);
        self
    }

    /// The help line beneath the field.
    pub fn help(mut self, text: &'a str) -> Self {
        self.help = Some(text);
        self
    }

    /// The error beneath the field. It replaces the help, thickens the rule in
    /// the danger ink and becomes the field's accessibility description.
    pub fn error(mut self, text: &'a str) -> Self {
        self.error = Some(text);
        self
    }

    /// The dense rung: 20 tall.
    pub fn dense(mut self) -> Self {
        self.dense = true;
        self
    }

    /// The field cannot be used now.
    pub fn unavailable(mut self) -> Self {
        self.unavailable = true;
        self
    }
}

/// What [`text_field`] hands back.
#[derive(Clone, Debug)]
pub struct TextFieldResponse {
    /// The field's box (not the message line), with the text edit's id: its
    /// focus is the field's.
    pub response: egui::Response,
    /// The text changed this frame.
    pub changed: bool,
    /// Enter was pressed while the field had the keys.
    pub submitted: bool,
}

/// One line of `text` at `size`, wrapped at `width`.
fn line(
    ui: &egui::Ui,
    text: &str,
    size: f32,
    ink: Color32,
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = LayoutJob::single_section(
        text.to_owned(),
        TextFormat::simple(FontId::proportional(size), ink),
    );
    job.wrap.max_width = width;
    ui.painter().layout_job(job)
}

/// Draw a text field and report what was done to it.
pub fn text_field(ui: &mut egui::Ui, field: TextField<'_>) -> TextFieldResponse {
    let t = ui.tokens();
    let sem = semantic(ui.visuals().dark_mode);
    let c = to_color32;
    let off = field.unavailable;
    let height = if field.dense { HEIGHT_SM } else { HEIGHT_MD };
    let width = ui.available_width();
    let pad = t.space[3];
    let gap = t.space[2];
    let radius = t.radius_control.round() as u8;

    let (field_rect, message) = {
        let mut result = None;
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = t.space[1];
            let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
            let message = field.error.or(field.help).map(|m| {
                let ink = if field.error.is_some() {
                    let red = if ui.visuals().dark_mode {
                        RED_DARK
                    } else {
                        RED_LIGHT
                    };
                    c(red[10])
                } else {
                    c(sem.text.muted)
                };
                let galley = line(ui, m, t.label_size, ink, width);
                let (r, _) = ui
                    .allocate_exact_size(galley.size().max(Vec2::new(width, 0.0)), Sense::hover());
                (r, galley)
            });
            result = Some((rect, message));
        });
        result.expect("scope ran")
    };

    // The box is painted under the text edit, once the state is known.
    let slot = ui.painter().add(Shape::Noop);
    let hovered = !off && ui.rect_contains_pointer(field_rect);

    let unit_galley = field
        .unit
        .map(|u| line(ui, u, UI_SIZE, c(sem.text.muted), f32::INFINITY));
    let mut left = field_rect.left() + pad;
    let mut right = field_rect.right() - pad;
    if let Some(icon) = field.icon {
        let r = Rect::from_center_size(
            egui::pos2(left + ICON_SM / 2.0, field_rect.center().y),
            Vec2::splat(ICON_SM),
        );
        icon.paint(ui.painter(), r, c(sem.text.muted));
        left += ICON_SM + t.icon_label_gap;
    }
    if let Some(g) = &unit_galley {
        let x = right - g.size().x;
        let y = optically_centred_galley_top(g, field_rect.center().y);
        ui.painter()
            .galley(egui::pos2(x, y), g.clone(), c(sem.text.muted));
        right = x - gap;
    }
    let row = ui.fonts_mut(|f| f.row_height(&FontId::proportional(UI_SIZE)));
    let edit_rect = Rect::from_min_max(
        egui::pos2(left, field_rect.center().y - row / 2.0),
        egui::pos2(right.max(left), field_rect.center().y + row / 2.0),
    );
    // egui's own hint text takes its own ink, so the placeholder is drawn here.
    if let (true, Some(p)) = (field.text.is_empty(), field.placeholder) {
        let g = line(ui, p, UI_SIZE, c(sem.text.placeholder), edit_rect.width());
        ui.painter()
            .galley(edit_rect.min, g, c(sem.text.placeholder));
    }

    let id = ui.next_auto_id();
    let text_ink = if off {
        c(sem.text.disabled)
    } else {
        c(sem.text.primary)
    };
    let edit = TextEdit::singleline(field.text)
        .id(id)
        .frame(egui::Frame::NONE)
        .margin(Margin::ZERO)
        .text_color(text_ink)
        .interactive(!off)
        .desired_width(edit_rect.width());
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(edit_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let mut response = child.add_sized(edit_rect.size(), edit);
    response.rect = field_rect;

    let focused = !off && response.has_focus();
    let danger = field.error.is_some();
    let (rule, thickness) = if off {
        (c(sem.role(Role::Neutral).border.disabled), 1.0)
    } else if danger {
        (c(sem.role(Role::Danger).border.base), 2.0)
    } else if focused {
        (c(sem.borders.focus), 2.0)
    } else if hovered {
        (c(sem.text.primary), 1.0)
    } else {
        (c(sem.borders.control), 1.0)
    };
    let fill = egui::epaint::RectShape::filled(
        field_rect,
        CornerRadius {
            nw: radius,
            ne: radius,
            sw: 0,
            se: 0,
        },
        c(sem.surfaces.sunken),
    );
    let foot = Rect::from_min_max(
        egui::pos2(field_rect.left(), field_rect.bottom() - thickness),
        field_rect.right_bottom(),
    );
    ui.painter().set(
        slot,
        Shape::Vec(vec![
            Shape::Rect(fill),
            Shape::rect_filled(foot, CornerRadius::ZERO, rule),
        ]),
    );

    if let Some((rect, galley)) = message {
        ui.painter().galley(rect.min, galley, Color32::PLACEHOLDER);
    }
    if off || danger {
        ui.ctx().accesskit_node_builder(id, |node| {
            if off {
                node.set_disabled();
            }
            if let Some(e) = field.error {
                node.set_description(e);
            }
        });
    }

    TextFieldResponse {
        changed: response.changed(),
        submitted: response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)),
        response,
    }
}
