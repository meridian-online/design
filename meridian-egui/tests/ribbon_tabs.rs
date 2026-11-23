//! The ribbon's tab strip and the key tip, read out of the paint list and the
//! accessibility tree: the strip and its rule, the open tab's cell and how it
//! covers that rule, the contextual tab and its divider, the collapsed
//! selection, clicks, and the key tips that sit over the tabs without moving
//! them.

mod common;

use common::{filled, ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::{Color32, CornerRadius, FontFamily, Rect, Vec2};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::control::{KEYCAP_HEIGHT, TAB_BAR_WIDTH};
use meridian_design::radius::{CHIP, CONTROL};
use meridian_design::scales::{MARITIME_DARK, MARITIME_LIGHT};
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{ROW_GRID, SPACE_2, SPACE_4, SPACE_5};
use meridian_egui::{
    key_tip, ribbon_tabs, theme, Mode, RibbonTab, RibbonTabs, RibbonTabsResponse, TabPick,
};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const LABELS: [&str; 4] = ["Bring in", "Shape", "Model", "Publish"];
const KEYS: [&str; 4] = ["B", "S", "M", "P"];
const CTX: &str = "Grid · ranking";

#[derive(Clone, Copy)]
struct Spec {
    stages: usize,
    contextual: bool,
    selected: Option<TabPick>,
    collapsed: bool,
    tips: bool,
    keys: bool,
    width: f32,
}

impl Spec {
    fn new(selected: Option<TabPick>) -> Self {
        Self {
            stages: 4,
            contextual: true,
            selected,
            collapsed: false,
            tips: false,
            keys: true,
            width: 600.0,
        }
    }
}

struct Out {
    strip: Rect,
    rects: Vec<(TabPick, Rect)>,
    clicked: Option<TabPick>,
    double_clicked: Option<TabPick>,
}

fn maritime(mode: Mode, step: usize) -> Color32 {
    ink(if mode.is_dark() {
        MARITIME_DARK[step - 1]
    } else {
        MARITIME_LIGHT[step - 1]
    })
}

fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(spec.width + 40.0, 120.0))
        // Frames short enough that two clicks, a frame apart, are a double click.
        .with_step_dt(0.05)
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(spec.width, 60.0), |ui| {
                    let stages: Vec<RibbonTab> = (0..spec.stages)
                        .map(|i| {
                            let t = RibbonTab::new(LABELS[i]);
                            if spec.keys {
                                t.keystroke(KEYS[i])
                            } else {
                                t
                            }
                        })
                        .collect();
                    let mut tabs = RibbonTabs::new(&stages, spec.selected)
                        .collapsed(spec.collapsed)
                        .key_tips(spec.tips);
                    if spec.contextual {
                        let c = RibbonTab::new(CTX);
                        tabs = tabs.contextual(if spec.keys { c.keystroke("G") } else { c });
                    }
                    let r: RibbonTabsResponse = ribbon_tabs(ui, tabs);
                    out.strip = r.response.rect;
                    out.rects = r.rects;
                    if r.clicked.is_some() {
                        out.clicked = r.clicked;
                    }
                    if r.double_clicked.is_some() {
                        out.double_clicked = r.double_clicked;
                    }
                });
            },
            Out {
                strip: Rect::NOTHING,
                rects: Vec::new(),
                clicked: None,
                double_clicked: None,
            },
        );
    harness.run();
    harness
}

fn label_of(pick: TabPick) -> &'static str {
    match pick {
        TabPick::Stage(i) => LABELS[i],
        TabPick::Contextual => CTX,
    }
}

fn rect_of(harness: &Harness<'_, Out>, pick: TabPick) -> Rect {
    harness
        .state()
        .rects
        .iter()
        .find(|(p, _)| *p == pick)
        .map(|(_, r)| *r)
        .unwrap_or_else(|| panic!("no rect for {pick:?}"))
}

#[derive(Clone, Copy, Debug)]
enum Interact {
    Hover,
    Pressed,
    Focus,
}

