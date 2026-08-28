//! One `Picker`, N delegates.
//!
//! Every "choose a thing" moment renders through one widget so it looks and
//! behaves identically without anyone policing it: a [`Picker`] owns the
//! query string, the selection, and the keyboard contract (arrows move,
//! enter confirms, escape dismisses), and a [`PickerDelegate`] owns
//! everything domain-shaped — the corpus, the match/filter/rank step, what a
//! row says, and what confirming means. The pattern is reimplemented from an
//! understanding of the delegate-driven pickers common in editor tooling,
//! shaped here for immediate mode (ADR 0011 records the licence firewall
//! around reading such patterns).
//!
//! The delegate hands back row *content* ([`PickerRow`]), never row
//! *treatment*: the picker draws every row through [`crate::list_row`], so a
//! delegate cannot introduce its own row colours or heights even on purpose.
//!
//! The trait is deliberately wide enough for the delegate shapes an
//! application actually has, without this module implementing any of them:
//!
//! - a **command palette**: query + ranked corpus, [`PickerOutcome::Close`]
//!   on confirm;
//! - a **typed argument prompt**: zero matches by design
//!   ([`PickerDelegate::empty_text`] returns `None`), the typed query is the
//!   value — [`PickerDelegate::confirm`] receives `index: None` plus the raw
//!   query, and returns [`PickerOutcome::KeepOpen`] while invalid, narrating
//!   through [`PickerDelegate::hint`];
//! - **flat jump lists**: the minimal surface — filter, rows, confirm;
//! - a **grouped read-only sheet**: [`PickerDelegate::header_before`] for
//!   group headers and [`PickerDelegate::confirmable`]` = false`, making
//!   enter another way out.

use std::sync::Arc;

use egui::text::{Galley, LayoutJob, TextFormat, TextWrapping};
use egui::{Align, Color32, FontSelection, Key, Layout, Modifiers, RichText, UiBuilder};
use meridian_design::semantic;

use crate::key_chip::{chip_width, key_chip};
use crate::list_row::{list_row, ListRow, RowHeight};
use crate::query::query_line;
use crate::theme::to_color32;
use crate::MeridianUi;

/// One row's content, as data. The picker turns this into a themed
/// [`crate::list_row`]; a delegate never draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PickerRow {
    /// Primary text.
    pub label: String,
    /// Supporting annotation, muted, inline after the label.
    pub detail: Option<String>,
    /// A keystroke chip, right-aligned — a caller-provided string (the
    /// binding registry stays application-side).
    pub keystroke: Option<String>,
}

impl PickerRow {
    /// A row with just a label.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            detail: None,
            keystroke: None,
        }
    }

    /// Add muted supporting annotation.
    #[must_use]
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Add a right-aligned keystroke chip.
    #[must_use]
    pub fn keystroke(mut self, keystroke: impl Into<String>) -> Self {
        self.keystroke = Some(keystroke.into());
        self
    }
}

/// A line the delegate shows under the query — validation feedback for
/// typed-value prompts, guidance for anything else.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PickerHint {
    /// Neutral guidance, muted ink.
    Info(String),
    /// The current query is not acceptable, error ink.
    Error(String),
}

/// What the delegate wants after a confirm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickerOutcome {
    /// Done — the host closes the picker.
    Close,
    /// Stay open (an invalid typed value, a multi-step collection).
    KeepOpen,
}

/// What the picker reports to its host this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickerEvent {
    /// A row (or the typed query) was confirmed and the delegate closed.
    Confirmed,
    /// Escape — the host closes the picker without confirming.
    Dismissed,
}

/// Per-frame response from [`Picker::show`].
#[derive(Clone, Copy, Debug, Default)]
pub struct PickerResponse {
    /// Confirmed / dismissed this frame, if either.
    pub event: Option<PickerEvent>,
    /// The query string changed this frame (the delegate has already been
    /// re-queried).
    pub query_changed: bool,
}

