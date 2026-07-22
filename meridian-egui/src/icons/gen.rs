//! The icon generator's pure core: Tabler SVG → manifest entry, and manifest →
//! the generated `data.rs` source. Hidden from docs — it exists for the
//! committed `gen-tabler-icons` bin and for `tests/icon_drift.rs`, which
//! re-derives `data.rs` from the committed manifest and fails on any
//! divergence (the regenerate-never-hand-edit discipline the token scales
//! use).
//!
//! Keeping this in the library (rather than inside the bin) is what makes the
//! drift gate honest: the test and the generator run the *same* derivation,
//! so "the committed output matches the generator" is checked, not assumed.

/// Manifest lines starting with this prefix carry the provenance line that is
/// copied into the generated file's header.
const SOURCE_PREFIX: &str = "# source:";

/// Extract a manifest entry (`name<TAB>d[;d...]`) from one Tabler outline SVG.
///
/// Strips every path with `stroke="none"` — Tabler's invisible bounding-rect
/// path (ADR 0009's ingest rule) — and keeps the visible artwork's `d`
/// strings in document order.
pub fn manifest_entry_from_svg(name: &str, svg: &str) -> Result<String, String> {
    validate_name(name)?;

    let mut ds: Vec<&str> = Vec::new();
    for path in svg.split("<path").skip(1) {
        let path = &path[..path
            .find("/>")
            .ok_or_else(|| format!("{name}: unterminated <path> element"))?];
        if path.contains("stroke=\"none\"") {
            continue; // The invisible bounding-rect path.
        }
        let d = attr(path, "d").ok_or_else(|| format!("{name}: <path> without a d attribute"))?;
        if d.contains('\t') || d.contains(';') {
            return Err(format!(
                "{name}: path data collides with the manifest format"
            ));
        }
        ds.push(d);
    }
    if ds.is_empty() {
        return Err(format!("{name}: no visible paths in the SVG"));
    }
    Ok(format!("{name}\t{}", ds.join(";")))
}

/// Derive the full `data.rs` source from the manifest. Deterministic: the
/// committed file must equal this output byte for byte.
pub fn derive(manifest: &str) -> Result<String, String> {
    let source_line = manifest
        .lines()
        .find(|line| line.starts_with(SOURCE_PREFIX))
        .map(|line| line.trim_start_matches("# ").trim())
        .unwrap_or("source unknown — manifest carried no `# source:` line");

    let mut names: Vec<&str> = Vec::new();
    let mut consts = String::new();
    for line in manifest.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (name, data) = line
            .split_once('\t')
            .ok_or_else(|| format!("malformed manifest line: {line:?}"))?;
        validate_name(name)?;
        if let Some(prev) = names.last() {
            if *prev >= name {
                return Err(format!(
                    "manifest must be sorted by unique name ({prev:?} then {name:?})"
                ));
            }
        }
        names.push(name);

        let mut path = kurbo::BezPath::new();
        for d in data.split(';') {
            let sub = kurbo::BezPath::from_svg(d)
                .map_err(|e| format!("{name}: bad path data {d:?}: {e}"))?;
            path.extend(sub);
        }
        if path.elements().is_empty() {
            return Err(format!("{name}: path data is empty"));
        }

        consts.push_str(&format!(
            "/// Tabler outline `{name}`.\n#[rustfmt::skip]\npub const {}: Icon = Icon {{\n    name: \"{name}\",\n    els: &[\n",
            const_name(name)
        ));
        for el in path.elements() {
            consts.push_str(&format!("        {},\n", fmt_el(*el)));
        }
        consts.push_str("    ],\n};\n\n");
    }
    if names.is_empty() {
        return Err("manifest carried no icons".to_owned());
    }

    let mut all = String::from(
        "/// Every icon in this set, sorted by name (binary-searchable).\n#[rustfmt::skip]\npub const ALL: &[Icon] = &[\n",
    );
    for name in &names {
        all.push_str(&format!("    {},\n", const_name(name)));
    }
    all.push_str("];\n");

    Ok(format!(
        "//! Generated Tabler icon path constants — do not edit by hand.\n\
         //!\n\
         //! {source_line}\n\
         //!\n\
         //! Regenerate with the committed `gen-tabler-icons` bin (the manifest header\n\
         //! carries the full refresh command); `tests/icon_drift.rs` fails when this\n\
         //! file and `tabler.manifest` disagree.\n\
         //!\n\
         //! Icon artwork: Tabler Icons — MIT — © Paweł Kuna and contributors —\n\
         //! <https://tabler.io/icons>.\n\
         \n\
         // Arc→cubic conversion emits whatever coordinates the maths produces; some\n\
         // happen to approximate well-known constants, which means nothing here.\n\
         #![allow(clippy::approx_constant)]\n\
         \n\
         use kurbo::{{PathEl, Point}};\n\
         \n\
         use super::Icon;\n\
         \n\
         {consts}{all}"
    ))
}

