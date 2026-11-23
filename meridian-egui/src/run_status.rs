//! Run status: the title band's activity area, and the teaching message.
//!
//! [`run_status`] is a 24 tall area, as wide as its contents, with `SPACE_4`
//! before its first part and after its last and `SPACE_3` between parts. Its
//! contents are the state mark in an `ICON_SM` slot and then, by case:
//!
//! - running: the step in Medium primary (when given), then `"{done} of
//!   {total}"` in secondary. Along its foot lies a `TAB_BAR_WIDTH` bar the
//!   area's full width, in the progress track ink with the progress bar ink
//!   over `done / total` of it, square. The bar is laid over the words and
//!   claims no width of its own, so the area is the same width whether or not
//!   a run has progress to show, and does not jump when a run starts;
//! - ran: `when` in secondary and `age` in muted, with the mark the caller
//!   gives (fresh, stale or failed);
//! - not run: the ring and the caller's `label` in muted.
//!
//! `done` past `total` fills the bar, and a `total` of zero draws the track
//! alone. Every word is the caller's. The area is clickable: under the pointer
//! it takes the neutral hover fill at the control corner, and with keyboard
//! focus the ring at the same corner. Its accessibility node is a `Status`
//! named by its words and the mark's state word, holding a `ProgressIndicator`
//! child while running (accesskit's name for what ARIA calls a
//! progress bar).
//!
//! [`teaching_message`] is the one piece of copy a component owns: the command
//! in primary, `" · next time, press"` in secondary, `SPACE_3`, and the
//! keystroke on a 21 key, on one line in a 24 row.
//!
//! Not drawn: the sheet's count uses tabular figures, but egui 0.35's
//! `TextFormat` has no OpenType feature switch, so a count's digits keep their
//! proportional widths and the area's width moves by the digit-width difference
//! as the count advances.

use egui::text::LayoutJob;
use egui::{Color32, CornerRadius, FontId, Rect, Sense, TextFormat, UiBuilder, Vec2};
use meridian_design::control::{ICON_SM, TAB_BAR_WIDTH};
use meridian_design::radius;
use meridian_design::spacing::ROW_GRID;
use meridian_design::typography::UI_SIZE;
use meridian_design::{semantic, Role};

pub use meridian_design::marks::StepMark;

use crate::key_chip::{key_size, paint_key, KeyInk, KeySize};
use crate::step_row::paint_mark;
use crate::theme::to_color32;
use crate::widgets::{cap_centred_galley_top, focus_ring, medium_family, mode_of};
use crate::MeridianUi;
use meridian_design::marks::word;

/// What the activity area says. Every string is the caller's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStatus<'a> {
    /// A run is under way: the step running, if known, and how many steps are
    /// done of how many.
    Running {
        step: Option<&'a str>,
        done: usize,
        total: usize,
    },
    /// The Protocol last ran: when, how long ago, and the state mark (fresh,
    /// stale or failed).
    Ran {
        when: &'a str,
        age: &'a str,
        mark: StepMark,
    },
    /// Before any run: the caller's words, such as "Not run yet".
    NotRun { label: &'a str },
}

/// One part of the area's words.
struct Part<'a> {
    text: &'a str,
    ink: Color32,
    medium: bool,
}

fn layout(ui: &egui::Ui, part: &Part<'_>) -> std::sync::Arc<egui::Galley> {
    let family = if part.medium {
        medium_family(ui)
    } else {
        egui::FontFamily::Proportional
    };
    let mut job = LayoutJob::single_section(
        part.text.to_owned(),
        TextFormat::simple(FontId::new(UI_SIZE, family), part.ink),
    );
    job.wrap.max_rows = 1;
    job.wrap.max_width = f32::INFINITY;
    ui.painter().layout_job(job)
}

