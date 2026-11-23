//! The text field, read out of the paint list and the accessibility tree: the
//! sunken fill and its rule in each state, where the text sits, the message
//! line beneath, and what typing and Enter report.

mod common;

use common::{filled, ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::{Color32, CornerRadius, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::radius::CONTROL;
use meridian_design::semantic::{semantic, Role};
use meridian_design::{control, scales};
use meridian_egui::{icons, text_field, theme, Mode, TextField, TextFieldResponse};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const HELP: &str = "How far back installs are counted.";
const ERROR: &str = "Enter a whole number of days.";

#[derive(Clone, Copy)]
struct Spec {
    text: &'static str,
    placeholder: Option<&'static str>,
    icon: bool,
    unit: Option<&'static str>,
    help: Option<&'static str>,
    error: Option<&'static str>,
    dense: bool,
    unavailable: bool,
    width: f32,
}

impl Spec {
    fn new() -> Self {
        Self {
            text: "ranking",
            placeholder: None,
            icon: false,
            unit: None,
            help: None,
            error: None,
            dense: false,
            unavailable: false,
            width: 200.0,
        }
    }
}

struct Out {
    text: String,
    rect: Rect,
    changed: bool,
    submitted: bool,
    focused: bool,
}

fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(spec.width + 40.0, 120.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(spec.width, 80.0), |ui| {
                    let mut f = TextField::new(&mut out.text);
                    if let Some(p) = spec.placeholder {
                        f = f.placeholder(p);
                    }
                    if spec.icon {
                        f = f.icon(&icons::ALL[0]);
                    }
                    if let Some(u) = spec.unit {
                        f = f.unit(u);
                    }
                    if let Some(h) = spec.help {
                        f = f.help(h);
                    }
                    if let Some(e) = spec.error {
                        f = f.error(e);
                    }
                    if spec.dense {
                        f = f.dense();
                    }
                    if spec.unavailable {
                        f = f.unavailable();
                    }
                    let r: TextFieldResponse = text_field(ui, f);
                    out.rect = r.response.rect;
                    out.changed |= r.changed;
                    out.submitted |= r.submitted;
                    out.focused = r.response.has_focus();
                });
            },
            Out {
                text: spec.text.to_owned(),
                rect: Rect::NOTHING,
                changed: false,
                submitted: false,
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
    Focus,
}

fn interacted(mode: Mode, spec: Spec, how: Interact) -> Harness<'static, Out> {
    let mut harness = drawn(mode, spec);
    match how {
        Interact::Rest => {}
        Interact::Hover => {
            harness.get_by_role(AccessRole::TextInput).hover();
            harness.run();
        }
        Interact::Focus => {
            harness.get_by_role(AccessRole::TextInput).focus();
            harness.run();
        }
    }
    harness
}

/// The field's own box, from what it reported: its fill, with the rule at the
/// foot.
fn field_fill(rects: &[Painted], fill: Color32, b: Rect, what: &str) -> Painted {
    let found: Vec<Painted> = filled(rects, fill)
        .into_iter()
        .filter(|r| near_rect(r.rect, b))
        .collect();
    assert_eq!(
        found.len(),
        1,
        "{what}: fills on the field's box: {found:?}"
    );
    found[0]
}

/// The rects of `ink` that are a rule along the field's foot: the width of the
/// box, `thickness` tall, flush with the bottom. Returns how many there are.
fn rules(rects: &[Painted], ink: Color32, b: Rect, thickness: f32) -> usize {
    filled(rects, ink)
        .iter()
        .filter(|r| {
            near(r.rect.left(), b.left())
                && near(r.rect.right(), b.right())
                && near(r.rect.bottom(), b.bottom())
                && near(r.rect.height(), thickness)
        })
        .count()
}

