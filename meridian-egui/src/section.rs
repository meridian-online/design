//! The section: a labelled, countable, collapsible group.
//!
//! A header row at 24 (a 1px `borders.divider` rule on its top edge, a 14
//! chevron, the title in the label style and, at the trailing edge, an optional
//! count) over a body that is laid out only while the section is open. The
//! header is square and fills the width it is given; hovering it fills it with
//! the accordion hover fill, and keyboard focus rings it.
//!
//! The caller holds whether the section is open ([`Section::new`]). The header
//! is one interactive element: a click, or Space or Enter with focus, reports
//! `toggled`, and the component does not flip anything itself. While closed the
//! body closure is not called (`inner` is `None`); while open it is called with
//! the body laid out inset from the header, filling the width.
//!
//! The title is drawn in capitals whatever the caller passed, and is cut with an
//! ellipsis before the count, so the count keeps its place. Its accessibility
//! node is a `Button` named by the title **as the caller passed it**, not the
//! capitalised form, so a screen reader does not spell capitals; its expanded
//! state is set.
//!
//! Not drawn: the sheet's count uses tabular figures, but egui 0.35's
//! `TextFormat` has no OpenType feature switch, so `typography::TNUM` is not
//! applied and a count's digits keep their proportional widths.

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId, Margin, Rect, Sense, Vec2};
use meridian_design::control::{HEIGHT_MD, ICON_SM};
use meridian_design::semantic;
use meridian_design::typography::UI_SIZE;

use crate::icons::Icon;
use crate::theme::to_color32;
use crate::widgets::{focus_ring, optically_centred_galley_top};
use crate::MeridianUi;

/// One section's configuration. Built with [`Section::new`] and
/// [`Section::count`], then drawn by [`section`].
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct Section<'a> {
    title: &'a str,
    open: bool,
    count: Option<usize>,
}

impl<'a> Section<'a> {
    /// A section titled `title`, open or closed as the caller holds it.
    pub fn new(title: &'a str, open: bool) -> Self {
        Self {
            title,
            open,
            count: None,
        }
    }

    /// The number of things inside, right-aligned in the header.
    pub fn count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }
}

/// What [`section`] hands back.
#[derive(Clone, Debug)]
pub struct SectionResponse<R> {
    /// The header's own response; its `rect` is the header row.
    pub response: egui::Response,
    /// The header was clicked, or activated from the keyboard. The caller flips
    /// `open`.
    pub toggled: bool,
    /// What the body returned. `None` while the section is closed.
    pub inner: Option<R>,
}

/// Draw a section across the width `ui` has left: its header, and its body while
/// `open`.
pub fn section<R>(
    ui: &mut egui::Ui,
    section: Section<'_>,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> SectionResponse<R> {
    // The header and its body touch: the gap between them is the body's own
    // inset, not the caller's item spacing, which the body gets back inside.
    let spacing = ui.spacing().item_spacing;
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        section_in(ui, section, spacing, body)
    })
    .inner
}

fn section_in<R>(
    ui: &mut egui::Ui,
    section: Section<'_>,
    spacing: Vec2,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> SectionResponse<R> {
    let t = ui.tokens();
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, HEIGHT_MD), Sense::click());

    let title = section.title;
    let open = section.open;
    let enabled = ui.is_enabled();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, title));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(open);
    });

    if ui.is_rect_visible(rect) {
        paint_header(ui, rect, &response, &section);
    }

    let inner = open.then(|| {
        let margin = Margin {
            left: t.space[6].round() as i8,
            right: t.space[4].round() as i8,
            top: t.space[2].round() as i8,
            bottom: t.space[5].round() as i8,
        };
        let inner_width = (width - t.space[6] - t.space[4]).max(0.0);
        egui::Frame::new()
            .inner_margin(margin)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = spacing;
                ui.set_width(inner_width);
                body(ui)
            })
            .inner
    });

    SectionResponse {
        toggled: response.clicked(),
        response,
        inner,
    }
}

fn paint_header(ui: &egui::Ui, rect: Rect, response: &egui::Response, section: &Section<'_>) {
    let t = ui.tokens();
    let sem = semantic(ui.visuals().dark_mode);
    let c = to_color32;
    let painter = ui.painter();

    if response.hovered() {
        painter.rect_filled(
            rect,
            meridian_design::radius::NONE,
            c(sem.containers.accordion_hover_background),
        );
    }
    painter.rect_filled(
        Rect::from_min_size(rect.min, egui::vec2(rect.width(), 1.0)),
        meridian_design::radius::NONE,
        c(sem.borders.divider),
    );

    let centre_y = rect.center().y;
    let gap = t.space[2];
    let left = rect.left() + t.space[2];
    let mut right = rect.right() - t.space[4];

    let name = if section.open {
        "chevron-down"
    } else {
        "chevron-right"
    };
    if let Some(chevron) = Icon::by_name(name) {
        let at = Rect::from_center_size(
            egui::pos2(left + ICON_SM / 2.0, centre_y),
            Vec2::splat(ICON_SM),
        );
        chevron.paint(painter, at, c(sem.text.muted));
    }
    let title_left = left + ICON_SM + gap;

    if let Some(n) = section.count {
        let galley = painter.layout_no_wrap(
            n.to_string(),
            FontId::proportional(UI_SIZE),
            c(sem.text.secondary),
        );
        let top = optically_centred_galley_top(&galley, centre_y);
        let x = right - galley.size().x;
        right = x - gap;
        painter.galley(egui::pos2(x, top), galley, Color32::PLACEHOLDER);
    }

    let text = if t.label_uppercase {
        section.title.to_uppercase()
    } else {
        section.title.to_owned()
    };
    let mut job = LayoutJob::single_section(
        text,
        TextFormat {
            font_id: FontId::proportional(t.label_size),
            color: c(sem.text.secondary),
            extra_letter_spacing: t.label_size * t.label_tracking_em,
            ..Default::default()
        },
    );
    job.wrap.max_width = (right - title_left).max(0.0);
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    let galley = painter.layout_job(job);
    let top = optically_centred_galley_top(&galley, centre_y);
    painter.galley(egui::pos2(title_left, top), galley, Color32::PLACEHOLDER);

    // The sheet's focus is an inset ring on a header with no radius: square.
    if response.has_focus() {
        focus_ring(ui, rect, meridian_design::radius::NONE);
    }
}
