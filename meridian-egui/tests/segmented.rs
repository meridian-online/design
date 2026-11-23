//! The segmented control, read out of the paint list and the accessibility
//! tree: the ruled box, the wash on the selected option, the corners, the
//! keycaps, and what a keyboard or screen-reader user is told when an option
//! is unavailable.

mod common;

use common::{filled, ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::{Color32, CornerRadius, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::control::KEYCAP_INSET_IN_CONTROL;
use meridian_design::control::{HEIGHT_MD, HEIGHT_SM, KEYCAP_HEIGHT_IN_CONTROL};
use meridian_design::radius::{CHIP, CONTROL};
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::SPACE_4;
use meridian_egui::{segmented, theme, Mode, Segmented, SegmentedOption, SegmentedResponse};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const LABELS: [&str; 3] = ["Grid", "Chart", "Model"];
const KEYS: [&str; 3] = ["G", "C", "M"];

#[derive(Clone, Copy)]
struct Spec {
    count: usize,
    selected: usize,
    keys: bool,
    /// The option, if any, that is unavailable.
    off: Option<usize>,
    width: f32,
}

impl Spec {
    fn new(selected: usize) -> Self {
        Self {
            count: 3,
            selected,
            keys: true,
            off: None,
            width: 400.0,
        }
    }
}

struct Out {
    rect: Rect,
    clicked: Option<usize>,
    pointed: Option<(usize, Rect)>,
}

fn options(spec: Spec) -> Vec<SegmentedOption<'static>> {
    (0..spec.count)
        .map(|i| {
            let mut o = SegmentedOption::new(LABELS[i]);
            if spec.keys {
                o = o.keystroke(KEYS[i]);
            }
            if spec.off == Some(i) {
                o = o.unavailable("Nothing to show");
            }
            o
        })
        .collect()
}

fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(spec.width + 40.0, 120.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(spec.width, 40.0), |ui| {
                    let opts = options(spec);
                    let r: SegmentedResponse = segmented(ui, Segmented::new(&opts, spec.selected));
                    out.rect = r.response.rect;
                    if r.clicked.is_some() {
                        out.clicked = r.clicked;
                    }
                    if r.pointed_unavailable.is_some() {
                        out.pointed = r.pointed_unavailable;
                    }
                });
            },
            Out {
                rect: Rect::NOTHING,
                clicked: None,
                pointed: None,
            },
        );
    harness.run();
    harness
}

#[derive(Clone, Copy, Debug)]
enum Interact {
    Rest,
    Hover,
    Pressed,
    Focus,
}

fn option_rect(harness: &Harness<'_, Out>, i: usize) -> Rect {
    harness
        .get_by_role_and_label(AccessRole::RadioButton, LABELS[i])
        .rect()
}

/// The frame's paint list with option `i` interacted with, and the option rects.
fn interacted(mode: Mode, spec: Spec, i: usize, how: Interact) -> (Vec<Painted>, Vec<Rect>, Rect) {
    let mut harness = drawn(mode, spec);
    let node = || harness_node(&harness, i);
    match how {
        Interact::Rest => {}
        Interact::Hover => {
            node().hover();
            harness.run();
        }
        Interact::Pressed => {
            node().hover();
            harness.run();
            let pos = option_rect(&harness, i).center();
            harness.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            });
            harness.run();
        }
        Interact::Focus => {
            node().focus();
            harness.run();
        }
    }
    let rects = (0..spec.count).map(|j| option_rect(&harness, j)).collect();
    let group = harness.state().rect;
    (painted(&harness, mode), rects, group)
}

fn harness_node<'h>(harness: &'h Harness<'_, Out>, i: usize) -> egui_kittest::Node<'h> {
    harness.get_by_role_and_label(AccessRole::RadioButton, LABELS[i])
}

fn corners(nw: u8, ne: u8, sw: u8, se: u8) -> CornerRadius {
    CornerRadius { nw, ne, sw, se }
}

/// The fills that are an option's own: not a keycap, not a 1px rule.
fn wash(rects: &[Painted], fill: Color32, option: Rect) -> Vec<Painted> {
    filled(rects, fill)
        .into_iter()
        .filter(|r| near_rect(r.rect, option))
        .collect()
}

