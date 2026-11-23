//! The section: a labelled, countable, collapsible group. Its header is read
//! out of the paint list and the accessibility tree, and the body's absence
//! while closed out of whether the caller's closure ran.

mod common;

use common::{ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::{Color32, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::control::{HEIGHT_MD, ICON_SM};
use meridian_design::semantic::semantic;
use meridian_design::spacing::{SPACE_2, SPACE_4, SPACE_5, SPACE_6};
use meridian_design::typography::{LABEL_SIZE, LABEL_TRACKING_EM, UI_SIZE};
use meridian_egui::{section, theme, Mode, Section, SectionResponse};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const TITLE: &str = "Fields";

#[derive(Clone, Copy)]
struct Spec {
    title: &'static str,
    open: bool,
    count: Option<usize>,
    width: f32,
}

impl Spec {
    fn new() -> Self {
        Self {
            title: TITLE,
            open: false,
            count: None,
            width: 240.0,
        }
    }
}

struct Out {
    rect: Rect,
    toggled: bool,
    inner: Option<Option<Rect>>,
    body_calls: usize,
    focused: bool,
    header: Rect,
}

fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(spec.width + 40.0, 160.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(spec.width, 120.0), |ui| {
                    let mut s = Section::new(spec.title, spec.open);
                    if let Some(n) = spec.count {
                        s = s.count(n);
                    }
                    let mut calls = 0;
                    let r: SectionResponse<Rect> = section(ui, s, |ui| {
                        calls += 1;
                        let (rect, _) =
                            ui.allocate_exact_size(egui::vec2(10.0, 20.0), egui::Sense::hover());
                        rect
                    });
                    out.body_calls += calls;
                    out.header = r.response.rect;
                    out.toggled |= r.toggled;
                    out.inner = Some(r.inner);
                    out.focused = r.response.has_focus();
                    out.rect = ui.min_rect();
                });
            },
            Out {
                rect: Rect::NOTHING,
                toggled: false,
                inner: None,
                body_calls: 0,
                focused: false,
                header: Rect::NOTHING,
            },
        );
    harness.run();
    harness
}

#[derive(Clone, Copy)]
enum How {
    Rest,
    Hover,
    Focus,
}

fn interacted(mode: Mode, spec: Spec, how: How) -> (Vec<Painted>, Rect) {
    let mut harness = drawn(mode, spec);
    let header = || (AccessRole::Button, spec.title);
    match how {
        How::Rest => {}
        How::Hover => {
            harness
                .get_by_role_and_label(header().0, header().1)
                .hover();
            harness.run();
        }
        How::Focus => {
            harness
                .get_by_role_and_label(header().0, header().1)
                .focus();
            harness.run();
        }
    }
    let header = harness.state().header;
    (painted(&harness, mode), header)
}

/// Each text the frame drew: what its rows show (an ellipsis included), the
/// galley's job and the box it occupies.
fn texts(harness: &Harness<'_, Out>) -> Vec<(String, Color32, f32, f32, Rect)> {
    let mut out = Vec::new();
    for c in &harness.output().shapes {
        if let egui::Shape::Text(t) = &c.shape {
            let s: String = t.galley.rows.iter().map(|r| r.text()).collect();
            let f = &t.galley.job.sections[0].format;
            out.push((
                s,
                f.color,
                f.font_id.size,
                f.extra_letter_spacing,
                t.galley.rect.translate(t.pos.to_vec2()),
            ));
        }
    }
    out
}

fn icon_paths(harness: &Harness<'_, Out>) -> Vec<(Color32, Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(Color32, Rect)>) {
        match shape {
            egui::Shape::Path(p) => {
                if let egui::epaint::ColorMode::Solid(c) = p.stroke.color {
                    out.push((c, Rect::from_points(&p.points)));
                }
            }
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for c in &harness.output().shapes {
        walk(&c.shape, &mut out);
    }
    out
}

#[test]
fn a_closed_section_does_not_run_its_body() {
    let harness = drawn(Mode::Light, Spec::new());
    assert_eq!(harness.state().body_calls, 0);
    assert_eq!(harness.state().inner, Some(None));
    // The header is the whole of it.
    assert!(near(harness.state().rect.height(), HEIGHT_MD));
}

#[test]
fn an_open_section_runs_its_body_inset_and_fills_the_width() {
    for mode in MODES {
        let mut spec = Spec::new();
        spec.open = true;
        let harness = drawn(mode, spec);
        // Control for the closed case: the body does run here.
        assert!(harness.state().body_calls >= 1, "{mode:?}");
        let body = harness.state().inner.flatten().expect("the body's rect");
        let h = harness.state().header;
        assert!(near(h.height(), HEIGHT_MD), "{mode:?}");
        assert!(near(h.width(), spec.width), "{mode:?}: {h:?}");
        assert!(near(body.left(), h.left() + SPACE_6), "{mode:?}: {body:?}");
        assert!(near(body.top(), h.bottom() + SPACE_2), "{mode:?}: {body:?}");
        // The whole section ends SPACE_5 below the body and fills the width.
        let whole = harness.state().rect;
        assert!(near(whole.bottom(), body.bottom() + SPACE_5), "{mode:?}");
        assert!(near(whole.width(), spec.width), "{mode:?}: {whole:?}");
    }
}

#[test]
fn an_open_body_that_fills_the_width_stops_short_of_the_trailing_edge_by_space_4() {
    for mode in MODES {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(300.0, 160.0))
            .build_ui_state(
                move |ui, out: &mut Option<Rect>| {
                    theme::apply(ui.ctx(), mode);
                    ui.allocate_ui(egui::vec2(240.0, 120.0), |ui| {
                        let r = section(ui, Section::new(TITLE, true), |ui| {
                            ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), 10.0),
                                egui::Sense::hover(),
                            )
                            .0
                        });
                        *out = r.inner;
                        assert!(near(r.response.rect.right(), ui.min_rect().right()));
                    });
                },
                None,
            );
        harness.run();
        let body = harness.state().expect("body drawn");
        assert!(
            near(body.width(), 240.0 - SPACE_6 - SPACE_4),
            "{mode:?}: {body:?}"
        );
    }
}

