//! Conformance: emitted artefacts are pinned exactly. If an intentional token
//! change lands, regenerate the snapshot in the same commit — a diff here in
//! CI means a consumer would have drifted.

/// The CSS artefact carries the non-colour layer too, so the web can stop
/// re-declaring its own radius and spacing scales.
#[test]
fn tokens_css_carries_the_geometry_layer() {
    use meridian_design::{control, focus, motion, radius, spacing, typography};

    let css = meridian_design::emit::tokens_css();
    for (name, value) in [
        (
            "--m-radius-control",
            format!("{}px", radius::CONTROL as u32),
        ),
        ("--m-radius-panel", format!("{}px", radius::PANEL as u32)),
        ("--m-space-4", format!("{}px", spacing::SPACE_4 as u32)),
        ("--m-row-dense", format!("{}px", spacing::ROW_DENSE as u32)),
        ("--m-control-sm", format!("{}px", control::HEIGHT_SM as u32)),
        ("--m-icon-sm", format!("{}px", control::ICON_SM as u32)),
        (
            "--m-focus-ring-width",
            format!("{}px", focus::RING_WIDTH as u32),
        ),
        ("--m-motion-spatial", format!("{}ms", motion::SPATIAL_MS)),
        (
            "--m-motion-animation-time",
            format!("{}s", motion::ANIMATION_TIME),
        ),
        (
            "--m-font-size-ui",
            format!("{}px", typography::UI_SIZE as u32),
        ),
    ] {
        assert!(
            css.contains(&format!("{name}: {value};")),
            "{name} missing or wrong in tokens.css"
        );
    }
    // Shadows exist only above the working plane — two, never four.
    assert!(css.contains("--m-shadow-overlay:"));
    assert!(css.contains("--m-shadow-modal:"));
    assert!(!css.contains("--m-shadow-raised"));
    assert!(!css.contains("--m-shadow-flat"));
}

/// Every semantic slot reaches the web artefact, in both modes.
#[test]
fn tokens_css_carries_the_semantic_layer_in_both_modes() {
    let css = meridian_design::emit::tokens_css();
    let (light, dark) = css.split_once("\n.dark {").expect("two mode blocks");
    for block in [light, dark] {
        for name in [
            "--m-surface-app",
            "--m-surface-raised",
            "--m-surface-overlay",
            "--m-border-subtle",
            "--m-border-control",
            "--m-border-focus",
            "--m-text-primary",
            "--m-text-on-solid",
            "--m-accent-bg",
            "--m-accent-bg-hover",
            "--m-accent-fg-disabled",
            "--m-danger-border-focus",
            "--m-rows-selected-bg",
            "--m-scrollbar-thumb",
            "--m-popover-bg",
        ] {
            assert!(
                block.contains(&format!("{name}:")),
                "{name} missing from a mode block"
            );
        }
    }
}

#[test]
fn bundled_fonts_are_present_and_nonempty() {
    for bytes in meridian_design::fonts::ALL {
        assert!(bytes.len() > 100_000, "a bundled font file looks truncated");
    }
}

#[test]
fn tokens_css_matches_snapshot() {
    let expected = include_str!("snapshots/tokens.css");
    assert_eq!(
        meridian_design::emit::tokens_css(),
        expected,
        "emit::tokens_css() no longer matches tests/snapshots/tokens.css — \
         regenerate the snapshot if the change is intentional"
    );
}

// ---------------------------------------------------------------------------
// The reference sheet — `reference/tokens.md`.
//
// Unlike `tokens.css`, the published file *is* the pin: there is no second
// copy under `tests/snapshots/`, because the sheet's reader is a person and a
// snapshot nobody reads would be the only version anyone checked.
// ---------------------------------------------------------------------------

