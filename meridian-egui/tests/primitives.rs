//! The chrome primitives drawn for real, headlessly (CPU tessellation only —
//! green on a runner with no GPU, like the theme harness test).
//!
//! The unit tests in `align.rs` pin the two-pass accumulator's mechanics with
//! exact numbers; this file checks the composition end to end: field rows
//! rendered through a live kittest harness end up sharing one value column,
//! and every primitive lays out and paints without panicking under the themed
//! context.

use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use meridian_design::semantic::Role;
use meridian_egui::{
    align, button, distribution_bar, icons, key_tip, menu_item, menu_separator, ribbon_row,
    ribbon_tabs, run_status, section, segmented, step_mark, step_row, teaching_message, text_field,
    theme, vertical_tab, widgets, Anchor, Bars, Button, DistributionBar, Gutter, MenuItem, Mode,
    Picker, PickerDelegate, PickerOutcome, PickerRow, Popover, RibbonRow, RibbonTab, RibbonTabs,
    RunStatus, Section, Segmented, SegmentedOption, StepEntry, StepMark, StepRow, TabPick,
    TextField, Timeline, VerticalTab, Weight,
};

/// One available row and one unavailable, for the picker row's unavailable
/// state.
struct TwoRows;

impl PickerDelegate for TwoRows {
    fn update_query(&mut self, _query: &str) {}

    fn match_count(&self) -> usize {
        2
    }

    fn row(&self, index: usize) -> PickerRow {
        match index {
            0 => PickerRow::new("Open a Protocol").keystroke("⌘O"),
            _ => PickerRow::new("Run this step")
                .keystroke("⌘↩")
                .unavailable("Nothing is focused."),
        }
    }

    fn confirm(&mut self, _index: Option<usize>, _query: &str) -> PickerOutcome {
        PickerOutcome::KeepOpen
    }
}

#[test]
fn field_rows_inside_a_scope_share_one_value_column() {
    let mut harness = Harness::new_ui(|ui| {
        theme::apply(ui.ctx(), Mode::Light);
        align::align_scope(ui, "inspector", |ui| {
            widgets::field_row(ui, "id", "value-a", None, true);
            widgets::field_row(ui, "a much longer label", "value-b", Some("why"), false);
            widgets::field_row(ui, "hash", "value-c", None, true);
        });
    });

    // Let the two-pass accumulator settle (frame n registers, n+1 consumes).
    harness.run();

    let ax = harness.get_by_label("value-a").rect().left();
    let bx = harness.get_by_label("value-b").rect().left();
    let cx = harness.get_by_label("value-c").rect().left();
    assert_eq!(ax, bx, "rows with different labels split at one shared x");
    assert_eq!(bx, cx, "all rows of the scope share the column");

    // The column clears the widest label: the value never overlaps it.
    let wide_label_right = harness.get_by_label("A MUCH LONGER LABEL").rect().right();
    assert!(
        bx >= wide_label_right,
        "the shared column ({bx}) starts after the widest label ({wide_label_right})"
    );
}

#[test]
fn outside_a_scope_field_rows_degrade_to_their_own_split() {
    let mut harness = Harness::new_ui(|ui| {
        theme::apply(ui.ctx(), Mode::Light);
        widgets::field_row(ui, "id", "value-a", None, false);
        widgets::field_row(ui, "a much longer label", "value-b", None, false);
    });
    harness.run();

    let ax = harness.get_by_label("value-a").rect().left();
    let bx = harness.get_by_label("value-b").rect().left();
    assert!(
        ax < bx,
        "without a scope each row splits after its own label (alignment-by-convention)"
    );
}