fn interact(harness: &mut Harness<'_, Out>, pick: TabPick, how: Interact) {
    let label = label_of(pick);
    match how {
        Interact::Hover => {
            harness
                .get_by_role_and_label(AccessRole::Tab, label)
                .hover();
            harness.run();
        }
        Interact::Pressed => {
            harness
                .get_by_role_and_label(AccessRole::Tab, label)
                .hover();
            harness.run();
            let pos = rect_of(harness, pick).center();
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
                .get_by_role_and_label(AccessRole::Tab, label)
                .focus();
            harness.run();
        }
    }
}

/// The frame's paint list with `pick` interacted with.
fn frame(
    mode: Mode,
    spec: Spec,
    pick: TabPick,
    how: Interact,
) -> (Vec<Painted>, Harness<'static, Out>) {
    let mut harness = drawn(mode, spec);
    interact(&mut harness, pick, how);
    (painted(&harness, mode), harness)
}

// ─── reading the text ───────────────────────────────────────────────────────

struct Txt {
    text: String,
    colour: Color32,
    family: FontFamily,
    size: Vec2,
}

fn texts(harness: &Harness<'_, Out>) -> Vec<Txt> {
    fn walk(shape: &egui::Shape, out: &mut Vec<Txt>) {
        match shape {
            egui::Shape::Text(t) => {
                let s = &t.galley.job.sections[0];
                out.push(Txt {
                    text: t.galley.text().to_owned(),
                    colour: t.override_text_color.unwrap_or(s.format.color),
                    family: s.format.font_id.family.clone(),
                    size: t.galley.size(),
                });
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

fn text_of(harness: &Harness<'_, Out>, label: &str) -> Txt {
    texts(harness)
        .into_iter()
        .find(|t| t.text == label)
        .unwrap_or_else(|| panic!("no text {label:?}"))
}

fn medium() -> FontFamily {
    FontFamily::Name("Inter-Medium".into())
}

/// Every rect the frame painted, the page's own fill included.
fn all_rects(harness: &Harness<'_, Out>) -> Vec<Painted> {
    fn walk(shape: &egui::Shape, clip: Rect, out: &mut Vec<Painted>) {
        match shape {
            egui::Shape::Rect(r) => out.push(Painted::from(r, clip)),
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, clip, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for c in &harness.output().shapes {
        walk(&c.shape, c.clip_rect, &mut out);
    }
    out
}

/// egui places a child `Ui` on a fraction of a pixel, so a key whose width is
/// not a whole number lands within a sixteenth of the exact quarter.
fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1.0 / 16.0
}

fn corners(nw: u8, ne: u8, sw: u8, se: u8) -> CornerRadius {
    CornerRadius { nw, ne, sw, se }
}

/// The strip's rule: 1px, subtle, along the foot.
fn rule_in(rects: &[Painted], strip: Rect, mode: Mode) -> Option<usize> {
    let subtle = ink(semantic(mode.is_dark()).borders.subtle);
    rects.iter().position(|r| {
        r.fill == subtle
            && near(r.rect.height(), 1.0)
            && near(r.rect.bottom(), strip.bottom())
            && near(r.rect.left(), strip.left())
            && near(r.rect.right(), strip.right())
    })
}

// ─── the strip ──────────────────────────────────────────────────────────────

#[test]
fn the_strip_is_a_grid_row_on_the_title_bar_with_a_subtle_rule_along_its_foot() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new(Some(TabPick::Stage(1))));
        let strip = h.state().strip;
        assert!(near(strip.height(), ROW_GRID), "{mode:?}: {strip:?}");
        assert!(near(strip.width(), 600.0), "{mode:?}: {strip:?}");
        let all = all_rects(&h);
        let bar = ink(sem.containers.title_bar_background);
        assert!(
            all.iter()
                .any(|r| r.fill == bar && near_rect(r.rect, strip)),
            "{mode:?}: no title-bar fill over the strip"
        );
        let rects = painted(&h, mode);
        assert!(rule_in(&rects, strip, mode).is_some(), "{mode:?}: no rule");
    }
}

#[test]
fn tabs_pad_their_labels_and_start_inside_the_strip_s_side_padding() {
    for mode in MODES {
        let h = drawn(mode, Spec::new(None));
        let strip = h.state().strip;
        let first = rect_of(&h, TabPick::Stage(0));
        assert!(near(first.left(), strip.left() + SPACE_4), "{first:?}");
        for pick in [0, 1, 2, 3].map(TabPick::Stage) {
            let r = rect_of(&h, pick);
            assert!(near(r.height(), ROW_GRID), "{pick:?}: {r:?}");
            assert!(near(r.top(), strip.top()));
            let w = text_of(&h, label_of(pick)).size.x;
            assert!(near(r.width(), w + 2.0 * SPACE_5), "{pick:?}: {r:?} vs {w}");
        }
        for i in 0..3 {
            let (a, b) = (
                rect_of(&h, TabPick::Stage(i)),
                rect_of(&h, TabPick::Stage(i + 1)),
            );
            assert!(near(a.right(), b.left()), "{a:?} {b:?}");
        }
    }
}

#[test]
fn a_rest_tab_is_its_label_in_secondary_ink_with_no_fill() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new(Some(TabPick::Stage(1))));
        let t = text_of(&h, "Model");
        assert_eq!(t.colour, ink(sem.text.secondary), "{mode:?}");
        assert_eq!(t.family, FontFamily::Proportional, "{mode:?}");
        let r = rect_of(&h, TabPick::Stage(2));
        let rects = painted(&h, mode);
        assert!(
            !rects
                .iter()
                .any(|p| near_rect(p.rect, r) && p.fill != Color32::TRANSPARENT),
            "{mode:?}: a rest tab drew a fill: {rects:?}"
        );
    }
}

