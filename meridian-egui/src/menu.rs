//! The menu item and the menu separator.
//!
//! A menu is a [`crate::Popover::show_menu`] whose body is [`menu_item`]s with a
//! [`menu_separator`] between groups; there is no menu component. An item is
//! the list row at 24 ([`crate::list_row`]) and takes its three states and the
//! cursor's bar from it: a check column when [`MenuItem::checked`] was called,
//! the label, a right-aligned 21 keycap and, for a submenu, a chevron. It fills
//! the width it is given, and a label too long for it is cut with an ellipsis
//! while the key keeps its place.
//!
//! The caller holds everything: where the cursor is ([`MenuItem::cursor`]),
//! whether a submenu is open. The item reports that it was clicked, and a
//! submenu's item reports its rect (`response.rect`) so the caller can open a
//! second popover beside it.
//!
//! An unavailable item ([`MenuItem::unavailable`]) is greyed with its key
//! dimmed and keeps its place. It still takes the cursor, so its reason can be
//! read from the keys; it senses the pointer, reports no click and reports its
//! rect when pointed at, and its accessibility node is disabled with the reason
//! as its description. It takes no hover or pressed fill and no keyboard focus:
//! a menu is walked with the caller's cursor.

use egui::{Color32, Rect, Sense, Vec2};
use meridian_design::control::ICON_SM;
use meridian_design::semantic;

use crate::button::label_galley;
use crate::icons::Icon;
use crate::key_chip::{key_size, paint_key, KeyInk, KeySize};
use crate::list_row::{list_row, ListRow, RowHeight};
use crate::theme::to_color32;
use crate::widgets::optically_centred_galley_top;
use crate::MeridianUi;

/// One menu item's configuration. Built with [`MenuItem::new`] and the
/// `#[must_use]` builders, then drawn by [`menu_item`].
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct MenuItem<'a> {
    label: &'a str,
    keystroke: Option<&'a str>,
    reason: Option<&'a str>,
    checked: Option<bool>,
    submenu: bool,
    cursor: bool,
}

impl<'a> MenuItem<'a> {
    /// An item labelled `label`.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            keystroke: None,
            reason: None,
            checked: None,
            submenu: false,
            cursor: false,
        }
    }

    /// The keystroke that does what this item does, as the caller's string,
    /// drawn as a keycap at the trailing edge.
    pub fn keystroke(mut self, keystroke: &'a str) -> Self {
        self.keystroke = Some(keystroke);
        self
    }

    /// The item cannot be used now, for `reason` (the caller's sentence).
    pub fn unavailable(mut self, reason: &'a str) -> Self {
        self.reason = Some(reason);
        self
    }

    /// The item is a choice that is on or off. Any call reserves the 14 check
    /// column, so a menu's labels line up; `on` draws the check.
    pub fn checked(mut self, on: bool) -> Self {
        self.checked = Some(on);
        self
    }

    /// The item opens a submenu: a chevron at the trailing edge. It reports
    /// that it was clicked, and the caller opens a popover beside its rect.
    pub fn submenu(mut self) -> Self {
        self.submenu = true;
        self
    }

    /// The item is under the caller's cursor: where the keys act. It takes the
    /// cursor fill and bar, unavailable or not.
    pub fn cursor(mut self, cursor: bool) -> Self {
        self.cursor = cursor;
        self
    }
}

/// What [`menu_item`] hands back.
#[derive(Clone, Debug)]
pub struct MenuItemResponse {
    /// The item's own response; its `rect` is the whole row.
    pub response: egui::Response,
    /// The item was clicked. Always false for an unavailable one.
    pub clicked: bool,
    /// The item is unavailable and the pointer is over it: its rect, for the
    /// caller to anchor the reason popover to. `None` otherwise.
    pub pointed_unavailable: Option<Rect>,
}

/// Draw a menu item across the width `ui` has left and report what was done to
/// it.
pub fn menu_item(ui: &mut egui::Ui, item: MenuItem<'_>) -> MenuItemResponse {
    let off = item.reason.is_some();
    let mut row = ListRow::new(RowHeight::Grid).selected(item.cursor);
    if off {
        row = row.inert();
    }
    let lead = ui.tokens().space[4];
    let r = list_row(ui, row, |ui, _state| paint_item(ui, &item, lead));

    let label = item.label;
    let response = r.response;
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, !off, label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::MenuItem);
        if let Some(reason) = item.reason {
            node.set_disabled();
            node.set_description(reason);
        }
    });

    MenuItemResponse {
        clicked: !off && response.clicked(),
        pointed_unavailable: (off && response.hovered()).then_some(response.rect),
        response,
    }
}

/// Lay the content out across the row: the check column, then the label, with
/// the key and the chevron claimed from the trailing edge first so the label is
/// cut to the room they leave.
fn paint_item(ui: &mut egui::Ui, item: &MenuItem<'_>, lead: f32) {
    let t = ui.tokens();
    let sem = semantic(ui.visuals().dark_mode);
    let off = item.reason.is_some();
    let c = to_color32;
    let (label_ink, check_ink, chevron_ink, key_ink) = if off {
        let d = c(sem.text.disabled);
        (d, d, d, KeyInk::Dimmed)
    } else {
        (
            c(sem.text.primary),
            c(sem.text.primary),
            c(sem.text.muted),
            KeyInk::Plain,
        )
    };

    // The list row's content box is its rect less `lead` each side; the item's
    // trailing padding is `SPACE_2`.
    let content = ui.max_rect();
    let centre_y = content.center().y;
    let gap = t.icon_label_gap;
    let mut left = content.left();
    let mut right = content.right() + lead - t.space[2];
    let painter = ui.painter().clone();
    let icon_rect = |x: f32| {
        Rect::from_center_size(
            egui::pos2(x + ICON_SM / 2.0, centre_y),
            Vec2::splat(ICON_SM),
        )
    };

    if item.submenu {
        if let Some(chevron) = Icon::by_name("chevron-right") {
            chevron.paint(&painter, icon_rect(right - ICON_SM), chevron_ink);
        }
        right -= ICON_SM + gap;
    }
    if let Some(k) = item.keystroke {
        let size = key_size(ui, k, KeySize::Row);
        let rect = Rect::from_center_size(egui::pos2(right - size.x / 2.0, centre_y), size);
        paint_key(ui, rect, k, KeySize::Row, key_ink);
        right -= size.x + gap;
    }
    if let Some(on) = item.checked {
        if on {
            if let Some(check) = Icon::by_name("check") {
                check.paint(&painter, icon_rect(left), check_ink);
            }
        }
        left += ICON_SM + gap;
    }
    let galley = label_galley(ui, item.label, label_ink, (right - left).max(0.0));
    let top = optically_centred_galley_top(&galley, centre_y);
    painter.galley(egui::pos2(left, top), galley, Color32::PLACEHOLDER);
}

/// The hairline between a menu's groups: a 1px `borders.divider` rule across the
/// width `ui` has left, with `SPACE_2` above and below.
pub fn menu_separator(ui: &mut egui::Ui) {
    let t = ui.tokens();
    let sem = semantic(ui.visuals().dark_mode);
    let pad = t.space[2];
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 1.0 + 2.0 * pad),
        Sense::hover(),
    );
    if ui.is_rect_visible(rect) {
        let rule = Rect::from_min_size(
            egui::pos2(rect.left(), rect.top() + pad),
            egui::vec2(rect.width(), 1.0),
        );
        ui.painter().rect_filled(
            rule,
            meridian_design::radius::NONE,
            to_color32(sem.borders.divider),
        );
    }
}