fn assert_look(mode: Mode, spec: Spec, how: Interact, rule: Color32, thickness: f32) {
    let sem = semantic(mode.is_dark());
    let h = interacted(mode, spec, how);
    let b = h.state().rect;
    let what = format!("{mode:?} {how:?} unavailable={}", spec.unavailable);
    let rects = painted(&h, mode);
    let fill = field_fill(&rects, ink(sem.surfaces.sunken), b, &what);
    let c = CONTROL.round() as u8;
    assert_eq!(
        fill.radius,
        CornerRadius {
            nw: c,
            ne: c,
            sw: 0,
            se: 0
        },
        "{what}: top corners at the control radius, bottom square"
    );
    assert_eq!(rules(&rects, rule, b, thickness), 1, "{what}: rule");
    // The other thickness in that ink is absent: the rule is one or the other.
    let other = if thickness == 1.0 { 2.0 } else { 1.0 };
    assert_eq!(rules(&rects, rule, b, other), 0, "{what}: wrong thickness");
}

#[test]
fn a_field_at_rest_is_a_sunken_fill_over_a_one_pixel_control_rule() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        assert_look(
            mode,
            Spec::new(),
            Interact::Rest,
            ink(sem.borders.control),
            1.0,
        );
        let h = drawn(mode, Spec::new());
        assert!(near(h.state().rect.height(), control::HEIGHT_MD));
    }
}

#[test]
fn a_hovered_field_rules_in_the_primary_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        assert_look(
            mode,
            Spec::new(),
            Interact::Hover,
            ink(sem.text.primary),
            1.0,
        );
        // Control: at rest the same ink is not a rule on the field.
        let h = drawn(mode, Spec::new());
        let b = h.state().rect;
        assert_eq!(
            rules(&painted(&h, mode), ink(sem.text.primary), b, 1.0),
            0,
            "{mode:?}"
        );
    }
}

#[test]
fn a_field_with_the_keys_rules_two_pixels_in_the_focus_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = interacted(mode, Spec::new(), Interact::Focus);
        assert!(h.state().focused, "{mode:?}: the harness did not focus it");
        assert_look(
            mode,
            Spec::new(),
            Interact::Focus,
            ink(sem.borders.focus),
            2.0,
        );
    }
}

#[test]
fn a_field_with_an_error_rules_two_pixels_in_the_danger_ink_and_keeps_it_with_the_keys() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let danger = ink(sem.role(Role::Danger).border.base);
        let mut spec = Spec::new();
        spec.error = Some(ERROR);
        assert_look(mode, spec, Interact::Rest, danger, 2.0);
        // The foot stays 2px in the danger ink with the keys too; the ring
        // that joins it is read in its own test.
        assert_look(mode, spec, Interact::Focus, danger, 2.0);
        assert_ne!(danger, ink(sem.borders.focus));
    }
}

/// The stroked rects of `ink` laid on the field's own box: a ring, not a rule.
fn rings(rects: &[Painted], ink: Color32, b: Rect) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| r.is_stroked() && r.stroke.color == ink && near_rect(r.rect, b))
        .copied()
        .collect()
}

#[test]
fn an_errored_field_with_the_keys_closes_the_danger_rule_into_a_ring() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let danger = ink(sem.role(Role::Danger).border.base);
        let mut spec = Spec::new();
        spec.error = Some(ERROR);
        let h = interacted(mode, spec, Interact::Focus);
        assert!(h.state().focused, "{mode:?}: the harness did not focus it");
        let b = h.state().rect;
        let rects = painted(&h, mode);
        let ring = rings(&rects, danger, b);
        assert_eq!(ring.len(), 1, "{mode:?}: danger rings on the box: {ring:?}");
        let ring = ring[0];
        assert!(near(ring.stroke.width, 2.0), "{mode:?}: {:?}", ring.stroke);
        assert_eq!(
            ring.kind,
            egui::StrokeKind::Inside,
            "{mode:?}: inside the edge"
        );
        let c = CONTROL.round() as u8;
        assert_eq!(
            ring.radius,
            CornerRadius {
                nw: c,
                ne: c,
                sw: 0,
                se: 0
            },
            "{mode:?}: top corners at the control radius, bottom square"
        );
        // The foot is still the 2px danger rule, and the text has not moved.
        assert_eq!(rules(&rects, danger, b, 2.0), 1, "{mode:?}: foot");
        let rest = drawn(mode, spec);
        let (_, at_rest) = text_of(&rest, "ranking");
        let (_, with_keys) = text_of(&h, "ranking");
        assert!(
            near(at_rest.min.x, with_keys.min.x) && near(at_rest.min.y, with_keys.min.y),
            "{mode:?}: the text moved from {at_rest:?} to {with_keys:?}"
        );
    }
}