#[test]
fn the_selected_option_takes_the_selection_wash_and_rule_in_its_own_ink() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        for selected in 0..3 {
            let (rects, opts, _) = interacted(mode, Spec::new(selected), 0, Interact::Rest);
            let w = wash(&rects, ink(n.background.selected), opts[selected]);
            assert_eq!(w.len(), 1, "{mode:?} {selected}: {rects:?}");
            let rule = rects.iter().any(|r| {
                r.fill == Color32::TRANSPARENT
                    && r.stroke.color == ink(n.border.selected)
                    && near(r.stroke.width, 1.0)
                    && near_rect(r.rect, opts[selected])
            });
            assert!(rule, "{mode:?} {selected}: no selected rule in {rects:?}");
            // And only that option: the control case.
            for other in (0..3).filter(|j| *j != selected) {
                assert!(
                    wash(&rects, ink(n.background.selected), opts[other]).is_empty(),
                    "{mode:?}: option {other} took the wash"
                );
            }
        }
    }
}

#[test]
fn the_box_is_ruled_once_in_the_neutral_border_and_options_are_divided_by_rules() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let (rects, opts, group) = interacted(mode, Spec::new(0), 0, Interact::Rest);
        let boxed: Vec<_> = rects
            .iter()
            .filter(|r| {
                r.fill == Color32::TRANSPARENT
                    && r.stroke.color == ink(n.border.base)
                    && near_rect(r.rect, group)
            })
            .collect();
        assert_eq!(boxed.len(), 1, "{mode:?}: {rects:?}");
        assert!(near(boxed[0].stroke.width, 1.0));
        assert_eq!(boxed[0].radius, CornerRadius::same(CONTROL as u8));
        let dividers: Vec<_> = filled(&rects, ink(n.border.base))
            .into_iter()
            .filter(|r| near(r.rect.width(), 1.0) && near(r.rect.height(), group.height()))
            .collect();
        assert_eq!(dividers.len(), 2, "{mode:?}: {rects:?}");
        for (d, o) in dividers.iter().zip(&opts[1..]) {
            assert!(near(d.rect.left(), o.left()), "{mode:?}: {d:?} vs {o:?}");
        }
    }
}

#[test]
fn only_the_outer_corners_round() {
    for mode in MODES {
        let c = CONTROL as u8;
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let want = |i: usize, count: usize| {
            corners(
                if i == 0 { c } else { 0 },
                if i + 1 == count { c } else { 0 },
                if i == 0 { c } else { 0 },
                if i + 1 == count { c } else { 0 },
            )
        };
        for selected in 0..3 {
            let (rects, opts, _) = interacted(mode, Spec::new(selected), 0, Interact::Rest);
            let w = wash(&rects, ink(n.background.selected), opts[selected]);
            assert_eq!(w.len(), 1);
            assert_eq!(w[0].radius, want(selected, 3), "{mode:?} wash {selected}");
            let rule = rects
                .iter()
                .find(|r| {
                    r.fill == Color32::TRANSPARENT
                        && r.stroke.color == ink(n.border.selected)
                        && near_rect(r.rect, opts[selected])
                })
                .expect("the selected rule");
            assert_eq!(rule.radius, want(selected, 3), "{mode:?} rule {selected}");
        }
        // A hovered inner option is square on all four corners.
        let (rects, opts, _) = interacted(mode, Spec::new(0), 1, Interact::Hover);
        let h = wash(&rects, ink(n.background.hover), opts[1]);
        assert_eq!(h.len(), 1);
        assert_eq!(h[0].radius, CornerRadius::ZERO);
        // The focus ring on the last option follows the same corners.
        let (rects, opts, _) = interacted(mode, Spec::new(0), 2, Interact::Focus);
        let ring = ring_on(&rects, opts[2], mode).expect("a ring");
        assert_eq!(ring.radius.nw, 0);
        assert_eq!(ring.radius.sw, 0);
        assert!(
            ring.radius.ne > 0 && ring.radius.se > 0,
            "{:?}",
            ring.radius
        );
        // A lone option is the whole box and round throughout.
        let mut one = Spec::new(0);
        one.count = 1;
        let (rects, opts, _) = interacted(mode, one, 0, Interact::Rest);
        let w = wash(&rects, ink(n.background.selected), opts[0]);
        assert_eq!(w[0].radius, CornerRadius::same(c));
    }
}

