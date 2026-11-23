//! The step row and the thin step mark, read out of the paint list and the
//! accessibility tree: the state marks and their inks, the gutter's lanes and
//! its one width, the columns after it, the timeline bar's clamped geometry,
//! the fold control, the list row's states plus the focused wash, and a long
//! name cut before the columns that follow it.

mod common;

use common::{ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::epaint::{CircleShape, PathShape};
use egui::{Color32, Pos2, Rect, Stroke, StrokeKind};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::chrome::{INK_DARK, INK_LIGHT};
use meridian_design::control::{
    DURATION_WIDTH, HEIGHT_MD, ROW_BAR_WIDTH, STEP_MARK, STEP_MARK_CELL, STEP_MARK_DIAMOND_REACH,
    STEP_MARK_DISC_RADIUS, STEP_MARK_DOT_RADIUS, STEP_MARK_STROKE, TIMELINE_BAR_HEIGHT,
    TIMELINE_WIDTH,
};
use meridian_design::marks::{glyph, word, STEP_MARKS};
use meridian_design::radius;
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{SPACE_1, SPACE_3, SPACE_4};
use meridian_design::viz::STATUS;
use meridian_egui::{
    step_mark, step_row, theme, Fold, Gutter, Mode, StepEntry, StepMark, StepRow, StepRowResponse,
    Timeline,
};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const WIDTH: f32 = 420.0;
const NAME: &str = "join_installs";
const OUTPUT: &str = "19,028 rows";
const DURATION: &str = "1.6 s";
/// The edge line weight the brief states.
const EDGE: f32 = 1.25;

#[derive(Clone, Debug)]
struct Spec {
    entry: StepEntry,
    lanes: u8,
    node: Option<u8>,
    through: Vec<u8>,
    from_above: Vec<u8>,
    to_below: Vec<u8>,
    mark: StepMark,
    label: &'static str,
    output: Option<&'static str>,
    timeline: Option<(f32, f32)>,
    fold: Option<Fold>,
    focused: bool,
    cursor: bool,
}

impl Spec {
    fn new() -> Self {
        Self {
            entry: StepEntry::Step,
            lanes: 2,
            node: Some(0),
            through: vec![1],
            from_above: vec![0],
            to_below: vec![0],
            mark: StepMark::Ok,
            label: NAME,
            output: Some(OUTPUT),
            timeline: Some((0.28, 0.52)),
            fold: None,
            focused: false,
            cursor: false,
        }
    }
}

#[derive(Default)]
struct Out {
    rect: Option<Rect>,
    clicked: bool,
    fold_clicked: bool,
}

fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(WIDTH + 40.0, 80.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(WIDTH, 40.0), |ui| {
                    let gutter = Gutter {
                        lanes: spec.lanes,
                        node: spec.node,
                        through: &spec.through,
                        from_above: &spec.from_above,
                        to_below: &spec.to_below,
                    };
                    let mut row = StepRow::new(spec.entry, gutter, spec.mark, spec.label)
                        .focused(spec.focused)
                        .cursor(spec.cursor);
                    if let Some(o) = spec.output {
                        row = row.output(o);
                    }
                    if let Some((start, end)) = spec.timeline {
                        row = row.timeline(Timeline { start, end }, DURATION);
                    }
                    if let Some(f) = spec.fold {
                        row = row.fold(f);
                    }
                    let r: StepRowResponse = step_row(ui, row);
                    out.rect = Some(r.response.rect);
                    out.clicked |= r.clicked;
                    out.fold_clicked |= r.fold_clicked;
                });
            },
            Out::default(),
        );
    harness.run();
    harness
}

fn row_rect(h: &Harness<'_, Out>) -> Rect {
    h.state().rect.expect("the row was drawn")
}

fn click_at(h: &mut Harness<'_, Out>, pos: Pos2) {
    h.event(egui::Event::PointerMoved(pos));
    h.run();
    for pressed in [true, false] {
        h.event(egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        });
        h.run();
    }
}

// ─── reading the shapes ─────────────────────────────────────────────────────

fn flat(shapes: &[egui::epaint::ClippedShape]) -> Vec<egui::Shape> {
    fn walk(shape: &egui::Shape, out: &mut Vec<egui::Shape>) {
        match shape {
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
            s => out.push(s.clone()),
        }
    }
    let mut out = Vec::new();
    for c in shapes {
        walk(&c.shape, &mut out);
    }
    out
}

fn circles<S>(h: &Harness<'_, S>) -> Vec<CircleShape> {
    flat(&h.output().shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Circle(c) => Some(c),
            _ => None,
        })
        .collect()
}

