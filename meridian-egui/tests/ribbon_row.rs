//! The ribbon's command row, read out of the paint list and the accessibility
//! tree: the band and its rule, the quiet buttons and the dividers between
//! groups, how labels are dropped from the trailing end to fit, the clip at the
//! trailing edge, unavailable commands, key tips, and the empty forms.

mod common;

use common::{filled, ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::{Color32, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::control::{KEYCAP_HEIGHT, KEYCAP_HEIGHT_IN_CONTROL};
use meridian_design::semantic::semantic;
use meridian_design::spacing::{BAND_HEIGHT, SPACE_1, SPACE_3, SPACE_4, SPACE_6};
use meridian_egui::{
    icons, ribbon_row, theme, Button, CommandAt, Mode, RibbonRow, RibbonRowResponse, Weight,
};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];

#[derive(Clone, Copy)]
struct Spec {
    /// Which of the three sample groups are present, in order.
    groups: [bool; 3],
    tips: bool,
    width: f32,
}

impl Spec {
    fn new(width: f32) -> Self {
        Self {
            groups: [true; 3],
            tips: false,
            width,
        }
    }
}

struct Out {
    row: Rect,
    rects: Vec<(CommandAt, Rect)>,
    icon_only: Vec<CommandAt>,
    clicked: Option<CommandAt>,
    pointed: Option<(CommandAt, Rect)>,
}

fn at(group: usize, index: usize) -> CommandAt {
    CommandAt { group, index }
}

/// The sheet's commands: three groups, Join unavailable, one carrying the
/// primary weight to show the row draws it quiet.
fn drawn(mode: Mode, spec: Spec) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(spec.width + 40.0, 160.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(spec.width, 80.0), |ui| {
                    let icon = &icons::ALL[0];
                    let a = [
                        Button::new("Filter").icon(icon).keystroke("F"),
                        Button::new("Sort").icon(icon).keystroke("S"),
                        Button::new("Hide")
                            .icon(icon)
                            .keystroke("H")
                            .weight(Weight::Primary),
                    ];
                    let b = [
                        Button::new("Derive").icon(icon).keystroke("D"),
                        Button::new("Join")
                            .icon(icon)
                            .keystroke("J")
                            .unavailable("Join needs a second table."),
                        Button::new("Group").icon(icon).keystroke("G"),
                    ];
                    let c = [Button::new("Delete").icon(icon).keystroke("\u{232b}")];
                    let all: [&[Button]; 3] = [&a, &b, &c];
                    let groups: Vec<&[Button]> =
                        (0..3).filter(|i| spec.groups[*i]).map(|i| all[i]).collect();
                    let r: RibbonRowResponse =
                        ribbon_row(ui, RibbonRow::new(&groups).key_tips(spec.tips));
                    out.row = r.response.rect;
                    out.rects = r.rects;
                    out.icon_only = r.icon_only;
                    if r.clicked.is_some() {
                        out.clicked = r.clicked;
                    }
                    if r.pointed_unavailable.is_some() {
                        out.pointed = r.pointed_unavailable;
                    }
                });
            },
            Out {
                row: Rect::NOTHING,
                rects: Vec::new(),
                icon_only: Vec::new(),
                clicked: None,
                pointed: None,
            },
        );
    harness.run();
    harness
}

fn rect_of(h: &Harness<'_, Out>, c: CommandAt) -> Rect {
    h.state()
        .rects
        .iter()
        .find(|(k, _)| *k == c)
        .map(|(_, r)| *r)
        .unwrap_or_else(|| panic!("no rect for {c:?}"))
}

