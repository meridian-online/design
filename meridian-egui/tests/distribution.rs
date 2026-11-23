//! The distribution bar, read out of the paint list: bar widths, heights, gaps,
//! inks and the hovered index.

mod common;

use common::{assert_square, ink, near, painted, Painted};
use egui::Rect;
use egui_kittest::Harness;
use meridian_design::chrome::{INK_DARK, INK_LIGHT};
use meridian_design::spacing::{ROW_DENSE, ROW_PREVIEW, SPACE_1, SPACE_2};
use meridian_design::viz::{
    CATEGORICAL_DARK, CATEGORICAL_LIGHT, NULL_INK_DARK, NULL_INK_LIGHT, SEQUENTIAL_MERIDIAN,
};
use meridian_egui::{distribution_bar, theme, Bars, DistributionBar, Mode};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];
const WIDTH: f32 = 240.0;

struct Out {
    rect: Rect,
    hovered: Option<usize>,
}

#[derive(Clone, Copy)]
struct Spec {
    bars: Bars<'static>,
    missing: u64,
    total: u64,
    dense: bool,
}

fn spec(bars: Bars<'static>, missing: u64) -> Spec {
    Spec {
        bars,
        missing,
        total: 100,
        dense: false,
    }
}

fn drawn(mode: Mode, spec: Spec, hover: Option<egui::Pos2>) -> (Vec<Painted>, Out) {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(WIDTH + 40.0, 100.0))
        .build_ui_state(
            move |ui, out: &mut Out| {
                theme::apply(ui.ctx(), mode);
                ui.allocate_ui(egui::vec2(WIDTH, 60.0), |ui| {
                    let mut b = DistributionBar::new(spec.bars, spec.missing, spec.total);
                    if spec.dense {
                        b = b.dense();
                    }
                    let r = distribution_bar(ui, b);
                    out.rect = r.response.rect;
                    out.hovered = r.hovered;
                });
            },
            Out {
                rect: Rect::NOTHING,
                hovered: None,
            },
        );
    harness.run();
    if let Some(p) = hover {
        harness.hover_at(p);
        harness.run();
    }
    let p = painted(&harness, mode);
    let out = Out {
        rect: harness.state().rect,
        hovered: harness.state().hovered,
    };
    (p, out)
}

fn baseline(mode: Mode) -> egui::Color32 {
    ink(if mode == Mode::Dark {
        INK_DARK
    } else {
        INK_LIGHT
    }
    .baseline)
}
fn cat(mode: Mode) -> egui::Color32 {
    ink(if mode == Mode::Dark {
        CATEGORICAL_DARK
    } else {
        CATEGORICAL_LIGHT
    }[0])
}
fn null(mode: Mode) -> egui::Color32 {
    ink(if mode == Mode::Dark {
        NULL_INK_DARK
    } else {
        NULL_INK_LIGHT
    })
}
fn muted(mode: Mode) -> egui::Color32 {
    ink(if mode == Mode::Dark {
        INK_DARK
    } else {
        INK_LIGHT
    }
    .ink_muted)
}
fn seq600() -> egui::Color32 {
    ink(SEQUENTIAL_MERIDIAN[10])
}

/// The bars, left to right, without the baseline rule.
fn bars(p: &[Painted], mode: Mode) -> Vec<Painted> {
    let mut v: Vec<Painted> = p
        .iter()
        .filter(|r| r.fill != baseline(mode))
        .copied()
        .collect();
    v.sort_by(|a, b| a.rect.left().total_cmp(&b.rect.left()));
    v
}

#[test]
fn bars_are_equal_in_width_and_proportional_in_height() {
    for mode in MODES {
        let (p, out) = drawn(mode, spec(Bars::Bins(&[100, 50, 25, 10]), 0), None);
        let b = bars(&p, mode);
        assert_eq!(b.len(), 4, "{mode:?}");
        assert!(near(out.rect.width(), WIDTH) && near(out.rect.height(), ROW_PREVIEW));
        let w = b[0].rect.width();
        // Four bins and the missing bar's slot (kept even at zero): 3 hairlines
        // and the SPACE_2 gap before the missing bar.
        let want = (WIDTH - 3.0 - SPACE_2) / 5.0;
        assert!(near(w, want), "{mode:?}: {w} vs {want}");
        for (i, r) in b.iter().enumerate() {
            assert!(near(r.rect.width(), w), "{mode:?} bar {i}");
            assert_eq!(r.fill, cat(mode));
            assert!(near(r.rect.bottom(), out.rect.bottom()), "{mode:?}");
            if i > 0 {
                assert!(near(r.rect.left() - b[i - 1].rect.right(), 1.0), "{mode:?}");
            }
        }
        for (r, count) in b.iter().zip([100.0, 50.0, 25.0, 10.0]) {
            assert!(
                near(r.rect.height(), ROW_PREVIEW * count / 100.0),
                "{mode:?}"
            );
        }
        // The baseline: one rule along the foot, the box's width.
        let rule = common::filled(&p, baseline(mode));
        assert_eq!(rule.len(), 1, "{mode:?}");
        assert!(near(rule[0].rect.height(), 1.0));
        assert!(near(rule[0].rect.width(), WIDTH));
        assert!(near(rule[0].rect.bottom(), out.rect.bottom()));
    }
}

