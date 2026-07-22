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
    list_row, theme, ListRow, ModalChrome, ModalLayer, Mode, Notification, NotificationId,
    NotificationLayer, Picker, PickerDelegate, PickerEvent, PickerOutcome, PickerRow, RowHeight,
    Severity, Toast, ToastLayer,
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
