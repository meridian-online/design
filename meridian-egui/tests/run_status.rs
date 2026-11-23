//! Run status and the teaching message, read out of the paint list and the
//! accessibility tree: the area's geometry and the bar along its foot, the
//! three cases' marks, words and inks, hover and focus, the degenerate inputs,
//! the click, and the roles.

mod common;

use common::{filled, ink, near, near_rect, painted, Painted};
use egui::accesskit::Role as AccessRole;
use egui::epaint::{CircleShape, PathShape};
use egui::{Color32, Pos2, Rect};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::control::{ICON_SM, KEYCAP_HEIGHT, STEP_MARK, STEP_MARK_DISC_RADIUS};
use meridian_design::radius::CONTROL;
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{ROW_GRID, SPACE_3, SPACE_4};
use meridian_egui::{run_status, teaching_message, theme, Mode, RunStatus, StepMark};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const WIDTH: f32 = 400.0;

#[derive(Default)]
struct Out {
    rect: Option<Rect>,
    clicked: bool,
}

fn drawn(mode: Mode, status: RunStatus<'static>) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(WIDTH + 40.0, 80.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(WIDTH, 40.0), |ui| {
                    let r = run_status(ui, status);
                    out.rect = Some(r.rect);
                    out.clicked |= r.clicked();
                });
            },
            Out::default(),
        );
    harness.run();
    harness
}

fn teaching(mode: Mode) -> Harness<'static, Out> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(WIDTH + 40.0, 80.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(WIDTH, 40.0), |ui| {
                    out.rect = Some(teaching_message(ui, "Fold group", "⌥F").rect);
                });
            },
            Out::default(),
        );
    harness.run();
    harness
}

fn area(h: &Harness<'_, Out>) -> Rect {
    h.state().rect.expect("the status was drawn")
}

const RUNNING: RunStatus<'static> = RunStatus::Running {
    step: Some("join_installs"),
    done: 3,
    total: 7,
};

fn ran(mark: StepMark) -> RunStatus<'static> {
    RunStatus::Ran {
        when: "Ran 22 Nov, 14:02",
        age: "3 hours ago",
        mark,
    }
}

const NOT_RUN: RunStatus<'static> = RunStatus::NotRun {
    label: "Not run yet",
};

fn move_to(h: &mut Harness<'_, Out>, pos: Pos2) {
    h.event(egui::Event::PointerMoved(pos));
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

fn polygons<S>(h: &Harness<'_, S>) -> Vec<PathShape> {
    flat(&h.output().shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Path(p) => Some(p),
            _ => None,
        })
        .collect()
}

struct Txt {
    text: String,
    rect: Rect,
    /// The ink of the first section.
    colour: Color32,
    medium: bool,
}

fn texts<S>(h: &Harness<'_, S>) -> Vec<Txt> {
    flat(&h.output().shapes)
        .into_iter()
        .filter_map(|s| match s {
            egui::Shape::Text(t) => {
                let job = &t.galley.job;
                let medium = matches!(
                    &job.sections[0].format.font_id.family,
                    egui::FontFamily::Name(n) if &**n == "Inter-Medium"
                );
                Some(Txt {
                    text: t.galley.rows.iter().map(|r| r.text()).collect(),
                    rect: t.galley.rect.translate(t.pos.to_vec2()),
                    colour: job.sections[0].format.color,
                    medium,
                })
            }
            _ => None,
        })
        .collect()
}

fn text_of<'a>(all: &'a [Txt], text: &str) -> &'a Txt {
    all.iter().find(|t| t.text == text).unwrap_or_else(|| {
        panic!(
            "no text {text:?} among {:?}",
            all.iter().map(|t| &t.text).collect::<Vec<_>>()
        )
    })
}

/// The mark's centre: `SPACE_4` in, centred in a `ICON_SM` slot.
fn mark_centre(a: Rect) -> Pos2 {
    egui::pos2(a.left() + SPACE_4 + ICON_SM / 2.0, a.center().y)
}

fn track_and_fill(mode: Mode, rects: &[Painted]) -> (Vec<Painted>, Vec<Painted>) {
    let sem = semantic(mode.is_dark());
    (
        filled(rects, ink(sem.feedback.progress_track)),
        filled(rects, ink(sem.feedback.progress_bar)),
    )
}

// ─── geometry ───────────────────────────────────────────────────────────────

