//! The vertical tab: one open place in the rail beside the canvas.
//!
//! [`vertical_tab`] is a 24 row filling its width, with `SPACE_4` before its
//! label and `SPACE_2` after its last part, and `SPACE_2` between parts. The
//! label is the node, then " · " and the activity in the muted ink, on one line
//! and cut with "…" when the room runs out; the node is in the secondary ink at
//! rest and the primary one under the pointer. A dot (`DOT` across, secondary)
//! stands before the close control while the place holds an unsaved draft. The
//! close control is an 18 square holding a 12 "x" in the muted ink; it is drawn
//! only under the pointer, on the current place, or while the row or the control
//! has keyboard focus (so a keyboard or screen-reader user can close any place),
//! and takes the active wash and the primary ink when the pointer is on it or it
//! has focus, with a focus ring at the chip radius while it has focus.
//!
//! The current place is a cell joined to the canvas: the raised surface with a
//! 1px rule on its top, bottom and leading side, the leading corners at the
//! control radius, and no rule on the trailing side. The cell reaches 1px past
//! the row's trailing edge, so it covers the rule the rail ends in. Its node is
//! Medium in the primary ink and its activity is secondary.
//!
//! The caller holds which place is current and which holds a draft; nothing is
//! kept between frames. A click on the close control reports `close_clicked`
//! and not `clicked`. The node and activity are the caller's strings.

use egui::text::LayoutJob;
use egui::{Color32, CornerRadius, FontId, Rect, Sense, TextFormat, UiBuilder, Vec2};
use meridian_design::control::{HEIGHT_XS, ICON_XS};
use meridian_design::focus;
use meridian_design::spacing::ROW_GRID;
use meridian_design::typography::UI_SIZE;
use meridian_design::{semantic, Role};

use crate::icons::Icon;
use crate::theme::to_color32;
use crate::widgets::{
    cap_centred_galley_top, focus_ring_corners, medium_family, paint_open_cell, OpenSide,
};
use crate::MeridianUi;

/// One open place: the node it shows and the activity it is in.
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct VerticalTab<'a> {
    node: &'a str,
    activity: &'a str,
    current: bool,
    dirty: bool,
}

impl<'a> VerticalTab<'a> {
    /// A tab for `node` in `activity`. An empty activity shows the node alone.
    pub fn new(node: &'a str, activity: &'a str) -> Self {
        Self {
            node,
            activity,
            current: false,
            dirty: false,
        }
    }

    /// This is the current place.
    pub fn current(mut self, current: bool) -> Self {
        self.current = current;
        self
    }

    /// The place holds an unsaved draft: a dot is shown.
    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }
}

/// What a [`vertical_tab`] reported.
#[derive(Debug)]
pub struct VerticalTabResponse {
    /// The row's response.
    pub response: egui::Response,
    /// The row was clicked anywhere but its close control.
    pub clicked: bool,
    /// The close control was clicked. The caller closes the place.
    pub close_clicked: bool,
}

/// Draw a vertical tab across the available width.
pub fn vertical_tab(ui: &mut egui::Ui, tab: VerticalTab<'_>) -> VerticalTabResponse {
    let t = ui.tokens();
    let (rect, slot) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(0.0), ROW_GRID),
        Sense::hover(),
    );

    // The row is a child `Ui` whose own widget is the tab, so the close
    // control's node is the tab's child. The `Ui` records its box and sense
    // under its id when it is dropped, so it senses what the tab does and holds
    // exactly the row.
    let mut row = ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .id_salt(slot.id)
            .sense(Sense::click()),
    );
    row.advance_cursor_after_rect(rect);
    let mut response = row.response();
    response.rect = rect;
    let label = if tab.activity.is_empty() {
        tab.node.to_owned()
    } else {
        format!("{} · {}", tab.node, tab.activity)
    };
    let enabled = row.is_enabled();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, &label));
    row.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Tab);
        node.set_selected(tab.current);
    });

    // The control is shown, and takes the pointer, only under the pointer or
    // on the current place, and while the row or the control has keyboard
    // focus, so that a keyboard or screen-reader user can close any place.
    // What is hidden is not sensed, so a click on its place goes to the row.
    let hovered = response.contains_pointer();
    let close_id = response.id.with("close");
    let close_focused = row.memory(|m| m.has_focus(close_id));
    let close_shown = hovered || tab.current || response.has_focus() || close_focused;
    let close_rect = Rect::from_center_size(
        egui::pos2(rect.right() - t.space[2] - HEIGHT_XS / 2.0, rect.center().y),
        Vec2::splat(HEIGHT_XS),
    );
    let mut close_clicked = false;
    let mut close_hovered = false;
    let mut close_focus_ring = false;
    if close_shown {
        let close = row.interact(close_rect, close_id, Sense::click());
        close.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, "Close"));
        close_clicked = close.clicked();
        close_hovered = close.hovered() || close.has_focus();
        close_focus_ring = close.has_focus();
    }

    if row.is_rect_visible(rect) {
        paint(
            &row,
            rect,
            &tab,
            &Paint {
                hovered,
                focused: response.has_focus(),
                close_shown,
                close_hovered,
                close_focus_ring,
                close_rect,
            },
        );
    }

    VerticalTabResponse {
        clicked: response.clicked() && !close_clicked,
        close_clicked,
        response,
    }
}