#[test]
fn a_hovered_tab_takes_the_primary_ink_and_no_fill() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = frame(
            mode,
            Spec::new(Some(TabPick::Stage(1))),
            TabPick::Stage(2),
            Interact::Hover,
        );
        assert_eq!(
            text_of(&h, "Model").colour,
            ink(sem.text.primary),
            "{mode:?}"
        );
        assert_eq!(text_of(&h, "Publish").colour, ink(sem.text.secondary));
        let r = rect_of(&h, TabPick::Stage(2));
        assert!(!rects
            .iter()
            .any(|p| near_rect(p.rect, r) && p.fill != Color32::TRANSPARENT));
    }
}

#[test]
fn a_pressed_tab_takes_the_neutral_active_wash() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let (rects, h) = frame(
            mode,
            Spec::new(Some(TabPick::Stage(1))),
            TabPick::Stage(2),
            Interact::Pressed,
        );
        let r = rect_of(&h, TabPick::Stage(2));
        let w = filled(&rects, ink(n.background.active));
        assert_eq!(w.len(), 1, "{mode:?}: {rects:?}");
        assert!(
            near_rect(w[0].rect, r),
            "{mode:?}: {:?} vs {r:?}",
            w[0].rect
        );
        assert_eq!(w[0].radius, corners(CONTROL as u8, CONTROL as u8, 0, 0));
    }
}

#[test]
fn a_pressed_contextual_tab_shows_the_neutral_active_wash_not_its_maritime_one() {
    for mode in MODES {
        let n = semantic(mode.is_dark()).role(Role::Neutral).to_owned();
        let (rects, h) = frame(
            mode,
            Spec::new(Some(TabPick::Stage(1))),
            TabPick::Contextual,
            Interact::Pressed,
        );
        let r = rect_of(&h, TabPick::Contextual);
        let w = filled(&rects, ink(n.background.active));
        assert_eq!(w.len(), 1, "{mode:?}: {rects:?}");
        assert!(near_rect(w[0].rect, r));
        assert!(filled(&rects, maritime(mode, 3)).is_empty(), "{mode:?}");
    }
}

