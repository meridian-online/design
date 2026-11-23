//! The distribution bar: a column's values drawn in chart ink.
//!
//! Bars stand on a 1px `baseline` rule along the foot of a box that fills the
//! width it is given and is 36 tall, or 20 for a column header
//! ([`DistributionBar::dense`]). Every bar is the same width with 1px between,
//! its height proportional to the largest count shown (the missing count
//! included), at least `SPACE_1` tall when its count is above zero, and square.
//! A bar of zero draws nothing but keeps its slot, so the bars after it stay
//! where they were.
//!
//! [`Bars::Bins`] draws equal bins low to high in the first categorical ink.
//! [`Bars::Top`] draws a category's top values in the same ink, a `SPACE_2` gap,
//! then the rest as one bar in the muted ink. Both forms end with a `SPACE_2`
//! gap and the missing count in the null ink.
//!
//! The bar under the pointer is the sequential ramp's 600 step, and its index is
//! reported: the bins or values in order, then (for `Top`) the rest, then the
//! missing bar, so the rest and missing are always the last two indices of a
//! `Top` and the missing bar the last of `Bins`. The component holds nothing
//! between frames; the caller writes the line a hovered bar earns.
//!
//! The values' names in [`Bars::Top`] are the caller's, for its own readout; the
//! bar draws no text.

use egui::{Rect, Sense};
use meridian_design::chrome::{INK_DARK, INK_LIGHT};
use meridian_design::colour::Rgba;
use meridian_design::spacing::{ROW_DENSE, ROW_PREVIEW, SPACE_1, SPACE_2};
use meridian_design::viz::{
    CATEGORICAL_DARK, CATEGORICAL_LIGHT, NULL_INK_DARK, NULL_INK_LIGHT, SEQUENTIAL_MERIDIAN,
};

use crate::theme::to_color32;

/// The sheet's 1px between bars, and the baseline's thickness.
const HAIRLINE: f32 = 1.0;
/// The sequential ramp's 600 step: the ramp runs 100 to 700 in steps of 50.
const HOVER_STEP: usize = (600 - 100) / 50;

/// What the bars show.
#[derive(Clone, Copy, Debug)]
pub enum Bars<'a> {
    /// Equal bins of a number, low to high: each bin's count.
    Bins(&'a [u64]),
    /// A category's top values, name and count, with every other value's rows
    /// as one `rest` bar.
    Top {
        values: &'a [(&'a str, u64)],
        rest: u64,
    },
}

/// One distribution bar's configuration. Built with [`DistributionBar::new`] and
/// [`DistributionBar::dense`], then drawn by [`distribution_bar`].
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct DistributionBar<'a> {
    bars: Bars<'a>,
    missing: u64,
    dense: bool,
}

impl<'a> DistributionBar<'a> {
    /// The bars, then the `missing` count. `total` is the column's row count; the
    /// bars are scaled against the largest count shown rather than against it,
    /// so it is accepted for the caller's readout and never divided by.
    pub fn new(bars: Bars<'a>, missing: u64, _total: u64) -> Self {
        Self {
            bars,
            missing,
            dense: false,
        }
    }

    /// 20 tall, for a column header, rather than 36.
    pub fn dense(mut self) -> Self {
        self.dense = true;
        self
    }
}

/// What [`distribution_bar`] hands back.
#[derive(Clone, Debug)]
pub struct DistributionResponse {
    /// The whole bar's response; its `rect` is the box.
    pub response: egui::Response,
    /// The index of the bar under the pointer, if any. A bar of zero is still
    /// under the pointer where its slot is.
    pub hovered: Option<usize>,
}

#[derive(Clone, Copy, PartialEq)]
enum Ink {
    Value,
    Rest,
    Null,
}

/// One slot: its count, its ink, and whether a `SPACE_2` gap comes before it.
struct Slot {
    count: u64,
    ink: Ink,
    gap_before: bool,
}

