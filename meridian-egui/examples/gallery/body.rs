//! The gallery's drawing: every component and composition of the crate, in the
//! order of `components/sheet.html` and under its section numbers and titles.
//!
//! A state the component lets its caller pass (selected, open, cursor, focused,
//! unavailable, checked, collapsed, key tips) is drawn fixed, as the sheet draws
//! it. Hover, pressed and keyboard focus are the pointer's and the keys', so they
//! are live: point at a thing or tab to it to see them.
//!
//! Shared by `examples/gallery/main.rs`, which puts it in a window, and by
//! `tests/gallery.rs`, which renders it headlessly in both modes.

use egui::{Color32, Rect, RichText, Sense, Stroke, StrokeKind, Vec2};
use meridian_design::marks::{word, STEP_MARKS};
use meridian_design::semantic::semantic;
use meridian_design::spacing::{BAND_HEIGHT, ROW_GRID};
use meridian_design::Role;
use meridian_egui::{
    align, button, distribution_bar, icons, key_chip, key_tip, line_tabs, list_row, menu_item,
    menu_separator, overlay_frame, query_line, ribbon_row, ribbon_tabs, run_status, section,
    segmented, step_mark, step_row, teaching_message, text_field, theme, tooltip_for_action,
    vertical_tab, widgets, Anchor, Bars, Button, DistributionBar, Fold, Gutter, LineTabs, ListRow,
    MenuItem, MeridianUi, ModalChrome, ModalLayer, Mode, Notification, NotificationId,
    NotificationLayer, Picker, PickerDelegate, PickerOutcome, PickerRow, Popover, RibbonRow,
    RibbonTab, RibbonTabs, RowHeight, RunStatus, Section, Segmented, SegmentedOption, Severity,
    StepEntry, StepMark, StepRow, TabPick, TextField, Timeline, Toast, ToastLayer, VerticalTab,
    Weight,
};

/// Everything the gallery's live parts hold between frames. The components
/// hold nothing; this is the caller's half.
pub struct GalleryState {
    pub mode: Mode,
    name: String,
    filled: String,
    find: String,
    days: String,
    locked: String,
    dense: String,
    window: String,
    window_bad: String,
    query: String,
    line_tab: usize,
    ribbon: Option<TabPick>,
    ribbon_collapsed: bool,
    activity: usize,
    sections: [bool; 4],
    hovered_bin: Option<usize>,
    show_missing: bool,
    wrap_text: bool,
    step_focus: usize,
    step_cursor: usize,
    loads_folded: bool,
    place: usize,
    picker: Picker<Commands>,
    notifications: NotificationLayer,
    toasts: ToastLayer,
    modal: bool,
}

impl Default for GalleryState {
    fn default() -> Self {
        let mut picker = Picker::new(Commands::default());
        // The sheet draws the cursor on the unavailable row.
        picker.move_selection(1);
        Self {
            mode: Mode::Light,
            name: String::new(),
            filled: "ranking".to_owned(),
            find: String::new(),
            days: "30".to_owned(),
            locked: "analytics_30d".to_owned(),
            dense: "ranking".to_owned(),
            window: "30".to_owned(),
            window_bad: "thirty".to_owned(),
            query: String::new(),
            line_tab: 0,
            ribbon: Some(TabPick::Stage(1)),
            ribbon_collapsed: false,
            activity: 0,
            sections: [true, true, false, false],
            hovered_bin: None,
            show_missing: true,
            wrap_text: false,
            step_focus: 3,
            step_cursor: 4,
            loads_folded: true,
            place: 1,
            picker,
            notifications: NotificationLayer::new(),
            toasts: ToastLayer::new(),
            modal: false,
        }
    }
}

/// The picker's rows: one available command and one unavailable, as the sheet
/// draws them under section 9.
#[derive(Default)]
struct Commands {
    matches: Vec<usize>,
}

const COMMANDS: [(&str, &str, &str, Option<&str>); 2] = [
    ("Open a Protocol", "from a folder", "⌘O", None),
    ("Run this step", "", "⌘↩", Some("Nothing is focused.")),
];

impl PickerDelegate for Commands {
    fn update_query(&mut self, query: &str) {
        let q = query.to_lowercase();
        self.matches = (0..COMMANDS.len())
            .filter(|&i| COMMANDS[i].0.to_lowercase().contains(&q))
            .collect();
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn row(&self, index: usize) -> PickerRow {
        let (label, detail, key, reason) = COMMANDS[self.matches[index]];
        let row = PickerRow::new(label).keystroke(key);
        match reason {
            Some(reason) => row.unavailable(reason),
            None => row.detail(detail),
        }
    }

    fn confirm(&mut self, _index: Option<usize>, _query: &str) -> PickerOutcome {
        PickerOutcome::KeepOpen
    }

    fn placeholder(&self) -> String {
        "Find a command".to_owned()
    }
}

// ─── layout helpers ─────────────────────────────────────────────────────────

fn muted(ui: &egui::Ui) -> Color32 {
    theme::to_color32(semantic(ui.visuals().dark_mode).text.muted)
}

/// A section's heading: the sheet's number and title, then its note.
fn heading(ui: &mut egui::Ui, title: &str, note: &str) {
    let t = ui.tokens();
    ui.add_space(t.space[8]);
    ui.separator();
    ui.add_space(t.space[4]);
    ui.horizontal(|ui| {
        ui.label(RichText::new(title).strong());
        if !note.is_empty() {
            ui.label(RichText::new(note).small().color(muted(ui)));
        }
    });
    ui.add_space(t.space[4]);
}

/// A sub-heading inside a section, as the sheet's bold run-in labels.
fn subheading(ui: &mut egui::Ui, title: &str) {
    let t = ui.tokens();
    ui.add_space(t.space[4]);
    ui.label(RichText::new(title).strong().small());
    ui.add_space(t.space[2]);
}

/// One figure: `width` wide, its drawing over its caption.
fn figure<R>(
    ui: &mut egui::Ui,
    width: f32,
    caption: &str,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let t = ui.tokens();
    ui.vertical(|ui| {
        ui.set_width(width);
        // The gap between figures is the row's; inside a figure the theme's
        // own spacing holds.
        ui.spacing_mut().item_spacing = ui.ctx().global_style().spacing.item_spacing;
        let inner = body(ui);
        ui.add_space(t.space[2]);
        ui.label(RichText::new(caption).small().color(muted(ui)));
        ui.add_space(t.space[4]);
        inner
    })
    .inner
}

/// A row of figures, tops aligned.
fn figures(ui: &mut egui::Ui, body: impl FnOnce(&mut egui::Ui)) {
    let t = ui.tokens();
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = t.space[8];
        body(ui);
    });
}