#[test]
fn a_non_zero_count_is_at_least_two_tall_and_a_zero_count_draws_nothing() {
    for mode in MODES {
        let (p, _) = drawn(mode, spec(Bars::Bins(&[1000, 1, 0, 1000]), 0), None);
        let b = bars(&p, mode);
        // The zero draws nothing, and its slot stays: the last bar is where it
        // would be with four.
        assert_eq!(b.len(), 3, "{mode:?}");
        assert!(
            near(b[1].rect.height(), SPACE_1),
            "{mode:?}: {:?}",
            b[1].rect
        );
        let slot = (WIDTH - 3.0 - SPACE_2) / 5.0 + 1.0;
        assert!(
            near(b[2].rect.left() - b[0].rect.left(), 3.0 * slot),
            "{mode:?}"
        );
    }
}

#[test]
fn the_rest_and_the_missing_bars_follow_a_gap_and_are_the_last_two_indices() {
    for mode in MODES {
        let top = Bars::Top {
            values: &[("a", 100), ("b", 60)],
            rest: 30,
        };
        let (p, out) = drawn(mode, spec(top, 20), None);
        let b = bars(&p, mode);
        assert_eq!(b.len(), 4, "{mode:?}");
        assert_eq!(b[2].fill, muted(mode));
        assert_eq!(b[3].fill, null(mode));
        assert!(
            near(b[2].rect.left() - b[1].rect.right(), SPACE_2),
            "{mode:?}"
        );
        assert!(
            near(b[3].rect.left() - b[2].rect.right(), SPACE_2),
            "{mode:?}"
        );
        assert!(near(b[1].rect.left() - b[0].rect.right(), 1.0), "{mode:?}");
        // The null bar is proportional to the same largest count.
        assert!(near(b[3].rect.height(), ROW_PREVIEW * 0.2), "{mode:?}");
        assert!(near(b[2].rect.height(), ROW_PREVIEW * 0.3), "{mode:?}");
        assert!(near(b[3].rect.right(), out.rect.right()), "{mode:?}");

        // Hover reports those indices: values 0 and 1, the rest 2, missing 3.
        for (i, bar) in b.iter().enumerate() {
            let at = egui::pos2(bar.rect.center().x, out.rect.bottom() - 1.0);
            let (_, o) = drawn(mode, spec(top, 20), Some(at));
            assert_eq!(o.hovered, Some(i), "{mode:?}");
        }

        // Bins end on the missing bar alone.
        let (p, _) = drawn(mode, spec(Bars::Bins(&[10, 5]), 5), None);
        let b = bars(&p, mode);
        assert_eq!(b.len(), 3);
        assert_eq!(b[2].fill, null(mode));
        assert!(near(b[2].rect.left() - b[1].rect.right(), SPACE_2));
    }
}

#[test]
fn hovering_a_bar_reports_its_index_and_darkens_only_it() {
    for mode in MODES {
        let s = spec(Bars::Bins(&[100, 80, 60]), 10);
        let (rest, out) = drawn(mode, s, None);
        assert_eq!(out.hovered, None);
        assert!(common::filled(&rest, seq600()).is_empty(), "{mode:?}");
        let b = bars(&rest, mode);
        let at = egui::pos2(b[1].rect.center().x, out.rect.bottom() - 1.0);
        let (p, out) = drawn(mode, s, Some(at));
        assert_eq!(out.hovered, Some(1), "{mode:?}");
        let lit = common::filled(&p, seq600());
        assert_eq!(lit.len(), 1, "{mode:?}");
        assert!(near(lit[0].rect.left(), b[1].rect.left()), "{mode:?}");
        assert_eq!(common::filled(&p, cat(mode)).len(), 2, "{mode:?}");
    }
}

#[test]
fn bars_are_square() {
    for mode in MODES {
        let top = Bars::Top {
            values: &[("a", 9)],
            rest: 3,
        };
        let (p, _) = drawn(mode, spec(top, 2), None);
        assert_square(&p, "the distribution bar");
    }
}

#[test]
fn no_bins_draws_the_baseline_alone() {
    for mode in MODES {
        let (p, _) = drawn(mode, spec(Bars::Bins(&[]), 0), None);
        assert_eq!(p.len(), 1, "{mode:?}");
        assert_eq!(p[0].fill, baseline(mode));
        let (p, _) = drawn(
            mode,
            spec(
                Bars::Top {
                    values: &[],
                    rest: 0,
                },
                0,
            ),
            None,
        );
        assert_eq!(p.len(), 1, "{mode:?}");
    }
}

#[test]
fn a_total_of_zero_does_not_divide() {
    for mode in MODES {
        let mut s = spec(Bars::Bins(&[0, 0, 0]), 0);
        s.total = 0;
        let (p, _) = drawn(mode, s, None);
        assert_eq!(p.len(), 1, "{mode:?}: the baseline alone");
        // Counts above a total of zero still draw, against the largest count.
        let mut s = spec(Bars::Bins(&[4, 2]), 0);
        s.total = 0;
        let (p, _) = drawn(mode, s, None);
        let b = bars(&p, mode);
        assert_eq!(b.len(), 2);
        assert!(b.iter().all(|r| r.rect.height().is_finite()));
        assert!(near(b[0].rect.height(), ROW_PREVIEW));
    }
}

#[test]
fn dense_is_twenty_tall() {
    for mode in MODES {
        let mut s = spec(Bars::Bins(&[8, 4]), 0);
        s.dense = true;
        let (p, out) = drawn(mode, s, None);
        assert!(near(out.rect.height(), ROW_DENSE), "{mode:?}");
        assert!(near(bars(&p, mode)[0].rect.height(), ROW_DENSE), "{mode:?}");
    }
}
