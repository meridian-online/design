# motion

The generator for Meridian's brand motion. Same shape as [`validation/`](../validation): a reproducible pipeline that runs by hand, whose result is committed rather than computed at build time (ADR 0007).

**What it emits lives in
[`meridian-design/brand/motion/`](../meridian-design/brand/motion), not here.**
Two assets, each as four files — Lottie for the desktop and animated SVG for the
web, in both themes — which is [ADR
0012](../decisions/0012-brand-motion-on-brand-surfaces.md)'s home for brand
motion, inside the crate so the desktop can take the bytes as a cargo
dependency. Nothing downstream builds this directory.

| Asset | Generator | What it is | Where it plays |
|---|---|---|---|
| `form{,_dark}.{json,svg}` | `build_form.py` | The mark forming. 7.2 s. | The >100 ms work cue, plus brand surfaces. |
| `lockup{,_dark}.{json,svg}` | `build_lockup.py` | Mark and wordmark forming together. 6.2 s. | Brand surfaces only — front door, install, OG, README. |

Two of ADR 0012's three capped assets. The third is not a third animation: the work indicator is a cut of `form`, and the bare wordmark forming is a cut of `lockup` — its second beat, frames 33-73.

## `form` is the mark-motion asset

**Decided 2026-08-16, landed 2026-08-16.** `form`, scheme `seq3`, one-frame lag: an arc laps the disc, three strokes flow through it, the mark assembles row by row, holds, and disperses. 7.2 s at 25 fps, looping. It is ADR 0012's first capped asset — the one the retired globe turn was going to be — and it is what plays as the >100 ms work cue `guidelines/speed.md` requires.

Three colours: the theme's own `--m-ink`, then two steps of the Meridian blue-240 sequential ramp — `--m-seq-500` and `--m-seq-300` on light, `350` and `550` on dark. Nothing here is a picked colour; see *Colour comes from the crate* below. `blue-violet` was the runner-up and `SCHEMES` still knows how to build it; everything else tried has been removed and the reasoning is kept in this file.

### Two formats, and what stops them drifting apart

The split is ADR 0012's and it is deliberately asymmetric. The desktop takes Lottie through velato; **the web takes animated SVG**, because the site ships no motion library and `lottie-web` is ~75 KB gzipped for two brand animations.

`svg_form.py` does not re-choreograph anything. It reads the built Lottie — the paths, the trim schedule, the colours, the stroke width, the loop length — so a change to `SEAM_OVERLAP` or to a wake's lag cannot land in one artefact and miss the other. That is the same rule `CLAUDE.md` puts on the crate's emitters, moved one level up: the value at risk here is the *schedule*, and a second choreography would restate it.

**One emitter serves both assets.** `svg_form.emit(doc, verbatim)` takes the tracked artwork the document is allowed to draw — the mark's teeth for `form`, the mark and the eight letters for `lockup` — and a filled shape whose geometry is in neither raises. That parameter is the whole of the generalisation, and it exists to keep the property the emitter is for: every filled path is written as the tracked file's own `d` text, so the animated artefact carries the artwork to the character rather than to four decimal places. A path it cannot name is one it would have to re-serialise, and a re-serialised artefact is no longer tied to the file it came from.

Three gates hold the result, and none of them replaces another:

| Gate | What it catches |
|---|---|
| `scripts/check-motion.sh` | The artefacts no longer match the generator — a schedule change committed without regenerating. Runs in CI, needs only Python 3. |
| `meridian-design/tests/motion.rs` | What a byte comparison cannot say: each animation draws its own tracked artwork, every colour is a token value, both formats loop for the same time and share the artwork's canvas, dark is light recoloured rather than redrawn, the SVG stops for reduced motion and fetches nothing. |
| `meridian-design/src/brand.rs` | That the lockup is *made of* the tracked components — its mark uniformly scaled, its wordmark purely translated — which is what lets `build_lockup.py` read the composite instead of composing one. Plus the winding both renderers depend on. |
| `meridian-design/tests/packaging.rs` | The eight artefacts ship, under `brand/`'s terms rather than the crate's MIT grant. |

### What the web artefact leans on