/// The domain half of a picker. Object-safe, so hosts can store delegates as
/// `Box<dyn PickerDelegate>` behind one modal slot — the forwarding impl
/// below makes the boxed form a delegate itself, so
/// `Picker<Box<dyn PickerDelegate>>` compiles.
pub trait PickerDelegate {
    /// The query changed (also called once with the empty query when the
    /// picker is created) — refilter, rerank, revalidate.
    fn update_query(&mut self, query: &str);

    /// How many rows the current query matches.
    fn match_count(&self) -> usize;

    /// Content for match `index` (`0..match_count()`).
    fn row(&self, index: usize) -> PickerRow;

    /// The user confirmed. `index` is the selected match, or `None` when
    /// there are no matches — in which case `query` is the typed value.
    fn confirm(&mut self, index: Option<usize>, query: &str) -> PickerOutcome;

    /// Placeholder for the empty query line.
    fn placeholder(&self) -> String {
        "Search…".to_owned()
    }

    /// A group header to draw before match `index`, for sheets that read in
    /// sections. Flat delegates keep the default.
    fn header_before(&self, _index: usize) -> Option<String> {
        None
    }

    /// A line under the query: guidance or validation feedback.
    fn hint(&self) -> Option<PickerHint> {
        None
    }

    /// Whether enter means anything here. Read-only sheets return `false`,
    /// which turns enter into a dismissal.
    fn confirmable(&self) -> bool {
        true
    }

    /// What to say when nothing matches. `None` suppresses the empty state
    /// entirely — the typed-value prompt shape, where "no matches" is the
    /// normal condition, not an outcome to narrate.
    fn empty_text(&self) -> Option<String> {
        Some("No matches".to_owned())
    }

    /// The picker is going away without a confirm.
    fn dismiss(&mut self) {}
}

/// The forwarding impl the one-modal-slot pattern rests on:
/// `Picker<Box<dyn PickerDelegate>>` needs `Box<dyn PickerDelegate>` to be a
/// delegate itself. Every method forwards — the defaulted ones deliberately
/// included, because leaving any of them to the trait default would silently
/// disconnect an inner delegate's `placeholder` / `header_before` / `hint` /
/// `confirmable` / `empty_text` / `dismiss` override behind the box.
impl<D: PickerDelegate + ?Sized> PickerDelegate for Box<D> {
    fn update_query(&mut self, query: &str) {
        (**self).update_query(query);
    }

    fn match_count(&self) -> usize {
        (**self).match_count()
    }

    fn row(&self, index: usize) -> PickerRow {
        (**self).row(index)
    }

    fn confirm(&mut self, index: Option<usize>, query: &str) -> PickerOutcome {
        (**self).confirm(index, query)
    }

    fn placeholder(&self) -> String {
        (**self).placeholder()
    }

    fn header_before(&self, index: usize) -> Option<String> {
        (**self).header_before(index)
    }

    fn hint(&self) -> Option<PickerHint> {
        (**self).hint()
    }

    fn confirmable(&self) -> bool {
        (**self).confirmable()
    }

    fn empty_text(&self) -> Option<String> {
        (**self).empty_text()
    }

    fn dismiss(&mut self) {
        (**self).dismiss();
    }
}

/// The chrome half: query line, hint, match list, selection, keyboard.
/// Generic over its delegate; hosts that juggle several picker kinds through
/// one slot can use `Picker<Box<dyn PickerDelegate>>`.
pub struct Picker<D: PickerDelegate> {
    /// The domain half, reachable for inspection after events.
    pub delegate: D,
    query: String,
    selected: usize,
    row_height: RowHeight,
    request_focus: bool,
    scroll_to_selected: bool,
}