/// A band of the title bar's height and fill with its divider along the foot,
/// with `body` centred in it, as the sheet draws the run status and the
/// teaching message.
fn band(ui: &mut egui::Ui, width: f32, body: impl FnOnce(&mut egui::Ui)) {
    let sem = semantic(ui.visuals().dark_mode);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, BAND_HEIGHT), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        0.0,
        theme::to_color32(sem.containers.title_bar_background),
    );
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, theme::to_color32(sem.borders.divider)),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    body(&mut child);
}

/// A status band, as the sheet draws the teaching message's: the row's height,
/// the status bar's fill and a subtle rule along its top.
fn status_band(ui: &mut egui::Ui, width: f32, body: impl FnOnce(&mut egui::Ui)) {
    let sem = semantic(ui.visuals().dark_mode);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, ROW_GRID), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        0.0,
        theme::to_color32(sem.containers.status_bar_background),
    );
    painter.hline(
        rect.x_range(),
        rect.top() + 0.5,
        Stroke::new(1.0, theme::to_color32(sem.borders.subtle)),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    body(&mut child);
}

// ─── the gallery ────────────────────────────────────────────────────────────

/// Draw the whole gallery into `ui`, applying the theme in `state.mode` first.
pub fn show(ui: &mut egui::Ui, state: &mut GalleryState) {
    theme::apply(ui.ctx(), state.mode);
    let t = ui.tokens();
    ui.spacing_mut().item_spacing.y = t.space[2];

    ui.horizontal(|ui| {
        ui.label(RichText::new("Meridian components").strong());
        ui.add_space(t.section_gap);
        let other = match state.mode {
            Mode::Light => "Dark",
            Mode::Dark => "Light",
        };
        if button(ui, Button::new(other)).clicked {
            state.mode = state.mode.toggled();
        }
        ui.label(
            RichText::new(
                "Selected, open, cursor, focused and unavailable are drawn fixed; \
                 hover, pressed and keyboard focus are live.",
            )
            .small()
            .color(muted(ui)),
        );
    });

    settled(ui, state);
    buttons(ui);
    text_fields(ui, state);
    ribbon(ui, state);
    popovers(ui);
    segmented_controls(ui, state);
    sections_and_bars(ui, state);
    menus(ui, state);
    steps(ui, state);
    vertical_tabs(ui, state);
    run_statuses(ui);
    together(ui, state);

    state.notifications.show(ui.ctx());
    state.toasts.tick(ui.input(|i| i.time));
    state.toasts.show(ui.ctx());
    if state.modal {
        let chrome = ModalChrome::new().title("A modal").enter_hint("run");
        let r = ModalLayer::show(ui.ctx(), "gallery-modal", &chrome, |ui| {
            ui.label("The one modal frame every overlay shares.");
        });
        if r.dismissed {
            state.modal = false;
        }
    }
}

/// 0 · What is already settled: the keycap, the list row and the line tabs,
/// then the crate's other settled pieces, which the sheet does not draw.
fn settled(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(
        ui,
        "0 · What is already settled",
        "drawn for comparison, not reopened",
    );
    figures(ui, |ui| {
        figure(ui, 40.0, "keycap", |ui| key_chip(ui, "⌘↩"));
        figure(ui, 40.0, "keycap", |ui| key_chip(ui, "Esc"));
        for (caption, cursor) in [("list row, rest; hover is live", false), ("cursor", true)] {
            figure(ui, 240.0, caption, |ui| {
                list_row(
                    ui,
                    ListRow::new(RowHeight::Grid).selected(cursor),
                    |ui, _| {
                        ui.label("Open a Protocol");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            key_chip(ui, "⌘O");
                        });
                    },
                );
            });
        }
    });
    figures(ui, |ui| {
        figure(ui, 360.0, "line tabs", |ui| {
            let names = ["Run log", "Problems", "Editor", "Versions"];
            if let Some(i) = line_tabs(ui, LineTabs::new(&names, state.line_tab)).clicked {
                state.line_tab = i;
            }
        });
    });

    subheading(ui, "Also in the crate");
    figures(ui, |ui| {
        figure(ui, 280.0, "pane header, field rows in one scope", |ui| {
            widgets::pane_header(ui, Some(&icons::TABLE), "ranking");
            align::align_scope(ui, "gallery-fields", |ui| {
                widgets::field_row(
                    ui,
                    "address",
                    "example/things",
                    Some("where it lives"),
                    true,
                );
                widgets::field_row(ui, "rows", "19,028", None, false);
            });
        });
        figure(ui, 180.0, "status pills", |ui| {
            widgets::status_pill(ui, &icons::CIRCLE_CHECK, "ok", Role::Success);
            widgets::status_pill(ui, &icons::CLOCK, "waiting", Role::Neutral);
            widgets::status_pill(ui, &icons::ALERT_TRIANGLE, "stale", Role::Warning);
        });
        figure(ui, 280.0, "query line", |ui| {
            query_line(ui, &mut state.query, "Search…");
        });
        figure(
            ui,
            260.0,
            "live: notification, toast, modal, tooltip",
            |ui| {
                if button(ui, Button::new("Raise a notification")).clicked {
                    state.notifications.raise(
                        Notification::new(
                            NotificationId::new("gallery"),
                            Severity::Warning,
                            "A source has changed",
                        )
                        .body("Run the Protocol again to bring its tables up to date."),
                    );
                }
                if button(ui, Button::new("Show a toast")).clicked {
                    state
                        .toasts
                        .push(Toast::new(Severity::Info, "Copied the address"));
                }
                if button(ui, Button::new("Open a modal")).clicked {
                    state.modal = true;
                }
                let r = button(ui, Button::new("Point at me")).response;
                tooltip_for_action(r, "Run the Protocol", Some("⌘↩"));
            },
        );
    });
    figures(ui, |ui| {
        figure(
            ui,
            480.0,
            "overlay frame: a modal's card, drawn in place",
            |ui| {
                overlay_frame(ui, &ModalChrome::new().title("Jump to asset"), |ui| {
                    ui.label("The card every modal and overlay draws inside.");
                });
            },
        );
    });
}