One request, no runtime, no script. Reference it with `<img>`. Inlining works — its ids and CSS rules are scoped to its own root so it cannot restyle the page around it — but two inlined copies of the *same* file collide on ids.

**Verified in all three engines** — Chromium 149, Firefox 153 and Safari 26.1 — by driving the artefact to two animation times and reading back the *computed* style, so a feature that parses and never moves fails the check rather than looking plausible. All three agree to seven significant figures on the wipe transform, the dash and the offset. The features worth knowing about if a consumer ever reports something: CSS animations inside SVG-as-image, animated `stroke-dasharray`/`stroke-dashoffset` with `pathLength="1"`, and a CSS `transform` on a `<rect>` inside a `<clipPath>`.

Firefox shares one animation clock across every `<img>` pointing at the same URL, so several copies on a page stay in step there and drift apart in Chromium. Nothing depends on either behaviour.

**The desktop was checked too.** velato 0.11 imports both Lottie files and draws every beat, all nine matted row precomps included, with alpha mattes arriving as `Compose::SrcIn`. That was measured without a GPU: velato renders through a `RenderSink` trait, so a sink that records instead of rasterising says exactly what the desktop would be asked to draw, with the layer names attached. What is *not* covered is vello painting that compositing on the GPU — velato pulls vello without its renderer — and the desktop shell exercises that for every scene it draws anyway. ADR 0012 has the detail.

Under `prefers-reduced-motion: reduce` every animation stops and the file settles to the assembled mark. **A surface using it as the work cue owes the reader a textual cue as well** — a still mark says the brand, not that anything is happening.

## Licence

The generator is MIT with the rest of the repository's code. **What it emits is not.** `brand/motion/` embeds the prime mark, and [`brand/LICENSE-BRAND.md`](../meridian-design/brand/LICENSE-BRAND.md) reserves the mark "and every artefact generated from" it. The root [`LICENSE`](../LICENSE) states the carve-out.

## Running it

```
python3 build_form.py             # writes form's four artefacts into brand/motion/
python3 build_form.py --check     # are they still what this generator produces?
python3 build_form.py --explore   # plus every scheme and lag, into ./output/
python3 build_lockup.py           # the same four for the lockup
python3 build_lockup.py --check
python3 -m http.server 8803       # from the REPOSITORY ROOT
```

Then `localhost:8803/motion/preview/form.html`. Plain Python 3, no dependencies.

The published copies are at <https://meridian-online.github.io/design/> and need no server at all — `.github/workflows/pages.yml` builds them from `main`, with `--explore` run first so the comparison pages have their scratch. Serve locally when you are changing a generator; read them published when you are checking one.

Serve from the repository root, not from `motion/` or `preview/`: the pages reach `../../meridian-design/brand/motion/*`, and a server will not look above its own root, so a server started lower leaves every canvas blank. `file://` fails for the same reason — the artefacts are fetched, and that is blocked.

`output/` is untracked. The runner-up and the slower cut belong to a decision that has been made; leaving them committed would put files in the tree that look like assets, that nothing pins, and that a consumer could take. Run `--explore` to get them back.

| Page | What it is |
|---|---|
| `preview/form.html` | The concept, three renderings, one at 120px. |
| `preview/lockup.html` | The lockup asset: filmstrip, both themes, and the reduced-motion still. |
| `preview/svg.html` | The SVG artefact against the Lottie it came from, frame by frame. |
| `preview/schemes.html` | Wake schemes and lags side by side, light and dark, kept in step. Needs `--explore`. |
| `preview/orbit-check.html` | One player per frame, stopped — what the renderer actually draws. |
| `preview/ink.html` | Opaque pixels per frame, flagging empty and near-empty runs. |

`preview/lottie.min.js` is vendored so the pages work offline.

## The mark comes from `brand/`, not from here

`build_form.py` parses `meridian-design/brand/meridian_black.svg` at run time and converts its six subpaths to Lottie form. It does **not** keep a copy of the geometry, for two reasons.

A second copy is the exact defect
[`brand/README.md`](../meridian-design/brand/README.md) documents: the web app
inlined the mark's path as source, and its copy drifted from the tracked file by
a byte before anyone noticed. A generator with a private mark drifts the same
way and nothing says so.

