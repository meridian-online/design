//! A throwaway render vehicle for one open question about the status pill.
//!
//! **Nothing in this file is meant to survive.** It exists to produce a
//! picture: two candidate horizontal geometries for the status pill, drawn
//! beside the one that ships, so the choice between them can be made by
//! looking rather than by arithmetic. Once the choice is made, the winning
//! geometry is cut into `widgets.rs` and `key_chip.rs` properly and this file
//! is deleted.
//!
//! # The question
//!
//! [`meridian_egui::widgets::status_pill`] spends `space[2]` (4.0) on each
//! outer edge of the capsule and `icon_label_gap` = `SPACE_3` (6.0) between
//! the icon and the label. The group is therefore *looser in its middle than
//! it is inset from its container* — the rhythm runs 4 / 6 / 4 outward-in
//! instead of settling. `key_chip` reaches for `space[2]` the same way, so
//! the two chip primitives share whatever answer is chosen.
//!
//! Two ways to settle it, and they are both constants:
//!
//! | row | outer inset | icon–label gap | what it costs |
//! |---|---|---|---|
//! | today | `space[2]` 4.0 | `SPACE_3` 6.0 | the inversion |
//! | arm 1 | `SPACE_3` 6.0 | `SPACE_3` 6.0 | every pill and chip grows |
//! | arm 2 | `space[2]` 4.0 | `space[2]` 4.0 | the pill leaves `ICON_LABEL_GAP` |
//!
//! Because both are constants, one frame can hold all three: [`pill_at`] and
//! [`chip_at`] are parameterised copies of the two shipping primitives that
//! take the inset and the gap as arguments. They are copies rather than a
//! refactor precisely so that the shipping bodies are untouched while the
//! picture is taken.
//!
//! # The vertical axis is rendered here, not fixed here
//!
//! The pill also centres its icon on the icon's own symmetric box while
//! centring the label galley on its full line box *including descent*, so a
//! label with a descender sits low and one without does not. That is a
//! separate defect on a separate axis; the copies below reproduce it exactly
//! rather than correcting it, and the specimen labels deliberately include
//! three descenders and one word without, so it is visible in the same frame.
//!
//! # Running it
//!
//! [`the_today_row_is_the_pill_that_ships_and_each_arm_moves_by_its_token_delta`]
//! runs on the CPU tessellation path and is an ordinary test. The render is
//! `#[ignore]`d because it needs a GPU adapter through wgpu, which a headless
//! CI runner has not got:
//!
//! ```text
//! cargo test -p meridian-egui --test pill_geometry_montage -- --ignored --nocapture
//! ```
//!
//! It writes `throwaway-pill-montage/pill-geometry-{light,dark}.png` at the
//! repo root, at 2.0 pixels per point — the scale the consuming app's pill
//! snapshots use, so the two are directly comparable.

use std::sync::Arc;

use egui::{Response, Sense, StrokeKind};
use egui_kittest::Harness;
use meridian_design::control;
use meridian_design::semantic::{semantic, Role};
use meridian_design::typography::{CHART_LABEL_SIZE, UI_SIZE};
use meridian_egui::{icons, key_chip, theme, widgets, Icon, MeridianUi, Mode, TOKENS};

/// The scale the consuming app photographs its pills at. A montage at any
/// other scale cannot be laid beside those images.
const PIXELS_PER_POINT: f32 = 2.0;

/// The montage frame, in points.
const FRAME: (f32, f32) = (660.0, 300.0);

// ---------------------------------------------------------------------------
// The three geometries.
// ---------------------------------------------------------------------------

/// One candidate horizontal geometry: what the capsule spends on each outer
/// edge, and what it spends between the icon and the label.
#[derive(Clone, Copy)]
struct Arm {
    /// Short key used in the measurement report and the row caption.
    key: &'static str,
    /// The capsule's outer inset, both edges.
    pad_x: f32,
    /// The gap between the icon and the label.
    gap: f32,
}

