//! The icon regeneration-drift gate — the discipline the generated token
//! scales already use, applied to the generated icon constants: the committed
//! `src/icons/data.rs` must equal, byte for byte, what the committed generator
//! derives from the committed `src/icons/tabler.manifest`. A hand edit to
//! either file (or a kurbo upgrade that changes arc→cubic emission) turns this
//! red, and the fix is always the same: regenerate, never hand-edit.

use meridian_egui::icons;

const MANIFEST: &str = include_str!("../src/icons/tabler.manifest");
const DATA: &str = include_str!("../src/icons/data.rs");

#[test]
fn generated_constants_match_the_manifest_byte_for_byte() {
    let derived = icons::gen::derive(MANIFEST).expect("the committed manifest derives");
    if derived != DATA {
        // Surface the first divergent line: a bare "something differs" is
        // undiagnosable from a CI log.
        let divergence = derived
            .lines()
            .zip(DATA.lines())
            .enumerate()
            .find(|(_, (d, c))| d != c)
            .map(|(i, (d, c))| {
                format!(
                    "first divergent line {}:\n  derived:   {d}\n  committed: {c}",
                    i + 1
                )
            })
            .unwrap_or_else(|| {
                format!(
                    "one file is a prefix of the other (derived {} lines, committed {})",
                    derived.lines().count(),
                    DATA.lines().count()
                )
            });
        panic!(
            "src/icons/data.rs has drifted from src/icons/tabler.manifest — \
             regenerate with: cargo run -p meridian-egui --bin gen-tabler-icons\n{divergence}"
        );
    }
}

#[test]
fn the_set_is_sorted_unique_and_at_the_agreed_size() {
    assert!(
        icons::ALL.len() >= 40,
        "the curated set carries ~40 glyphs; found {}",
        icons::ALL.len()
    );
    for pair in icons::ALL.windows(2) {
        assert!(
            pair[0].name < pair[1].name,
            "ALL must be sorted by unique name ({:?} then {:?}) — by_name binary-searches it",
            pair[0].name,
            pair[1].name
        );
    }
}

#[test]
fn every_glyph_stays_on_the_native_grid() {
    use kurbo::Shape as _;

    for icon in icons::ALL {
        assert!(!icon.els.is_empty(), "{}: empty glyph", icon.name);
        let bounds = icon.bez_path().bounding_box();
        for (axis, lo, hi) in [("x", bounds.x0, bounds.x1), ("y", bounds.y0, bounds.y1)] {
            assert!(
                lo >= -0.5 && hi <= f64::from(icons::GRID) + 0.5,
                "{}: {axis} bounds [{lo}, {hi}] leave the 24×24 grid",
                icon.name
            );
        }
    }
}
