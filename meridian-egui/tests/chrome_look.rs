//! The chrome look, read out of the paint list.
//!
//! A chip, a control and a card take a small corner (2, 3 and 3); a row, a bar,
//! a hairline and a band that runs edge to edge stay square. The design crate
//! states the look in tokens and pins them; this file asks the primitives
//! whether they *draw* it — every claim below is made of what a frame painted,
//! because a token can read `3` while a painter still passes a radius of its
//! own, offsets a ring outward, or strokes a hairline where a foot was wanted,
//! and none of those is visible from the token.
//!
//! **What each test is looking at is the shape list, not a picture.** A
//! `RectShape` says its rect, its fill, its stroke and where the stroke sits
//! (`StrokeKind`), its corner and its blur, so "the ring lies inside the rect"
//! is a claim about the shape (the stroke's kind and width against the rect it
//! was given) rather than about pixels an antialiasing fringe would blur. The
//! expectations are laid out from the design crate's named constants and inks,
//! never asked of the code under test, and the literal widths the look states
//! (2, 1, 3) are held beside the constants so a token drifting cannot take the
//! claim with it.
//!
//! CPU tessellation only — green on a runner with no GPU, like its siblings.

mod common;

use common::{assert_corner, assert_square, filled, ink, near, near_rect, painted, Painted, EPS};
use egui::{Color32, CornerRadius, Rect, StrokeKind};
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use meridian_design::control::{KEYCAP_FOOT_WIDTH, ROW_BAR_WIDTH};
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{CHIP_PADDING_X, SPACE_1};
use meridian_design::{focus, radius, Elevation};
use meridian_egui::{
    icons, key_chip, line_tabs, list_row, overlay_frame, query_line, theme, widgets, LineTabs,
    ListRow, ModalChrome, ModalLayer, Mode, Notification, NotificationId, NotificationLayer,
    RowHeight, RowState, Severity, Toast, ToastLayer, TOKENS,
};

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];

struct OwnWidgets {
    text: String,
    on: bool,
    v: f32,
    pick: usize,
}

/// egui's own widgets, and its own windows and menus, drawn under the theme.
///
/// The widgets that read their corners from the visuals: a button, a checkbox
/// and a radio, a text edit, a slider (rail and thumb), a drag value, a
/// selectable label, a collapsing header, a scroll bar, a combo box and the menu
/// it opens, a menu button and the menu it opens, and a window. The scroll bar
/// is set solid so it is drawn without a pointer over it.
///
/// **What this does not reach:** `egui::ProgressBar` rounds itself to half its
/// height unless the caller gives it a corner, and no slot in the visuals
/// changes that — it is left out because the theme cannot square it, not
/// because it is square.
fn own_widgets(mode: Mode) -> Harness<'static, OwnWidgets> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(700.0, 700.0))
        .build_ui_state(
            move |ui, s: &mut OwnWidgets| {
                theme::apply(ui.ctx(), mode);
                ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
                let _ = ui.button("button");
                ui.checkbox(&mut s.on, "check");
                let _ = ui.radio(s.on, "radio");
                ui.text_edit_singleline(&mut s.text);
                ui.add(egui::Slider::new(&mut s.v, 0.0..=1.0));
                ui.add(egui::DragValue::new(&mut s.v));
                let _ = ui.selectable_label(s.on, "selectable");
                ui.collapsing("collapsing", |ui| {
                    ui.label("inside");
                });
                egui::ScrollArea::vertical()
                    .id_salt("bars")
                    .max_height(40.0)
                    .show(ui, |ui| {
                        for i in 0..30 {
                            ui.label(format!("line {i}"));
                        }
                    });
                egui::ComboBox::from_label("combo")
                    .selected_text(format!("choice {}", s.pick))
                    .show_ui(ui, |ui| {
                        for i in 0..3 {
                            ui.selectable_value(&mut s.pick, i, format!("option {i}"));
                        }
                    });
                ui.menu_button("menu", |ui| {
                    let _ = ui.button("menu item");
                });
                egui::Window::new("window").show(ui.ctx(), |ui| {
                    ui.label("in a window");
                });
            },
            OwnWidgets {
                text: "text".into(),
                on: true,
                v: 0.5,
                pick: 0,
            },
        );
    harness.run();
    harness
}