/// The three rows, read off the ladder rather than written as literals — a
/// row that disagreed with the token it claims to spend would make the whole
/// picture a lie.
fn arms() -> [Arm; 3] {
    let space = TOKENS.space;
    [
        Arm {
            key: "today",
            pad_x: space[2],
            gap: TOKENS.icon_label_gap,
        },
        Arm {
            key: "arm 1",
            pad_x: TOKENS.icon_label_gap,
            gap: TOKENS.icon_label_gap,
        },
        Arm {
            key: "arm 2",
            pad_x: space[2],
            gap: space[2],
        },
    ]
}

/// One pill specimen. The four the consuming app's gallery draws, which is
/// why the set is what it is: three of the four labels carry a descender and
/// one does not, so the vertical defect and the horizontal question are both
/// legible in one row.
struct Specimen {
    label: &'static str,
    icon: &'static Icon,
    role: Role,
}

const SPECIMENS: [Specimen; 4] = [
    Specimen {
        label: "ready",
        icon: &icons::CHECK,
        role: Role::Accent,
    },
    Specimen {
        label: "ok",
        icon: &icons::CIRCLE_CHECK,
        role: Role::Success,
    },
    Specimen {
        label: "waiting",
        icon: &icons::CLOCK,
        role: Role::Neutral,
    },
    Specimen {
        label: "failing",
        icon: &icons::ALERT_TRIANGLE,
        role: Role::Danger,
    },
];

/// The keycap specimens. The chip is in the montage because it reaches for
/// the same constant the pill's outer inset does: it moves under arm 1 and
/// stands still under arm 2, so half of what the choice changes would be
/// invisible without it.
const KEYSTROKES: [&str; 2] = ["Esc", "Ctrl+K"];

// ---------------------------------------------------------------------------
// Parameterised copies of the two shipping primitives.
//
// Kept line-for-line faithful to `widgets::status_pill` and `key_chip::
// key_chip` apart from the two arguments. `the_today_row_is_the_pill_that_
// ships_and_each_arm_moves_by_its_token_delta` is what holds them faithful.
// ---------------------------------------------------------------------------

/// The pill's label galley, laid out exactly as the shipping pill lays it out.
fn pill_galley(ui: &egui::Ui, label: &str, ink: egui::Color32) -> Arc<egui::Galley> {
    ui.painter().layout_no_wrap(
        label.to_owned(),
        egui::FontId::new(UI_SIZE, egui::FontFamily::Proportional),
        ink,
    )
}

/// How wide the pill draws for `label` under one arm, without drawing it.
fn pill_width(ui: &egui::Ui, label: &str, arm: Arm) -> f32 {
    let galley = pill_galley(ui, label, egui::Color32::WHITE);
    arm.pad_x + control::ICON_XS + arm.gap + galley.size().x + arm.pad_x
}

/// [`meridian_egui::widgets::status_pill`] with the outer inset and the
/// icon–label gap lifted out as arguments.
fn pill_at(ui: &mut egui::Ui, spec: &Specimen, arm: Arm) -> Response {
    let sem = semantic(widgets::mode_of(ui).is_dark());
    let colours = sem.role(spec.role);
    let ink = theme::to_color32(colours.foreground.base);
    let tokens = ui.tokens();

    let height = control::HEIGHT_XS;
    let icon_size = control::ICON_XS;
    let pad_x = arm.pad_x;
    let gap = arm.gap;

    let galley = pill_galley(ui, spec.label, ink);
    let width = pad_x + icon_size + gap + galley.size().x + pad_x;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
    let label = spec.label;
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));

    if ui.is_rect_visible(rect) {
        let radius = egui::CornerRadius::same(tokens.radius_chip.round() as u8);
        let painter = ui.painter();
        painter.rect_filled(rect, radius, theme::to_color32(colours.background.base));
        painter.rect_stroke(
            rect,
            radius,
            egui::Stroke::new(1.0, theme::to_color32(colours.border.base)),
            StrokeKind::Inside,
        );

        let icon_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + pad_x, rect.center().y - icon_size / 2.0),
            egui::vec2(icon_size, icon_size),
        );
        spec.icon.paint(painter, icon_rect, ink);
        // Centred on the full line box, descent included — the vertical
        // defect, reproduced rather than repaired.
        painter.galley(
            egui::pos2(
                icon_rect.right() + gap,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            ink,
        );
    }
    response
}

