//! The Meridian mark, as bytes consumers can reference instead of redraw.
//!
//! **These assets are not MIT.** The crate around them is; `brand/` is reserved
//! (`brand/LICENSE-BRAND.md`, echoed in the root `LICENSE`). They live inside
//! the crate for one reason: the desktop app takes `meridian-design` as a cargo
//! dependency and needs the bytes at compile time, which a directory at the
//! repository root could not supply.
//!
//! This module exists because the alternative already happened. With no mark in
//! the system, the web app stopped referencing the file and inlined the *path* —
//! and that copy diverged from the tracked SVG by a byte before anyone noticed.
//! A consumer that reads [`MARK_BLACK`] cannot drift; a consumer that pastes 953
//! characters of path data into a component will, and nothing will say so.
//!
//! Which variant: [`MARK_BLACK`] on light, [`MARK_WHITE`] on dark,
//! [`MARK_WHITEOB`] where the background cannot be controlled — it brings its
//! own opaque plate rather than adapting to what is behind it.
//! The `_PAD` forms bake in the standard clear space — reach for them when you
//! cannot control the surrounding margin, and only then. Motion rules are ADR
//! 0012; usage rules are `guidelines/identity.md`.

/// The prime mark, dark on transparent. Light contexts.
pub const MARK_BLACK: &str = include_str!("../brand/meridian_black.svg");
/// The prime mark in white. Dark contexts.
pub const MARK_WHITE: &str = include_str!("../brand/meridian_white.svg");
/// The white mark on an opaque black plate that fills the canvas. Favicons,
/// and surfaces whose background cannot be controlled.
///
/// The plate is a filled rectangle behind the path, not an outline around it,
/// so this variant brings a black tile with it rather than adapting to the
/// surface. Where that surface should show through, use [`MARK_WHITE`].
pub const MARK_WHITEOB: &str = include_str!("../brand/meridian_whiteob.svg");

/// [`MARK_BLACK`] with the standard clear space baked in.
pub const MARK_BLACK_PAD: &str = include_str!("../brand/meridian_black_pad.svg");
/// [`MARK_WHITE`] with the standard clear space baked in.
pub const MARK_WHITE_PAD: &str = include_str!("../brand/meridian_white_pad.svg");
/// [`MARK_WHITEOB`] with the standard clear space baked in.
pub const MARK_WHITEOB_PAD: &str = include_str!("../brand/meridian_whiteob_pad.svg");

/// The mark's construction file: one tooth path, placed three times, mirrored
/// once, clipped by a circle. Not an artefact to ship — the parametric source
/// the motion work derives from, kept here so it cannot be lost.
pub const MARK_COMPONENTS: &str = include_str!("../brand/sources/Meridian Prime Components.svg");

/// The wordmark, MERIDIAN, as outlined curves on a `viewBox="0 0 237 32"`.
///
/// **There is no text here and there must never be.** The letterforms are
/// curves, so this file needs no font to render and cannot reflow — which is
/// what makes it a wordmark rather than a string set in a typeface. Set in
/// Anybody; `brand/README.md` records the size and the measured fingerprint, and
/// that record is the only thing standing between this file and an unreproducible
/// asset, because the curves have absorbed everything the character panel knew.
///
/// Eight drawable elements in left-to-right reading order, one per letter, each
/// carrying its own counters as subpaths. Note that the two I's are `<rect>`
/// rather than `<path>` — Affinity optimises axis-aligned rectangles that way,
/// and a reader that looks only at `d` attributes drops them silently.
pub const WORDMARK: &str = include_str!("../brand/meridian_wordmark.svg");

