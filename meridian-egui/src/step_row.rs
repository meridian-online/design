//! The step row and the thin step mark: the spine's list and its thin form.
//!
//! A step row is the list row at 24 ([`crate::list_row()`]), filling the width it
//! is given. At its leading edge is a graph gutter, `GUTTER_PAD + lanes ×
//! GUTTER_LANE + GUTTER_PAD` wide, where the row's state mark sits on its lane
//! as a commit graph draws it, over the lanes that pass, arrive and leave. Then,
//! `SPACE_3` apart: an optional fold control (a 12 chevron and, when folded, the
//! member count), the name, filling and cut with an ellipsis, what the row
//! produced, right-aligned, and for a step a timeline column and a duration.
//! `SPACE_4` trails the row.
//!
//! A row too narrow for all of that keeps its gutter, its mark and its fold
//! control, and gives up the rest least essential first: when the name would
//! be squeezed below four ems (or its own width, if shorter), the timeline and
//! duration go, then the output, and the name is cut with an ellipsis to what
//! is left. No two parts are drawn over each other or over the gutter.
//!
//! The row has two marks of place. The cursor ([`StepRow::cursor`]) is the list
//! row's: where the keys act, a fill and a bar. Focused ([`StepRow::focused`])
//! is the node the canvas shows: the selection wash with a 1px inside rule.
//! A row with both takes the wash, the rule and the bar. Hover and pressed are
//! the list row's.
//!
//! The caller holds everything, the gutter included: [`Gutter::lanes`] is the
//! list's widest row, so every row's gutter is one width, and a lane at or past
//! it is ignored. A [`Timeline`] is clamped to the run's span and put in order;
//! one with a NaN end draws no bar. The fold control is its own target: a click
//! on it reports `fold_clicked`, not `clicked`.
//!
//! The row's accessibility node is a `Row` named `"<name>, <state word>"`,
//! selected while focused. The fold control is a `Button` named by the row's
//! name, expanded while unfolded.
//!
//! [`step_mark`] is the thin form: the mark alone in a `STEP_MARK_CELL` square,
//! with the focused treatment and the hover fill.
//!
//! Not drawn: the sheet's output, count and duration use tabular figures, but
//! egui 0.35's `TextFormat` has no OpenType feature switch, so their digits
//! keep their proportional widths.

use egui::epaint::CubicBezierShape;
use egui::{Color32, FontId, Pos2, Rect, Sense, Shape, Stroke, StrokeKind, Vec2};
use meridian_design::chrome::{INK_DARK, INK_LIGHT};
// The fold chevron's size is not in `Tokens`, so it is read from `control`.
use meridian_design::control::ICON_XS;
use meridian_design::marks::{glyph, word, MarkShape};
use meridian_design::typography::UI_SIZE;
use meridian_design::viz::STATUS;
use meridian_design::{radius, semantic};

pub use meridian_design::marks::StepMark;

use crate::button::label_galley;
use crate::icons::Icon;
use crate::list_row::{list_row, paint_wash, ListRow, RowHeight};
use crate::theme::to_color32;
use crate::widgets::{cap_centred_galley_top, mode_of};
use crate::{MeridianUi, Mode, Tokens};

/// The graph edges at one row, by lane. Lane `0` is at the leading edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gutter<'a> {
    /// How many lanes the list's widest row uses, so every row's gutter is one
    /// width. A lane at or past it is ignored.
    pub lanes: u8,
    /// The lane this row's mark sits on; `None` for a row with no node.
    pub node: Option<u8>,
    /// Lanes passing this row without stopping.
    pub through: &'a [u8],
    /// Lanes arriving at this row's node from above.
    pub from_above: &'a [u8],
    /// Lanes leaving this row's node for below.
    pub to_below: &'a [u8],
}

/// Where a step ran within the run, as fractions `0.0..=1.0` of the run's span.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Timeline {
    pub start: f32,
    pub end: f32,
}

impl Timeline {
    /// The span clamped to `0..=1` and put in order; `None` when either end is
    /// NaN.
    fn span(self) -> Option<(f32, f32)> {
        if self.start.is_nan() || self.end.is_nan() {
            return None;
        }
        let (a, b) = (self.start.clamp(0.0, 1.0), self.end.clamp(0.0, 1.0));
        Some((a.min(b), a.max(b)))
    }
}