#[test]
fn the_area_is_a_grid_row_tall_and_as_wide_as_its_contents() {
    for mode in MODES {
        let h = drawn(mode, RUNNING);
        let a = area(&h);
        assert!(near(a.height(), ROW_GRID), "{a:?}");
        let all = texts(&h);
        let step = text_of(&all, "join_installs");
        let count = text_of(&all, "3 of 7");
        let want = 2.0 * SPACE_4 + ICON_SM + 2.0 * SPACE_3 + step.rect.width() + count.rect.width();
        assert!(near(a.width(), want), "{} vs {want}", a.width());
        assert!(near(
            step.rect.left(),
            a.left() + SPACE_4 + ICON_SM + SPACE_3
        ));
        assert!(near(count.rect.left(), step.rect.right() + SPACE_3));
    }
}

#[test]
fn the_area_is_the_same_width_running_with_and_without_progress() {
    // The bar lies along the foot over the words' own width and adds none: the
    // area is the mark, the step and the count whatever the bar shows.
    for mode in MODES {
        for (done, total) in [(3, 7), (0, 7), (0, 0), (9, 7)] {
            let status = RunStatus::Running {
                step: Some("join_installs"),
                done,
                total,
            };
            let h = drawn(mode, status);
            let a = area(&h);
            let all = texts(&h);
            let count = format!("{done} of {total}");
            let want = 2.0 * SPACE_4
                + ICON_SM
                + 2.0 * SPACE_3
                + text_of(&all, "join_installs").rect.width()
                + text_of(&all, &count).rect.width();
            assert!(
                near(a.width(), want),
                "{done}/{total}: {} vs {want}",
                a.width()
            );
        }
    }
}

// ─── the bar ────────────────────────────────────────────────────────────────

#[test]
fn running_draws_the_bar_along_the_foot_at_done_over_total() {
    for mode in MODES {
        let h = drawn(mode, RUNNING);
        let a = area(&h);
        let rects = painted(&h, mode);
        let (track, fill) = track_and_fill(mode, &rects);
        assert_eq!(track.len(), 1, "{rects:?}");
        assert_eq!(fill.len(), 1, "{rects:?}");
        let want_track =
            Rect::from_min_max(egui::pos2(a.left(), a.bottom() - 2.0), a.right_bottom());
        assert!(near_rect(track[0].rect, want_track), "{:?}", track[0]);
        let want_fill = Rect::from_min_size(want_track.min, egui::vec2(a.width() * 3.0 / 7.0, 2.0));
        assert!(near_rect(fill[0].rect, want_fill), "{:?}", fill[0]);
        common::assert_square(&[track[0], fill[0]], "the bar");
    }
}

#[test]
fn done_past_total_fills_the_bar_and_a_total_of_zero_draws_the_track_alone() {
    for mode in MODES {
        let h = drawn(
            mode,
            RunStatus::Running {
                step: None,
                done: 9,
                total: 7,
            },
        );
        let a = area(&h);
        let (track, fill) = track_and_fill(mode, &painted(&h, mode));
        assert_eq!(track.len(), 1);
        assert_eq!(fill.len(), 1);
        assert!(near(fill[0].rect.width(), a.width()), "{:?}", fill[0]);

        for done in [0, 4] {
            let h = drawn(
                mode,
                RunStatus::Running {
                    step: Some("join_installs"),
                    done,
                    total: 0,
                },
            );
            let (track, fill) = track_and_fill(mode, &painted(&h, mode));
            assert_eq!(track.len(), 1, "done {done}");
            assert!(fill.is_empty(), "done {done}: {fill:?}");
        }
        // Nothing done of some: the track alone.
        let h = drawn(
            mode,
            RunStatus::Running {
                step: None,
                done: 0,
                total: 7,
            },
        );
        let (_, fill) = track_and_fill(mode, &painted(&h, mode));
        assert!(fill.is_empty(), "{fill:?}");
    }
}

// ─── the three cases ────────────────────────────────────────────────────────

#[test]
fn running_draws_its_mark_the_step_in_medium_primary_and_the_count_in_secondary() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, RUNNING);
        let all = texts(&h);
        let step = text_of(&all, "join_installs");
        assert_eq!(step.colour, ink(sem.text.primary));
        assert!(step.medium, "the step is Medium");
        let count = text_of(&all, "3 of 7");
        assert_eq!(count.colour, ink(sem.text.secondary));
        assert!(!count.medium);
        // The half disc's ring is centred in its slot.
        let ring = circles(&h)
            .into_iter()
            .find(|c| c.stroke.color == ink(sem.marks.running))
            .expect("the running mark's ring");
        assert!((ring.center - mark_centre(area(&h))).length() < 0.01);
        assert!(near(ring.radius, STEP_MARK / 2.0));
    }
}