fn ring_on(rects: &[Painted], option: Rect, mode: Mode) -> Option<Painted> {
    let focus = ink(semantic(mode.is_dark()).borders.focus);
    rects
        .iter()
        .find(|r| {
            r.fill == Color32::TRANSPARENT
                && r.stroke.color == focus
                && near(r.stroke.width, 2.0)
                && near_rect(r.rect, option)
        })
        .copied()
}

#[test]
fn an_option_under_the_pointer_takes_the_hover_fill_and_one_held_down_the_active_fill() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let (rects, opts, _) = interacted(mode, Spec::new(0), 1, Interact::Rest);
        assert!(wash(&rects, ink(n.background.hover), opts[1]).is_empty());
        assert!(wash(&rects, ink(n.background.active), opts[1]).is_empty());
        let (rects, opts, _) = interacted(mode, Spec::new(0), 1, Interact::Hover);
        assert_eq!(wash(&rects, ink(n.background.hover), opts[1]).len(), 1);
        for other in [0, 2] {
            assert!(wash(&rects, ink(n.background.hover), opts[other]).is_empty());
        }
        let (rects, opts, _) = interacted(mode, Spec::new(0), 1, Interact::Pressed);
        assert_eq!(wash(&rects, ink(n.background.active), opts[1]).len(), 1);
        assert!(wash(&rects, ink(n.background.hover), opts[1]).is_empty());
    }
}

#[test]
fn the_selected_option_keeps_its_wash_under_the_pointer() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let (rects, opts, _) = interacted(mode, Spec::new(1), 1, Interact::Hover);
        assert_eq!(wash(&rects, ink(n.background.selected), opts[1]).len(), 1);
        assert!(wash(&rects, ink(n.background.hover), opts[1]).is_empty());
    }
}

#[test]
fn a_focused_option_wears_the_ring_and_an_unfocused_one_does_not() {
    for mode in MODES {
        let (rects, opts, _) = interacted(mode, Spec::new(0), 1, Interact::Focus);
        assert!(ring_on(&rects, opts[1], mode).is_some(), "{rects:?}");
        assert!(ring_on(&rects, opts[0], mode).is_none());
        let (rects, opts, _) = interacted(mode, Spec::new(0), 1, Interact::Rest);
        assert!(ring_on(&rects, opts[1], mode).is_none());
    }
}

fn label_ink(harness: &Harness<'_, Out>, text: &str) -> Color32 {
    for c in &harness.output().shapes {
        if let egui::Shape::Text(t) = &c.shape {
            if t.galley.job.text == text {
                return t.galley.job.sections[0].format.color;
            }
        }
    }
    panic!("{text:?} was not drawn");
}

fn label_left(harness: &Harness<'_, Out>, text: &str) -> f32 {
    for c in &harness.output().shapes {
        if let egui::Shape::Text(t) = &c.shape {
            if t.galley.job.text == text {
                return t.galley.rect.translate(t.pos.to_vec2()).left();
            }
        }
    }
    panic!("{text:?} was not drawn");
}

#[test]
fn labels_take_the_selected_the_tab_and_the_disabled_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let n = sem.role(Role::Neutral).to_owned();
        let mut spec = Spec::new(0);
        spec.off = Some(2);
        let harness = drawn(mode, spec);
        assert_eq!(label_ink(&harness, "Grid"), ink(n.foreground.selected));
        assert_eq!(label_ink(&harness, "Chart"), ink(sem.tabs.foreground));
        assert_eq!(label_ink(&harness, "Model"), ink(sem.text.disabled));
        assert_ne!(ink(n.foreground.selected), ink(sem.tabs.foreground));
    }
}