/// Whether a group's members are hidden under its row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fold {
    /// The members are hidden; the row shows how many there are.
    Folded { members: usize },
    /// The members are shown beneath the row.
    Unfolded,
}

/// What a row stands for. A table draws its name in the secondary ink and has
/// no timeline or duration, though it keeps their columns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepEntry {
    Step,
    Table,
}

/// One step row's configuration. Built with [`StepRow::new`] and the
/// `#[must_use]` builders, then drawn by [`step_row`].
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct StepRow<'a> {
    entry: StepEntry,
    gutter: Gutter<'a>,
    mark: StepMark,
    label: &'a str,
    output: Option<&'a str>,
    timeline: Option<(Timeline, &'a str)>,
    fold: Option<Fold>,
    focused: bool,
    cursor: bool,
}

impl<'a> StepRow<'a> {
    /// A row for `entry`, named `label`, in state `mark`, with `gutter`'s edges.
    pub fn new(entry: StepEntry, gutter: Gutter<'a>, mark: StepMark, label: &'a str) -> Self {
        Self {
            entry,
            gutter,
            mark,
            label,
            output: None,
            timeline: None,
            fold: None,
            focused: false,
            cursor: false,
        }
    }

    /// What the row produced, in the caller's words ("19,028 rows").
    pub fn output(mut self, text: &'a str) -> Self {
        self.output = Some(text);
        self
    }

    /// Where the step ran within the run, and how long it took in the caller's
    /// words. A table row draws neither.
    pub fn timeline(mut self, timeline: Timeline, duration: &'a str) -> Self {
        self.timeline = Some((timeline, duration));
        self
    }

    /// The row heads a group: a fold control, and the member count while
    /// folded.
    pub fn fold(mut self, fold: Fold) -> Self {
        self.fold = Some(fold);
        self
    }

    /// The row is the node the canvas shows: the selection wash.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// The row is under the caller's cursor: where the keys act.
    pub fn cursor(mut self, cursor: bool) -> Self {
        self.cursor = cursor;
        self
    }
}

/// What [`step_row`] hands back.
#[derive(Clone, Debug)]
pub struct StepRowResponse {
    /// The row's own response; its `rect` is the whole row.
    pub response: egui::Response,
    /// The row was clicked anywhere but its fold control.
    pub clicked: bool,
    /// The fold control was clicked. The caller flips the fold.
    pub fold_clicked: bool,
}

/// Draw a step row across the width `ui` has left and report what was clicked.
pub fn step_row(ui: &mut egui::Ui, row: StepRow<'_>) -> StepRowResponse {
    let list = ListRow::new(RowHeight::Grid)
        .selected(row.cursor)
        .wash(row.focused);
    let response = list_row(ui, list, |_, _| ()).response;
    let rect = response.rect;

    let label = format!("{}, {}", row.label, word(row.mark));
    let enabled = ui.is_enabled();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, enabled, &label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_selected(row.focused);
    });

    let cols = Columns::of(ui, rect, &row);

    let mut fold_clicked = false;
    if let (Some(fold), Some((left, right))) = (row.fold, cols.fold) {
        let fold_rect =
            Rect::from_x_y_ranges(left..=right.max(left), rect.y_range()).intersect(rect);
        let fold_response = ui.interact(fold_rect, response.id.with("fold"), Sense::click());
        let name = row.label;
        fold_response
            .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, name));
        ui.ctx().accesskit_node_builder(fold_response.id, |node| {
            node.set_expanded(fold == Fold::Unfolded);
        });
        fold_clicked = fold_response.clicked();
    }

    if ui.is_rect_visible(rect) {
        paint_row(ui, rect, &row, &cols);
    }

    StepRowResponse {
        clicked: response.clicked(),
        fold_clicked,
        response,
    }
}

