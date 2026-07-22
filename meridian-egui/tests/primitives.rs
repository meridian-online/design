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
use meridian_egui::{align, icons, theme, widgets, Mode};

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
        let mut harness = Harness::new_ui(move |ui| {
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
        });
        harness.run();

        harness.get_by_label("Steps");
        harness.get_by_label("ok");
    }
}