#[test]
fn running_without_a_step_draws_the_mark_and_the_count_alone() {
    let h = drawn(
        Mode::Light,
        RunStatus::Running {
            step: None,
            done: 3,
            total: 7,
        },
    );
    let all = texts(&h);
    assert_eq!(
        all.len(),
        1,
        "{:?}",
        all.iter().map(|t| &t.text).collect::<Vec<_>>()
    );
    assert_eq!(all[0].text, "3 of 7");
}

#[test]
fn ran_draws_the_mark_it_was_given() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, ran(StepMark::Ok));
        let disc = circles(&h)
            .into_iter()
            .find(|c| c.fill == ink(sem.marks.ok))
            .expect("a disc in the ok ink");
        assert!(near(disc.radius, STEP_MARK_DISC_RADIUS));
        assert!((disc.center - mark_centre(area(&h))).length() < 0.01);

        let h = drawn(mode, ran(StepMark::Failed));
        assert!(
            polygons(&h).iter().any(|p| p.fill == ink(sem.marks.failed)),
            "a diamond in the failed ink"
        );
        assert!(circles(&h).iter().all(|c| c.fill != ink(sem.marks.ok)));

        let h = drawn(mode, ran(StepMark::Stale));
        assert!(
            circles(&h)
                .iter()
                .any(|c| c.stroke.color == ink(sem.marks.stale)),
            "the stale ring"
        );
    }
}

#[test]
fn ran_draws_when_in_secondary_age_in_muted_and_no_bar() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, ran(StepMark::Ok));
        let all = texts(&h);
        let when = text_of(&all, "Ran 22 Nov, 14:02");
        let age = text_of(&all, "3 hours ago");
        assert_eq!(when.colour, ink(sem.text.secondary));
        assert_eq!(age.colour, ink(sem.text.muted));
        assert!(!when.medium);
        let a = area(&h);
        assert!(near(
            a.width(),
            2.0 * SPACE_4 + ICON_SM + 2.0 * SPACE_3 + when.rect.width() + age.rect.width()
        ));
        let (track, fill) = track_and_fill(mode, &painted(&h, mode));
        assert!(track.is_empty() && fill.is_empty());
    }
}

#[test]
fn not_run_draws_the_ring_and_the_caller_s_label() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = drawn(mode, NOT_RUN);
        let all = texts(&h);
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].text, "Not run yet");
        assert_eq!(all[0].colour, ink(sem.text.muted));
        let ring = circles(&h)
            .into_iter()
            .find(|c| c.stroke.color == ink(sem.marks.not_run))
            .expect("the not-run ring");
        assert!(ring.fill == Color32::TRANSPARENT);
        let a = area(&h);
        assert!(near(
            a.width(),
            2.0 * SPACE_4 + ICON_SM + SPACE_3 + all[0].rect.width()
        ));
    }
}

// ─── hover and focus ────────────────────────────────────────────────────────

#[test]
fn the_area_fills_under_the_pointer_at_the_control_corner_and_not_at_rest() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let hover = ink(sem.role(Role::Neutral).background.hover);
        let mut h = drawn(mode, RUNNING);
        assert!(filled(&painted(&h, mode), hover).is_empty());
        let a = area(&h);
        move_to(&mut h, a.center());
        let fills = filled(&painted(&h, mode), hover);
        assert_eq!(fills.len(), 1, "{fills:?}");
        assert!(near_rect(fills[0].rect, a), "{:?}", fills[0]);
        common::assert_corner(&fills, CONTROL, "the hover fill");
    }
}

#[test]
fn a_focused_area_is_ringed_at_the_control_corner() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let mut h = drawn(mode, ran(StepMark::Failed));
        let a = area(&h);
        assert!(painted(&h, mode)
            .iter()
            .all(|p| p.stroke.color != ink(sem.borders.focus)));
        h.get_by_role(AccessRole::Status).focus();
        h.run();
        let rects = painted(&h, mode);
        let ring = rects
            .iter()
            .find(|p| p.stroke.color == ink(sem.borders.focus))
            .unwrap_or_else(|| panic!("{mode:?}: no ring in {rects:?}"));
        assert!(near_rect(ring.rect, a), "{ring:?}");
        assert!(near(ring.stroke.width, 2.0));
        let r = meridian_design::focus::ring_radius(CONTROL).round() as u8;
        assert_eq!(ring.radius, egui::CornerRadius::same(r));
    }
}

// ─── the click ──────────────────────────────────────────────────────────────

