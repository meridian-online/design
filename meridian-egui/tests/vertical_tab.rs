//! The vertical tab, read out of the paint list and the accessibility tree:
//! the row's geometry, its inks at rest and under the pointer, the current
//! place's cell and how it covers the rail's rule, the dirty dot, the close
//! control and its own click, a long name cut before the dot and the close
//! control, and the accessibility roles.

mod common;

use common::{filled, ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::epaint::{CircleShape, PathShape};
use egui::{Color32, CornerRadius, Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::control::{DOT, HEIGHT_XS, ICON_XS};
use meridian_design::radius::{CHIP, CONTROL};
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{ROW_GRID, SPACE_2, SPACE_4};
use meridian_egui::{theme, vertical_tab, Mode, VerticalTab, VerticalTabResponse};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const WIDTH: f32 = 200.0;
const NODE: &str = "ranking";
const ACTIVITY: &str = "Grid";
const LABEL: &str = "ranking · Grid";

#[derive(Clone, Copy)]
struct Spec {
    node: &'static str,
    activity: &'static str,
    current: bool,
    dirty: bool,
}

impl Spec {
    fn new() -> Self {
        Self {
            node: NODE,
            activity: ACTIVITY,
            current: false,
            dirty: false,
        }
    }
    fn current(mut self) -> Self {
        self.current = true;
        self
    }
    fn dirty(mut self) -> Self {
        self.dirty = true;
        self
    }
}

#[derive(Default)]
struct Out {
    rect: Option<Rect>,
    clicked: bool,
    close_clicked: bool,
}

fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(WIDTH + 40.0, 80.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(WIDTH, 40.0), |ui| {
                    let tab = VerticalTab::new(spec.node, spec.activity)
                        .current(spec.current)
                        .dirty(spec.dirty);
                    let r: VerticalTabResponse = vertical_tab(ui, tab);
                    out.rect = Some(r.response.rect);
                    out.clicked |= r.clicked;
                    out.close_clicked |= r.close_clicked;
                });
            },
            Out::default(),
        );
    harness.run();
    harness
}

fn label_of(spec: Spec) -> String {
    format!("{} · {}", spec.node, spec.activity)
}

fn row_rect(h: &Harness<'_, Out>) -> Rect {
    h.state().rect.expect("the tab was drawn")
}

/// The close control's box: 18 square, `SPACE_2` in from the trailing edge.
fn close_rect(row: Rect) -> Rect {
    Rect::from_center_size(
        egui::pos2(row.right() - SPACE_2 - HEIGHT_XS / 2.0, row.center().y),
        egui::vec2(HEIGHT_XS, HEIGHT_XS),
    )
}

fn move_to(h: &mut Harness<'_, Out>, pos: Pos2) {
    h.event(egui::Event::PointerMoved(pos));
    h.run();
}

fn click_at(h: &mut Harness<'_, Out>, pos: Pos2) {
    move_to(h, pos);
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

/// The strokes of the "x": paths whose box lies in `area`, with their colour.
fn glyph_in<S>(h: &Harness<'_, S>, area: Rect) -> Vec<(Rect, Color32)> {
    flat(&h.output().shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Path(p) => Some(p),
            _ => None,
        })
        .map(|p: PathShape| {
            let colour = match p.stroke.color {
                egui::epaint::ColorMode::Solid(c) => c,
                _ => Color32::TRANSPARENT,
            };
            (Rect::from_points(&p.points), colour)
        })
        .filter(|(r, _)| area.expand(0.5).contains_rect(*r))
        .collect()
}

struct Txt {
    text: String,
    rect: Rect,
    /// The ink of each section, in order, with the text it covers.
    sections: Vec<(String, Color32)>,
    medium: bool,
}

fn texts<S>(h: &Harness<'_, S>) -> Vec<Txt> {
    flat(&h.output().shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Text(t) => {
                let job = &t.galley.job;
                let full = job.text.clone();
                let sections = job
                    .sections
                    .iter()
                    .map(|sec| {
                        (
                            full[sec.byte_range.start.0..sec.byte_range.end.0].to_owned(),
                            sec.format.color,
                        )
                    })
                    .collect();
                let medium = matches!(
                    &job.sections[0].format.font_id.family,
                    egui::FontFamily::Name(n) if &**n == "Inter-Medium"
                );
                Some(Txt {
                    text: t.galley.rows.iter().map(|r| r.text()).collect(),
                    rect: t.galley.rect.translate(t.pos.to_vec2()),
                    sections,
                    medium,
                })
            }
            _ => None,
        })
        .collect()
}