/// The horizontal lockup: [`MARK_BLACK`] and [`WORDMARK`] in one
/// `viewBox="0 0 336 84"`, at the placement the brand actually uses.
///
/// This is the authority on how the two sit together, and it is worth stating
/// what that placement is, because it is not the obvious one: the wordmark is
/// centred on its **cap-height midline**, not its baseline, and that midline
/// lands on the mark's centre. The mark is on the canvas centreline, radius
/// 29.28 at (40.32, 42.0); the wordmark's ink starts 18.24 further right, which
/// is 0.623 mark radii of gap.
///
/// Both components are the tracked files unchanged rather than redrawn: the
/// wordmark here is [`WORDMARK`] translated by exactly (87.6, 26.239) with no
/// scale, and the mark is [`MARK_BLACK`] at a uniform 0.0976 — verified vertex
/// by vertex, so a distorted or restyled copy cannot hide in here.
pub const LOCKUP: &str = include_str!("../brand/meridian_lockup.svg");

/// `form`: an arc laps the disc, three strokes flow through, and the mark
/// assembles row by row. 7.2 s at 25 fps, looping. ADR 0012's mark-formation
/// asset, and the >100 ms work cue `guidelines/speed.md` requires.
///
/// **Two formats, one animation.** The desktop takes the Lottie through velato;
/// the web takes the SVG, because the site ships no motion library and a Lottie
/// runtime is ~75 KB gzipped for two brand animations. Both are generated from
/// one source by `motion/build_form.py` — the SVG is a translation of the very
/// bytes in the Lottie, not a second choreography — and `tests/motion.rs` pins
/// what they have to agree on.
///
/// Which variant: the plain forms on light, the `_DARK` forms on dark. There is
/// no adaptive single file; the ink and the wake are different token values in
/// each theme, and the chart ramp they walk runs the other way round.
pub const MOTION_FORM_LOTTIE: &str = include_str!("../brand/motion/form.json");
/// [`MOTION_FORM_LOTTIE`] in the dark theme's ink.
pub const MOTION_FORM_LOTTIE_DARK: &str = include_str!("../brand/motion/form_dark.json");

/// [`MOTION_FORM_LOTTIE`] as animated SVG — the form the web takes.
///
/// Self-contained and script-free: serve it and reference it with `<img>`. It
/// honours `prefers-reduced-motion`, where it settles to the assembled mark; a
/// surface using it as the work cue owes the reader a textual cue as well,
/// because a still mark says the brand and not that anything is happening.
///
/// Inlining it into a page works — its ids and CSS rules are scoped to its own
/// root so it cannot restyle the document around it — but two inlined copies of
/// the *same* file collide on ids, which `<img>` avoids.
pub const MOTION_FORM_SVG: &str = include_str!("../brand/motion/form.svg");
/// [`MOTION_FORM_SVG`] in the dark theme's ink.
pub const MOTION_FORM_SVG_DARK: &str = include_str!("../brand/motion/form_dark.svg");

/// `lockup`: one edge crosses the brand — the mark's rows arrive behind it,
/// then it carries on and writes the word. 6.2 s at 25 fps, looping. ADR 0012's
/// second capped asset, at the placement [`LOCKUP`] holds.
///
/// **This is the front-door animation, not the work cue.** [`MOTION_FORM_SVG`]
/// is what a surface reaches for while something is happening; this one says
/// who the software is, so it belongs on marketing, install, an OG image, a
/// desktop front door or a README — the brand surfaces ADR 0012 lists, and
/// nowhere in application chrome.
///
/// It travels left to right, where `form` travels right to left, because a word
/// is read left to right and the lockup is the mark *then* the word. The two
/// assets differ here on purpose.
pub const MOTION_LOCKUP_LOTTIE: &str = include_str!("../brand/motion/lockup.json");
/// [`MOTION_LOCKUP_LOTTIE`] in the dark theme's ink.
pub const MOTION_LOCKUP_LOTTIE_DARK: &str = include_str!("../brand/motion/lockup_dark.json");

