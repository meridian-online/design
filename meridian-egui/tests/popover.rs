//! The popover, read out of the paint list and the accessibility tree: where
//! it opens, where it flips, that it never leaves the screen, what dismisses
//! it, and the look of the card and of the reason form.

mod common;

use common::{assert_corner, filled, ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::{pos2, vec2, Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::elevation::Elevation;
use meridian_design::radius;
use meridian_design::semantic::semantic;
use meridian_design::spacing::{POPOVER_WIDTH, SPACE_2, SPACE_4, SPACE_5};
use meridian_egui::{theme, Anchor, Mode, Popover};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const BODY: egui::Vec2 = egui::vec2(100.0, 60.0);

#[derive(Clone, Copy)]
enum Form {
    General,
    Menu,
}

struct State {
    anchor: Rect,
    side: Anchor,
    form: Form,
    dismissed: bool,
    clicked: bool,
    block: Rect,
}

fn state(anchor: Rect, side: Anchor) -> State {
    State {
        anchor,
        side,
        form: Form::General,
        dismissed: false,
        clicked: false,
        block: Rect::NOTHING,
    }
}

fn harness(mode: Mode, screen: egui::Vec2, s: State) -> Harness<'static, State> {
    let mut h = Harness::builder().with_size(screen).build_ui_state(
        move |ui, s: &mut State| {
            theme::apply(ui.ctx(), mode);
            let body = |ui: &mut egui::Ui| {
                let w = match s.form {
                    Form::General => BODY.x,
                    Form::Menu => ui.available_width(),
                };
                let (rect, r) = ui.allocate_exact_size(vec2(w, BODY.y), egui::Sense::click());
                s.block = rect;
                if r.clicked() {
                    s.clicked = true;
                }
            };
            let r = match s.form {
                Form::General => Popover::show(ui.ctx(), "p", s.anchor, s.side, body),
                Form::Menu => Popover::show_menu(ui.ctx(), "p", s.anchor, s.side, body),
            };
            s.dismissed |= r.dismissed;
        },
        s,
    );
    h.run();
    h
}

/// The one card the frame painted: the overlay-filled, stroked rect.
fn card(h: &Harness<'_, State>, mode: Mode) -> Painted {
    let sem = semantic(mode.is_dark());
    let found: Vec<Painted> = filled(&painted(h, mode), ink(sem.surfaces.overlay))
        .into_iter()
        .filter(|r| r.is_stroked() && r.rect.height() >= 30.0)
        .collect();
    assert_eq!(found.len(), 1, "{mode:?}: exactly one card; {found:?}");
    found[0]
}

fn rect_at(x: f32, y: f32) -> Rect {
    Rect::from_min_size(pos2(x, y), vec2(40.0, 20.0))
}

#[test]
fn it_opens_below_its_anchor_and_above_when_there_is_no_room() {
    let screen = vec2(400.0, 300.0);
    let roomy = rect_at(50.0, 50.0);
    let h = harness(Mode::Light, screen, state(roomy, Anchor::Below));
    let c = card(&h, Mode::Light).rect;
    assert!(near(c.min.y, roomy.max.y + SPACE_2), "{c:?}");
    assert!(near(c.min.x, roomy.min.x), "leading edges align: {c:?}");

    let low = rect_at(50.0, 260.0);
    let h = harness(Mode::Light, screen, state(low, Anchor::Below));
    let c = card(&h, Mode::Light).rect;
    assert!(near(c.max.y, low.min.y - SPACE_2), "above: {c:?}");
    assert!(near(c.min.x, low.min.x), "{c:?}");
}

#[test]
fn beside_opens_to_the_trailing_side_and_flips_when_there_is_no_room() {
    let screen = vec2(400.0, 300.0);
    let roomy = rect_at(50.0, 100.0);
    let h = harness(Mode::Light, screen, state(roomy, Anchor::Beside));
    let c = card(&h, Mode::Light).rect;
    assert!(near(c.min.x, roomy.max.x + SPACE_2), "{c:?}");
    assert!(near(c.min.y, roomy.min.y), "tops align: {c:?}");

    let right = rect_at(340.0, 100.0);
    let h = harness(Mode::Light, screen, state(right, Anchor::Beside));
    let c = card(&h, Mode::Light).rect;
    assert!(near(c.max.x, right.min.x - SPACE_2), "leading side: {c:?}");
    assert!(near(c.min.y, right.min.y), "{c:?}");
}

#[test]
fn an_anchor_at_the_screen_s_corner_keeps_the_card_wholly_on_screen() {
    // A screen the card barely fits, so neither side has room and the flip
    // alone would put it off the edge.
    let screen = vec2(130.0, 100.0);
    let bounds = Rect::from_min_size(Pos2::ZERO, screen);
    let corners = [
        rect_at(0.0, 0.0),
        rect_at(screen.x - 40.0, 0.0),
        rect_at(0.0, screen.y - 20.0),
        rect_at(screen.x - 40.0, screen.y - 20.0),
    ];
    for side in [Anchor::Below, Anchor::Beside] {
        for anchor in corners {
            let h = harness(Mode::Light, screen, state(anchor, side));
            let c = card(&h, Mode::Light);
            assert!(
                bounds.contains_rect(c.outer()),
                "{side:?} at {anchor:?}: the card {:?} leaves the screen",
                c.outer()
            );
        }
    }
}

fn click_at(h: &mut Harness<'_, State>, pos: Pos2) {
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

#[test]
fn esc_and_a_click_outside_report_dismissed() {
    let screen = vec2(400.0, 300.0);
    let anchor = rect_at(50.0, 50.0);

    // The control: an open popover nobody touched is not dismissed.
    let h = harness(Mode::Light, screen, state(anchor, Anchor::Below));
    assert!(!h.state().dismissed);

    let mut h = harness(Mode::Light, screen, state(anchor, Anchor::Below));
    h.key_press(egui::Key::Escape);
    h.run();
    assert!(h.state().dismissed, "Esc");

    let mut h = harness(Mode::Light, screen, state(anchor, Anchor::Below));
    click_at(&mut h, pos2(350.0, 250.0));
    assert!(h.state().dismissed, "a click outside");
}

#[test]
fn a_click_inside_does_not() {
    let screen = vec2(400.0, 300.0);
    let anchor = rect_at(50.0, 50.0);
    let mut h = harness(Mode::Light, screen, state(anchor, Anchor::Below));
    let c = card(&h, Mode::Light).rect;
    click_at(&mut h, c.center() + vec2(0.0, -10.0));
    assert!(!h.state().dismissed, "a click inside the card");

    // A click on the anchor is the caller's toggle, not a dismissal.
    click_at(&mut h, anchor.center());
    assert!(!h.state().dismissed, "a click on the anchor");

    // The control: the same harness does report a click outside.
    click_at(&mut h, pos2(350.0, 250.0));
    assert!(h.state().dismissed, "control: outside");
}

#[test]
fn the_card_is_ruled_with_a_hard_shadow_three_by_three_and_a_three_corner() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = harness(
            mode,
            vec2(400.0, 300.0),
            state(rect_at(50.0, 50.0), Anchor::Below),
        );
        let c = card(&h, mode);
        assert_eq!(c.stroke.color, ink(sem.borders.default_), "{mode:?}");
        assert!(near(c.stroke.width, 1.0), "{mode:?}");
        assert_corner(&[c], radius::PANEL, "the popover card");

        let token = Elevation::Overlay.shadow(mode.is_dark()).expect("casts");
        assert_eq!((token.x, token.y, token.blur), (3.0, 3.0, 0.0));
        let want = c.rect.translate(vec2(3.0, 3.0));
        let rects = painted(&h, mode);
        let shadows: Vec<&Painted> = rects
            .iter()
            .filter(|r| near_rect(r.rect, want) && r.fill == ink(token.colour))
            .collect();
        assert_eq!(shadows.len(), 1, "{mode:?}: shadow at {want:?}; {rects:?}");
        assert_eq!(shadows[0].blur, 0.0, "{mode:?}");
        assert_corner(&[*shadows[0]], radius::PANEL, "the popover shadow");
    }
}