/// `chart-bar` → `CHART_BAR`.
fn const_name(name: &str) -> String {
    name.to_uppercase().replace('-', "_")
}

/// Icon names are Tabler's kebab-case slugs; anything else is rejected before
/// it can reach generated source.
fn validate_name(name: &str) -> Result<(), String> {
    let ok = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !name.starts_with(|c: char| c.is_ascii_digit());
    if ok {
        Ok(())
    } else {
        Err(format!("invalid icon name {name:?}"))
    }
}

/// The value of an XML attribute inside an element's raw text, if present.
fn attr<'a>(element: &'a str, name: &str) -> Option<&'a str> {
    let start = element.find(&format!("{name}=\""))? + name.len() + 2;
    let end = start + element[start..].find('"')?;
    Some(&element[start..end])
}

fn fmt_el(el: kurbo::PathEl) -> String {
    use kurbo::PathEl as E;
    match el {
        E::MoveTo(p) => format!("PathEl::MoveTo({})", fmt_point(p)),
        E::LineTo(p) => format!("PathEl::LineTo({})", fmt_point(p)),
        E::QuadTo(p1, p2) => format!("PathEl::QuadTo({}, {})", fmt_point(p1), fmt_point(p2)),
        E::CurveTo(p1, p2, p3) => format!(
            "PathEl::CurveTo({}, {}, {})",
            fmt_point(p1),
            fmt_point(p2),
            fmt_point(p3)
        ),
        E::ClosePath => "PathEl::ClosePath".to_owned(),
    }
}

fn fmt_point(p: kurbo::Point) -> String {
    format!("Point::new({}, {})", fmt_f64(p.x), fmt_f64(p.y))
}

/// Emit a coordinate as a Rust float literal, quantised for cross-platform
/// determinism.
///
/// Arc→cubic conversion runs through the platform's libm (`sin`/`cos`), whose
/// results differ in the last ulp between targets (macOS arm64 vs Linux
/// x86_64) — at full precision the shortest round-trip decimal then differs
/// byte-for-byte across machines, which broke the drift gate in CI. Rounding
/// to six decimal places (≈ 5e-7 on a 24×24 grid — far below anything
/// renderable) absorbs the libm noise: `*`, `round` and `/` are IEEE
/// correctly-rounded, so the quantised value is identical on every platform.
/// `Debug` then prints the shortest decimal that round-trips — always a valid
/// Rust float literal (`3.0`, `0.591`).
fn fmt_f64(v: f64) -> String {
    let q = (v * 1e6).round() / 1e6;
    format!("{q:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24">
  <path stroke="none" d="M0 0h24v24H0z" fill="none" />
  <path d="M4 20h14" />
  <path d="M3 13l4 -4" />
</svg>"#;

    #[test]
    fn svg_ingest_strips_the_bounding_rect_and_keeps_artwork_order() {
        let entry = manifest_entry_from_svg("chart-bar", SVG).unwrap();
        assert_eq!(entry, "chart-bar\tM4 20h14;M3 13l4 -4");
    }

    #[test]
    fn derive_emits_constants_and_the_sorted_table() {
        let manifest = "# source: test set\nchart-bar\tM4 20h14\nx\tM6 6l12 12;M6 18l12 -12\n";
        let out = derive(manifest).unwrap();
        assert!(out.contains("pub const CHART_BAR: Icon"));
        assert!(out.contains("PathEl::MoveTo(Point::new(4.0, 20.0))"));
        assert!(out.contains("PathEl::LineTo(Point::new(18.0, 20.0))"));
        assert!(out.contains("pub const ALL: &[Icon] = &[\n    CHART_BAR,\n    X,\n];"));
        assert!(out.contains("//! source: test set"));
    }

    #[test]
    fn derive_rejects_unsorted_names_and_bad_data() {
        assert!(derive("x\tM0 0\nchart-bar\tM4 20h14\n").is_err());
        assert!(derive("chart-bar\tnot-a-path\n").is_err());
        assert!(derive("Bad Name\tM0 0\n").is_err());
        assert!(derive("# only comments\n").is_err());
    }
}