/// [`MOTION_LOCKUP_LOTTIE`] as animated SVG — the form the web takes.
///
/// Self-contained and script-free, on the same terms as [`MOTION_FORM_SVG`]:
/// serve it and reference it with `<img>`. Under `prefers-reduced-motion` it
/// settles to the assembled lockup, which is the whole statement rather than a
/// fragment of one — so unlike the work cue, this still frame needs no textual
/// cue beside it. It is saying the brand, which is all it was ever saying.
pub const MOTION_LOCKUP_SVG: &str = include_str!("../brand/motion/lockup.svg");
/// [`MOTION_LOCKUP_SVG`] in the dark theme's ink.
pub const MOTION_LOCKUP_SVG_DARK: &str = include_str!("../brand/motion/lockup_dark.svg");

/// The mark's design canvas: `viewBox="0 0 600 600"`, centred, radius 300.
pub const MARK_VIEWBOX: f32 = 600.0;

/// The mark's outer radius on its own canvas, in the same units as
/// [`MARK_VIEWBOX`]. The three latitude bands sit at y = 100, 300 and 500.
pub const MARK_RADIUS: f32 = 300.0;

/// The clear space the `_PAD` variants bake in, per side, in [`MARK_VIEWBOX`]
/// units — a 640 canvas around a 600 mark.
///
/// Measured from the shipped assets rather than asserted at them: the padded
/// files are the unpadded path translated by exactly this much, and nothing
/// else. Use this when laying out the unpadded mark yourself, so a hand-built
/// lockup and the padded file agree.
pub const MARK_CLEAR_SPACE: f32 = 20.0;