#[test]
fn an_errored_field_without_the_keys_draws_only_the_foot() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let danger = ink(sem.role(Role::Danger).border.base);
        let mut spec = Spec::new();
        spec.error = Some(ERROR);
        for how in [Interact::Rest, Interact::Hover] {
            let h = interacted(mode, spec, how);
            let b = h.state().rect;
            let rects = painted(&h, mode);
            assert!(rings(&rects, danger, b).is_empty(), "{mode:?} {how:?}");
            assert_eq!(rules(&rects, danger, b, 2.0), 1, "{mode:?} {how:?}: foot");
        }
    }
}

#[test]
fn a_focused_field_without_an_error_keeps_the_focus_foot_and_draws_no_ring() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = interacted(mode, Spec::new(), Interact::Focus);
        let b = h.state().rect;
        let rects = painted(&h, mode);
        for colour in [sem.borders.focus, sem.role(Role::Danger).border.base] {
            assert!(rings(&rects, ink(colour), b).is_empty(), "{mode:?}");
        }
        assert_eq!(rules(&rects, ink(sem.borders.focus), b, 2.0), 1, "{mode:?}");
    }
}

#[test]
fn an_unavailable_field_keeps_its_fill_and_rules_one_pixel_in_the_disabled_border() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.unavailable = true;
        assert_look(
            mode,
            spec,
            Interact::Rest,
            ink(sem.role(Role::Neutral).border.disabled),
            1.0,
        );
    }
}

/// Every text drawn, with its ink and where it sits.
fn texts(harness: &Harness<'_, Out>) -> Vec<(String, Color32, Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, Color32, Rect)>) {
        match shape {
            egui::Shape::Text(t) => out.push((
                t.galley.job.text.clone(),
                t.galley.job.sections[0].format.color,
                t.galley.rect.translate(t.pos.to_vec2()),
            )),
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

fn text_of(harness: &Harness<'_, Out>, s: &str) -> (Color32, Rect) {
    texts(harness)
        .into_iter()
        .find(|(t, _, _)| t == s)
        .map(|(_, c, r)| (c, r))
        .unwrap_or_else(|| panic!("{s:?} was not drawn"))
}

#[test]
fn the_rule_thickens_without_moving_the_text() {
    for mode in MODES {
        let rest = drawn(mode, Spec::new());
        let keys = interacted(mode, Spec::new(), Interact::Focus);
        let (_, at_rest) = text_of(&rest, "ranking");
        let (_, with_keys) = text_of(&keys, "ranking");
        assert!(
            near(at_rest.min.x, with_keys.min.x) && near(at_rest.min.y, with_keys.min.y),
            "{mode:?}: the text moved from {at_rest:?} to {with_keys:?}"
        );
        // Control: the rule did change between the two frames.
        let sem = semantic(mode.is_dark());
        let b = rest.state().rect;
        assert_eq!(
            rules(&painted(&rest, mode), ink(sem.borders.control), b, 1.0),
            1
        );
        assert_eq!(
            rules(&painted(&keys, mode), ink(sem.borders.focus), b, 2.0),
            1
        );
        // The text is vertically centred in the field, to within the half pixel
        // egui snaps a text edit's line to. A top-aligned line would be 4 off.
        assert!(
            (at_rest.center().y - b.center().y).abs() < 0.5,
            "{mode:?}: {at_rest:?} in {b:?}"
        );
    }
}

#[test]
fn inks_the_text_placeholder_unit_and_icon_from_the_semantic_set() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.text = "30";
        spec.unit = Some("days");
        spec.icon = true;
        let h = drawn(mode, spec);
        assert_eq!(text_of(&h, "30").0, ink(sem.text.primary), "{mode:?}");
        assert_eq!(text_of(&h, "days").0, ink(sem.text.muted), "{mode:?}");
        let (_, unit) = text_of(&h, "days");
        assert!(
            unit.right() <= h.state().rect.right(),
            "{mode:?}: unit outside"
        );

        let mut spec = Spec::new();
        spec.text = "";
        spec.placeholder = Some("Name");
        let h = drawn(mode, spec);
        assert_eq!(text_of(&h, "Name").0, ink(sem.text.placeholder), "{mode:?}");

        let mut spec = Spec::new();
        spec.unavailable = true;
        let h = drawn(mode, spec);
        assert_eq!(text_of(&h, "ranking").0, ink(sem.text.disabled), "{mode:?}");
    }
}

