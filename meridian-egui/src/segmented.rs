//! The segmented control: mutually exclusive options in one ruled box.
//!
//! Each option is a label and, optionally, a keycap. The box is ruled 1px in the
//! neutral border, with its options divided by 1px rules; the selected option
//! takes the selection wash and its own rule. Only the box's outer corners
//! round: an inner edge, fill and ring are square. 24 tall when any option has
//! a keystroke (the keycap needs the room), otherwise 20.
//!
//! The caller holds which option is selected. [`Segmented::new`] takes its
//! index; one past the end selects nothing, and no options at all draws
//! nothing. A click on an available option, the selected one included, is
//! reported in [`SegmentedResponse::clicked`].
//!
//! An unavailable option ([`SegmentedOption::unavailable`]) keeps its place,
//! senses the pointer, reports no click and reports its rect when pointed at
//! ([`SegmentedResponse::pointed_unavailable`]) so the caller can anchor a
//! popover holding the reason. It takes no keyboard focus, and its
//! accessibility node is disabled with the reason as its description. An
//! available option takes focus: it is a radio button in a radio group.
//!
//! The text is the caller's: labels, keystrokes (`"G"`) and reasons are strings
//! it passes. Nothing here animates.
//!
//! Too narrow for its options, a control gives up label width first: each
//! option keeps its padding and keycap and only the labels shrink, the longest
//! first (a short label stays whole), cut with an ellipsis. Narrower than the padding and keycaps alone, it keeps those widths
//! and is clipped at the edges of the room it was given, taking no pointer past them; no option
//! overlaps another.

use egui::{Color32, CornerRadius, Rect, Sense, Stroke, StrokeKind, UiBuilder, Vec2};
use meridian_design::control::{HEIGHT_MD, HEIGHT_SM};
use meridian_design::focus;
use meridian_design::{semantic, Role};

use crate::button::label_galley;
use crate::key_chip::{control_key_rect, key_size, paint_key, KeyInk, KeySize};
use crate::theme::to_color32;
use crate::widgets::{cap_centred_galley_top, focus_ring_corners};
use crate::MeridianUi;

/// One option: a label, an optional keystroke, an optional reason it is
/// unavailable.
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct SegmentedOption<'a> {
    label: &'a str,
    keystroke: Option<&'a str>,
    reason: Option<&'a str>,
}

impl<'a> SegmentedOption<'a> {
    /// An option labelled `label`.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            keystroke: None,
            reason: None,
        }
    }

    /// The keystroke that selects this option, as the caller's string, drawn as
    /// a keycap at its trailing edge.
    pub fn keystroke(mut self, keystroke: &'a str) -> Self {
        self.keystroke = Some(keystroke);
        self
    }

    /// The option cannot be chosen now, for `reason` (the caller's sentence).
    pub fn unavailable(mut self, reason: &'a str) -> Self {
        self.reason = Some(reason);
        self
    }
}

/// A segmented control's configuration, drawn by [`segmented`].
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct Segmented<'a> {
    options: &'a [SegmentedOption<'a>],
    selected: usize,
}

impl<'a> Segmented<'a> {
    /// `options`, with the one at index `selected` selected. An index past the
    /// end selects nothing.
    pub fn new(options: &'a [SegmentedOption<'a>], selected: usize) -> Self {
        Self { options, selected }
    }

    /// 24 when any option has a keystroke, else 20.
    fn height(&self) -> f32 {
        if self.options.iter().any(|o| o.keystroke.is_some()) {
            HEIGHT_MD
        } else {
            HEIGHT_SM
        }
    }
}

/// What [`segmented`] hands back.
#[derive(Clone, Debug)]
pub struct SegmentedResponse {
    /// The whole box's response.
    pub response: egui::Response,
    /// The index of the option clicked, the selected one included. Never an
    /// unavailable one.
    pub clicked: Option<usize>,
    /// An unavailable option has the pointer over it: its index and its rect,
    /// for the caller to anchor the reason popover to.
    pub pointed_unavailable: Option<(usize, Rect)>,
}