#[test]
fn egui_own_buttons_fields_windows_and_menus_draw_their_token_corner() {
    for mode in MODES {
        let mut harness = own_widgets(mode);
        // The menus draw only while open, and a menu is the thing this claim is
        // about, so open both and let them lay out.
        harness.get_by_label("menu").click();
        harness.run();
        harness.get_by_role(egui::accesskit::Role::ComboBox).click();
        harness.run();

        let rects = painted(&harness, mode);
        let v = theme::meridian_visuals(mode);
        let present = |fill: Color32| !filled(&rects, fill).is_empty();
        // A button and a text field take the control corner; a window and a
        // menu take the panel corner.
        assert_corner(
            &filled(&rects, v.widgets.inactive.weak_bg_fill),
            radius::CONTROL,
            &format!("egui's buttons, {mode:?}"),
        );
        assert_corner(
            &filled(&rects, v.extreme_bg_color),
            radius::CONTROL,
            &format!("egui's fields, {mode:?}"),
        );
        assert_corner(
            &filled(&rects, v.window_fill),
            radius::PANEL,
            &format!("egui's windows and menus, {mode:?}"),
        );

        // The claim is only as wide as what was drawn: the parts it names must
        // have been on the frame, or "every corner is right" is "nothing here".
        assert!(
            present(v.window_fill),
            "{mode:?}: no window or menu surface"
        );
        assert!(
            present(v.widgets.inactive.weak_bg_fill),
            "{mode:?}: no button"
        );
        assert!(
            present(v.extreme_bg_color),
            "{mode:?}: no text edit or scroll track"
        );
    }
}

#[test]
fn the_slider_thumb_stays_round() {
    // The look squares every corner but a slider's thumb, one of the marks
    // that holds no icon and no word. egui draws its stock thumb as a rounded
    // rectangle with the widget corner, so squaring the corners would square
    // the thumb unless the theme asks for a circle. The slider is drawn alone:
    // a radio button draws a circle of its own, which would answer for it.
    for mode in MODES {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(300.0, 100.0))
            .build_ui_state(
                move |ui, v: &mut f32| {
                    theme::apply(ui.ctx(), mode);
                    ui.add(egui::Slider::new(v, 0.0..=1.0));
                },
                0.5,
            );
        harness.run();
        let circles: Vec<_> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|c| match &c.shape {
                egui::Shape::Circle(circle) => Some(*circle),
                _ => None,
            })
            .collect();
        assert_eq!(circles.len(), 1, "{mode:?}: the slider drew {circles:?}");
        assert!(circles[0].radius > 0.0, "{mode:?}: a thumb of no size");
    }
}

/// The status pills and a key chip, each drawn alone so the shapes in the frame
/// are the primitive's own.
fn chips(mode: Mode) -> (Harness<'static, ()>, Harness<'static, ()>) {
    let mut pills = Harness::builder()
        .with_size(egui::vec2(400.0, 100.0))
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), mode);
            widgets::status_pill(ui, &icons::CLOCK, "waiting", Role::Neutral);
            widgets::status_pill(ui, &icons::CIRCLE_CHECK, "ok", Role::Success);
        });
    pills.run();
    let mut key = Harness::builder()
        .with_size(egui::vec2(300.0, 100.0))
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), mode);
            key_chip(ui, "Esc");
        });
    key.run();
    (pills, key)
}

#[test]
fn a_key_and_a_pill_draw_the_chip_corner() {
    for mode in MODES {
        let (pills, key) = chips(mode);
        assert_corner(
            &painted(&pills, mode),
            radius::CHIP,
            &format!("the status pills, {mode:?}"),
        );

        let rects = painted(&key, mode);
        let sunken = ink(semantic(mode.is_dark()).surfaces.sunken);
        let boxes = filled(&rects, sunken);
        assert_eq!(boxes.len(), 1, "{mode:?}: one key box");
        assert_corner(&boxes, radius::CHIP, &format!("the key's box, {mode:?}"));

        // The foot is the box's own rect at the chip corner, so its corners
        // are the box's; the key test below pins what it is clipped to.
        let foot_ink = ink(semantic(mode.is_dark()).borders.default_);
        let feet = filled(&rects, foot_ink);
        assert_eq!(feet.len(), 1, "{mode:?}: one foot");
        assert_eq!(
            feet[0].radius,
            CornerRadius::same(radius::CHIP as u8),
            "{mode:?}: the foot's corners"
        );
    }
}

/// Line tabs and a pane header, drawn into `ui`.
fn tabs_and_header(ui: &mut egui::Ui) {
    let _ = line_tabs(ui, LineTabs::new(&["one", "two"], 0));
    let _ = widgets::pane_header(ui, None, "header");
}

/// A query line and a row in each state: the primitives that stay square.
fn bands(mode: Mode) -> Harness<'static, String> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(700.0, 400.0))
        .build_ui_state(
            move |ui, q: &mut String| {
                theme::apply(ui.ctx(), mode);
                query_line(ui, q, "Search…");
                list_row(ui, ListRow::new(RowHeight::Grid), |ui, _| {
                    ui.label("at rest");
                });
                list_row(ui, ListRow::new(RowHeight::Grid).selected(true), |ui, _| {
                    ui.label("under the cursor");
                });
                tabs_and_header(ui);
            },
            String::new(),
        );
    harness.run();
    harness
}

