//! The overlay/picker primitives drawn for real, headlessly.
//!
//! The unit tests beside each module assert the pure logic (selection
//! wrapping, banner dedup, token resolution); these run a live
//! `egui::Context` through `egui_kittest` and drive the primitives the way a
//! user would — typing, arrows, enter, escape, hover, clicks — on the CPU
//! tessellation path only, so they are green on a headless CI runner with no
//! GPU and carry no pixel baselines.

use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use meridian_egui::{
    key_chip, list_row, theme, ListRow, MeridianUi, ModalChrome, ModalLayer, Mode, Notification,
    NotificationId, NotificationLayer, Picker, PickerDelegate, PickerEvent, PickerHint,
    PickerOutcome, PickerRow, RowHeight, Severity, Toast, ToastLayer,
};

/// A flat list delegate with case-insensitive substring filtering — the
/// jump-list shape, and enough of the palette shape to drive the chrome.
struct JumpList {
    items: Vec<&'static str>,
    matches: Vec<usize>,
    confirmed: Option<(Option<usize>, String)>,
    dismissed: bool,
}

impl JumpList {
    fn new(items: Vec<&'static str>) -> Self {
        Self {
            matches: (0..items.len()).collect(),
            items,
            confirmed: None,
            dismissed: false,
        }
    }
}

impl PickerDelegate for JumpList {
    fn update_query(&mut self, query: &str) {
        let needle = query.to_lowercase();
        self.matches = (0..self.items.len())
            .filter(|&i| self.items[i].to_lowercase().contains(&needle))
            .collect();
    }
    fn match_count(&self) -> usize {
        self.matches.len()
    }
    fn row(&self, index: usize) -> PickerRow {
        PickerRow::new(self.items[self.matches[index]])
    }
    fn confirm(&mut self, index: Option<usize>, query: &str) -> PickerOutcome {
        self.confirmed = Some((index, query.to_owned()));
        PickerOutcome::Close
    }
    fn dismiss(&mut self) {
        self.dismissed = true;
    }
}

struct PickerFixture {
    picker: Picker<JumpList>,
    last_event: Option<PickerEvent>,
}

fn picker_harness() -> Harness<'static, PickerFixture> {
    Harness::new_ui_state(
        |ui, s: &mut PickerFixture| {
            theme::apply(ui.ctx(), Mode::Light);
            let r = s.picker.show(ui);
            if r.event.is_some() {
                s.last_event = r.event;
            }
        },
        PickerFixture {
            picker: Picker::new(JumpList::new(vec!["alpha", "beta", "gamma"])),
            last_event: None,
        },
    )
}

#[test]
fn picker_lists_every_match_before_any_query() {
    let mut harness = picker_harness();
    harness.run();
    harness.get_by_label("alpha");
    harness.get_by_label("beta");
    harness.get_by_label("gamma");
}

#[test]
fn typing_filters_the_list_through_the_delegate() {
    let mut harness = picker_harness();
    harness.run();
    // The picker focuses its own query line on first show; typed text lands
    // there without any clicking.
    harness.event(egui::Event::Text("ga".to_owned()));
    harness.run();
    harness.get_by_label("gamma");
    assert!(harness.query_by_label("alpha").is_none());
    assert!(harness.query_by_label("beta").is_none());
    assert_eq!(harness.state().picker.query(), "ga");
}

#[test]
fn arrows_move_the_selection_and_enter_confirms_it() {
    let mut harness = picker_harness();
    harness.run();
    harness.key_press(egui::Key::ArrowDown);
    harness.run();
    assert_eq!(harness.state().picker.selected(), 1);
    harness.key_press(egui::Key::Enter);
    harness.run();
    let state = harness.state();
    assert_eq!(state.last_event, Some(PickerEvent::Confirmed));
    assert_eq!(
        state.picker.delegate.confirmed,
        Some((Some(1), String::new()))
    );
}

#[test]
fn escape_dismisses_and_tells_the_delegate() {
    let mut harness = picker_harness();
    harness.run();
    harness.key_press(egui::Key::Escape);
    harness.run();
    let state = harness.state();
    assert_eq!(state.last_event, Some(PickerEvent::Dismissed));
    assert!(state.picker.delegate.dismissed);
    assert!(state.picker.delegate.confirmed.is_none());
}

#[test]
fn clicking_a_row_confirms_that_row() {
    let mut harness = picker_harness();
    harness.run();
    harness.get_by_label("beta").click();
    harness.run();
    let state = harness.state();
    assert_eq!(state.last_event, Some(PickerEvent::Confirmed));
    assert_eq!(
        state.picker.delegate.confirmed,
        Some((Some(1), String::new()))
    );
}

