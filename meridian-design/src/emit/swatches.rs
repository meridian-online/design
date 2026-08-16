//! The palette sheet — the colours, as colours.
//!
//! `emit::tokens_md` publishes every value the system defines and can show
//! none of them, because GitHub renders a hex code in a repository file as
//! text and nothing else. That is measured, not assumed, and it is the whole
//! reason this emitter exists: a design system whose palette can only be read
//! as 184 hex strings is documented but not shown.
//!
//! SVG is the format that closes the gap. GitHub serves one from a repository
//! path as `image/svg+xml` under a policy that permits presentation, so a
//! generated swatch grid renders in the README the same way the brand
//! animation does — one request, no runtime, nothing fetched.
//!
//! Like the reference sheet, it reads `tokens_css()` rather than the token
//! modules, so it restates no value: a chip's fill *is* the string the web
//! consumes, and its own furniture — background, labels, the outline round the
//! safe four — is drawn in semantic tokens rather than in ink picked to look
//! right. There is no `<style>` block and no `id`, deliberately. Shared text
//! properties ride on `<g>` inheritance instead, so the file survives being
//! inlined into a host document: no rule to leak, and no identifier to collide.

use crate::viz::{CATEGORICAL_LIGHT, DIVERGING_BLUE_ARM, DIVERGING_RED_ARM, SEQUENTIAL_MERIDIAN};

/// Canvas and grid, in user units. The width is a README's content column;
/// every other measure is derived from it, so one number moves the sheet.
const W: f64 = 880.0;
const PAD: f64 = 16.0;
const LABEL_W: f64 = 74.0;
const GAP: f64 = 4.0;
const CHIP_H: f64 = 34.0;
const ROW_H: f64 = CHIP_H + 18.0;
/// Baseline of a chip's value caption, below the chip. Far enough that the
/// outline round the safe four clears the text rather than resting on it.
const CAPTION_DY: f64 = CHIP_H + 11.0;
const SECTION_GAP: f64 = 20.0;

/// The width a row of chips is laid out across.
const TRACK: f64 = W - PAD - (PAD + LABEL_W);