/// The thin form: `mark` alone in a `STEP_MARK_CELL` square cell, in the
/// selection wash with its rule while `focused`, and in the hover fill under the
/// pointer otherwise. Its accessibility node is named by the state word and
/// selected while focused.
pub fn step_mark(ui: &mut egui::Ui, mark: StepMark, focused: bool) -> egui::Response {
    let t = ui.tokens();
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(t.step_mark_cell), Sense::click());
    let enabled = ui.is_enabled();
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, enabled, word(mark)));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_selected(focused);
    });

    if ui.is_rect_visible(rect) {
        let mode = mode_of(ui);
        let rows = &semantic(mode.is_dark()).rows;
        let painter = ui.painter();
        if focused {
            paint_wash(painter, rect, mode.is_dark());
        } else if response.hovered() {
            painter.rect_filled(rect, radius::NONE, to_color32(rows.hover_background));
        }
        paint_mark(painter, t, rect.center(), mark, mode);
    }
    response
}

/// Draw `mark`'s shape in its ink, centred on `centre`.
///
/// An outline (the ring, and the rings under the half disc and the centre dot)
/// is `step_mark` across at its stroke's centre, with a `step_mark_stroke`
/// stroke. The solid shapes and the centre dot take their own sizes:
/// `step_mark_disc_radius`, `step_mark_diamond_reach` (the diamond's
/// half-diagonal) and `step_mark_dot_radius`.
pub(crate) fn paint_mark(
    painter: &egui::Painter,
    t: &Tokens,
    centre: Pos2,
    mark: StepMark,
    mode: Mode,
) {
    let g = glyph(mark, mode.is_dark());
    let ink = to_color32(g.ink);
    let fill = to_color32(g.fill.unwrap_or(g.ink));
    let r = t.step_mark / 2.0;
    let ring = || {
        painter.circle_stroke(centre, r, Stroke::new(t.step_mark_stroke, ink));
    };
    match g.shape {
        MarkShape::Ring => ring(),
        MarkShape::HalfDisc => {
            ring();
            // From the top, round the right, to the foot: clockwise on screen.
            const STEPS: usize = 16;
            let points = (0..=STEPS)
                .map(|i| {
                    let a = -std::f32::consts::FRAC_PI_2
                        + std::f32::consts::PI * i as f32 / STEPS as f32;
                    centre + r * Vec2::new(a.cos(), a.sin())
                })
                .collect();
            painter.add(Shape::convex_polygon(points, ink, Stroke::NONE));
        }
        MarkShape::Disc => {
            painter.circle_filled(centre, t.step_mark_disc_radius, fill);
        }
        MarkShape::Diamond => {
            let d = t.step_mark_diamond_reach;
            let points = vec![
                centre - Vec2::new(0.0, d),
                centre + Vec2::new(d, 0.0),
                centre + Vec2::new(0.0, d),
                centre - Vec2::new(d, 0.0),
            ];
            painter.add(Shape::convex_polygon(points, fill, Stroke::NONE));
        }
        MarkShape::RingWithCentre => {
            ring();
            painter.circle_filled(centre, t.step_mark_dot_radius, ink);
        }
    }
}

/// How far a mark reaches from its centre: the ring's outer edge. An edge stops
/// here rather than running under a hollow mark.
fn mark_reach(t: &Tokens) -> f32 {
    (t.step_mark + t.step_mark_stroke) / 2.0
}

/// The narrowest a name is squeezed to before a column gives way to it: four
/// ems of UI text, room for a short word and its ellipsis. A shorter name
/// needs only its own width.
const NAME_FLOOR: f32 = 4.0 * UI_SIZE;

/// Where each part of a row goes, for the width it was given.
///
/// The gutter and its mark are always placed; then, at the leading edge, the
/// fold control and the name, and at the trailing edge the timeline and
/// duration, then the output. When the name would be squeezed below
/// [`NAME_FLOOR`] (or its own width, if shorter), the trailing columns give
/// way, least essential first: the timeline and duration, then the output.
/// What is left of the room goes to the name, cut with an ellipsis. No part is
/// placed over another, and none over the gutter; a row too narrow even for
/// the gutter and fold control is clipped at its own edges.
struct Columns {
    /// The gutter's trailing edge.
    gutter_right: f32,
    /// The fold control's span, cut at the row's trailing padding.
    fold: Option<(f32, f32)>,
    /// Where the name starts.
    name_left: f32,
    /// The widest the name may be drawn.
    name_room: f32,
    /// The output's span, when it keeps its column.
    output: Option<(f32, f32)>,
    /// Where the timeline column starts, when it keeps its column.
    timeline_left: Option<f32>,
    duration_right: f32,
}