fn label_text<S>(h: &Harness<'_, S>) -> Txt {
    texts(h)
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("no text drawn"))
}

fn corners(nw: u8, ne: u8, sw: u8, se: u8) -> CornerRadius {
    CornerRadius { nw, ne, sw, se }
}

#[derive(Clone, Copy, Debug)]
enum Interact {
    Rest,
    Hover,
    CloseHover,
    Focus,
}

fn frame(mode: Mode, spec: Spec, how: Interact) -> (Vec<Painted>, Harness<'static, Out>) {
    let mut h = drawn(mode, spec);
    let row = row_rect(&h);
    match how {
        Interact::Rest => {}
        Interact::Hover => move_to(
            &mut h,
            egui::pos2(row.left() + SPACE_4 + 4.0, row.center().y),
        ),
        Interact::CloseHover => move_to(&mut h, close_rect(row).center()),
        Interact::Focus => {
            h.get_by_role_and_label(AccessRole::Tab, &label_of(spec))
                .focus();
            h.run();
        }
    }
    (painted(&h, mode), h)
}

// ─── geometry ───────────────────────────────────────────────────────────────

#[test]
fn a_tab_fills_the_width_and_is_one_grid_row_tall() {
    for mode in MODES {
        let h = drawn(mode, Spec::new());
        let r = row_rect(&h);
        assert!(near(r.width(), WIDTH), "{r:?}");
        assert!(near(r.height(), ROW_GRID), "{r:?}");
    }
}

#[test]
fn a_tab_at_rest_draws_no_fill_and_no_rule() {
    for mode in MODES {
        let (rects, _) = frame(mode, Spec::new(), Interact::Rest);
        assert!(rects.is_empty(), "{mode:?}: {rects:?}");
    }
}

#[test]
fn the_label_starts_a_leading_pad_in() {
    for mode in MODES {
        let h = drawn(mode, Spec::new());
        let t = label_text(&h);
        assert!(
            near(t.rect.left(), row_rect(&h).left() + SPACE_4),
            "{t:?}",
            t = t.rect
        );
        assert_eq!(t.text, LABEL);
    }
}

// ─── the label's inks ───────────────────────────────────────────────────────

#[test]
fn at_rest_the_node_is_secondary_and_the_activity_muted() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new());
        let t = label_text(&h);
        assert_eq!(t.sections.len(), 2);
        assert_eq!(t.sections[0], (NODE.to_owned(), ink(sem.text.secondary)));
        assert_eq!(t.sections[1], (" · Grid".to_owned(), ink(sem.text.muted)));
        assert!(!t.medium);
    }
}

#[test]
fn under_the_pointer_the_node_is_primary_and_the_row_takes_the_hover_fill() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = frame(mode, Spec::new(), Interact::Hover);
        let row = row_rect(&h);
        let t = label_text(&h);
        assert_eq!(t.sections[0].1, ink(sem.text.primary), "{mode:?}");
        assert_eq!(t.sections[1].1, ink(sem.text.muted), "{mode:?}");
        let fills = filled(&rects, ink(sem.rows.hover_background));
        assert_eq!(fills.len(), 1, "{mode:?}: {rects:?}");
        assert!(near_rect(fills[0].rect, row), "{:?}", fills[0]);
        assert_eq!(fills[0].radius, CornerRadius::ZERO);
    }
}

// ─── the current place ─────────────────────────────────────────────────────