/// At most two decimals, and no trailing `.0` — the artefact reads like
/// hand-written SVG and its diffs stay legible.
fn n(v: f64) -> String {
    let r = (v * 100.0).round() / 100.0;
    if r.fract() == 0.0 {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

/// Every `--*: value;` in one mode block of the CSS artefact.
fn declarations(block: &str) -> Vec<(String, String)> {
    block
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with("--"))
        .filter_map(|l| l.split_once(": "))
        .map(|(k, v)| (k.to_string(), v.trim_end_matches(';').to_string()))
        .collect()
}

/// The sheet under construction: this mode's token values, and the cursor.
struct Board {
    token: Vec<(String, String)>,
    svg: String,
    y: f64,
    /// Presentation for the value captions, section headings, quiet notes and
    /// the outline. Resolved once from semantic tokens, then written inline —
    /// a `<style>` block would need an `id` to scope it, and an inlined SVG's
    /// styles apply to the whole host document.
    caption: String,
    heading: String,
    quiet: String,
    outline: String,
    /// A hairline on every chip. Without it the low steps of a ramp and the
    /// diverging midpoint sit within a step or two of the sheet's own surface
    /// and read as gaps — which is the one thing a palette sheet must not do,
    /// since those steps are page and panel backgrounds and their whole job is
    /// to be that quiet.
    edge: String,
}

impl Board {
    fn new(dark: bool) -> Self {
        let css = super::tokens_css();
        let (light, dark_block) = css
            .split_once("\n.dark {")
            .expect("the artefact has two mode blocks");
        // The dark block redefines a subset and the rest cascades, so light is
        // the base in both modes and dark is laid over it.
        let mut token = declarations(light);
        if dark {
            for (name, value) in declarations(dark_block) {
                match token.iter_mut().find(|(n, _)| *n == name) {
                    Some(slot) => slot.1 = value,
                    None => token.push((name, value)),
                }
            }
        }
        let mut b = Board {
            token,
            svg: String::new(),
            y: PAD,
            caption: String::new(),
            heading: String::new(),
            quiet: String::new(),
            outline: String::new(),
            edge: String::new(),
        };
        let mono = crate::typography::MONO_FAMILY;
        b.caption = format!(
            "font-family=\"{mono}, ui-monospace, monospace\" font-size=\"7\" fill=\"{}\"",
            b.ink("--m-text-muted")
        );
        b.heading = format!(
            "font-size=\"11\" font-weight=\"600\" fill=\"{}\"",
            b.ink("--m-text-secondary")
        );
        b.quiet = format!("font-size=\"9\" fill=\"{}\"", b.ink("--m-text-muted"));
        b.outline = format!(
            "fill=\"none\" stroke=\"{}\" stroke-width=\"1\" stroke-dasharray=\"3 2\"",
            b.ink("--m-border-focus")
        );
        b.edge = format!(
            "stroke=\"{}\" stroke-width=\"0.5\"",
            b.ink("--m-border-subtle")
        );
        b
    }

    /// A token's value in this mode. Every colour on the sheet comes through
    /// here, which is what makes "this artefact cannot invent a colour" true
    /// by construction rather than by discipline.
    fn ink(&self, name: &str) -> String {
        self.token
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| panic!("{name} is not a token the CSS artefact emits"))
    }

    fn push(&mut self, s: &str) {
        self.svg.push_str(s);
        self.svg.push('\n');
    }

    /// A left-hand row label, vertically centred on the chips beside it.
    fn label(&mut self, text: &str) {
        let y = self.y + CHIP_H / 2.0 + 3.0;
        self.push(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{text}</text>",
            n(PAD + LABEL_W - 10.0),
            n(y)
        ));
    }

    /// A section heading, above the rows it introduces.
    fn heading(&mut self, text: &str) {
        let (x, y, style) = (n(PAD), n(self.y + 9.0), self.heading.clone());
        self.push(&format!("<text {style} x=\"{x}\" y=\"{y}\">{text}</text>"));
        self.y += 15.0;
    }

    /// A caption under a section, in the quiet ink.
    fn note(&mut self, text: &str) {
        let (x, y, style) = (n(PAD + LABEL_W), n(self.y + 7.0), self.quiet.clone());
        self.push(&format!("<text {style} x=\"{x}\" y=\"{y}\">{text}</text>"));
        self.y += 15.0;
    }

    /// One row of chips across the track, optionally captioned with its value.
    ///
    /// `boxed` outlines a leading run — the categorical set's first four,
    /// which validate all-pairs under simulated colour-vision deficiency and
    /// are therefore the four to reach for first.
    fn row(&mut self, values: &[String], captioned: bool, boxed: usize) {
        let count = values.len() as f64;
        let chip = (TRACK - GAP * (count - 1.0)) / count;
        let x0 = PAD + LABEL_W;
        let caption = self.caption.clone();
        let edge = self.edge.clone();
        for (i, value) in values.iter().enumerate() {
            let x = x0 + (chip + GAP) * i as f64;
            let cell = format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"2\" fill=\"{value}\" {edge}/>",
                n(x),
                n(self.y),
                n(chip),
                n(CHIP_H)
            );
            self.push(&cell);
            if captioned {
                let text = format!(
                    "<text {caption} x=\"{}\" y=\"{}\" text-anchor=\"middle\">{value}</text>",
                    n(x + chip / 2.0),
                    n(self.y + CAPTION_DY)
                );
                self.push(&text);
            }
        }
        if boxed > 0 {
            let width = chip * boxed as f64 + GAP * (boxed as f64 - 1.0) + 6.0;
            let (x, y, h, style) = (
                n(x0 - 3.0),
                n(self.y - 3.0),
                n(CHIP_H + 6.0),
                self.outline.clone(),
            );
            self.push(&format!(
                "<rect {style} x=\"{x}\" y=\"{y}\" width=\"{}\" height=\"{h}\" rx=\"4\"/>",
                n(width)
            ));
        }
        self.y += if captioned { ROW_H } else { CHIP_H + 5.0 };
    }
}

