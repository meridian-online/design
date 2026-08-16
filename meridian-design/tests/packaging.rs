//! What `cargo package` would ship out of `brand/`, pinned.
//!
//! The crate declares `license = "MIT"`. `brand/` does not fall under it —
//! `brand/LICENSE-BRAND.md` reserves the directory, echoed in the root
//! `LICENSE` — so `exclude` in `Cargo.toml` keeps the Affinity masters and the
//! raster wordmark out of the tarball a registry would receive. They stay in
//! git, editable, on GitHub; the repository and the published package are two
//! different artefacts and this is the line between them.
//!
//! The two lists below are the gate on that split. A file added under `brand/`
//! that `exclude` does not catch lands in an MIT-declared tarball, and reports
//! here instead — writing it into `PACKAGED` is then a decision someone made
//! rather than a side effect of dropping a file in a directory.

use std::path::PathBuf;
use std::process::Command;

/// The `brand/` files `cargo package` includes: the SVGs `src/brand.rs`
/// embeds, plus the directory's licence and README.
const PACKAGED: &[&str] = &[
    "brand/LICENSE-BRAND.md",
    "brand/README.md",
    // Brand motion (ADR 0012): one animation, both formats. The desktop takes
    // the Lottie; the SVG ships too, because `brand/` is how this repository
    // hands an asset to a consumer and a Rust consumer serving web assets
    // should not have to reach past the crate for it.
    "brand/motion/form.json",
    "brand/motion/form.svg",
    "brand/motion/form_dark.json",
    "brand/motion/form_dark.svg",
    // The second capped asset: the lockup forming, on the same terms.
    "brand/motion/lockup.json",
    "brand/motion/lockup.svg",
    "brand/motion/lockup_dark.json",
    "brand/motion/lockup_dark.svg",
    "brand/meridian_black.svg",
    "brand/meridian_black_pad.svg",
    // The wordmark as outlined curves, and the lockup that places it against
    // the mark. The raster wordmark below stays excluded: these supersede it
    // for anything that can take a vector, and nothing embeds the PNG.
    "brand/meridian_lockup.svg",
    "brand/meridian_wordmark.svg",
    "brand/meridian_white.svg",
    "brand/meridian_white_pad.svg",
    "brand/meridian_whiteob.svg",
    "brand/meridian_whiteob_pad.svg",
    "brand/sources/Meridian Prime Components.svg",
];

/// Retained in git, kept out of the tarball: the Affinity masters and the
/// raster wordmark. `git grep include_str -- '*.rs'` finds no reference to
/// them, which is what makes dropping them from the package free.
///
/// `Meridian Prime Components.svg` sits in the same directory and is packaged.
/// That is why `exclude` names `brand/sources/*.af` and not `brand/sources/` —
/// see `every_brand_file_the_crate_embeds_is_packaged`.
const EXCLUDED: &[&str] = &[
    "brand/meridian_wordmark.png",
    "brand/sources/Meridian Exports.af",
    "brand/sources/Meridian Prime Components.af",
    "brand/sources/Meridian Prime.af",
    "brand/sources/Meridian Wordmark.af",
];

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// `cargo package --list`, restricted to `brand/` and sorted.
///
/// Asking cargo rather than re-implementing its glob matching is the point:
/// an `exclude` pattern that silently matches nothing would pass a test that
/// only re-read the pattern.
///
/// `--allow-dirty` because the question is what the working tree would ship.
fn packaged_brand_files() -> Vec<String> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| env!("CARGO").to_owned());
    let out = Command::new(cargo)
        .current_dir(crate_root())
        .args(["package", "--list", "--allow-dirty"])
        .output()
        .expect("cargo package --list must be runnable");
    assert!(
        out.status.success(),
        "cargo package --list failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut files: Vec<String> = String::from_utf8(out.stdout)
        .expect("cargo package --list emits utf-8")
        .lines()
        .filter(|line| line.starts_with("brand/"))
        .map(str::to_owned)
        .collect();
    files.sort();
    files
}