#[test]
fn the_current_tab_is_a_raised_cell_ruled_on_three_sides_with_leading_corners() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = frame(mode, Spec::new().current(), Interact::Rest);
        let row = row_rect(&h);
        let fill = filled(&rects, ink(sem.surfaces.raised));
        assert_eq!(fill.len(), 1, "{mode:?}: {rects:?}");
        let fill = fill[0];
        let want = corners(CONTROL as u8, 0, CONTROL as u8, 0);
        assert_eq!(fill.radius, want, "{mode:?}");
        assert!(near(fill.rect.left(), row.left()));
        assert!(near(fill.rect.top(), row.top()) && near(fill.rect.bottom(), row.bottom()));
        let rule: Vec<_> = rects
            .iter()
            .filter(|p| p.is_stroked() && p.stroke.color == ink(sem.borders.subtle))
            .collect();
        assert_eq!(rule.len(), 1, "{mode:?}: {rects:?}");
        let rule = rule[0];
        assert_eq!(rule.radius, want, "{mode:?}");
        assert_eq!(rule.fill, Color32::TRANSPARENT);
        assert!(near(rule.stroke.width, 1.0));
        assert!(near(rule.rect.left(), row.left()));
        // The trailing run of the rule lies past the clip, so the cell is
        // ruled on its top, bottom and leading sides only.
        assert!(
            rule.rect.right() >= rule.clip.right() + 1.0 - 0.01,
            "{rule:?}"
        );
        assert!(near(rule.clip.right(), row.right() + 1.0), "{rule:?}");
        assert!(near(rule.clip.left(), row.left()));
    }
}

#[test]
fn the_current_tab_extends_one_pixel_past_its_trailing_edge() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = frame(mode, Spec::new().current(), Interact::Rest);
        let row = row_rect(&h);
        let cell = filled(&rects, ink(sem.surfaces.raised))[0];
        assert!(
            near(cell.rect.right(), row.right() + 1.0)
                && near(cell.clip.right(), row.right() + 1.0),
            "{mode:?}: {cell:?} against {row:?}"
        );
        // The row itself, and the content in it, stay where they were.
        assert!(near(row.width(), WIDTH));
    }
}

#[test]
fn the_current_node_is_medium_primary_and_the_activity_secondary() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new().current());
        let t = label_text(&h);
        assert!(t.medium, "{mode:?}: the node is Medium");
        assert_eq!(t.sections[0].1, ink(sem.text.primary));
        assert_eq!(t.sections[1].1, ink(sem.text.secondary));
        assert_eq!(t.sections[0].0, NODE);
        assert_eq!(t.sections[1].0, " · Grid");
    }
}

#[test]
fn a_current_tab_under_the_pointer_stays_raised() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, _) = frame(mode, Spec::new().current(), Interact::Hover);
        assert_eq!(
            filled(&rects, ink(sem.surfaces.raised)).len(),
            1,
            "{mode:?}"
        );
        assert!(filled(&rects, ink(sem.rows.hover_background)).is_empty());
    }
}

// ─── the close control ─────────────────────────────────────────────────────

#[test]
fn the_close_control_is_drawn_only_under_the_pointer_or_on_the_current_place() {
    for mode in MODES {
        let cases = [
            (Spec::new(), Interact::Rest, false),
            (Spec::new(), Interact::Hover, true),
            (Spec::new().current(), Interact::Rest, true),
            (Spec::new().current(), Interact::Hover, true),
        ];
        for (spec, how, shown) in cases {
            let (_, h) = frame(mode, spec, how);
            let area = close_rect(row_rect(&h));
            assert_eq!(
                !glyph_in(&h, area).is_empty(),
                shown,
                "{mode:?} current {} {how:?}",
                spec.current
            );
        }
    }
}

#[test]
fn the_close_glyph_is_a_twelve_x_in_muted_and_primary_under_the_pointer_itself() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (_, h) = frame(mode, Spec::new().current(), Interact::Rest);
        let area = close_rect(row_rect(&h));
        let strokes = glyph_in(&h, area);
        assert!(!strokes.is_empty());
        for (_, c) in &strokes {
            assert_eq!(*c, ink(sem.text.muted), "{mode:?}");
        }
        let all = strokes.iter().fold(Rect::NOTHING, |a, (r, _)| a.union(*r));
        assert!(all.width() <= ICON_XS + 0.01 && all.height() <= ICON_XS + 0.01);
        assert!(near(all.center().x, area.center().x) && near(all.center().y, area.center().y));

        let (rects, h) = frame(mode, Spec::new().current(), Interact::CloseHover);
        let area = close_rect(row_rect(&h));
        for (_, c) in glyph_in(&h, area) {
            assert_eq!(c, ink(sem.text.primary), "{mode:?}");
        }
        let neutral = sem.role(Role::Neutral);
        let wash = filled(&rects, ink(neutral.background.active));
        assert_eq!(wash.len(), 1, "{mode:?}: {rects:?}");
        assert!(near_rect(wash[0].rect, area), "{:?}", wash[0]);
        assert_eq!(wash[0].radius, CornerRadius::same(CHIP as u8));
    }
}