/// An option's fixed width (its padding, its 1px border either side and its keycap) and
/// its label's natural width. The sheet draws each option with a transparent 1px
/// border inside `SPACE_4` of leading padding, so a label starts `SPACE_4 + 1` in;
/// a keycap takes the inset from the option's edge, and a keyless option pads its
/// trailing edge as it does its leading one.
fn parts(ui: &egui::Ui, o: &SegmentedOption<'_>) -> (f32, f32) {
    let t = ui.tokens();
    let key = o.keystroke.map(|k| key_size(ui, k, KeySize::Control).x);
    let (trailing, key_part) = match key {
        Some(k) => (t.keycap_inset_in_control, t.icon_label_gap + k),
        None => (lead(ui), 0.0),
    };
    let label = label_galley(ui, o.label, Color32::PLACEHOLDER, f32::INFINITY)
        .size()
        .x
        .ceil();
    (lead(ui) + key_part + trailing, label)
}

/// Where an option's label starts, from its edge: `SPACE_4` and the 1px border.
fn lead(ui: &egui::Ui) -> f32 {
    ui.tokens().space[4] + 1.0
}

/// The corners option `i` of `count` rounds: the box's outer ones.
fn corners_of(i: usize, count: usize, radius: u8) -> CornerRadius {
    let (first, last) = (i == 0, i + 1 == count);
    CornerRadius {
        nw: if first { radius } else { 0 },
        sw: if first { radius } else { 0 },
        ne: if last { radius } else { 0 },
        se: if last { radius } else { 0 },
    }
}

/// The widest a label may be so that the options fit in `room`: infinite when
/// they already do, whole pixels otherwise, zero when the fixed parts alone do
/// not fit.
fn label_cap(sizes: &[(f32, f32)], room: f32) -> f32 {
    let fixed: f32 = sizes.iter().map(|s| s.0).sum();
    let labels: f32 = sizes.iter().map(|s| s.1).sum();
    if fixed + labels <= room {
        return f32::INFINITY;
    }
    let left = (room - fixed).max(0.0);
    let mut ls: Vec<f32> = sizes.iter().map(|s| s.1).collect();
    ls.sort_by(f32::total_cmp);
    // Walk up the labels shortest first: those below the cap keep their width,
    // and the rest share what remains.
    let mut spent = 0.0;
    for (k, l) in ls.iter().enumerate() {
        let share = (left - spent) / (ls.len() - k) as f32;
        if *l > share {
            return share.floor();
        }
        spent += l;
    }
    f32::INFINITY
}

/// Draw a segmented control and report what was done to it.
///
/// It is as wide as its options and no wider than the width `ui` has left; a
/// label too long for that is cut with an ellipsis and its keycap keeps its
/// place.
pub fn segmented(ui: &mut egui::Ui, control: Segmented<'_>) -> SegmentedResponse {
    let count = control.options.len();
    if count == 0 {
        let (_, response) = ui.allocate_exact_size(Vec2::ZERO, Sense::hover());
        return SegmentedResponse {
            response,
            clicked: None,
            pointed_unavailable: None,
        };
    }
    let height = control.height();
    let sizes: Vec<(f32, f32)> = control.options.iter().map(|o| parts(ui, o)).collect();
    let room = ui.available_width().max(0.0);
    // Read before allocating: a box wider than the room stretches `max_rect`.
    let room_x = ui.max_rect().x_range();
    // Only the labels give way, the longest first: the room the fixed parts
    // leave is shared out by a cap on label width, so a short label stays whole.
    // Below the fixed parts the box keeps them and is clipped at the trailing
    // edge.
    let cap = label_cap(&sizes, room);
    let widths: Vec<f32> = sizes.iter().map(|(f, l)| f + l.min(cap)).collect();

    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(widths.iter().sum::<f32>(), height),
        Sense::hover(),
    );
    let mut x = rect.left();
    let rects: Vec<Rect> = widths
        .iter()
        .map(|w| {
            let r = Rect::from_min_size(egui::pos2(x, rect.top()), Vec2::new(*w, height));
            x += w;
            r
        })
        .collect();

    // The options sit in a child `Ui`, whose own accessibility node is the
    // radio group: the options are its children, with no container between.
    let mut child = ui.new_child(UiBuilder::new().max_rect(rect).id_salt(response.id));
    // A box wider than the room it was given is cut at the room's edges, and
    // what is cut takes no pointer: the clip goes on before the options sense.
    let clip = Rect::from_x_y_ranges(room_x, rect.y_range());
    child.set_clip_rect(child.clip_rect().intersect(clip));
    child
        .ctx()
        .accesskit_node_builder(child.unique_id(), |node| {
            node.set_role(egui::accesskit::Role::RadioGroup);
        });
    let mut clicked = None;
    let mut pointed = None;
    let mut options: Vec<egui::Response> = Vec::with_capacity(count);
    for (i, o) in control.options.iter().enumerate() {
        let off = o.reason.is_some();
        let sense = if off { Sense::hover() } else { Sense::click() };
        let r = child.interact(rects[i], response.id.with(("segmented", i)), sense);
        r.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::RadioButton,
                !off,
                i == control.selected,
                o.label,
            )
        });
        if let Some(reason) = o.reason {
            crate::button::mark_unavailable(child.ctx(), r.id, reason);
            if r.hovered() {
                pointed = Some((i, rects[i].intersect(child.clip_rect())));
            }
        } else if r.clicked() {
            clicked = Some(i);
        }
        options.push(r);
    }

    if ui.is_rect_visible(rect) {
        paint(&child, rect, &rects, &options, &control);
    }

    SegmentedResponse {
        response,
        clicked,
        pointed_unavailable: pointed,
    }
}