fn paths<S>(h: &Harness<'_, S>) -> Vec<PathShape> {
    flat(&h.output().shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Path(p) => Some(p),
            _ => None,
        })
        .collect()
}

fn segments<S>(h: &Harness<'_, S>) -> Vec<([Pos2; 2], Stroke)> {
    flat(&h.output().shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::LineSegment { points, stroke } => Some((points, stroke)),
            _ => None,
        })
        .collect()
}

fn path_colour(p: &PathShape) -> Color32 {
    match p.stroke.color {
        egui::epaint::ColorMode::Solid(c) => c,
        _ => Color32::TRANSPARENT,
    }
}

/// Every text the frame drew: its string, its box and its ink.
fn texts<S>(h: &Harness<'_, S>) -> Vec<(String, Rect, Color32)> {
    flat(&h.output().shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Text(t) => {
                let s: String = t.galley.rows.iter().map(|r| r.text()).collect();
                let colour = t.galley.job.sections[0].format.color;
                Some((s, t.galley.rect.translate(t.pos.to_vec2()), colour))
            }
            _ => None,
        })
        .collect()
}

fn text<S>(h: &Harness<'_, S>, starts: &str) -> (String, Rect, Color32) {
    texts(h)
        .into_iter()
        .find(|(s, _, _)| s.starts_with(starts))
        .unwrap_or_else(|| panic!("{starts:?} was not drawn: {:?}", texts(h)))
}

/// The gutter's edges: the 1.25 strokes, straight or curved, left of `right`.
fn edges<S>(h: &Harness<'_, S>, right: f32) -> (Vec<[Pos2; 2]>, Vec<Vec<Pos2>>) {
    let lines = segments(h)
        .into_iter()
        .filter(|(p, s)| near(s.width, EDGE) && p[0].x <= right && p[1].x <= right)
        .map(|(p, _)| p)
        .collect();
    let curves = paths(h)
        .into_iter()
        .filter(|p| near(p.stroke.width, EDGE) && !p.closed)
        .map(|p| p.points)
        .collect();
    (lines, curves)
}

// ─── the columns, as the brief states them ─────────────────────────────────

fn gutter_width(lanes: u8) -> f32 {
    5.0 + f32::from(lanes) * 12.0 + 5.0
}

fn lane_x(row: Rect, lane: u8) -> f32 {
    row.left() + 5.0 + f32::from(lane) * 12.0 + 6.0
}

fn duration_right(row: Rect) -> f32 {
    row.right() - SPACE_4
}

fn timeline_left(row: Rect) -> f32 {
    duration_right(row) - DURATION_WIDTH - SPACE_3 - TIMELINE_WIDTH
}

fn output_right(row: Rect) -> f32 {
    timeline_left(row) - SPACE_3
}

/// The timeline bar: the ruled rect with the chip corner.
fn bars_in_timeline(rects: &[Painted], row: Rect) -> Vec<Painted> {
    let left = timeline_left(row);
    rects
        .iter()
        .filter(|r| {
            r.is_stroked()
                && r.radius == egui::CornerRadius::same(radius::CHIP as u8)
                && r.rect.left() >= left - 0.01
                && r.rect.right() <= left + TIMELINE_WIDTH + 0.01
        })
        .copied()
        .collect()
}

fn on_row(rects: &[Painted], row: Rect) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| r.fill != Color32::TRANSPARENT && near_rect(r.rect, row))
        .copied()
        .collect()
}

fn leading_bars(rects: &[Painted], row: Rect) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| {
            r.fill != Color32::TRANSPARENT
                && near(r.rect.width(), ROW_BAR_WIDTH)
                && near(r.rect.left(), row.left())
                && near(r.rect.height(), row.height())
        })
        .copied()
        .collect()
}

// ─── the state marks ────────────────────────────────────────────────────────

struct MarkOut {
    rect: Option<Rect>,
}

fn mark_drawn(mode: Mode, mark: StepMark, focused: bool) -> Harness<'static, MarkOut> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(80.0, 80.0))
        .build_ui_state(
            move |ui, out: &mut MarkOut| {
                theme::apply(ui.ctx(), mode);
                out.rect = Some(step_mark(ui, mark, focused).rect);
            },
            MarkOut { rect: None },
        );
    harness.run();
    harness
}