impl<D: PickerDelegate> Picker<D> {
    /// Wrap a delegate. Primes it with the empty query so `match_count` is
    /// meaningful before the first frame, and focuses the query line on the
    /// first show.
    pub fn new(mut delegate: D) -> Self {
        delegate.update_query("");
        Self {
            delegate,
            query: String::new(),
            selected: 0,
            row_height: RowHeight::Grid,
            request_focus: true,
            scroll_to_selected: false,
        }
    }

    /// Pick a taller (or shorter) row rung than the dense default.
    #[must_use]
    pub fn with_row_height(mut self, row_height: RowHeight) -> Self {
        self.row_height = row_height;
        self
    }

    /// The current query string.
    #[must_use]
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Replace the query programmatically (pre-seeding a prompt). Re-queries
    /// the delegate and resets the selection.
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.delegate.update_query(&self.query);
        self.selected = 0;
        self.scroll_to_selected = true;
    }

    /// The selected match index (meaningful while `match_count() > 0`).
    #[must_use]
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Move the selection by `delta`, wrapping at both ends.
    pub fn move_selection(&mut self, delta: isize) {
        let count = self.delegate.match_count();
        if count == 0 {
            self.selected = 0;
            return;
        }
        let count_i = count as isize;
        let current = self.selected.min(count - 1) as isize;
        self.selected = (current + delta).rem_euclid(count_i) as usize;
        self.scroll_to_selected = true;
    }

    fn confirm_current(&mut self) -> Option<PickerEvent> {
        if !self.delegate.confirmable() {
            self.delegate.dismiss();
            return Some(PickerEvent::Dismissed);
        }
        let count = self.delegate.match_count();
        let index = if count == 0 {
            None
        } else {
            Some(self.selected.min(count - 1))
        };
        match self.delegate.confirm(index, &self.query) {
            PickerOutcome::Close => Some(PickerEvent::Confirmed),
            PickerOutcome::KeepOpen => None,
        }
    }

    /// Draw the picker into `ui` and run one frame of its keyboard contract:
    /// arrows move the selection, enter confirms, escape dismisses. The
    /// arrow/enter/escape keys are consumed so the query line's text edit
    /// never sees them; inside a [`crate::ModalLayer`] this means the picker
    /// reports the escape (as [`PickerEvent::Dismissed`]) rather than the
    /// modal — a host should treat either signal as "close".
    pub fn show(&mut self, ui: &mut egui::Ui) -> PickerResponse {
        let mut out = PickerResponse::default();

        let count = self.delegate.match_count();
        if count > 0 {
            self.selected = self.selected.min(count - 1);
        } else {
            self.selected = 0;
        }

        let (down, up, enter, escape) = ui.input_mut(|i| {
            (
                i.consume_key(Modifiers::NONE, Key::ArrowDown),
                i.consume_key(Modifiers::NONE, Key::ArrowUp),
                i.consume_key(Modifiers::NONE, Key::Enter),
                i.consume_key(Modifiers::NONE, Key::Escape),
            )
        });
        if down {
            self.move_selection(1);
        }
        if up {
            self.move_selection(-1);
        }

        let placeholder = self.delegate.placeholder();
        let q = query_line(ui, &mut self.query, &placeholder);
        if self.request_focus {
            q.response.request_focus();
            self.request_focus = false;
        }
        if q.changed {
            self.delegate.update_query(&self.query);
            self.selected = 0;
            out.query_changed = true;
        }

        if let Some(hint) = self.delegate.hint() {
            self.show_hint(ui, &hint);
        }

        let count = self.delegate.match_count();
        let mut clicked_row = None;
        if count == 0 {
            if let Some(empty) = self.delegate.empty_text() {
                self.show_empty(ui, &empty);
            }
        } else {
            clicked_row = self.show_list(ui, count);
        }
        self.scroll_to_selected = false;

        if let Some(index) = clicked_row {
            self.selected = index;
            out.event = self.confirm_current();
        }
        if enter && out.event.is_none() {
            out.event = self.confirm_current();
        }
        if escape {
            self.delegate.dismiss();
            out.event = Some(PickerEvent::Dismissed);
        }

        out
    }

    fn show_hint(&self, ui: &mut egui::Ui, hint: &PickerHint) {
        let t = ui.tokens();
        let dark = ui.visuals().dark_mode;
        let (text, colour) = match hint {
            PickerHint::Info(text) => (text, to_color32(semantic(dark).text.muted)),
            PickerHint::Error(text) => (text, ui.visuals().error_fg_color),
        };
        ui.add_space(t.space[2]);
        ui.horizontal(|ui| {
            ui.add_space(t.space[4]);
            ui.label(RichText::new(text).small().color(colour));
        });
        ui.add_space(t.space[2]);
    }

    fn show_empty(&self, ui: &mut egui::Ui, text: &str) {
        let t = ui.tokens();
        let dark = ui.visuals().dark_mode;
        let muted = to_color32(semantic(dark).text.muted);
        ui.add_space(t.space[8]);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(text).color(muted));
        });
        ui.add_space(t.space[8]);
    }

    /// The match list. Returns the row that was clicked, if any.
    fn show_list(&self, ui: &mut egui::Ui, count: usize) -> Option<usize> {
        let t = ui.tokens();
        let dark = ui.visuals().dark_mode;
        let sem = semantic(dark);
        let muted = to_color32(sem.text.muted);
        let secondary = to_color32(sem.text.secondary);
        let mut clicked = None;

        ui.add_space(t.space[2]);
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .max_height(ui.available_height())
            .show(ui, |ui| {
                for index in 0..count {
                    if let Some(header) = self.delegate.header_before(index) {
                        ui.add_space(t.space[5]);
                        ui.horizontal(|ui| {
                            ui.add_space(t.space[4]);
                            ui.label(RichText::new(header).small().color(secondary));
                        });
                        ui.add_space(t.space[1]);
                    }
                    let row = self.delegate.row(index);
                    let selected = index == self.selected;
                    let r = list_row(
                        ui,
                        ListRow::new(self.row_height).selected(selected),
                        |ui, _state| {
                            // The keycap's column is claimed BEFORE anything is
                            // laid out against it. Both labels below read
                            // `ui.available_width()` at the moment they are
                            // added, so a chip added afterwards is drawn over
                            // text that was handed the whole row — and
                            // `key_chip` paints an opaque fill, which means the
                            // last characters and the elision marker were drawn
                            // and then covered rather than never drawn.
                            // `chip_width` is the horizontal twin of
                            // `chip_height`, and it exists so this row can know
                            // the column without drawing the chip first.
                            let row_rect = ui.max_rect();
                            let chip_column = row
                                .keystroke
                                .as_deref()
                                .map_or(0.0, |k| chip_width(ui, k) + t.icon_label_gap);
                            let text_rect = row_rect.with_max_x(row_rect.right() - chip_column);

                            ui.scope_builder(
                                UiBuilder::new()
                                    .max_rect(text_rect)
                                    .layout(Layout::left_to_right(Align::Center)),
                                |ui| {
                                    ui.add(
                                        egui::Label::new(row.label.as_str())
                                            .selectable(false)
                                            .truncate(),
                                    );
                                    if let Some(detail) = &row.detail {
                                        ui.add_space(t.icon_label_gap);
                                        detail_label(ui, detail, muted);
                                    }
                                },
                            );

                            if let Some(keystroke) = &row.keystroke {
                                ui.scope_builder(
                                    UiBuilder::new()
                                        .max_rect(row_rect)
                                        .layout(Layout::right_to_left(Align::Center)),
                                    |ui| {
                                        key_chip(ui, keystroke);
                                    },
                                );
                            }
                        },
                    );
                    if selected && self.scroll_to_selected {
                        r.response.scroll_to_me(None);
                    }
                    if r.clicked() {
                        clicked = Some(index);
                    }
                }
            });
        clicked
    }
}

