//! The horizontal geometry of the two chip primitives, read out of the paint
//! list.
//!
//! **Nothing held this before.** Reverting either `status_pill` or `key_chip`
//! to the raw `space[2]` it used to spend left the whole workspace suite green
//! — 14 test binaries, no failures, under each mutation separately. The
//! accessibility tree carries the label, not the capsule, and the capsule is
//! the thing whose width moved; `tests/overlay_picker.rs` reads the paint list
//! for chips but asks it only about height. So the axis this file is about was
//! unguarded in a design system whose whole argument is that its own
//! components are not off.
//!
//! Every expectation below is laid out from `meridian-design` constants here,
//! never asked of the code under test, and every measurement comes back out of
//! the shapes the frame actually painted.

use egui_kittest::Harness;
use meridian_design::control::{HEIGHT_XS, ICON_XS};
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{CHIP_PADDING_X, ICON_LABEL_GAP, SPACE_1};
use meridian_egui::{icons, key_chip, theme, widgets, Mode};

/// The hairline every box in this crate is stroked with. Named here so the
/// insets below are spelled out term by term rather than borrowed from the
/// code under test — the same term `tests/overlay_picker.rs` spends on the
/// vertical axis.
const HAIRLINE: f32 = 1.0;

/// Logical pixels are exact multiples of the ladder here, so this is float
/// noise tolerance and nothing else. Every mutation these tests are built to
/// catch moves a number by 2.0 or more.
const EPS: f32 = 0.01;

fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

/// Every rect and every text run the frame painted, flattened out of the shape
/// tree. A deliberately separate walk from the one in `overlay_picker.rs`: if
/// the two shared a reader, a bug in the reader would hide from both.
struct Painted {
    rects: Vec<(egui::Rect, egui::Color32, egui::CornerRadius)>,
    texts: Vec<egui::Rect>,
}

fn painted<S>(harness: &Harness<'_, S>) -> Painted {
    fn walk(shape: &egui::Shape, out: &mut Painted) {
        match shape {
            egui::Shape::Rect(r) => out.rects.push((r.rect, r.fill, r.corner_radius)),
            egui::Shape::Text(t) => out
                .texts
                .push(egui::Rect::from_min_size(t.pos, t.galley.size())),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Painted {
        rects: Vec::new(),
        texts: Vec::new(),
    };
    for clipped in &harness.output().shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

impl Painted {
    /// The one box painted with `fill` at the chip radius. Filtering on the
    /// pair is what addresses a chip exactly: the surface behind it carries the
    /// same rect in a one-widget frame, and is drawn square.
    fn chip_box(&self, fill: egui::Color32) -> egui::Rect {
        let radius = egui::CornerRadius::from(meridian_design::radius::CHIP);
        let found: Vec<egui::Rect> = self
            .rects
            .iter()
            .filter(|(_, f, r)| *f == fill && *r == radius)
            .map(|(rect, _, _)| *rect)
            .collect();
        assert_eq!(found.len(), 1, "exactly one chip box painted");
        found[0]
    }

    /// The one text run painted. Each harness below draws a single chip, so a
    /// second run would mean the frame is not the frame under test.
    fn only_text(&self) -> egui::Rect {
        assert_eq!(self.texts.len(), 1, "exactly one text run painted");
        self.texts[0]
    }
}

/// Draw one status pill on its own and measure the capsule and the label.
fn drawn_pill(mode: Mode, label: &'static str, role: Role) -> (egui::Rect, egui::Rect) {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 200.0))
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), mode);
            widgets::status_pill(ui, &icons::CLOCK, label, role);
        });
    harness.run();
    let p = painted(&harness);
    let fill = theme::to_color32(semantic(mode.is_dark()).role(role).background.base);
    (p.chip_box(fill), p.only_text())
}

/// Draw one keycap chip on its own and measure the keycap and the keystroke.
fn drawn_key_chip(mode: Mode, keystroke: &'static str) -> (egui::Rect, egui::Rect) {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 200.0))
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), mode);
            key_chip(ui, keystroke);
        });
    harness.run();
    let p = painted(&harness);
    let fill = theme::to_color32(semantic(mode.is_dark()).surfaces.sunken);
    (p.chip_box(fill), p.only_text())
}

/// The labels the gallery's pills carry, two of them with descenders — the
/// defect on the *vertical* axis is descender-dependent, and a horizontal claim
/// that only held for `ok` would be worth nothing.
const PILL_LABELS: [(&str, Role); 4] = [
    ("ready", Role::Info),
    ("ok", Role::Success),
    ("waiting", Role::Neutral),
    ("failing", Role::Danger),
];

/// The pill is inset from its capsule by the named chip padding, on both edges.
///
/// This is the defect the token was introduced for, stated as the picture
/// shows it: the outer inset used to be 4.0 against a 6.0 gap between the icon
/// and the label, so the group sat looser in its middle than it sat inside the
/// capsule. The trailing inset is measured directly; the leading one is what is
/// left of the run from the capsule's edge to the label once the icon and the
/// gap are taken out of it.
#[test]
fn the_pill_spends_the_named_chip_padding_on_both_outer_edges() {
    for mode in [Mode::Light, Mode::Dark] {
        for (label, role) in PILL_LABELS {
            let (capsule, text) = drawn_pill(mode, label, role);

            let trailing = capsule.right() - text.right();
            let leading = (text.left() - capsule.left()) - ICON_XS - ICON_LABEL_GAP;

            assert!(
                near(trailing, CHIP_PADDING_X),
                "{mode:?} {label}: trailing inset {trailing} is not CHIP_PADDING_X \
                 ({CHIP_PADDING_X})"
            );
            assert!(
                near(leading, CHIP_PADDING_X),
                "{mode:?} {label}: leading inset {leading} is not CHIP_PADDING_X \
                 ({CHIP_PADDING_X})"
            );
            assert!(
                trailing + EPS >= ICON_LABEL_GAP,
                "{mode:?} {label}: the rhythm is inverted — the capsule insets the \
                 group by {trailing} while the group is {ICON_LABEL_GAP} loose in \
                 its middle"
            );
        }
    }
}