fn paint(
    ui: &egui::Ui,
    rect: Rect,
    rects: &[Rect],
    options: &[egui::Response],
    control: &Segmented<'_>,
) {
    let sem = semantic(ui.visuals().dark_mode);
    let n = sem.role(Role::Neutral);
    let c = to_color32;
    let t = ui.tokens();
    let painter = ui.painter();
    let count = rects.len();
    let radius = t.radius_control.round() as u8;

    // The washes, then the rules over them, so a fill never hides the box.
    for (i, o) in control.options.iter().enumerate() {
        let r = &options[i];
        let off = o.reason.is_some();
        let fill = if i == control.selected {
            Some(n.background.selected)
        } else if off {
            None
        } else if r.is_pointer_button_down_on() {
            Some(n.background.active)
        } else if r.hovered() {
            Some(n.background.hover)
        } else {
            None
        };
        if let Some(fill) = fill {
            painter.rect_filled(rects[i], corners_of(i, count, radius), c(fill));
        }
    }
    for r in &rects[1..] {
        painter.rect_filled(
            Rect::from_min_size(r.min, Vec2::new(1.0, r.height())),
            CornerRadius::ZERO,
            c(n.border.base),
        );
    }
    painter.rect_stroke(
        rect,
        CornerRadius::same(radius),
        Stroke::new(1.0, c(n.border.base)),
        StrokeKind::Inside,
    );
    if let Some(r) = rects.get(control.selected) {
        painter.rect_stroke(
            *r,
            corners_of(control.selected, count, radius),
            Stroke::new(1.0, c(n.border.selected)),
            StrokeKind::Inside,
        );
    }

    for (i, o) in control.options.iter().enumerate() {
        let r = rects[i];
        let off = o.reason.is_some();
        if options[i].has_focus() && !off {
            let ring = focus::ring_radius(t.radius_control).round() as u8;
            focus_ring_corners(ui, r, corners_of(i, count, ring));
        }
        // The sheet's `.on` out-specifies `.off`: a selected option keeps its ink.
        let label_ink = if i == control.selected {
            c(n.foreground.selected)
        } else if off {
            c(sem.text.disabled)
        } else {
            c(sem.tabs.foreground)
        };
        let mut right = r.right() - lead(ui);
        if let Some(k) = o.keystroke {
            let size = key_size(ui, k, KeySize::Control);
            let key_rect = control_key_rect(ui, r, size);
            let key_ink = if off { KeyInk::Dimmed } else { KeyInk::Plain };
            paint_key(ui, key_rect, k, KeySize::Control, key_ink);
            right = key_rect.left() - t.icon_label_gap;
        }
        let x = r.left() + lead(ui);
        let galley = label_galley(ui, o.label, label_ink, (right - x).max(0.0));
        let top = cap_centred_galley_top(ui.painter(), &galley, r.center().y);
        painter.galley(egui::pos2(x, top), galley, label_ink);
    }
}