/// The keycap galley, laid out exactly as the shipping chip lays it out —
/// through a `RichText` layout job rather than `layout_no_wrap`, because the
/// two do not produce the same line box.
fn chip_galley(ui: &egui::Ui, keystroke: &str, ink: egui::Color32) -> Arc<egui::Galley> {
    let job = egui::WidgetText::from(
        egui::RichText::new(keystroke)
            .font(egui::FontId::new(
                CHART_LABEL_SIZE,
                egui::FontFamily::Monospace,
            ))
            .color(ink),
    )
    .into_layout_job(ui.style(), egui::FontSelection::Default, egui::Align::Min);
    ui.painter().layout_job(Arc::unwrap_or_clone(job))
}

/// The keycap's frame under one arm. Only the horizontal margin moves; the
/// chip has no icon–label gap of its own, which is exactly why arm 2 leaves
/// it alone.
fn chip_frame(ui: &egui::Ui, pad_x: f32) -> egui::Frame {
    let t = ui.tokens();
    let sem = semantic(ui.visuals().dark_mode);
    egui::Frame::new()
        .fill(theme::to_color32(sem.surfaces.sunken))
        .stroke(egui::Stroke::new(
            1.0,
            theme::to_color32(sem.borders.default_),
        ))
        .corner_radius(t.radius_chip)
        .inner_margin(egui::Margin::symmetric(pad_x as i8, t.space[1] as i8))
}

/// How wide the chip draws for `keystroke` under one arm, without drawing it.
fn chip_width(ui: &egui::Ui, keystroke: &str, pad_x: f32) -> f32 {
    let galley = chip_galley(ui, keystroke, egui::Color32::WHITE);
    galley.size().x + chip_frame(ui, pad_x).total_margin().sum().x
}

/// [`meridian_egui::key_chip`] with its horizontal margin lifted out as an
/// argument.
fn chip_at(ui: &mut egui::Ui, keystroke: &str, pad_x: f32) -> Response {
    let ink = theme::to_color32(semantic(ui.visuals().dark_mode).text.secondary);
    let frame = chip_frame(ui, pad_x);
    let galley = chip_galley(ui, keystroke, ink);
    let margin = frame.total_margin();

    let (rect, response) = ui.allocate_exact_size(galley.size() + margin.sum(), Sense::hover());
    let content_rect = rect - margin;

    if ui.is_rect_visible(rect) {
        ui.painter().add(frame.paint(content_rect));
        ui.painter().galley(content_rect.min, galley, ink);
    }
    response
}

// ---------------------------------------------------------------------------
// The montage.
// ---------------------------------------------------------------------------

/// One drawn box, in points. Collected during the frame so the caption in the
/// picture and the numbers in the test report come from the same measurement
/// rather than from two arithmetics that could disagree.
#[derive(Clone, Debug)]
struct Measured {
    arm: &'static str,
    item: &'static str,
    width: f32,
}

/// Everything the frame measured while drawing itself.
#[derive(Default)]
struct Montage {
    measured: Vec<Measured>,
}

impl Montage {
    fn width(&self, arm: &str, item: &str) -> f32 {
        self.measured
            .iter()
            .find(|m| m.arm == arm && m.item == item)
            .unwrap_or_else(|| panic!("{arm} / {item} was never drawn"))
            .width
    }
}

/// Indent a line of content to the panel padding, so captions and specimen
/// rows share one left edge and the rows can be compared vertically.
fn indented(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.add_space(ui.tokens().panel_padding);
        add(ui);
    });
}