#[test]
fn a_leading_icon_is_drawn_in_the_muted_ink() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.icon = true;
        let h = drawn(mode, spec);
        let mut inks = Vec::new();
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
        for c in &h.output().shapes {
            walk(&c.shape, &mut inks);
        }
        assert!(!inks.is_empty(), "{mode:?}: no icon drawn");
        assert!(
            inks.iter().all(|c| *c == ink(sem.text.muted)),
            "{mode:?}: {inks:?}"
        );
    }
}

#[test]
fn a_dense_field_is_twenty_tall() {
    let mut spec = Spec::new();
    spec.dense = true;
    let h = drawn(Mode::Light, spec);
    assert!(near(h.state().rect.height(), control::HEIGHT_SM));
    assert!(near(
        drawn(Mode::Light, Spec::new()).state().rect.height(),
        control::HEIGHT_MD
    ));
}

#[test]
fn a_help_line_sits_beneath_in_the_muted_ink_at_label_size() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.help = Some(HELP);
        let h = drawn(mode, spec);
        let (c, r) = text_of(&h, HELP);
        assert_eq!(c, ink(sem.text.muted), "{mode:?}");
        assert!(
            r.top() >= h.state().rect.bottom() - 0.01,
            "{mode:?}: {r:?} not beneath"
        );
        let size = h
            .output()
            .shapes
            .iter()
            .find_map(|c| match &c.shape {
                egui::Shape::Text(t) if t.galley.job.text == HELP => {
                    Some(t.galley.job.sections[0].format.font_id.size)
                }
                _ => None,
            })
            .expect("help drawn");
        assert!(
            near(size, meridian_design::typography::LABEL_SIZE),
            "{mode:?}: {size}"
        );
        // The field's own response is the field, not the message line.
        assert!(near(h.state().rect.height(), control::HEIGHT_MD));
    }
}

#[test]
fn an_error_replaces_the_help_line() {
    for mode in MODES {
        let red = if mode.is_dark() {
            scales::RED_DARK[10]
        } else {
            scales::RED_LIGHT[10]
        };
        let mut spec = Spec::new();
        spec.help = Some(HELP);
        spec.error = Some(ERROR);
        let h = drawn(mode, spec);
        let (c, r) = text_of(&h, ERROR);
        assert_eq!(c, ink(red), "{mode:?}");
        assert!(r.top() >= h.state().rect.bottom() - 0.01);
        assert!(
            !texts(&h).iter().any(|(t, _, _)| t == HELP),
            "{mode:?}: help still drawn beside the error"
        );
        // Control: without the error, the help is drawn.
        let mut spec = Spec::new();
        spec.help = Some(HELP);
        text_of(&drawn(mode, spec), HELP);
    }
}

#[test]
fn typing_changes_the_string_and_reports_changed() {
    let mut h = drawn(Mode::Light, Spec::new());
    assert!(!h.state().changed);
    h.get_by_role(AccessRole::TextInput).focus();
    h.run();
    assert!(!h.state().changed, "focusing alone reported a change");
    h.get_by_role(AccessRole::TextInput).type_text("42");
    h.run();
    assert!(h.state().changed);
    assert!(h.state().text.contains("42"), "text: {:?}", h.state().text);
}

