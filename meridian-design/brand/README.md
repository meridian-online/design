# brand

The Meridian mark and wordmark, and the Affinity sources they come from.

**Licence: not MIT.** See [`LICENSE-BRAND.md`](LICENSE-BRAND.md). The code in
this repository is MIT; this directory is reserved, the same way `../fonts/`
carries its own OFL terms.

## What is here

| File | What it is |
|---|---|
| `meridian_black.svg` | The prime mark, dark-on-transparent. Light contexts. |
| `meridian_white.svg` | The prime mark in white. Dark contexts. |
| `meridian_whiteob.svg` | White with an outline — high contrast, favicons, unpredictable backgrounds. |
| `meridian_*_pad.svg` | The same three with the standard clear-space padding baked in. Use these when you cannot control the surrounding margin. |
| `meridian_wordmark.png` | The wordmark, 1000×410 raster. **There is no wordmark SVG yet** — see below. |
| `sources/*.af` | Affinity Designer originals. The editable truth. |
| `sources/Meridian Prime Components.svg` | The mark's *construction* file — see below. It is not an artefact to ship. |

## They did not arrive consistent

The six SVGs came in byte-identical to the copies in the web repo's `brand/`
directory — verified, not assumed — but **they did not agree with each other**.
`meridian_white.svg` and `meridian_white_pad.svg` carried `c-0,-35.056` where the
other four carried `c0,-35.056`. Numerically identical, so every renderer drew
the same picture and nothing ever complained; but a negative zero does not appear
by choice, so the set came from more than one export session.

They were normalised to the majority form on the way in, which is why these files
are no longer byte-equal to web's copies. `all_six_files_carry_one_geometry` in
`../src/brand.rs` is what stops the set fragmenting again.

Web's inlined path matches the *white* variant's form, incidentally — while the
comment above it names `meridian_black.svg` as its source. It was copied from a
file other than the one it claims.

## The construction file is the interesting one

`sources/Meridian Prime Components.svg` is the mark before the boolean
operations: **one** tooth path, placed three times, mirrored once by
`matrix(-1,0,0,-1,800,800)`, clipped by `circle r=300` at `(400,400)`. Six teeth
from one primitive and two transforms.

That is why the mark can be animated by derivation rather than by hand. The
rounded band-ends are the projection of a meridian on a sphere: a meridian at
longitude θ projects to an ellipse of horizontal semi-axis `R·cos θ`. A genuine
globe turn therefore holds the latitude bands fixed and animates each cap's
horizontal radius as `R·cos(ωt)` — it loops exactly, and it reads as a sphere
rather than as a logo on a turntable.

Keep this file. Re-deriving it from the flattened mark is real work.

## Two gaps, stated rather than discovered later

**There is no wordmark SVG.** The wordmark exists as a 1000×410 PNG and as
`sources/Meridian Wordmark.af`, nothing else. Any vector use — and the
wordmark-forming animation in particular — needs an outlined SVG exported from
Affinity first. Outlined, not live text: the desktop Lottie renderer has no text
layers at all, so live type would silently render as nothing.

**Web still inlines the mark as source.** `components/datasets/dataset-explorer.tsx`
carries the 600×600 path as a React component rather than referencing a tracked
file, and that copy has already drifted from this one by a byte (`c0,-35.056`
against `c-0,-35.056` — same geometry, different export). Nothing pins them
together yet. `scripts/check-brand.mjs` on the web side is what closes it.

## Rules

The mark's usage rules are in [`../../guidelines/identity.md`](../../guidelines/identity.md).
Motion — what may be animated, where, and the cap on how many — is
[ADR 0012](../../decisions/0012-brand-motion-on-brand-surfaces.md).
