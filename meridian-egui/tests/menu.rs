//! The menu item and the menu separator, read out of the paint list and the
//! accessibility tree: the list row's three states at 24, the check column,
//! the key and the chevron, and the unavailable state that still takes the
//! cursor.

mod common;

use common::{ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::{Color32, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::control::{HEIGHT_MD, ICON_SM, ROW_BAR_WIDTH};
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{ICON_LABEL_GAP, SPACE_2, SPACE_4};
use meridian_egui::{menu_item, menu_separator, theme, MenuItem, MenuItemResponse, Mode};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const LABEL: &str = "Filter to this value";
const KEY: &str = "F";
const REASON: &str = "Nothing is selected";

#[derive(Clone, Copy)]
struct Spec {
    key: Option<&'static str>,
    unavailable: bool,
    checked: Option<bool>,
    submenu: bool,
    cursor: bool,
    label: &'static str,
    width: f32,
}

impl Spec {
    fn new() -> Self {
        Self {
            key: None,
            unavailable: false,
            checked: None,
            submenu: false,
            cursor: false,
            label: LABEL,
            width: 240.0,
        }
    }
}

struct Out {
    rect: Rect,
    clicked: bool,
    pointed: Option<Rect>,
    focused: bool,
}

fn build(spec: Spec) -> MenuItem<'static> {
    let mut item = MenuItem::new(spec.label).cursor(spec.cursor);
    if let Some(k) = spec.key {
        item = item.keystroke(k);
    }
    if spec.unavailable {
        item = item.unavailable(REASON);
    }
    if let Some(on) = spec.checked {
        item = item.checked(on);
    }
    if spec.submenu {
        item = item.submenu();
    }
    item
}

fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(spec.width + 40.0, 120.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(spec.width, 60.0), |ui| {
                    let r: MenuItemResponse = menu_item(ui, build(spec));
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
}

fn interacted(mode: Mode, spec: Spec, how: Interact) -> (Vec<Painted>, Rect) {
    let mut harness = drawn(mode, spec);
    if !matches!(how, Interact::Rest) {
        harness
            .get_by_role_and_label(AccessRole::MenuItem, spec.label)
            .hover();
        harness.run();
    }
    if matches!(how, Interact::Pressed) {
        let pos = harness.state().rect.center();
        harness.event(egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        });
        harness.run();
    }
    let rect = harness.state().rect;
    (painted(&harness, mode), rect)
}

/// What the frame painted over the row's whole box: the fills of the state.
fn on_row(rects: &[Painted], row: Rect) -> Vec<Painted> {
    rects
        .iter()
        .filter(|r| r.fill != Color32::TRANSPARENT && near_rect(r.rect, row))
        .copied()
        .collect()
}

/// The 3px rects on the row's leading edge.
fn bars(rects: &[Painted], row: Rect) -> Vec<Painted> {
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

/// The strokes an icon was painted with: each path's colour and the box its
/// points lie in.
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
fn a_menu_item_is_a_list_row_at_24_and_fills_its_width() {
    for mode in MODES {
        let harness = drawn(mode, Spec::new());
        let r = harness.state().rect;
        assert!(near(r.height(), HEIGHT_MD), "{mode:?}: {r:?}");
        assert!(near(r.width(), 240.0), "{mode:?}: {r:?}");
    }
}

#[test]
fn a_menu_item_at_rest_draws_no_fill_and_no_bar() {
    for mode in MODES {
        let (rects, row) = interacted(mode, Spec::new(), Interact::Rest);
        assert!(on_row(&rects, row).is_empty(), "{mode:?}: {rects:?}");
        assert!(bars(&rects, row).is_empty(), "{mode:?}");
    }
}

#[test]
fn a_menu_item_under_the_pointer_takes_the_hover_fill() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, row) = interacted(mode, Spec::new(), Interact::Hover);
        let fills = on_row(&rects, row);
        assert_eq!(fills.len(), 1, "{mode:?}: {fills:?}");
        assert_eq!(fills[0].fill, ink(sem.rows.hover_background), "{mode:?}");
        assert!(
            bars(&rects, row).is_empty(),
            "{mode:?}: the pointer has no bar"
        );
        assert_ne!(sem.rows.hover_background, sem.rows.cursor_background);
    }
}