/// Every `--*` declaration in one mode block, as the sheet's own reader sees
/// them. Deliberately a second, independent parse: if this and the emitter's
/// reader disagreed about what a declaration is, the tests below would be
/// checking the emitter against itself.
fn declarations(block: &str) -> Vec<(String, String)> {
    block
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("--"))
        .filter_map(|l| l.split_once(": "))
        .map(|(n, v)| (n.to_string(), v.trim_end_matches(';').to_string()))
        .collect()
}

#[test]
fn tokens_md_matches_the_published_sheet() {
    let expected = include_str!("../../reference/tokens.md");
    assert_eq!(
        meridian_design::emit::tokens_md(),
        expected,
        "emit::tokens_md() no longer matches reference/tokens.md — regenerate \
         in the same commit if the change is intentional: \
         cargo run --example dump_md > ../reference/tokens.md"
    );
}

/// Every token the web is given is a token the sheet shows.
///
/// This is the claim a snapshot cannot make. The pin above compares the sheet
/// against its own past output, so a token that stopped being rendered would
/// simply produce a new snapshot and pass forever after — and a reference that
/// silently omits a token is worse than no reference, because the omission
/// reads as "this token does not exist".
///
/// Two shapes of evidence, because the sheet renders two shapes of table. Flat
/// tables print the name, so the name is the claim. The scale and role
/// matrices print a step or a state down the side and values in the cells —
/// which is what makes 252 declarations readable — so there the value inside
/// that section is the claim. The section boundaries come from the document
/// itself rather than from a list restated here.
#[test]
fn the_sheet_carries_every_token_the_css_emits() {
    let css = meridian_design::emit::tokens_css();
    let md = meridian_design::emit::tokens_md();
    let (light, dark) = css.split_once("\n.dark {").expect("two mode blocks");

    let region = |from: &str, to: &str| -> String {
        let start = md
            .find(from)
            .unwrap_or_else(|| panic!("the sheet has a {from} section"));
        let end = md[start..]
            .find(to)
            .expect("a section is followed by another")
            + start;
        md[start..end].to_string()
    };
    let matrices = format!(
        "{}{}",
        region("## Colour scales", "## Chart palettes"),
        region("## Interaction roles", "## Component surfaces")
    );

    let mut checked = 0;
    for block in [light, dark] {
        for (name, value) in declarations(block) {
            let named = md.contains(&format!("`{name}`"));
            let in_matrix = matrices.contains(&format!("`{value}`"));
            assert!(
                named || in_matrix,
                "{name} reaches tokens.css and never reaches the sheet — it is \
                 neither named in a table nor drawn in the scale or role matrices"
            );
            checked += 1;
        }
    }
    assert!(
        checked > 500,
        "only {checked} declarations parsed — parse error?"
    );
}

/// Nothing lands in the catch-all.
///
/// The emitter files declarations by name prefix, and anything no section
/// claims is printed under `## Unclassified` rather than dropped. A new family
/// of tokens therefore fails here until somebody decides which heading it
/// belongs under and what the sentence above it should say — which is the
/// judgement the generator cannot make and the one most worth forcing.
#[test]
fn the_sheet_files_every_token_under_a_heading() {
    assert!(
        !meridian_design::emit::tokens_md().contains("## Unclassified"),
        "the sheet has tokens no section claims — see the Unclassified table \
         for which, and give them a home in emit/markdown.rs"
    );
}

/// The floors the sheet publishes are the floors CI enforces.
///
/// The sheet's whole argument for printing contrast numbers is that each one
/// is already gated. That argument fails the moment the published floor and
/// the asserted floor are two different literals, so both now read
/// `validate::TEXT_MIN` and `validate::NON_TEXT_MIN` and this derives its
/// expectation from the same constants rather than from the document.
#[test]
fn the_sheet_publishes_the_gated_floors() {
    use meridian_design::validate::{NON_TEXT_MIN, TEXT_MIN};

    let md = meridian_design::emit::tokens_md();
    for floor in [TEXT_MIN, NON_TEXT_MIN] {
        assert!(
            md.contains(&format!("`{floor}`")),
            "the sheet does not publish the {floor} floor it measures against"
        );
    }
    assert!(
        md.contains(&format!("| primary | {TEXT_MIN} |")),
        "load-bearing text no longer carries the gate's own floor in the sheet"
    );
}