#[test]
fn a_row_the_query_line_line_tabs_and_the_pane_header_stay_square() {
    for mode in MODES {
        let harness = bands(mode);
        assert_square(
            &painted(&harness, mode),
            &format!("the bands and rows, {mode:?}"),
        );
    }
}

/// The three floating cards, drawn: the modal (through the layer, and inline),
/// a banner and a toast.
fn cards(mode: Mode) -> Harness<'static, (NotificationLayer, ToastLayer)> {
    let mut banners = NotificationLayer::new();
    banners.raise(Notification::new(
        NotificationId::new("card"),
        Severity::Error,
        "banner",
    ));
    let mut toasts = ToastLayer::new();
    toasts.push(Toast::new(Severity::Success, "toast"));

    let mut harness = Harness::builder()
        .with_size(egui::vec2(900.0, 700.0))
        .build_ui_state(
            move |ui, (banners, toasts): &mut (NotificationLayer, ToastLayer)| {
                theme::apply(ui.ctx(), mode);
                banners.show(ui.ctx());
                toasts.show(ui.ctx());
                overlay_frame(ui, &ModalChrome::new(), |ui| {
                    ui.label("inline card");
                });
                ModalLayer::show(ui.ctx(), "modal", &ModalChrome::new(), |ui| {
                    ui.label("modal card");
                });
            },
            (banners, toasts),
        );
    harness.run();
    harness
}

#[test]
fn the_modal_card_the_banner_and_the_toast_draw_the_panel_corner() {
    for mode in MODES {
        let harness = cards(mode);
        let sem = semantic(mode.is_dark());
        let rects = painted(&harness, mode);

        // Each card is a ruled rect in the overlay fill: the inline modal card,
        // the modal layer's card, the banner and the toast (a dismiss button inside one
        // is a smaller ruled rect in the same fill, and is a control). Four must be on the
        // frame, or a missing card would pass for a right corner.
        let overlay = ink(sem.surfaces.overlay);
        let cards: Vec<Painted> = filled(&rects, overlay)
            .into_iter()
            .filter(|r| r.is_stroked() && r.rect.height() >= 30.0)
            .collect();
        assert_eq!(
            cards.len(),
            4,
            "{mode:?}: the inline card, the modal card, the banner and the toast; painted {rects:?}"
        );
        assert_corner(&cards, radius::PANEL, &format!("the cards, {mode:?}"));

        // And nothing else the cards paint is rounded differently: their
        // shadows take the panel corner too. The keystroke hint in the modal is
        // a chip and the scrim dims the whole screen; neither is a card.
        let keycap = [ink(sem.surfaces.sunken), ink(sem.borders.default_)];
        let rest: Vec<Painted> = rects
            .into_iter()
            .filter(|r| !keycap.contains(&r.fill) && r.rect.width() < 900.0)
            .collect();
        assert_corner(
            &rest,
            radius::PANEL,
            &format!("the cards' shadows, {mode:?}"),
        );
    }
}

// ─── the focus ring ───────────────────────────────────────────────

/// A control's rect, drawn at a place that is not the origin so a ring
/// mis-anchored to (0,0) is not a ring on it.
fn ring_frame(mode: Mode, draw: impl Fn(&egui::Ui, Rect) + 'static) -> (Vec<Painted>, Rect) {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 200.0))
        .build_ui_state(
            move |ui, control: &mut Rect| {
                theme::apply(ui.ctx(), mode);
                ui.add_space(20.0);
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(96.0, 28.0), egui::Sense::hover());
                *control = rect;
                draw(ui, rect);
            },
            Rect::NOTHING,
        );
    harness.run();
    let control = *harness.state();
    (painted(&harness, mode), control)
}

#[test]
fn a_focused_controls_ring_is_a_two_pixel_rule_in_the_focus_ink_inside_its_rect() {
    // The vault states the width as 2; the token is held beside it.
    assert_eq!(focus::RING_WIDTH, 2.0);
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let focus_ink = ink(sem.borders.focus);
        let (rects, control) = ring_frame(mode, |ui, rect| {
            widgets::focus_ring(ui, rect, TOKENS.radius_control);
        });

        let rings: Vec<Painted> = rects
            .iter()
            .filter(|r| r.stroke.color == focus_ink)
            .copied()
            .collect();
        assert_eq!(
            rings.len(),
            1,
            "{mode:?}: one ring painted, in the focus ink"
        );
        let ring = rings[0];
        assert!(
            near(ring.stroke.width, 2.0),
            "{mode:?}: the ring is {} wide",
            ring.stroke.width
        );
        // The ring follows the control's corner, so the two stay concentric.
        assert_eq!(
            ring.radius,
            CornerRadius::same(focus::ring_radius(radius::CONTROL).round() as u8),
            "{mode:?}: the ring's corner is the control's"
        );
        assert_eq!(ring.radius, CornerRadius::same(3), "{mode:?}");
        assert!(
            control.expand(EPS).contains_rect(ring.outer()),
            "{mode:?}: the ring paints over {:?}, outside the control's rect {:?}",
            ring.outer(),
            control
        );
        // And it is *on* the control, not merely somewhere inside it: the ring
        // hugs the edge, so its outer edge is the control's edge.
        assert!(
            near_rect(ring.outer(), control),
            "{mode:?}: the ring's outer edge is {:?} and the control's edge {:?}",
            ring.outer(),
            control
        );
    }
}

