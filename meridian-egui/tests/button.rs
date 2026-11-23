//! The button, read out of the paint list and the accessibility tree: what it
//! filled, stroked and inked in each weight and state, where its keycap sits,
//! and what a keyboard or screen-reader user is told when it is unavailable.

mod common;

use common::{assert_corner, filled, ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::{Color32, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::control::{KEYCAP_HEIGHT_IN_CONTROL, KEYCAP_INSET_IN_CONTROL};
use meridian_design::radius::{CHIP, CONTROL};
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{SPACE_3, SPACE_4};
use meridian_egui::{button, icons, theme, Button, ButtonResponse, Mode, Weight};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const LABEL: &str = "Run";
const KEY: &str = "\u{2318}\u{21a9}";

#[derive(Clone, Copy)]
struct Spec {
    weight: Weight,
    dense: bool,
    unavailable: bool,
    key: Option<&'static str>,
    icon: bool,
    icon_only: bool,
    label: &'static str,
    width: f32,
}

impl Spec {
    fn new(weight: Weight) -> Self {
        Self {
            weight,
            dense: false,
            unavailable: false,
            key: None,
            icon: false,
            icon_only: false,
            label: LABEL,
            width: 400.0,
        }
    }
}

struct Out {
    rect: Rect,
    clicked: bool,
    pointed: Option<Rect>,
    focused: bool,
}

fn build(spec: Spec) -> Button<'static> {
    let mut b = if spec.icon_only {
        Button::icon_only(&icons::ALL[0], spec.label)
    } else {
        Button::new(spec.label)
    };
    if spec.icon && !spec.icon_only {
        b = b.icon(&icons::ALL[0]);
    }
    if let Some(k) = spec.key {
        b = b.keystroke(k);
    }
    b = b.weight(spec.weight);
    if spec.dense {
        b = b.dense();
    }
    if spec.unavailable {
        b = b.unavailable("Nothing to run");
    }
    b
}

fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(spec.width + 40.0, 120.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(spec.width, 40.0), |ui| {
                    let r: ButtonResponse = button(ui, build(spec));
                    out.rect = r.response.rect;
                    out.clicked |= r.clicked;
                    out.focused = r.response.has_focus();
                    if r.pointed_unavailable.is_some() {
                        out.pointed = r.pointed_unavailable;
                    }
                });
            },
            Out {
                rect: Rect::NOTHING,
                clicked: false,
                pointed: None,
                focused: false,
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

fn interacted(mode: Mode, spec: Spec, how: Interact) -> (Vec<Painted>, Rect) {
    let mut harness = drawn(mode, spec);
    match how {
        Interact::Rest => {}
        Interact::Hover => {
            harness
                .get_by_role_and_label(AccessRole::Button, spec.label)
                .hover();
            harness.run();
        }
        Interact::Pressed => {
            harness
                .get_by_role_and_label(AccessRole::Button, spec.label)
                .hover();
            harness.run();
            let pos = harness.state().rect.center();
            harness.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            });
            harness.run();
        }
        Interact::Focus => {
            harness
                .get_by_role_and_label(AccessRole::Button, spec.label)
                .focus();
            harness.run();
        }
    }
    let rect = harness.state().rect;
    (painted(&harness, mode), rect)
}

/// The rects painted on the button's own box.
fn on_box(rects: &[Painted], b: Rect) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| near_rect(r.rect, b))
        .copied()
        .collect()
}

/// One weight at one state: the fill and stroke of its box, and its corner.
fn assert_box(mode: Mode, spec: Spec, how: Interact, fill: Color32, stroke: Option<Color32>) {
    let (rects, b) = interacted(mode, spec, how);
    let boxes: Vec<_> = on_box(&rects, b)
        .into_iter()
        .filter(|r| r.fill != Color32::TRANSPARENT || r.is_stroked())
        .collect();
    let what = format!("{:?} {mode:?} {how:?}", spec.weight);
    if fill == Color32::TRANSPARENT && stroke.is_none() {
        // Nothing drawn on the box is also an answer. The focus ring is the
        // one unfilled stroke allowed, and only the ring's own ink.
        let ring = ink(semantic(mode.is_dark()).borders.focus);
        let rest: Vec<_> = boxes
            .iter()
            .filter(|r| !(r.fill == Color32::TRANSPARENT && r.stroke.color == ring))
            .collect();
        assert!(
            rest.iter()
                .all(|r| !r.is_stroked() && r.fill == Color32::TRANSPARENT),
            "{what}: painted {rest:?}"
        );
        return;
    }
    let fills = filled(&boxes, fill);
    assert_eq!(
        fills.len(),
        1,
        "{what}: wanted one fill {fill:?}, got {boxes:?}"
    );
    assert_corner(&fills, CONTROL, &what);
    match stroke {
        Some(s) => {
            assert!(fills[0].is_stroked(), "{what}: the box has no rule");
            assert_eq!(fills[0].stroke.color, s, "{what}: rule colour");
            assert!(near(fills[0].stroke.width, 1.0), "{what}: rule width");
        }
        None => assert!(!fills[0].is_stroked(), "{what}: the box has a rule"),
    }
}