#[test]
fn it_is_twenty_tall_with_no_keystrokes_and_twenty_four_with_one() {
    for mode in MODES {
        let mut spec = Spec::new(0);
        spec.keys = false;
        let (_, opts, group) = interacted(mode, spec, 0, Interact::Rest);
        assert!(near(group.height(), HEIGHT_SM), "{group:?}");
        assert!(opts.iter().all(|o| near(o.height(), HEIGHT_SM)));

        let (_, opts, group) = interacted(mode, Spec::new(0), 0, Interact::Rest);
        assert!(near(group.height(), HEIGHT_MD), "{group:?}");
        assert!(opts.iter().all(|o| near(o.height(), HEIGHT_MD)));

        // One keystroke is enough, and every option takes the taller rung.
        let mut h = Harness::builder()
            .with_size(egui::vec2(440.0, 120.0))
            .build_ui(move |ui| {
                theme::apply(ui.ctx(), mode);
                let opts = [
                    SegmentedOption::new("Grid"),
                    SegmentedOption::new("Chart").keystroke("C"),
                ];
                segmented(ui, Segmented::new(&opts, 0));
            });
        h.run();
        let r = h
            .get_by_role_and_label(AccessRole::RadioButton, "Grid")
            .rect();
        assert!(near(r.height(), HEIGHT_MD), "{r:?}");
    }
}

#[test]
fn options_abut_and_fill_the_box() {
    let (_, opts, group) = interacted(Mode::Light, Spec::new(0), 0, Interact::Rest);
    assert!(near(opts[0].left(), group.left()));
    assert!(near(opts[2].right(), group.right()));
    for w in opts.windows(2) {
        assert!(near(w[0].right(), w[1].left()), "{opts:?}");
    }
}

#[test]
fn a_key_sits_four_from_the_top_bottom_and_trailing_edge_of_its_option() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, opts, group) = interacted(mode, Spec::new(0), 0, Interact::Rest);
        let caps: Vec<_> = rects
            .iter()
            .filter(|r| r.is_stroked() && r.radius == CornerRadius::same(CHIP as u8))
            .collect();
        assert_eq!(caps.len(), 3, "{mode:?}: {rects:?}");
        for (cap, o) in caps.iter().zip(&opts) {
            assert!(near(cap.rect.height(), KEYCAP_HEIGHT_IN_CONTROL));
            assert!(near(cap.rect.top() - group.top(), KEYCAP_INSET_IN_CONTROL));
            assert!(near(
                group.bottom() - cap.rect.bottom(),
                KEYCAP_INSET_IN_CONTROL
            ));
            assert!(
                near(o.right() - cap.rect.right(), KEYCAP_INSET_IN_CONTROL),
                "{mode:?}: {cap:?} in {o:?}"
            );
            assert_eq!(cap.fill, ink(sem.surfaces.sunken));
        }
    }
}

#[test]
fn a_label_sits_space_4_from_its_options_leading_edge_and_an_unavailable_key_is_dimmed() {
    let harness = drawn(Mode::Light, Spec::new(0));
    for (i, label) in LABELS.iter().enumerate() {
        let o = option_rect(&harness, i);
        assert!(
            near(label_left(&harness, label) - o.left(), SPACE_4 + 1.0),
            "option {i}"
        );
    }
    // A keyless option in a 24 control is padded SPACE_4 each side.
    let mut h = Harness::builder()
        .with_size(egui::vec2(440.0, 120.0))
        .build_ui(|ui| {
            theme::apply(ui.ctx(), Mode::Light);
            let opts = [
                SegmentedOption::new("Grid"),
                SegmentedOption::new("Chart").keystroke("C"),
            ];
            segmented(ui, Segmented::new(&opts, 0));
        });
    h.run();
    let grid = h
        .get_by_role_and_label(AccessRole::RadioButton, "Grid")
        .rect();
    let left = label_left_in(&h, "Grid");
    assert!(near(left - grid.left(), SPACE_4 + 1.0));
    let text_w = label_width_in(&h, "Grid");
    assert!(
        near(grid.width(), text_w.ceil() + 2.0 * (SPACE_4 + 1.0)),
        "{grid:?} {text_w}"
    );

    let mut spec = Spec::new(0);
    spec.off = Some(2);
    let key_ink = |h: &Harness<'_, Out>| label_ink(h, "M");
    assert_ne!(
        key_ink(&drawn(Mode::Light, spec)),
        key_ink(&drawn(Mode::Light, Spec::new(0))),
        "the key was not dimmed"
    );
}