/// 1 · Button.
fn buttons(ui: &mut egui::Ui) {
    heading(
        ui,
        "1 · Button",
        "the ribbon command is this, laid out in a row",
    );
    let export = Button::new("Export").icon(&icons::DOWNLOAD).keystroke("⌘E");
    let run = Button::new("Run")
        .icon(&icons::PLAYER_PLAY)
        .keystroke("⌘↩")
        .weight(Weight::Primary);
    let filter = Button::new("Filter")
        .icon(&icons::FILTER)
        .keystroke("F")
        .weight(Weight::Quiet);
    let join = Button::new("Join")
        .icon(&icons::ARROWS_JOIN)
        .keystroke("J")
        .weight(Weight::Quiet);

    subheading(ui, "Default");
    figures(ui, |ui| {
        figure(ui, 110.0, "rest; hover, pressed, focus live", |ui| {
            button(ui, export);
        });
        figure(ui, 110.0, "label only", |ui| {
            button(ui, Button::new("Choose…"));
        });
        figure(ui, 80.0, "icon only", |ui| {
            button(ui, Button::icon_only(&icons::REFRESH, "Refresh"));
        });
        figure(ui, 110.0, "20 rung, no keycap", |ui| {
            button(ui, Button::new("Rename").dense());
        });
    });
    subheading(ui, "Primary");
    figures(ui, |ui| {
        figure(ui, 110.0, "rest", |ui| {
            button(ui, run);
        });
    });
    subheading(ui, "Quiet  the toolbar and ribbon weight");
    figures(ui, |ui| {
        figure(ui, 110.0, "rest", |ui| {
            button(ui, filter);
        });
        figure(ui, 110.0, "icon only", |ui| {
            button(
                ui,
                Button::icon_only(&icons::FILTER, "Filter").weight(Weight::Quiet),
            );
        });
    });
    subheading(ui, "Unavailable");
    let reason = "Join needs a second table.";
    figures(ui, |ui| {
        figure(ui, 110.0, "default", |ui| {
            button(ui, export.unavailable(reason));
        });
        figure(ui, 110.0, "quiet", |ui| {
            button(ui, join.unavailable(reason));
        });
        figure(ui, 110.0, "primary", |ui| {
            button(ui, run.unavailable("Nothing is focused."));
        });
    });
}

/// 2 · Text field.
fn text_fields(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(
        ui,
        "2 · Text field",
        "wraps egui's own text editing; has the keys and selection are live",
    );
    figures(ui, |ui| {
        figure(ui, 200.0, "empty, with placeholder", |ui| {
            text_field(ui, TextField::new(&mut state.name).placeholder("Name"));
        });
        figure(ui, 200.0, "filled", |ui| {
            text_field(ui, TextField::new(&mut state.filled));
        });
        figure(ui, 200.0, "leading icon", |ui| {
            text_field(
                ui,
                TextField::new(&mut state.find)
                    .icon(&icons::SEARCH)
                    .placeholder("Find a column"),
            );
        });
        figure(ui, 200.0, "unit", |ui| {
            text_field(ui, TextField::new(&mut state.days).unit("days"));
        });
    });
    figures(ui, |ui| {
        figure(ui, 200.0, "unavailable", |ui| {
            text_field(ui, TextField::new(&mut state.locked).unavailable());
        });
        figure(ui, 200.0, "20 rung", |ui| {
            text_field(ui, TextField::new(&mut state.dense).dense());
        });
    });
    figures(ui, |ui| {
        let window = |ui: &mut egui::Ui, field: TextField<'_>| {
            ui.horizontal_top(|ui| {
                ui.set_width(300.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(96.0, ROW_GRID),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.set_min_size(egui::vec2(96.0, ROW_GRID));
                        ui.label("Window");
                    },
                );
                ui.vertical(|ui| {
                    text_field(ui, field);
                });
            });
        };
        figure(ui, 300.0, "in a field row, with help", |ui| {
            window(
                ui,
                TextField::new(&mut state.window)
                    .unit("days")
                    .help("How far back installs are counted."),
            );
        });
        figure(ui, 300.0, "error", |ui| {
            window(
                ui,
                TextField::new(&mut state.window_bad)
                    .unit("days")
                    .error("Enter a whole number of days."),
            );
        });
    });
}

const STAGES: [&str; 4] = ["Bring in", "Shape", "Model", "Publish"];
const STAGE_KEYS: [&str; 4] = ["B", "S", "M", "P"];

fn stages() -> [RibbonTab<'static>; 4] {
    std::array::from_fn(|i| RibbonTab::new(STAGES[i]).keystroke(STAGE_KEYS[i]))
}

fn contextual() -> RibbonTab<'static> {
    RibbonTab::new("Grid · ranking").keystroke("G")
}