It is also where the licence boundary sits. The trademark stays in `brand/` under the terms that govern it, and this file borrows it at build time.

The mark uses five path commands — `M`, `m`, `l`, `c`, `Z` — so `mark_paths()` is a complete parser for *that path*, not for SVG in general. It is checked against a known-good decomposition: six subpaths, `[6, 7, 7, 7, 6, 7]` vertices.

## `form`

Three beats.

1. **orbit** — an arc laps the disc outline, running leftward over the top, frames 0–40
2. **S curves** — one per row, flowing right to left, staggered, seen 37–80
3. **fills** — the mark assembles row by row, same direction, then holds and disperses, frames 75–180

7.20 s at 25 fps. 49 KB as Lottie, 16 KB as SVG — the SVG is smaller because every wake ghost is one `@keyframes` block reused at a different delay, which is the wake's own definition rather than a compression trick. Every movement carries a coloured wake; see below. The S is the figure the mark repeats three times: two quarter circles of radius 100, centres ±100 either side of a pinch, meeting with colinear tangents. The strokes run on the mark's own edges, so each fill lands on the line that predicted it.

### The arc's length is its own speed

Head and tail are the **same eased ramp offset by four frames**, so what gets drawn is the gap between two copies of one curve — and the size of that gap is the curve's derivative. The arc grows out of a point, opens to about 100° at the midpoint, and contracts back into the point it left from. Nothing fades: the geometry arrives from nothing and leaves to nothing on its own.

The same mechanism draws the S's, which is why they are shaped as well as moved — each enters short, opens to about 68% of its path, and closes as it leaves.

### The seam is not the right limb

The orbit starts and ends at `ORBIT_THETA`, the point where the first S becomes visible, which sits a row higher — (575, 200) against the S's (583, 200), the difference being only the orbit's inset radius. `LEAD` is derived so the arc dies *when* the S appears as well as *where*.

Abutting them exactly leaves a one-frame hole, because the frame the head crosses the disc is the frame it has zero length: a boundary, not a mark. `SEAM_OVERLAP` covers it, and the handover measures as a continuous V bottoming at about 24px of stroke.

### The wake, and its colour

Every movement carries one. A ghost is not a copy that has been moved — it is the **same path with the same trim, run three frames late**, so the wake is the stroke's own past and nothing about the geometry or the schedule changes. Its length is therefore the stroke's speed, like everything else here.

The mark's rows are built the other way round, because a row is revealed by a matte rather than drawn by a trim: a ghost that simply starts late has uncovered *less* than the ink in front of it and is completely hidden. So each ghost is on screen slightly **longer at both ends** — earlier in, later out — and colour runs ahead of the ink as a row arrives and is left behind as it leaves. They also take a shorter lag than the strokes (`MARK_TRAIL_LAG`), because a fill carries colour as *area* where a stroke carries it as a line: at the stroke's three frames each row came out 41% coloured, which read as colour-blocking rather than as a trail.

`SCHEMES` sets what the wake is made of. The leader is always the theme's own ink — black on light, paper on dark — so a scheme only describes what happens behind it, and every one of them is black/white plus a single family:

| Scheme | Wake | Note |
|---|---|---|
| `seq3` | Sequential blue-240, 500 / 300 | **Chosen.** The gallery's Meridian ramp, two steps. |
| `blue-violet` | Sequential 500, then categorical violet | Runner-up. Blue starts the wake; violet is the thing you catch at the end. |

**Ruled out, so they are not re-proposed blind.** The *categorical set* as a wake — 1/3/8 and every other combination tried: its eight slots are tuned to roughly equal weight so that no data series outranks another, and that is exactly the property that stops them stacking into a fade, so they read as equal bands rather than as a receding wake. Where a categorical slot survives it is spent **once**, at the tail, with sequential blue doing the fading in front of it — which is what `blue-violet` is. The *Maritime scale*, the *diverging blue arm* and the `m-blue` scale all work but are softer than `m-seq`. *Grey* reads as motion blur rather than colour, which may yet be the right register for the in-app indicator. *Amber* muddies on dark, where the scale runs through brown.

Four wake steps, then three, then two were compared. Three keeps enough gradient to dissolve rather than stop, and at 120px is indistinguishable from four.

