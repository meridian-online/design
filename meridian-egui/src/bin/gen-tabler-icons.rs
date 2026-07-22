//! Offline generator for the committed Tabler icon constants.
//!
//! Two modes, both writing into `src/icons/` of this crate:
//!
//! ```text
//! # Re-emit src/icons/data.rs from the committed manifest (the usual run —
//! # e.g. after the emitter in icons::gen changes):
//! cargo run -p meridian-egui --bin gen-tabler-icons
//!
//! # Refresh the manifest AND data.rs from a directory of Tabler outline SVGs
//! # (download the pinned @tabler/icons npm tarball, extract, point at
//! # icons/outline; only the icon names already in the manifest are taken):
//! cargo run -p meridian-egui --bin gen-tabler-icons -- <svg-dir> <version>
//! ```
//!
//! Network access never happens here — fetching the SVG set is the caller's
//! offline step. `tests/icon_drift.rs` gates that the committed outputs agree
//! with the committed manifest.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use meridian_egui::icons::gen;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("gen-tabler-icons: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let icons_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/icons");
    let manifest_path = icons_dir.join("tabler.manifest");
    let data_path = icons_dir.join("data.rs");

    let args: Vec<String> = std::env::args().skip(1).collect();
    let manifest = match args.as_slice() {
        [] => std::fs::read_to_string(&manifest_path)
            .map_err(|e| format!("read {}: {e}", manifest_path.display()))?,
        [svg_dir, version] => {
            let old = std::fs::read_to_string(&manifest_path)
                .map_err(|e| format!("read {}: {e}", manifest_path.display()))?;
            let manifest = refresh_manifest(&old, Path::new(svg_dir), version)?;
            std::fs::write(&manifest_path, &manifest)
                .map_err(|e| format!("write {}: {e}", manifest_path.display()))?;
            println!("wrote {}", manifest_path.display());
            manifest
        }
        _ => {
            return Err("usage: gen-tabler-icons [<svg-dir> <version>]".to_owned());
        }
    };

    let data = gen::derive(&manifest)?;
    std::fs::write(&data_path, data).map_err(|e| format!("write {}: {e}", data_path.display()))?;
    println!("wrote {}", data_path.display());
    Ok(())
}

/// Rebuild the manifest from a Tabler outline SVG directory, keeping the icon
/// names the old manifest already carries — the set is a curated list, and the
/// list lives in the manifest itself.
fn refresh_manifest(old: &str, svg_dir: &Path, version: &str) -> Result<String, String> {
    let mut names: Vec<&str> = old
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| l.split_once('\t').map_or(l, |(name, _)| name))
        .collect();
    names.sort_unstable();
    names.dedup();
    if names.is_empty() {
        return Err("the existing manifest lists no icon names to refresh".to_owned());
    }

    let mut manifest = format!(
        "# Tabler Icons — MIT — © Paweł Kuna and contributors — https://tabler.io/icons\n\
         # source: @tabler/icons {version}, outline style (24×24 grid, stroke-width 2)\n\
         # format: <name>\\t<svg path d>[;<svg path d>...] — invisible bounding-rect path stripped\n\
         # regenerate: cargo run -p meridian-egui --bin gen-tabler-icons -- <svg-dir> <version>\n"
    );
    for name in names {
        let svg_path = svg_dir.join(format!("{name}.svg"));
        let svg = std::fs::read_to_string(&svg_path)
            .map_err(|e| format!("read {}: {e}", svg_path.display()))?;
        manifest.push_str(&gen::manifest_entry_from_svg(name, &svg)?);
        manifest.push('\n');
    }
    Ok(manifest)
}