#[test]
fn focus_ring_for_draws_a_ring_only_on_a_control_that_has_the_keys() {
    for mode in MODES {
        let focus_ink = ink(semantic(mode.is_dark()).borders.focus);
        for focused in [false, true] {
            let mut harness = Harness::builder()
                .with_size(egui::vec2(300.0, 100.0))
                .build_ui(move |ui| {
                    theme::apply(ui.ctx(), mode);
                    let response = ui.button("go");
                    if focused {
                        response.request_focus();
                    }
                    widgets::focus_ring_for(ui, &response);
                });
            harness.run();
            let rings = painted(&harness, mode)
                .iter()
                .filter(|r| r.stroke.color == focus_ink && near(r.stroke.width, 2.0))
                .count();
            assert_eq!(
                rings,
                usize::from(focused),
                "{mode:?}, focused {focused}: {rings} ring(s) drawn"
            );
        }
    }
}

#[test]
fn the_ring_on_an_accent_fill_is_one_pixel_of_the_on_solid_ink_one_pixel_in() {
    assert_eq!(focus::RING_WIDTH_ON_SOLID, 1.0);
    assert_eq!(focus::RING_INSET, 1.0);
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let on_solid = ink(sem.text.on_solid);
        assert_ne!(
            on_solid,
            ink(sem.borders.focus),
            "{mode:?}: the two inks are one, so this cannot tell which ring it drew"
        );
        let (rects, control) = ring_frame(mode, |ui, rect| {
            widgets::focus_ring_on_solid(ui, rect, TOKENS.radius_control);
        });

        let rings: Vec<Painted> = rects
            .iter()
            .filter(|r| r.stroke.color == on_solid)
            .copied()
            .collect();
        assert_eq!(
            rings.len(),
            1,
            "{mode:?}: one ring painted, in the on-solid ink"
        );
        let ring = rings[0];
        assert!(
            near(ring.stroke.width, 1.0),
            "{mode:?}: the ring is {} wide",
            ring.stroke.width
        );
        // Drawn one pixel in, the ring's corner is the control's less that
        // pixel, so it still runs parallel to the edge.
        assert_eq!(
            ring.radius,
            CornerRadius::same(focus::inset_ring_radius(radius::CONTROL).round() as u8),
            "{mode:?}: the ring's corner is the control's, one pixel in"
        );
        assert_eq!(ring.radius, CornerRadius::same(2), "{mode:?}");
        assert!(
            near_rect(ring.outer(), control.shrink(1.0)),
            "{mode:?}: the ring runs {:?}; a ring one pixel in from {:?} runs {:?}",
            ring.outer(),
            control,
            control.shrink(1.0)
        );
    }
}

#[test]
fn focus_ring_on_solid_for_follows_the_keys_too() {
    for mode in MODES {
        let on_solid = ink(semantic(mode.is_dark()).text.on_solid);
        for focused in [false, true] {
            let mut harness = Harness::builder()
                .with_size(egui::vec2(300.0, 100.0))
                .build_ui(move |ui| {
                    theme::apply(ui.ctx(), mode);
                    let response = ui.button("go");
                    if focused {
                        response.request_focus();
                    }
                    widgets::focus_ring_on_solid_for(ui, &response);
                });
            harness.run();
            let rings = painted(&harness, mode)
                .iter()
                .filter(|r| r.stroke.color == on_solid && near(r.stroke.width, 1.0))
                .count();
            assert_eq!(rings, usize::from(focused), "{mode:?}, focused {focused}");
        }
    }
}

// ─── the keycap ────────────────────────────────────────────────────────