The 12-step scales flip with the theme, so one list of steps serves both: light runs pale to deep, dark runs deep to pale, with 9 the accent in each. **The chart ramps do not.** `m-seq` and `m-div-blue` are defined once, in `:root` only, because a data ramp is the same ramp whichever theme surrounds it — so the dark wake walks them the other way round to recede into its background rather than out of it. `m-seq` also respects its own ordinal guidance: on light nothing paler than step 250, on dark nothing deeper than 600.

### How fleeting

`TRAIL_LAG` is the second axis, and it decides whether the colour is a wake or a **flash**. The visible wake is speed × lag, so a small lag means colour appears only where a stroke is quickest and is gone at the limbs — you catch it rather than watch it, which is what the `5uvsTa4DXp` reference does with four copies one frame apart at 25 fps.

At 3 the wake was continuously visible and read as a coloured stroke. At 1 each ghost sits about a ninth of a dash length behind, and the colour is a tip that opens across the middle of a travel and closes at both ends. `mark_lag()` derives the mark's from it, so one knob keeps the two in proportion.

There is a measurable cost. The longer wake had been covering the orbit → S seam; at a one-frame lag frames 36–40 return to being flagged near-empty by `ink.html`, which is where they sat before the wake existed. Nothing is empty, and `blue-quick` at two frames still covers them.

### Everything eases symmetrically

`cubic-bezier(0.6, 0, 0.4, 1)` — slow at both limbs, quickest across the middle. Only the mark's own wipe still runs a front-loaded curve. On a closed lap a front-loaded ramp puts the widest point a third of the way round and leaves the rest stalled, because the arc's length is that ramp's speed and the speed has gone.

The cost is that two symmetric-eased things meet at their slowest moments, which is what the seam overlap exists to cover.

## `lockup`

**Landed 2026-08-16.** ADR 0012's second capped asset: one edge crosses the brand, the mark's three rows arrive behind it, and the edge carries on and writes the word. 6.24 s at 25 fps, looping, in the same three colours `form` uses — the scheme is read out of `build_form.SCHEMES` rather than restated.

### One instrument, four times

Every beat is the same thing: a rectangular matte whose edge sweeps, with a wake of ghost copies that uncover slightly more than the ink in front of them. That is `form`'s mark-arrival mechanism exactly, applied to the mark's three rows and then to the word. Using one instrument is the point rather than an economy — a lockup is a mark and a word that belong together, and drawing them with two different gestures would say the opposite.

No orbit and no S-strokes. Both are about the *mark's* construction and have nothing to say about a word; borrowing them would have made this a longer `form` with a wordmark appended, and pushed the loop past ten seconds for a front door.

### It runs left to right, where `form` runs right to left

Not a preference. A word is read left to right, and the lockup is the mark *then* the word; an edge crossing the other way would write the word backwards and hand the eye to the mark last. The direction follows what the artwork is, so the two assets differ here and should.

### The sweep is scoped to each beat's own ink

The one real defect in the first build, and worth recording because it looked fine in a filmstrip. `form` sweeps its canvas, which is correct there because its mark fills it. Here the mark occupies 58 units of a 336-wide canvas, so a canvas-wide edge cleared it at **21% of the sweep** and the row then sat fully drawn while the wipe finished somewhere off to the right.

It also flattened the wake, which is the part a still cannot show: a wake's width is the edge's speed times its lag, so an edge crossing five times the distance in the same time is five times too fast, and the colour arrived as a block covering a whole row instead of a band at its leading edge. `matte()` now takes the beat's own extent, read off the paths that beat draws.

### The wake runs early, so the loop starts late

A ghost opens *before* its leader — that is how a matte-revealed wake works at all, since colour only shows where a ghost has uncovered something the ink in front of it has not. With the first row at frame 0 the outermost ghost wanted frame -2, which the composition clips; the row entered with a truncated wake and the two lost frames reappeared as a stray blue sliver at the far end of the loop, wrapped around. `LEAD_IN` is that depth, and nothing is now asked to happen before the composition starts.

### Reading the composite, not composing one