fn press(h: &mut Harness<'_, Out>, pos: egui::Pos2) {
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

// ─── the band ───────────────────────────────────────────────────────────────

#[test]
fn the_band_is_raised_with_a_subtle_rule_on_its_foot() {
    for mode in MODES {
        let h = drawn(mode, Spec::new(600.0));
        let sem = semantic(mode.is_dark());
        let row = h.state().row;
        assert!(near(row.height(), BAND_HEIGHT), "{row:?}");
        assert!(near(row.width(), 600.0), "{row:?}");
        let all = painted(&h, mode);
        let bg = filled(&all, ink(sem.surfaces.raised));
        assert!(
            bg.iter().any(|p| near_rect(p.rect, row)),
            "{mode:?}: no raised band over {row:?}"
        );
        let rules: Vec<Painted> = filled(&all, ink(sem.borders.subtle))
            .into_iter()
            .filter(|p| near(p.rect.height(), 1.0) && near(p.rect.width(), row.width()))
            .collect();
        assert_eq!(rules.len(), 1, "{mode:?}: {rules:?}");
        assert!(near(rules[0].rect.bottom(), row.bottom()));
        assert!(near(rules[0].rect.left(), row.left()));
    }
}

#[test]
fn commands_sit_between_side_padding_with_two_between_neighbours_and_dividers_between_groups() {
    for mode in MODES {
        let h = drawn(mode, Spec::new(900.0));
        let row = h.state().row;
        let first = rect_of(&h, at(0, 0));
        assert!(near(first.left() - row.left(), SPACE_4), "{first:?}");
        for c in [at(0, 0), at(0, 1), at(0, 2), at(1, 0), at(1, 2), at(2, 0)] {
            let r = rect_of(&h, c);
            assert!(near(r.height(), 24.0), "{c:?} {r:?}");
            assert!(near(r.center().y, row.center().y), "{c:?} {r:?}");
        }
        let (a, b) = (rect_of(&h, at(0, 0)), rect_of(&h, at(0, 1)));
        assert!(near(b.left() - a.right(), SPACE_1), "{a:?} {b:?}");
        // A divider: 1px wide, 16 tall, centred, with SPACE_3 either side.
        let sem = semantic(mode.is_dark());
        let all = painted(&h, mode);
        let dividers: Vec<Painted> = filled(&all, ink(sem.borders.subtle))
            .into_iter()
            .filter(|p| near(p.rect.width(), 1.0) && near(p.rect.height(), SPACE_6))
            .collect();
        assert_eq!(dividers.len(), 2, "{mode:?}: {dividers:?}");
        let (end, next) = (rect_of(&h, at(0, 2)), rect_of(&h, at(1, 0)));
        let d = dividers
            .iter()
            .find(|d| d.rect.left() > end.right() && d.rect.right() < next.left())
            .expect("a divider between groups 0 and 1");
        assert!(near(d.rect.left() - end.right(), SPACE_3), "{d:?} {end:?}");
        assert!(
            near(next.left() - d.rect.right(), SPACE_3),
            "{d:?} {next:?}"
        );
        assert!(near(d.rect.center().y, row.center().y), "{d:?}");
    }
}

#[test]
fn buttons_are_drawn_quiet_whatever_weight_they_carry_with_spacing_3_leading() {
    for mode in MODES {
        let h = drawn(mode, Spec::new(900.0));
        let sem = semantic(mode.is_dark());
        let all = painted(&h, mode);
        // No accent fill: Hide is primary and is drawn without it.
        let accent = semantic(mode.is_dark())
            .role(meridian_design::Role::Accent)
            .background
            .base;
        assert!(
            filled(&all, ink(accent)).is_empty(),
            "{mode:?}: a primary fill"
        );
        // At rest a quiet button has no box: no wash of the neutral fills.
        let n = sem.role(meridian_design::Role::Neutral);
        for (c, r) in &h.state().rects {
            assert!(
                !all.iter()
                    .any(|p| p.rect == *r && (p.fill == ink(n.background.base) || p.is_stroked())),
                "{mode:?}: {c:?} has a box at rest"
            );
        }
        // The icon starts SPACE_3 inside its button: the ink is painted, so
        // read it off the first key-free measure: the width of Filter is
        // SPACE_3 + icon + gap + label + gap + key + inset.
        let r = rect_of(&h, at(0, 0));
        assert!(r.width() > 2.0 * SPACE_3 + 16.0, "{r:?}");
    }
}

// ─── fitting ────────────────────────────────────────────────────────────────

const ORDER: [CommandAt; 7] = [
    CommandAt { group: 0, index: 0 },
    CommandAt { group: 0, index: 1 },
    CommandAt { group: 0, index: 2 },
    CommandAt { group: 1, index: 0 },
    CommandAt { group: 1, index: 1 },
    CommandAt { group: 1, index: 2 },
    CommandAt { group: 2, index: 0 },
];

/// The row's width when the last `k` commands have lost their labels, built
/// from the widths the row itself reports with everything drawn (`full`) and
/// with everything dropped (`bare`).
fn width_with_dropped(full: &[f32], bare: &[f32], k: usize) -> f32 {
    let n = ORDER.len();
    let mut total = 2.0 * SPACE_4;
    for (i, c) in ORDER.iter().enumerate() {
        total += if i >= n - k { bare[i] } else { full[i] };
        if i + 1 < n {
            total += if ORDER[i + 1].group == c.group {
                SPACE_1
            } else {
                2.0 * SPACE_3 + 1.0
            };
        }
    }
    total
}

fn widths(h: &Harness<'_, Out>) -> Vec<f32> {
    ORDER.iter().map(|c| rect_of(h, *c).width()).collect()
}

#[test]
fn at_full_width_every_label_is_drawn_and_icon_only_is_empty() {
    let h = drawn(Mode::Light, Spec::new(900.0));
    assert!(h.state().icon_only.is_empty(), "{:?}", h.state().icon_only);
    assert_eq!(h.state().rects.len(), 7);
    for label in [
        "Filter", "Sort", "Hide", "Derive", "Join", "Group", "Delete",
    ] {
        assert!(
            painted_text(&h).iter().any(|t| t == label),
            "no label {label:?}"
        );
    }
}

fn painted_text(h: &Harness<'_, Out>) -> Vec<String> {
    fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(t) => out.push(t.galley.text().to_owned()),
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for c in &h.output().shapes {
        walk(&c.shape, &mut out);
    }
    out
}

#[test]
fn labels_are_dropped_from_the_trailing_end_until_it_fits() {
    let full = widths(&drawn(Mode::Light, Spec::new(900.0)));
    let bare = widths(&drawn(Mode::Light, Spec::new(10.0)));
    assert!(
        bare.iter().zip(&full).all(|(b, f)| b < f),
        "{bare:?} {full:?}"
    );
    for k in [3usize, 5] {
        let fits = width_with_dropped(&full, &bare, k);
        // Exactly wide enough for `k` dropped: that many, from the end.
        let h = drawn(Mode::Light, Spec::new(fits));
        assert_eq!(
            h.state().icon_only,
            ORDER[ORDER.len() - k..].to_vec(),
            "width {fits}"
        );
        // A pixel narrower: one more.
        let h = drawn(Mode::Light, Spec::new(fits - 1.0));
        assert_eq!(
            h.state().icon_only,
            ORDER[ORDER.len() - k - 1..].to_vec(),
            "width {}",
            fits - 1.0
        );
        // The dropped labels are not painted; the others are.
        let text = painted_text(&h);
        for (i, label) in [
            "Filter", "Sort", "Hide", "Derive", "Join", "Group", "Delete",
        ]
        .iter()
        .enumerate()
        {
            let dropped = i >= ORDER.len() - k - 1;
            assert_eq!(
                text.iter().any(|t| t == label),
                !dropped,
                "{label} at width {}",
                fits - 1.0
            );
        }
    }
    // Everything dropped and still too narrow: all seven, and no more.
    let h = drawn(Mode::Light, Spec::new(10.0));
    assert_eq!(h.state().icon_only, ORDER.to_vec());
}

#[test]
fn narrower_than_every_icon_and_key_it_clips_at_the_trailing_edge_and_no_two_commands_overlap() {
    for mode in MODES {
        let mut h = drawn(mode, Spec::new(120.0));
        let row = h.state().row;
        assert!(near(row.width(), 120.0), "{row:?}");
        let rects = h.state().rects.clone();
        assert_eq!(rects.len(), 7);
        for (i, (ca, a)) in rects.iter().enumerate() {
            for (cb, b) in &rects[i + 1..] {
                let both = a.intersect(*b);
                assert!(
                    both.width() <= 0.0 || both.height() <= 0.0,
                    "{ca:?} {a:?} overlaps {cb:?} {b:?}"
                );
            }
        }
        let last = rects.last().unwrap().1;
        assert!(last.right() > row.right(), "{last:?} in {row:?}");
        // Nothing is painted outside the band.
        for p in painted(&h, mode) {
            let seen = p.rect.intersect(p.clip);
            assert!(
                seen.width() <= 0.0 || seen.height() <= 0.0 || row.expand(0.5).contains_rect(seen),
                "{mode:?}: {p:?} shows outside {row:?}"
            );
        }
        // A command wholly past the edge takes no input; one inside does.
        press(&mut h, last.center());
        assert_eq!(
            h.state().clicked,
            None,
            "{mode:?}: the clipped part clicked"
        );
        let first = rect_of(&h, at(0, 0));
        assert!(row.contains_rect(first));
        press(&mut h, first.center());
        assert_eq!(h.state().clicked, Some(at(0, 0)));
    }
}

// ─── states and interaction ─────────────────────────────────────────────────

#[test]
fn an_unavailable_command_reports_pointed_unavailable_and_never_clicked() {
    let mut h = drawn(Mode::Light, Spec::new(900.0));
    let join = rect_of(&h, at(1, 1));
    h.get_by_role_and_label(AccessRole::Button, "Join").hover();
    h.run();
    assert_eq!(h.state().pointed, Some((at(1, 1), join)));
    press(&mut h, join.center());
    assert_eq!(h.state().clicked, None);

    let node = h.get_by_role_and_label(AccessRole::Button, "Join");
    assert!(node.accesskit_node().is_disabled());
    assert_eq!(
        node.accesskit_node().description().as_deref(),
        Some("Join needs a second table.")
    );
    let live = h.get_by_role_and_label(AccessRole::Button, "Derive");
    assert!(!live.accesskit_node().is_disabled());
    assert_eq!(live.accesskit_node().description(), None);

    // An available one clicks and reports where.
    let derive = rect_of(&h, at(1, 0));
    press(&mut h, derive.center());
    assert_eq!(h.state().clicked, Some(at(1, 0)));
    assert_eq!(h.state().pointed.map(|p| p.0), Some(at(1, 1)));
}

#[test]
fn an_unavailable_command_takes_no_keyboard_focus() {
    let mut h = drawn(Mode::Light, Spec::new(900.0));
    h.get_by_role_and_label(AccessRole::Button, "Join").focus();
    h.run();
    let join = h.get_by_role_and_label(AccessRole::Button, "Join");
    assert!(!join.accesskit_node().is_focused());
}

#[test]
fn the_row_is_a_toolbar_of_buttons() {
    let h = drawn(Mode::Light, Spec::new(900.0));
    let bar = h.get_by_role(AccessRole::Toolbar);
    let buttons: Vec<_> = bar.get_all_by_role(AccessRole::Button).collect();
    assert_eq!(buttons.len(), 7);
}

// ─── key tips ───────────────────────────────────────────────────────────────

fn key_faces(h: &Harness<'_, Out>, mode: Mode, height: f32) -> Vec<Painted> {
    let sunken = ink(semantic(mode.is_dark()).surfaces.sunken);
    filled(&painted(h, mode), sunken)
        .into_iter()
        .filter(|p| near(p.rect.height(), height))
        .collect()
}

#[test]
fn key_tips_replace_each_command_s_own_key() {
    for mode in MODES {
        let off = drawn(mode, Spec::new(900.0));
        assert_eq!(key_faces(&off, mode, KEYCAP_HEIGHT_IN_CONTROL).len(), 7);
        assert!(key_faces(&off, mode, KEYCAP_HEIGHT).is_empty());

        let mut spec = Spec::new(900.0);
        spec.tips = true;
        let on = drawn(mode, spec);
        assert!(
            key_faces(&on, mode, KEYCAP_HEIGHT_IN_CONTROL).is_empty(),
            "{mode:?}: a command's own key beside its tip"
        );
        let tips = key_faces(&on, mode, KEYCAP_HEIGHT);
        assert_eq!(tips.len(), 7, "{mode:?}");
        // Each tip is at its command's trailing bottom corner; nothing moved.
        for (c, r) in &on.state().rects {
            assert!(near_rect(*r, rect_of(&off, *c)), "{c:?} moved");
            assert!(
                tips.iter().any(|k| {
                    near(k.rect.right(), r.right() + k.rect.width() / 4.0)
                        && near(k.rect.bottom(), r.bottom() + k.rect.height() / 2.0)
                }),
                "{mode:?}: no tip at {c:?} {r:?}"
            );
        }
    }
}

#[test]
fn a_clipped_command_gets_no_key_tip() {
    let mode = Mode::Light;
    let mut spec = Spec::new(120.0);
    spec.tips = true;
    let h = drawn(mode, spec);
    let row = h.state().row;
    let inside = h
        .state()
        .rects
        .iter()
        .filter(|(_, r)| row.contains_rect(*r))
        .count();
    assert!(inside < 7);
    assert_eq!(key_faces(&h, mode, KEYCAP_HEIGHT).len(), inside);
}

// ─── empty forms ────────────────────────────────────────────────────────────

#[test]
fn no_groups_draws_the_empty_band() {
    for mode in MODES {
        let mut spec = Spec::new(300.0);
        spec.groups = [false; 3];
        let h = drawn(mode, spec);
        let sem = semantic(mode.is_dark());
        let row = h.state().row;
        assert!(near(row.height(), BAND_HEIGHT));
        assert!(h.state().rects.is_empty() && h.state().icon_only.is_empty());
        let all = painted(&h, mode);
        assert!(filled(&all, ink(sem.surfaces.raised))
            .iter()
            .any(|p| near_rect(p.rect, row)));
        assert!(
            filled(&all, ink(sem.borders.subtle)).len() == 1,
            "only the foot's rule: {all:?}"
        );
        assert!(painted_text(&h).is_empty());
    }
}

#[test]
fn an_empty_group_draws_no_divider() {
    let mode = Mode::Light;
    let sem = semantic(false);
    let dividers = |h: &Harness<'_, Out>| {
        filled(&painted(h, mode), ink(sem.borders.subtle))
            .into_iter()
            .filter(|p| near(p.rect.width(), 1.0) && near(p.rect.height(), SPACE_6))
            .count()
    };
    // One group alone has no divider; two have one.
    let mut spec = Spec::new(900.0);
    spec.groups = [true, false, false];
    assert_eq!(dividers(&drawn(mode, spec)), 0);
    spec.groups = [true, false, true];
    assert_eq!(dividers(&drawn(mode, spec)), 1);

    // A group with no commands, among others, makes none either.
    let mut harness = Harness::builder()
        .with_size(egui::vec2(700.0, 120.0))
        .build_ui(|ui| {
            theme::apply(ui.ctx(), Mode::Light);
            let one = [Button::new("Filter").icon(&icons::ALL[0])];
            let none: [Button; 0] = [];
            let groups: Vec<&[Button]> = vec![&one, &none, &one];
            let r = ribbon_row(ui, RibbonRow::new(&groups));
            assert_eq!(r.rects.len(), 2);
        });
    harness.run();
    let n = filled(&painted(&harness, mode), ink(sem.borders.subtle))
        .into_iter()
        .filter(|p| near(p.rect.width(), 1.0) && near(p.rect.height(), SPACE_6))
        .count();
    assert_eq!(n, 1, "one divider between the two non-empty groups");
    let _ = Color32::WHITE;
}