/// The grouped read-only-sheet shape: sections with headers, filtering keeps
/// each surviving section's header on its first surviving row.
struct GroupedSheet {
    /// `(section, label)` pairs, in section order.
    items: Vec<(&'static str, &'static str)>,
    matches: Vec<usize>,
}

impl GroupedSheet {
    fn new(items: Vec<(&'static str, &'static str)>) -> Self {
        Self {
            matches: (0..items.len()).collect(),
            items,
        }
    }
}

impl PickerDelegate for GroupedSheet {
    fn update_query(&mut self, query: &str) {
        let needle = query.to_lowercase();
        self.matches = (0..self.items.len())
            .filter(|&i| self.items[i].1.to_lowercase().contains(&needle))
            .collect();
    }
    fn match_count(&self) -> usize {
        self.matches.len()
    }
    fn row(&self, index: usize) -> PickerRow {
        PickerRow::new(self.items[self.matches[index]].1)
    }
    fn confirm(&mut self, _index: Option<usize>, _query: &str) -> PickerOutcome {
        PickerOutcome::Close
    }
    fn header_before(&self, index: usize) -> Option<String> {
        let section = self.items[self.matches[index]].0;
        let first_of_section = index == 0 || self.items[self.matches[index - 1]].0 != section;
        first_of_section.then(|| section.to_owned())
    }
}

#[test]
fn grouped_headers_render_once_per_section_and_follow_the_filter() {
    let mut harness = Harness::new_ui_state(
        |ui, picker: &mut Picker<GroupedSheet>| {
            theme::apply(ui.ctx(), Mode::Light);
            picker.show(ui);
        },
        Picker::new(GroupedSheet::new(vec![
            ("Recent", "alpha.csv"),
            ("Recent", "beta.csv"),
            ("Pinned", "gamma.csv"),
        ])),
    );
    harness.run();
    // `get_by_label` panics on duplicates, so each hit also asserts the
    // header rendered exactly once — before its section, not per row.
    harness.get_by_label("Recent");
    harness.get_by_label("Pinned");
    harness.get_by_label("alpha.csv");
    harness.get_by_label("beta.csv");
    harness.get_by_label("gamma.csv");

    // Filtering to the second section re-derives the headers: "Pinned" now
    // heads the first visible row and "Recent" is gone with its rows.
    harness.event(egui::Event::Text("gamma".to_owned()));
    harness.run();
    harness.get_by_label("Pinned");
    harness.get_by_label("gamma.csv");
    assert!(harness.query_by_label("Recent").is_none());
    assert!(harness.query_by_label("alpha.csv").is_none());
}

/// The typed-value-prompt shape: zero matches by design, the query is the
/// value, validation narrates through `hint`.
struct DurationPrompt {
    hint: Option<PickerHint>,
    value: Option<String>,
}

impl DurationPrompt {
    fn new() -> Self {
        let mut prompt = Self {
            hint: None,
            value: None,
        };
        prompt.update_query("");
        prompt
    }

    fn valid(query: &str) -> bool {
        query
            .strip_suffix("ms")
            .is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()))
    }
}

impl PickerDelegate for DurationPrompt {
    fn update_query(&mut self, query: &str) {
        self.hint = if query.is_empty() {
            Some(PickerHint::Info("for example 250ms".to_owned()))
        } else if Self::valid(query) {
            None
        } else {
            Some(PickerHint::Error("enter a duration in ms".to_owned()))
        };
    }
    fn match_count(&self) -> usize {
        0
    }
    fn row(&self, _index: usize) -> PickerRow {
        unreachable!("a typed-value prompt never has matches")
    }
    fn confirm(&mut self, _index: Option<usize>, query: &str) -> PickerOutcome {
        if Self::valid(query) {
            self.value = Some(query.to_owned());
            PickerOutcome::Close
        } else {
            PickerOutcome::KeepOpen
        }
    }
    fn empty_text(&self) -> Option<String> {
        None
    }
    fn hint(&self) -> Option<PickerHint> {
        self.hint.clone()
    }
}