/// The keycap boxes in the frame: the rects at the chip corner that carry a
/// rule. (The button's own fill can be the keycap's face colour, so they are
/// told apart by their corner, not their fill.)
fn key_boxes(rects: &[Painted]) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| r.is_stroked() && r.radius == egui::CornerRadius::same(CHIP as u8))
        .copied()
        .collect()
}

/// A focus ring: a stroked, unfilled rect in `colour` on the button's box.
fn has_ring(rects: &[Painted], b: Rect, colour: Color32, inset: f32) -> bool {
    rects.iter().any(|r| {
        r.fill == Color32::TRANSPARENT
            && r.stroke.color == colour
            && near_rect(r.rect, b.shrink(inset))
    })
}

#[test]
fn a_default_button_wears_the_neutral_role_in_each_state() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let spec = Spec::new(Weight::Default);
        assert_box(
            mode,
            spec,
            Interact::Rest,
            ink(n.background.base),
            Some(ink(n.border.base)),
        );
        assert_box(
            mode,
            spec,
            Interact::Hover,
            ink(n.background.hover),
            Some(ink(n.border.hover)),
        );
        assert_box(
            mode,
            spec,
            Interact::Pressed,
            ink(n.background.active),
            Some(ink(n.border.active)),
        );
        assert_box(
            mode,
            spec,
            Interact::Focus,
            ink(n.background.base),
            Some(ink(n.border.base)),
        );
        let (rects, b) = interacted(mode, spec, Interact::Focus);
        assert!(
            has_ring(&rects, b, ink(semantic(mode.is_dark()).borders.focus), 0.0),
            "{mode:?}: no focus ring on {b:?}: {rects:?}"
        );
    }
}

#[test]
fn a_primary_button_wears_the_accent_role_and_rings_in_the_on_solid_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let a = sem.role(Role::Accent).to_owned();
        let spec = Spec::new(Weight::Primary);
        assert_box(
            mode,
            spec,
            Interact::Rest,
            ink(a.background.base),
            Some(ink(a.border.base)),
        );
        assert_box(
            mode,
            spec,
            Interact::Hover,
            ink(a.background.hover),
            Some(ink(a.border.hover)),
        );
        assert_box(
            mode,
            spec,
            Interact::Pressed,
            ink(a.background.active),
            Some(ink(a.border.active)),
        );
        assert_box(
            mode,
            spec,
            Interact::Focus,
            ink(a.background.base),
            Some(ink(a.border.base)),
        );
        let (rects, b) = interacted(mode, spec, Interact::Focus);
        let inset = meridian_design::focus::RING_INSET;
        assert!(
            has_ring(&rects, b, ink(sem.text.on_solid), inset),
            "{mode:?}: no on-solid ring {inset} in from {b:?}: {rects:?}"
        );
        assert!(
            !has_ring(&rects, b, ink(sem.borders.focus), 0.0),
            "{mode:?}: a primary button took the ordinary ring"
        );
    }
}

#[test]
fn a_quiet_button_is_bare_until_pointed_at() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let n = sem.role(Role::Neutral).to_owned();
        let spec = Spec::new(Weight::Quiet);
        assert_box(mode, spec, Interact::Rest, Color32::TRANSPARENT, None);
        assert_box(mode, spec, Interact::Hover, ink(n.background.hover), None);
        assert_box(
            mode,
            spec,
            Interact::Pressed,
            ink(n.background.active),
            None,
        );
        assert_box(mode, spec, Interact::Focus, Color32::TRANSPARENT, None);
        let (rects, b) = interacted(mode, spec, Interact::Focus);
        assert!(
            has_ring(&rects, b, ink(sem.borders.focus), 0.0),
            "{mode:?}: no ring"
        );
    }
}