impl Columns {
    fn of(ui: &egui::Ui, rect: Rect, row: &StepRow<'_>) -> Self {
        let t = ui.tokens();
        let gap = t.space[3];
        let gutter_right =
            rect.left() + 2.0 * t.gutter_pad + f32::from(row.gutter.lanes) * t.gutter_lane;
        let lead = gutter_right + gap;
        let duration_right = rect.right() - t.space[4];
        let fold = row.fold.map(|f| (lead, fold_end(ui, lead, f)));
        let name_left = fold.map_or(lead, |(_, end)| end + gap);
        let natural = label_galley(ui, row.label, Color32::PLACEHOLDER, f32::INFINITY)
            .size()
            .x;
        let floor = natural.min(NAME_FLOOR);

        let output_width = row.output.map(|o| {
            label_galley(ui, o, Color32::PLACEHOLDER, f32::INFINITY)
                .size()
                .x
        });
        // The space the name and the output need before a column gives way.
        let needs = |output: Option<f32>| name_left + floor + output.map_or(0.0, |w| gap + w);

        // The timeline column goes first: it stays only while the name's floor
        // and the whole output fit before it. The test reads each row's own
        // parts, so in a band of widths a row with a wider fold control,
        // output or name can give up the column a little sooner than its
        // neighbours, and the list's lattice breaks there.
        let timeline_left = duration_right - t.duration_width - gap - t.timeline_width;
        let timeline_left = (needs(output_width) + gap <= timeline_left).then_some(timeline_left);
        let mut name_right = timeline_left.map_or(duration_right, |l| l - gap);

        // Then the output, once the name's floor no longer fits beside it.
        let output = output_width
            .filter(|w| needs(Some(*w)) <= name_right)
            .map(|w| (name_right - w, name_right));
        if let Some((left, _)) = output {
            name_right = left - gap;
        }
        Self {
            gutter_right,
            fold: fold.map(|(a, b)| (a, b.min(duration_right))),
            name_left,
            name_room: (name_right - name_left).max(0.0),
            output,
            timeline_left,
            duration_right,
        }
    }
}

/// The trailing edge of the fold control that starts at `left`: the chevron
/// and, while folded, `SPACE_1` and the member count.
fn fold_end(ui: &egui::Ui, left: f32, fold: Fold) -> f32 {
    match fold {
        Fold::Unfolded => left + ICON_XS,
        Fold::Folded { members } => {
            left + ICON_XS
                + ui.tokens().space[1]
                + count_galley(ui, members, Color32::PLACEHOLDER).size().x
        }
    }
}

fn count_galley(ui: &egui::Ui, members: usize, ink: Color32) -> std::sync::Arc<egui::Galley> {
    ui.painter()
        .layout_no_wrap(members.to_string(), FontId::proportional(UI_SIZE), ink)
}

