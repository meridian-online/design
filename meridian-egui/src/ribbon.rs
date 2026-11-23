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
//! [`ribbon_row`] is the command row beneath it: a 32 band on the raised
//! surface with a 1px rule along its foot, holding groups of quiet buttons
//! divided by a 1px rule. When the row is narrower than its commands it drops
//! their labels from the trailing end, one command at a time, until it fits;
//! the icon and the key remain. Narrower than that, it clips at its trailing
//! edge and what is clipped takes no pointer.
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
use meridian_design::spacing::{BAND_HEIGHT, ROW_GRID};
use meridian_design::{semantic, Role};

use crate::button::{button_size, mark_unavailable, paint_button, Button, ButtonPaint, Weight};
use crate::key_chip::{key_size, paint_key, KeyInk, KeySize};
use crate::theme::to_color32;
use crate::widgets::{
    focus_ring_corners, medium_family, optically_centred_galley_top, paint_open_cell, OpenSide,
};
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
    // A tip floats on a layer of its own, so one is drawn only for a tab that
    // lies wholly inside a strip that is on screen: a tab cut by the strip's
    // edge takes no pointer and gets no tip.
    if tabs.key_tips && ui.is_rect_visible(strip) {
        for c in cells.iter().filter(|c| strip.contains_rect(c.rect)) {
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
            // The rule on the top and sides only: the box is drawn one pixel
            // taller than the cell, so its bottom run falls under the strip's
            // clip.
            paint_open_cell(
                &painter,
                rect,
                top_corners(corner),
                c(sem.surfaces.raised),
                c(sem.borders.subtle),
                OpenSide::Bottom,
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
            painter.rect_filled(rect, top_corners(corner), c(neutral.background.active));
        } else if cell.contextual {
            painter.rect_filled(rect, top_corners(corner), maritime(dark, 3));
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
            let corners = top_corners(ring);
            focus_ring_corners(ui, rect, corners);
        }
    }
}

/// Draw a keycap of the row size (21 tall) at `target`'s trailing bottom
/// corner, offset from it by a quarter of its width and half its height, on a
/// layer above the rest and casting the overlay's hard shadow. It senses
/// nothing and is read as a label carrying the keystroke.
pub fn key_tip(ui: &egui::Ui, target: Rect, keystroke: &str) {
    key_tip_inked(ui, target, keystroke, KeyInk::Plain);
}

/// [`key_tip`] in `ink`: an unavailable command's tip is dimmed.
pub(crate) fn key_tip_inked(ui: &egui::Ui, target: Rect, keystroke: &str, ink: KeyInk) {
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
    let (rect, response) = tip.allocate_exact_size(size, Sense::empty());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, keystroke));
    let shadow = tip.visuals().popup_shadow;
    let chip = CornerRadius::same(radius::CHIP.round() as u8);
    tip.painter().add(shadow.as_shape(rect, chip));
    paint_key(&tip, rect, keystroke, KeySize::Row, ink);
}

/// Which command: its group and its place in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommandAt {
    /// The group's index in the slice passed to [`RibbonRow::new`].
    pub group: usize,
    /// The command's index in its group.
    pub index: usize,
}

/// A ribbon's command row, drawn by [`ribbon_row`].
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct RibbonRow<'a> {
    groups: &'a [&'a [Button<'a>]],
    key_tips: bool,
}

impl<'a> RibbonRow<'a> {
    /// `groups` in order. Every button is drawn in the quiet weight whatever
    /// weight it carries. A group with no commands is not drawn and has no
    /// divider.
    pub fn new(groups: &'a [&'a [Button<'a>]]) -> Self {
        Self {
            groups,
            key_tips: false,
        }
    }

    /// Show each command's key as a tip at its corner instead of the keycap
    /// the command draws itself. Nothing moves when they are on.
    pub fn key_tips(mut self, shown: bool) -> Self {
        self.key_tips = shown;
        self
    }
}

/// What [`ribbon_row`] hands back.
#[derive(Clone, Debug)]
pub struct RibbonRowResponse {
    /// The whole band's response.
    pub response: egui::Response,
    /// The command clicked. Never an unavailable one.
    pub clicked: Option<CommandAt>,
    /// An unavailable command has the pointer over it: which, and its visible
    /// rect, for the caller to anchor the reason popover to.
    pub pointed_unavailable: Option<(CommandAt, Rect)>,
    /// Every command's rect in order, a clipped one's included whole.
    pub rects: Vec<(CommandAt, Rect)>,
    /// The commands whose label was dropped to fit, in order.
    pub icon_only: Vec<CommandAt>,
}

/// One command, laid out.
struct Command<'a> {
    at: CommandAt,
    button: Button<'a>,
    bare: bool,
    rect: Rect,
}