#[test]
fn each_state_draws_its_shape_in_its_ink() {
    let ring_r = STEP_MARK / 2.0;
    // The sheet's sizes, as the tokens hold them.
    let (disc_r, diamond_r, dot_r) = (4.0, 4.8, 1.4);
    assert_eq!(
        (
            STEP_MARK_DISC_RADIUS,
            STEP_MARK_DIAMOND_REACH,
            STEP_MARK_DOT_RADIUS
        ),
        (disc_r, diamond_r, dot_r)
    );
    for mode in MODES {
        for mark in STEP_MARKS {
            let g = glyph(mark, mode.is_dark());
            let h = mark_drawn(mode, mark, false);
            let cell = h.state().rect.unwrap();
            let c = cell.center();
            let what = format!("{mode:?} {mark:?}");
            let cs = circles(&h);
            let filled_paths: Vec<PathShape> = paths(&h)
                .into_iter()
                .filter(|p| p.closed && p.fill != Color32::TRANSPARENT)
                .collect();
            let ring = |cs: &[CircleShape]| {
                cs.iter().any(|k| {
                    near(k.center.x, c.x)
                        && near(k.center.y, c.y)
                        && near(k.radius, ring_r)
                        && k.fill == Color32::TRANSPARENT
                        && near(k.stroke.width, STEP_MARK_STROKE)
                        && k.stroke.color == ink(g.ink)
                })
            };
            match mark {
                StepMark::NotRun => {
                    assert!(ring(&cs), "{what}: {cs:?}");
                    assert_eq!(cs.len(), 1, "{what}: one ring and nothing else");
                    assert!(filled_paths.is_empty(), "{what}");
                }
                StepMark::Running => {
                    assert!(ring(&cs), "{what}: {cs:?}");
                    assert_eq!(filled_paths.len(), 1, "{what}: one half disc");
                    let half = &filled_paths[0];
                    assert_eq!(half.fill, ink(g.ink), "{what}");
                    let b = Rect::from_points(&half.points);
                    assert!(near(b.left(), c.x), "{what}: the right half, {b:?}");
                    assert!(near(b.right(), c.x + ring_r), "{what}: {b:?}");
                    assert!(near(b.top(), c.y - ring_r), "{what}: {b:?}");
                    assert!(near(b.bottom(), c.y + ring_r), "{what}: {b:?}");
                }
                StepMark::Ok => {
                    assert_eq!(cs.len(), 1, "{what}: {cs:?}");
                    let d = cs[0];
                    assert!(near(d.center.x, c.x) && near(d.center.y, c.y), "{what}");
                    assert!(near(d.radius, disc_r), "{what}: {}", d.radius);
                    assert_eq!(d.fill, ink(g.fill.unwrap()), "{what}");
                    assert!(filled_paths.is_empty(), "{what}");
                }
                StepMark::Failed => {
                    assert!(cs.is_empty(), "{what}: {cs:?}");
                    assert_eq!(filled_paths.len(), 1, "{what}");
                    let d = &filled_paths[0];
                    assert_eq!(d.fill, ink(g.fill.unwrap()), "{what}");
                    assert_eq!(d.points.len(), 4, "{what}: {:?}", d.points);
                    for p in &d.points {
                        let (dx, dy) = ((p.x - c.x).abs(), (p.y - c.y).abs());
                        assert!(
                            (near(dx, 0.0) && near(dy, diamond_r))
                                || (near(dy, 0.0) && near(dx, diamond_r)),
                            "{what}: a corner off the axes at {p:?}"
                        );
                    }
                }
                StepMark::Stale => {
                    assert!(ring(&cs), "{what}: {cs:?}");
                    assert_eq!(cs.len(), 2, "{what}: {cs:?}");
                    let dot = cs.iter().find(|k| k.fill != Color32::TRANSPARENT).unwrap();
                    assert_eq!(dot.fill, ink(g.ink), "{what}");
                    assert!(near(dot.radius, dot_r), "{what}: {}", dot.radius);
                    assert!(near(dot.center.x, c.x) && near(dot.center.y, c.y));
                }
            }
        }
    }
}

#[test]
fn a_step_mark_is_a_square_cell_with_the_focused_treatment() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = mark_drawn(mode, StepMark::Ok, false);
        let cell = h.state().rect.unwrap();
        assert!(near(cell.width(), STEP_MARK_CELL) && near(cell.height(), STEP_MARK_CELL));
        assert!(on_row(&painted(&h, mode), cell).is_empty(), "{mode:?}");

        let h = mark_drawn(mode, StepMark::Ok, true);
        let fills = on_row(&painted(&h, mode), cell);
        assert_eq!(fills.len(), 1, "{mode:?}: {fills:?}");
        assert_eq!(fills[0].fill, ink(sem.rows.selected_background));
        assert_eq!(
            fills[0].stroke,
            Stroke::new(1.0, ink(sem.rows.selected_border))
        );
        assert_eq!(fills[0].kind, StrokeKind::Inside);
        assert_eq!(fills[0].radius, egui::CornerRadius::ZERO);
    }
}