fn label_left_in<S>(h: &Harness<'_, S>, text: &str) -> f32 {
    for c in &h.output().shapes {
        if let egui::Shape::Text(t) = &c.shape {
            if t.galley.job.text == text {
                return t.galley.rect.translate(t.pos.to_vec2()).left();
            }
        }
    }
    panic!("{text:?} was not drawn");
}

fn label_width_in<S>(h: &Harness<'_, S>, text: &str) -> f32 {
    for c in &h.output().shapes {
        if let egui::Shape::Text(t) = &c.shape {
            if t.galley.job.text == text {
                return t.galley.rect.width();
            }
        }
    }
    panic!("{text:?} was not drawn");
}

#[test]
fn a_click_on_an_available_option_reports_it_including_the_selected_one() {
    let mut harness = drawn(Mode::Light, Spec::new(0));
    assert_eq!(harness.state().clicked, None);
    harness_node(&harness, 1).click();
    harness.run();
    assert_eq!(harness.state().clicked, Some(1));
    harness.state_mut().clicked = None;
    harness_node(&harness, 0).click();
    harness.run();
    assert_eq!(harness.state().clicked, Some(0));
}

#[test]
fn an_unavailable_option_reports_its_rect_and_no_click() {
    let mut spec = Spec::new(0);
    spec.off = Some(1);
    let mut harness = drawn(Mode::Light, spec);
    assert_eq!(
        harness.state().pointed,
        None,
        "pointed at with nothing over it"
    );

    harness_node(&harness, 1).hover();
    harness.run();
    let r = option_rect(&harness, 1);
    let (i, pointed) = harness.state().pointed.expect("pointed at, it reports");
    assert_eq!(i, 1);
    assert!(near_rect(pointed, r), "{pointed:?} is not {r:?}");

    harness_node(&harness, 1).click();
    harness.run();
    assert_eq!(
        harness.state().clicked,
        None,
        "an unavailable option clicked"
    );

    // An available option next to it does click, so the above is not a dead harness.
    harness_node(&harness, 2).click();
    harness.run();
    assert_eq!(harness.state().clicked, Some(2));
}

#[test]
fn an_unavailable_option_is_disabled_with_its_reason_and_takes_no_focus() {
    let mut spec = Spec::new(0);
    spec.off = Some(1);
    let mut harness = drawn(Mode::Light, spec);
    let off = harness_node(&harness, 1);
    assert!(off.accesskit_node().is_disabled());
    assert_eq!(
        off.accesskit_node().description().as_deref(),
        Some("Nothing to show")
    );
    let on = harness_node(&harness, 2);
    assert!(!on.accesskit_node().is_disabled());
    assert_eq!(on.accesskit_node().description(), None);

    // The harness can focus an available option, so one that stays unringed is
    // the control's doing.
    harness_node(&harness, 2).focus();
    harness.run();
    let (rects, opts) = (
        painted(&harness, Mode::Light),
        [option_rect(&harness, 1), option_rect(&harness, 2)],
    );
    assert!(
        ring_on(&rects, opts[1], Mode::Light).is_some(),
        "the harness cannot focus an option"
    );
    harness_node(&harness, 1).focus();
    harness.run();
    let rects = painted(&harness, Mode::Light);
    assert!(
        ring_on(&rects, opts[0], Mode::Light).is_none(),
        "an unavailable option was focused"
    );
}

#[test]
fn the_group_is_a_radio_group_of_radio_buttons_and_the_selected_one_is_checked() {
    let harness = drawn(Mode::Light, Spec::new(1));
    let group = harness
        .query_by_role(AccessRole::RadioGroup)
        .expect("a radio group");
    let radios: Vec<_> = harness.query_all_by_role(AccessRole::RadioButton).collect();
    assert_eq!(radios.len(), 3);
    for (i, r) in radios.iter().enumerate() {
        let checked = r.accesskit_node().toggled() == Some(egui::accesskit::Toggled::True);
        assert_eq!(checked, i == 1, "option {i}");
        assert_eq!(
            r.accesskit_node().parent_id(),
            Some(group.accesskit_node().id()),
            "option {i}'s parent"
        );
    }
}