/// The Shape stage's commands, in three groups; Join is unavailable.
fn shape_commands() -> [Vec<Button<'static>>; 3] {
    [
        vec![
            Button::new("Filter").icon(&icons::FILTER).keystroke("F"),
            Button::new("Sort").icon(&icons::ARROWS_SORT).keystroke("S"),
            Button::new("Hide").icon(&icons::EYE_OFF).keystroke("H"),
        ],
        vec![
            Button::new("Derive")
                .icon(&icons::MATH_FUNCTION)
                .keystroke("D"),
            Button::new("Join")
                .icon(&icons::ARROWS_JOIN)
                .keystroke("J")
                .unavailable("Join needs a second table."),
            Button::new("Group").icon(&icons::SUM).keystroke("G"),
        ],
        vec![Button::new("Remove").icon(&icons::TRASH).keystroke("⌫")],
    ]
}

/// The contextual tab's commands.
fn grid_commands() -> [Vec<Button<'static>>; 2] {
    [
        vec![
            Button::new("Columns")
                .icon(&icons::LAYOUT_COLUMNS)
                .keystroke("C"),
            Button::new("Rename")
                .icon(&icons::CURSOR_TEXT)
                .keystroke("F2"),
        ],
        vec![
            Button::new("Zoom in").icon(&icons::ZOOM_IN).keystroke("+"),
            Button::new("Zoom out")
                .icon(&icons::ZOOM_OUT)
                .keystroke("-"),
        ],
    ]
}