#[test]
fn enter_reports_submitted() {
    let mut h = drawn(Mode::Light, Spec::new());
    h.get_by_role(AccessRole::TextInput).focus();
    h.run();
    h.key_press(egui::Key::A);
    h.run();
    assert!(!h.state().submitted, "a letter submitted");
    h.key_press(egui::Key::Enter);
    h.run();
    assert!(h.state().submitted);
}

#[test]
fn an_unavailable_field_takes_no_focus_and_no_typing() {
    let mut spec = Spec::new();
    spec.unavailable = true;
    let mut h = drawn(Mode::Light, spec);
    h.key_press(egui::Key::Tab);
    h.run();
    assert!(!h.state().focused);
    h.event(egui::Event::Text("zz".into()));
    h.run();
    assert_eq!(h.state().text, "ranking");
    assert!(!h.state().changed);
    // Control: an available field does take the same keys.
    let mut live = drawn(Mode::Light, Spec::new());
    live.key_press(egui::Key::Tab);
    live.run();
    assert!(live.state().focused, "Tab did not reach an available field");
}

#[test]
fn the_field_fills_the_width_it_is_given() {
    for w in [180.0, 320.0] {
        let mut spec = Spec::new();
        spec.width = w;
        spec.help = Some(HELP);
        let h = drawn(Mode::Light, spec);
        assert!(near(h.state().rect.width(), w), "{w}: {:?}", h.state().rect);
        let sem = semantic(false);
        let fill = field_fill(
            &painted(&h, Mode::Light),
            ink(sem.surfaces.sunken),
            h.state().rect,
            "width",
        );
        assert!(
            near(fill.rect.width(), w),
            "{w}: painted fill {:?}",
            fill.rect
        );
    }
}

#[test]
fn the_field_is_a_text_input_and_the_error_is_its_description() {
    let mut spec = Spec::new();
    spec.error = Some(ERROR);
    let h = drawn(Mode::Light, spec);
    let node = h.get_by_role(AccessRole::TextInput);
    assert_eq!(node.accesskit_node().description().as_deref(), Some(ERROR));

    let plain = drawn(Mode::Light, Spec::new());
    let node = plain.get_by_role(AccessRole::TextInput);
    assert_eq!(node.accesskit_node().description(), None);

    let mut spec = Spec::new();
    spec.unavailable = true;
    let off = drawn(Mode::Light, spec);
    assert!(off
        .get_by_role(AccessRole::TextInput)
        .accesskit_node()
        .is_disabled());
}

fn click_at(h: &mut Harness<'_, Out>, pos: egui::Pos2) {
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
fn a_press_anywhere_in_the_box_gives_the_field_the_keys() {
    let mut spec = Spec::new();
    spec.icon = true;
    spec.unit = Some("days");
    spec.help = Some(HELP);
    for (what, dx) in [("left padding", 3.0), ("icon", 8.0 + 7.0), ("unit", 195.0)] {
        let mut h = drawn(Mode::Light, spec);
        let r = h.state().rect;
        click_at(&mut h, egui::pos2(r.left() + dx, r.center().y));
        assert!(
            h.state().focused,
            "{what}: the press did not focus the field"
        );
    }
    // The strip above the text line, inside the box.
    let mut h = drawn(Mode::Light, spec);
    let r = h.state().rect;
    click_at(&mut h, egui::pos2(r.center().x, r.top() + 1.0));
    assert!(h.state().focused, "top strip");
    // Control: a press on the message line beneath is not in the box.
    let mut h = drawn(Mode::Light, spec);
    let r = h.state().rect;
    click_at(&mut h, egui::pos2(r.center().x, r.bottom() + 8.0));
    assert!(!h.state().focused, "the message line took the keys");
    // An unavailable field stays unfocused.
    let mut off = spec;
    off.unavailable = true;
    let mut h = drawn(Mode::Light, off);
    let r = h.state().rect;
    click_at(&mut h, egui::pos2(r.left() + 3.0, r.center().y));
    assert!(!h.state().focused);
}

/// How many vertices of a text's selection wash are in `want`: egui paints the
/// selection into the galley's own mesh.
fn washes(h: &Harness<'_, Out>, want: Color32) -> usize {
    fn walk(s: &egui::Shape, want: Color32, n: &mut usize) {
        match s {
            egui::Shape::Text(t) => {
                for r in &t.galley.rows {
                    *n += r
                        .visuals
                        .mesh
                        .vertices
                        .iter()
                        .filter(|v| v.color == want)
                        .count();
                }
            }
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, want, n)),
            _ => {}
        }
    }
    let mut n = 0;
    for c in &h.output().shapes {
        walk(&c.shape, want, &mut n);
    }
    n
}