#[test]
fn a_key_is_one_rounded_box_with_a_hairline_on_three_sides_and_a_foot() {
    // The look states the foot as 2 and the hairline as 1.
    assert_eq!(KEYCAP_FOOT_WIDTH, 2.0);
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (sunken, side, foot_ink) = (
            ink(sem.surfaces.sunken),
            ink(sem.borders.subtle),
            ink(sem.borders.default_),
        );
        assert_ne!(
            side, foot_ink,
            "{mode:?}: the foot is a step darker than the sides"
        );

        let mut harness = Harness::builder()
            .with_size(egui::vec2(300.0, 100.0))
            .build_ui(move |ui| {
                theme::apply(ui.ctx(), mode);
                key_chip(ui, "Esc");
            });
        harness.run();
        let rects = painted(&harness, mode);

        // One box with the sunken fill and the side hairline, the key's whole
        // extent, drawn once at the chip corner: the corner is the box's.
        let boxes = filled(&rects, sunken);
        assert_eq!(boxes.len(), 1, "{mode:?}: one sunken box");
        let b = boxes[0];
        assert_eq!(
            b.radius,
            CornerRadius::same(2),
            "{mode:?}: the box's corner"
        );
        assert!(b.is_stroked(), "{mode:?}: the box carries the hairline");
        assert_eq!(b.stroke.color, side, "{mode:?}: the hairline's ink");
        assert!(near(b.stroke.width, 1.0), "{mode:?}: the hairline's width");
        assert_eq!(b.kind, StrokeKind::Inside, "{mode:?}: the hairline's side");

        // The foot is the box's own rect at the box's corner, clipped to the
        // bottom 2px. A rect only 2px tall would clamp its corners to 1px and
        // poke out of the box's outline; this cannot.
        let feet = filled(&rects, foot_ink);
        assert_eq!(feet.len(), 1, "{mode:?}: one foot");
        assert!(
            near_rect(feet[0].rect, b.rect),
            "{mode:?}: the foot's rect is {:?}, not the box's {:?}",
            feet[0].rect,
            b.rect
        );
        assert_eq!(
            feet[0].radius,
            CornerRadius::same(2),
            "{mode:?}: the foot follows the box's corners"
        );
        let band = Rect::from_min_max(
            egui::pos2(b.rect.left(), b.rect.bottom() - KEYCAP_FOOT_WIDTH),
            b.rect.max,
        );
        assert!(
            near_rect(feet[0].clip, band),
            "{mode:?}: the foot shows through {:?}, not the bottom band {band:?}",
            feet[0].clip
        );
        assert_eq!(
            rects.len(),
            2,
            "{mode:?}: the box and its foot, and nothing else"
        );
    }
}

#[test]
fn a_key_is_the_size_it_was_and_its_keystroke_is_centred_across_the_face() {
    for mode in MODES {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(300.0, 100.0))
            .build_ui(move |ui| {
                theme::apply(ui.ctx(), mode);
                key_chip(ui, "Ctrl+K");
            });
        harness.run();
        let rects = painted(&harness, mode);
        let sunken = ink(semantic(mode.is_dark()).surfaces.sunken);
        let b = filled(&rects, sunken)[0].rect;

        let text = harness
            .output()
            .shapes
            .iter()
            .find_map(|c| match &c.shape {
                egui::Shape::Text(t) => Some(t.clone()),
                _ => None,
            })
            .expect("the key drew its keystroke");
        let galley = text.galley.size();

        // The box a key chip was drawn in when it was one frame and one
        // hairline: the galley, the ladder padding and a hairline on each edge.
        assert!(
            near(b.width(), galley.x + 2.0 * (CHIP_PADDING_X + 1.0)),
            "{mode:?}: the key is {} wide against a galley of {}",
            b.width(),
            galley.x
        );
        assert!(
            near(b.height(), galley.y + 2.0 * (SPACE_1 + 1.0)),
            "{mode:?}: the key is {} tall against a galley of {}",
            b.height(),
            galley.y
        );

        let face = Rect::from_min_max(b.min + egui::vec2(1.0, 1.0), b.max - egui::vec2(1.0, 2.0));
        let font_box = Rect::from_min_size(text.pos, galley);
        assert!(
            near(font_box.center().x, face.center().x),
            "{mode:?}: the keystroke is centred at x {} on a face centred at {}",
            font_box.center().x,
            face.center().x
        );
        assert!(
            face.expand(EPS).contains_rect(font_box),
            "{mode:?}: the keystroke's box {font_box:?} leaves the face {face:?}"
        );
    }
}

// ─── the list row ──────────────────────────────────────────────────────

