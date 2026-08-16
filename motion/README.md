# motion

An offline generator for brand motion, and its output. Same shape as
[`validation/`](../validation): a reproducible pipeline that runs by hand, whose
result is committed rather than computed at build time (ADR 0007). Nothing
downstream builds this, and no consumer takes it yet.

## Decided: `seq3-flash` is the loading animation

**2026-08-16 — `form`, scheme `seq3`, one-frame lag, is the loading animation
for web and desktop.** Files: `output/form-seq3-flash.json` and
`form-seq3-flash-dark.json`. `blue-violet` was the runner-up and is kept beside
it; everything else that was tried has been removed from `SCHEMES` and the
reasoning is in this file.

Three colours: the theme's own ink, then two steps of the Meridian blue-240
sequential ramp — `--m-seq-500` and `--m-seq-300` on light, `350` and `550` on
dark. Nothing here is a picked colour; see *Colour comes from the crate* below.

**This is a decision, not yet the sanctioned asset**, and the difference is
work that has not been done. [ADR
0012](../decisions/0012-brand-motion-on-brand-surfaces.md) requires brand motion
to live in `meridian-design/brand/`, pinned by a conformance test, with an
**animated-SVG emitter for web** beside the Lottie one for desktop — because web
takes no Lottie runtime. None of those three exist. Until they do, this is a
chosen candidate in an exploration directory, and a consumer that takes it takes
an unpinned file.

Outstanding, in the order it has to happen:

1. An **animated-SVG emitter**, so web has an artefact it can actually use.
2. A **conformance test** pinning both emitted artefacts byte-for-byte.
3. The move into `brand/`, and the **ADR 0012 amendment** naming the form — the
   amendment that section explicitly leaves open, and which also has to say
   which of the three capped assets this occupies. It is the honest-work cue
   `guidelines/speed.md` already requires, which is the one asset ADR 0012
   admits *inside* the apps rather than only on brand surfaces.

## Licence

The generator is MIT with the rest of the repository's code.

**`output/` is not.** Those files embed the Meridian prime mark, which is a
trademark reserved outside the MIT grant — see
[`meridian-design/brand/LICENSE-BRAND.md`](../meridian-design/brand/LICENSE-BRAND.md),
which reserves the mark "and every artefact generated from" it, and the
carve-out in the root [`LICENSE`](../LICENSE).

## Running it

```
python3 build_form.py                    # writes output/*.json
python3 -m http.server 8802              # from THIS directory
```

Then `localhost:8802/preview/form.html`. Plain Python 3, no dependencies.

Serve from `motion/`, not from `preview/`: the pages fetch `../output/*.json`,
and a server rooted at `preview/` will not look above its own root, so every
animation 404s and the canvases sit blank. `file://` fails for the same reason —
the JSON is fetched, and that is blocked.

| Page | What it is |
|---|---|
| `preview/form.html` | The concept, three renderings, one at 120px. |
| `preview/schemes.html` | Wake schemes and lags side by side, light and dark, kept in step. |
| `preview/orbit-check.html` | One player per frame, stopped — what the renderer actually draws. |
| `preview/ink.html` | Opaque pixels per frame, flagging empty and near-empty runs. |

`preview/lottie.min.js` is vendored so the pages work offline.

## The mark comes from `brand/`, not from here

`build_form.py` parses `meridian-design/brand/meridian_black.svg` at run time
and converts its six subpaths to Lottie form. It does **not** keep a copy of the
geometry, for two reasons.

A second copy is the exact defect
[`brand/README.md`](../meridian-design/brand/README.md) documents: the web app
inlined the mark's path as source, and its copy drifted from the tracked file by
a byte before anyone noticed. A generator with a private mark drifts the same
way and nothing says so.

It is also where the licence boundary sits. The trademark stays in `brand/`
under the terms that govern it, and this file borrows it at build time.

The mark uses five path commands — `M`, `m`, `l`, `c`, `Z` — so `mark_paths()`
is a complete parser for *that path*, not for SVG in general. It is checked
against a known-good decomposition: six subpaths, `[6, 7, 7, 7, 6, 7]` vertices.

## `form`

Three beats.

1. **orbit** — an arc laps the disc outline, running leftward over the top,
   frames 0–40
2. **S curves** — one per row, flowing right to left, staggered, seen 37–80
3. **fills** — the mark assembles row by row, same direction, then holds and
   disperses, frames 75–181

7.20 s at 25 fps, 65 KB. Every movement carries a coloured wake; see below. The S is the figure the mark repeats three times: two
quarter circles of radius 100, centres ±100 either side of a pinch, meeting with
colinear tangents. The strokes run on the mark's own edges, so each fill lands
on the line that predicted it.

### The arc's length is its own speed

Head and tail are the **same eased ramp offset by four frames**, so what gets
drawn is the gap between two copies of one curve — and the size of that gap is
the curve's derivative. The arc grows out of a point, opens to about 100° at the
midpoint, and contracts back into the point it left from. Nothing fades: the
geometry arrives from nothing and leaves to nothing on its own.

The same mechanism draws the S's, which is why they are shaped as well as moved
— each enters short, opens to about 68% of its path, and closes as it leaves.

### The seam is not the right limb

The orbit starts and ends at `ORBIT_THETA`, the point where the first S becomes
visible, which sits a row higher — (575, 200) against the S's (583, 200), the
difference being only the orbit's inset radius. `LEAD` is derived so the arc
dies *when* the S appears as well as *where*.

Abutting them exactly leaves a one-frame hole, because the frame the head
crosses the disc is the frame it has zero length: a boundary, not a mark.
`SEAM_OVERLAP` covers it, and the handover measures as a continuous V bottoming
at about 24px of stroke.