#[test]
fn a_menu_item_held_down_takes_the_neutral_active_fill() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, row) = interacted(mode, Spec::new(), Interact::Pressed);
        let fills = on_row(&rects, row);
        assert_eq!(fills.len(), 1, "{mode:?}: {fills:?}");
        assert_eq!(
            fills[0].fill,
            ink(sem.role(Role::Neutral).background.active),
            "{mode:?}"
        );
    }
}

#[test]
fn the_cursor_takes_the_cursor_fill_and_a_three_pixel_bar() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.cursor = true;
        let (rects, row) = interacted(mode, spec, Interact::Rest);
        let fills = on_row(&rects, row);
        assert_eq!(fills.len(), 1, "{mode:?}: {fills:?}");
        assert_eq!(fills[0].fill, ink(sem.rows.cursor_background), "{mode:?}");
        let bar = bars(&rects, row);
        assert_eq!(bar.len(), 1, "{mode:?}: {bar:?}");
        assert_eq!(bar[0].fill, ink(sem.rows.cursor_bar), "{mode:?}");
    }
}

#[test]
fn an_unavailable_item_is_greyed_and_takes_no_hover_or_pressed_fill() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.unavailable = true;
        spec.key = Some(KEY);
        spec.checked = Some(true);
        for how in [Interact::Rest, Interact::Hover, Interact::Pressed] {
            let (rects, row) = interacted(mode, spec, how);
            assert!(
                on_row(&rects, row).is_empty(),
                "{mode:?} {how:?}: {rects:?}"
            );
        }
        let harness = drawn(mode, spec);
        assert_eq!(
            text_ink(&harness, LABEL),
            ink(sem.text.disabled),
            "{mode:?}"
        );
        assert_eq!(text_ink(&harness, KEY), ink(sem.text.disabled), "{mode:?}");
        let icons = icon_paths(&harness);
        assert!(!icons.is_empty(), "{mode:?}: no check drawn");
        assert!(
            icons.iter().all(|(c, _)| *c == ink(sem.text.disabled)),
            "{mode:?}: {icons:?}"
        );

        // An available item inks its label otherwise, so the above is not
        // everything being grey.
        let mut live = spec;
        live.unavailable = false;
        let harness = drawn(mode, live);
        assert_ne!(
            text_ink(&harness, LABEL),
            ink(sem.text.disabled),
            "{mode:?}"
        );
        assert_ne!(text_ink(&harness, KEY), ink(sem.text.disabled), "{mode:?}");
    }
}

#[test]
fn an_unavailable_item_with_the_cursor_takes_the_cursor_fill_and_bar_and_stays_grey() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.unavailable = true;
        spec.cursor = true;
        spec.key = Some(KEY);
        let (rects, row) = interacted(mode, spec, Interact::Rest);
        let fills = on_row(&rects, row);
        assert_eq!(fills.len(), 1, "{mode:?}: {fills:?}");
        assert_eq!(fills[0].fill, ink(sem.rows.cursor_background), "{mode:?}");
        let bar = bars(&rects, row);
        assert_eq!(bar.len(), 1, "{mode:?}");
        assert_eq!(bar[0].fill, ink(sem.rows.cursor_bar), "{mode:?}");
        let harness = drawn(mode, spec);
        assert_eq!(
            text_ink(&harness, LABEL),
            ink(sem.text.disabled),
            "{mode:?}"
        );
        assert_eq!(text_ink(&harness, KEY), ink(sem.text.disabled), "{mode:?}");
    }
}

#[test]
fn an_unavailable_submenu_keeps_its_chevron_muted() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.submenu = true;
        spec.unavailable = true;
        let icons = icon_paths(&drawn(mode, spec));
        assert!(!icons.is_empty(), "{mode:?}: no chevron");
        assert!(
            icons.iter().all(|(c, _)| *c == ink(sem.text.muted)),
            "{mode:?}"
        );
        assert_ne!(ink(sem.text.muted), ink(sem.text.disabled));
    }
}

#[test]
fn the_label_sits_space_4_from_the_leading_edge() {
    let harness = drawn(Mode::Light, Spec::new());
    let (_, label) = texts(&harness)
        .into_iter()
        .find(|(s, _)| s == LABEL)
        .unwrap();
    // The galley's own bearing is the same in every case below; compare.
    let lead = label.left() - harness.state().rect.left();
    assert!(near(lead, SPACE_4), "{lead}");
}