fn row_of<'a>(groups: &'a [Vec<Button<'static>>]) -> Vec<&'a [Button<'static>]> {
    groups.iter().map(Vec::as_slice).collect()
}

/// 3 · Ribbon tabs and 4 · Ribbon row.
fn ribbon(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(ui, "3 · Ribbon tabs  and  4 · Ribbon row", "");
    let stages = stages();
    let shape = shape_commands();
    let grid = grid_commands();
    let width = 640.0;

    figures(ui, |ui| {
        figure(
            ui,
            width,
            "live: click a tab to open it, double-click to collapse; Join unavailable",
            |ui| {
                let r = ribbon_tabs(
                    ui,
                    RibbonTabs::new(&stages, state.ribbon)
                        .contextual(contextual())
                        .collapsed(state.ribbon_collapsed),
                );
                if let Some(pick) = r.clicked {
                    state.ribbon = Some(pick);
                }
                if r.double_clicked.is_some() {
                    state.ribbon_collapsed = !state.ribbon_collapsed;
                }
                if !state.ribbon_collapsed {
                    match state.ribbon {
                        Some(TabPick::Contextual) => {
                            ribbon_row(ui, RibbonRow::new(&row_of(&grid)));
                        }
                        _ => {
                            ribbon_row(ui, RibbonRow::new(&row_of(&shape)));
                        }
                    }
                }
            },
        );
    });
    figures(ui, |ui| {
        figure(ui, width, "open on Shape", |ui| {
            ribbon_tabs(
                ui,
                RibbonTabs::new(&stages, Some(TabPick::Stage(1))).contextual(contextual()),
            );
            ribbon_row(ui, RibbonRow::new(&row_of(&shape)));
        });
    });
    figures(ui, |ui| {
        figure(ui, width, "open on the contextual tab", |ui| {
            ribbon_tabs(
                ui,
                RibbonTabs::new(&stages, Some(TabPick::Contextual)).contextual(contextual()),
            );
            ribbon_row(ui, RibbonRow::new(&row_of(&grid)));
        });
    });
    figures(ui, |ui| {
        figure(
            ui,
            width,
            "collapsed: Shape is selected and not open",
            |ui| {
                ribbon_tabs(
                    ui,
                    RibbonTabs::new(&stages, Some(TabPick::Stage(1)))
                        .contextual(contextual())
                        .collapsed(true),
                );
            },
        );
    });

    subheading(ui, "The row as it narrows");
    for (w, caption) in [
        (520.0, "narrower: labels dropped from the trailing end"),
        (400.0, "narrowest: every label dropped"),
        (240.0, "narrower than its icons and keys: clipped"),
    ] {
        figures(ui, |ui| {
            figure(ui, w, caption, |ui| {
                ribbon_row(ui, RibbonRow::new(&row_of(&shape)));
            });
        });
    }

    subheading(ui, "Key tips on");
    figures(ui, |ui| {
        figure(ui, width, "a tip on each tab", |ui| {
            ribbon_tabs(
                ui,
                RibbonTabs::new(&stages, Some(TabPick::Stage(1)))
                    .contextual(contextual())
                    .key_tips(true),
            );
            ribbon_row(ui, RibbonRow::new(&row_of(&shape)));
        });
    });
    ui.add_space(ui.tokens().space[4]);
    figures(ui, |ui| {
        figure(
            ui,
            width,
            "a tab chosen: a tip on each command, in place of its own keycap",
            |ui| {
                ribbon_tabs(
                    ui,
                    RibbonTabs::new(&stages, Some(TabPick::Stage(1))).contextual(contextual()),
                );
                ribbon_row(ui, RibbonRow::new(&row_of(&shape)).key_tips(true));
            },
        );
        figure(ui, 120.0, "key_tip over any target", |ui| {
            let r = button(ui, Button::new("Choose…"));
            key_tip(ui, r.response.rect, "O");
        });
    });
    ui.add_space(ui.tokens().space[4]);
}

/// Whether a popover's anchor is on screen, so a scrolled-away anchor does not
/// leave its card floating over the rest of the gallery.
fn on_screen(ui: &egui::Ui, anchor: Rect) -> bool {
    ui.clip_rect().intersects(anchor)
}

/// 5 · Popover.
fn popovers(ui: &mut egui::Ui) {
    heading(
        ui,
        "5 · Popover",
        "the reason popover and the anchored layer as one component",
    );
    let space = ui.tokens().space[9];
    figures(ui, |ui| {
        figure(ui, 280.0, "reason, 240, below its anchor", |ui| {
            let r = button(
                ui,
                Button::new("Join")
                    .icon(&icons::ARROWS_JOIN)
                    .keystroke("J")
                    .weight(Weight::Quiet)
                    .unavailable("Join needs a second table."),
            );
            if on_screen(ui, r.response.rect) {
                Popover::show_reason(
                    ui.ctx(),
                    "gallery-reason-key",
                    r.response.rect,
                    "Join needs a second table. Bring one in first.",
                    Some("J"),
                );
            }
            ui.add_space(space * 2.0);
        });
        figure(ui, 280.0, "reason, no keycap", |ui| {
            let r = button(
                ui,
                Button::new("Publish")
                    .icon(&icons::UPLOAD)
                    .keystroke("⌘P")
                    .weight(Weight::Quiet)
                    .unavailable("Nothing has run yet."),
            );
            if on_screen(ui, r.response.rect) {
                Popover::show_reason(
                    ui.ctx(),
                    "gallery-reason-plain",
                    r.response.rect,
                    "Nothing has run yet, so there is nothing to publish.",
                    None,
                );
            }
            ui.add_space(space * 2.0);
        });
        figure(
            ui,
            360.0,
            "any body: the collapsed ribbon's row, dropped from its tab",
            |ui| {
                let r = button(ui, Button::new("Shape").weight(Weight::Quiet));
                if on_screen(ui, r.response.rect) {
                    Popover::show(
                        ui.ctx(),
                        "gallery-general",
                        r.response.rect,
                        Anchor::Below,
                        |ui| {
                            let groups = [
                                vec![
                                    Button::new("Filter").icon(&icons::FILTER).keystroke("F"),
                                    Button::new("Sort").icon(&icons::ARROWS_SORT).keystroke("S"),
                                ],
                                vec![Button::new("Derive")
                                    .icon(&icons::MATH_FUNCTION)
                                    .keystroke("D")],
                            ];
                            ui.set_width(280.0);
                            ribbon_row(ui, RibbonRow::new(&row_of(&groups)));
                        },
                    );
                }
                ui.add_space(space * 2.0);
            },
        );
    });
}

/// 6 · Segmented control.
fn segmented_controls(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(ui, "6 · Segmented control", "the activity switcher");
    let keyed = [
        SegmentedOption::new("Grid").keystroke("G"),
        SegmentedOption::new("Chart").keystroke("C"),
        SegmentedOption::new("Model").keystroke("M"),
    ];
    let with_unavailable = [
        SegmentedOption::new("Grid").keystroke("G"),
        SegmentedOption::new("Chart").keystroke("C"),
        SegmentedOption::new("Model")
            .keystroke("M")
            .unavailable("Model needs a Protocol that has run."),
    ];
    let plain = [SegmentedOption::new("Grid"), SegmentedOption::new("Chart")];
    let pair = [
        SegmentedOption::new("Grid").keystroke("G"),
        SegmentedOption::new("Chart").keystroke("C"),
    ];

    figures(ui, |ui| {
        figure(ui, 220.0, "live: click to select", |ui| {
            if let Some(i) = segmented(ui, Segmented::new(&keyed, state.activity)).clicked {
                state.activity = i;
            }
        });
        figure(ui, 220.0, "Grid selected", |ui| {
            segmented(ui, Segmented::new(&keyed, 0));
        });
        figure(ui, 220.0, "Model unavailable", |ui| {
            segmented(ui, Segmented::new(&with_unavailable, 0));
        });
        figure(ui, 160.0, "20 rung, no keycaps", |ui| {
            segmented(ui, Segmented::new(&plain, 0));
        });
    });
    figures(ui, |ui| {
        figure(ui, 520.0, "in a pane header", |ui| {
            widgets::pane_header_with(ui, Some(&icons::TABLE), "ranking", |right| {
                segmented(right, Segmented::new(&pair, 0));
                right.label(RichText::new("19,028 rows").color(muted(right)));
            });
        });
    });
}

const BINS: [u64; 12] = [1204, 880, 610, 402, 330, 214, 160, 120, 90, 70, 52, 40];
const TOP: [(&str, u64); 5] = [
    ("cask", 1800),
    ("formula", 1500),
    ("tap", 820),
    ("bottle", 460),
    ("service", 260),
];

/// 7 · Section and 8 · Distribution bar.
fn sections_and_bars(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(
        ui,
        "7 · Section with a count  and  8 · Distribution bar",
        "",
    );
    figures(ui, |ui| {
        figure(
            ui,
            280.0,
            "live: click a header to open or close it",
            |ui| {
                let fields = |ui: &mut egui::Ui, id: &str, rows: &[(&str, &str)]| {
                    align::align_scope(ui, id, |ui| {
                        for (k, v) in rows {
                            widgets::field_row(ui, k, v, None, false);
                        }
                    });
                };
                let r = section(ui, Section::new("Column", state.sections[0]), |ui| {
                    fields(
                        ui,
                        "gallery-column",
                        &[
                            ("Name", "install_count"),
                            ("Type", "whole number"),
                            ("Missing", "412 of 19,028"),
                        ],
                    );
                    let d =
                        distribution_bar(ui, DistributionBar::new(Bars::Bins(&BINS), 412, 19_028));
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("3,000 to 4,000").small());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new("1,204 rows").small().color(muted(ui)));
                        });
                    });
                    d.hovered
                });
                state.hovered_bin = r.inner.flatten();
                state.sections[0] ^= r.toggled;
                let r = section(
                    ui,
                    Section::new("Fields", state.sections[1]).count(7),
                    |ui| {
                        fields(
                            ui,
                            "gallery-fields",
                            &[("x", "install_count"), ("y", "growth_30d")],
                        )
                    },
                );
                state.sections[1] ^= r.toggled;
                let r = section(
                    ui,
                    Section::new("Parameters", state.sections[2]).count(2),
                    |ui| fields(ui, "gallery-parameters", &[("bins", "12")]),
                );
                state.sections[2] ^= r.toggled;
                let r = section(ui, Section::new("Step", state.sections[3]), |ui| {
                    fields(ui, "gallery-step", &[("name", "ranking")])
                });
                state.sections[3] ^= r.toggled;
            },
        );
        figure(ui, 280.0, "closed, with a count", |ui| {
            section(ui, Section::new("Fields", false).count(7), |_| {});
        });
    });

    subheading(ui, "Distribution bar");
    let hovered = match state.hovered_bin {
        Some(i) => format!("the column's bar under the pointer: {i}"),
        None => "point at the column's bars: the one under the pointer is reported".to_owned(),
    };
    ui.label(RichText::new(hovered).small().color(muted(ui)));
    figures(ui, |ui| {
        figure(ui, 240.0, "a number: twelve bins, then the missing", |ui| {
            distribution_bar(ui, DistributionBar::new(Bars::Bins(&BINS), 412, 19_028));
        });
        figure(
            ui,
            240.0,
            "a category: top five, the others, the missing (none)",
            |ui| {
                distribution_bar(
                    ui,
                    DistributionBar::new(
                        Bars::Top {
                            values: &TOP,
                            rest: 570,
                        },
                        0,
                        5_410,
                    ),
                );
            },
        );
        figure(ui, 240.0, "at the 20 rung, for a column header", |ui| {
            distribution_bar(
                ui,
                DistributionBar::new(Bars::Bins(&BINS), 412, 19_028).dense(),
            );
        });
    });
}

