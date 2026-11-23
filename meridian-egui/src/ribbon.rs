//! The ribbon's tab strip, and the key tip that names a key over a target.
//!
//! [`ribbon_tabs`] is a 24 strip on the title-bar surface with one 1px rule
//! along its foot. A tab is its label with `SPACE_5` either side. The open tab
//! is a cell: the raised surface with a 1px rule on its top and sides, its top
//! corners round and no rule below, so it covers the strip's rule and joins the
//! row beneath it. A contextual tab follows a divider and wears the Maritime
//! wash; open, it is the same cell with a 2px Maritime bar on its top edge. The
//! strip can be collapsed, and then no tab is open and the selected one is
//! marked by its label alone: Medium, in the primary ink.
//!
//! The caller holds which tab is selected and whether the strip is collapsed;
//! nothing is kept between frames. A click on any tab, the selected one
//! included, and a double click are reported. The labels and keystrokes are the
//! caller's strings.
//!
//! [`key_tip`] draws a 21 keycap at a target's trailing bottom corner, on a
//! layer above everything else and casting the overlay's hard shadow. A strip
//! draws one over each tab with a keystroke when its key tips are on, and the
//! tabs do not move when they are.

use std::sync::Arc;

use egui::{
    Color32, CornerRadius, FontId, Galley, Id, LayerId, Order, Rect, Sense, TextStyle, UiBuilder,
    Vec2,
};
use meridian_design::control::TAB_BAR_WIDTH;
use meridian_design::focus;
use meridian_design::radius;
use meridian_design::scales::{MARITIME_DARK, MARITIME_LIGHT};
use meridian_design::spacing::ROW_GRID;
use meridian_design::{semantic, Role};

use crate::key_chip::{key_size, paint_key, KeyInk, KeySize};
use crate::theme::to_color32;
use crate::widgets::{focus_ring_corners, medium_family, optically_centred_galley_top};
use crate::MeridianUi;

/// One tab: a label and, optionally, the keystroke that opens it.
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct RibbonTab<'a> {
    label: &'a str,
    keystroke: Option<&'a str>,
}

impl<'a> RibbonTab<'a> {
    /// A tab labelled `label`.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            keystroke: None,
        }
    }

    /// The keystroke that opens this tab, shown as a key tip when the strip's
    /// key tips are on.
    pub fn keystroke(mut self, keystroke: &'a str) -> Self {
        self.keystroke = Some(keystroke);
        self
    }
}

/// Which tab: a stage by its index, or the contextual one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TabPick {
    /// The stage at this index.
    Stage(usize),
    /// The contextual tab.
    Contextual,
}

/// A ribbon's tab strip, drawn by [`ribbon_tabs`].
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct RibbonTabs<'a> {
    stages: &'a [RibbonTab<'a>],
    contextual: Option<RibbonTab<'a>>,
    selected: Option<TabPick>,
    collapsed: bool,
    key_tips: bool,
}

impl<'a> RibbonTabs<'a> {
    /// `stages` in order, with `selected` selected. A pick that names no tab
    /// selects nothing.
    pub fn new(stages: &'a [RibbonTab<'a>], selected: Option<TabPick>) -> Self {
        Self {
            stages,
            contextual: None,
            selected,
            collapsed: false,
            key_tips: false,
        }
    }

    /// A contextual tab, set apart after the stages.
    pub fn contextual(mut self, tab: RibbonTab<'a>) -> Self {
        self.contextual = Some(tab);
        self
    }

    /// Collapsed: the selected tab is marked but none is open.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Show a key tip over each tab that has a keystroke.
    pub fn key_tips(mut self, shown: bool) -> Self {
        self.key_tips = shown;
        self
    }
}

/// What [`ribbon_tabs`] hands back.
#[derive(Clone, Debug)]
pub struct RibbonTabsResponse {
    /// The whole strip's response.
    pub response: egui::Response,
    /// The tab clicked, the selected one included.
    pub clicked: Option<TabPick>,
    /// The tab double-clicked.
    pub double_clicked: Option<TabPick>,
    /// Every tab's rect, the stages in order and then the contextual one.
    pub rects: Vec<(TabPick, Rect)>,
}

/// One tab, measured and ready to paint.
struct Cell<'a> {
    pick: TabPick,
    tab: RibbonTab<'a>,
    galley: Arc<Galley>,
    rect: Rect,
    selected: bool,
    open: bool,
    contextual: bool,
}