/// Emit the palette sheet for one mode. Deterministic by construction;
/// `tests/conformance.rs` pins the exact output of both.
pub fn palette_svg(dark: bool) -> String {
    let mut b = Board::new(dark);
    let surface = b.ink("--m-surface-app");
    let primary = b.ink("--m-text-primary");
    let sans = crate::typography::SANS_FAMILY;

    // Step numbers, aligned to the twelve chips beneath them.
    b.y += 6.0;
    let chip = (TRACK - GAP * 11.0) / 12.0;
    let quiet = b.quiet.clone();
    for step in 1..=12 {
        let x = PAD + LABEL_W + (chip + GAP) * (step - 1) as f64 + chip / 2.0;
        let row = format!(
            "<text {quiet} x=\"{}\" y=\"{}\" text-anchor=\"middle\">{step}</text>",
            n(x),
            n(b.y)
        );
        b.push(&row);
    }
    b.y += 8.0;

    for (hue, ramp) in super::ramps(dark) {
        let values: Vec<String> = (1..=ramp.len())
            .map(|s| b.ink(&format!("--m-{hue}-{s}")))
            .collect();
        b.label(hue);
        b.row(&values, true, 0);
    }
    b.y += SECTION_GAP;

    b.heading("Categorical — the Harbour set");
    let cats: Vec<String> = (1..=CATEGORICAL_LIGHT.len())
        .map(|i| b.ink(&format!("--m-cat-{i}")))
        .collect();
    b.row(&cats, true, 4);
    b.note(
        "The outlined four validate all-pairs under simulated colour-vision deficiency, not \
         merely pairwise-adjacent — take them in order and a four-series chart is safe by \
         construction.",
    );
    b.y += SECTION_GAP;

    b.heading("Sequential — Meridian blue");
    let seq: Vec<String> = (0..SEQUENTIAL_MERIDIAN.len())
        .map(|i| b.ink(&format!("--m-seq-{}", 100 + i * 50)))
        .collect();
    b.row(&seq, false, 0);
    b.note(
        "One hue, lightness strictly descending. Opt-in by name — the default ordinal scheme \
         stays viridis.",
    );
    b.y += SECTION_GAP;

    b.heading("Diverging — blue to brick red");
    let mut div: Vec<String> = (1..=DIVERGING_BLUE_ARM.len())
        .rev()
        .map(|i| b.ink(&format!("--m-div-blue-{i}")))
        .collect();
    div.push(b.ink("--m-div-mid"));
    div.extend((1..=DIVERGING_RED_ARM.len()).map(|i| b.ink(&format!("--m-div-red-{i}"))));
    b.row(&div, false, 0);
    b.note("Each arm lightens toward the midpoint, and the midpoint carries no chroma.");
    b.y += SECTION_GAP;

    b.heading("Status, and the null ink");
    let status: Vec<String> = ["good", "warning", "serious", "critical"]
        .iter()
        .map(|s| b.ink(&format!("--m-status-{s}")))
        .chain(std::iter::once(b.ink("--m-null-ink")))
        .collect();
    b.row(&status, true, 0);
    b.note(
        "Good, warning, serious, critical — then the null ink, which carries no chroma so it can \
         never be read as a series.",
    );

    let h = b.y + PAD - 15.0;
    let mode = if dark { "dark" } else { "light" };
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" \
         height=\"{h}\" role=\"img\" aria-label=\"The Meridian palette, {mode} theme\">\
         <title>The Meridian palette, {mode} theme</title>\
         <rect width=\"{w}\" height=\"{h}\" fill=\"{surface}\"/>\
         <g font-family=\"{sans}, ui-sans-serif, system-ui, sans-serif\" font-size=\"11\" \
         fill=\"{primary}\">\n{body}</g></svg>\n",
        w = n(W),
        h = n(h),
        body = b.svg,
    )
}
