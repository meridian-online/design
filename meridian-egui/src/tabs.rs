//! The one line-tab strip.
//!
//! Anywhere a surface names its panels, its panes or its lists in a row and one
//! of them is open, it draws that row through [`line_tabs`]. The strip is the
//! words in a row over one rule, and the open one is marked by a
//! [`TAB_BAR_WIDTH`] bar under it, over the rule. A closed tab is not a
//! button: there is no fill, no border and no corner, so nothing about it needs
//! a look of its own — only which word is open.
//!
//! The function takes no colour and no width. The words are in the text inks,
//! the rule is the divider ink and the bar is [`semantic::Tabs::active_bar`],
//! so a hand-rolled tab background cannot be expressed through this API at
//! all. The one ink a caller may pass is the bar's ([`LineTabs::bar_ink`]): a
//! surface that marks a kind of data by hue puts it there and nowhere else.
//!
//! Each word is a tab in the accessibility tree, inside one tab list, and the
//! open one reads selected. Moving between the tabs by the arrow keys is the
//! caller's: this strip reports a click and nothing else.

use std::sync::Arc;

use egui::{Color32, CornerRadius, Galley, Rect, Sense, TextStyle, UiBuilder};
use meridian_design::control::TAB_BAR_WIDTH;
use meridian_design::semantic;

use crate::theme::to_color32;
use crate::widgets::optically_centred_galley_top;
use crate::MeridianUi;

/// The thickness of the strip's one rule. The bar under the open tab is
/// [`TAB_BAR_WIDTH`] and is drawn over the bottom of this rule, so the strip's
/// height does not depend on which tab is open.
const STRIP_RULE: f32 = 1.0;

/// Configuration for one [`line_tabs`] strip: the names, which one is open and,
/// for a caller that marks a kind of data with its bar, the bar's ink. Nothing
/// else is configurable — treatment is the tokens' job.
#[derive(Clone, Copy, Debug)]
pub struct LineTabs<'a> {
    names: &'a [&'a str],
    open: usize,
    bar_ink: Option<Color32>,
}

impl<'a> LineTabs<'a> {
    /// A strip of `names` with the tab at index `open` open. An index past the
    /// end leaves every tab closed.
    #[must_use]
    pub fn new(names: &'a [&'a str], open: usize) -> Self {
        Self {
            names,
            open,
            bar_ink: None,
        }
    }

    /// The ink of the bar under the open tab, for a caller that marks a kind of
    /// data with it. Left unset, the bar is in the token's ink
    /// ([`semantic::Tabs::active_bar`], the focus ink). It is the bar's ink
    /// alone: the words and the rule keep theirs.
    #[must_use]
    pub fn bar_ink(mut self, ink: Color32) -> Self {
        self.bar_ink = Some(ink);
        self
    }
}

/// What [`line_tabs`] hands back.
pub struct LineTabsResponse {
    /// The strip's own response, for a caller that needs its rect — to lay
    /// something beside the words, say. It senses hover only: a click on a tab
    /// is reported in [`Self::clicked`].
    pub response: egui::Response,
    /// The index of the closed tab that was clicked this frame. A click on the
    /// open tab reports none: it is already where the caller is.
    pub clicked: Option<usize>,
}

/// One word of the strip, measured before anything is painted.
struct Word {
    galley: Arc<Galley>,
    cell: Rect,
}

/// Draw the strip: allocate the full available width at the ladder's grid row
/// height, lay the names out left to right, and paint them from the semantic
/// tokens.
///
/// A strip has one rule and one bar and no corner. The rule is [`STRIP_RULE`]
/// thick in the divider ink along the bottom of the strip. Each word is in the
/// secondary ink, except the open one, which is in the primary ink and has a
/// [`TAB_BAR_WIDTH`] bar in [`semantic::Tabs::active_bar`] under it, over the
/// rule — unless the caller passes an ink with [`LineTabs::bar_ink`]. Each word
/// is padded by the spacing ladder on both sides, and is placed on the glyphs
/// it inks rather than on the font box they were laid out in
/// ([`optically_centred_galley_top`]), above the bar.
///
/// Each word is a tab node in the accessibility tree and the strip is the one
/// tab-list node they sit inside; the open word reads selected.
pub fn line_tabs(ui: &mut egui::Ui, tabs: LineTabs<'_>) -> LineTabsResponse {
    let t = ui.tokens();
    let sem = semantic(ui.visuals().dark_mode);

    let desired = egui::vec2(ui.available_width(), t.rows[1]);
    let (strip, response) = ui.allocate_exact_size(desired, Sense::hover());

    // The strip is one element, so the tabs have one thing to be inside.
    let list = ui.new_child(UiBuilder::new().max_rect(strip));
    ui.ctx().accesskit_node_builder(list.unique_id(), |node| {
        node.set_role(egui::accesskit::Role::TabList);
    });

    let font = TextStyle::Button.resolve(ui.style());
    let pad = t.space[4];
    let word_ink = |i: usize| {
        to_color32(if i == tabs.open {
            sem.text.primary
        } else {
            sem.text.secondary
        })
    };
    let mut x = strip.left();
    let words: Vec<Word> = tabs
        .names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let galley = ui
                .painter()
                .layout_no_wrap((*name).to_owned(), font.clone(), word_ink(i));
            let width = galley.size().x + 2.0 * pad;
            let cell = Rect::from_min_size(
                egui::pos2(x, strip.top()),
                egui::vec2(width, strip.height()),
            );
            x = cell.right();
            Word { galley, cell }
        })
        .collect();

    if ui.is_rect_visible(strip) {
        let painter = ui.painter_at(strip);
        let rule = Rect::from_min_max(
            egui::pos2(strip.left(), strip.bottom() - STRIP_RULE),
            strip.max,
        );
        painter.rect_filled(rule, CornerRadius::ZERO, to_color32(sem.borders.divider));

        // The words sit above the bar, so the open tab's bar is not a line
        // through its own text.
        let centre_y = strip.top() + (strip.height() - TAB_BAR_WIDTH) / 2.0;
        for (i, word) in words.iter().enumerate() {
            if i == tabs.open {
                let bar = Rect::from_min_max(
                    egui::pos2(word.cell.left(), strip.bottom() - TAB_BAR_WIDTH),
                    word.cell.right_bottom(),
                );
                let ink = tabs
                    .bar_ink
                    .unwrap_or_else(|| to_color32(sem.tabs.active_bar));
                painter.rect_filled(bar, CornerRadius::ZERO, ink);
            }
            let top = optically_centred_galley_top(&word.galley, centre_y);
            painter.galley(
                egui::pos2(word.cell.left() + pad, top),
                word.galley.clone(),
                word_ink(i),
            );
        }
    }

    let enabled = ui.is_enabled();
    let mut clicked = None;
    for (i, (word, name)) in words.iter().zip(tabs.names).enumerate() {
        let open = i == tabs.open;
        let tab = list.interact(word.cell, response.id.with(i), Sense::click());
        if tab.clicked() && !open {
            clicked = Some(i);
        }
        // egui has no tab widget type, so it describes the word as a button and
        // the role is put right after: the label and the click action stay its.
        tab.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, *name));
        list.ctx().accesskit_node_builder(tab.id, |node| {
            node.set_role(egui::accesskit::Role::Tab);
            node.set_selected(open);
        });
    }

    LineTabsResponse { response, clicked }
}