/// Draw the activity area and report its clicks through the response.
pub fn run_status(ui: &mut egui::Ui, status: RunStatus<'_>) -> egui::Response {
    let t = ui.tokens();
    let mode = mode_of(ui);
    let sem = semantic(mode.is_dark());
    let c = to_color32;

    let count;
    let (mark, parts, progress): (StepMark, Vec<Part<'_>>, Option<(usize, usize)>) = match status {
        RunStatus::Running { step, done, total } => {
            count = format!("{done} of {total}");
            let mut parts = Vec::new();
            if let Some(step) = step {
                parts.push(Part {
                    text: step,
                    ink: c(sem.text.primary),
                    medium: true,
                });
            }
            parts.push(Part {
                text: &count,
                ink: c(sem.text.secondary),
                medium: false,
            });
            (StepMark::Running, parts, Some((done, total)))
        }
        RunStatus::Ran { when, age, mark } => (
            mark,
            vec![
                Part {
                    text: when,
                    ink: c(sem.text.secondary),
                    medium: false,
                },
                Part {
                    text: age,
                    ink: c(sem.text.muted),
                    medium: false,
                },
            ],
            None,
        ),
        RunStatus::NotRun { label } => (
            StepMark::NotRun,
            vec![Part {
                text: label,
                ink: c(sem.text.muted),
                medium: false,
            }],
            None,
        ),
    };
    let galleys: Vec<_> = parts.iter().map(|p| layout(ui, p)).collect();

    // The width is what is always there: the mark and the words. The bar is
    // laid over it.
    let gap = t.space[3];
    let words: f32 = galleys.iter().map(|g| g.size().x + gap).sum();
    let width = 2.0 * t.space[4] + ICON_SM + words;
    let (rect, slot) = ui.allocate_exact_size(Vec2::new(width, ROW_GRID), Sense::hover());

    // The area is a child `Ui` whose own widget is the status, so the
    // progress node is the status's child.
    let mut area = ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .id_salt(slot.id)
            .sense(Sense::click()),
    );
    area.advance_cursor_after_rect(rect);
    let mut response = area.response();
    response.rect = rect;

    let mut label = parts.iter().map(|p| p.text).collect::<Vec<_>>().join(", ");
    label.push_str(", ");
    label.push_str(word(mark));
    let enabled = area.is_enabled();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, enabled, &label));
    area.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
    });

    let bar_rect = Rect::from_min_max(
        egui::pos2(rect.left(), rect.bottom() - TAB_BAR_WIDTH),
        rect.right_bottom(),
    );
    if let Some((done, total)) = progress {
        let bar = area.interact(bar_rect, response.id.with("progress"), Sense::hover());
        bar.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::ProgressIndicator, enabled, "")
        });
        area.ctx().accesskit_node_builder(bar.id, |node| {
            node.set_role(egui::accesskit::Role::ProgressIndicator);
            node.set_min_numeric_value(0.0);
            node.set_max_numeric_value(total as f64);
            node.set_numeric_value(done.min(total) as f64);
        });
    }

    if area.is_rect_visible(rect) {
        let painter = area.painter_at(rect);
        let corner = CornerRadius::same(t.radius_control.round() as u8);
        if response.hovered() {
            painter.rect_filled(rect, corner, c(sem.role(Role::Neutral).background.hover));
        }
        let centre_y = rect.center().y;
        paint_mark(
            &painter,
            egui::pos2(rect.left() + t.space[4] + ICON_SM / 2.0, centre_y),
            mark,
            mode,
        );
        let mut x = rect.left() + t.space[4] + ICON_SM + gap;
        for galley in galleys {
            let top = cap_centred_galley_top(ui.painter(), &galley, centre_y);
            let w = galley.size().x;
            painter.galley(egui::pos2(x, top), galley, Color32::PLACEHOLDER);
            x += w + gap;
        }
        if let Some((done, total)) = progress {
            painter.rect_filled(bar_rect, radius::NONE, c(sem.feedback.progress_track));
            // A total of zero has nothing to be a fraction of; done past total
            // is all of it.
            if total > 0 && done > 0 {
                let fraction = (done as f32 / total as f32).min(1.0);
                painter.rect_filled(
                    Rect::from_min_size(
                        bar_rect.min,
                        Vec2::new(bar_rect.width() * fraction, bar_rect.height()),
                    ),
                    radius::NONE,
                    c(sem.feedback.progress_bar),
                );
            }
        }
        if response.has_focus() {
            focus_ring(&area, rect, t.radius_control);
        }
    }
    response
}

/// The sentence's own words, after the command.
const TEACHING_COPY: &str = " · next time, press";

/// `<command> · next time, press [key]`: the one piece of copy a component
/// owns. The command is in primary, the sentence's own words in secondary,
/// then `SPACE_3` and the keystroke on a 21 key, on one line in a 24 row. Its
/// accessibility node is a `Status` named by the whole sentence; the key is
/// drawn into it and is not a second node.
pub fn teaching_message(ui: &mut egui::Ui, command: &str, keystroke: &str) -> egui::Response {
    let t = ui.tokens();
    let sem = semantic(mode_of(ui).is_dark());
    let c = to_color32;
    let primary = Part {
        text: command,
        ink: c(sem.text.primary),
        medium: false,
    };
    let copy = Part {
        text: TEACHING_COPY,
        ink: c(sem.text.secondary),
        medium: false,
    };
    let (first, second) = (layout(ui, &primary), layout(ui, &copy));
    let key = key_size(ui, keystroke, KeySize::Row);
    let gap = t.space[3];
    let width = first.size().x + second.size().x + gap + key.x;
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, ROW_GRID), Sense::hover());

    let sentence = format!("{command}{TEACHING_COPY} {keystroke}");
    let enabled = ui.is_enabled();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, enabled, &sentence));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
    });

    if ui.is_rect_visible(rect) {
        let centre_y = rect.center().y;
        let painter = ui.painter_at(rect);
        let mut x = rect.left();
        for galley in [first, second] {
            let top = cap_centred_galley_top(ui.painter(), &galley, centre_y);
            let w = galley.size().x;
            painter.galley(egui::pos2(x, top), galley, Color32::PLACEHOLDER);
            x += w;
        }
        let key_rect = Rect::from_min_size(egui::pos2(x + gap, centre_y - key.y / 2.0), key);
        paint_key(ui, key_rect, keystroke, KeySize::Row, KeyInk::Plain);
    }
    response
}