fn paint_row(ui: &egui::Ui, rect: Rect, row: &StepRow<'_>, cols: &Columns) {
    let t = ui.tokens();
    let mode = mode_of(ui);
    let dark = mode.is_dark();
    let sem = semantic(dark);
    let c = to_color32;
    let painter = ui.painter_at(rect);
    let centre_y = rect.center().y;

    paint_gutter(
        &painter,
        t,
        rect,
        cols.gutter_right,
        &row.gutter,
        row.mark,
        mode,
    );

    // The fold control, cut at the row's trailing padding.
    if let (Some(fold), Some((left, right))) = (row.fold, cols.fold) {
        let painter = painter.with_clip_rect(painter.clip_rect().intersect(Rect::from_x_y_ranges(
            left..=right.max(left),
            rect.y_range(),
        )));
        let muted = c(sem.text.muted);
        let name = match fold {
            Fold::Folded { .. } => "chevron-right",
            Fold::Unfolded => "chevron-down",
        };
        if let Some(chevron) = Icon::by_name(name) {
            let at = Rect::from_center_size(
                egui::pos2(left + ICON_XS / 2.0, centre_y),
                Vec2::splat(ICON_XS),
            );
            chevron.paint(&painter, at, muted);
        }
        if let Fold::Folded { members } = fold {
            let galley = count_galley(ui, members, muted);
            let top = cap_centred_galley_top(ui.painter(), &galley, centre_y);
            painter.galley(
                egui::pos2(left + ICON_XS + t.space[1], top),
                galley,
                Color32::PLACEHOLDER,
            );
        }
    }

    // The output, right-aligned in its column, when it keeps one.
    if let (Some(output), Some((left, right))) = (row.output, cols.output) {
        let galley = label_galley(ui, output, c(sem.text.secondary), right - left);
        let top = cap_centred_galley_top(ui.painter(), &galley, centre_y);
        painter.galley(
            egui::pos2(right - galley.size().x, top),
            galley,
            Color32::PLACEHOLDER,
        );
    }

    // The name takes what the columns leave, cut with an ellipsis; with no
    // room at all it is not drawn, so a lone ellipsis cannot run past its room.
    if cols.name_room > 0.0 {
        let name_ink = match row.entry {
            StepEntry::Step => sem.text.primary,
            StepEntry::Table => sem.text.secondary,
        };
        let galley = label_galley(ui, row.label, c(name_ink), cols.name_room);
        if galley.size().x <= cols.name_room + 0.5 {
            let top = cap_centred_galley_top(ui.painter(), &galley, centre_y);
            painter.galley(
                egui::pos2(cols.name_left, top),
                galley,
                Color32::PLACEHOLDER,
            );
        }
    }

    // The timeline column: a gridline at the start of each half, on every row so
    // the list reads as one lattice; then the step's bar and its duration. A
    // row too narrow for the column draws none of it.
    let Some(timeline_left) = cols.timeline_left else {
        return;
    };
    let gridline = c(if dark { INK_DARK } else { INK_LIGHT }.gridline);
    for x in [timeline_left, timeline_left + t.timeline_width / 2.0] {
        painter.rect_filled(
            Rect::from_x_y_ranges(x..=x + 1.0, rect.y_range()),
            radius::NONE,
            gridline,
        );
    }
    let Some((timeline, duration)) = row.timeline.filter(|_| row.entry == StepEntry::Step) else {
        return;
    };
    if let Some((a, b)) = timeline.span() {
        let right_edge = timeline_left + t.timeline_width;
        // The sheet's `min-width: var(--m-space-1)` on the bar.
        let min = t.space[1];
        let mut x0 = timeline_left + a * t.timeline_width;
        let mut x1 = timeline_left + b * t.timeline_width;
        if x1 - x0 < min {
            x1 = (x0 + min).min(right_edge);
            x0 = x1 - min;
        }
        let (fill, rule) = match row.mark {
            StepMark::Failed => (STATUS.critical, STATUS.critical),
            StepMark::Running => (sem.feedback.progress_bar, sem.feedback.progress_bar),
            _ => (sem.borders.strong, sem.borders.control),
        };
        let half = t.timeline_bar_height / 2.0;
        painter.rect(
            Rect::from_x_y_ranges(x0..=x1, centre_y - half..=centre_y + half),
            radius::CHIP,
            c(fill),
            Stroke::new(1.0, c(rule)),
            StrokeKind::Inside,
        );
    }
    let galley = label_galley(ui, duration, c(sem.text.muted), t.duration_width);
    let top = cap_centred_galley_top(ui.painter(), &galley, centre_y);
    painter.galley(
        egui::pos2(cols.duration_right - galley.size().x, top),
        galley,
        Color32::PLACEHOLDER,
    );
}