// ─── the gutter ─────────────────────────────────────────────────────────────

#[test]
fn the_mark_sits_on_its_lane() {
    for mode in MODES {
        for lane in [0u8, 1] {
            let mut spec = Spec::new();
            spec.mark = StepMark::NotRun;
            spec.node = Some(lane);
            spec.through = vec![];
            spec.from_above = vec![lane];
            spec.to_below = vec![];
            let h = drawn(mode, spec);
            let row = row_rect(&h);
            let rings = circles(&h);
            assert_eq!(rings.len(), 1, "{mode:?} {lane}: {rings:?}");
            assert!(
                near(rings[0].center.x, lane_x(row, lane)),
                "{mode:?} lane {lane}: at {} not {}",
                rings[0].center.x,
                lane_x(row, lane)
            );
            assert!(near(rings[0].center.y, row.center().y));
        }
    }
}

/// A node, then the lanes through, arriving and leaving.
type GutterShape = (Option<u8>, &'static [u8], &'static [u8], &'static [u8]);

#[test]
fn every_row_s_gutter_is_one_width_for_a_given_lane_count() {
    let mode = Mode::Light;
    let shapes: [GutterShape; 4] = [
        (Some(0), &[], &[], &[]),
        (Some(1), &[0], &[1], &[1]),
        (None, &[0, 1], &[], &[]),
        (Some(0), &[], &[0, 1], &[0, 1]),
    ];
    for lanes in [1u8, 2, 3] {
        for (node, through, up, down) in shapes {
            let mut spec = Spec::new();
            spec.lanes = lanes;
            spec.node = node;
            spec.through = through.to_vec();
            spec.from_above = up.to_vec();
            spec.to_below = down.to_vec();
            let h = drawn(mode, spec);
            let row = row_rect(&h);
            let (_, name, _) = text(&h, NAME);
            assert!(
                near(name.left(), row.left() + gutter_width(lanes) + SPACE_3),
                "lanes {lanes}, {node:?} {through:?}: the name starts at {}, not {}",
                name.left() - row.left(),
                gutter_width(lanes) + SPACE_3
            );
        }
    }
}

#[test]
fn edges_are_lines_in_the_strong_border_and_meet_the_mark() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.lanes = 3;
        spec.node = Some(1);
        spec.through = vec![2];
        spec.from_above = vec![1, 0];
        spec.to_below = vec![1, 2];
        let h = drawn(mode, spec);
        let row = row_rect(&h);
        let gutter_right = row.left() + gutter_width(3);
        let centre = egui::pos2(lane_x(row, 1), row.center().y);
        let reach = (STEP_MARK + STEP_MARK_STROKE) / 2.0;

        let strokes: Vec<Stroke> = segments(&h)
            .into_iter()
            .filter(|(p, _)| p[0].x <= gutter_right)
            .map(|(_, s)| s)
            .collect();
        assert!(!strokes.is_empty());
        for s in &strokes {
            assert_eq!(*s, Stroke::new(EDGE, ink(sem.borders.strong)), "{mode:?}");
        }
        for p in paths(&h).iter().filter(|p| near(p.stroke.width, EDGE)) {
            assert_eq!(path_colour(p), ink(sem.borders.strong), "{mode:?}");
        }

        let (lines, curves) = edges(&h, gutter_right);
        // Through lane 2, the top to bottom of the row.
        assert!(
            lines.iter().any(|p| near(p[0].x, lane_x(row, 2))
                && near(p[1].x, lane_x(row, 2))
                && near(p[0].y.min(p[1].y), row.top())
                && near(p[0].y.max(p[1].y), row.bottom())),
            "{mode:?}: {lines:?}"
        );
        // Lane 1 arrives straight from the top and leaves straight to the foot,
        // each stopping at the mark's edge.
        let x1 = lane_x(row, 1);
        assert!(
            lines.iter().any(|p| near(p[0].x, x1)
                && near(p[0].y, row.top())
                && near(p[1].y, centre.y - reach)),
            "{mode:?}: {lines:?}"
        );
        assert!(
            lines.iter().any(|p| near(p[0].x, x1)
                && near(p[0].y, centre.y + reach)
                && near(p[1].y, row.bottom())),
            "{mode:?}: {lines:?}"
        );
        // Lane 0 curves in from the top; lane 2 curves out to the foot.
        assert_eq!(curves.len(), 2, "{mode:?}: {curves:?}");
        let into = curves
            .iter()
            .find(|c| near(c[0].x, lane_x(row, 0)))
            .expect("the arriving curve");
        assert!(near(into[0].y, row.top()));
        let end = *into.last().unwrap();
        assert!(
            near(end.distance(centre), reach),
            "{mode:?}: ends at {end:?}"
        );
        let out = curves
            .iter()
            .find(|c| near(c.last().unwrap().x, lane_x(row, 2)))
            .expect("the leaving curve");
        assert!(near(out.last().unwrap().y, row.bottom()));
        assert!(
            near(out[0].distance(centre), reach),
            "{mode:?}: {:?}",
            out[0]
        );
    }
}