#[test]
fn every_primitive_renders_headlessly_in_both_modes() {
    for mode in [Mode::Light, Mode::Dark] {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(800.0, 1400.0))
            .build_ui(move |ui| {
                theme::apply(ui.ctx(), mode);

                widgets::pane_header(ui, Some(&icons::TABLE), "Steps");
                widgets::pane_header_with(ui, None, "Runs", |right| {
                    icons::X.show(right, 14.0, right.visuals().text_color());
                });

                align::align_scope(ui, "fields", |ui| {
                    widgets::field_row(
                        ui,
                        "address",
                        "example/things",
                        Some("where it lives"),
                        true,
                    );
                });

                // The pill is generic: the caller brings icon + label + role.
                widgets::status_pill(ui, &icons::CIRCLE_CHECK, "ok", Role::Success);
                widgets::status_pill(ui, &icons::CLOCK, "waiting", Role::Neutral);

                // One focus concept: ring drawn from token colour + geometry.
                let response = ui.button("focusable");
                widgets::focus_ring(ui, response.rect, meridian_egui::TOKENS.radius_control);
                widgets::focus_ring_for(ui, &response);

                let colours = widgets::slider_colours(mode);
                assert_ne!(colours.track, colours.thumb);

                // The components and compositions of the component review, each
                // called once; the look of each is its own test file's business.
                let mut text = "ranking".to_owned();

                let export = button(
                    ui,
                    Button::new("Export")
                        .icon(&icons::DOWNLOAD)
                        .keystroke("⌘E")
                        .weight(Weight::Primary),
                );
                text_field(
                    ui,
                    TextField::new(&mut text)
                        .unit("days")
                        .error("Enter a whole number of days."),
                );

                let stages = [
                    RibbonTab::new("Bring in").keystroke("B"),
                    RibbonTab::new("Shape"),
                ];
                ribbon_tabs(
                    ui,
                    RibbonTabs::new(&stages, Some(TabPick::Stage(1)))
                        .contextual(RibbonTab::new("Grid · ranking"))
                        .key_tips(true),
                );
                let commands = [
                    Button::new("Filter").icon(&icons::FILTER).keystroke("F"),
                    Button::new("Join")
                        .icon(&icons::ARROWS_JOIN)
                        .keystroke("J")
                        .unavailable("Join needs a second table."),
                ];
                let groups: [&[Button]; 1] = [&commands];
                ribbon_row(ui, RibbonRow::new(&groups));
                key_tip(ui, export.response.rect, "E");

                Popover::show_reason(
                    ui.ctx(),
                    "reason",
                    export.response.rect,
                    "Join needs a second table.",
                    Some("J"),
                );
                Popover::show_menu(
                    ui.ctx(),
                    "menu",
                    export.response.rect,
                    Anchor::Beside,
                    |ui| {
                        menu_item(ui, MenuItem::new("Sort ascending").keystroke("S"));
                        menu_separator(ui);
                        menu_item(ui, MenuItem::new("Show missing").checked(true));
                        menu_item(
                            ui,
                            MenuItem::new("Join on this column")
                                .unavailable("Join needs a second table.")
                                .cursor(true),
                        );
                    },
                );

                let options = [
                    SegmentedOption::new("Grid").keystroke("G"),
                    SegmentedOption::new("Model").unavailable("Nothing has run."),
                ];
                segmented(ui, Segmented::new(&options, 0));

                section(ui, Section::new("Fields", true).count(7), |ui| {
                    distribution_bar(
                        ui,
                        DistributionBar::new(
                            Bars::Top {
                                values: &[("cask", 1800), ("tap", 820)],
                                rest: 300,
                            },
                            12,
                            2932,
                        ),
                    );
                });

                let gutter = Gutter {
                    lanes: 1,
                    node: Some(0),
                    through: &[],
                    from_above: &[0],
                    to_below: &[0],
                };
                step_row(
                    ui,
                    StepRow::new(StepEntry::Step, gutter, StepMark::Running, "ranking")
                        .output("19,028 rows")
                        .timeline(
                            Timeline {
                                start: 0.2,
                                end: 0.6,
                            },
                            "1.3 s",
                        )
                        .focused(true)
                        .cursor(true),
                );
                step_mark(ui, StepMark::Failed, true);

                vertical_tab(
                    ui,
                    VerticalTab::new("ranking", "Chart")
                        .current(true)
                        .dirty(true),
                );
                run_status(
                    ui,
                    RunStatus::Running {
                        step: Some("join_installs"),
                        done: 3,
                        total: 7,
                    },
                );
                teaching_message(ui, "Fold group", "⌥F");

                // The picker row's unavailable state, through the picker.
                Picker::new(TwoRows).show(ui);
            });
        harness.run();

        harness.get_by_label("Steps");
        harness.get_by_label("ok");
        for label in [
            "Export",
            "Sort ascending",
            "Fields",
            "Run this step",
            "Grid · ranking",
        ] {
            harness.get_by_label(label);
        }
    }
}