#[test]
fn a_hovered_tab_closed_control_takes_no_wash_until_the_pointer_is_on_it() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let neutral = sem.role(Role::Neutral);
        let (rects, _) = frame(mode, Spec::new(), Interact::Hover);
        assert!(filled(&rects, ink(neutral.background.active)).is_empty());
        // The row stays hovered while the pointer is on the close control.
        let (rects, _) = frame(mode, Spec::new(), Interact::CloseHover);
        assert_eq!(filled(&rects, ink(sem.rows.hover_background)).len(), 1);
    }
}

#[test]
fn a_click_on_close_reports_close_clicked_and_not_clicked() {
    for mode in MODES {
        let mut h = drawn(mode, Spec::new().current());
        let pos = close_rect(row_rect(&h)).center();
        click_at(&mut h, pos);
        assert!(h.state().close_clicked, "{mode:?}");
        assert!(!h.state().clicked, "{mode:?}");

        let mut h = drawn(mode, Spec::new().current());
        let row = row_rect(&h);
        click_at(
            &mut h,
            egui::pos2(row.left() + SPACE_4 + 4.0, row.center().y),
        );
        assert!(h.state().clicked, "{mode:?}");
        assert!(!h.state().close_clicked, "{mode:?}");
    }
}

#[test]
fn a_pointer_that_arrives_over_the_close_place_of_a_resting_tab_finds_it_shown() {
    // Hidden, the control takes no pointer: the pointer first has to be over
    // the row, and then the control is shown. A press that arrives with the
    // pointer already there is the close control's.
    let mut h = drawn(Mode::Light, Spec::new());
    let pos = close_rect(row_rect(&h)).center();
    click_at(&mut h, pos);
    assert!(h.state().close_clicked);
}

// ─── the dot ────────────────────────────────────────────────────────────────

#[test]
fn a_dirty_tab_draws_a_round_secondary_dot_before_the_close_control() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new().dirty());
        let row = row_rect(&h);
        let dots = circles(&h);
        assert_eq!(dots.len(), 1, "{mode:?}");
        let d = dots[0];
        assert!(near(d.radius, DOT / 2.0));
        assert_eq!(d.fill, ink(sem.text.secondary));
        let want_x = close_rect(row).left() - SPACE_2 - DOT / 2.0;
        assert!(near(d.center.x, want_x), "{d:?} want {want_x}");
        assert!(near(d.center.y, row.center().y));

        let h = drawn(mode, Spec::new());
        assert!(circles(&h).is_empty());
    }
}

// ─── a long name ───────────────────────────────────────────────────────────

#[test]
fn a_long_node_name_is_cut_and_the_dot_and_close_keep_their_places() {
    const LONG: &str = "an_extremely_long_node_name_that_will_never_fit_in_the_rail_at_all";
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let spec = Spec {
            node: LONG,
            activity: "Chart",
            current: true,
            dirty: true,
        };
        let h = drawn(mode, spec);
        let row = row_rect(&h);
        assert!(near(row.height(), ROW_GRID));
        let t = label_text(&h);
        assert!(t.text.ends_with('…'), "{:?}", t.text);
        assert!(t.rect.height() < ROW_GRID, "one line: {:?}", t.rect);
        let d = circles(&h)[0];
        let dot_left = d.center.x - DOT / 2.0;
        assert!(
            t.rect.right() <= dot_left - SPACE_2 + 0.01,
            "{:?} against the dot at {dot_left}",
            t.rect
        );
        // The dot and the close control are where they are for a short name.
        let short = drawn(mode, Spec::new().current().dirty());
        let s = circles(&short)[0];
        assert!(near(s.center.x, d.center.x), "{mode:?}");
        let area = close_rect(row);
        let all = glyph_in(&h, area);
        assert!(
            !all.is_empty(),
            "{mode:?}: the close control kept its place"
        );
        assert!(row.contains_rect(t.rect.intersect(row)));
        // The glyph is still the muted ink.
        assert_eq!(all[0].1, ink(sem.text.muted));
    }
}