#[test]
fn a_lane_at_or_past_the_lane_count_is_ignored() {
    let mode = Mode::Light;
    let mut spec = Spec::new();
    spec.lanes = 2;
    spec.node = Some(0);
    spec.through = vec![0, 2, 7];
    spec.from_above = vec![0, 3];
    spec.to_below = vec![1, 2];
    let h = drawn(mode, spec.clone());
    let row = row_rect(&h);
    let gutter_right = row.left() + gutter_width(2);
    let (lines, curves) = edges(&h, f32::INFINITY);
    assert_eq!(lines.len(), 2, "through 0 and arriving 0: {lines:?}");
    assert_eq!(curves.len(), 1, "leaving to 1: {curves:?}");
    for p in lines.iter().flatten().chain(curves.iter().flatten()) {
        assert!(p.x <= gutter_right, "{p:?} is past the gutter");
    }
    assert_eq!(circles(&h).len(), 1, "the mark on lane 0");

    // A node past the count: no mark, and no edge arrives or leaves.
    spec.node = Some(2);
    spec.from_above = vec![0];
    spec.to_below = vec![0];
    let h = drawn(mode, spec);
    let (lines, curves) = edges(&h, f32::INFINITY);
    assert!(circles(&h).is_empty(), "{:?}", circles(&h));
    assert!(paths(&h)
        .iter()
        .all(|p| !p.closed || p.fill == Color32::TRANSPARENT));
    assert_eq!(lines.len(), 1, "only through lane 0: {lines:?}");
    assert!(curves.is_empty(), "{curves:?}");
}

// ─── the columns ────────────────────────────────────────────────────────────

#[test]
fn a_step_row_is_24_and_fills_its_width_with_its_columns_in_place() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new());
        let row = row_rect(&h);
        assert!(
            near(row.height(), HEIGHT_MD) && near(row.width(), WIDTH),
            "{row:?}"
        );

        let (_, name, name_ink) = text(&h, NAME);
        assert_eq!(name_ink, ink(sem.text.primary), "{mode:?}");
        let (_, out, out_ink) = text(&h, OUTPUT);
        assert_eq!(out_ink, ink(sem.text.secondary), "{mode:?}");
        assert!(near(out.right(), output_right(row)), "{mode:?}: {out:?}");
        assert!(
            name.right() <= out.left() - SPACE_3 + 0.01,
            "{name:?} {out:?}"
        );
        let (_, dur, dur_ink) = text(&h, DURATION);
        assert_eq!(dur_ink, ink(sem.text.muted), "{mode:?}");
        assert!(near(dur.right(), duration_right(row)), "{mode:?}: {dur:?}");

        // The timeline column's gridlines, at the start of each half.
        let grid = ink(if mode.is_dark() { INK_DARK } else { INK_LIGHT }.gridline);
        let lines: Vec<Painted> = painted(&h, mode)
            .into_iter()
            .filter(|r| r.fill == grid && near(r.rect.width(), 1.0))
            .collect();
        let left = timeline_left(row);
        for x in [left, left + TIMELINE_WIDTH / 2.0] {
            assert!(
                lines.iter().any(|r| near(r.rect.left(), x)
                    && near(r.rect.top(), row.top())
                    && near(r.rect.bottom(), row.bottom())),
                "{mode:?}: no gridline at {x}: {lines:?}"
            );
        }
    }
}