#[test]
fn a_checked_item_draws_its_check_and_an_unchecked_one_reserves_the_column() {
    let sem = semantic(false);
    let lead = |spec: Spec| {
        let harness = drawn(Mode::Light, spec);
        let (_, label) = texts(&harness)
            .into_iter()
            .find(|(s, _)| s == LABEL)
            .unwrap();
        (
            label.left() - harness.state().rect.left(),
            icon_paths(&harness),
            harness.state().rect,
        )
    };
    let (plain_lead, plain_icons, _) = lead(Spec::new());
    assert!(plain_icons.is_empty(), "a plain item drew an icon");

    let mut on = Spec::new();
    on.checked = Some(true);
    let (on_lead, on_icons, row) = lead(on);
    assert!(!on_icons.is_empty(), "a checked item drew no check");
    for (c, bounds) in &on_icons {
        assert_eq!(*c, ink(sem.text.primary));
        assert!(
            bounds.left() >= row.left() + SPACE_4 - 0.01
                && bounds.right() <= row.left() + SPACE_4 + ICON_SM + 0.01,
            "the check {bounds:?} is outside its column"
        );
    }
    assert!(near(on_lead - plain_lead, ICON_SM + ICON_LABEL_GAP));

    let mut off = Spec::new();
    off.checked = Some(false);
    let (off_lead, off_icons, _) = lead(off);
    assert!(off_icons.is_empty(), "an unchecked item drew a check");
    assert!(
        near(off_lead, on_lead),
        "the column is reserved: {off_lead} against {on_lead}"
    );
}

#[test]
fn a_key_is_the_21_row_cap_two_steps_from_the_trailing_edge() {
    for mode in MODES {
        let mut spec = Spec::new();
        spec.key = Some(KEY);
        let harness = drawn(mode, spec);
        let row = harness.state().rect;
        let caps: Vec<Painted> = painted(&harness, mode)
            .into_iter()
            .filter(|r| r.is_stroked() && r.rect.height() < row.height())
            .collect();
        assert_eq!(caps.len(), 1, "{mode:?}: {caps:?}");
        assert!(near(caps[0].rect.height(), 21.0), "{mode:?}: {:?}", caps[0]);
        assert!(
            near(row.right() - caps[0].rect.right(), SPACE_2),
            "{mode:?}: {:?} in {row:?}",
            caps[0].rect
        );
        assert!(near(caps[0].rect.center().y, row.center().y), "{mode:?}");
    }
}

#[test]
fn a_submenu_draws_a_muted_chevron_at_the_trailing_edge_and_still_clicks() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new();
        spec.submenu = true;
        let mut harness = drawn(mode, spec);
        let row = harness.state().rect;
        let icons = icon_paths(&harness);
        assert!(!icons.is_empty(), "{mode:?}: no chevron");
        for (c, b) in &icons {
            assert_eq!(*c, ink(sem.text.muted), "{mode:?}");
            assert!(
                b.right() <= row.right() - SPACE_2 + 0.01
                    && b.left() >= row.right() - SPACE_2 - ICON_SM - 0.01,
                "{mode:?}: chevron {b:?} in {row:?}"
            );
        }
        harness
            .get_by_role_and_label(AccessRole::MenuItem, LABEL)
            .click();
        harness.run();
        assert!(harness.state().clicked, "{mode:?}");
    }
}

#[test]
fn an_unavailable_item_reports_no_click_and_reports_its_rect() {
    let mut spec = Spec::new();
    spec.unavailable = true;
    let mut harness = drawn(Mode::Light, spec);
    assert_eq!(harness.state().pointed, None);

    harness
        .get_by_role_and_label(AccessRole::MenuItem, LABEL)
        .hover();
    harness.run();
    let b = harness.state().rect;
    let pointed = harness
        .state()
        .pointed
        .expect("pointed at, it reports its rect");
    assert!(near_rect(pointed, b), "{pointed:?} is not {b:?}");

    harness
        .get_by_role_and_label(AccessRole::MenuItem, LABEL)
        .click();
    harness.run();
    assert!(
        !harness.state().clicked,
        "an unavailable item reported a click"
    );

    let mut live = drawn(Mode::Light, Spec::new());
    live.get_by_role_and_label(AccessRole::MenuItem, LABEL)
        .click();
    live.run();
    assert!(live.state().clicked);
    assert_eq!(live.state().pointed, None);
}