/// The row's supporting annotation, drawn for the column it was actually
/// given.
///
/// Deliberately not an `egui::Label`. That widget publishes its *galley's*
/// text to the accessibility tree, and [`elided_detail`]'s word-boundary pass
/// hands back a galley whose text is the cut string — so a reader, and every
/// test that resolves a row by its description, would get the abbreviation
/// instead of the description. The node below is published from the source
/// string, and the tooltip egui gives an elided label is reproduced here
/// rather than lost.
fn detail_label(ui: &mut egui::Ui, detail: &str, colour: Color32) -> egui::Response {
    let galley = elided_detail(ui, detail, colour, ui.available_width());
    let cut = galley.elided || galley.text() != detail;
    let (rect, mut response) = ui.allocate_exact_size(galley.size(), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter().galley(rect.left_top(), galley, colour);
    }
    let enabled = ui.is_enabled();
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, enabled, detail));
    if cut {
        response = response.on_hover_text(detail);
    }
    response
}

/// The elision marker a description is cut with — `epaint`'s own default,
/// named here because the word-boundary pass below appends it by hand.
pub(crate) const ELISION: char = '\u{2026}';

/// A description laid out for the column it was actually given: one row of
/// whole words, ending in a visible [`ELISION`] when it does not fit.
///
/// `egui::Label::truncate()` cannot express this. It takes
/// `ui.available_width()` for its `max_width` — which is why the caller
/// reserves the keycap's column before this is reached rather than after — and
/// it leaves the cut wherever the width ran out, which is mid-word for almost
/// every string.
///
/// Two passes, and the division between them is the point. The first asks
/// epaint *how much fits*, marker included, with `break_anywhere` set exactly
/// as `truncate()` sets it: the answer is the most characters the column will
/// take. The second asks *where a reader would cut it*, which is the last whole
/// word inside that answer. Neither question can be put to the layouter alone —
/// `break_anywhere = false` makes epaint break at the last word boundary and
/// then pop glyphs back off it to make room for the marker
/// (`replace_last_glyph_with_overflow_character`), so the cut still lands
/// mid-word, which is what `TextWrapping::break_anywhere`'s own documentation
/// warns about; and it keeps the space it broke on, so the row reads `producer
/// \u{2026}` with the marker hanging off a gap.
///
/// The string the second pass lays out is a prefix of one epaint already
/// fitted inside `max_width`, so it fits too.
fn elided_detail(ui: &egui::Ui, detail: &str, colour: Color32, max_width: f32) -> Arc<Galley> {
    let format = TextFormat {
        font_id: FontSelection::Default.resolve(ui.style()),
        color: colour,
        valign: ui.text_valign(),
        ..Default::default()
    };
    let wrap = TextWrapping {
        max_width,
        max_rows: 1,
        break_anywhere: true,
        overflow_character: Some(ELISION),
    };

    let mut job = LayoutJob::single_section(detail.to_owned(), format.clone());
    job.wrap = wrap.clone();
    // The reservation the caller made is exact, so the galley's width has to be
    // too. `round_output_to_gui` exists for the feedback loop where egui reports
    // a rounded galley width and hands it back as the next frame's `max_width`;
    // it pays for that by rounding the row up by as much as half a point and by
    // wrapping half a point late (`LayoutJob::effective_wrap_width`), which is
    // one whole point of licence to cross the column boundary. Nothing here
    // feeds a reported width back — `max_width` comes from the row's own
    // geometry — so the loop it guards against does not exist and the licence is
    // pure loss.
    job.round_output_to_gui = false;
    let galley = ui.painter().layout_job(job);
    if !galley.elided {
        return galley;
    }

    let kept = galley.rows[0].row.text();
    let body = kept.strip_suffix(ELISION).unwrap_or(kept.as_str());
    let words = body.trim_end();
    let cut = if detail
        .get(words.len()..)
        .is_some_and(|rest| rest.starts_with(char::is_whitespace))
    {
        // The first pass already stopped after a whole word; all that is left
        // is whatever whitespace it stopped on.
        words
    } else if let Some(space) = words.rfind(char::is_whitespace) {
        &words[..space]
    } else {
        // One word wider than the column, so there is no boundary to cut at.
        // A mid-glyph cut carrying the marker is what is left, and it is what
        // the first pass already produced.
        return galley;
    };
    if cut == body {
        return galley;
    }

    let mut retry = LayoutJob::single_section(format!("{cut}{ELISION}"), format);
    retry.wrap = wrap;
    retry.round_output_to_gui = false;
    ui.painter().layout_job(retry)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixed {
        count: usize,
        confirmed: Option<(Option<usize>, String)>,
        dismissed: bool,
    }

    impl Fixed {
        fn new(count: usize) -> Self {
            Self {
                count,
                confirmed: None,
                dismissed: false,
            }
        }
    }

    impl PickerDelegate for Fixed {
        fn update_query(&mut self, _query: &str) {}
        fn match_count(&self) -> usize {
            self.count
        }
        fn row(&self, index: usize) -> PickerRow {
            PickerRow::new(format!("item {index}"))
        }
        fn confirm(&mut self, index: Option<usize>, query: &str) -> PickerOutcome {
            self.confirmed = Some((index, query.to_owned()));
            PickerOutcome::Close
        }
        fn dismiss(&mut self) {
            self.dismissed = true;
        }
    }

    #[test]
    fn selection_wraps_in_both_directions() {
        let mut p = Picker::new(Fixed::new(3));
        assert_eq!(p.selected(), 0);
        p.move_selection(-1);
        assert_eq!(p.selected(), 2, "wraps upward from the first row");
        p.move_selection(1);
        assert_eq!(p.selected(), 0, "wraps downward from the last row");
        p.move_selection(2);
        assert_eq!(p.selected(), 2);
    }

    #[test]
    fn selection_is_inert_with_no_matches() {
        let mut p = Picker::new(Fixed::new(0));
        p.move_selection(1);
        p.move_selection(-1);
        assert_eq!(p.selected(), 0);
    }

    #[test]
    fn confirm_with_no_matches_hands_over_the_typed_query() {
        let mut p = Picker::new(Fixed::new(0));
        p.set_query("42ms");
        let event = p.confirm_current();
        assert_eq!(event, Some(PickerEvent::Confirmed));
        assert_eq!(
            p.delegate.confirmed,
            Some((None, "42ms".to_owned())),
            "a typed-value prompt confirms the raw query, not a row"
        );
    }

    #[test]
    fn confirm_with_matches_hands_over_the_selected_row() {
        let mut p = Picker::new(Fixed::new(3));
        p.move_selection(1);
        let event = p.confirm_current();
        assert_eq!(event, Some(PickerEvent::Confirmed));
        assert_eq!(p.delegate.confirmed, Some((Some(1), String::new())));
    }

    struct ReadOnly(Fixed);
    impl PickerDelegate for ReadOnly {
        fn update_query(&mut self, q: &str) {
            self.0.update_query(q);
        }
        fn match_count(&self) -> usize {
            self.0.match_count()
        }
        fn row(&self, index: usize) -> PickerRow {
            self.0.row(index)
        }
        fn confirm(&mut self, index: Option<usize>, query: &str) -> PickerOutcome {
            self.0.confirm(index, query)
        }
        fn confirmable(&self) -> bool {
            false
        }
        fn dismiss(&mut self) {
            self.0.dismiss();
        }
    }

    #[test]
    fn enter_on_a_read_only_sheet_dismisses_instead_of_confirming() {
        let mut p = Picker::new(ReadOnly(Fixed::new(2)));
        let event = p.confirm_current();
        assert_eq!(event, Some(PickerEvent::Dismissed));
        assert!(p.delegate.0.dismissed);
        assert!(p.delegate.0.confirmed.is_none());
    }

    struct KeepsOpen;
    impl PickerDelegate for KeepsOpen {
        fn update_query(&mut self, _query: &str) {}
        fn match_count(&self) -> usize {
            0
        }
        fn row(&self, _index: usize) -> PickerRow {
            unreachable!("no matches ever")
        }
        fn confirm(&mut self, _index: Option<usize>, _query: &str) -> PickerOutcome {
            PickerOutcome::KeepOpen
        }
    }

    #[test]
    fn keep_open_swallows_the_event() {
        let mut p = Picker::new(KeepsOpen);
        assert_eq!(p.confirm_current(), None);
    }

    /// Overrides every defaulted trait method, so a forwarding impl that let
    /// any one of them fall back to the trait default is caught by name.
    struct Full {
        dismissed: std::rc::Rc<std::cell::Cell<bool>>,
    }

    impl PickerDelegate for Full {
        fn update_query(&mut self, _query: &str) {}
        fn match_count(&self) -> usize {
            1
        }
        fn row(&self, _index: usize) -> PickerRow {
            PickerRow::new("row")
        }
        fn confirm(&mut self, _index: Option<usize>, _query: &str) -> PickerOutcome {
            PickerOutcome::Close
        }
        fn placeholder(&self) -> String {
            "Type a duration".to_owned()
        }
        fn header_before(&self, index: usize) -> Option<String> {
            (index == 0).then(|| "Section".to_owned())
        }
        fn hint(&self) -> Option<PickerHint> {
            Some(PickerHint::Info("guidance".to_owned()))
        }
        fn confirmable(&self) -> bool {
            false
        }
        fn empty_text(&self) -> Option<String> {
            None
        }
        fn dismiss(&mut self) {
            self.dismissed.set(true);
        }
    }

    #[test]
    fn the_boxed_slot_compiles_and_hosts_different_delegate_kinds() {
        // The advertised one-modal-slot pattern, verbatim: the concrete box
        // coerces to `Box<dyn PickerDelegate>` and drives the picker.
        let mut p: Picker<Box<dyn PickerDelegate>> = Picker::new(Box::new(Fixed::new(3)));
        p.move_selection(1);
        assert_eq!(p.confirm_current(), Some(PickerEvent::Confirmed));
        p.delegate = Box::new(KeepsOpen);
        assert_eq!(
            p.confirm_current(),
            None,
            "the same slot hosts a different delegate kind"
        );
    }

    #[test]
    fn box_forwarding_preserves_every_defaulted_override() {
        let dismissed = std::rc::Rc::new(std::cell::Cell::new(false));
        let mut boxed: Box<dyn PickerDelegate> = Box::new(Full {
            dismissed: std::rc::Rc::clone(&dismissed),
        });
        assert_eq!(boxed.placeholder(), "Type a duration");
        assert_eq!(boxed.header_before(0), Some("Section".to_owned()));
        assert_eq!(boxed.header_before(1), None);
        assert_eq!(boxed.hint(), Some(PickerHint::Info("guidance".to_owned())));
        assert!(!boxed.confirmable());
        assert_eq!(boxed.empty_text(), None);
        boxed.dismiss();
        assert!(
            dismissed.get(),
            "dismiss reaches the inner delegate, not the trait's no-op default"
        );
    }

    #[test]
    fn a_shrinking_match_set_clamps_the_selection_on_show() {
        // Covered structurally: `show` clamps before drawing. Exercise the
        // clamp through the public pieces that feed it.
        let mut p = Picker::new(Fixed::new(5));
        p.move_selection(4);
        assert_eq!(p.selected(), 4);
        p.delegate.count = 2;
        let event = p.confirm_current();
        assert_eq!(event, Some(PickerEvent::Confirmed));
        assert_eq!(
            p.delegate.confirmed,
            Some((Some(1), String::new())),
            "confirm clamps into the shrunken match set"
        );
    }
}
