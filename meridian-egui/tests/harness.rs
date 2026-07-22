//! The adapter drawn for real, headlessly.
//!
//! The unit tests in `theme.rs` assert the derived `Style`/`Visuals` values; this
//! goes one step further and runs a live `egui::Context` through `egui_kittest`,
//! confirming the `apply` path lays out a themed frame end to end and that the
//! `ui.tokens()` extension resolves against a real `Ui`. `egui_kittest` is a
//! dev-dependency (with the same `wgpu`/`snapshot` features the brightfield
//! consumer uses), so a pixel baseline authored here would mean the same thing
//! there — but this test stays on the CPU tessellation path so it is green on a
//! headless CI runner with no GPU.

use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use meridian_egui::{theme, MeridianUi, Mode};

#[test]
fn a_themed_frame_renders_headlessly() {
    let mut harness = Harness::new_ui(|ui| {
        theme::apply(ui.ctx(), Mode::Light);
        ui.label("Meridian");
        let _ = ui.button("Run");
        // The box-model extension resolves against the live `Ui`, and agrees
        // with the shared constant.
        assert_eq!(ui.tokens().control_gap, meridian_egui::TOKENS.control_gap);
    });

    // A CPU frame — no GPU. Panics if the harness cannot lay the frame out.
    harness.run();

    // The themed nodes are in the rendered tree (panics if absent).
    harness.get_by_label("Meridian");
    harness.get_by_label("Run");
}
