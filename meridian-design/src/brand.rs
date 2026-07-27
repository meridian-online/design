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
//! [`MARK_WHITEOB`] where the background is unpredictable (favicons, photos).
//! The `_PAD` forms bake in the standard clear space — reach for them when you
//! cannot control the surrounding margin, and only then. Motion rules are ADR
//! 0012; usage rules are `guidelines/identity.md`.

/// The prime mark, dark on transparent. Light contexts.
pub const MARK_BLACK: &str = include_str!("../brand/meridian_black.svg");
/// The prime mark in white. Dark contexts.
pub const MARK_WHITE: &str = include_str!("../brand/meridian_white.svg");
/// White with an outline — high contrast, favicons, unpredictable backgrounds.
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
