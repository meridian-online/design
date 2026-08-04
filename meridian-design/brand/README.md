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
| `meridian_whiteob.svg` | The white mark on an opaque black plate that fills the canvas — a filled rectangle behind the path, not an outline around it. Favicons, and surfaces whose background cannot be controlled. |
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

That is why the mark can be animated by derivation rather than by hand.

Decomposing the flattened path confirms the same structure and adds the detail
that matters: each tooth is a bar running from a vertical axis out to the limb,
closed at its inner end by a **cap of constant radius**. The bars alternate
side, top to bottom.

The radius is constant at every latitude. A meridian projected onto a sphere
would give a cap widest at the equator and pinched at the poles, so **the mark
is a stylised globe rather than a projection of one** — and an animation that
scaled the existing curve like an ellipse would be animating a shape that is not
there. The turn instead sweeps one parameter per bar, the position of its cap:
`x_cap(t) = 300 + side·100 + A·sin(ωt + φ)`, clipped to the disc. At `A = 0` it
is the shipped mark exactly, which is how the generator is checked.

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