/// Draw the whole montage for one mode, recording every box it draws.
fn montage_ui(ui: &mut egui::Ui, mode: Mode, state: &mut Montage) {
    theme::apply(ui.ctx(), mode);
    state.measured.clear();

    // An opaque backdrop across the entire frame, including the harness's own
    // outer margin — the wgpu renderer clears to transparent, and a montage
    // with a transparent border reads differently in every viewer it is
    // opened in. Same colour the central panel paints, so there is no seam.
    let backdrop = ui.visuals().panel_fill;
    ui.ctx()
        .layer_painter(egui::LayerId::background())
        .rect_filled(ui.ctx().content_rect(), 0.0, backdrop);

    let t = ui.tokens();
    ui.add_space(t.space[2]);
    indented(ui, |ui| {
        ui.label(format!(
            "status pill — outer inset against icon–label gap · {mode:?} · {PIXELS_PER_POINT:.1} px/pt"
        ));
    });

    for arm in arms() {
        ui.add_space(t.section_gap);

        // Measured before the caption is written, so the caption states what
        // the row below it actually draws.
        let widest = pill_width(ui, "failing", arm);
        let chip = chip_width(ui, KEYSTROKES[0], arm.pad_x);
        indented(ui, |ui| {
            ui.label(format!(
                "{} · inset {:.1}, gap {:.1} — \"failing\" pill {:.1}pt, \"{}\" chip {:.1}pt",
                arm.key, arm.pad_x, arm.gap, widest, KEYSTROKES[0], chip
            ));
        });

        ui.add_space(t.space[1]);
        indented(ui, |ui| {
            for spec in &SPECIMENS {
                let r = pill_at(ui, spec, arm);
                state.measured.push(Measured {
                    arm: arm.key,
                    item: spec.label,
                    width: r.rect.width(),
                });
                ui.add_space(t.control_gap);
            }
            ui.add_space(t.pane_gap);
            for k in KEYSTROKES {
                let r = chip_at(ui, k, arm.pad_x);
                state.measured.push(Measured {
                    arm: arm.key,
                    item: k,
                    width: r.rect.width(),
                });
                ui.add_space(t.control_gap);
            }
        });
    }
}

/// Lay the montage out for one mode and settle it.
fn montage_harness(mode: Mode, wgpu: bool) -> Harness<'static, Montage> {
    let builder = Harness::builder()
        .with_size(egui::vec2(FRAME.0, FRAME.1))
        .with_pixels_per_point(PIXELS_PER_POINT);
    let builder = if wgpu { builder.wgpu() } else { builder };
    let mut harness = builder.build_ui_state(
        move |ui, state: &mut Montage| montage_ui(ui, mode, state),
        Montage::default(),
    );
    // The theme and the bundled fonts both land on the frame *after* the one
    // that installs them, so the montage is settled before it is measured or
    // photographed.
    harness.run();
    harness.run();
    harness
}

// ---------------------------------------------------------------------------
// The gate on the picture.
// ---------------------------------------------------------------------------

/// Widths measured to the nearest hundredth of a point. The arms differ by
/// whole ladder steps, so this is far tighter than the smallest real change
/// and loose enough to survive the order f32 addition happens to run in.
const EPSILON: f32 = 0.01;

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() <= EPSILON
}