#[test]
fn no_options_draws_nothing_and_does_not_panic() {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(200.0, 80.0))
        .build_ui_state(
            |ui, size: &mut egui::Vec2| {
                theme::apply(ui.ctx(), Mode::Light);
                let r = segmented(ui, Segmented::new(&[], 0));
                *size = r.response.rect.size();
                assert_eq!(r.clicked, None);
            },
            egui::vec2(1.0, 1.0),
        );
    harness.run();
    assert_eq!(*harness.state(), egui::Vec2::ZERO);
    assert!(painted(&harness, Mode::Light).is_empty());
}

#[test]
fn a_selected_index_past_the_end_selects_nothing() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let (rects, _, group) = interacted(mode, Spec::new(7), 0, Interact::Rest);
        assert!(
            filled(&rects, ink(n.background.selected)).is_empty(),
            "{rects:?}"
        );
        assert!(!rects
            .iter()
            .any(|r| r.stroke.color == ink(n.border.selected)));
        // The control still draws: its box is there (a control case for the above).
        assert!(rects.iter().any(|r| near_rect(r.rect, group)));
        let harness = drawn(mode, Spec::new(7));
        for r in harness.query_all_by_role(AccessRole::RadioButton) {
            assert_ne!(
                r.accesskit_node().toggled(),
                Some(egui::accesskit::Toggled::True)
            );
        }
        assert_eq!(
            harness.query_all_by_role(AccessRole::RadioButton).count(),
            3
        );
    }
}

#[test]
fn a_selected_unavailable_option_keeps_the_selected_ink_and_a_dimmed_key() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let mut spec = Spec::new(2);
        spec.off = Some(2);
        let h = drawn(mode, spec);
        assert_eq!(label_ink(&h, "Model"), ink(n.foreground.selected));
        let mut live = Spec::new(2);
        live.off = None;
        assert_ne!(label_ink(&h, "M"), label_ink(&drawn(mode, live), "M"));
    }
}

const LONG: &str = "A rather long activity name";

/// Three keyed options, the middle one long, in a ui `width` wide.
fn narrow(width: f32) -> Harness<'static, ()> {
    let mut h = Harness::builder()
        .with_size(egui::vec2(700.0, 80.0))
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), Mode::Light);
            ui.allocate_ui(egui::vec2(width, 40.0), |ui| {
                let opts = [
                    SegmentedOption::new("Grid").keystroke("G"),
                    SegmentedOption::new(LONG).keystroke("C"),
                    SegmentedOption::new("Model").keystroke("M"),
                ];
                segmented(ui, Segmented::new(&opts, 0));
            });
        });
    h.run();
    h
}

fn option_rects<S>(h: &Harness<'_, S>, labels: &[&str]) -> Vec<Rect> {
    labels
        .iter()
        .map(|l| h.get_by_role_and_label(AccessRole::RadioButton, l).rect())
        .collect()
}

fn text_rects<S>(h: &Harness<'_, S>) -> Vec<(String, Rect)> {
    let mut out = Vec::new();
    for c in &h.output().shapes {
        if let egui::Shape::Text(t) = &c.shape {
            let s: String = t.galley.rows.iter().map(|r| r.text()).collect();
            out.push((s, t.galley.rect.translate(t.pos.to_vec2())));
        }
    }
    out
}