/// The top corners of an open cell round; the bottom ones never do.
fn top_corners(radius: u8) -> CornerRadius {
    CornerRadius {
        nw: radius,
        ne: radius,
        sw: 0,
        se: 0,
    }
}

/// A Maritime step, counted from 1 as the scale is named.
fn maritime(dark: bool, step: usize) -> Color32 {
    to_color32(if dark {
        MARITIME_DARK[step - 1]
    } else {
        MARITIME_LIGHT[step - 1]
    })
}

/// Draw the strip and report what was done to it.
///
/// It is as wide as the room `ui` has left, 24 tall. Tabs past its trailing
/// edge are clipped and take no pointer.
pub fn ribbon_tabs(ui: &mut egui::Ui, tabs: RibbonTabs<'_>) -> RibbonTabsResponse {
    let t = ui.tokens();
    let (strip, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(0.0), ROW_GRID),
        Sense::hover(),
    );
    let side = t.space[4];
    let pad = t.space[5];
    let regular = TextStyle::Button.resolve(ui.style());
    let medium = FontId::new(regular.size, medium_family(ui));

    // Lay the tabs out: the stages, then a divider and the contextual tab.
    let mut x = strip.left() + side;
    let mut cells: Vec<Cell<'_>> = Vec::new();
    let mut divider: Option<Rect> = None;
    let items = tabs
        .stages
        .iter()
        .enumerate()
        .map(|(i, tab)| (TabPick::Stage(i), *tab, false))
        .chain(tabs.contextual.map(|c| (TabPick::Contextual, c, true)));
    for (pick, tab, contextual) in items {
        if contextual && !tabs.stages.is_empty() {
            let d = Rect::from_min_max(
                egui::pos2(x + side, strip.top() + t.space[2]),
                egui::pos2(x + side + 1.0, strip.bottom() - t.space[2]),
            );
            divider = Some(d);
            x = d.right() + side;
        }
        let selected = tabs.selected == Some(pick);
        let font = if selected {
            medium.clone()
        } else {
            regular.clone()
        };
        let galley = ui
            .painter()
            .layout_no_wrap(tab.label.to_owned(), font, Color32::PLACEHOLDER);
        let rect = Rect::from_min_size(
            egui::pos2(x, strip.top()),
            Vec2::new(galley.size().x + 2.0 * pad, strip.height()),
        );
        x = rect.right();
        cells.push(Cell {
            pick,
            tab,
            galley,
            rect,
            selected,
            open: selected && !tabs.collapsed,
            contextual,
        });
    }

    // The tabs sit in a child `Ui` whose accessibility node is the tab list. A
    // strip narrower than its tabs cuts them at its edges, and what is cut
    // takes no pointer: the clip goes on before the tabs sense.
    let mut list = ui.new_child(UiBuilder::new().max_rect(strip).id_salt(response.id));
    list.set_clip_rect(list.clip_rect().intersect(strip));
    list.ctx().accesskit_node_builder(list.unique_id(), |node| {
        node.set_role(egui::accesskit::Role::TabList);
    });
    let enabled = list.is_enabled();
    let mut clicked = None;
    let mut double_clicked = None;
    let mut sensed: Vec<egui::Response> = Vec::with_capacity(cells.len());
    for (i, c) in cells.iter().enumerate() {
        let r = list.interact(c.rect, response.id.with(("ribbon_tab", i)), Sense::click());
        if r.clicked() {
            clicked = Some(c.pick);
        }
        if r.double_clicked() {
            double_clicked = Some(c.pick);
        }
        // egui has no tab widget type: it describes the tab as a button and
        // the role is put right after, the label and click action staying.
        r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, c.tab.label));
        list.ctx().accesskit_node_builder(r.id, |node| {
            node.set_role(egui::accesskit::Role::Tab);
            node.set_selected(c.selected);
        });
        sensed.push(r);
    }

    if ui.is_rect_visible(strip) {
        paint(&list, strip, &cells, &sensed, divider);
    }
    if tabs.key_tips {
        for c in &cells {
            if let Some(k) = c.tab.keystroke {
                key_tip(ui, c.rect, k);
            }
        }
    }

    RibbonTabsResponse {
        response,
        clicked,
        double_clicked,
        rects: cells.iter().map(|c| (c.pick, c.rect)).collect(),
    }
}