#[test]
fn a_focused_tab_is_ringed_inside_its_edge() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, h) = frame(
            mode,
            Spec::new(Some(TabPick::Stage(1))),
            TabPick::Stage(3),
            Interact::Focus,
        );
        let r = rect_of(&h, TabPick::Stage(3));
        let ring = rects
            .iter()
            .find(|p| {
                p.fill == Color32::TRANSPARENT
                    && p.stroke.color == ink(sem.borders.focus)
                    && near_rect(p.rect, r)
            })
            .unwrap_or_else(|| panic!("{mode:?}: no ring in {rects:?}"));
        assert!(near(ring.stroke.width, 2.0), "{ring:?}");
        let ring_corner = meridian_design::focus::ring_radius(CONTROL).round() as u8;
        assert_eq!(ring.radius, corners(ring_corner, ring_corner, 0, 0));
        // Nobody else is ringed.
        assert_eq!(
            rects
                .iter()
                .filter(|p| p.stroke.color == ink(sem.borders.focus))
                .count(),
            1
        );
    }
}

// ─── the open tab ───────────────────────────────────────────────────────────

#[test]
fn the_open_tab_is_a_raised_cell_rounded_at_the_top_only_with_a_medium_label() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new(Some(TabPick::Stage(1))));
        let r = rect_of(&h, TabPick::Stage(1));
        let rects = painted(&h, mode);
        let raised = ink(sem.surfaces.raised);
        let cell: Vec<_> = filled(&rects, raised)
            .into_iter()
            .filter(|p| near_rect(p.rect, r))
            .collect();
        assert_eq!(cell.len(), 1, "{mode:?}: {rects:?}");
        assert_eq!(cell[0].radius, corners(CONTROL as u8, CONTROL as u8, 0, 0));
        let rule = rects
            .iter()
            .find(|p| {
                p.fill == Color32::TRANSPARENT
                    && p.stroke.color == ink(sem.borders.subtle)
                    && near(p.stroke.width, 1.0)
                    && p.rect.left() <= r.left() + 0.01
                    && p.rect.right() >= r.right() - 0.01
                    && near(p.rect.top(), r.top())
            })
            .unwrap_or_else(|| panic!("{mode:?}: no border in {rects:?}"));
        assert_eq!(rule.radius, corners(CONTROL as u8, CONTROL as u8, 0, 0));
        let t = text_of(&h, "Shape");
        assert_eq!(t.colour, ink(sem.text.primary));
        assert_eq!(t.family, medium(), "{mode:?}");
        // No other tab is a cell.
        assert_eq!(filled(&rects, raised).len(), 1, "{mode:?}");
    }
}

#[test]
fn the_open_tab_covers_the_strip_s_rule_beneath_it() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new(Some(TabPick::Stage(1))));
        let strip = h.state().strip;
        let r = rect_of(&h, TabPick::Stage(1));
        let rects = painted(&h, mode);
        let rule = rule_in(&rects, strip, mode).expect("the rule");
        let cell = rects
            .iter()
            .position(|p| p.fill == ink(sem.surfaces.raised) && near_rect(p.rect, r))
            .expect("the cell");
        // The cell reaches the strip's foot and is painted after the rule, so
        // the rule is hidden under it.
        assert!(near(rects[cell].rect.bottom(), strip.bottom()), "{mode:?}");
        assert!(cell > rule, "{mode:?}: the cell is painted before the rule");
        // The cell's own border has no bottom run: it is drawn past the foot
        // and cut off by the strip's clip.
        let border = rects
            .iter()
            .find(|p| {
                p.is_stroked()
                    && p.stroke.color == ink(sem.borders.subtle)
                    && p.rect.top() <= r.top() + 0.01
                    && p.rect.left() <= r.left() + 0.01
                    && p.rect.right() >= r.right() - 0.01
                    && p.rect.height() > 1.5
            })
            .expect("the border");
        assert!(border.rect.bottom() > strip.bottom() + 0.5, "{border:?}");
        assert!(near(border.clip.bottom(), strip.bottom()), "{border:?}");
    }
}