#[test]
fn a_menu_is_two_hundred_and_forty_wide_with_space_two_above_and_below() {
    let mut s = state(rect_at(50.0, 50.0), Anchor::Below);
    s.form = Form::Menu;
    let h = harness(Mode::Light, vec2(400.0, 300.0), s);
    let c = card(&h, Mode::Light).rect;
    assert!(near(c.width(), POPOVER_WIDTH), "{c:?}");
    let block = h.state().block;
    assert!(
        near(block.min.y - c.min.y, 1.0 + SPACE_2),
        "top padding; {block:?} in {c:?}"
    );
    assert!(
        near(block.width(), POPOVER_WIDTH - 2.0),
        "no side padding: {block:?}"
    );
    assert!(near(block.min.x - c.min.x, 1.0), "{block:?} in {c:?}");
    let bottom = c.max.y - block.max.y;
    assert!(near(bottom, SPACE_2 + 1.0), "bottom padding {bottom}");
}

struct Reason {
    anchor: Rect,
    text: &'static str,
    key: Option<&'static str>,
}

fn reason_harness(mode: Mode, r: Reason) -> Harness<'static, Reason> {
    let mut h = Harness::builder()
        .with_size(vec2(500.0, 400.0))
        .build_ui_state(
            move |ui, r: &mut Reason| {
                theme::apply(ui.ctx(), mode);
                Popover::show_reason(ui.ctx(), "r", r.anchor, r.text, r.key);
            },
            r,
        );
    h.run();
    h
}