/// The gutter: lanes passing, arriving and leaving, then the mark over them on
/// its lane. Lanes at or past `gutter.lanes` are ignored, and so is everything
/// that meets a node past it.
fn paint_gutter(
    painter: &egui::Painter,
    t: &Tokens,
    rect: Rect,
    gutter_right: f32,
    gutter: &Gutter<'_>,
    mark: StepMark,
    mode: Mode,
) {
    let live = |l: &&u8| **l < gutter.lanes;
    let x = |l: u8| rect.left() + t.gutter_pad + f32::from(l) * t.gutter_lane + t.gutter_lane / 2.0;
    let stroke = Stroke::new(
        t.gutter_edge_stroke,
        to_color32(semantic(mode.is_dark()).borders.strong),
    );
    let (top, mid, foot) = (rect.top(), rect.center().y, rect.bottom());
    // The gutter clips its own edges, so no curve can bow into the columns.
    let own = Rect::from_x_y_ranges(rect.left()..=gutter_right, rect.y_range());
    let painter = painter.with_clip_rect(painter.clip_rect().intersect(own));

    for &l in gutter.through.iter().filter(live) {
        painter.line_segment([egui::pos2(x(l), top), egui::pos2(x(l), foot)], stroke);
    }
    let Some(node) = gutter.node.filter(|n| *n < gutter.lanes) else {
        return;
    };
    let centre = egui::pos2(x(node), mid);
    let reach = mark_reach(t);
    for &l in gutter.from_above.iter().filter(live) {
        if l == node {
            painter.line_segment(
                [egui::pos2(x(l), top), centre - Vec2::new(0.0, reach)],
                stroke,
            );
        } else {
            let curve = [
                egui::pos2(x(l), top),
                egui::pos2(x(l), mid),
                egui::pos2(x(node), top),
                centre,
            ];
            let points = trimmed_end(flatten(curve), centre, reach);
            painter.add(Shape::line(points, stroke));
        }
    }
    for &l in gutter.to_below.iter().filter(live) {
        if l == node {
            painter.line_segment(
                [centre + Vec2::new(0.0, reach), egui::pos2(x(l), foot)],
                stroke,
            );
        } else {
            let curve = [
                centre,
                egui::pos2(x(node), foot),
                egui::pos2(x(l), mid),
                egui::pos2(x(l), foot),
            ];
            let mut points = flatten(curve);
            points.reverse();
            let mut points = trimmed_end(points, centre, reach);
            points.reverse();
            painter.add(Shape::line(points, stroke));
        }
    }
    paint_mark(&painter, t, centre, mark, mode);
}

fn flatten(points: [Pos2; 4]) -> Vec<Pos2> {
    /// The curve is drawn within this many pixels of its true path.
    const TOLERANCE: f32 = 0.1;
    CubicBezierShape::from_points_stroke(points, false, Color32::TRANSPARENT, Stroke::NONE)
        .flatten(Some(TOLERANCE))
}

/// `points` up to where they first come within `reach` of `centre`, ending
/// exactly on that circle.
fn trimmed_end(points: Vec<Pos2>, centre: Pos2, reach: f32) -> Vec<Pos2> {
    let mut out: Vec<Pos2> = Vec::with_capacity(points.len());
    for p in points {
        if p.distance(centre) >= reach {
            out.push(p);
            continue;
        }
        if let Some(&outside) = out.last() {
            // Bisect the segment that crosses the circle.
            let (mut lo, mut hi) = (outside, p);
            for _ in 0..24 {
                let m = lo.lerp(hi, 0.5);
                if m.distance(centre) >= reach {
                    lo = m;
                } else {
                    hi = m;
                }
            }
            out.push(lo);
        }
        break;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_span_is_clamped_ordered_and_none_for_nan() {
        let span = |start, end| Timeline { start, end }.span();
        assert_eq!(span(-0.5, 0.2), Some((0.0, 0.2)));
        assert_eq!(span(0.9, 1.7), Some((0.9, 1.0)));
        assert_eq!(span(0.6, 0.4), Some((0.4, 0.6)));
        assert_eq!(span(f32::NAN, 0.5), None);
        assert_eq!(span(0.5, f32::NAN), None);
        assert_eq!(span(f32::NEG_INFINITY, f32::INFINITY), Some((0.0, 1.0)));
    }

    #[test]
    fn a_trimmed_line_ends_on_the_circle() {
        let c = egui::pos2(0.0, 10.0);
        let out = trimmed_end(vec![egui::pos2(0.0, 0.0), c], c, 4.0);
        assert_eq!(out.len(), 2);
        assert!((out[1].distance(c) - 4.0).abs() < 0.01, "{out:?}");
    }
}