#[test]
fn a_collapsed_strip_draws_no_cell() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new(Some(TabPick::Stage(1)));
        spec.collapsed = true;
        let h = drawn(mode, spec);
        let rects = painted(&h, mode);
        assert!(
            filled(&rects, ink(sem.surfaces.raised)).is_empty(),
            "{mode:?}: {rects:?}"
        );
        let r = rect_of(&h, TabPick::Stage(1));
        assert!(
            !rects.iter().any(|p| p.is_stroked()
                && near(p.rect.top(), r.top())
                && p.rect.width() >= r.width() - 0.01
                && p.rect.height() > 1.5),
            "{mode:?}: a border was drawn: {rects:?}"
        );
        // The selected tab keeps the open tab's label.
        let t = text_of(&h, "Shape");
        assert_eq!(t.colour, ink(sem.text.primary), "{mode:?}");
        assert_eq!(t.family, medium(), "{mode:?}");
        assert_eq!(text_of(&h, "Model").colour, ink(sem.text.secondary));
    }
}

// ─── the contextual tab ─────────────────────────────────────────────────────

#[test]
fn the_contextual_tab_follows_a_divider_and_wears_the_maritime_wash() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new(Some(TabPick::Stage(1))));
        let strip = h.state().strip;
        let rects = painted(&h, mode);
        let last = rect_of(&h, TabPick::Stage(3));
        let ctx = rect_of(&h, TabPick::Contextual);
        // The divider: 1 wide, inset SPACE_2 top and bottom, SPACE_4 each side.
        let div: Vec<_> = filled(&rects, ink(sem.borders.subtle))
            .into_iter()
            .filter(|p| near(p.rect.width(), 1.0) && p.rect.height() > 1.5)
            .collect();
        assert_eq!(div.len(), 1, "{mode:?}: {rects:?}");
        let d = div[0].rect;
        assert!(near(d.top(), strip.top() + SPACE_2), "{d:?}");
        assert!(near(d.bottom(), strip.bottom() - SPACE_2), "{d:?}");
        assert!(near(d.left(), last.right() + SPACE_4), "{d:?} {last:?}");
        assert!(near(ctx.left(), d.right() + SPACE_4), "{d:?} {ctx:?}");
        // The wash and the ink.
        let wash = filled(&rects, maritime(mode, 3));
        assert_eq!(wash.len(), 1, "{mode:?}");
        assert!(near_rect(wash[0].rect, ctx));
        assert_eq!(wash[0].radius, corners(CONTROL as u8, CONTROL as u8, 0, 0));
        let t = text_of(&h, CTX);
        assert_eq!(t.colour, maritime(mode, 11), "{mode:?}");
        assert_eq!(t.family, FontFamily::Proportional);
    }
}

#[test]
fn the_open_contextual_tab_is_the_raised_cell_with_a_maritime_bar_on_its_top_edge() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new(Some(TabPick::Contextual)));
        let ctx = rect_of(&h, TabPick::Contextual);
        let rects = painted(&h, mode);
        assert!(
            filled(&rects, maritime(mode, 3)).is_empty(),
            "{mode:?}: the wash stays"
        );
        let cell = filled(&rects, ink(sem.surfaces.raised));
        assert_eq!(cell.len(), 1, "{mode:?}");
        assert!(near_rect(cell[0].rect, ctx));
        assert_eq!(cell[0].radius, corners(CONTROL as u8, CONTROL as u8, 0, 0));
        let bar = filled(&rects, maritime(mode, 9));
        assert_eq!(bar.len(), 1, "{mode:?}: {rects:?}");
        let b = bar[0];
        assert!(near(b.rect.height(), TAB_BAR_WIDTH), "{b:?}");
        assert!(
            b.rect.top() >= ctx.top() - 0.01 && b.rect.top() <= ctx.top() + 1.01,
            "{b:?}"
        );
        assert!(b.rect.left() >= ctx.left() - 0.01 && b.rect.right() <= ctx.right() + 0.01);
        assert!(b.rect.width() >= ctx.width() - 2.01, "{b:?}");
        assert_eq!(b.radius, CornerRadius::ZERO);
        let t = text_of(&h, CTX);
        assert_eq!(t.family, medium());
        assert_eq!(t.colour, maritime(mode, 12));
    }
}