/// `git ls-files brand/`, relative to the crate root and sorted.
///
/// The tracked inventory and the packaged inventory answer different
/// questions. Cargo also packages files that are untracked but not ignored, so
/// [`packaged_brand_files`] catches a file dropped into `brand/` and left
/// unstaged; this catches one that was committed and then swallowed by an
/// `exclude` pattern, which the packaged list cannot see by construction.
fn tracked_brand_files() -> Vec<String> {
    let out = Command::new("git")
        .current_dir(crate_root())
        .args(["ls-files", "brand"])
        .output()
        .expect("git ls-files must be runnable");
    assert!(
        out.status.success(),
        "git ls-files failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut files: Vec<String> = String::from_utf8(out.stdout)
        .expect("git ls-files emits utf-8")
        .lines()
        .map(str::to_owned)
        .collect();
    files.sort();
    files
}

/// Everything committed under `brand/` is one of the two lists above.
///
/// A brand asset arriving in the repository is then a question someone answers
/// — publish it or reserve it — instead of a default settled by whichever
/// glob happens to match its extension.
#[test]
fn the_tracked_brand_inventory_is_pinned() {
    let mut expected: Vec<String> = PACKAGED
        .iter()
        .chain(EXCLUDED.iter())
        .map(|f| (*f).to_owned())
        .collect();
    expected.sort();
    assert_eq!(
        tracked_brand_files(),
        expected,
        "brand/ gained or lost a tracked file; add it to PACKAGED or to EXCLUDED, \
         and check `exclude` in Cargo.toml agrees"
    );
}

/// The packaged brand set, pinned exactly.
#[test]
fn the_packaged_brand_assets_are_pinned() {
    let mut expected: Vec<String> = PACKAGED.iter().map(|f| (*f).to_owned()).collect();
    expected.sort();
    assert_eq!(
        packaged_brand_files(),
        expected,
        "the brand files cargo would publish have changed"
    );
}

/// The masters stay in the repository and out of the tarball. Both halves are
/// asserted because the arrangement is the difference between them: excluding
/// a file that has also been deleted proves nothing.
#[test]
fn the_brand_masters_are_retained_and_unpublished() {
    let packaged = packaged_brand_files();
    for master in EXCLUDED {
        assert!(
            crate_root().join(master).is_file(),
            "{master} has left the repository — it is meant to be kept, just not published"
        );
        assert!(
            !packaged.contains(&(*master).to_string()),
            "{master} would be published under the crate's MIT declaration"
        );
    }
}

/// Each `brand/` file `src/brand.rs` embeds has to survive `exclude`.
///
/// This is the check that catches the tempting wrong fix. `exclude =
/// ["brand/sources/"]` reads as "drop the editable masters", and also drops
/// `Meridian Prime Components.svg`, which `src/brand.rs` embeds — so the
/// tarball fails to compile at `cargo package`'s verify step, a long way from
/// the manifest edit that caused it.
///
/// The embedded paths are read out of `src/brand.rs` rather than restated.
#[test]
fn every_brand_file_the_crate_embeds_is_packaged() {
    const NEEDLE: &str = "include_str!(\"../brand/";
    let source = include_str!("../src/brand.rs");
    let embedded: Vec<String> = source
        .match_indices(NEEDLE)
        .map(|(at, _)| {
            let tail = &source[at + NEEDLE.len()..];
            let path = tail
                .split_once('"')
                .expect("an include_str! path is quoted")
                .0;
            format!("brand/{path}")
        })
        .collect();
    assert_eq!(
        embedded.len(),
        17,
        "src/brand.rs embeds a different number of brand files than this gate reads; \
         if a line was wrapped, the match above stopped seeing it"
    );

    let packaged = packaged_brand_files();
    for asset in &embedded {
        assert!(
            packaged.contains(asset),
            "{asset} is embedded by src/brand.rs but absent from the package — \
             the published tarball would not compile"
        );
    }
}
