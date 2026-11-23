//! Holds `INVENTORY.md` against the crate's public surface (ADR 0011).
//!
//! The surface is read as source text, so the test needs no dependency. Every
//! top-level public `fn` (with any of `const`, `async`, `unsafe`, `extern`),
//! `struct`, `enum`, `union`, `trait`, `type` and `static` in `src/lib.rs`
//! needs a row, and so does the same in each `pub mod` it declares, followed
//! recursively, in an inline `pub mod x { .. }`, and in a private `mod x;`
//! whose items are re-exported with `pub use x::*` or `pub use x::{A, B}`.
//! Methods (indented inside an `impl`) belong to their type's row, and `pub
//! const`s are data with no row. The `bin/` generator is not the library's
//! surface.
//!
//! A top-level `pub` line the reader does not recognise fails the test, so a
//! new form cannot slip past. What it cannot see: an item produced by a macro,
//! and a `pub use` of an item from outside this crate.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const KINDS: [&str; 7] = ["fn", "struct", "enum", "union", "trait", "type", "static"];
const QUALIFIERS: [&str; 2] = ["async ", "unsafe "];

fn src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// The source file behind `mod name;` declared in `dir`, and the directory its
/// own child modules live in (`dir/name/` for `name.rs`, `dir/name/` for
/// `name/mod.rs`).
fn module_file(dir: &Path, name: &str) -> (PathBuf, PathBuf) {
    let flat = dir.join(format!("{name}.rs"));
    let child_dir = dir.join(name);
    if flat.exists() {
        (flat, child_dir)
    } else {
        (child_dir.join("mod.rs"), child_dir)
    }
}

fn ident(s: &str) -> String {
    s.chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// The item a `pub ...` line declares: `Some(name)` for an item that needs a
/// row, `None` for a `mod`, `use` or `const`. Panics on a form not known.
fn declared(rest: &str, line: &str) -> Option<String> {
    let mut rest = rest;
    let mut was_const = false;
    loop {
        if let Some(r) = rest.strip_prefix("const ") {
            was_const = true;
            rest = r;
        } else if let Some(r) = QUALIFIERS.iter().find_map(|q| rest.strip_prefix(q)) {
            rest = r;
        } else if let Some(r) = rest.strip_prefix("extern ") {
            let r = r.trim_start();
            rest = match r.strip_prefix('"') {
                Some(abi) => abi
                    .split_once('"')
                    .map_or(r, |(_, after)| after.trim_start()),
                None => r,
            };
        } else {
            break;
        }
    }
    for kind in KINDS {
        if let Some(r) = rest.strip_prefix(&format!("{kind} ")) {
            let r = r.strip_prefix("mut ").unwrap_or(r);
            return Some(ident(r));
        }
    }
    if rest.starts_with("mod ") || rest.starts_with("use ") || was_const {
        return None;
    }
    panic!("the inventory test does not know this public form: {line:?}");
}

/// Names a `pub use name::...;` statement re-exports from `name`, or `None`
/// for a glob.
fn reexported(statement: &str, module: &str) -> Option<Option<BTreeSet<String>>> {
    let tail = statement
        .strip_prefix("pub use ")?
        .trim_start()
        .strip_prefix(module)?
        .strip_prefix("::")?
        .trim_end_matches(';');
    assert!(
        !tail.contains(" as "),
        "a renamed re-export is not read: {statement:?}"
    );
    if tail.trim() == "*" {
        return Some(None);
    }
    let names = tail
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect();
    Some(Some(names))
}

fn collect(file: &Path, dir: &Path, out: &mut BTreeSet<String>) {
    let text = fs::read_to_string(file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
    let mut private: Vec<(String, BTreeSet<String>)> = Vec::new();
    let mut uses: Vec<String> = Vec::new();
    let mut pending_use = String::new();
    let mut inline_mod = false;
    let add = |name: String, out: &mut BTreeSet<String>| {
        assert!(out.insert(name.clone()), "{name} is public twice");
    };
    for line in text.lines() {
        if !pending_use.is_empty() {
            pending_use.push(' ');
            pending_use.push_str(line.trim());
            if line.contains(';') {
                uses.push(std::mem::take(&mut pending_use));
            }
            continue;
        }
        if inline_mod {
            if line.starts_with('}') {
                inline_mod = false;
            } else if let Some(rest) = line.strip_prefix("    pub ") {
                if let Some(name) = declared(rest, line) {
                    add(name, out);
                }
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("mod ") {
            if let Some(name) = rest.strip_suffix(';') {
                let (child, child_dir) = module_file(dir, name.trim());
                let mut items = BTreeSet::new();
                collect(&child, &child_dir, &mut items);
                private.push((name.trim().to_string(), items));
            }
            continue;
        }
        let Some(rest) = line.strip_prefix("pub ") else {
            continue;
        };
        if let Some(r) = rest.strip_prefix("mod ") {
            if let Some(name) = r.strip_suffix(';') {
                let (child, child_dir) = module_file(dir, name.trim());
                collect(&child, &child_dir, out);
            } else if r.trim_end().ends_with('{') {
                inline_mod = true;
            } else {
                panic!("the inventory test does not know this module form: {line:?}");
            }
        } else if rest.starts_with("use ") {
            if line.contains(';') {
                uses.push(line.to_string());
            } else {
                pending_use = line.trim().to_string();
            }
        } else if let Some(name) = declared(rest, line) {
            add(name, out);
        }
    }
    for (module, items) in &private {
        for statement in &uses {
            match reexported(statement, module) {
                Some(None) => items.iter().for_each(|i| {
                    out.insert(i.clone());
                }),
                Some(Some(names)) => items.intersection(&names).for_each(|i| {
                    out.insert(i.clone());
                }),
                None => {}
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