#[test]
fn an_unavailable_button_takes_the_disabled_fill_rule_and_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let n = sem.role(Role::Neutral).to_owned();
        let a = sem.role(Role::Accent).to_owned();
        let mut spec = Spec::new(Weight::Default);
        spec.unavailable = true;
        // The pointer over it changes nothing.
        for how in [Interact::Rest, Interact::Hover] {
            assert_box(
                mode,
                spec,
                how,
                ink(n.background.disabled),
                Some(ink(n.border.disabled)),
            );
        }
        spec.weight = Weight::Primary;
        assert_box(
            mode,
            spec,
            Interact::Hover,
            ink(a.background.disabled),
            Some(ink(a.border.disabled)),
        );
        spec.weight = Weight::Quiet;
        assert_box(mode, spec, Interact::Hover, Color32::TRANSPARENT, None);
    }
}

#[test]
fn a_key_sits_four_from_the_top_bottom_and_trailing_edge() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new(Weight::Default);
        spec.key = Some(KEY);
        let (rects, b) = interacted(mode, spec, Interact::Rest);
        let caps = key_boxes(&rects);
        assert_eq!(caps.len(), 1, "{mode:?}: {rects:?}");
        assert_eq!(
            caps[0].fill,
            ink(sem.surfaces.sunken),
            "{mode:?}: the key's face"
        );
        let cap = caps[0].rect;
        assert!(
            near(cap.height(), KEYCAP_HEIGHT_IN_CONTROL),
            "{mode:?}: {cap:?}"
        );
        assert!(
            near(cap.top() - b.top(), KEYCAP_INSET_IN_CONTROL),
            "{mode:?}: top {cap:?} in {b:?}"
        );
        assert!(
            near(b.bottom() - cap.bottom(), KEYCAP_INSET_IN_CONTROL),
            "{mode:?}: bottom"
        );
        assert!(
            near(b.right() - cap.right(), KEYCAP_INSET_IN_CONTROL),
            "{mode:?}: trailing"
        );
        assert_corner(&caps, CHIP, "the keycap");
        // The foot is the heavier rule across its bottom.
        assert!(
            rects
                .iter()
                .any(|r| r.fill == ink(sem.borders.default_) && near_rect(r.rect, cap)),
            "{mode:?}: no foot"
        );

        // On the accent it is the on-solid key: no face, the on-solid rule.
        spec.weight = Weight::Primary;
        let (rects, b) = interacted(mode, spec, Interact::Rest);
        assert_eq!(
            key_boxes(&rects)[0].fill,
            Color32::TRANSPARENT,
            "{mode:?}: a key on the accent kept its face"
        );
        let on_solid = rects
            .iter()
            .find(|r| r.stroke.color == ink(sem.keycap.on_solid_rule))
            .unwrap_or_else(|| panic!("{mode:?}: no on-solid key in {rects:?}"));
        assert!(near(
            b.right() - on_solid.rect.right(),
            KEYCAP_INSET_IN_CONTROL
        ));
    }
}

#[test]
fn an_unavailable_button_reports_no_click_and_reports_its_rect_when_pointed_at() {
    let mut spec = Spec::new(Weight::Default);
    spec.unavailable = true;
    let mut harness = drawn(Mode::Light, spec);
    assert_eq!(
        harness.state().pointed,
        None,
        "reported its rect with nothing over it"
    );

    harness
        .get_by_role_and_label(AccessRole::Button, LABEL)
        .hover();
    harness.run();
    let b = harness.state().rect;
    let pointed = harness
        .state()
        .pointed
        .expect("pointed at, it reports its rect");
    assert!(
        near_rect(pointed, b),
        "{pointed:?} is not the button's {b:?}"
    );

    harness
        .get_by_role_and_label(AccessRole::Button, LABEL)
        .click();
    harness.run();
    assert!(
        !harness.state().clicked,
        "an unavailable button reported a click"
    );

    // An available one does click, so the above is not a dead harness.
    let mut live = drawn(Mode::Light, Spec::new(Weight::Default));
    live.get_by_role_and_label(AccessRole::Button, LABEL)
        .click();
    live.run();
    assert!(live.state().clicked);
    assert_eq!(live.state().pointed, None);
}