/// 9 · Menu item, and the picker row's unavailable state.
fn menus(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(ui, "9 · Menu item", "");
    let t = ui.tokens();
    figures(ui, |ui| {
        figure(
            ui,
            520.0,
            "rest; hover is live; cursor; checked; submenu; unavailable; unavailable with the cursor",
            |ui| {
                let anchor = button(
                    ui,
                    Button::new("Column")
                        .icon(&icons::CHEVRON_DOWN)
                        .weight(Weight::Quiet),
                )
                .response
                .rect;
                if on_screen(ui, anchor) {
                    let encode = Popover::show_menu(
                        ui.ctx(),
                        "gallery-menu",
                        anchor,
                        Anchor::Below,
                        |ui| {
                            menu_item(ui, MenuItem::new("Sort ascending").checked(false).keystroke("S"));
                            menu_item(ui, MenuItem::new("Sort descending").checked(false).keystroke("⇧S"));
                            menu_item(
                                ui,
                                MenuItem::new("Filter to this value").checked(false)
                                    .keystroke("F")
                                    .cursor(true),
                            );
                            menu_separator(ui);
                            if menu_item(
                                ui,
                                MenuItem::new("Show missing").checked(state.show_missing),
                            )
                            .clicked
                            {
                                state.show_missing = !state.show_missing;
                            }
                            if menu_item(ui, MenuItem::new("Wrap text").checked(state.wrap_text))
                                .clicked
                            {
                                state.wrap_text = !state.wrap_text;
                            }
                            menu_separator(ui);
                            let encode =
                                menu_item(ui, MenuItem::new("Encode as").checked(false).submenu()).response.rect;
                            menu_item(
                                ui,
                                MenuItem::new("Join on this column").checked(false)
                                    .keystroke("J")
                                    .unavailable("Join needs a second table."),
                            );
                            menu_item(
                                ui,
                                MenuItem::new("Split by delimiter").checked(false)
                                    .keystroke("⌥S")
                                    .unavailable("Only a text column splits.")
                                    .cursor(true),
                            );
                            encode
                        },
                    );
                    Popover::show_menu(
                        ui.ctx(),
                        "gallery-submenu",
                        encode.inner,
                        Anchor::Beside,
                        |ui| {
                            menu_item(ui, MenuItem::new("Bars").checked(true));
                            menu_item(ui, MenuItem::new("Points").checked(false));
                            menu_item(ui, MenuItem::new("Lines").checked(false));
                        },
                    );
                }
                ui.add_space(10.0 * ROW_GRID + 2.0 * (1.0 + 2.0 * t.space[2]) + 2.0 * t.space[2]);
            },
        );
    });
    figures(ui, |ui| {
        figure(
            ui,
            360.0,
            "the picker row's unavailable state: the reason is its detail (live)",
            |ui| {
                state.picker.show(ui);
            },
        );
    });
}

