//! The gallery example's drawing, run headlessly.
//!
//! The gallery is what a person looks at beside `components/sheet.html`; this
//! file keeps it drawing. The first test lays the whole gallery out and paints
//! it in both modes on the CPU, as every other test here does. The second is
//! ignored by the gate: it renders the gallery through wgpu and writes one PNG
//! per mode into the directory named by `GALLERY_PNG_DIR`, for a person to read
//! beside the sheet. Nothing it writes is committed, and nothing compares it.
//!
//! ```text
//! GALLERY_PNG_DIR=/some/dir cargo test -p meridian-egui --test gallery -- --ignored
//! ```

#[path = "../examples/gallery/body.rs"]
mod body;

use body::GalleryState;
use egui_kittest::kittest::Queryable;
use egui_kittest::Harness;
use meridian_egui::Mode;

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];

/// Wide enough for the widest row of figures and tall enough for every
/// section, so nothing is clipped.
const SIZE: egui::Vec2 = egui::vec2(1400.0, 6200.0);

fn gallery(mode: Mode, wgpu: bool) -> Harness<'static, GalleryState> {
    let mut builder = Harness::builder().with_size(SIZE);
    if wgpu {
        builder = builder.wgpu();
    }
    let mut state = GalleryState::default();
    state.mode = mode;
    let mut harness = builder.build_ui_state(body::show, state);
    harness.run();
    harness
}

#[test]
fn the_gallery_draws_every_section_in_both_modes() {
    for mode in MODES {
        let harness = gallery(mode, false);
        assert_eq!(harness.state().mode, mode);
        for title in [
            "0 · What is already settled",
            "1 · Button",
            "2 · Text field",
            "3 · Ribbon tabs  and  4 · Ribbon row",
            "5 · Popover",
            "6 · Segmented control",
            "7 · Section with a count  and  8 · Distribution bar",
            "9 · Menu item",
            "10 · Step row",
            "11 · Vertical tab",
            "12 · Run status",
            "The three treatments together",
        ] {
            harness.get_by_label(title);
        }
    }
}

#[test]
fn esc_closes_the_open_modal_and_no_popover_under_it_takes_the_key() {
    for mode in MODES {
        let mut harness = gallery(mode, false);
        harness.get_by_label("Open a modal").click();
        harness.run();
        assert!(
            harness.query_by_label("A modal").is_some(),
            "{mode:?}: the modal did not open"
        );
        harness.key_press(egui::Key::Escape);
        harness.run();
        assert!(
            harness.query_by_label("A modal").is_none(),
            "{mode:?}: Esc did not close the modal"
        );
    }
}

#[test]
#[ignore = "renders through wgpu and writes PNGs to GALLERY_PNG_DIR; run by hand"]
fn render_the_gallery_to_png() {
    let dir = std::env::var_os("GALLERY_PNG_DIR")
        .map(std::path::PathBuf::from)
        .expect("set GALLERY_PNG_DIR to the directory the PNGs go in");
    std::fs::create_dir_all(&dir).expect("create GALLERY_PNG_DIR");
    for mode in MODES {
        let mut harness = gallery(mode, true);
        let image = harness.render().expect("render the gallery through wgpu");
        let path = dir.join(match mode {
            Mode::Light => "gallery-light.png",
            Mode::Dark => "gallery-dark.png",
        });
        image.save(&path).expect("write the PNG");
        eprintln!("wrote {}", path.display());
    }
}