/// The picture is only worth looking at if its "today" row is the primitive
/// that actually ships and its two arms differ from it by exactly the token
/// step each claims to spend.
///
/// This is the whole reason a copy is safe: [`pill_at`] and [`chip_at`]
/// duplicate two shipping bodies, and a duplicate drifts silently. Drawn on
/// the CPU tessellation path, so it is an ordinary test on a runner with no
/// GPU.
#[test]
fn the_today_row_is_the_pill_that_ships_and_each_arm_moves_by_its_token_delta() {
    #[derive(Default)]
    struct Widths {
        /// `(item, shipping, today, arm 1, arm 2)`, in points.
        rows: Vec<(&'static str, f32, f32, f32, f32)>,
    }

    let [today, arm1, arm2] = arms();

    let mut harness = Harness::builder()
        .with_size(egui::vec2(900.0, 600.0))
        .build_ui_state(
            |ui, w: &mut Widths| {
                theme::apply(ui.ctx(), Mode::Light);
                w.rows.clear();
                for spec in &SPECIMENS {
                    let shipping = widgets::status_pill(ui, spec.icon, spec.label, spec.role)
                        .rect
                        .width();
                    w.rows.push((
                        spec.label,
                        shipping,
                        pill_at(ui, spec, today).rect.width(),
                        pill_at(ui, spec, arm1).rect.width(),
                        pill_at(ui, spec, arm2).rect.width(),
                    ));
                }
                for k in KEYSTROKES {
                    let shipping = key_chip(ui, k).rect.width();
                    w.rows.push((
                        k,
                        shipping,
                        chip_at(ui, k, today.pad_x).rect.width(),
                        chip_at(ui, k, arm1.pad_x).rect.width(),
                        chip_at(ui, k, arm2.pad_x).rect.width(),
                    ));
                }
            },
            Widths::default(),
        );
    harness.run();
    harness.run();

    assert_eq!(
        harness.state().rows.len(),
        SPECIMENS.len() + KEYSTROKES.len(),
        "every specimen was drawn"
    );

    // What the two arms are worth, stated from the ladder rather than as
    // literals: the pill spends its inset twice and its gap once, the chip
    // spends its inset twice and has no gap.
    let step = TOKENS.icon_label_gap - TOKENS.space[2];
    let pill_arm1 = 2.0 * step;
    let pill_arm2 = -step;
    let chip_arm1 = 2.0 * step;

    for &(item, shipping, today_w, arm1_w, arm2_w) in &harness.state().rows {
        let is_chip = KEYSTROKES.contains(&item);
        assert!(
            close(today_w, shipping),
            "{item}: the montage's today row draws {today_w}pt but the shipping \
             primitive draws {shipping}pt — the copy has drifted and the picture \
             no longer shows what ships"
        );
        let (want1, want2) = if is_chip {
            (chip_arm1, 0.0)
        } else {
            (pill_arm1, pill_arm2)
        };
        assert!(
            close(arm1_w - today_w, want1),
            "{item}: arm 1 moved by {}pt, not the {want1}pt its inset costs",
            arm1_w - today_w
        );
        assert!(
            close(arm2_w - today_w, want2),
            "{item}: arm 2 moved by {}pt, not the {want2}pt its gap saves",
            arm2_w - today_w
        );
    }

    // And the premise the picture exists to settle: the pill really is looser
    // in its middle than it is inset from its container. If this ever stops
    // being true the montage is showing a question nobody is asking.
    assert!(
        TOKENS.space[2] < TOKENS.icon_label_gap,
        "the outer inset ({}) is no longer tighter than the icon–label gap ({})",
        TOKENS.space[2],
        TOKENS.icon_label_gap
    );
}

// ---------------------------------------------------------------------------
// The render.
// ---------------------------------------------------------------------------

/// Write the two montage PNGs.
///
/// `#[ignore]`d deliberately: `render()` needs a real GPU adapter through
/// wgpu, and the CI runner for this repo has not got one. The sibling tests
/// here stay on the CPU tessellation path for the same reason. Run it by
/// hand:
///
/// ```text
/// cargo test -p meridian-egui --test pill_geometry_montage -- --ignored --nocapture
/// ```
#[test]
#[ignore = "needs a GPU adapter through wgpu, and writes PNGs — run by hand"]
fn write_the_montage_pngs() {
    let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits inside the repo")
        .join("throwaway-pill-montage");
    std::fs::create_dir_all(&out).expect("create the montage directory");

    for (mode, name) in [(Mode::Light, "light"), (Mode::Dark, "dark")] {
        let mut harness = montage_harness(mode, true);

        let image = harness.render().expect("render the montage through wgpu");
        let path = out.join(format!("pill-geometry-{name}.png"));
        image.save(&path).expect("write the montage PNG");
        println!("wrote {} ({}x{} px)", path.display(), image.width(), image.height());

        let state = harness.state();
        for arm in arms() {
            for item in SPECIMENS
                .iter()
                .map(|s| s.label)
                .chain(KEYSTROKES.iter().copied())
            {
                println!(
                    "{name:5}  {:5}  {item:8}  {:6.2}pt",
                    arm.key,
                    state.width(arm.key, item)
                );
            }
        }
    }
}