#[test]
fn a_timeline_bar_is_the_strong_border_ruled_with_the_control_border_or_its_state_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        for (mark, fill, rule) in [
            (StepMark::Ok, sem.borders.strong, sem.borders.control),
            (StepMark::Stale, sem.borders.strong, sem.borders.control),
            (StepMark::Failed, STATUS.critical, STATUS.critical),
            (
                StepMark::Running,
                sem.feedback.progress_bar,
                sem.feedback.progress_bar,
            ),
        ] {
            let mut spec = Spec::new();
            spec.mark = mark;
            let h = drawn(mode, spec);
            let row = row_rect(&h);
            let bars = bars_in_timeline(&painted(&h, mode), row);
            assert_eq!(bars.len(), 1, "{mode:?} {mark:?}: {bars:?}");
            let b = bars[0];
            assert_eq!(b.fill, ink(fill), "{mode:?} {mark:?}");
            assert_eq!(b.stroke, Stroke::new(1.0, ink(rule)), "{mode:?} {mark:?}");
            assert_eq!(b.kind, StrokeKind::Inside);
            assert!(near(b.rect.height(), TIMELINE_BAR_HEIGHT));
            assert!(near(b.rect.center().y, row.center().y));
        }
    }
}

#[test]
fn a_timeline_is_clamped_ordered_and_never_narrower_than_two() {
    let mode = Mode::Light;
    for ((start, end), want) in [
        ((-0.5, 0.2), Some((0.0, 0.2))),
        ((0.9, 1.7), Some((0.9, 1.0))),
        ((0.6, 0.4), Some((0.4, 0.6))),
        ((f32::NAN, 0.5), None),
        ((0.5, f32::NAN), None),
    ] {
        let mut spec = Spec::new();
        spec.timeline = Some((start, end));
        let h = drawn(mode, spec);
        let row = row_rect(&h);
        let bars = bars_in_timeline(&painted(&h, mode), row);
        let left = timeline_left(row);
        match want {
            None => assert!(bars.is_empty(), "({start}, {end}): {bars:?}"),
            Some((a, b)) => {
                assert_eq!(bars.len(), 1, "({start}, {end}): {bars:?}");
                let r = bars[0].rect;
                assert!(
                    near(r.left(), left + a * TIMELINE_WIDTH),
                    "({start}, {end}): {r:?}"
                );
                assert!(
                    near(r.right(), left + b * TIMELINE_WIDTH),
                    "({start}, {end}): {r:?}"
                );
            }
        }
    }

    // A zero span, and a sliver at the very end, are still two wide and stay
    // inside the column.
    for (start, end) in [(0.5, 0.5), (1.0, 1.0), (0.99, 1.0)] {
        let mut spec = Spec::new();
        spec.timeline = Some((start, end));
        let h = drawn(mode, spec);
        let row = row_rect(&h);
        let bars = bars_in_timeline(&painted(&h, mode), row);
        assert_eq!(bars.len(), 1, "({start}, {end})");
        let r = bars[0].rect;
        assert!(near(r.width(), SPACE_1), "({start}, {end}): {r:?}");
        assert!(r.right() <= timeline_left(row) + TIMELINE_WIDTH + 0.01);
    }
}

#[test]
fn a_table_row_draws_no_timeline_or_duration_but_keeps_the_columns() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let step = drawn(mode, Spec::new());
        let (_, step_out, _) = text(&step, OUTPUT);

        let mut spec = Spec::new();
        spec.entry = StepEntry::Table;
        spec.label = "installs";
        let h = drawn(mode, spec);
        let row = row_rect(&h);
        assert!(
            bars_in_timeline(&painted(&h, mode), row).is_empty(),
            "{mode:?}"
        );
        assert!(
            texts(&h).iter().all(|(s, _, _)| s != DURATION),
            "{mode:?}: {:?}",
            texts(&h)
        );
        let (_, out, _) = text(&h, OUTPUT);
        assert!(
            near(out.right(), step_out.right()),
            "{mode:?}: {out:?} {step_out:?}"
        );
        let (_, _, name_ink) = text(&h, "installs");
        assert_eq!(name_ink, ink(sem.text.secondary), "{mode:?}");
    }
}