`lockup.py` parses `meridian_lockup.svg`, which already carries the mark and the wordmark in one coordinate space. The alternative — reading the two components and applying a scale and an offset here — would put the lockup's placement in Python as a third copy of something two tracked SVGs already state, and it would cost the property `svg_form.py` depends on: geometry transformed in Python has no `d` text to be verbatim about.

That trade only holds while the composite really is made of the tracked files, so it is not assumed. `brand.rs` registers the lockup's mark against `meridian_black.svg` as a uniform scale — least squares over every coordinate, absolute residual, which catches a non-uniform scale as sharply as a redraw — and its wordmark against `meridian_wordmark.svg` as a pure translation, where the path text after each opening move must match character for character.

### Two export shapes that bite a reader

**The two I's are `<rect>`, not `<path>`.** Affinity optimises axis-aligned rectangles that way, and a reader that looks only at `d` attributes drops them and animates MER DAN without raising. They convert to four vertices exactly.

**The mark is last in document order despite being leftmost**, so reading order maps to letter order for the wordmark but not for the file. It is found by `id="prime_black"` — which is also why any reader here has to match `' d="'` *with the leading space*, since matching `d="` alone finds the `d` in `id="` and reads `prime_black` as path data.

### The counters are holes because of how they wind

velato parses a fill rule into its schema and then hardcodes `Fill::NonZero` in `runtime/vello.rs` without reading it back, and the SVG declares `fill-rule:nonzero` too. Under that rule a counter is a hole only if it winds against its outline. Every outline here winds clockwise and every counter anticlockwise, so R, D and A come out right in both — and `brand.rs` asserts it, because it is a property both renderers depend on and neither declares. A flipped winding would fail in both at once rather than in one, which is why nothing else would catch it.

## The schedule is derived, not typed

`LEAD`, `S_VANISH`, `MARK_IN`, `LOOP` and `ORBIT_THETA` are computed from the geometry — see the derivation block under `circle_path`. This is not tidiness. Hand-computed equivalents went stale the moment an easing curve changed, because the entry and exit fractions of `s_path` are properties of the **path** while the frames they land on are properties of the **ramp**, and nothing connected the two.

**The rows are not alike**, which is easy to miss. Row 0's upper band runs along `y = 0` and row 2's lower band along `y = H`, and both are *tangent* to the disc — they touch at a single point and are otherwise outside it. So row 0 appears first but vanishes early at the tangent, while row 2 barely exists until it has risen. First-in and last-out are different rows: row 0 enters at 12.0% of its path, row 2 leaves at 88.0% of its.

## Measure it, do not look at it

`preview/ink.html` exists because filmstrips lie. An earlier cut of this loop had **eleven fully empty frames** between the last S and the mark's arrival and looked fine in every still. Open it after any timing change; a run of red outside the loop's last few frames is a defect.

`preview/svg.html` is the same discipline applied to the second format: the SVG paused through the Web Animations API, beside the Lottie seeked to the same frame. The pairs should be indistinguishable apart from the dash ends, which sit up to 1.6 units apart on a 600-unit canvas — 13% of a stroke width — because a dash *length* is the difference of two eased ramps and CSS has no timing function that says that, so it is sampled once per frame. The build prints the figure rather than this file asserting it.

Renderer notes worth having:

- **python-lottie cannot render these.** Its SVG exporter ignores precomps and track mattes, which is all of the S's and the mark. It draws the orbit, because that is a plain top-level shape layer, and silently draws nothing else — which reads exactly like a broken animation. Use lottie-web.
- **A round cap on a zero-length trim is not nothing.** lottie-web draws it as a dot of the full stroke width, so a layer whose trim closes on its last frame wants `op` on that frame rather than after it. SVG does the same, and for the same reason — `stroke-dasharray: 0 10` with a round cap *is* how a dotted line is drawn — so each stroke carries a visibility animation bounding it to the frames its dash has length.
- **A `@keyframes` block with no `100%` stop does not hold its last value.** CSS synthesises the missing keyframe from the element's *underlying* value and interpolates toward it. Left open, every dash ran back toward `none` — an undashed path is a solid stroke — and every wipe drifted back toward untranslated, which covers the whole canvas, so all three rows of the mark stood on screen throughout the orbit. Both artefacts had the same schedule and only one of them was showing it. `svg_form.py` closes both ends of every block.