#[test]
fn a_narrow_control_shrinks_only_its_labels_and_keeps_every_key_in_its_option() {
    let labels = ["Grid", LONG, "Model"];
    let wide = narrow(650.0);
    let natural = option_rects(&wide, &labels);
    let natural_w = natural[2].right() - natural[0].left();
    // The wide control cuts nothing: the control case.
    assert!(text_rects(&wide).iter().any(|(t, _)| t == LONG));

    let h = narrow(natural_w - 40.0);
    let opts = option_rects(&h, &labels);
    let group_w = opts[2].right() - opts[0].left();
    assert!(group_w <= natural_w - 40.0 + 0.01, "{group_w}");
    let texts = text_rects(&h);
    let caps: Vec<Rect> = painted(&h, Mode::Light)
        .iter()
        .filter(|r| r.is_stroked() && r.radius == CornerRadius::same(CHIP as u8))
        .map(|r| r.rect)
        .collect();
    assert_eq!(caps.len(), 3);
    for (cap, o) in caps.iter().zip(&opts) {
        assert!(
            near(o.right() - cap.right(), KEYCAP_INSET_IN_CONTROL),
            "{cap:?} in {o:?}"
        );
        assert!(cap.left() > o.left(), "{cap:?} left its option {o:?}");
    }
    // A short label is whole and a long one is cut.
    assert!(texts.iter().any(|(t, _)| t == "Grid"), "Grid whole");
    assert!(texts.iter().any(|(t, _)| t == "Model"));
    assert!(
        near(opts[0].width(), natural[0].width()),
        "a short option shrank"
    );
    assert!(
        texts.iter().all(|(t, _)| t != LONG),
        "the long label was not cut"
    );
    assert!(opts[1].width() < natural[1].width());
    // No label reaches its own key, and none leaves its option.
    for (i, l) in labels.iter().enumerate() {
        let (_, r) = texts
            .iter()
            .find(|(t, _)| t.starts_with(&l[..2]) && t != "G" && t != "C" && t != "M")
            .expect("a label");
        assert!(
            r.right() <= caps[i].left() + 0.01,
            "{l}: {r:?} reaches {:?}",
            caps[i]
        );
        assert!(r.left() >= opts[i].left());
    }
}

#[test]
fn a_control_narrower_than_its_fixed_parts_overlaps_nothing_and_paints_inside_the_room() {
    let labels = ["Grid", LONG, "Model"];
    let room = 60.0;
    let h = narrow(room);
    let opts = option_rects(&h, &labels);
    for w in opts.windows(2) {
        assert!(w[0].right() <= w[1].left() + 0.01, "{opts:?}");
    }
    // It really is wider than the room, so the clip below is what is tested.
    assert!(opts[2].right() - opts[0].left() > room);
    let edge = opts[0].left() + room;
    let page = theme::meridian_visuals(Mode::Light).panel_fill;
    let mut seen = 0;
    for c in &h.output().shapes {
        if let egui::Shape::Rect(r) = &c.shape {
            if r.fill == page {
                continue;
            }
        }
        seen += 1;
        assert!(
            c.clip_rect.right() <= edge + 0.5 || !has_rect_past(&c.shape, edge),
            "painted past the room: {:?} under {:?}",
            c.shape,
            c.clip_rect
        );
    }
    assert!(seen > 3);
}

fn has_rect_past(shape: &egui::Shape, edge: f32) -> bool {
    match shape {
        egui::Shape::Rect(r) => r.rect.right() > edge + 0.5,
        egui::Shape::Text(t) => t.galley.rect.translate(t.pos.to_vec2()).right() > edge + 0.5,
        egui::Shape::Vec(v) => v.iter().any(|s| has_rect_past(s, edge)),
        _ => false,
    }
}

