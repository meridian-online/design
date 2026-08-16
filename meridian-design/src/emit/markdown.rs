//! The reference sheet — every emitted token, in one browsable document.
//!
//! This is the second artefact the crate emits, and the first with no
//! consuming repository: nothing builds it, and its reader is a person. That
//! makes it the artefact most exposed to the failure `CLAUDE.md` warns about,
//! because a hand-written table of hexes looks right for as long as anyone
//! checks it and drifts silently forever after.
//!
//! So it restates nothing. **The sheet reads `tokens_css()` and renders what
//! it finds**, the same rule `motion/svg_form.py` follows against the built
//! Lottie rather than re-choreographing the schedule. A token added to the
//! CSS emitter appears here without anybody editing this file; a token whose
//! value changes changes in both places or in neither.
//!
//! What is written here is editorial and nothing else — which heading a family
//! of tokens belongs under, and the sentence explaining what the family is
//! for. Those are judgements a machine cannot make. Everything a machine can
//! make, it makes: names, values, both modes, and every contrast ratio, which
//! is measured with the same `validate::contrast` the chrome gate uses,
//! against the same floors, so the published number and the gated number
//! cannot be two different numbers.
//!
//! A family nobody has classified lands under `## Unclassified`, which
//! `tests/conformance.rs` fails on. That is the point: a new token group has
//! to be filed deliberately, and the alternative — dropping it silently — is
//! exactly the drift the sheet exists to avoid.

use crate::colour::Rgba;
use crate::semantic::{semantic, Role, Semantic};
use crate::validate::{contrast, NON_TEXT_MIN, TEXT_MIN};

use super::STATES;

/// One `--m-*` declaration, paired across the artefact's two mode blocks.
struct Token {
    name: String,
    light: String,
    /// `None` when the dark block does not redefine it — the CSS cascade
    /// leaves those at their light value on purpose.
    dark: Option<String>,
}

/// Every `--*` declaration in one mode block, in the order it was emitted.
fn declarations(block: &str) -> Vec<(String, String)> {
    block
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with("--") {
                return None;
            }
            let (name, value) = line.split_once(": ")?;
            Some((name.to_string(), value.trim_end_matches(';').to_string()))
        })
        .collect()
}

/// The CSS artefact, read back as paired declarations.
fn tokens() -> Vec<Token> {
    let css = super::tokens_css();
    let (light, dark) = css
        .split_once("\n.dark {")
        .expect("the artefact has two mode blocks");
    let dark = declarations(dark);
    declarations(light)
        .into_iter()
        .map(|(name, light)| {
            let dark = dark
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, v)| v.to_string());
            Token { name, light, dark }
        })
        .collect()
}

/// A contrast ratio, or a dash where the number would be a lie.
///
/// `validate::contrast` reads only the colour channels, which is correct for
/// every opaque token and wrong for a wash: handing it `surfaces.scrim` would
/// measure the ink the scrim is made from rather than the result of painting
/// it over something. There is no compositing maths in the crate and no gate
/// for one, so the sheet declines to answer instead of guessing.
fn ratio(fg: Rgba, bg: Rgba) -> String {
    if fg.a < 1.0 || bg.a < 1.0 {
        return "—".to_string();
    }
    format!("{:.2}", contrast(fg, bg))
}