/// The capsule is exactly the width its four terms add up to, and nothing else
/// has crept into the row.
///
/// Sizes in points, decoded off the sign-off montage at 2.0 px/pt: `ready`
/// 62.5, `ok` 43.5, `waiting` 71.0, `failing` 64.0 — each 4.0 wider than it was
/// before this token existed. Those figures are not asserted here because they
/// are font metrics; the equation that produced them is.
#[test]
fn the_capsule_is_its_terms_and_no_more() {
    for (label, role) in PILL_LABELS {
        let (capsule, text) = drawn_pill(Mode::Light, label, role);
        let expected = CHIP_PADDING_X + ICON_XS + ICON_LABEL_GAP + text.width() + CHIP_PADDING_X;
        assert!(
            near(capsule.width(), expected),
            "{label}: the capsule drew {} wide against the {expected} its terms \
             add up to",
            capsule.width()
        );
    }
}

/// The keycap spends the same named padding, which is the half of the token
/// that makes it shared rather than a pill constant with a general name.
///
/// The hairline is in the expectation because the chip's box is a stroked
/// `egui::Frame` and the painted rect carries the stroke outside the margin —
/// the same term `overlay_picker.rs` spends when it derives the chip's height.
#[test]
fn the_keycap_spends_the_same_named_chip_padding() {
    for mode in [Mode::Light, Mode::Dark] {
        for keystroke in ["Esc", "Ctrl+K"] {
            let (chip, text) = drawn_key_chip(mode, keystroke);

            let leading = text.left() - chip.left();
            let trailing = chip.right() - text.right();
            let expected = CHIP_PADDING_X + HAIRLINE;

            assert!(
                near(leading, expected),
                "{mode:?} {keystroke}: leading inset {leading} is not \
                 CHIP_PADDING_X + hairline ({expected})"
            );
            assert!(
                near(trailing, expected),
                "{mode:?} {keystroke}: trailing inset {trailing} is not \
                 CHIP_PADDING_X + hairline ({expected})"
            );
            assert!(
                near(chip.width(), text.width() + 2.0 * expected),
                "{mode:?} {keystroke}: the keycap drew {} wide against the {} its \
                 terms add up to",
                chip.width(),
                text.width() + 2.0 * expected
            );
        }
    }
}

/// The vertical ladder does not move, and this is the test that says so.
///
/// Naming the chip inset is a horizontal change and the height ladder is not
/// part of it. The pill's capsule is a control rung; the keycap is its content
/// plus the hairline gap the ladder's smallest step allows, top and bottom.
/// Both are stated from the constants, so a horizontal edit that reached the
/// vertical axis by accident reddens here rather than in a picture nobody
/// looks at.
#[test]
fn neither_chip_moves_on_the_vertical_ladder() {
    for (label, role) in PILL_LABELS {
        let (capsule, _) = drawn_pill(Mode::Light, label, role);
        assert!(
            near(capsule.height(), HEIGHT_XS),
            "{label}: the capsule drew {} tall against the {HEIGHT_XS} rung it \
             sits on",
            capsule.height()
        );
    }
    for keystroke in ["Esc", "Ctrl+K"] {
        let (chip, text) = drawn_key_chip(Mode::Light, keystroke);
        let expected = text.height() + 2.0 * SPACE_1 + 2.0 * HAIRLINE;
        assert!(
            near(chip.height(), expected),
            "{keystroke}: the keycap drew {} tall against the {expected} its \
             terms add up to",
            chip.height()
        );
    }
}

/// Geometry is not a theme artefact. The card's pixel decoding found the dark
/// golden identical to the light one box for box; this holds the two primitives
/// to that.
#[test]
fn the_two_modes_draw_the_same_boxes() {
    for (label, role) in PILL_LABELS {
        let (light, light_text) = drawn_pill(Mode::Light, label, role);
        let (dark, dark_text) = drawn_pill(Mode::Dark, label, role);
        assert!(
            near(light.width(), dark.width()) && near(light.height(), dark.height()),
            "{label}: the pill is {:?} in light and {:?} in dark",
            light.size(),
            dark.size()
        );
        assert!(
            near(
                light_text.left() - light.left(),
                dark_text.left() - dark.left()
            ),
            "{label}: the label sits at a different inset in the two modes"
        );
    }
    for keystroke in ["Esc", "Ctrl+K"] {
        let (light, _) = drawn_key_chip(Mode::Light, keystroke);
        let (dark, _) = drawn_key_chip(Mode::Dark, keystroke);
        assert!(
            near(light.width(), dark.width()) && near(light.height(), dark.height()),
            "{keystroke}: the keycap is {:?} in light and {:?} in dark",
            light.size(),
            dark.size()
        );
    }
}