#[test]
fn an_item_is_a_menu_item_node_and_an_unavailable_one_is_disabled_with_its_reason() {
    let mut spec = Spec::new();
    spec.unavailable = true;
    let harness = drawn(Mode::Light, spec);
    let node = harness.get_by_role_and_label(AccessRole::MenuItem, LABEL);
    assert!(node.accesskit_node().is_disabled());
    assert_eq!(node.accesskit_node().description().as_deref(), Some(REASON));

    let live = drawn(Mode::Light, Spec::new());
    let node = live.get_by_role_and_label(AccessRole::MenuItem, LABEL);
    assert!(!node.accesskit_node().is_disabled());
    assert_eq!(node.accesskit_node().description(), None);
}

#[test]
fn an_unavailable_item_takes_no_keyboard_focus() {
    let mut live = drawn(Mode::Light, Spec::new());
    live.get_by_role_and_label(AccessRole::MenuItem, LABEL)
        .focus();
    live.run();
    assert!(live.state().focused, "the harness cannot focus a menu item");

    let mut spec = Spec::new();
    spec.unavailable = true;
    let mut harness = drawn(Mode::Light, spec);
    harness
        .get_by_role_and_label(AccessRole::MenuItem, LABEL)
        .focus();
    harness.run();
    assert!(!harness.state().focused);
}

#[test]
fn a_long_label_is_cut_and_the_key_keeps_its_column() {
    for mode in MODES {
        let mut spec = Spec::new();
        spec.label = "Export the whole of the filtered table to a file";
        spec.key = Some("\u{2318}\u{21e7}E");
        spec.checked = Some(true);
        spec.width = 160.0;
        let harness = drawn(mode, spec);
        let row = harness.state().rect;
        let cap = painted(&harness, mode)
            .into_iter()
            .find(|r| r.is_stroked() && r.rect.height() < row.height())
            .expect("the key is drawn");
        assert!(near(row.right() - cap.rect.right(), SPACE_2), "{mode:?}");

        let label = texts(&harness)
            .into_iter()
            .find(|(s, _)| s.contains('\u{2026}'))
            .unwrap_or_else(|| panic!("{mode:?}: no ellipsis in {:?}", texts(&harness)));
        assert!(label.0.len() < spec.label.len(), "{mode:?}: not cut");
        assert!(
            label.1.right() <= cap.rect.left(),
            "{mode:?}: label {:?} runs under the key {:?}",
            label.1,
            cap.rect
        );
    }
}

#[test]
fn a_separator_is_a_one_pixel_divider_with_space_2_above_and_below() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut harness = Harness::builder()
            .with_size(egui::vec2(280.0, 80.0))
            .build_ui_state(
                move |ui, out: &mut (Rect, Rect)| {
                    theme::apply(ui.ctx(), mode);
                    ui.allocate_ui(egui::vec2(240.0, 60.0), |ui| {
                        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                        let top = ui.cursor().top();
                        menu_separator(ui);
                        out.0 = Rect::from_min_max(
                            egui::pos2(ui.min_rect().left(), top),
                            egui::pos2(ui.min_rect().right(), ui.cursor().top()),
                        );
                    });
                },
                (Rect::NOTHING, Rect::NOTHING),
            );
        harness.run();
        let slot = harness.state().0;
        assert!(
            near(slot.height(), 1.0 + 2.0 * SPACE_2),
            "{mode:?}: {slot:?}"
        );
        let rules: Vec<Painted> = painted(&harness, mode)
            .into_iter()
            .filter(|r| r.fill == ink(sem.borders.divider))
            .collect();
        assert_eq!(rules.len(), 1, "{mode:?}: {rules:?}");
        let rule = rules[0].rect;
        assert!(near(rule.height(), 1.0), "{mode:?}: {rule:?}");
        assert!(near(rule.top() - slot.top(), SPACE_2), "{mode:?}");
        assert!(near(slot.bottom() - rule.bottom(), SPACE_2), "{mode:?}");
        assert!(near(rule.width(), 240.0), "{mode:?}: {rule:?}");
    }
}