#[test]
fn a_long_name_is_cut_and_the_output_bar_and_duration_keep_their_columns() {
    for mode in MODES {
        let short = drawn(mode, Spec::new());
        let row = row_rect(&short);
        let (_, short_out, _) = text(&short, OUTPUT);
        let (_, short_dur, _) = text(&short, DURATION);
        let short_bar = bars_in_timeline(&painted(&short, mode), row)[0].rect;

        let mut spec = Spec::new();
        spec.label = "a_very_long_step_name_that_runs_far_past_the_room_this_row_leaves_for_it";
        let h = drawn(mode, spec);
        let (name, name_rect, _) = text(&h, "a_very");
        assert!(name.ends_with('…'), "{name:?}");
        assert_eq!(name.lines().count(), 1, "{name:?}");
        let (_, out, _) = text(&h, OUTPUT);
        let (_, dur, _) = text(&h, DURATION);
        let bar = bars_in_timeline(&painted(&h, mode), row)[0].rect;
        assert!(near_rect(out, short_out), "{mode:?}: {out:?} {short_out:?}");
        assert!(near_rect(dur, short_dur), "{mode:?}");
        assert!(near_rect(bar, short_bar), "{mode:?}");
        assert!(
            name_rect.right() <= out.left() - SPACE_3 + 0.01,
            "{name_rect:?} {out:?}"
        );
        assert!(name_rect.height() <= HEIGHT_MD);
    }
}

// ─── the fold control ───────────────────────────────────────────────────────

#[test]
fn a_folded_row_shows_its_member_count_and_reports_fold_clicked_apart_from_clicked() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.fold = Some(Fold::Folded { members: 3 });
        let mut h = drawn(mode, spec.clone());
        let row = row_rect(&h);
        let fold_left = row.left() + gutter_width(2) + SPACE_3;
        let (_, count, count_ink) = text(&h, "3");
        assert_eq!(count_ink, ink(sem.text.muted), "{mode:?}");
        assert!(count.left() > fold_left, "{mode:?}: {count:?}");
        let (_, name, _) = text(&h, NAME);
        assert!(
            name.left() >= count.right() + SPACE_3 - 0.01,
            "{name:?} {count:?}"
        );
        // The chevron, in the muted ink, at the fold's leading edge.
        assert!(
            paths(&h)
                .iter()
                .any(|p| path_colour(p) == ink(sem.text.muted)
                    && Rect::from_points(&p.points).left() >= fold_left - 0.01
                    && Rect::from_points(&p.points).right() <= count.left()),
            "{mode:?}: no chevron"
        );

        click_at(&mut h, egui::pos2(count.center().x, row.center().y));
        assert!(h.state().fold_clicked, "{mode:?}");
        assert!(!h.state().clicked, "{mode:?}: the fold is not the row");

        let mut h = drawn(mode, spec.clone());
        click_at(&mut h, name.center());
        assert!(h.state().clicked, "{mode:?}");
        assert!(!h.state().fold_clicked, "{mode:?}");

        // Unfolded: the chevron and no count.
        let mut open = spec.clone();
        open.fold = Some(Fold::Unfolded);
        let h = drawn(mode, open);
        assert!(texts(&h).iter().all(|(s, _, _)| s != "3"));
        let (_, name, _) = text(&h, NAME);
        assert!(
            name.left() > fold_left + 12.0,
            "{mode:?}: the chevron keeps its room"
        );
    }
}

// ─── the states ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
enum Interact {
    Rest,
    Hover,
    Pressed,
}

fn interacted(mode: Mode, spec: Spec, how: Interact) -> (Vec<Painted>, Rect) {
    let label = format!("{}, {}", spec.label, word(spec.mark));
    let mut h = drawn(mode, spec);
    if !matches!(how, Interact::Rest) {
        h.get_by_role_and_label(AccessRole::Row, &label).hover();
        h.run();
    }
    if matches!(how, Interact::Pressed) {
        let pos = row_rect(&h).center();
        h.event(egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        });
        h.run();
    }
    let rect = row_rect(&h);
    (painted(&h, mode), rect)
}

#[test]
fn a_row_at_rest_draws_no_fill_and_no_bar() {
    for mode in MODES {
        let (rects, row) = interacted(mode, Spec::new(), Interact::Rest);
        assert!(on_row(&rects, row).is_empty(), "{mode:?}: {rects:?}");
        assert!(leading_bars(&rects, row).is_empty());
    }
}

#[test]
fn hover_and_pressed_are_the_list_row_s() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        for (how, want) in [
            (Interact::Hover, sem.rows.hover_background),
            (Interact::Pressed, sem.role(Role::Neutral).background.active),
        ] {
            let (rects, row) = interacted(mode, Spec::new(), how);
            let fills = on_row(&rects, row);
            assert_eq!(fills.len(), 1, "{mode:?} {how:?}: {fills:?}");
            assert_eq!(fills[0].fill, ink(want), "{mode:?} {how:?}");
            assert!(leading_bars(&rects, row).is_empty(), "{mode:?} {how:?}");
        }
    }
}