#[test]
fn a_selected_contextual_tab_while_collapsed_keeps_its_wash_and_takes_medium() {
    for mode in MODES {
        let mut spec = Spec::new(Some(TabPick::Contextual));
        spec.collapsed = true;
        let h = drawn(mode, spec);
        let rects = painted(&h, mode);
        assert_eq!(filled(&rects, maritime(mode, 3)).len(), 1, "{mode:?}");
        assert!(
            filled(&rects, maritime(mode, 9)).is_empty(),
            "{mode:?}: a bar"
        );
        let t = text_of(&h, CTX);
        assert_eq!(t.family, medium());
        assert_eq!(t.colour, maritime(mode, 12));
    }
}

// ─── reporting ──────────────────────────────────────────────────────────────

#[test]
fn clicks_and_double_clicks_report_the_tab_including_the_selected_one() {
    for pick in [TabPick::Stage(0), TabPick::Stage(1), TabPick::Contextual] {
        let mut h = drawn(Mode::Light, Spec::new(Some(TabPick::Stage(1))));
        let pos = rect_of(&h, pick).center();
        h.event(egui::Event::PointerMoved(pos));
        h.run();
        for pressed in [true, false] {
            h.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
        }
        h.run();
        assert_eq!(h.state().clicked, Some(pick), "{pick:?}");
        assert_eq!(h.state().double_clicked, None, "{pick:?}");
        // Two quick clicks make a double click, on a fresh strip.
        let mut h = drawn(Mode::Light, Spec::new(Some(TabPick::Stage(1))));
        h.event(egui::Event::PointerMoved(pos));
        h.run();
        // Each click takes one frame, so the second lands inside the
        // double-click delay.
        for _ in 0..2 {
            for pressed in [true, false] {
                h.event(egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                });
            }
            h.step();
        }
        h.run();
        assert_eq!(h.state().double_clicked, Some(pick), "{pick:?}");
    }
}

#[test]
fn rects_holds_every_tab() {
    let h = drawn(Mode::Light, Spec::new(None));
    let picks: Vec<TabPick> = h.state().rects.iter().map(|(p, _)| *p).collect();
    assert_eq!(
        picks,
        vec![
            TabPick::Stage(0),
            TabPick::Stage(1),
            TabPick::Stage(2),
            TabPick::Stage(3),
            TabPick::Contextual
        ]
    );
    let mut spec = Spec::new(None);
    spec.contextual = false;
    assert_eq!(drawn(Mode::Light, spec).state().rects.len(), 4);
}

#[test]
fn nothing_is_open_or_selected_when_the_caller_selects_nothing() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, Spec::new(None));
        let rects = painted(&h, mode);
        assert!(filled(&rects, ink(sem.surfaces.raised)).is_empty());
        for l in LABELS {
            assert_eq!(text_of(&h, l).family, FontFamily::Proportional);
        }
        for n in h.query_all_by_role(AccessRole::Tab) {
            assert_ne!(n.accesskit_node().is_selected(), Some(true));
        }
    }
}

// ─── key tips ───────────────────────────────────────────────────────────────

/// The keycap faces on the frame: the sunken fill with the chip corner.
fn faces(rects: &[Painted], mode: Mode) -> Vec<Painted> {
    let sunken = ink(semantic(mode.is_dark()).surfaces.sunken);
    filled(rects, sunken)
        .into_iter()
        .filter(|p| p.radius == CornerRadius::same(CHIP as u8))
        .collect()
}