#[test]
fn a_click_is_reported_in_every_case() {
    for status in [RUNNING, ran(StepMark::Stale), NOT_RUN] {
        let mut h = drawn(Mode::Light, status);
        let pos = area(&h).center();
        move_to(&mut h, pos);
        for pressed in [true, false] {
            h.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            h.run();
        }
        assert!(h.state().clicked, "{status:?}");
    }
}

#[test]
fn enter_and_space_on_a_focused_area_click_it() {
    for key in [egui::Key::Enter, egui::Key::Space] {
        let mut h = drawn(Mode::Light, NOT_RUN);
        h.get_by_role(AccessRole::Status).focus();
        h.run();
        press(&mut h, key);
        assert!(h.state().clicked, "{key:?}");
    }
}

// ─── accessibility ──────────────────────────────────────────────────────────

#[test]
fn a_status_names_its_words_and_state_and_runs_a_progress_bar_child_while_running() {
    let h = drawn(Mode::Light, RUNNING);
    let status = h.get_by_role_and_label(AccessRole::Status, "join_installs, 3 of 7, running");
    let bar = h.get_by_role(AccessRole::ProgressIndicator);
    assert_eq!(
        bar.accesskit_node().parent().expect("a parent").id(),
        status.accesskit_node().id()
    );
    let data = bar.accesskit_node().data().clone();
    assert_eq!(data.numeric_value(), Some(3.0));
    assert_eq!(data.min_numeric_value(), Some(0.0));
    assert_eq!(data.max_numeric_value(), Some(7.0));
}

#[test]
fn a_status_that_is_not_running_has_no_progress_bar() {
    let h = drawn(Mode::Light, ran(StepMark::Stale));
    let _ = h.get_by_role_and_label(AccessRole::Status, "Ran 22 Nov, 14:02, 3 hours ago, stale");
    assert!(h.query_by_role(AccessRole::ProgressIndicator).is_none());
    let h = drawn(Mode::Light, NOT_RUN);
    let _ = h.get_by_role_and_label(AccessRole::Status, "Not run yet, not run");
    assert!(h.query_by_role(AccessRole::ProgressIndicator).is_none());
}

// ─── the teaching message ───────────────────────────────────────────────────

#[test]
fn the_teaching_message_reads_as_one_sentence_to_accesskit() {
    let h = teaching(Mode::Light);
    let _ = h.get_by_role_and_label(AccessRole::Status, "Fold group · next time, press ⌥F");
    // The key is part of the sentence, not a second thing to read.
    assert!(h.query_by_role(AccessRole::Label).is_none());
}

#[test]
fn the_teaching_message_is_the_command_the_copy_and_a_21_key_on_one_line() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let h = teaching(mode);
        let a = area(&h);
        assert!(near(a.height(), ROW_GRID), "{a:?}");
        let all = texts(&h);
        let command = text_of(&all, "Fold group");
        let copy = text_of(&all, " · next time, press");
        assert_eq!(command.colour, ink(sem.text.primary));
        assert_eq!(copy.colour, ink(sem.text.secondary));
        assert!(near(command.rect.center().y, copy.rect.center().y));
        assert!(copy.rect.left() >= command.rect.right() - 0.01);
        let rects = painted(&h, mode);
        let key = filled(&rects, ink(sem.surfaces.sunken));
        assert_eq!(key.len(), 1, "{rects:?}");
        assert!(
            near(key[0].rect.height(), KEYCAP_HEIGHT),
            "{:?}",
            key[0].rect
        );
        assert!(
            near(key[0].rect.left(), copy.rect.right() + SPACE_3),
            "{:?} vs {:?}",
            key[0].rect,
            copy.rect
        );
        assert!(
            near(key[0].rect.right(), a.right()),
            "the area ends at the key"
        );
        assert!(near(key[0].rect.center().y, a.center().y));
        assert!(near(command.rect.left(), a.left()));
    }
}

#[test]
fn the_bar_along_the_foot_takes_neither_hover_nor_click_from_the_area() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let hover = ink(sem.role(Role::Neutral).background.hover);
        let mut h = drawn(mode, RUNNING);
        let a = area(&h);
        let pos = egui::pos2(a.center().x, a.bottom() - 1.0);
        move_to(&mut h, pos);
        assert_eq!(
            filled(&painted(&h, mode), hover).len(),
            1,
            "{mode:?}: hover"
        );
        for pressed in [true, false] {
            h.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            h.run();
        }
        assert!(h.state().clicked, "{mode:?}: click");
    }
}
