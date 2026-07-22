//! A minimal eframe window that applies the Meridian theme, so the adapter can
//! be looked at rather than only asserted. `eframe` is a dev-dependency, so this
//! example — and the windowing/render stack it pulls — never reaches a consumer
//! that links `meridian-egui`.
//!
//! Run with `cargo run --example gallery` (needs a display). Under CI it is
//! *compiled* by `clippy --all-targets`, which is what keeps it from rotting.

use meridian_egui::{theme, MeridianUi, Mode};

#[derive(Default)]
struct Gallery {
    mode: Mode,
    name: String,
}

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // The Meridian style, fonts, spacing, radii and shadows, applied to the
        // live context each frame (idempotent).
        theme::apply(ui.ctx(), self.mode);

        let t = ui.tokens();
        ui.horizontal(|ui| {
            ui.heading("Meridian · egui");
            ui.add_space(t.section_gap);
            if ui.button("Toggle mode").clicked() {
                self.mode = self.mode.toggled();
            }
        });
        ui.add_space(t.section_gap);

        ui.label("Controls draw in Meridian ink, type, spacing and radii:");
        ui.add_space(t.control_gap);
        ui.horizontal(|ui| {
            let _ = ui.button("Primary");
            ui.add_space(t.control_gap);
            let _ = ui.button("Secondary");
        });
        ui.add_space(t.section_gap);
        ui.text_edit_singleline(&mut self.name);
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "meridian-egui gallery",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Gallery::default()))),
    )
}