/// One row drawn on its own, its rect and everything the frame painted. `hover`
/// puts the pointer over the row; `bar` is the ink the caller passes, if any.
fn drawn_row(
    mode: Mode,
    selected: bool,
    hover: bool,
    bar: Option<Color32>,
) -> (Vec<Painted>, Rect) {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 200.0))
        .build_ui_state(
            move |ui, rect: &mut Rect| {
                theme::apply(ui.ctx(), mode);
                let mut row = ListRow::new(RowHeight::Grid).selected(selected);
                if let Some(ink) = bar {
                    row = row.bar_ink(ink);
                }
                let r = list_row(ui, row, |ui, _state: RowState| {
                    ui.label("row");
                });
                *rect = r.response.rect;
            },
            Rect::NOTHING,
        );
    harness.run();
    if hover {
        harness.get_by_label("row").hover();
        harness.run();
    }
    let rect = *harness.state();
    (painted(&harness, mode), rect)
}

#[test]
fn a_row_at_rest_draws_nothing() {
    for mode in MODES {
        let (rects, row) = drawn_row(mode, false, false, None);
        let on_row: Vec<_> = rects.iter().filter(|r| r.rect.intersects(row)).collect();
        assert!(
            on_row.is_empty(),
            "{mode:?}: a row at rest painted {on_row:?}"
        );
    }
}

#[test]
fn a_row_under_the_pointer_takes_the_hover_fill_and_no_bar() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (hover, cursor) = (
            ink(sem.rows.hover_background),
            ink(sem.rows.cursor_background),
        );
        assert_ne!(
            hover, cursor,
            "{mode:?}: the two fills are one, so this cannot tell them apart"
        );

        let (rects, row) = drawn_row(mode, false, true, None);
        let on_row: Vec<_> = rects.iter().filter(|r| r.rect.intersects(row)).collect();
        assert_eq!(
            on_row.len(),
            1,
            "{mode:?}: the hovered row painted {on_row:?}"
        );
        assert!(
            near_rect(on_row[0].rect, row),
            "{mode:?}: the fill is not the row"
        );
        assert_eq!(on_row[0].fill, hover, "{mode:?}: not the hover fill");
        assert_eq!(on_row[0].radius, CornerRadius::ZERO);
        assert!(
            !on_row[0].is_stroked(),
            "{mode:?}: a hovered row is stroked"
        );
    }
}

#[test]
fn a_row_under_the_cursor_takes_the_cursor_fill_and_a_three_pixel_bar_on_its_leading_edge() {
    assert_eq!(ROW_BAR_WIDTH, 3.0);
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        // With the pointer over it too: the cursor outranks the glance.
        for hover in [false, true] {
            let (rects, row) = drawn_row(mode, true, hover, None);
            let on_row: Vec<_> = rects.iter().filter(|r| r.rect.intersects(row)).collect();
            assert_eq!(
                on_row.len(),
                2,
                "{mode:?}, hover {hover}: the row painted {on_row:?}"
            );

            let fill = on_row
                .iter()
                .find(|r| near_rect(r.rect, row))
                .unwrap_or_else(|| panic!("{mode:?}: no fill over the row {row:?}: {on_row:?}"));
            assert_eq!(
                fill.fill,
                ink(sem.rows.cursor_background),
                "{mode:?}, hover {hover}"
            );

            let bar = on_row
                .iter()
                .find(|r| !near_rect(r.rect, row))
                .expect("a bar");
            let expected = Rect::from_min_size(row.min, egui::vec2(3.0, row.height()));
            assert!(
                near_rect(bar.rect, expected),
                "{mode:?}, hover {hover}: the bar is {:?}, a 3px bar on the leading edge is {:?}",
                bar.rect,
                expected
            );
            assert_eq!(
                bar.fill,
                ink(sem.rows.cursor_bar),
                "{mode:?}, hover {hover}: bar ink"
            );
            assert!(
                on_row
                    .iter()
                    .all(|r| !r.is_stroked() && r.radius == CornerRadius::ZERO),
                "{mode:?}, hover {hover}: the cursor row is stroked or rounded: {on_row:?}"
            );
        }
    }
}

#[test]
fn a_caller_can_pass_the_cursor_bars_ink_and_only_the_bars() {
    let hue = Color32::from_rgb(0xd0, 0x40, 0x80);
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (rects, row) = drawn_row(mode, true, false, Some(hue));
        let on_row: Vec<_> = rects.iter().filter(|r| r.rect.intersects(row)).collect();
        let bar = on_row
            .iter()
            .find(|r| r.fill == hue)
            .expect("a bar in the caller's ink");
        assert!(
            near(bar.rect.width(), 3.0),
            "{mode:?}: the bar is {} wide",
            bar.rect.width()
        );
        assert!(
            on_row
                .iter()
                .any(|r| near_rect(r.rect, row) && r.fill == ink(sem.rows.cursor_background)),
            "{mode:?}: the fill under a passed ink is still the cursor fill"
        );

        // The ink is the bar's: a row that is not under the cursor has no bar
        // to put it on, whatever it was handed.
        for hover in [false, true] {
            let (rects, row) = drawn_row(mode, false, hover, Some(hue));
            assert!(
                rects
                    .iter()
                    .filter(|r| r.rect.intersects(row))
                    .all(|r| r.fill != hue),
                "{mode:?}, hover {hover}: a row that is not under the cursor drew the caller's ink"
            );
        }
    }
}