#[test]
fn a_command_is_its_quiet_button_with_spacing_3_leading_and_washes_under_the_pointer() {
    for mode in MODES {
        let mut h = drawn(mode, Spec::new(900.0));
        // The same command as a lone quiet button, whose leading padding is
        // SPACE_4: the row's is SPACE_3.
        let mut lone = Harness::builder()
            .with_size(egui::vec2(400.0, 80.0))
            .build_ui(|ui| {
                theme::apply(ui.ctx(), mode);
                let b = Button::new("Sort")
                    .icon(&icons::ALL[0])
                    .keystroke("S")
                    .weight(Weight::Quiet);
                let r = meridian_egui::button(ui, b);
                ui.data_mut(|d| d.insert_temp(egui::Id::new("w"), r.response.rect.width()));
            });
        lone.run();
        let lone_w: f32 = lone
            .ctx
            .data(|d| d.get_temp(egui::Id::new("w")))
            .expect("width");
        let sort = rect_of(&h, at(0, 1));
        assert!(
            near(lone_w - sort.width(), SPACE_4 - SPACE_3),
            "{mode:?}: {lone_w} vs {sort:?}"
        );

        // Under the pointer: the neutral hover wash on exactly its rect.
        h.get_by_role_and_label(AccessRole::Button, "Sort").hover();
        h.run();
        let hover = ink(semantic(mode.is_dark())
            .role(meridian_design::Role::Neutral)
            .background
            .hover);
        let washes = filled(&painted(&h, mode), hover);
        assert!(washes.iter().any(|p| near_rect(p.rect, sort)), "{washes:?}");
    }
}