#[test]
fn the_part_of_a_clipped_option_past_the_room_takes_no_hover_and_no_click() {
    struct St {
        clicked: Option<usize>,
        pointed: Option<(usize, Rect)>,
    }
    let room = 60.0;
    let mut h = Harness::builder()
        .with_size(egui::vec2(700.0, 80.0))
        .build_ui_state(
            move |ui, st: &mut St| {
                theme::apply(ui.ctx(), Mode::Light);
                ui.allocate_ui(egui::vec2(room, 40.0), |ui| {
                    let opts = [
                        SegmentedOption::new("Grid").keystroke("G"),
                        SegmentedOption::new(LONG).keystroke("C").unavailable("No"),
                        SegmentedOption::new("Model").keystroke("M"),
                    ];
                    let r = segmented(ui, Segmented::new(&opts, 0));
                    st.clicked = st.clicked.or(r.clicked);
                    st.pointed = r.pointed_unavailable.or(st.pointed);
                });
            },
            St {
                clicked: None,
                pointed: None,
            },
        );
    h.run();
    let last = h
        .get_by_role_and_label(AccessRole::RadioButton, "Model")
        .rect();
    let second = h
        .get_by_role_and_label(AccessRole::RadioButton, LONG)
        .rect();
    let first = h
        .get_by_role_and_label(AccessRole::RadioButton, "Grid")
        .rect();
    let edge = first.left() + room;
    assert!(
        last.right() > edge + 10.0,
        "the control fits its room: the test is idle"
    );
    let beyond = egui::pos2(
        (last.right() + edge.max(last.left())) / 2.0 + 5.0,
        last.center().y,
    );
    assert!(
        last.contains(beyond),
        "the point is not inside the last option"
    );
    h.event(egui::Event::PointerMoved(beyond));
    h.run();
    h.event(egui::Event::PointerButton {
        pos: beyond,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    });
    h.run();
    h.event(egui::Event::PointerButton {
        pos: beyond,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    });
    h.run();
    assert_eq!(h.state().clicked, None, "a click past the room landed");
    // Control: a point inside the room, in the first option, does click.
    let inside = egui::pos2(edge - 30.0, last.center().y);
    let first = h
        .get_by_role_and_label(AccessRole::RadioButton, "Grid")
        .rect();
    let inside = if first.contains(inside) {
        inside
    } else {
        first.center()
    };
    h.event(egui::Event::PointerMoved(inside));
    h.run();
    for pressed in [true, false] {
        h.event(egui::Event::PointerButton {
            pos: inside,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        });
        h.run();
    }
    assert_eq!(h.state().clicked, Some(0), "the harness cannot click");
    // The unavailable option's reported rect stays inside the room.
    h.event(egui::Event::PointerMoved(
        second.left_center() + egui::vec2(20.0, 0.0),
    ));
    h.run();
    if let Some((_, r)) = h.state().pointed {
        assert!(r.right() <= edge + 0.5, "{r:?} past {edge}");
    }
}

#[test]
fn a_right_to_left_control_paints_nothing_left_of_its_room() {
    let room = 60.0;
    let mut h = Harness::builder()
        .with_size(egui::vec2(700.0, 80.0))
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), Mode::Light);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.allocate_ui(egui::vec2(room, 40.0), |ui| {
                    let opts = [
                        SegmentedOption::new("Grid").keystroke("G"),
                        SegmentedOption::new(LONG).keystroke("C"),
                        SegmentedOption::new("Model").keystroke("M"),
                    ];
                    segmented(ui, Segmented::new(&opts, 0));
                });
            });
        });
    h.run();
    let page = theme::meridian_visuals(Mode::Light).panel_fill;
    let rects = painted(&h, Mode::Light);
    assert!(rects.iter().any(|r| r.fill != page), "nothing painted");
    let opts = option_rects(&h, &["Grid", LONG, "Model"]);
    let right = opts.iter().map(|r| r.right()).fold(f32::MIN, f32::max);
    let left = opts.iter().map(|r| r.left()).fold(f32::MAX, f32::min);
    assert!(
        left < right - room,
        "the control fits its room: the test is idle"
    );
    // Every shape's visible part lies in the room: 60 wide, ending at the ui's right.
    let room_right = right.min(692.0);
    for c in &h.output().shapes {
        if let egui::Shape::Rect(r) = &c.shape {
            if r.fill == page {
                continue;
            }
            let seen = r.rect.intersect(c.clip_rect);
            assert!(
                seen.left() >= room_right - room - 0.5 || seen.width() <= 0.0,
                "painted left of the room: {:?} under {:?}",
                r.rect,
                c.clip_rect
            );
        }
    }
}

// ─── one baseline ───────────────────────────────────────────────────────────

#[test]
fn every_option_label_sits_on_one_baseline_descender_or_not() {
    const MIXED: [&str; 3] = ["Log", "Quality", "Rows"];
    for mode in MODES {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(440.0, 120.0))
            .build_ui(move |ui| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(400.0, 40.0), |ui| {
                    let opts: Vec<SegmentedOption<'static>> =
                        MIXED.iter().map(|l| SegmentedOption::new(l)).collect();
                    segmented(ui, Segmented::new(&opts, 1));
                });
            });
        harness.run();
        let lines = common::text_baselines(&harness);
        let texts: Vec<&str> = lines.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(texts, MIXED, "{mode:?}: the labels painted");
        let first = lines[0].1;
        for (text, y) in &lines {
            assert!(
                near(*y, first),
                "{mode:?}: {text:?} sits on a baseline at {y}, {:?} at {first}",
                MIXED[0]
            );
        }
    }
}