// ─── the query line ────────────────────────────────────────────────────

/// The query line drawn alone: what it painted, with the keys on it or not.
fn drawn_query(mode: Mode, keys: bool) -> Vec<Painted> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 200.0))
        .build_ui_state(
            move |ui, q: &mut String| {
                theme::apply(ui.ctx(), mode);
                let r = query_line(ui, q, "Search…");
                if keys {
                    r.response.request_focus();
                }
            },
            String::new(),
        );
    harness.run();
    painted(&harness, mode)
}

#[test]
fn the_query_line_is_the_sunken_fill_over_a_one_pixel_rule_and_two_in_the_focus_ink_with_the_keys()
{
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (sunken, control, focus_ink) = (
            ink(sem.surfaces.sunken),
            ink(sem.borders.control),
            ink(sem.borders.focus),
        );
        for keys in [false, true] {
            let rects = drawn_query(mode, keys);
            let fields = filled(&rects, sunken);
            assert_eq!(fields.len(), 1, "{mode:?}, keys {keys}: one sunken field");
            let field = fields[0];
            assert_eq!(field.radius, CornerRadius::ZERO, "{mode:?}, keys {keys}");
            assert!(
                !field.is_stroked(),
                "{mode:?}, keys {keys}: the field is bordered, not ruled"
            );

            // The row it always was, and the strip its separator always took:
            // nothing under the field moves.
            assert!(
                near(field.rect.height(), TOKENS.control_heights[3] + 1.0),
                "{mode:?}, keys {keys}: the field is {} tall",
                field.rect.height()
            );

            let (rule_ink, rule_height, other_ink) = if keys {
                (focus_ink, 2.0, control)
            } else {
                (control, 1.0, focus_ink)
            };
            let rule = Rect::from_min_max(
                egui::pos2(field.rect.left(), field.rect.bottom() - rule_height),
                field.rect.max,
            );
            assert!(
                filled(&rects, rule_ink).iter().any(|r| near_rect(r.rect, rule)),
                "{mode:?}, keys {keys}: no {rule_height}px rule in the expected ink at {rule:?}; painted {rects:?}"
            );
            assert!(
                filled(&rects, other_ink).is_empty(),
                "{mode:?}, keys {keys}: the other state's rule is painted too"
            );
            // The rule goes down over the fill.
            let at = |r: &Painted| {
                rects
                    .iter()
                    .position(|p| p.rect == r.rect && p.fill == r.fill)
            };
            let rule_at = rects
                .iter()
                .position(|r| r.fill == rule_ink && near_rect(r.rect, rule))
                .unwrap();
            assert!(
                at(&field).unwrap() < rule_at,
                "{mode:?}, keys {keys}: the fill covers the rule"
            );
        }
    }
}

// ─── the floating cards ────────────────────────────────────────────────

/// The card among `rects`: the one rect with the overlay surface and a stroke.
fn the_card(rects: &[Painted], mode: Mode, what: &str) -> Painted {
    let overlay = ink(semantic(mode.is_dark()).surfaces.overlay);
    let found: Vec<Painted> = rects
        .iter()
        .filter(|r| r.fill == overlay && r.is_stroked())
        .copied()
        .collect();
    assert!(!found.is_empty(), "{mode:?}: {what}: no card painted");
    found[0]
}

/// The card is ruled 1px in the default border ink, and its shadow is the
/// card's rect offset by the elevation's `offset` in the elevation's ink with
/// no blur.
fn assert_ruled_with_a_hard_shadow(
    rects: &[Painted],
    mode: Mode,
    what: &str,
    elevation: Elevation,
    offset: f32,
) {
    let sem = semantic(mode.is_dark());
    let card = the_card(rects, mode, what);
    assert_eq!(
        card.stroke.color,
        ink(sem.borders.default_),
        "{mode:?}: {what}: rule ink"
    );
    assert!(
        near(card.stroke.width, 1.0),
        "{mode:?}: {what}: rule is {} wide",
        card.stroke.width
    );
    assert_eq!(card.kind, StrokeKind::Inside, "{mode:?}: {what}");

    let token = elevation
        .shadow(mode.is_dark())
        .expect("a floating card casts");
    assert_eq!(
        (token.x, token.y),
        (offset, offset),
        "{mode:?}: {what}: the token's offset"
    );
    assert_eq!(token.blur, 0.0, "{mode:?}: {what}: the token's blur");

    let shadow_rect = card.rect.translate(egui::vec2(offset, offset));
    let shadows: Vec<&Painted> = rects
        .iter()
        .filter(|r| near_rect(r.rect, shadow_rect) && r.fill == ink(token.colour))
        .collect();
    assert_eq!(
        shadows.len(),
        1,
        "{mode:?}: {what}: no shadow at {shadow_rect:?} (the card is {:?}); painted {rects:?}",
        card.rect
    );
    assert_eq!(
        shadows[0].blur, 0.0,
        "{mode:?}: {what}: the shadow is blurred"
    );
}