#[test]
fn a_click_on_the_header_reports_toggled_and_changes_nothing_itself() {
    for open in [false, true] {
        let mut spec = Spec::new();
        spec.open = open;
        let mut harness = drawn(Mode::Light, spec);
        assert!(!harness.state().toggled);
        harness
            .get_by_role_and_label(AccessRole::Button, TITLE)
            .click();
        harness.run();
        assert!(harness.state().toggled, "open={open}");
        // The caller holds `open`: still what it passed, so the body is
        // still as it was.
        assert_eq!(harness.state().inner.map(|i| i.is_some()), Some(open));
    }
}

#[test]
fn enter_and_space_on_a_focused_header_toggle() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let mut harness = drawn(Mode::Light, Spec::new());
        harness
            .get_by_role_and_label(AccessRole::Button, TITLE)
            .focus();
        harness.run();
        assert!(!harness.state().toggled);
        harness.key_press(key);
        harness.run();
        assert!(harness.state().toggled, "{key:?}");
    }
}

#[test]
fn a_header_in_a_disabled_ui_reports_disabled() {
    for enabled in [true, false] {
        let mut harness = Harness::new_ui(move |ui| {
            ui.add_enabled_ui(enabled, |ui| {
                section(ui, Section::new(TITLE, false), |_| ());
            });
        });
        harness.run();
        let node = harness.get_by_role_and_label(AccessRole::Button, TITLE);
        assert_eq!(node.accesskit_node().data().is_disabled(), !enabled);
    }
}

#[test]
fn the_title_is_drawn_in_capitals_whatever_the_caller_passed() {
    for title in ["Fields", "fields", "FiElDs"] {
        let mut spec = Spec::new();
        spec.title = title;
        let harness = drawn(Mode::Light, spec);
        let t = texts(&harness);
        assert!(
            t.iter().any(|(s, ..)| s == "FIELDS"),
            "{title}: drawn {t:?}"
        );
        assert!(
            !t.iter().any(|(s, ..)| s == title),
            "raw title drawn: {t:?}"
        );
    }
}

#[test]
fn the_title_takes_the_label_style_and_the_count_the_ui_size() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.count = Some(7);
        let harness = drawn(mode, spec);
        let t = texts(&harness);
        let title = t.iter().find(|x| x.0 == "FIELDS").expect("title");
        assert_eq!(title.1, ink(sem.text.secondary), "{mode:?}");
        assert!(near(title.2, LABEL_SIZE), "{mode:?}");
        assert!(near(title.3, LABEL_TRACKING_EM * LABEL_SIZE), "{mode:?}");
        let count = t.iter().find(|x| x.0 == "7").expect("count");
        assert_eq!(count.1, ink(sem.text.secondary), "{mode:?}");
        assert!(near(count.2, UI_SIZE), "{mode:?}");
        assert!(near(count.3, 0.0), "{mode:?}");
    }
}

#[test]
fn a_count_sits_at_the_trailing_edge_and_a_long_title_is_cut_before_it() {
    for mode in MODES {
        let mut spec = Spec::new();
        spec.open = false;
        spec.count = Some(12);
        spec.title = "A very long section title that cannot possibly fit in this width";
        let harness = drawn(mode, spec);
        let h = harness.state().header;
        let t = texts(&harness);
        let count = t.iter().find(|x| x.0 == "12").expect("count drawn");
        assert!(
            near(count.4.right(), h.right() - SPACE_4),
            "{mode:?}: {:?}",
            count.4
        );
        let title = t
            .iter()
            .find(|x| x.0.starts_with("A VERY"))
            .expect("title drawn");
        assert!(title.0.ends_with('…'), "{mode:?}: not cut: {:?}", title.0);
        assert!(
            title.4.right() <= count.4.left() + 0.01,
            "{mode:?}: title {:?} overlaps count {:?}",
            title.4,
            count.4
        );
    }
}