fn paint(
    ui: &egui::Ui,
    strip: Rect,
    cells: &[Cell<'_>],
    sensed: &[egui::Response],
    divider: Option<Rect>,
) {
    let t = ui.tokens();
    let dark = ui.visuals().dark_mode;
    let sem = semantic(dark);
    let c = to_color32;
    let neutral = sem.role(Role::Neutral);
    let painter = ui.painter_at(strip);
    let corner = t.radius_control.round() as u8;
    let none = radius::NONE.round() as u8;
    debug_assert_eq!(none, 0);

    // The strip, then its rule along the foot. An open cell paints over the
    // rule below it.
    painter.rect_filled(
        strip,
        CornerRadius::ZERO,
        c(sem.containers.title_bar_background),
    );
    painter.rect_filled(
        Rect::from_min_max(egui::pos2(strip.left(), strip.bottom() - 1.0), strip.max),
        CornerRadius::ZERO,
        c(sem.borders.subtle),
    );
    if let Some(d) = divider {
        painter.rect_filled(d, CornerRadius::ZERO, c(sem.borders.subtle));
    }

    for (cell, r) in cells.iter().zip(sensed) {
        let rect = cell.rect;
        let pressed = r.is_pointer_button_down_on();
        if cell.open {
            painter.rect_filled(rect, top_corners(corner), c(sem.surfaces.raised));
            // The rule on the top and sides only: the box is drawn one pixel
            // taller than the cell, so its bottom run falls under the strip's
            // clip.
            painter.rect_stroke(
                Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.bottom() + 1.0)),
                top_corners(corner),
                egui::Stroke::new(1.0, c(sem.borders.subtle)),
                egui::StrokeKind::Inside,
            );
            if cell.contextual {
                let top = rect.top() + 1.0;
                painter.rect_filled(
                    Rect::from_min_max(
                        egui::pos2(rect.left() + 1.0, top),
                        egui::pos2(rect.right() - 1.0, top + TAB_BAR_WIDTH),
                    ),
                    CornerRadius::ZERO,
                    maritime(dark, 9),
                );
            }
        } else if pressed {
            painter.rect_filled(rect, CornerRadius::ZERO, c(neutral.background.active));
        } else if cell.contextual {
            painter.rect_filled(rect, CornerRadius::ZERO, maritime(dark, 3));
        }

        // The ink: a contextual tab keeps its Maritime one when pointed at.
        let ink = if cell.contextual {
            maritime(dark, if cell.selected { 12 } else { 11 })
        } else if cell.selected || r.hovered() {
            c(sem.text.primary)
        } else {
            c(sem.text.secondary)
        };
        // The label sits in the box inside the 1px rule on the top.
        let centre_y = rect.top() + (rect.height() + 1.0) / 2.0;
        let top = optically_centred_galley_top(&cell.galley, centre_y);
        painter.galley_with_override_text_color(
            egui::pos2(rect.left() + t.space[5], top),
            cell.galley.clone(),
            ink,
        );

        if r.has_focus() {
            let ring = focus::ring_radius(t.radius_control).round() as u8;
            let corners = if cell.open {
                top_corners(ring)
            } else {
                CornerRadius::ZERO
            };
            focus_ring_corners(ui, rect, corners);
        }
    }
}

/// Draw a keycap of the row size (21 tall) at `target`'s trailing bottom
/// corner, offset from it by a quarter of its width and half its height, on a
/// layer above the rest and casting the overlay's hard shadow. It senses
/// nothing and is read as a label carrying the keystroke.
pub fn key_tip(ui: &egui::Ui, target: Rect, keystroke: &str) {
    let size = key_size(ui, keystroke, KeySize::Row);
    let corner = target.right_bottom() + Vec2::new(size.x / 4.0, size.y / 2.0);
    let at = corner - size;
    let id = Id::new((
        "meridian_key_tip",
        keystroke,
        at.x.to_bits(),
        at.y.to_bits(),
    ));
    // A child `Ui` on a foreground layer of its own rather than an `Area`,
    // which would snap its place to a whole pixel and move the key off the
    // quarter and the half the corner is offset by.
    let layer = LayerId::new(Order::Foreground, id);
    let ctx = ui.ctx();
    let mut tip = egui::Ui::new(
        ctx.clone(),
        id,
        UiBuilder::new()
            .layer_id(layer)
            .max_rect(Rect::from_min_size(at, size)),
    );
    tip.set_clip_rect(ctx.content_rect());
    let (rect, response) = tip.allocate_exact_size(size, Sense::hover());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, keystroke));
    let shadow = tip.visuals().popup_shadow;
    let chip = CornerRadius::same(radius::CHIP.round() as u8);
    tip.painter().add(shadow.as_shape(rect, chip));
    paint_key(&tip, rect, keystroke, KeySize::Row, KeyInk::Plain);
}