/// The row's contents laid out from the band's left edge: each command's rect
/// and the dividers between non-empty groups.
fn lay_out<'a>(
    ui: &egui::Ui,
    row: &RibbonRow<'a>,
    band: Rect,
    bare: &[bool],
) -> (Vec<Command<'a>>, Vec<Rect>) {
    let t = ui.tokens();
    let mut commands = Vec::new();
    let mut dividers = Vec::new();
    let mut x = band.left() + t.space[4];
    let mut started = false;
    let mut n = 0;
    for (g, group) in row.groups.iter().enumerate() {
        if group.is_empty() {
            continue;
        }
        if started {
            x += t.space[3];
            dividers.push(Rect::from_center_size(
                egui::pos2(x + 0.5, band.center().y),
                Vec2::new(1.0, t.space[6]),
            ));
            x += 1.0 + t.space[3];
        }
        started = true;
        for (i, b) in group.iter().enumerate() {
            if i > 0 {
                x += t.space[1];
            }
            let b = b.weight(Weight::Quiet).with_lead(t.space[3]);
            let is_bare = bare.get(n).copied().unwrap_or(false);
            let size = button_size(ui, &b, is_bare);
            let rect = Rect::from_min_size(egui::pos2(x, band.center().y - size.y / 2.0), size);
            x = rect.right();
            commands.push(Command {
                at: CommandAt { group: g, index: i },
                button: b,
                bare: is_bare,
                rect,
            });
            n += 1;
        }
    }
    (commands, dividers)
}

/// Draw the command row and report what was done to it.
///
/// It fills `ui.available_width()`, so it belongs in a bounded layout: in an
/// unbounded one (a horizontal scroll area) it takes whatever width that
/// reports. It is `BAND_HEIGHT` tall. While its
/// commands are wider than that it drops the label of the last command that
/// still has one (a command with an icon; one without keeps its label), and
/// what still does not fit is clipped at its trailing edge.
pub fn ribbon_row(ui: &mut egui::Ui, row: RibbonRow<'_>) -> RibbonRowResponse {
    let t = ui.tokens();
    let (band, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(0.0), BAND_HEIGHT),
        Sense::hover(),
    );

    // Drop labels from the trailing end until the contents fit.
    let count: usize = row.groups.iter().map(|g| g.len()).sum();
    let mut bare = vec![false; count];
    let (mut commands, mut dividers) = lay_out(ui, &row, band, &bare);
    let fits = |commands: &[Command<'_>]| {
        commands
            .last()
            .is_none_or(|c| c.rect.right() + t.space[4] <= band.right() + 0.01)
    };
    while !fits(&commands) {
        let Some(i) = commands
            .iter()
            .rposition(|c| !c.bare && c.button.has_icon() && !c.button.is_icon_only())
        else {
            break;
        };
        bare[i] = true;
        (commands, dividers) = lay_out(ui, &row, band, &bare);
    }

    let mut list = ui.new_child(UiBuilder::new().max_rect(band).id_salt(response.id));
    list.set_clip_rect(list.clip_rect().intersect(band));
    list.ctx().accesskit_node_builder(list.unique_id(), |node| {
        node.set_role(egui::accesskit::Role::Toolbar);
    });
    let enabled = list.is_enabled();
    let mut clicked = None;
    let mut pointed = None;
    let mut sensed: Vec<egui::Response> = Vec::with_capacity(commands.len());
    for c in &commands {
        let off = c.button.reason().is_some();
        let sense = if off { Sense::hover() } else { Sense::click() };
        let r = list.interact(
            c.rect,
            response.id.with(("ribbon_row", c.at.group, c.at.index)),
            sense,
        );
        let label = c.button.label();
        r.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled && !off, label)
        });
        if let Some(reason) = c.button.reason() {
            mark_unavailable(list.ctx(), r.id, reason);
            if r.hovered() {
                pointed = Some((c.at, c.rect.intersect(list.clip_rect())));
            }
        } else if r.clicked() {
            clicked = Some(c.at);
        }
        sensed.push(r);
    }

    if ui.is_rect_visible(band) {
        let sem = semantic(list.visuals().dark_mode);
        let painter = list.painter();
        painter.rect_filled(band, CornerRadius::ZERO, to_color32(sem.surfaces.raised));
        painter.rect_filled(
            Rect::from_min_max(egui::pos2(band.left(), band.bottom() - 1.0), band.max),
            CornerRadius::ZERO,
            to_color32(sem.borders.subtle),
        );
        for d in &dividers {
            painter.rect_filled(*d, CornerRadius::ZERO, to_color32(sem.borders.subtle));
        }
        for (c, r) in commands.iter().zip(&sensed) {
            paint_button(
                &list,
                c.rect,
                &c.button,
                ButtonPaint {
                    hovered: r.hovered(),
                    pressed: r.is_pointer_button_down_on(),
                    focused: r.has_focus(),
                    label: !c.bare,
                    key: !row.key_tips,
                },
            );
        }
    }
    // A tip floats on a layer of its own, so one is drawn only for a command
    // wholly inside a band that is on screen.
    if row.key_tips && ui.is_rect_visible(band) {
        for c in commands.iter().filter(|c| band.contains_rect(c.rect)) {
            if let Some(k) = c.button.own_key() {
                let ink = if c.button.reason().is_some() {
                    KeyInk::Dimmed
                } else {
                    KeyInk::Plain
                };
                key_tip_inked(ui, c.rect, k, ink);
            }
        }
    }

    RibbonRowResponse {
        response,
        clicked,
        pointed_unavailable: pointed,
        rects: commands.iter().map(|c| (c.at, c.rect)).collect(),
        icon_only: commands.iter().filter(|c| c.bare).map(|c| c.at).collect(),
    }
}
