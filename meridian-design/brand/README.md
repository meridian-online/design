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
| `meridian_wordmark.svg` | The wordmark, MERIDIAN, as outlined curves on a 237×32 canvas. Eight elements in reading order, one per letter. |
| `meridian_lockup.svg` | Mark and wordmark together on a 336×84 canvas, at the placement the brand uses — see below. |
| `meridian_wordmark.png` | The wordmark, 1000×410 raster. Superseded by the SVG for anything that can take a vector; kept because raster contexts exist. |
| `motion/form.{json,svg}`, `form_dark.{json,svg}` | `form` — the mark forming. ADR 0012's first capped asset and the >100 ms work cue. Lottie for the desktop, animated SVG for the web. |
| `motion/lockup.{json,svg}`, `lockup_dark.{json,svg}` | `lockup` — mark and wordmark forming together, at the placement below. The second capped asset, and a **front-door** animation rather than a work cue. |
| `sources/*.af` | Affinity Designer originals. The editable truth. |
| `sources/Meridian Prime Components.svg` | The mark's *construction* file — see below. It is not an artefact to ship. |

## `motion/` is generated, and both formats come from one source

An arc laps the disc, three strokes flow through it, and the mark assembles row
by row. 7.2 s at 25 fps, looping. ADR 0012's first capped asset, and what plays
as the >100 ms work cue.

**Two formats because two consumers.** The desktop takes Lottie through velato;
the web takes animated SVG, because the site ships no motion library and a
Lottie runtime is ~75 KB gzipped for two brand animations. The web file is
self-contained and script-free — reference it with `<img>` — and honours
`prefers-reduced-motion`, where it settles to the assembled mark.

**Do not hand-edit these four files.** They are emitted by
[`motion/build_form.py`](../../motion) at the repository root, which reads
`meridian_black.svg` for the geometry and `tests/snapshots/tokens.css` for every
colour, so neither is retyped anywhere. `scripts/check-motion.sh` fails CI if
what is committed here is no longer what that generator produces, and
`tests/motion.rs` fails if the two formats have drifted apart or if a colour has
stopped being a token value.

Which variant: `form.*` on light, `form_dark.*` on dark. There is no adaptive
single file — the ink and the wake are different token values in each theme, and
the chart ramp they walk runs the other way round.

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

## The wordmark, and how the lockup places it

The wordmark is **outlined curves, never live text**, and that is not a
preference: the desktop Lottie renderer has no text layers at all, so live type
would render as nothing and say nothing about it. Outlining also means the file
carries no font dependency — and that it has absorbed everything the character
panel knew, which is why the settings are written down here.

Set in **[Anybody](https://fonts.google.com/specimen/Anybody) at 46 pt**,
tracking 0‰, no horizontal or vertical scaling, no baseline shift. Anybody is a
variable family with weight *and* width axes, so the family name alone does not
identify these outlines. Measured off the curves, as a fingerprint anyone can
re-derive without the source file:

| | Wordmark | Anybody SemiBold (600), standard width |
|---|---|---|
| cap height / em | 0.6763 | 0.6750 |
| stem / cap height | 0.2186 | 0.2096 |
| ink width / cap height | 7.5945 | 6.8215 |

The cap height confirms 46 pt in point units. The other two do not match the
600 the web app renders live text in: the same string is **11% wider** at
barely 4% heavier stems, which is the signature of the width axis rather than
the weight axis. Treat the wordmark and any live-text setting of "MERIDIAN" as
different things until the exact instance is recorded here.

**The lockup is the authority on placement, and the rule is not the obvious
one.** The wordmark is centred on its **cap-height midline**, not its baseline,
and that midline sits on the mark's centre. On the 336×84 canvas the mark has
radius 29.28 at (40.32, 42.0) — dead on the canvas centreline — and the
wordmark's ink begins at x 87.836, a gap of 18.236 or 0.623 mark radii.

Both components are the tracked files unchanged, and that is checkable rather
than asserted: the lockup's wordmark is `meridian_wordmark.svg` translated by
exactly (87.6, 26.239) at scale 1 — one unique offset across all 104 vertices —
and its mark is `meridian_black.svg` at a uniform 0.0976, matching to 0.001
units with 1.5 ppm of anisotropy. Nothing in the lockup was redrawn or
stretched.

One export detail that bites readers: **the two I's are `<rect>`, not
`<path>`**, in both files. Affinity optimises axis-aligned rectangles that way,
so a reader that looks only at `d` attributes drops them and animates MER DAN
without complaining. In the lockup the mark is also *last* in document order
despite being leftmost — key off `id="prime_black"` rather than position, and
match `' d="'` with the leading space or `id="` answers instead.

## Rules

The mark's usage rules are in [`../../guidelines/identity.md`](../../guidelines/identity.md).
Motion — what may be animated, where, and the cap on how many — is
[ADR 0012](../../decisions/0012-brand-motion-on-brand-surfaces.md).