### The wake, and its colour

Every movement carries one. A ghost is not a copy that has been moved — it is
the **same path with the same trim, run three frames late**, so the wake is the
stroke's own past and nothing about the geometry or the schedule changes. Its
length is therefore the stroke's speed, like everything else here.

The mark's rows are built the other way round, because a row is revealed by a
matte rather than drawn by a trim: a ghost that simply starts late has
uncovered *less* than the ink in front of it and is completely hidden. So each
ghost is on screen slightly **longer at both ends** — earlier in, later out —
and colour runs ahead of the ink as a row arrives and is left behind as it
leaves. They also take a shorter lag than the strokes (`MARK_TRAIL_LAG`),
because a fill carries colour as *area* where a stroke carries it as a line: at
the stroke's three frames each row came out 41% coloured, which read as
colour-blocking rather than as a trail.

`SCHEMES` sets what the wake is made of. The leader is always the theme's own
ink — black on light, paper on dark — so a scheme only describes what happens
behind it, and every one of them is black/white plus a single family:

| Scheme | Wake | Note |
|---|---|---|
| `seq3` | Sequential blue-240, 500 / 300 | **Chosen.** The gallery's Meridian ramp, two steps. |
| `blue-violet` | Sequential 500, then categorical violet | Runner-up. Blue starts the wake; violet is the thing you catch at the end. |

**Ruled out, so they are not re-proposed blind.** The *categorical set* as a
wake — 1/3/8 and every other combination tried: its eight slots are tuned to
roughly equal weight so that no data series outranks another, and that is
exactly the property that stops them stacking into a fade, so they read as equal
bands rather than as a receding wake. Where a categorical slot survives it is
spent **once**, at the tail, with sequential blue doing the fading in front of
it — which is what `blue-violet` is. The *Maritime scale*, the *diverging blue
arm* and the `m-blue` scale all work but are softer than `m-seq`. *Grey* reads
as motion blur rather than colour, which may yet be the right register for the
in-app indicator. *Amber* muddies on dark, where the scale runs through brown.

Four wake steps, then three, then two were compared. Three keeps enough gradient
to dissolve rather than stop, and at 120px is indistinguishable from four.

The 12-step scales flip with the theme, so one list of steps serves both: light
runs pale to deep, dark runs deep to pale, with 9 the accent in each. **The
chart ramps do not.** `m-seq` and `m-div-blue` are defined once, in `:root`
only, because a data ramp is the same ramp whichever theme surrounds it — so
the dark wake walks them the other way round to recede into its background
rather than out of it. `m-seq` also respects its own ordinal guidance: on light
nothing paler than step 250, on dark nothing deeper than 600.

### How fleeting

`TRAIL_LAG` is the second axis, and it decides whether the colour is a wake or
a **flash**. The visible wake is speed × lag, so a small lag means colour
appears only where a stroke is quickest and is gone at the limbs — you catch it
rather than watch it, which is what the `5uvsTa4DXp` reference does with four
copies one frame apart at 25 fps.

At 3 the wake was continuously visible and read as a coloured stroke. At 1 each
ghost sits about a ninth of a dash length behind, and the colour is a tip that
opens across the middle of a travel and closes at both ends. `mark_lag()`
derives the mark's from it, so one knob keeps the two in proportion.

There is a measurable cost. The longer wake had been covering the orbit → S
seam; at a one-frame lag frames 36–40 return to being flagged near-empty by
`ink.html`, which is where they sat before the wake existed. Nothing is empty,
and `blue-quick` at two frames still covers them.

### Everything eases symmetrically

`cubic-bezier(0.6, 0, 0.4, 1)` — slow at both limbs, quickest across the middle.
Only the mark's own wipe still runs a front-loaded curve. On a closed lap a
front-loaded ramp puts the widest point a third of the way round and leaves the
rest stalled, because the arc's length is that ramp's speed and the speed has
gone.

The cost is that two symmetric-eased things meet at their slowest moments, which
is what the seam overlap exists to cover.

## The schedule is derived, not typed

`LEAD`, `S_VANISH`, `MARK_IN`, `LOOP` and `ORBIT_THETA` are computed from the
geometry — see the derivation block under `circle_path`. This is not tidiness.
Hand-computed equivalents went stale the moment an easing curve changed, because
the entry and exit fractions of `s_path` are properties of the **path** while
the frames they land on are properties of the **ramp**, and nothing connected
the two.

**The rows are not alike**, which is easy to miss. Row 0's upper band runs along
`y = 0` and row 2's lower band along `y = H`, and both are *tangent* to the disc
— they touch at a single point and are otherwise outside it. So row 0 appears
first but vanishes early at the tangent, while row 2 barely exists until it has
risen. First-in and last-out are different rows: row 0 enters at 12.0% of its
path, row 2 leaves at 88.0% of its.

## Measure it, do not look at it

`preview/ink.html` exists because filmstrips lie. An earlier cut of this loop
had **eleven fully empty frames** between the last S and the mark's arrival and
looked fine in every still. Open it after any timing change; a run of red
outside the loop's last few frames is a defect.

Two renderer notes worth having:

- **python-lottie cannot render these.** Its SVG exporter ignores precomps and
  track mattes, which is all of the S's and the mark. It draws the orbit,
  because that is a plain top-level shape layer, and silently draws nothing
  else — which reads exactly like a broken animation. Use lottie-web.
- **A round cap on a zero-length trim is not nothing.** lottie-web draws it as a
  dot of the full stroke width, so a layer whose trim closes on its last frame
  wants `op` on that frame rather than after it.