#[test]
fn key_tips_draw_one_key_per_tab_with_a_keystroke_and_do_not_move_the_tabs() {
    for mode in MODES {
        let off = drawn(mode, Spec::new(Some(TabPick::Stage(1))));
        let mut spec = Spec::new(Some(TabPick::Stage(1)));
        spec.tips = true;
        let on = drawn(mode, spec);
        assert_eq!(off.state().rects.len(), on.state().rects.len());
        for ((pa, ra), (pb, rb)) in off.state().rects.iter().zip(&on.state().rects) {
            assert_eq!(pa, pb);
            assert!(near_rect(*ra, *rb), "{pa:?}: {ra:?} vs {rb:?}");
        }
        assert!(near_rect(off.state().strip, on.state().strip));
        assert!(
            faces(&painted(&off, mode), mode).is_empty(),
            "{mode:?}: keys with tips off"
        );
        let keys = faces(&painted(&on, mode), mode);
        assert_eq!(keys.len(), 5, "{mode:?}: one per tab");
        // Each tab has a key at its corner; the layers' order is not the tabs'.
        for (pick, t) in &on.state().rects {
            let k = keys
                .iter()
                .find(|k| {
                    close(k.rect.right(), t.right() + k.rect.width() / 4.0)
                        && close(k.rect.bottom(), t.bottom() + k.rect.height() / 2.0)
                })
                .unwrap_or_else(|| panic!("{pick:?}: no key at the corner of {t:?} in {keys:?}"));
            assert!(near(k.rect.height(), KEYCAP_HEIGHT), "{k:?}");
        }
        // The overlay shadow sits under each key, in the shadow's colour.
        let shadow = theme::meridian_visuals(mode).popup_shadow;
        let casts: Vec<_> = painted(&on, mode)
            .into_iter()
            .filter(|p| p.fill == shadow.color && p.rect.width() > 4.0 && p.rect.height() > 4.0)
            .collect();
        assert_eq!(casts.len(), 5, "{mode:?}: {casts:?}");
        // The keystrokes are in the frame.
        for k in ["B", "S", "M", "P", "G"] {
            let _ = text_of(&on, k);
        }
    }
}

#[test]
fn a_tab_without_a_keystroke_gets_no_tip_and_tips_off_draws_none() {
    for mode in MODES {
        let mut spec = Spec::new(None);
        spec.tips = true;
        spec.keys = false;
        let h = drawn(mode, spec);
        assert!(faces(&painted(&h, mode), mode).is_empty(), "{mode:?}");
    }
}

#[test]
fn key_tip_draws_a_21_key_offset_from_its_target_over_the_foreground() {
    for mode in MODES {
        let target = Rect::from_min_size(egui::pos2(50.0, 30.0), egui::vec2(60.0, 24.0));
        let mut h = Harness::builder()
            .with_size(egui::vec2(300.0, 120.0))
            .build_ui(move |ui| {
                theme::apply(ui.ctx(), mode);
                key_tip(ui, target, "F12");
            });
        h.run();
        let keys = faces(&painted(&h, mode), mode);
        assert_eq!(keys.len(), 1, "{mode:?}");
        let k = keys[0].rect;
        assert!(near(k.height(), KEYCAP_HEIGHT));
        assert!(close(k.right(), target.right() + k.width() / 4.0), "{k:?}");
        assert!(
            close(k.bottom(), target.bottom() + k.height() / 2.0),
            "{k:?}"
        );
        let n = h.query_all_by_role(AccessRole::Label).count();
        assert!(n >= 1);
        h.get_by_label("F12");
    }
}

// ─── degenerate input ───────────────────────────────────────────────────────