#[test]
fn the_caret_and_the_selection_wear_the_editor_inks() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut h = drawn(mode, Spec::new());
        h.get_by_role(AccessRole::TextInput).focus();
        h.run();
        // A blinking caret is off for half its period: look across frames.
        let mut carets = 0;
        for _ in 0..40 {
            h.step();
            fn walk(s: &egui::Shape, want: Color32, n: &mut usize) {
                match s {
                    egui::Shape::LineSegment { stroke, .. } if stroke.color == want => *n += 1,
                    egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, want, n)),
                    _ => {}
                }
            }
            for c in &h.output().shapes {
                walk(&c.shape, ink(sem.editor.caret), &mut carets);
            }
        }
        assert!(carets > 0, "{mode:?}: no caret in the editor ink");

        h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
        h.run();
        assert!(
            washes(&h, ink(sem.editor.selection)) > 0,
            "{mode:?}: no selection wash"
        );
        // Control: with nothing selected there is no such wash.
        let rest = drawn(mode, Spec::new());
        assert_eq!(washes(&rest, ink(sem.editor.selection)), 0);
        assert_ne!(
            ink(sem.editor.selection),
            ink(semantic(mode.is_dark()).borders.focus)
        );
    }
}

#[test]
fn the_placeholder_goes_in_the_frame_the_first_letter_arrives() {
    let mut spec = Spec::new();
    spec.text = "";
    spec.placeholder = Some("Name");
    let mut h = drawn(Mode::Light, spec);
    text_of(&h, "Name");
    h.get_by_role(AccessRole::TextInput).focus();
    h.run();
    text_of(&h, "Name");
    h.get_by_role(AccessRole::TextInput).type_text("x");
    h.step();
    assert_eq!(h.state().text, "x");
    assert!(
        !texts(&h).iter().any(|(t, _, _)| t == "Name"),
        "the placeholder lingered a frame over the typed letter"
    );
}

#[test]
fn a_long_placeholder_is_cut_with_an_ellipsis_on_one_line() {
    let mut spec = Spec::new();
    spec.text = "";
    spec.placeholder = Some("Search every column of every table in the workspace");
    spec.width = 120.0;
    let h = drawn(Mode::Light, spec);
    let b = h.state().rect;
    let mut shown = None;
    for c in &h.output().shapes {
        if let egui::Shape::Text(t) = &c.shape {
            if t.galley.job.text.starts_with("Search") {
                let rows: String = t.galley.rows.iter().map(|r| r.text()).collect();
                shown = Some((
                    t.galley.rows.len(),
                    rows,
                    t.galley.rect.translate(t.pos.to_vec2()),
                ));
            }
        }
    }
    let (rows, text, rect) = shown.expect("placeholder drawn");
    assert_eq!(rows, 1, "wrapped onto {rows} lines");
    assert!(text.ends_with('\u{2026}'), "not cut: {text:?}");
    assert!(rect.bottom() < b.bottom() - 2.0 && rect.right() <= b.right());
}

#[test]
fn a_value_and_its_unit_share_a_baseline() {
    let mut spec = Spec::new();
    spec.text = "30";
    spec.unit = Some("days");
    let h = drawn(Mode::Light, spec);
    let (_, v) = text_of(&h, "30");
    let (_, u) = text_of(&h, "days");
    // egui snaps the edit's line to a pixel, so allow the tenth that costs.
    assert!(
        (v.bottom() - u.bottom()).abs() < 0.25 && (v.top() - u.top()).abs() < 0.25,
        "{v:?} vs {u:?}"
    );
    assert!(v.height() > 10.0);
}