#[test]
fn a_typed_value_prompt_narrates_through_the_hint_line() {
    let mut harness = Harness::new_ui_state(
        |ui, s: &mut PromptFixture| {
            theme::apply(ui.ctx(), Mode::Light);
            let r = s.picker.show(ui);
            if r.event.is_some() {
                s.last_event = r.event;
            }
        },
        PromptFixture {
            picker: Picker::new(DurationPrompt::new()),
            last_event: None,
        },
    );
    harness.run();
    // The info hint renders under the query line; the empty state is
    // suppressed (`empty_text` = None), not narrated as "no matches".
    harness.get_by_label("for example 250ms");
    assert!(harness.query_by_label("No matches").is_none());

    // An invalid value swaps the hint to the error line, and enter keeps the
    // prompt open instead of confirming.
    harness.event(egui::Event::Text("42".to_owned()));
    harness.run();
    harness.get_by_label("enter a duration in ms");
    harness.key_press(egui::Key::Enter);
    harness.run();
    assert_eq!(harness.state().last_event, None, "invalid value keeps open");

    // Completing the value clears the hint; enter now confirms the raw query.
    harness.event(egui::Event::Text("ms".to_owned()));
    harness.run();
    assert!(harness.query_by_label("enter a duration in ms").is_none());
    assert!(harness.query_by_label("for example 250ms").is_none());
    harness.key_press(egui::Key::Enter);
    harness.run();
    let state = harness.state();
    assert_eq!(state.last_event, Some(PickerEvent::Confirmed));
    assert_eq!(state.picker.delegate.value, Some("42ms".to_owned()));
}

struct PromptFixture {
    picker: Picker<DurationPrompt>,
    last_event: Option<PickerEvent>,
}

struct BoxedFixture {
    picker: Picker<Box<dyn PickerDelegate>>,
    last_event: Option<PickerEvent>,
}

#[test]
fn one_boxed_slot_drives_the_full_chrome() {
    // The one-modal-slot pattern the docs advertise, drawn for real: the
    // delegate lives behind `Box<dyn PickerDelegate>` and the picker still
    // lists, filters, and confirms through it.
    let mut harness = Harness::new_ui_state(
        |ui, s: &mut BoxedFixture| {
            theme::apply(ui.ctx(), Mode::Light);
            let r = s.picker.show(ui);
            if r.event.is_some() {
                s.last_event = r.event;
            }
        },
        BoxedFixture {
            picker: Picker::new(Box::new(JumpList::new(vec!["alpha", "beta"]))),
            last_event: None,
        },
    );
    harness.run();
    harness.get_by_label("alpha");
    harness.event(egui::Event::Text("be".to_owned()));
    harness.run();
    harness.get_by_label("beta");
    assert!(harness.query_by_label("alpha").is_none());
    harness.key_press(egui::Key::Enter);
    harness.run();
    assert_eq!(harness.state().last_event, Some(PickerEvent::Confirmed));
}

#[test]
fn modal_layer_draws_the_chrome_and_escape_dismisses() {
    struct S {
        open: bool,
    }
    let mut harness = Harness::new_ui_state(
        |ui, s: &mut S| {
            theme::apply(ui.ctx(), Mode::Light);
            if s.open {
                let r = ModalLayer::show(
                    ui.ctx(),
                    "test_modal",
                    &ModalChrome::new()
                        .title("Rename dataset")
                        .enter_hint("apply"),
                    |ui| ui.label("Body content"),
                );
                if r.dismissed {
                    s.open = false;
                }
            }
        },
        S { open: true },
    );
    harness.run();
    harness.get_by_label("Rename dataset");
    harness.get_by_label("Body content");
    // The one escape-affordance treatment: keycap chip plus verb.
    harness.get_by_label("Esc");
    harness.get_by_label("close");
    harness.get_by_label("Enter");
    harness.get_by_label("apply");

    harness.key_press(egui::Key::Escape);
    harness.run();
    assert!(!harness.state().open, "escape closes via `dismissed`");
    assert!(harness.query_by_label("Rename dataset").is_none());
}

#[test]
fn a_picker_inside_the_modal_layer_reports_the_escape_itself() {
    struct S {
        picker: Picker<JumpList>,
        open: bool,
    }
    let mut harness = Harness::new_ui_state(
        |ui, s: &mut S| {
            theme::apply(ui.ctx(), Mode::Light);
            if s.open {
                let r = ModalLayer::show(ui.ctx(), "palette", &ModalChrome::new(), |ui| {
                    s.picker.show(ui)
                });
                if r.dismissed || r.inner.event.is_some() {
                    s.open = false;
                }
            }
        },
        S {
            picker: Picker::new(JumpList::new(vec!["alpha", "beta"])),
            open: true,
        },
    );
    harness.run();
    harness.get_by_label("alpha");
    harness.key_press(egui::Key::Escape);
    harness.run();
    assert!(!harness.state().open);
    assert!(harness.state().picker.delegate.dismissed);
}