/// The ratios are measured, not typed.
///
/// The failure this guards against is the one that produced every structural
/// test in this file: a value restated as a literal, correct on the day it was
/// written and quietly wrong afterwards. Contrast is the most tempting place
/// for it, because the numbers look stable and recomputing them is work.
///
/// So the expectation is recomputed here from `semantic.rs` through the same
/// `validate::contrast` the chrome gate uses, and searched for in the sheet.
/// Change an ink token and this fails until the sheet's number moves with it.
#[test]
fn the_sheet_measures_contrast_rather_than_restating_it() {
    use meridian_design::semantic::semantic;
    use meridian_design::validate::contrast;

    let md = meridian_design::emit::tokens_md();
    let mut rows = 0;
    for dark in [false, true] {
        let s = semantic(dark);
        for (slot, ink) in [
            ("primary", s.text.primary),
            ("secondary", s.text.secondary),
            ("muted", s.text.muted),
        ] {
            // The row as the sheet must render it: the slot, its floor, then a
            // ratio per plane in the gate's own order.
            let cells: Vec<String> = [
                s.surfaces.app,
                s.surfaces.raised,
                s.surfaces.sunken,
                s.surfaces.overlay,
                s.surfaces.sidebar,
                s.surfaces.header,
            ]
            .iter()
            .map(|bg| format!("{:.2}", contrast(ink, *bg)))
            .collect();
            let row = format!("| {} |", cells.join(" | "));
            assert!(
                md.contains(&row),
                "the {} row for text.{slot} is not what contrast() computes today \
                 ({row}) — a published ratio has stopped tracking its token",
                if dark { "dark" } else { "light" }
            );
            rows += 1;
        }
    }
    assert_eq!(rows, 6, "both modes and three slots each");
}

/// Every colour the sheet prints is a token value.
///
/// The sheet reads `tokens_css()`, so this should hold by construction — which
/// is exactly why it is worth asserting. Prose is written by hand, and an
/// example hex typed into a sentence is how a reference starts documenting a
/// colour the system does not have.
#[test]
fn every_colour_in_the_sheet_is_a_token_value() {
    // Every `#rrggbb` or `#rrggbbaa` in a document. Six or eight digits and
    // nothing else, which is what separates a colour from a markdown heading —
    // and from the hex *inside* a shadow, whose declaration value is a
    // compound `0px 4px 12px #231f1c1f` rather than a bare colour.
    fn hexes(text: &str) -> std::collections::HashSet<String> {
        text.match_indices('#')
            .map(|(at, _)| {
                text[at..]
                    .chars()
                    .take_while(|c| *c == '#' || c.is_ascii_hexdigit())
                    .collect::<String>()
            })
            .filter(|h| h.len() == 7 || h.len() == 9)
            .collect()
    }

    let allowed = hexes(&meridian_design::emit::tokens_css());
    let used = hexes(&meridian_design::emit::tokens_md());
    assert!(allowed.len() > 100, "parse error — the CSS has colours");

    let mut invented: Vec<&String> = used.difference(&allowed).collect();
    let mut missing: Vec<&String> = allowed.difference(&used).collect();
    invented.sort();
    missing.sort();
    assert!(
        invented.is_empty(),
        "the sheet prints {invented:?}, which tokens.css does not emit — a \
         reference has started inventing colours"
    );
    assert!(
        missing.is_empty(),
        "the system defines {missing:?} and the sheet shows them nowhere — a \
         reference that omits a colour reads as a colour that does not exist"
    );
}

