//! An eframe window showing every component and composition this crate draws,
//! in the order and under the section numbers of `components/sheet.html`, so the
//! adapter can be looked at beside the sheet rather than only asserted.
//! `eframe` is a dev-dependency, so this example — and the windowing and render
//! stack it pulls — never reaches a consumer that links `meridian-egui`.
//!
//! Run with `cargo run --example gallery -p meridian-egui` (needs a display).
//! Under CI it is *compiled* by `clippy --all-targets`, which keeps it from
//! rotting. The drawing itself is `body::show`, which `tests/gallery.rs` also
//! renders headlessly in both modes.

mod body;

use body::GalleryState;

#[derive(Default)]
struct Gallery {
    state: GalleryState,
}

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| body::show(ui, &mut self.state));
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "meridian-egui gallery",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Gallery::default()))),
    )
}