#[test]
fn list_row_reveals_affordances_by_visibility_on_hover() {
    let mut harness = Harness::new_ui(|ui| {
        theme::apply(ui.ctx(), Mode::Light);
        list_row(ui, ListRow::new(RowHeight::Comfortable), |ui, state| {
            ui.label("Row A");
            // Visibility-based reveal: the widget exists only while the row
            // is hovered. There is no zero-opacity phantom keeping a hover
            // target alive — the row itself is the sensor.
            if state.hovered {
                let _ = ui.small_button("Remove");
            }
        });
    });
    harness.run();
    assert!(
        harness.query_by_label("Remove").is_none(),
        "affordance absent until hover"
    );
    harness.get_by_label("Row A").hover();
    harness.run();
    harness.get_by_label("Remove");
}

#[test]
fn notification_layer_renders_the_replacing_banner_only() {
    let mut layer = NotificationLayer::new();
    let id = NotificationId::composite("publish", "upload");
    layer.raise(Notification::new(id, Severity::Error, "upload failed"));
    layer.raise(Notification::new(
        id,
        Severity::Error,
        "upload failed: retry 2",
    ));

    let mut harness = Harness::new_ui_state(
        |ui, layer: &mut NotificationLayer| {
            theme::apply(ui.ctx(), Mode::Light);
            layer.show(ui.ctx());
        },
        layer,
    );
    harness.run();
    harness.get_by_label("upload failed: retry 2");
    assert!(harness.query_by_label("upload failed").is_none());
    assert_eq!(harness.state().len(), 1);
}

#[test]
fn notification_banner_dismiss_button_removes_it() {
    let mut layer = NotificationLayer::new();
    layer.raise(Notification::new(
        NotificationId::new("solo"),
        Severity::Info,
        "one banner",
    ));
    let mut harness = Harness::new_ui_state(
        |ui, layer: &mut NotificationLayer| {
            theme::apply(ui.ctx(), Mode::Light);
            layer.show(ui.ctx());
        },
        layer,
    );
    harness.run();
    harness.get_by_label("×").click();
    harness.run();
    assert!(harness.state().is_empty());
    assert!(harness.query_by_label("one banner").is_none());
}

#[test]
fn toast_layer_renders_queued_toasts() {
    let mut layer = ToastLayer::new();
    layer.push(Toast::new(Severity::Success, "dataset saved"));
    let mut harness = Harness::new_ui_state(
        |ui, layer: &mut ToastLayer| {
            theme::apply(ui.ctx(), Mode::Light);
            layer.show(ui.ctx());
        },
        layer,
    );
    harness.run();
    harness.get_by_label("dataset saved");
}

// ─── the keycap chips in a modal footer ──────────────────────────────────────
//
// These read the *paint list*, not the accesskit tree, and that is the point.
// A chip is an `egui::Frame` around a galley: the galley is what the tree
// carries a size for, and the galley never stretched. What stretched was the
// chip's own box, which exists only as a painted shape. `surfaces.sunken` at
// `radius_chip` is drawn by `key_chip` and by nothing else in the crate, so
// that pair addresses the chip boxes exactly.

/// The hairline every box in this crate is stroked with. Named here so the
/// content-derived chip height below is spelled out term by term rather than
/// borrowed from the code under test.
const HAIRLINE: f32 = 1.0;