#[test]
fn the_cursor_is_the_list_row_s_fill_and_bar() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.cursor = true;
        let (rects, row) = interacted(mode, spec, Interact::Rest);
        let fills = on_row(&rects, row);
        assert_eq!(fills.len(), 1, "{mode:?}: {fills:?}");
        assert_eq!(fills[0].fill, ink(sem.rows.cursor_background));
        assert!(!fills[0].is_stroked(), "{mode:?}: the cursor has no rule");
        let bar = leading_bars(&rects, row);
        assert_eq!(bar.len(), 1, "{mode:?}");
        assert_eq!(bar[0].fill, ink(sem.rows.cursor_bar));
    }
}

#[test]
fn focused_is_the_selection_wash_with_an_inside_rule_under_the_pointer_too() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.focused = true;
        for how in [Interact::Rest, Interact::Hover, Interact::Pressed] {
            let (rects, row) = interacted(mode, spec.clone(), how);
            let fills = on_row(&rects, row);
            assert_eq!(fills.len(), 1, "{mode:?} {how:?}: {fills:?}");
            assert_eq!(fills[0].fill, ink(sem.rows.selected_background));
            assert_eq!(
                fills[0].stroke,
                Stroke::new(1.0, ink(sem.rows.selected_border))
            );
            assert_eq!(fills[0].kind, StrokeKind::Inside);
            assert!(leading_bars(&rects, row).is_empty(), "{mode:?} {how:?}");
        }
    }
}

#[test]
fn focused_with_the_cursor_is_the_wash_the_rule_and_the_bar_on_top() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.focused = true;
        spec.cursor = true;
        let (rects, row) = interacted(mode, spec, Interact::Rest);
        let fills = on_row(&rects, row);
        assert_eq!(fills.len(), 1, "{mode:?}: {fills:?}");
        assert_eq!(fills[0].fill, ink(sem.rows.selected_background));
        assert_eq!(
            fills[0].stroke,
            Stroke::new(1.0, ink(sem.rows.selected_border))
        );
        let bar = leading_bars(&rects, row);
        assert_eq!(bar.len(), 1, "{mode:?}");
        assert_eq!(bar[0].fill, ink(sem.rows.cursor_bar));
        let at = |p: &Painted| rects.iter().position(|r| std::ptr::eq(r, p)).unwrap();
        let wash = rects.iter().find(|r| near_rect(r.rect, row)).unwrap();
        let bar = rects
            .iter()
            .find(|r| near(r.rect.width(), ROW_BAR_WIDTH) && near(r.rect.left(), row.left()))
            .unwrap();
        assert!(
            at(bar) > at(wash),
            "{mode:?}: the bar is drawn over the rule"
        );
    }
}

// ─── accessibility ─────────────────────────────────────────────────────────

#[test]
fn a_step_row_is_a_row_named_by_its_name_and_state_word() {
    for mark in STEP_MARKS {
        let mut spec = Spec::new();
        spec.mark = mark;
        let h = drawn(Mode::Light, spec);
        let label = format!("{NAME}, {}", word(mark));
        let _ = h.get_by_role_and_label(AccessRole::Row, &label);
    }
}

#[test]
fn a_focused_row_s_node_is_selected_and_an_unfocused_one_is_not() {
    let label = format!("{NAME}, {}", word(StepMark::Ok));
    for focused in [false, true] {
        let mut spec = Spec::new();
        spec.focused = focused;
        let h = drawn(Mode::Light, spec);
        let node = h.get_by_role_and_label(AccessRole::Row, &label);
        assert_eq!(
            node.accesskit_node().data().is_selected() == Some(true),
            focused,
            "focused {focused}"
        );
    }
}

#[test]
fn the_fold_control_is_a_button_expanded_only_while_unfolded() {
    for (fold, expanded) in [(Fold::Unfolded, true), (Fold::Folded { members: 3 }, false)] {
        let mut spec = Spec::new();
        spec.fold = Some(fold);
        let h = drawn(Mode::Light, spec);
        let node = h.get_by_role_and_label(AccessRole::Button, NAME);
        assert_eq!(
            node.accesskit_node().data().is_expanded(),
            Some(expanded),
            "{fold:?}"
        );
    }
}

#[test]
fn a_step_mark_s_node_is_selected_while_focused() {
    for focused in [false, true] {
        let h = mark_drawn(Mode::Light, StepMark::Failed, focused);
        let node = h.get_by_label(word(StepMark::Failed));
        assert_eq!(
            node.accesskit_node().data().is_selected() == Some(true),
            focused,
            "focused {focused}"
        );
    }
}