fn slots(bar: &DistributionBar<'_>) -> Vec<Slot> {
    let mut out = Vec::new();
    let mut push = |count, ink, gap_before| {
        out.push(Slot {
            count,
            ink,
            gap_before,
        })
    };
    match bar.bars {
        Bars::Bins(bins) => bins.iter().for_each(|&c| push(c, Ink::Value, false)),
        Bars::Top { values, rest } => {
            values.iter().for_each(|&(_, c)| push(c, Ink::Value, false));
            push(rest, Ink::Rest, !values.is_empty());
        }
    }
    let gap = !out.is_empty();
    out.push(Slot {
        count: bar.missing,
        ink: Ink::Null,
        gap_before: gap,
    });
    out
}

/// The slot a pointer at `x` is over: each slot owns half of the gap on either
/// side of it, so every x inside the box belongs to exactly one slot.
fn slot_at(columns: &[(f32, f32)], x: f32) -> usize {
    columns
        .iter()
        .enumerate()
        .position(|(i, c)| {
            let gap = columns.get(i + 1).map_or(f32::INFINITY, |n| n.0 - c.1);
            x < c.1 + gap / 2.0
        })
        .unwrap_or(columns.len() - 1)
}

/// Draw a distribution bar across the width `ui` has left. Heights scale
/// against the largest count shown, the rest and the missing counts included.
pub fn distribution_bar(ui: &mut egui::Ui, bar: DistributionBar<'_>) -> DistributionResponse {
    let height = if bar.dense { ROW_DENSE } else { ROW_PREVIEW };
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());

    let slots = slots(&bar);
    let columns = columns(rect, &slots);
    let hovered = response.hover_pos().map(|p| slot_at(&columns, p.x));

    if ui.is_rect_visible(rect) {
        paint(ui, rect, &slots, &columns, hovered);
    }
    DistributionResponse { response, hovered }
}

/// Each slot's left and right edge. The bars share what the gaps leave.
fn columns(rect: Rect, slots: &[Slot]) -> Vec<(f32, f32)> {
    let gaps: f32 = slots
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                0.0
            } else if s.gap_before {
                SPACE_2
            } else {
                HAIRLINE
            }
        })
        .sum();
    let each = ((rect.width() - gaps) / slots.len() as f32).max(0.0);
    let mut x = rect.left();
    slots
        .iter()
        .enumerate()
        .map(|(i, s)| {
            if i > 0 {
                x += if s.gap_before { SPACE_2 } else { HAIRLINE };
            }
            let col = (x, x + each);
            x += each;
            col
        })
        .collect()
}

fn paint(
    ui: &egui::Ui,
    rect: Rect,
    slots: &[Slot],
    columns: &[(f32, f32)],
    hovered: Option<usize>,
) {
    let dark = ui.visuals().dark_mode;
    let chrome = if dark { INK_DARK } else { INK_LIGHT };
    let painter = ui.painter();
    let square = meridian_design::radius::NONE;

    painter.rect_filled(
        Rect::from_min_max(
            egui::pos2(rect.left(), rect.bottom() - HAIRLINE),
            rect.right_bottom(),
        ),
        square,
        to_color32(chrome.baseline),
    );

    let largest = slots.iter().map(|s| s.count).max().unwrap_or(0);
    if largest == 0 {
        return;
    }
    let (cat, null): (Rgba, Rgba) = if dark {
        (CATEGORICAL_DARK[0], NULL_INK_DARK)
    } else {
        (CATEGORICAL_LIGHT[0], NULL_INK_LIGHT)
    };
    for (i, (slot, &(left, right))) in slots.iter().zip(columns).enumerate() {
        if slot.count == 0 {
            continue;
        }
        let h = (rect.height() * slot.count as f32 / largest as f32).max(SPACE_1);
        let ink = if hovered == Some(i) {
            SEQUENTIAL_MERIDIAN[HOVER_STEP]
        } else {
            match slot.ink {
                Ink::Value => cat,
                Ink::Rest => chrome.ink_muted,
                Ink::Null => null,
            }
        };
        painter.rect_filled(
            Rect::from_min_max(
                egui::pos2(left, rect.bottom() - h),
                egui::pos2(right, rect.bottom()),
            ),
            square,
            to_color32(ink),
        );
    }
}