#[test]
fn a_long_name_without_a_dot_stops_before_the_close_control() {
    const LONG: &str = "an_extremely_long_node_name_that_will_never_fit_in_the_rail_at_all";
    let spec = Spec {
        node: LONG,
        activity: "Grid",
        current: false,
        dirty: false,
    };
    let h = drawn(Mode::Light, spec);
    let t = label_text(&h);
    assert!(t.rect.right() <= close_rect(row_rect(&h)).left() - SPACE_2 + 0.01);
}

// ─── focus ─────────────────────────────────────────────────────────────────

#[test]
fn a_focused_tab_is_ringed_inside_its_edge() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = frame(mode, Spec::new(), Interact::Focus);
        let row = row_rect(&h);
        let ring = rects
            .iter()
            .find(|p| p.stroke.color == ink(sem.borders.focus))
            .unwrap_or_else(|| panic!("{mode:?}: no ring in {rects:?}"));
        assert!(near_rect(ring.rect, row), "{ring:?}");
        assert!(near(ring.stroke.width, 2.0));
        assert_eq!(ring.radius, CornerRadius::ZERO, "a resting tab is square");

        let (rects, _) = frame(mode, Spec::new().current(), Interact::Focus);
        let ring = rects
            .iter()
            .find(|p| p.stroke.color == ink(sem.borders.focus))
            .unwrap_or_else(|| panic!("{mode:?}: no ring in {rects:?}"));
        let r = meridian_design::focus::ring_radius(CONTROL).round() as u8;
        assert_eq!(ring.radius, corners(r, 0, r, 0), "{mode:?}");
    }
}

#[test]
fn only_a_focused_tab_is_ringed() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, _) = frame(mode, Spec::new().current(), Interact::Rest);
        assert!(rects
            .iter()
            .all(|p| p.stroke.color != ink(sem.borders.focus)));
    }
}

// ─── accessibility ─────────────────────────────────────────────────────────

#[test]
fn a_tab_is_a_tab_selected_only_when_current() {
    for current in [false, true] {
        let mut spec = Spec::new();
        spec.current = current;
        let h = drawn(Mode::Light, spec);
        let node = h.get_by_role_and_label(AccessRole::Tab, LABEL);
        assert_eq!(
            node.accesskit_node().data().is_selected() == Some(true),
            current
        );
    }
}

#[test]
fn the_close_control_is_a_button_named_close_inside_the_tab() {
    let h = drawn(Mode::Light, Spec::new().current());
    let tab = h.get_by_role_and_label(AccessRole::Tab, LABEL);
    let close = h.get_by_role_and_label(AccessRole::Button, "Close");
    let parent = close.accesskit_node().parent().expect("close has a parent");
    assert_eq!(parent.id(), tab.accesskit_node().id());
}

#[test]
fn a_click_on_the_tab_by_accessibility_is_a_click() {
    let mut h = drawn(Mode::Light, Spec::new());
    h.get_by_role_and_label(AccessRole::Tab, LABEL).click();
    h.run();
    assert!(h.state().clicked);
    assert!(!h.state().close_clicked);
}

#[test]
fn an_empty_activity_shows_the_node_alone() {
    let spec = Spec {
        activity: "",
        ..Spec::new()
    };
    let h = drawn(Mode::Light, spec);
    let t = label_text(&h);
    assert_eq!(t.text, NODE);
    assert_eq!(t.sections.len(), 1);
    let _ = h.get_by_role_and_label(AccessRole::Tab, NODE);
}

// ─── keyboard, assistive technology and clipping ───────────────────────────

fn focus_close(h: &mut Harness<'_, Out>) {
    h.get_by_role_and_label(AccessRole::Button, "Close").focus();
    h.run();
}

fn press(h: &mut Harness<'_, Out>, key: egui::Key) {
    for pressed in [true, false] {
        h.event(egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        });
        h.run();
    }
}

#[test]
fn a_tab_row_is_focusable() {
    let h = drawn(Mode::Light, Spec::new());
    h.get_by_role_and_label(AccessRole::Tab, LABEL).focus();
}

#[test]
fn a_non_current_tab_with_keyboard_focus_exposes_close_and_draws_the_x() {
    for mode in MODES {
        let (_, h) = frame(mode, Spec::new(), Interact::Focus);
        let _ = h.get_by_role_and_label(AccessRole::Button, "Close");
        let area = close_rect(row_rect(&h));
        assert!(!glyph_in(&h, area).is_empty(), "{mode:?}");
    }
}