#[test]
fn an_unavailable_button_is_disabled_with_its_reason_as_its_description() {
    let mut spec = Spec::new(Weight::Default);
    spec.unavailable = true;
    let harness = drawn(Mode::Light, spec);
    let node = harness.get_by_role_and_label(AccessRole::Button, LABEL);
    assert!(node.accesskit_node().is_disabled());
    assert_eq!(
        node.accesskit_node().description().as_deref(),
        Some("Nothing to run")
    );

    let live = drawn(Mode::Light, Spec::new(Weight::Default));
    let node = live.get_by_role_and_label(AccessRole::Button, LABEL);
    assert!(!node.accesskit_node().is_disabled());
    assert_eq!(node.accesskit_node().description(), None);
}

#[test]
fn an_unavailable_button_takes_no_keyboard_focus() {
    // The harness can focus an available button, so a button that stays
    // unfocused below is the button's doing.
    let mut live = drawn(Mode::Light, Spec::new(Weight::Default));
    live.get_by_role_and_label(AccessRole::Button, LABEL)
        .focus();
    live.run();
    assert!(live.state().focused, "the harness cannot focus a button");

    let mut spec = Spec::new(Weight::Default);
    spec.unavailable = true;
    let mut harness = drawn(Mode::Light, spec);
    harness
        .get_by_role_and_label(AccessRole::Button, LABEL)
        .focus();
    harness.run();
    assert!(
        !harness.state().focused,
        "an accessibility focus request focused it"
    );
    harness.key_press(egui::Key::Tab);
    harness.run();
    assert!(
        !harness.state().focused,
        "Tab focused an unavailable button"
    );
}

#[test]
fn a_dense_button_is_twenty_tall_and_draws_no_key() {
    for mode in MODES {
        let mut spec = Spec::new(Weight::Default);
        spec.dense = true;
        let (rects, b) = interacted(mode, spec, Interact::Rest);
        assert!(near(b.height(), 20.0), "{mode:?}: {b:?}");
        assert!(key_boxes(&rects).is_empty(), "{mode:?}: a key");
        let full = {
            let (_, full) = interacted(mode, Spec::new(Weight::Default), Interact::Rest);
            full
        };
        assert!(near(full.height(), 24.0));
        assert!(b.width() < full.width(), "dense pads SPACE_3, not SPACE_4");
    }
}

/// Passing a keystroke to a dense button is a mistake the debug build names.
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "dense")]
fn a_dense_button_given_a_keystroke_is_a_debug_assertion() {
    let mut spec = Spec::new(Weight::Default);
    spec.dense = true;
    spec.key = Some(KEY);
    let _ = drawn(Mode::Light, spec);
}

