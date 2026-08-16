# motion

An offline generator for brand motion, and its output. Same shape as
[`validation/`](../validation): a reproducible pipeline that runs by hand, whose
result is committed rather than computed at build time (ADR 0007). Nothing
downstream builds this, and no consumer takes it yet.

**This is exploration, not the sanctioned asset.** [ADR
0012](../decisions/0012-brand-motion-on-brand-surfaces.md) puts brand motion in
`meridian-design/brand/`, pinned by a conformance test, with an animated-SVG
emitter for web alongside the Lottie one for desktop. None of that exists here.
When a form is settled, it moves there and the ADR is amended to name it; until
then this directory is where candidates are built and looked at.

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
python3 -m http.server 8801              # from THIS directory
```

Then `localhost:8801/preview/form.html`. Plain Python 3, no dependencies.

Serve from `motion/`, not from `preview/`: the pages fetch `../output/*.json`,
and a server rooted at `preview/` will not look above its own root, so every
animation 404s and the canvases sit blank. `file://` fails for the same reason —
the JSON is fetched, and that is blocked.

| Page | What it is |
|---|---|
| `preview/form.html` | The concept, four colourways, one at 120px. |
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
   disperses, frames 75–175

7.08 s at 25 fps, 17 KB. The S is the figure the mark repeats three times: two
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