/// The surfaces a control or a line of text can be drawn on.
///
/// This is `tests/chrome_gate.rs`'s list, which is deliberately one longer
/// than `Semantic::planes()` — that method omits `header`, and a sheet that
/// iterated it would publish a narrower claim than CI actually holds.
fn planes(s: &Semantic) -> [(&'static str, Rgba); 6] {
    [
        ("app", s.surfaces.app),
        ("raised", s.surfaces.raised),
        ("sunken", s.surfaces.sunken),
        ("overlay", s.surfaces.overlay),
        ("sidebar", s.surfaces.sidebar),
        ("header", s.surfaces.header),
    ]
}

/// The sheet under construction, and which tokens have found a home.
struct Sheet {
    tokens: Vec<Token>,
    taken: Vec<bool>,
    md: String,
}

impl Sheet {
    fn new() -> Self {
        let tokens = tokens();
        Sheet {
            taken: vec![false; tokens.len()],
            tokens,
            md: String::new(),
        }
    }

    fn line(&mut self, s: &str) {
        self.md.push_str(s);
        self.md.push('\n');
    }

    /// Claim declarations by exact name, in the order the CSS emitted them.
    fn exact(&mut self, names: &[&str]) -> Vec<usize> {
        self.claim(&mut |n: &str| names.contains(&n))
    }

    /// Claim declarations by name prefix.
    fn prefix(&mut self, prefixes: &[&str]) -> Vec<usize> {
        self.claim(&mut |n: &str| prefixes.iter().any(|p| n.starts_with(p)))
    }

    fn claim(&mut self, matches: &mut dyn FnMut(&str) -> bool) -> Vec<usize> {
        let mut hit = Vec::new();
        for (i, t) in self.tokens.iter().enumerate() {
            if !self.taken[i] && matches(&t.name) {
                hit.push(i);
            }
        }
        for &i in &hit {
            self.taken[i] = true;
        }
        hit
    }

    /// Render claimed declarations as a name/value table.
    fn table(&mut self, rows: &[usize]) {
        if rows.is_empty() {
            return;
        }
        let themed = rows.iter().any(|&i| self.tokens[i].dark.is_some());
        let partial = themed && rows.iter().any(|&i| self.tokens[i].dark.is_none());
        if themed {
            self.line("| Token | Light | Dark |");
            self.line("|---|---|---|");
        } else {
            self.line("| Token | Value |");
            self.line("|---|---|");
        }
        for &i in rows {
            let row = if themed {
                format!(
                    "| `{}` | `{}` | {} |",
                    self.tokens[i].name,
                    self.tokens[i].light,
                    match &self.tokens[i].dark {
                        Some(d) => format!("`{d}`"),
                        None => "—".to_string(),
                    }
                )
            } else {
                format!("| `{}` | `{}` |", self.tokens[i].name, self.tokens[i].light)
            };
            self.line(&row);
        }
        if partial {
            self.line("");
            self.line(
                "A dash means the dark block does not redefine the token and the light \
                 value stands — the web artefact leans on the cascade for that, so a \
                 value appearing once is a decision rather than an omission.",
            );
        }
        self.line("");
    }

    /// The value of one declaration in one mode, for the matrix renderers.
    fn value(&self, name: &str, dark: bool) -> String {
        match self.tokens.iter().find(|t| t.name == name) {
            Some(t) if dark => t.dark.clone().unwrap_or_else(|| t.light.clone()),
            Some(t) => t.light.clone(),
            None => "—".to_string(),
        }
    }
}

/// The six 12-step ramps, as a step-by-hue matrix rather than 144 flat rows.
fn scales(sheet: &mut Sheet) {
    const HUES: [&str; 6] = ["gray", "maritime", "red", "amber", "green", "blue"];
    let prefixes: Vec<String> = HUES.iter().map(|h| format!("--m-{h}-")).collect();
    let refs: Vec<&str> = prefixes.iter().map(|s| s.as_str()).collect();
    let steps = sheet.prefix(&refs).len() / HUES.len();

    for (mode, dark) in [("Light", false), ("Dark", true)] {
        let mut rows = vec![
            format!("#### {mode}"),
            String::new(),
            format!("| Step | {} |", HUES.join(" | ")),
            format!("|---|{}", "---|".repeat(HUES.len())),
        ];
        for step in 1..=steps {
            let cells: Vec<String> = HUES
                .iter()
                .map(|h| format!("`{}`", sheet.value(&format!("--m-{h}-{step}"), dark)))
                .collect();
            rows.push(format!("| {step} | {} |", cells.join(" | ")));
        }
        rows.push(String::new());
        for row in rows {
            sheet.line(&row);
        }
    }
}

/// One table per interaction role: six states down, three channels across,
/// both modes side by side.
fn roles(sheet: &mut Sheet) {
    let prefixes: Vec<String> = Role::ALL
        .iter()
        .map(|r| format!("--m-{}-", r.name()))
        .collect();
    let refs: Vec<&str> = prefixes.iter().map(|s| s.as_str()).collect();
    sheet.prefix(&refs);

    for role in Role::ALL {
        let mut rows = vec![
            format!("#### `{}`", role.name()),
            String::new(),
            "| State | Light bg | Light fg | Light border | Dark bg | Dark fg | Dark border |"
                .to_string(),
            "|---|---|---|---|---|---|---|".to_string(),
        ];
        for state in STATES {
            // "base" has no suffix in the CSS; every other state is the
            // suffix without its leading hyphen, so the two cannot drift.
            let label = if state.is_empty() {
                "base"
            } else {
                &state[1..]
            };
            let cells: Vec<String> = [false, true]
                .iter()
                .flat_map(|&dark| {
                    ["bg", "fg", "border"].map(|channel| {
                        format!(
                            "`{}`",
                            sheet.value(&format!("--m-{}-{channel}{state}", role.name()), dark)
                        )
                    })
                })
                .collect();
            rows.push(format!("| {label} | {} |", cells.join(" | ")));
        }
        rows.push(String::new());
        for row in rows {
            sheet.line(&row);
        }
    }
}

/// Measured contrast — every ratio here is one a gate holds to a floor.
fn measured(sheet: &mut Sheet) {
    sheet.line("## Measured contrast");
    sheet.line("");
    sheet.line(&format!(
        "Every ratio below is computed by `validate::contrast` — WCAG 2.x relative \
         luminance, the method ADRs 0006 and 0007 settle for this system — and every \
         one of them is a pair `tests/chrome_gate.rs` or `tests/palette_gate.rs` \
         already holds to a floor. Nothing measured here is unguarded, because a \
         number published without a gate behind it is a claim that can quietly stop \
         being true. The floors are `{TEXT_MIN}` for text that carries meaning and for \
         ink on a solid, and `{NON_TEXT_MIN}` for non-text boundaries and quiet ink."
    ));
    sheet.line("");

    // Text ink on every surface it can land on.
    for (mode, dark) in [("Light", false), ("Dark", true)] {
        let s = semantic(dark);
        sheet.line(&format!("### Text ink — {} mode", mode.to_lowercase()));
        sheet.line("");
        let cols = planes(s);
        sheet.line(&format!(
            "| Slot | Floor | {} |",
            cols.iter().map(|(n, _)| *n).collect::<Vec<_>>().join(" | ")
        ));
        sheet.line(&format!("|---|---|{}", "---|".repeat(cols.len())));
        let slots: [(&str, Rgba, Option<f64>); 8] = [
            ("primary", s.text.primary, Some(TEXT_MIN)),
            ("secondary", s.text.secondary, Some(TEXT_MIN)),
            ("link", s.text.link, Some(TEXT_MIN)),
            ("link-hover", s.text.link_hover, Some(TEXT_MIN)),
            ("link-active", s.text.link_active, Some(TEXT_MIN)),
            ("muted", s.text.muted, Some(NON_TEXT_MIN)),
            ("placeholder", s.text.placeholder, Some(NON_TEXT_MIN)),
            ("disabled", s.text.disabled, None),
        ];
        for (name, ink, floor) in slots {
            let cells: Vec<String> = cols.iter().map(|(_, bg)| ratio(ink, *bg)).collect();
            let floor = match floor {
                Some(f) => format!("{f}"),
                None => "exempt".to_string(),
            };
            sheet.line(&format!("| {name} | {floor} | {} |", cells.join(" | ")));
        }
        sheet.line("");
    }
    sheet.line(
        "`disabled` carries no floor because WCAG exempts disabled controls; the gate \
         holds it to rank order instead — quieter than `placeholder`, and never louder \
         than the slot above it.",
    );
    sheet.line("");

    // Ink on every solid, in the states where the pair is actually visible.
    sheet.line("### Ink on a solid");
    sheet.line("");
    sheet.line(&format!(
        "A role's foreground against its own background, floor `{TEXT_MIN}`. The \
         `focus` state repeats `base` by construction and `disabled` is exempt, so \
         neither is gated and neither is printed."
    ));
    sheet.line("");
    sheet.line("| Role | Light base | Light hover | Light active | Light selected | Dark base | Dark hover | Dark active | Dark selected |");
    sheet.line("|---|---|---|---|---|---|---|---|---|");
    for role in Role::ALL {
        let mut cells = Vec::new();
        for dark in [false, true] {
            let rc = semantic(dark).role(role);
            let bg = rc.background.all();
            let fg = rc.foreground.all();
            for i in 0..4 {
                cells.push(ratio(fg[i], bg[i]));
            }
        }
        sheet.line(&format!("| `{}` | {} |", role.name(), cells.join(" | ")));
    }
    sheet.line("");

    // The boundaries that have to be findable.
    sheet.line("### Findable boundaries");
    sheet.line("");
    sheet.line(&format!(
        "The line that identifies a control, and the ring that says where focus is. \
         Floor `{NON_TEXT_MIN}` on every surface either can be drawn over."
    ));
    sheet.line("");
    for (mode, dark) in [("Light", false), ("Dark", true)] {
        let s = semantic(dark);
        let cols = planes(s);
        sheet.line(&format!("#### {mode}"));
        sheet.line("");
        sheet.line(&format!(
            "| Token | {} |",
            cols.iter().map(|(n, _)| *n).collect::<Vec<_>>().join(" | ")
        ));
        sheet.line(&format!("|---|{}", "---|".repeat(cols.len())));
        for (name, ink) in [
            ("border-control", s.borders.control),
            ("border-focus", s.borders.focus),
        ] {
            let cells: Vec<String> = cols.iter().map(|(_, bg)| ratio(ink, *bg)).collect();
            sheet.line(&format!("| `--m-{name}` | {} |", cells.join(" | ")));
        }
        sheet.line("");
    }

    // The categorical set against the surface it is drawn on.
    sheet.line("### Chart series on the chart surface");
    sheet.line("");
    sheet.line(&format!(
        "Floor `{NON_TEXT_MIN}`, with three deliberate exceptions. Gold, teal and \
         orange sit in a documented relief band between 2 and 3 — the gate holds them \
         *inside* that band from both sides, so drifting up out of it fails just as \
         loudly as drifting down. They are legible because a chart labels its series \
         directly, not because the fill clears a text floor."
    ));
    sheet.line("");
    sheet.line("| Slot | Light | Dark |");
    sheet.line("|---|---|---|");
    for slot in 1..=crate::viz::CATEGORICAL_LIGHT.len() {
        let cells: Vec<String> = [false, true]
            .iter()
            .map(|&dark| {
                let (set, surface) = if dark {
                    (
                        crate::viz::CATEGORICAL_DARK,
                        crate::chrome::INK_DARK.surface,
                    )
                } else {
                    (
                        crate::viz::CATEGORICAL_LIGHT,
                        crate::chrome::INK_LIGHT.surface,
                    )
                };
                let r = ratio(set[slot - 1], surface);
                // Gold, teal and orange, light mode only — the slots the gate
                // holds inside the relief band rather than above the floor.
                if !dark && [2, 3, 6].contains(&slot) {
                    format!("{r} (relief)")
                } else {
                    r
                }
            })
            .collect();
        sheet.line(&format!("| `--m-cat-{slot}` | {} |", cells.join(" | ")));
    }
    sheet.line("");
}

/// The tables the CSS artefact has no room for, read from their own modules.
fn beyond_css(sheet: &mut Sheet) {
    use crate::a11y::{KeyIntent, WidgetRole};
    use crate::control::BINDINGS;
    use crate::spacing;
    use crate::typography::{CALT_OFF, TNUM, ZERO};

    sheet.line("## Density bindings");
    sheet.line("");
    sheet.line(
        "A row rung is not a height on its own — it binds a control height, an icon \
         size, a text size and a padding pair that were chosen together. `tokens.css` \
         emits the rungs as separate custom properties because CSS has no way to say \
         they travel as a set; this is the set.",
    );
    sheet.line("");
    sheet.line("| Rung | Row | Control | Icon | Text | Pad x | Pad y |");
    sheet.line("|---|---|---|---|---|---|---|");
    for b in BINDINGS {
        let rung = if b.row <= spacing::ROW_DENSE {
            "dense"
        } else if b.row <= spacing::ROW_GRID {
            "grid"
        } else if b.row <= spacing::ROW_PREVIEW {
            "preview"
        } else {
            "comfortable"
        };
        sheet.line(&format!(
            "| {rung} | {} | {} | {} | {} | {} | {} |",
            super::px(b.row),
            super::px(b.control),
            super::px(b.icon),
            super::px(b.text),
            super::px(b.pad_x),
            super::px(b.pad_y),
        ));
    }
    sheet.line("");

    sheet.line("## OpenType features");
    sheet.line("");
    sheet.line(
        "Carried as `(tag, value)` pairs rather than a CSS string, because the desktop \
         needs them as font features and the web needs them as `font-feature-settings` \
         — one representation, two spellings downstream.",
    );
    sheet.line("");
    sheet.line("| Feature | Tag | Value | Applies to |");
    sheet.line("|---|---|---|---|");
    for (name, f, use_) in [
        ("TNUM", TNUM, "Tabular figures — every numeric column."),
        ("ZERO", ZERO, "Slashed zero, alongside tabular figures."),
        (
            "CALT_OFF",
            CALT_OFF,
            "Contextual alternates **off** — required on data surfaces, where the \
             mono face's ligatures would fuse characters that must stay countable.",
        ),
    ] {
        let tag = std::str::from_utf8(&f.tag).expect("an OpenType tag is four ASCII bytes");
        sheet.line(&format!("| `{name}` | `{tag}` | {} | {use_} |", f.value));
    }
    sheet.line("");

    sheet.line("## Keyboard and role contract");
    sheet.line("");
    sheet.line(
        "The half of a component spec colour tokens cannot carry: what a widget is, \
         whether it takes focus, and which key intents it must answer. Generated from \
         `a11y.rs`, where the enum, the roster and the name table come from one list \
         precisely so a new widget cannot escape the tables that iterate it.",
    );
    sheet.line("");
    sheet.line("| Widget | ARIA role | Focus | Tab stop | Key intents |");
    sheet.line("|---|---|---|---|---|");
    for role in WidgetRole::ALL {
        let intents: Vec<&str> = role
            .intents()
            .iter()
            .map(|i: &KeyIntent| i.name())
            .collect();
        sheet.line(&format!(
            "| `{:?}` | `{}` | {} | {} | {} |",
            role,
            role.aria(),
            format!("{:?}", role.focus()).to_lowercase(),
            if role.tab_stop() { "yes" } else { "no" },
            if intents.is_empty() {
                "—".to_string()
            } else {
                intents
                    .iter()
                    .map(|i| format!("`{i}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        ));
    }
    sheet.line("");
}

/// Emit the reference sheet. Deterministic by construction;
/// `tests/conformance.rs` pins the exact output.
pub fn tokens_md() -> String {
    let mut sheet = Sheet::new();

    sheet.line("<!-- Generated by meridian-design — do not edit by hand. -->");
    sheet.line("<!-- Regenerate: cargo run --example dump_md > ../reference/tokens.md -->");
    sheet.line("");
    sheet.line("# Token reference");
    sheet.line("");
    sheet.line(
        "Every value this design system defines, in both themes, generated from the \
         crate. Nothing here is typed by hand: the tables are rendered from the same \
         `emit::tokens_css()` the web consumes, so a value on this page and a value in \
         production cannot disagree. Colours are sRGB hex, the sRGB conversion of an \
         OKLCH design (ADR 0008); dimensions are logical pixels.",
    );
    sheet.line("");
    sheet.line(
        "GitHub renders no colour swatch for a hex code in a repository file, so this \
         page is the numbers. For the reasoning behind them, read `guidelines/`; for \
         the decisions, `decisions/`.",
    );
    sheet.line("");

    sheet.line("## Brand");
    sheet.line("");
    sheet.line(
        "Maritime, the signature. It appears on what the reader can act on or has \
         selected, and never tints static chrome — see `guidelines/identity.md`.",
    );
    sheet.line("");
    let rows = sheet.exact(&["--meridian-maritime"]);
    sheet.table(&rows);

    sheet.line("## Colour scales");
    sheet.line("");
    sheet.line(
        "Six 12-step ramps with Radix step semantics: 1–2 backgrounds, 3–5 component \
         fills, 6–8 borders, 9–10 solid fills, 11–12 text. Dark is derived from the \
         same construction with identical step meanings, never a naive inversion.",
    );
    sheet.line("");
    scales(&mut sheet);

    sheet.line("## Chart palettes");
    sheet.line("");
    sheet.line(
        "The categorical set is ordered so the first four validate all-pairs under \
         simulated colour-vision deficiency, not merely pairwise-adjacent — take them \
         in order and a four-series chart is safe by construction. The sequential and \
         diverging ramps and the status quartet are mode-invariant: a data ramp is the \
         same ramp in either theme.",
    );
    sheet.line("");
    let rows = sheet.prefix(&["--m-cat-", "--m-seq-", "--m-div-"]);
    let mut rows = rows;
    rows.extend(sheet.exact(&[
        "--m-null-ink",
        "--m-status-good",
        "--m-status-warning",
        "--m-status-serious",
        "--m-status-critical",
    ]));
    sheet.table(&rows);

    sheet.line("## Chart ink");
    sheet.line("");
    sheet.line(
        "What a chart is drawn *on* and *with*, as distinct from what it plots. The \
         renderer reads these directly; they are the one place chrome and canvas meet.",
    );
    sheet.line("");
    let rows = sheet.exact(&[
        "--m-surface",
        "--m-page",
        "--m-ink",
        "--m-ink-secondary",
        "--m-ink-muted",
        "--m-gridline",
        "--m-baseline",
        "--m-focus",
    ]);
    sheet.table(&rows);

    sheet.line("## Surfaces, borders and text");
    sheet.line("");
    sheet.line(
        "The semantic layer: what a colour is *for*, framework-neutral. Reach for these \
         before dropping to a raw scale — and if the thing being coloured has no \
         semantic name yet, the fix is usually to give it one.",
    );
    sheet.line("");
    let rows = sheet.prefix(&["--m-surface-", "--m-border-", "--m-text-"]);
    sheet.table(&rows);

    sheet.line("## Interaction roles");
    sheet.line("");
    sheet.line(
        "Six roles, three channels each, six states each: interaction state is a token \
         slot here rather than a cascade, so a consumer never computes a hover colour. \
         `focus` repeats `base` deliberately — the ring carries focus, not a fill \
         change.",
    );
    sheet.line("");
    roles(&mut sheet);

    sheet.line("## Component surfaces");
    sheet.line("");
    sheet.line(
        "Named parts that earn their own slots because two consumers both need them and \
         neither should invent them. Only the two planes above the working plane cast a \
         shadow, which is why there are two shadow tokens and not four.",
    );
    sheet.line("");
    let rows = sheet.prefix(&[
        "--m-rows-",
        "--m-tabs-",
        "--m-scrollbar-",
        "--m-editor-",
        "--m-progress-",
        "--m-slider-",
        "--m-sidebar-",
        "--m-title-bar-",
        "--m-status-bar-",
        "--m-group-box-",
        "--m-description-label-",
        "--m-accordion-",
        "--m-shadow-",
    ]);
    let mut rows = rows;
    rows.extend(sheet.exact(&[
        "--m-drop-target",
        "--m-drag-border",
        "--m-skeleton",
        "--m-tiles-bg",
        "--m-popover-bg",
        "--m-window-border",
    ]));
    rows.sort_unstable();
    sheet.table(&rows);

    sheet.line("## Box model");
    sheet.line("");
    sheet.line(
        "Stated once, in logical pixels, and mode-invariant throughout. The spacing \
         ladder is the only one whose index is genuinely its name: `--m-space-4` is the \
         fourth rung.",
    );
    sheet.line("");
    let rows = sheet.prefix(&[
        "--m-radius-",
        "--m-space-",
        "--m-row-",
        "--m-control-",
        "--m-icon-",
        "--m-focus-ring-",
        "--m-modal-",
    ]);
    let mut rows = rows;
    rows.extend(sheet.exact(&["--m-panel-padding", "--m-section-gap", "--m-pane-gap"]));
    rows.sort_unstable();
    sheet.table(&rows);

    sheet.line("## Type and motion");
    sheet.line("");
    sheet.line(
        "Two faces and two sizes: this is a dense data system, not a marketing site, \
         and the display face is deliberately absent from the app artefact. The motion \
         budget is two numbers because `guidelines/speed.md` allows two — a spatial \
         move and a state change — and nothing decorative.",
    );
    sheet.line("");
    let rows = sheet.prefix(&["--m-font-", "--m-motion-"]);
    sheet.table(&rows);

    // Anything nobody filed. Empty in a healthy tree, and gated that way.
    let leftover: Vec<usize> = (0..sheet.tokens.len())
        .filter(|&i| !sheet.taken[i])
        .collect();
    if !leftover.is_empty() {
        sheet.line("## Unclassified");
        sheet.line("");
        sheet.line(
            "These reached the CSS artefact and no section here claims them. That is a \
             gap in this emitter, not in the tokens.",
        );
        sheet.line("");
        sheet.table(&leftover);
    }

    measured(&mut sheet);
    beyond_css(&mut sheet);

    sheet.md
}
