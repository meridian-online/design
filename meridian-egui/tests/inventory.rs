//! Holds `INVENTORY.md` against the crate's public surface (ADR 0011).
//!
//! The surface is read as source text, so the test needs no dependency: every
//! top-level `pub fn`, `pub struct`, `pub enum`, `pub trait` and `pub static`
//! in `src/lib.rs` and in each `pub mod` it declares, followed recursively.
//! Methods (indented) and `pub const`s are not items of their own: a method
//! belongs to its type's row, and constants are data. The `bin/` generator is
//! not the library's surface.
//!
//! Known limits of reading text: an item produced by a macro, or a `pub use`
//! of a glob from outside this crate, would not be seen.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const KINDS: [&str; 5] = ["fn", "struct", "enum", "trait", "static"];

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// The source file behind `pub mod name;` declared in `dir`.
fn module_file(dir: &Path, name: &str) -> PathBuf {
    let flat = dir.join(format!("{name}.rs"));
    if flat.exists() {
        flat
    } else {
        dir.join(name).join("mod.rs")
    }
}

fn collect(file: &Path, dir: &Path, out: &mut BTreeSet<String>) {
    let text = fs::read_to_string(file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("pub mod ") {
            let name = rest.trim_end_matches(';').trim();
            let child = module_file(dir, name);
            let child_dir = child.parent().unwrap().to_path_buf();
            let child_dir = if child.file_name().unwrap() == "mod.rs" {
                child_dir
            } else {
                dir.to_path_buf()
            };
            collect(&child, &child_dir, out);
        }
        for kind in KINDS {
            if let Some(rest) = line.strip_prefix(&format!("pub {kind} ")) {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                assert!(out.insert(name.clone()), "{name} is public twice");
            }
        }
    }
}

fn public_surface() -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    collect(&src().join("lib.rs"), &src(), &mut out);
    out
}

/// The inventory's body rows as cells, header and separator skipped.
fn rows() -> Vec<Vec<String>> {
    let text = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("INVENTORY.md"))
        .expect("meridian-egui/INVENTORY.md exists");
    text.lines()
        .filter(|l| l.trim_start().starts_with('|'))
        .skip(2)
        .map(|l| {
            l.trim()
                .trim_matches('|')
                .split('|')
                .map(|c| c.trim().to_string())
                .collect()
        })
        .collect()
}

fn listed() -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for row in rows() {
        for name in row[0].split(',') {
            let name = name.trim().trim_matches('`').to_string();
            assert!(!name.is_empty(), "empty name in row {row:?}");
            assert!(out.insert(name.clone()), "{name} is listed twice");
        }
    }
    out
}

#[test]
fn every_public_item_has_a_row_and_every_row_names_a_public_item() {
    let surface = public_surface();
    let listed = listed();
    let unlisted: Vec<_> = surface.difference(&listed).collect();
    let phantom: Vec<_> = listed.difference(&surface).collect();
    assert!(
        unlisted.is_empty() && phantom.is_empty(),
        "public but not in INVENTORY.md: {unlisted:?}\nin INVENTORY.md but not public: {phantom:?}"
    );
}

#[test]
fn every_component_row_states_a_door() {
    let mut components = 0;
    for row in rows() {
        assert_eq!(row.len(), 4, "a row has four columns: {row:?}");
        let kind = row[1].as_str();
        assert!(
            ["component", "composition", "helper"].contains(&kind),
            "unknown kind {kind:?} in {row:?}"
        );
        assert!(!row[3].is_empty(), "the fourth column is empty in {row:?}");
        if kind == "component" {
            components += 1;
            assert!(
                ["call sites", "requirement"].contains(&row[2].as_str()),
                "a component states its door: {row:?}"
            );
        }
    }
    assert!(components > 0, "no component rows were read");
}