fn reason_card(h: &Harness<'_, Reason>, mode: Mode) -> Rect {
    let sem = semantic(mode.is_dark());
    let found: Vec<Painted> = filled(&painted(h, mode), ink(sem.surfaces.overlay))
        .into_iter()
        .filter(|r| r.is_stroked() && r.rect.height() >= 30.0)
        .collect();
    assert_eq!(found.len(), 1, "{found:?}");
    found[0].rect
}

/// The sentence's own label node, not the tooltip node that carries it too.
fn sentence(h: &Harness<'_, Reason>, text: &str) -> Rect {
    let nodes: Vec<_> = h
        .query_all_by_label(text)
        .filter(|n| n.accesskit_node().role() != AccessRole::Tooltip)
        .collect();
    assert_eq!(nodes.len(), 1, "one label node carries the sentence");
    nodes[0].rect()
}

const SHORT: &str = "Needs a table.";
const LONG: &str = "This command needs a table with at least one numeric column selected, and \
                    none of the open tables has one, so there is nothing for it to run on.";

#[test]
fn a_reason_is_two_hundred_and_forty_wide_and_wraps() {
    let anchor = rect_at(100.0, 50.0);
    let short = reason_harness(
        Mode::Light,
        Reason {
            anchor,
            text: SHORT,
            key: None,
        },
    );
    let long = reason_harness(
        Mode::Light,
        Reason {
            anchor,
            text: LONG,
            key: None,
        },
    );
    let (s, l) = (
        reason_card(&short, Mode::Light),
        reason_card(&long, Mode::Light),
    );
    assert!(near(s.width(), POPOVER_WIDTH), "short: {s:?}");
    assert!(near(l.width(), POPOVER_WIDTH), "long: {l:?}");
    assert!(
        l.height() > s.height() + 20.0,
        "the sentence wraps: {s:?} vs {l:?}"
    );
    // Padding SPACE_4 by SPACE_5: the text starts inside the rule and padding.
    let text = sentence(&long, LONG);
    assert!(
        near(text.min.x - l.min.x, 1.0 + SPACE_5),
        "{text:?} in {l:?}"
    );
    assert!(
        near(text.min.y - l.min.y, 1.0 + SPACE_4),
        "{text:?} in {l:?}"
    );
    assert!(
        text.max.x <= l.max.x - 1.0 - SPACE_5 + 0.5,
        "{text:?} in {l:?}"
    );
}

#[test]
fn a_reason_puts_its_key_at_the_top_right() {
    let anchor = rect_at(100.0, 50.0);
    let h = reason_harness(
        Mode::Light,
        Reason {
            anchor,
            text: LONG,
            key: Some("F2"),
        },
    );
    let c = reason_card(&h, Mode::Light);
    let key = h.get_by_label("F2").rect();
    assert!(near(key.max.x, c.max.x - 1.0 - SPACE_5), "{key:?} in {c:?}");
    assert!(near(key.min.y, c.min.y + 1.0 + SPACE_4), "{key:?} in {c:?}");
    assert!(
        near(key.height(), meridian_design::control::KEYCAP_HEIGHT),
        "{key:?}"
    );
    let text = sentence(&h, LONG);
    assert!(
        text.max.x < key.min.x,
        "the sentence wraps clear of the key"
    );
}

#[test]
fn a_reason_is_published_as_a_tooltip_labelled_with_it() {
    let anchor = rect_at(100.0, 50.0);
    let h = reason_harness(
        Mode::Light,
        Reason {
            anchor,
            text: SHORT,
            key: None,
        },
    );
    let tip = h.get_by_role(AccessRole::Tooltip);
    assert_eq!(tip.accesskit_node().label().as_deref(), Some(SHORT));

    // The control: a general popover is no tooltip.
    let h = harness(
        Mode::Light,
        vec2(400.0, 300.0),
        state(anchor, Anchor::Below),
    );
    assert!(h.query_by_role(AccessRole::Tooltip).is_none());
}