#[test]
fn enter_and_space_on_a_focused_tab_click_it_and_do_not_close() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let mut h = drawn(Mode::Light, Spec::new());
        h.get_by_role_and_label(AccessRole::Tab, LABEL).focus();
        h.run();
        press(&mut h, key);
        assert!(h.state().clicked, "{key:?}");
        assert!(!h.state().close_clicked, "{key:?}");
    }
}

#[test]
fn an_accessibility_click_on_close_closes_and_does_not_click_the_tab() {
    let mut h = drawn(Mode::Light, Spec::new().current());
    h.get_by_role_and_label(AccessRole::Button, "Close").click();
    h.run();
    assert!(h.state().close_clicked);
    assert!(!h.state().clicked);
}

#[test]
fn a_focused_close_control_takes_the_wash_the_primary_ink_and_a_ring() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let neutral = sem.role(Role::Neutral);
        let mut h = drawn(mode, Spec::new().current());
        focus_close(&mut h);
        let area = close_rect(row_rect(&h));
        let rects = painted(&h, mode);
        let ring = rects
            .iter()
            .find(|p| p.stroke.color == ink(sem.borders.focus))
            .unwrap_or_else(|| panic!("{mode:?}: no ring in {rects:?}"));
        assert!(near_rect(ring.rect, area), "{ring:?}");
        let r = meridian_design::focus::ring_radius(CHIP).round() as u8;
        assert_eq!(ring.radius, CornerRadius::same(r));
        assert_eq!(filled(&rects, ink(neutral.background.active)).len(), 1);
        for (_, c) in glyph_in(&h, area) {
            assert_eq!(c, ink(sem.text.primary), "{mode:?}");
        }
        // The row itself is not ringed.
        assert_eq!(
            rects
                .iter()
                .filter(|p| p.stroke.color == ink(sem.borders.focus))
                .count(),
            1
        );
    }
}

#[test]
fn the_cell_reaches_past_the_edge_when_the_parent_clip_ends_at_the_row() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut h = Harness::builder()
            .with_size(egui::vec2(WIDTH + 40.0, 80.0))
            .build_ui_state(
                move |ui, out: &mut Out| {
                    theme::apply(ui.ctx(), mode);
                    ui.allocate_ui(egui::vec2(WIDTH, 40.0), |ui| {
                        let mut clip = ui.clip_rect();
                        clip.max.x = ui.max_rect().right();
                        ui.set_clip_rect(clip);
                        let r = vertical_tab(ui, VerticalTab::new(NODE, ACTIVITY).current(true));
                        out.rect = Some(r.response.rect);
                    });
                },
                Out::default(),
            );
        h.run();
        let row = row_rect(&h);
        let fills = filled(&painted(&h, mode), ink(sem.surfaces.raised));
        assert_eq!(fills.len(), 1, "{mode:?}");
        assert!(near(fills[0].rect.right(), row.right() + 1.0));
        assert!(
            fills[0].clip.right() >= row.right() + 1.0 - 0.01,
            "{mode:?}: {:?}",
            fills[0]
        );
    }
}

#[test]
fn a_close_control_that_took_focus_from_a_non_current_tab_stays_shown_and_closes() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut h = drawn(mode, Spec::new());
        h.get_by_role_and_label(AccessRole::Tab, LABEL).focus();
        h.run();
        // Focus moves from the row to the close control, which leaves the row
        // unfocused with the pointer nowhere near.
        h.get_by_role_and_label(AccessRole::Button, "Close").focus();
        h.run();
        for _ in 0..3 {
            h.run();
        }
        let close = h.get_by_role_and_label(AccessRole::Button, "Close");
        assert!(close.accesskit_node().is_focused(), "{mode:?}");
        let area = close_rect(row_rect(&h));
        let rects = painted(&h, mode);
        let rings: Vec<_> = rects
            .iter()
            .filter(|p| p.stroke.color == ink(sem.borders.focus))
            .collect();
        assert_eq!(rings.len(), 1, "{mode:?}: {rects:?}");
        assert!(near_rect(rings[0].rect, area));
        assert!(!glyph_in(&h, area).is_empty());
        press(&mut h, egui::Key::Enter);
        assert!(h.state().close_clicked, "{mode:?}");
        assert!(!h.state().clicked, "{mode:?}");
    }
}