/// Every rounded rect a frame painted, flattened out of the shape tree — an
/// `egui::Frame` nests its background under a `Shape::Vec` when it also draws
/// a shadow, which the modal card does.
fn painted_rects<S>(
    harness: &Harness<'_, S>,
) -> Vec<(egui::Rect, egui::Color32, egui::CornerRadius)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(egui::Rect, egui::Color32, egui::CornerRadius)>) {
        match shape {
            egui::Shape::Rect(r) => out.push((r.rect, r.fill, r.corner_radius)),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &harness.output().shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// The rects painted with `fill` at `radius`.
fn boxes_of<S>(harness: &Harness<'_, S>, fill: egui::Color32, radius: f32) -> Vec<egui::Rect> {
    let radius = egui::CornerRadius::from(radius);
    painted_rects(harness)
        .into_iter()
        .filter(|(_, f, r)| *f == fill && *r == radius)
        .map(|(rect, _, _)| rect)
        .collect()
}

/// The height a keycap chip's *content* implies: the keystroke's galley in the
/// chip's monospace ink, plus the two spacing-ladder padding steps and the
/// hairline, top and bottom. Laid out here from the tokens rather than asked
/// of the code under test.
fn content_chip_height(ui: &egui::Ui, keystroke: &str) -> f32 {
    let galley = ui.painter().layout_no_wrap(
        keystroke.to_owned(),
        egui::FontId::new(
            meridian_design::typography::CHART_LABEL_SIZE,
            egui::FontFamily::Monospace,
        ),
        egui::Color32::PLACEHOLDER,
    );
    galley.size().y + 2.0 * ui.tokens().space[1] + 2.0 * HAIRLINE
}

/// What one drawn modal measured.
#[derive(Default)]
struct Drawn {
    /// The chip boxes the footer painted, tallest first.
    chips: Vec<egui::Rect>,
    /// The card box itself.
    card: Option<egui::Rect>,
    /// The chip height this modal's content implies.
    content_height: f32,
}

/// Draw `chrome` around `body` on a window of `size` and measure what was
/// painted.
fn draw_modal(
    size: (f32, f32),
    mode: Mode,
    chrome: ModalChrome,
    body: impl Fn(&mut egui::Ui) + Copy,
) -> Drawn {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(size.0, size.1))
        .build_ui_state(
            |ui, m: &mut Drawn| {
                theme::apply(ui.ctx(), mode);
                m.content_height = content_chip_height(ui, "Esc");
                ModalLayer::show(ui.ctx(), "measured", &chrome, |ui| body(ui));
            },
            Drawn::default(),
        );
    harness.run();

    let sem = meridian_design::semantic(mode.is_dark());
    let mut chips = boxes_of(
        &harness,
        theme::to_color32(sem.surfaces.sunken),
        meridian_egui::TOKENS.radius_chip,
    );
    chips.sort_by(|a, b| b.height().total_cmp(&a.height()));
    let card = boxes_of(
        &harness,
        theme::to_color32(sem.surfaces.overlay),
        meridian_egui::TOKENS.radius_panel,
    )
    .into_iter()
    .max_by(|a, b| a.height().total_cmp(&b.height()));

    let content_height = harness.state().content_height;
    Drawn {
        chips,
        card,
        content_height,
    }
}

/// A body of two plain lines — content-sized, so the card's own measurement is
/// what is under test rather than the body's.
fn plain_body(ui: &mut egui::Ui) {
    ui.label("Body line one");
    ui.label("Body line two");
}

#[test]
fn a_footer_keycap_is_keycap_sized_however_much_height_the_card_has_left() {
    let chrome = || ModalChrome::new().title("Commands").enter_hint("run");
    let tall = draw_modal((1400.0, 900.0), Mode::Light, chrome(), plain_body);
    let short = draw_modal((900.0, 260.0), Mode::Light, chrome(), plain_body);

    for (window, drawn) in [("tall", &tall), ("short", &short)] {
        assert_eq!(
            drawn.chips.len(),
            2,
            "{window} window: the footer paints an Esc box and an Enter box"
        );
        for chip in &drawn.chips {
            assert!(
                (chip.height() - drawn.content_height).abs() < 0.5,
                "{window} window: a keycap chip drew {:.1}pt tall; its content \
                 is {:.1}pt. A chip is its galley plus its padding and \
                 hairline, not the height the card had spare.",
                chip.height(),
                drawn.content_height,
            );
        }
    }

    // The observable failure the fix is for: the same chip on a taller window.
    assert!(
        (tall.chips[0].height() - short.chips[0].height()).abs() < 0.5,
        "the tallest chip drew {:.1}pt on a 900pt-high window and {:.1}pt on a \
         260pt-high one — a keycap cannot depend on the window",
        tall.chips[0].height(),
        short.chips[0].height(),
    );
}

#[test]
fn every_chrome_shape_keeps_its_keycaps_keycap_sized() {
    // Every distinct `ModalChrome` shape a consumer builds. The chips live in
    // the shared chrome, so one modal cannot be right while another stretches
    // — but the footer's height is derived from the hints it draws, so each
    // combination of them is drawn and measured.
    let shapes: [(&str, ModalChrome, usize); 6] = [
        (
            "title, escape and enter",
            ModalChrome::new().title("Commands").enter_hint("run"),
            2,
        ),
        (
            "title and escape only",
            ModalChrome::new().title("Keyboard help"),
            1,
        ),
        (
            "narrow, title and escape",
            ModalChrome::new().title("Specimen").narrow(),
            1,
        ),
        ("no title", ModalChrome::new(), 1),
        (
            "renamed escape verb",
            ModalChrome::new().esc_hint("cancel"),
            1,
        ),
        (
            "enter only, escape removed",
            ModalChrome::new().without_esc_hint().enter_hint("jump"),
            1,
        ),
    ];

    for (name, chrome, expected_chips) in shapes {
        for mode in [Mode::Light, Mode::Dark] {
            let drawn = draw_modal((1400.0, 900.0), mode, chrome.clone(), plain_body);
            assert_eq!(
                drawn.chips.len(),
                expected_chips,
                "{name} ({mode:?}): chip boxes painted"
            );
            for chip in &drawn.chips {
                assert!(
                    (chip.height() - drawn.content_height).abs() < 0.5,
                    "{name} ({mode:?}): a keycap chip drew {:.1}pt tall against \
                     a content height of {:.1}pt",
                    chip.height(),
                    drawn.content_height,
                );
            }
        }
    }
}

#[test]
fn the_card_stops_at_its_content_rather_than_at_its_height_cap() {
    let chrome = || ModalChrome::new().title("Commands").enter_hint("run");
    let tall = draw_modal((1400.0, 900.0), Mode::Light, chrome(), plain_body);
    let short = draw_modal((900.0, 260.0), Mode::Light, chrome(), plain_body);

    let tall_card = tall.card.expect("the card box is painted");
    let short_card = short.card.expect("the card box is painted");

    // Content-driven height, stated as the property: the same content draws
    // the same card, whatever room the window had.
    assert!(
        (tall_card.height() - short_card.height()).abs() < 0.5,
        "the same modal drew {:.1}pt tall on a 900pt-high window and {:.1}pt on \
         a 260pt-high one — the module's height claim is content-driven",
        tall_card.height(),
        short_card.height(),
    );

    // And the cap the same claim promises: the largest ladder gap of breathing
    // room, top and bottom.
    let gap = meridian_egui::TOKENS.space[9];
    assert!(
        900.0 - tall_card.height() >= 2.0 * gap,
        "a {:.1}pt card on a 900pt window leaves {:.1}pt, under the {:.1}pt the \
         cap promises",
        tall_card.height(),
        900.0 - tall_card.height(),
        2.0 * gap,
    );
}

/// How one of the cases below puts a chip on a `Ui`.
type ChipPlacement = fn(&mut egui::Ui);

/// Draw one chip through `place` and report the height of the box it painted,
/// beside the height its content implies.
fn drawn_chip_box(place: ChipPlacement) -> (f32, f32) {
    #[derive(Default)]
    struct Content(f32);
    let mut harness = Harness::builder()
        .with_size(egui::vec2(600.0, 400.0))
        .build_ui_state(
            move |ui, c: &mut Content| {
                theme::apply(ui.ctx(), Mode::Light);
                c.0 = content_chip_height(ui, "Esc");
                place(ui);
            },
            Content::default(),
        );
    harness.run();
    let content = harness.state().0;
    let boxes = boxes_of(
        &harness,
        theme::to_color32(meridian_design::semantic(false).surfaces.sunken),
        meridian_egui::TOKENS.radius_chip,
    );
    assert_eq!(boxes.len(), 1, "one chip box painted");
    (boxes[0].height(), content)
}

#[test]
fn a_chip_is_keycap_sized_in_a_layout_that_offers_it_a_column() {
    // The chip's own invariant, and it needs its own test: the modal footer no
    // longer hands a chip a column, so the footer tests above cannot tell a
    // self-measuring chip from one that fits the row it is given. These
    // layouts still hand it one — `Ui::horizontal` is what
    // `tooltip_for_action` and a host's own chip row use, and its rows are a
    // control rung tall, which is not the chip's size either.
    let cases: [(&str, ChipPlacement); 3] = [
        ("a cross-centred column", |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                key_chip(ui, "Esc");
            });
        }),
        ("a control-rung row", |ui| {
            ui.horizontal(|ui| {
                key_chip(ui, "Esc");
            });
        }),
        ("a plain stack", |ui| {
            ui.vertical(|ui| {
                key_chip(ui, "Esc");
            });
        }),
    ];

    for (layout, place) in cases {
        let (drawn, content) = drawn_chip_box(place);
        assert!(
            (drawn - content).abs() < 0.5,
            "in {layout} a chip drew {drawn:.1}pt tall against a content height \
             of {content:.1}pt",
        );
    }
}