#[test]
fn no_stages_and_no_contextual_draws_the_empty_strip() {
    for mode in MODES {
        let mut spec = Spec::new(None);
        spec.stages = 0;
        spec.contextual = false;
        spec.tips = true;
        let h = drawn(mode, spec);
        assert!(h.state().rects.is_empty());
        let strip = h.state().strip;
        assert!(near(strip.height(), ROW_GRID), "{strip:?}");
        let rects = painted(&h, mode);
        assert!(rule_in(&rects, strip, mode).is_some(), "{mode:?}: the rule");
        assert!(faces(&rects, mode).is_empty());
        assert_eq!(h.query_all_by_role(AccessRole::Tab).count(), 0);
        // A selection that points at nothing opens nothing.
        let mut spec = Spec::new(Some(TabPick::Stage(7)));
        spec.stages = 2;
        spec.contextual = false;
        let h = drawn(mode, spec);
        let sem = semantic(mode.is_dark());
        assert!(filled(&painted(&h, mode), ink(sem.surfaces.raised)).is_empty());
    }
}

#[test]
fn a_contextual_tab_alone_has_no_divider() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut spec = Spec::new(None);
        spec.stages = 0;
        let h = drawn(mode, spec);
        assert_eq!(h.state().rects.len(), 1);
        let divs = filled(&painted(&h, mode), ink(sem.borders.subtle))
            .into_iter()
            .filter(|p| near(p.rect.width(), 1.0) && p.rect.height() > 1.5)
            .count();
        assert_eq!(divs, 0, "{mode:?}");
    }
}

// ─── accessibility ──────────────────────────────────────────────────────────

#[test]
fn the_strip_is_a_tab_list_of_tabs_with_the_open_one_selected() {
    let h = drawn(Mode::Light, Spec::new(Some(TabPick::Stage(1))));
    let list = h.get_by_role(AccessRole::TabList);
    assert_eq!(list.query_all_by_role(AccessRole::Tab).count(), 5);
    for l in LABELS.iter().chain([&CTX]) {
        let tab = h.get_by_role_and_label(AccessRole::Tab, l);
        assert_eq!(
            tab.accesskit_node().is_selected(),
            Some(*l == "Shape"),
            "{l}"
        );
    }
}

// ─── key tips and the strip's edge ──────────────────────────────────────────

#[test]
fn a_tab_clipped_by_the_strip_gets_no_key_tip_and_the_others_keep_theirs() {
    for mode in MODES {
        let mut spec = Spec::new(None);
        spec.tips = true;
        // Room for the stages and not the contextual tab.
        spec.width = 330.0;
        let h = drawn(mode, spec);
        let strip = h.state().strip;
        let ctx = rect_of(&h, TabPick::Contextual);
        assert!(ctx.right() > strip.right(), "{ctx:?} {strip:?}");
        let keys = faces(&painted(&h, mode), mode);
        let inside: Vec<_> = h
            .state()
            .rects
            .iter()
            .filter(|(_, r)| strip.contains_rect(*r))
            .collect();
        assert_eq!(keys.len(), inside.len(), "{mode:?}: {keys:?}");
        assert!(inside.len() >= 2 && inside.len() < 5);
        for k in &keys {
            assert!(k.rect.left() < strip.right(), "{mode:?}: {k:?}");
        }
    }
}

#[test]
fn tips_do_not_take_the_pointer_from_the_tab_beneath() {
    let mut spec = Spec::new(Some(TabPick::Stage(1)));
    spec.tips = true;
    let mut h = drawn(Mode::Light, spec);
    // A tab's tip covers the next tab's leading corner; point at the corner.
    let a = rect_of(&h, TabPick::Stage(0));
    let b = rect_of(&h, TabPick::Stage(1));
    let pos = egui::pos2(b.left() + 3.0, a.bottom() - 2.0);
    assert!(b.contains(pos));
    h.event(egui::Event::PointerMoved(pos));
    h.run();
    for pressed in [true, false] {
        h.event(egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        });
    }
    h.run();
    assert_eq!(h.state().clicked, Some(TabPick::Stage(1)));
}