/// One row of the sheet's spine: its entry, label, mark and gutter.
struct Spine {
    entry: StepEntry,
    label: &'static str,
    mark: StepMark,
    output: &'static str,
    timeline: Option<(f32, f32, &'static str)>,
    fold: Option<Fold>,
    node: u8,
    through: &'static [u8],
    from_above: &'static [u8],
    to_below: &'static [u8],
}

/// The sheet's list, row for row: the loads group folded, then the steps and
/// tables of one Protocol, with `rising` on a second lane.
const SPINE: [Spine; 8] = [
    Spine {
        entry: StepEntry::Step,
        label: "loads",
        mark: StepMark::Ok,
        output: "3 tables",
        timeline: Some((0.0, 0.28, "1.9 s")),
        fold: Some(Fold::Folded { members: 3 }),
        node: 0,
        through: &[],
        from_above: &[],
        to_below: &[0, 1],
    },
    Spine {
        entry: StepEntry::Step,
        label: "join_installs",
        mark: StepMark::Ok,
        output: "19,028 rows",
        timeline: Some((0.28, 0.52, "1.6 s")),
        fold: None,
        node: 0,
        through: &[1],
        from_above: &[0],
        to_below: &[0],
    },
    Spine {
        entry: StepEntry::Table,
        label: "installs",
        mark: StepMark::Ok,
        output: "19,028 rows",
        timeline: None,
        fold: None,
        node: 0,
        through: &[1],
        from_above: &[0],
        to_below: &[0],
    },
    Spine {
        entry: StepEntry::Step,
        label: "ranking",
        mark: StepMark::Ok,
        output: "19,028 rows",
        timeline: Some((0.52, 0.71, "1.3 s")),
        fold: None,
        node: 0,
        through: &[1],
        from_above: &[0],
        to_below: &[0],
    },
    Spine {
        entry: StepEntry::Step,
        label: "rising",
        mark: StepMark::Failed,
        output: "no rows",
        timeline: Some((0.52, 0.6, "0.5 s")),
        fold: None,
        node: 1,
        through: &[0],
        from_above: &[1],
        to_below: &[],
    },
    Spine {
        entry: StepEntry::Step,
        label: "publish",
        mark: StepMark::Stale,
        output: "2 tables",
        timeline: Some((0.71, 0.95, "1.6 s")),
        fold: Some(Fold::Unfolded),
        node: 0,
        through: &[],
        from_above: &[0],
        to_below: &[0],
    },
    Spine {
        entry: StepEntry::Step,
        label: "top_100",
        mark: StepMark::Running,
        output: "",
        timeline: Some((0.71, 0.86, "")),
        fold: None,
        node: 0,
        through: &[],
        from_above: &[0],
        to_below: &[0],
    },
    Spine {
        entry: StepEntry::Table,
        label: "export_csv",
        mark: StepMark::NotRun,
        output: "",
        timeline: None,
        fold: None,
        node: 0,
        through: &[],
        from_above: &[0],
        to_below: &[],
    },
];

fn spine_row(s: &Spine, folded: bool) -> StepRow<'static> {
    let gutter = Gutter {
        lanes: 2,
        node: Some(s.node),
        through: s.through,
        from_above: s.from_above,
        to_below: s.to_below,
    };
    let mut row = StepRow::new(s.entry, gutter, s.mark, s.label);
    if !s.output.is_empty() {
        row = row.output(s.output);
    }
    if let Some((start, end, duration)) = s.timeline {
        row = row.timeline(Timeline { start, end }, duration);
    }
    match s.fold {
        Some(Fold::Folded { .. }) if !folded => row = row.fold(Fold::Unfolded),
        Some(fold) => row = row.fold(fold),
        None => {}
    }
    row
}

/// 10 · Step row, its state marks and its thin form.
fn steps(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(
        ui,
        "10 · Step row",
        "the spine row, with its state mark and its thin form",
    );
    subheading(ui, "State marks");
    figures(ui, |ui| {
        for mark in STEP_MARKS {
            figure(ui, 56.0, word(mark), |ui| {
                step_mark(ui, mark, false);
            });
        }
    });

    figures(ui, |ui| {
        figure(
            ui,
            520.0,
            "live: click a row to focus it, the chevron folds; ranking focused, rising has the cursor",
            |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for (i, s) in SPINE.iter().enumerate() {
                    let r = step_row(
                        ui,
                        spine_row(s, state.loads_folded)
                            .focused(i == state.step_focus)
                            .cursor(i == state.step_cursor),
                    );
                    if r.clicked {
                        state.step_focus = i;
                        state.step_cursor = i;
                    }
                    if r.fold_clicked && i == 0 {
                        state.loads_folded = !state.loads_folded;
                    }
                }
            },
        );
    });
    figures(ui, |ui| {
        figure(
            ui,
            520.0,
            "rest · focused · cursor · focused with the cursor (hover and pressed are live)",
            |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let s = &SPINE[3];
                for (focused, cursor) in
                    [(false, false), (true, false), (false, true), (true, true)]
                {
                    step_row(ui, spine_row(s, true).focused(focused).cursor(cursor));
                }
            },
        );
    });
    figures(ui, |ui| {
        figure(ui, 120.0, "the thin form; ranking focused", |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            for (i, s) in SPINE.iter().enumerate() {
                step_mark(ui, s.mark, i == 3);
            }
        });
    });
}

/// A rail of vertical tabs beside a canvas, `width` wide, the rail 200 of it:
/// the sidebar's fill and rule on the rail, the raised surface and the subtle
/// rule on the canvas, whose leading rule is the one the current place covers.
fn rail(ui: &mut egui::Ui, width: f32, tabs: &[(&str, &str, bool, bool)]) -> Option<(usize, bool)> {
    let sem = semantic(ui.visuals().dark_mode);
    let c = theme::to_color32;
    let rail_w = 200.0;
    let h = tabs.len() as f32 * ROW_GRID + 2.0;
    let (outer, _) = ui.allocate_exact_size(egui::vec2(width, h), Sense::hover());
    let rail = Rect::from_min_size(outer.min, egui::vec2(rail_w, h));
    let canvas = Rect::from_min_max(egui::pos2(rail.right(), outer.top()), outer.max);
    let painter = ui.painter();
    painter.rect_filled(canvas, 0.0, c(sem.surfaces.raised));
    painter.rect_stroke(
        canvas,
        0.0,
        Stroke::new(1.0, c(sem.borders.subtle)),
        StrokeKind::Inside,
    );
    painter.text(
        canvas.center(),
        egui::Align2::CENTER_CENTER,
        "canvas",
        egui::TextStyle::Body.resolve(ui.style()),
        c(sem.text.muted),
    );
    painter.rect_filled(rail, 0.0, c(sem.surfaces.sidebar));
    let edge = Stroke::new(1.0, c(sem.containers.sidebar_border));
    painter.hline(rail.x_range(), rail.top() + 0.5, edge);
    painter.hline(rail.x_range(), rail.bottom() - 0.5, edge);
    painter.vline(rail.left() + 0.5, rail.y_range(), edge);

    let inner = Rect::from_min_max(
        rail.min + Vec2::splat(1.0),
        egui::pos2(rail.right(), rail.bottom() - 1.0),
    );
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner));
    child.spacing_mut().item_spacing.y = 0.0;
    let mut out = None;
    for (i, (node, activity, current, dirty)) in tabs.iter().enumerate() {
        let r = vertical_tab(
            &mut child,
            VerticalTab::new(node, activity)
                .current(*current)
                .dirty(*dirty),
        );
        if r.close_clicked {
            out = Some((i, true));
        } else if r.clicked {
            out = Some((i, false));
        }
    }
    out
}