#[test]
fn the_modal_card_draws_a_one_pixel_rule_and_a_hard_shadow_six_by_six() {
    for mode in MODES {
        let mut inline = Harness::builder()
            .with_size(egui::vec2(700.0, 500.0))
            .build_ui(move |ui| {
                theme::apply(ui.ctx(), mode);
                overlay_frame(ui, &ModalChrome::new(), |ui| {
                    ui.label("card");
                });
            });
        inline.run();
        assert_ruled_with_a_hard_shadow(
            &painted(&inline, mode),
            mode,
            "overlay_frame",
            Elevation::Modal,
            6.0,
        );

        let mut layer = Harness::builder()
            .with_size(egui::vec2(900.0, 700.0))
            .build_ui(move |ui| {
                theme::apply(ui.ctx(), mode);
                ModalLayer::show(ui.ctx(), "m", &ModalChrome::new(), |ui| {
                    ui.label("card");
                });
            });
        layer.run();
        assert_ruled_with_a_hard_shadow(
            &painted(&layer, mode),
            mode,
            "ModalLayer",
            Elevation::Modal,
            6.0,
        );
    }
}

#[test]
fn the_banner_and_the_toast_draw_a_one_pixel_rule_and_a_hard_shadow_three_by_three() {
    for mode in MODES {
        let mut banner = Harness::builder()
            .with_size(egui::vec2(900.0, 700.0))
            .build_ui_state(
                move |ui, layer: &mut NotificationLayer| {
                    theme::apply(ui.ctx(), mode);
                    layer.show(ui.ctx());
                },
                {
                    let mut layer = NotificationLayer::new();
                    layer.raise(Notification::new(
                        NotificationId::new("b"),
                        Severity::Info,
                        "banner",
                    ));
                    layer
                },
            );
        banner.run();
        assert_ruled_with_a_hard_shadow(
            &painted(&banner, mode),
            mode,
            "banner",
            Elevation::Overlay,
            3.0,
        );

        let mut toast = Harness::builder()
            .with_size(egui::vec2(900.0, 700.0))
            .build_ui_state(
                move |ui, layer: &mut ToastLayer| {
                    theme::apply(ui.ctx(), mode);
                    layer.show(ui.ctx());
                },
                {
                    let mut layer = ToastLayer::new();
                    layer.push(Toast::new(Severity::Success, "toast"));
                    layer
                },
            );
        toast.run();
        assert_ruled_with_a_hard_shadow(
            &painted(&toast, mode),
            mode,
            "toast",
            Elevation::Overlay,
            3.0,
        );
    }
}

// ─── the signatures a consumer calls ──────────────────────────────────

/// Each primitive called the way a consumer calls it, with the arguments its
/// signature has always taken. This is a compile-time claim — a changed
/// signature fails to build here — with a run behind it so each call is
/// exercised and not just typed.
#[test]
fn every_primitive_is_still_called_the_way_it_always_was() {
    let mut query = String::new();
    let mut harness = Harness::builder()
        .with_size(egui::vec2(700.0, 700.0))
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), Mode::Light);
            let response = ui.button("focusable");
            let radius: f32 = ui.tokens_radius();
            widgets::focus_ring(ui, response.rect, radius);
            widgets::focus_ring_for(ui, &response);
            let _: egui::Response = key_chip(ui, "Esc");
            let _: egui::Response = widgets::status_pill(ui, &icons::CLOCK, "ok", Role::Success);
            let row: ListRow = ListRow::new(RowHeight::Grid).selected(true);
            let out = list_row(ui, row, |ui, state: RowState| {
                ui.label("row");
                state.selected
            });
            assert!(out.inner);
            let q: meridian_egui::QueryLineResponse = query_line(ui, &mut query, "Search…");
            let _ = q.changed;
            let inner: u8 = overlay_frame(ui, &ModalChrome::new(), |_| 7);
            assert_eq!(inner, 7);
        });
    harness.run();
}

/// `ui.tokens().radius_control`, named so the call above reads as a consumer's.
trait TokensRadius {
    fn tokens_radius(&self) -> f32;
}

impl TokensRadius for egui::Ui {
    fn tokens_radius(&self) -> f32 {
        use meridian_egui::MeridianUi;
        self.tokens().radius_control
    }
}