/// Both palette sheets are pinned, and both are still the tokens' own colours.
///
/// The sheet exists because a hex code renders as text on GitHub and a design
/// system's palette should be visible. That only holds while every fill is a
/// token value: an SVG is the easiest artefact in the repo to nudge toward
/// "looking right", because a chip whose colour was tweaked by hand looks
/// exactly like a chip whose colour is correct.
#[test]
fn the_palette_sheets_are_pinned_and_drawn_in_tokens() {
    let sheets = [
        (
            "palette.svg",
            include_str!("../../reference/palette.svg"),
            false,
        ),
        (
            "palette_dark.svg",
            include_str!("../../reference/palette_dark.svg"),
            true,
        ),
    ];

    // Every colour the CSS emits, in the mode's own resolution: the dark block
    // redefines a subset and the rest cascades, so a dark sheet may legitimately
    // draw a light-block value that dark never overrides.
    let css = meridian_design::emit::tokens_css();
    let (light, dark_block) = css.split_once("\n.dark {").expect("two mode blocks");

    for (name, pinned, dark) in sheets {
        assert_eq!(
            meridian_design::emit::palette_svg(dark),
            pinned,
            "emit::palette_svg({dark}) no longer matches reference/{name} — \
             regenerate in the same commit if the change is intentional: \
             cargo run --example dump_palette {} > ../reference/{name}",
            if dark { "dark" } else { "light" }
        );

        let mut allowed: Vec<String> = declarations(light).into_iter().map(|(_, v)| v).collect();
        if dark {
            allowed.extend(declarations(dark_block).into_iter().map(|(_, v)| v));
        }
        let mut drawn = 0;
        for (at, _) in pinned.match_indices('#') {
            let hex: String = pinned[at..]
                .chars()
                .take_while(|c| *c == '#' || c.is_ascii_hexdigit())
                .collect();
            if hex.len() != 7 && hex.len() != 9 {
                continue;
            }
            assert!(
                allowed.contains(&hex),
                "{name} draws {hex}, which is not a colour tokens.css emits in that \
                 mode — a swatch has been picked rather than read"
            );
            drawn += 1;
        }
        assert!(
            drawn > 90,
            "{name}: only {drawn} colours drawn — parse error?"
        );
    }
}

/// The palette sheets fetch nothing and script nothing.
///
/// Same claim `tests/motion.rs` makes of the brand artefacts, for the same
/// reason: the site ships no runtime, GitHub serves these under a policy that
/// forbids every outbound request, and a sheet that reached for a font file or
/// a script would simply fail to draw where it matters most.
///
/// Asserted against the emitter's output rather than the committed file, so it
/// fires on the change that introduces the problem instead of waiting for
/// somebody to regenerate. The pin above already watches the file.
#[test]
fn the_palette_sheets_are_self_contained() {
    for (name, svg) in [
        ("palette.svg", meridian_design::emit::palette_svg(false)),
        ("palette_dark.svg", meridian_design::emit::palette_svg(true)),
    ] {
        for reach in ["<script", "href=", "src=", "url(", "@import", "<image"] {
            assert!(
                !svg.contains(reach),
                "{name} contains {reach:?} — it is served under a policy that blocks \
                 every outbound request, so this would draw as nothing"
            );
        }
        // No `<style>` and no `id`: an inlined SVG's styles apply to the whole
        // host document, and its identifiers join the host's namespace.
        assert!(
            !svg.contains("<style") && !svg.contains(" id="),
            "{name} carries a style block or an id, so inlining it would reach \
             outside itself"
        );
    }
}

/// The sheet shows the picture, and the picture has a dark twin.
#[test]
fn the_reference_sheet_embeds_the_palette() {
    let md = meridian_design::emit::tokens_md();
    for src in ["palette.svg", "palette_dark.svg"] {
        assert!(
            md.contains(src),
            "the reference sheet no longer shows {src} — the numbers are back to \
             being the only representation of the palette"
        );
    }
}