struct Paint {
    hovered: bool,
    focused: bool,
    close_shown: bool,
    close_hovered: bool,
    close_focus_ring: bool,
    close_rect: Rect,
}

fn paint(ui: &egui::Ui, rect: Rect, tab: &VerticalTab<'_>, p: &Paint) {
    let t = ui.tokens();
    let dark = ui.visuals().dark_mode;
    let sem = semantic(dark);
    let c = to_color32;
    let neutral = sem.role(Role::Neutral);
    let corner = t.radius_control.round() as u8;
    let centre_y = rect.center().y;

    // The ground: the current place's cell, or the hover fill.
    if tab.current {
        // The cell reaches 1px past the trailing edge. Its rule is drawn one
        // pixel further on the trailing side and clipped away, leaving the
        // top, bottom and leading runs.
        let cell = Rect::from_min_max(rect.min, egui::pos2(rect.right() + 1.0, rect.bottom()));
        // A clip that ends at the row (a scroll area, a side panel) would cut
        // the cell's extra column: widen the parent's clip by that column and
        // no more, and hold the cell to it.
        let mut painter = ui.painter().clone();
        let parent = painter.clip_rect();
        painter.set_clip_rect(
            Rect::from_min_max(parent.min, egui::pos2(parent.max.x + 1.0, parent.max.y))
                .intersect(cell),
        );
        paint_open_cell(
            &painter,
            cell,
            leading_corners(corner),
            c(sem.surfaces.raised),
            c(sem.borders.subtle),
            OpenSide::Trailing,
        );
    } else if p.hovered {
        ui.painter()
            .rect_filled(rect, CornerRadius::ZERO, c(sem.rows.hover_background));
    }

    let painter = ui.painter_at(rect);

    // The close control claims the trailing end; the dot sits before it.
    let mut right = p.close_rect.left() - t.space[2];
    if tab.dirty {
        let centre = egui::pos2(right - t.dot / 2.0, centre_y);
        painter.circle_filled(centre, t.dot / 2.0, c(sem.text.secondary));
        right = centre.x - t.dot / 2.0 - t.space[2];
    }
    if p.close_shown {
        let ink = if p.close_hovered {
            painter.rect_filled(
                p.close_rect,
                CornerRadius::same(t.radius_chip.round() as u8),
                c(neutral.background.active),
            );
            sem.text.primary
        } else {
            sem.text.muted
        };
        if let Some(x) = Icon::by_name("x") {
            let at = Rect::from_center_size(p.close_rect.center(), Vec2::splat(ICON_XS));
            x.paint(&painter, at, c(ink));
        }
    }

    // The label: the node, then the activity, on one line cut to the room.
    let left = rect.left() + t.space[4];
    let regular = FontId::proportional(UI_SIZE);
    let (node_font, node_ink, activity_ink) = if tab.current {
        (
            FontId::new(UI_SIZE, medium_family(ui)),
            sem.text.primary,
            sem.text.secondary,
        )
    } else {
        (
            regular.clone(),
            if p.hovered {
                sem.text.primary
            } else {
                sem.text.secondary
            },
            sem.text.muted,
        )
    };
    let mut job = LayoutJob::single_section(
        tab.node.to_owned(),
        TextFormat::simple(node_font, c(node_ink)),
    );
    if !tab.activity.is_empty() {
        job.append(
            &format!(" · {}", tab.activity),
            0.0,
            TextFormat::simple(regular, c(activity_ink)),
        );
    }
    job.wrap.max_width = (right - left).max(0.0);
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    let galley = ui.painter().layout_job(job);
    let top = cap_centred_galley_top(ui.painter(), &galley, centre_y);
    painter.galley(egui::pos2(left, top), galley, Color32::PLACEHOLDER);

    if p.close_focus_ring {
        let r = focus::ring_radius(t.radius_chip).round() as u8;
        focus_ring_corners(ui, p.close_rect, CornerRadius::same(r));
    }
    if p.focused {
        let ring = if tab.current {
            leading_corners(focus::ring_radius(t.radius_control).round() as u8)
        } else {
            CornerRadius::ZERO
        };
        focus_ring_corners(ui, rect, ring);
    }
}

fn leading_corners(radius: u8) -> CornerRadius {
    CornerRadius {
        nw: radius,
        ne: 0,
        sw: radius,
        se: 0,
    }
}