/// The mark's single `d` attribute, sliced out of [`MARK_BLACK`].
///
/// For consumers that need the geometry rather than a whole SVG document — an
/// icon font, a `Path2D`, a Lottie shape layer. Returns `None` only if the SVG
/// has been replaced by something without a `d` attribute, which the tests in
/// this module are here to catch first.
///
/// Plain `str` slicing, no parser: this crate is dependency-free by contract
/// (ADR 0003), and the input is a file in this repository rather than arbitrary
/// user SVG.
#[must_use]
pub fn mark_path() -> Option<&'static str> {
    let rest = MARK_BLACK.split_once(" d=\"")?.1;
    rest.split_once('"').map(|(path, _)| path)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FNV-1a, so the pin below is a short number instead of 953 pasted
    /// characters. Dependency-free by contract, and a digest is all a
    /// tripwire needs — the file it guards is two directories away.
    fn digest(s: &str) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in s.as_bytes() {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
        h
    }

    #[test]
    fn the_mark_path_is_extractable() {
        let path = mark_path().expect("meridian_black.svg must carry a d attribute");
        assert!(path.starts_with("M300,100"), "unexpected start: {path:.32}");
        assert!(path.ends_with('Z'), "a mark subpath must close");
    }

    /// The mark is six teeth. If a re-export changes that count, the geometry
    /// changed — which may be intended, but it is never incidental.
    #[test]
    fn the_mark_is_six_closed_subpaths() {
        let path = mark_path().unwrap();
        assert_eq!(
            path.matches('Z').count(),
            6,
            "the prime mark is six closed subpaths; re-check the export"
        );
    }

    /// The tripwire. A new export from the design tool that moves a control
    /// point lands here rather than in production.
    ///
    /// Intentional geometry changes update this number **and** re-check every
    /// consumer: the mark is referenced by file, but anything that has already
    /// rasterised it (favicons, OG images, the wordmark lockup) is a separate
    /// artefact that does not update itself.
    #[test]
    fn the_mark_geometry_is_pinned() {
        let path = mark_path().unwrap();
        assert_eq!(path.len(), 953, "mark path length changed — re-export?");
        assert_eq!(
            digest(path),
            0x8996_ac27_9057_0204,
            "the mark's geometry changed. If that was deliberate, update this \
             pin in the same commit and re-derive every rasterised artefact."
        );
    }

    /// All six shipped files are one drawing. They were not, when they arrived:
    /// the two white variants carried `c-0,-35.056` where the other four carried
    /// `c0,-35.056` — numerically identical, so it rendered the same and nothing
    /// caught it, but proof the set came from more than one export session. They
    /// were normalised to the majority form on the way in. This test is what
    /// stops the set fragmenting again.
    #[test]
    fn all_six_files_carry_one_geometry() {
        let expected = mark_path().unwrap().split_once('l').unwrap().1;
        for (name, svg) in [
            ("black", MARK_BLACK),
            ("white", MARK_WHITE),
            ("whiteob", MARK_WHITEOB),
            ("black_pad", MARK_BLACK_PAD),
            ("white_pad", MARK_WHITE_PAD),
            ("whiteob_pad", MARK_WHITEOB_PAD),
        ] {
            let d = svg
                .split_once(" d=\"")
                .unwrap()
                .1
                .split_once('"')
                .unwrap()
                .0;
            assert_eq!(
                d.split_once('l').unwrap().1,
                expected,
                "{name} has drifted from the canonical mark geometry"
            );
        }
    }

    /// The three unpadded variants are the same drawing in different ink. A
    /// variant that stops agreeing on the canvas has been redrawn rather than
    /// recoloured.
    #[test]
    fn every_variant_shares_the_marks_canvas() {
        for (name, svg) in [
            ("black", MARK_BLACK),
            ("white", MARK_WHITE),
            ("whiteob", MARK_WHITEOB),
        ] {
            assert!(
                svg.contains("viewBox=\"0 0 600 600\""),
                "{name} is not on the mark's 600x600 canvas"
            );
        }
    }

    /// The padded forms add margin; they do not redraw anything.
    ///
    /// Every command after the opening `moveto` is relative, so "the same
    /// drawing, translated" is exactly: identical path text from the first
    /// `l` onwards, and an opening point offset by [`MARK_CLEAR_SPACE`] in both
    /// axes on a canvas larger by twice that. Asserting the tail rather than
    /// the whole string is what lets this catch a *redraw* while ignoring the
    /// translation it is supposed to have.
    #[test]
    fn the_padded_forms_are_the_unpadded_mark_translated() {
        let plain = mark_path().unwrap();
        let (plain_move, plain_tail) = plain.split_once('l').expect("mark starts M…l");
        let pad = MARK_VIEWBOX + 2.0 * MARK_CLEAR_SPACE;

        for (name, svg) in [
            ("black_pad", MARK_BLACK_PAD),
            ("white_pad", MARK_WHITE_PAD),
            ("whiteob_pad", MARK_WHITEOB_PAD),
        ] {
            assert!(
                svg.contains(&format!("viewBox=\"0 0 {pad} {pad}\"")),
                "{name} is not on the padded {pad}x{pad} canvas"
            );
            let d = svg
                .split_once(" d=\"")
                .unwrap()
                .1
                .split_once('"')
                .unwrap()
                .0;
            let (moved, tail) = d.split_once('l').expect("padded mark starts M…l");
            assert_eq!(
                tail, plain_tail,
                "{name} has been redrawn, not merely translated"
            );

            let point = |s: &str| -> (f32, f32) {
                let (x, y) = s.trim_start_matches('M').split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            };
            let (x0, y0) = point(plain_move);
            let (x1, y1) = point(moved);
            assert_eq!(
                (x1 - x0, y1 - y0),
                (MARK_CLEAR_SPACE, MARK_CLEAR_SPACE),
                "{name}'s offset disagrees with MARK_CLEAR_SPACE"
            );
        }
    }

    /// The `whiteob` variants are plated, not outlined.
    ///
    /// This is the distinction [`MARK_WHITEOB`]'s documentation promises a
    /// consumer, and the two render differently on the case that variant is
    /// reached for: a filled `<rect>` behind the path covers whatever is
    /// under it, where an outline would let the surface through. Both halves
    /// are legible in the file, so both are asserted here rather than left to
    /// the description.
    #[test]
    fn the_whiteob_variants_are_plated_rather_than_outlined() {
        for (name, svg) in [("whiteob", MARK_WHITEOB), ("whiteob_pad", MARK_WHITEOB_PAD)] {
            assert!(
                svg.contains("<rect id=\"black\""),
                "{name} has lost the opaque plate its documentation describes"
            );
            assert!(
                !svg.contains("stroke:"),
                "{name} now carries stroke paint; if it has gained an outline, \
                 the docs on MARK_WHITEOB describe a plate and need re-checking"
            );
        }
        assert!(
            !MARK_WHITE.contains("<rect"),
            "meridian_white.svg has gained a plate, which is what whiteob is for"
        );
    }

    /// Every ` d="…"` in a document, in order.
    fn path_data(svg: &str) -> Vec<&str> {
        svg.match_indices(" d=\"")
            .map(|(at, _)| {
                svg[at + 4..]
                    .split_once('"')
                    .expect("a d attribute is quoted")
                    .0
            })
            .collect()
    }

    /// Every number in a path, in order, as written.
    ///
    /// A scan rather than a parser: after the opening move every command in
    /// these exports is relative, so the numbers ARE the geometry and their
    /// order is the drawing order.
    fn numbers(d: &str) -> Vec<f64> {
        let mut out = Vec::new();
        let bytes = d.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let c = bytes[i] as char;
            if c.is_ascii_digit() || c == '.' || (c == '-' && i + 1 < bytes.len()) {
                let start = i;
                if c == '-' {
                    i += 1;
                }
                while i < bytes.len() && ((bytes[i] as char).is_ascii_digit() || bytes[i] == b'.') {
                    i += 1;
                }
                if i > start + usize::from(c == '-') {
                    if let Ok(n) = d[start..i].parse() {
                        out.push(n);
                    }
                }
            } else {
                i += 1;
            }
        }
        out
    }

    /// The lockup's mark is [`MARK_BLACK`] scaled, not a second drawing of it.
    ///
    /// This is the claim `brand/motion/lockup.*` rests on. The lockup is the
    /// authority on placement, and the motion generator reads its geometry
    /// straight out of it rather than composing the two components in Python —
    /// which is only sound while the composite really is made of the tracked
    /// files. A redrawn or squashed mark inside the lockup would animate
    /// perfectly and be wrong, and nothing else would notice.
    ///
    /// Scale rather than the tail-comparison the padded forms get: the mark is
    /// placed at about a tenth of its own canvas here, so the text cannot
    /// match. Every number is compared instead, in order — same count, and one
    /// ratio across all of them. That catches a non-uniform scale, which is
    /// what `guidelines/identity.md` actually forbids, as sharply as it catches
    /// a redraw.
    #[test]
    fn the_lockups_mark_is_the_tracked_mark_scaled() {
        // From the first drawing command, not from the start: the opening move
        // is the one ABSOLUTE pair in the path, so it carries the translation
        // as well as the scale and would report a ratio of its own. Everything
        // after it is relative, and therefore pure scale. The same seam the
        // padded forms are compared across, for the same reason.
        let tail = |d: &str| {
            d.split_once(['l', 'c'])
                .expect("a drawn subpath")
                .1
                .to_owned()
        };
        let plain = numbers(&tail(mark_path().expect("the mark has path data")));
        let placed = numbers(&tail(
            LOCKUP
                .split_once("id=\"prime_black\" d=\"")
                .expect("the lockup names its mark")
                .1
                .split_once('"')
                .unwrap()
                .0,
        ));
        assert_eq!(
            plain.len(),
            placed.len(),
            "the lockup's mark has a different number of coordinates from the \
             tracked mark, so it has been redrawn rather than placed"
        );

        // Least squares over every pair, so no single coordinate decides the
        // scale and the estimate is not hostage to whichever one comes first.
        let dot: f64 = placed.iter().zip(&plain).map(|(&q, &p)| q * p).sum();
        let norm: f64 = plain.iter().map(|&p| p * p).sum();
        assert!(norm > 0.0, "the mark has no non-zero coordinates");
        let scale = dot / norm;
        assert!(
            (0.0..1.0).contains(&scale),
            "the lockup's mark is not smaller than the tracked mark"
        );

        // ABSOLUTE residual, not a ratio. The lockup is written to three
        // decimals, so a coordinate the mark states as 0.002 arrives as 0.0002
        // and is rounded away entirely — a ratio test reads that as a scale of
        // zero and cries distortion over a rounding step. What actually has to
        // hold is that every coordinate lands within half a step of where the
        // scale puts it.
        let step = 1.0e-3;
        for (i, (&q, &p)) in placed.iter().zip(&plain).enumerate() {
            assert!(
                (q - p * scale).abs() <= step,
                "coordinate {i}: the lockup has {q} where a uniform scale of \
                 {scale} puts {} — the mark is distorted, not merely placed",
                p * scale
            );
        }
    }

    /// The lockup's wordmark is [`WORDMARK`] translated, and nothing else.
    ///
    /// Stronger than the mark's check and by the same trick the padded forms
    /// use: the wordmark is placed at scale 1, so after each subpath's opening
    /// move — every later command being relative — the text must be identical
    /// character for character. A single retouched control point fails here.
    #[test]
    fn the_lockups_wordmark_is_the_tracked_wordmark_translated() {
        let letters: Vec<&str> = path_data(WORDMARK);
        let placed: Vec<&str> = path_data(LOCKUP)
            .into_iter()
            .filter(|d| !LOCKUP.contains(&format!("id=\"prime_black\" d=\"{d}\"")))
            .collect();
        assert_eq!(
            letters.len(),
            placed.len(),
            "the lockup carries a different number of letter paths than the \
             tracked wordmark"
        );
        assert!(!letters.is_empty(), "the wordmark has no path data");

        let tail = |d: &str| {
            let (_, rest) = d.split_once(['l', 'c']).expect("a drawn subpath");
            rest.to_owned()
        };
        for (i, (a, b)) in letters.iter().zip(&placed).enumerate() {
            assert_eq!(
                tail(a),
                tail(b),
                "letter path {i} differs between the wordmark and the lockup — \
                 the lockup has been redrawn, not merely placed"
            );
        }

        // The two I's are <rect>, which carry no path text at all. Their size
        // is the whole of their geometry, so it is what has to match.
        let sizes = |svg: &str| -> Vec<String> {
            svg.match_indices("<rect")
                .map(|(at, _)| {
                    let el = &svg[at..at + svg[at..].find("/>").expect("a closed rect")];
                    let of = |k: &str| {
                        el.split_once(&format!("{k}=\""))
                            .expect("a rect has width and height")
                            .1
                            .split_once('"')
                            .unwrap()
                            .0
                            .to_owned()
                    };
                    format!("{}x{}", of("width"), of("height"))
                })
                .collect()
        };
        assert_eq!(
            sizes(WORDMARK),
            sizes(LOCKUP),
            "the lockup's rectangular letters are a different size from the \
             wordmark's, so it is not a pure translation"
        );
    }

    /// The construction file is what makes the mark animatable by derivation
    /// rather than by hand. Losing it costs real work to rebuild, so its
    /// defining structure is asserted rather than assumed.
    #[test]
    fn the_construction_file_still_carries_one_tooth_and_the_mirror() {
        assert!(
            MARK_COMPONENTS.contains("matrix(-1,-0,0,-1,800,800)"),
            "the components file has lost its mirror transform"
        );
        assert_eq!(
            MARK_COMPONENTS.matches(" d=\"").count(),
            6,
            "the components file is six placements of one tooth"
        );
    }
}