/// Every row text the frame painted, with the bounds it was painted in.
fn texts(harness: &Harness<'_, Out>) -> Vec<(String, Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, Rect)>) {
        match shape {
            egui::Shape::Text(t) => {
                let s: String = t.galley.rows.iter().map(|r| r.text()).collect();
                out.push((s, t.galley.rect.translate(t.pos.to_vec2())));
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
fn a_long_label_is_cut_with_an_ellipsis_and_the_key_keeps_its_place() {
    for mode in MODES {
        let mut spec = Spec::new(Weight::Default);
        spec.label = "Export the whole of the filtered table to a file";
        spec.key = Some(KEY);
        spec.icon = true;
        spec.width = 120.0;
        let harness = drawn(mode, spec);
        let b = harness.state().rect;
        assert!(
            b.width() <= 120.0 + 0.01,
            "{mode:?}: {b:?} is wider than its ui"
        );

        let rects = painted(&harness, mode);
        let cap = key_boxes(&rects)[0].rect;
        assert!(
            near(b.right() - cap.right(), KEYCAP_INSET_IN_CONTROL),
            "{mode:?}: key moved"
        );

        let label = texts(&harness)
            .into_iter()
            .find(|(s, _)| s.contains('\u{2026}'))
            .unwrap_or_else(|| panic!("{mode:?}: no ellipsis in {:?}", texts(&harness)));
        assert!(
            label.0.len() < spec.label.len(),
            "{mode:?}: not cut: {}",
            label.0
        );
        assert!(
            label.1.right() <= cap.left(),
            "{mode:?}: the label {:?} runs under the key {cap:?}",
            label.1
        );
        assert!(label.1.left() >= b.left());
    }
}

#[test]
fn an_icon_only_button_is_square_and_named_by_its_label() {
    for mode in MODES {
        let mut spec = Spec::new(Weight::Default);
        spec.icon_only = true;
        spec.label = "Add a step";
        let harness = drawn(mode, spec);
        let b = harness.state().rect;
        assert!(
            near(b.width(), 24.0) && near(b.height(), 24.0),
            "{mode:?}: {b:?}"
        );
        harness.get_by_role_and_label(AccessRole::Button, "Add a step");
        assert!(
            texts(&harness).iter().all(|(s, _)| s != "Add a step"),
            "{mode:?}: the name was drawn"
        );
        spec.dense = true;
        let dense = drawn(mode, spec).state().rect;
        assert!(
            near(dense.width(), 20.0) && near(dense.height(), 20.0),
            "{mode:?}: {dense:?}"
        );
    }
}

#[test]
fn an_unavailable_default_button_inks_its_label_in_the_disabled_text() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let ink_of = |unavailable: bool| {
            let mut spec = Spec::new(Weight::Default);
            spec.unavailable = unavailable;
            let harness = drawn(mode, spec);
            let mut found = None;
            for c in &harness.output().shapes {
                if let egui::Shape::Text(t) = &c.shape {
                    if t.galley.job.text == LABEL {
                        found = Some(t.galley.job.sections[0].format.color);
                    }
                }
            }
            found.unwrap_or_else(|| panic!("{mode:?}: the label was not drawn"))
        };
        assert_eq!(ink_of(true), ink(sem.text.disabled), "{mode:?}");
        assert_ne!(ink_of(false), ink(sem.text.disabled), "{mode:?}");
    }
}

/// Every colour an icon's strokes were painted in (the glyph is drawn as
/// stroked paths, which is all that is stroked and not a rect or text).
fn icon_inks(harness: &Harness<'_, Out>) -> Vec<Color32> {
    fn walk(shape: &egui::Shape, out: &mut Vec<Color32>) {
        match shape {
            egui::Shape::Path(p) => {
                if let egui::epaint::ColorMode::Solid(c) = p.stroke.color {
                    out.push(c);
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

fn text_ink(harness: &Harness<'_, Out>, text: &str) -> Color32 {
    for c in &harness.output().shapes {
        if let egui::Shape::Text(t) = &c.shape {
            if t.galley.job.text == text {
                return t.galley.job.sections[0].format.color;
            }
        }
    }
    panic!("{text:?} was not drawn");
}

#[test]
fn an_unavailable_primary_button_keeps_the_accent_ink_on_its_icon_and_dims_its_label() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let a = sem.role(Role::Accent).to_owned();
        let mut spec = Spec::new(Weight::Primary);
        spec.icon = true;
        spec.unavailable = true;
        let harness = drawn(mode, spec);
        let icons = icon_inks(&harness);
        assert!(!icons.is_empty(), "{mode:?}: no icon drawn");
        assert!(
            icons.iter().all(|c| *c == ink(a.foreground.base)),
            "{mode:?}: icon inks {icons:?}"
        );
        assert_eq!(
            text_ink(&harness, LABEL),
            ink(a.foreground.disabled),
            "{mode:?}"
        );
        assert_ne!(ink(a.foreground.base), ink(a.foreground.disabled));

        // The default weight's icon goes to the disabled text, as its label does.
        let mut spec = Spec::new(Weight::Default);
        spec.icon = true;
        spec.unavailable = true;
        let icons = icon_inks(&drawn(mode, spec));
        assert!(
            icons.iter().all(|c| *c == ink(sem.text.disabled)),
            "{mode:?}: {icons:?}"
        );
    }
}

#[test]
fn an_unavailable_button_dims_its_key_and_a_dense_one_pads_space_3() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new(Weight::Default);
        spec.key = Some(KEY);
        assert_eq!(
            text_ink(&drawn(mode, spec), KEY),
            ink(sem.text.secondary),
            "{mode:?}"
        );
        spec.unavailable = true;
        assert_eq!(
            text_ink(&drawn(mode, spec), KEY),
            ink(sem.text.disabled),
            "{mode:?}"
        );

        let lead = |dense: bool| {
            let mut spec = Spec::new(Weight::Default);
            spec.dense = dense;
            let harness = drawn(mode, spec);
            let (_, label) = texts(&harness)
                .into_iter()
                .find(|(s, _)| s == LABEL)
                .unwrap();
            label.left() - harness.state().rect.left()
        };
        // The galley's own bearing is the same in both, so the difference is
        // the padding.
        assert!(
            near(lead(false) - lead(true), SPACE_4 - SPACE_3),
            "{mode:?}"
        );
        assert!(near(lead(false), SPACE_4), "{mode:?}: {}", lead(false));
    }
}