#[test]
fn with_no_count_a_long_title_uses_the_full_width() {
    let mut spec = Spec::new();
    spec.title = "A very long section title that cannot possibly fit in this width";
    let harness = drawn(Mode::Light, spec);
    let h = harness.state().header;
    let t = texts(&harness);
    let title = t.iter().find(|x| x.0.starts_with("A VERY")).expect("title");
    assert!(title.0.ends_with('…'));
    assert!(
        title.4.right() > h.right() - SPACE_4 - 12.0,
        "{:?} in {h:?}",
        title.4
    );
    assert!(title.4.right() <= h.right() - SPACE_4 + 0.01);
}

#[test]
fn the_chevron_points_down_when_open_and_right_when_closed_in_muted_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        for open in [false, true] {
            let mut spec = Spec::new();
            spec.open = open;
            let harness = drawn(mode, spec);
            let h = harness.state().header;
            let icons = icon_paths(&harness);
            assert!(!icons.is_empty(), "{mode:?} open={open}: no chevron");
            for (c, b) in &icons {
                assert_eq!(*c, ink(sem.text.muted), "{mode:?}");
                assert!(b.left() >= h.left() + SPACE_2 - 0.01, "{b:?} in {h:?}");
                assert!(b.right() <= h.left() + SPACE_2 + ICON_SM + 0.01, "{b:?}");
                // Down is wider than tall; right is taller than wide.
                assert_eq!(b.width() > b.height(), open, "{mode:?} open={open}: {b:?}");
            }
        }
    }
}

/// The 1px rule on the header's top edge.
fn rule(rects: &[Painted], header: Rect, colour: Color32) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| {
            r.fill == colour
                && near(r.rect.height(), 1.0)
                && near(r.rect.top(), header.top())
                && near(r.rect.width(), header.width())
        })
        .copied()
        .collect()
}

fn washes(rects: &[Painted], header: Rect, colour: Color32) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| r.fill == colour && near_rect(r.rect, header))
        .copied()
        .collect()
}

fn rings(rects: &[Painted], header: Rect, colour: Color32) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| r.is_stroked() && r.stroke.color == colour && near_rect(r.rect, header))
        .copied()
        .collect()
}

#[test]
fn at_rest_the_header_has_the_rule_and_no_wash_and_no_ring() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = interacted(mode, Spec::new(), How::Rest);
        assert_eq!(
            rule(&rects, h, ink(sem.borders.divider)).len(),
            1,
            "{mode:?}: {rects:?}"
        );
        assert!(washes(&rects, h, ink(sem.containers.accordion_hover_background)).is_empty());
        assert!(rings(&rects, h, ink(sem.borders.focus)).is_empty());
    }
}

#[test]
fn hovering_the_header_fills_it_with_the_accordion_hover_fill() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = interacted(mode, Spec::new(), How::Hover);
        assert_eq!(
            washes(&rects, h, ink(sem.containers.accordion_hover_background)).len(),
            1,
            "{mode:?}: {rects:?}"
        );
        assert_eq!(
            rule(&rects, h, ink(sem.borders.divider)).len(),
            1,
            "{mode:?}"
        );
        assert!(
            rings(&rects, h, ink(sem.borders.focus)).is_empty(),
            "{mode:?}"
        );
    }
}

#[test]
fn keyboard_focus_rings_the_header_square_in_the_focus_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = interacted(mode, Spec::new(), How::Focus);
        let r = rings(&rects, h, ink(sem.borders.focus));
        assert_eq!(r.len(), 1, "{mode:?}: {rects:?}");
        assert_eq!(
            r[0].radius,
            egui::CornerRadius::ZERO,
            "{mode:?}: ring is rounded"
        );
        assert!(
            near(r[0].stroke.width, meridian_design::focus::RING_WIDTH),
            "{mode:?}"
        );
        assert!(washes(&rects, h, ink(sem.containers.accordion_hover_background)).is_empty());
    }
}

#[test]
fn the_header_is_a_button_named_by_the_title_with_its_expanded_state() {
    for open in [false, true] {
        let mut spec = Spec::new();
        spec.open = open;
        spec.title = "fields";
        let harness = drawn(Mode::Light, spec);
        // Named as the caller passed it, so a screen reader does not spell
        // capitals.
        let node = harness.get_by_role_and_label(AccessRole::Button, "fields");
        assert_eq!(node.accesskit_node().data().is_expanded(), Some(open));
    }
}