/// 11 · Vertical tab.
fn vertical_tabs(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(ui, "11 · Vertical tab", "an open place");
    let places = [
        ("ranking", "Grid", false),
        ("ranking", "Chart", true),
        ("rising", "Grid", false),
        ("join_installs", "Model", true),
        ("analytics_30d", "Grid", false),
    ];
    figures(ui, |ui| {
        figure(
            ui,
            360.0,
            "live: click to make a place current; the close control shows under the pointer, \
             on the current place and with the keys",
            |ui| {
                let tabs: Vec<_> = places
                    .iter()
                    .enumerate()
                    .map(|(i, (n, a, d))| (*n, *a, i == state.place, *d))
                    .collect();
                if let Some((i, close)) = rail(ui, 360.0, &tabs) {
                    if !close {
                        state.place = i;
                    }
                }
            },
        );
        figure(
            ui,
            360.0,
            "rest · current, unsaved · rest · unsaved · rest",
            |ui| {
                let tabs: Vec<_> = places
                    .iter()
                    .enumerate()
                    .map(|(i, (n, a, d))| (*n, *a, i == 1, *d))
                    .collect();
                rail(ui, 360.0, &tabs);
            },
        );
    });
}

/// 12 · Run status, and the teaching message.
fn run_statuses(ui: &mut egui::Ui) {
    heading(
        ui,
        "12 · Run status",
        "the activity area in the title band's centre; hover and focus are live",
    );
    let width = 640.0;
    let cases = [
        (
            "running",
            RunStatus::Running {
                step: Some("join_installs"),
                done: 3,
                total: 7,
            },
        ),
        (
            "ran, fresh",
            RunStatus::Ran {
                when: "Ran 22 Nov, 14:02",
                age: "3 hours ago",
                mark: StepMark::Ok,
            },
        ),
        (
            "ran, stale",
            RunStatus::Ran {
                when: "Ran 19 Nov, 09:40",
                age: "3 days ago · a source has changed",
                mark: StepMark::Stale,
            },
        ),
        (
            "failed",
            RunStatus::Ran {
                when: "Failed 22 Nov, 14:02",
                age: "at join_installs",
                mark: StepMark::Failed,
            },
        ),
        (
            "not run",
            RunStatus::NotRun {
                label: "Not run yet",
            },
        ),
    ];
    for (caption, status) in cases {
        figures(ui, |ui| {
            figure(ui, width, caption, |ui| {
                band(ui, width, |ui| {
                    // Centred along the band: measured on a pass that is not
                    // painted, then drawn at the middle.
                    let mut measured = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(ui.max_rect())
                            .layout(egui::Layout::left_to_right(egui::Align::Center))
                            .invisible()
                            .sense(Sense::hover()),
                    );
                    let w = run_status(&mut measured, status).rect.width();
                    ui.add_space(((ui.available_width() - w) / 2.0).max(0.0));
                    run_status(ui, status);
                });
            });
        });
    }

    subheading(ui, "The teaching message");
    figures(ui, |ui| {
        figure(ui, width, "in a status band at the 24 rung", |ui| {
            status_band(ui, width, |ui| {
                ui.add_space(ui.tokens().space[4]);
                teaching_message(ui, "Fold group", "⌥F");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(ui.tokens().space[4]);
                    let secondary = semantic(ui.visuals().dark_mode).text.secondary;
                    ui.label(RichText::new("19,028 rows").color(theme::to_color32(secondary)));
                });
            });
        });
    });
}

/// The three treatments together, with the line tabs; then one row of labels
/// with and without descenders, to show they share one baseline.
fn together(ui: &mut egui::Ui, state: &mut GalleryState) {
    heading(
        ui,
        "The three treatments together",
        "with the line tabs that exist today",
    );
    figures(ui, |ui| {
        figure(ui, 240.0, "line tabs: a bar under the word", |ui| {
            line_tabs(ui, LineTabs::new(&["Run log", "Problems", "Editor"], 0));
        });
        figure(ui, 240.0, "ribbon tabs: a cell joined to its row", |ui| {
            let stages = [
                RibbonTab::new("Bring in"),
                RibbonTab::new("Shape"),
                RibbonTab::new("Model"),
            ];
            ribbon_tabs(ui, RibbonTabs::new(&stages, Some(TabPick::Stage(1))));
        });
    });
    figures(ui, |ui| {
        figure(
            ui,
            240.0,
            "segmented control: a wash in a ruled box",
            |ui| {
                let options = [
                    SegmentedOption::new("Grid").keystroke("G"),
                    SegmentedOption::new("Chart").keystroke("C"),
                ];
                segmented(ui, Segmented::new(&options, 0));
            },
        );
        figure(
            ui,
            260.0,
            "vertical tabs: a cell joined to the canvas",
            |ui| {
                rail(
                    ui,
                    260.0,
                    &[
                        ("ranking", "Grid", false, false),
                        ("ranking", "Chart", true, false),
                        ("rising", "Grid", false, false),
                    ],
                );
            },
        );
    });

    subheading(ui, "One baseline");
    figures(ui, |ui| {
        figure(
            ui,
            900.0,
            "labels with and without descenders share one baseline, cap height centred",
            |ui| {
                ui.horizontal(|ui| {
                    ui.allocate_ui(egui::vec2(380.0, ROW_GRID), |ui| {
                        let names = ["Log", "Quality", "Rows", "Editor", "Versions"];
                        if let Some(i) =
                            line_tabs(ui, LineTabs::new(&names, state.line_tab.min(4))).clicked
                        {
                            state.line_tab = i;
                        }
                    });
                    button(ui, Button::new("Log"));
                    button(ui, Button::new("Rows"));
                    button(ui, Button::new("Log").weight(Weight::Quiet));
                    button(ui, Button::new("Rows").weight(Weight::Primary));
                });
            },
        );
    });
}
