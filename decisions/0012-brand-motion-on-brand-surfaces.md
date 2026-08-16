---
status: accepted
date-created: 2026-07-27
date-modified: 2026-08-16
---
# 0012. Brand motion lives on brand surfaces, and there are three of it

## Context and Problem Statement

The brand assets have never been in this repository. The mark reached both
consumers by hand-copy from a designer's local folder, and the predictable thing
happened: web stopped copying the file and started copying the *path*.
`components/datasets/dataset-explorer.tsx` carries the full 600×600 prime-mark
geometry inline as a React component, animated by a private CSS keyframe. That
inline copy has already diverged from web's own tracked `brand/meridian_black.svg`
— the two path strings differ at character 335, `c0,-35.056` against
`c-0,-35.056`. Identical geometry, different bytes, which means it came from a
separate export rather than from the tracked asset, and nothing would notice if
the next re-export moved a control point that mattered.

This is ADR 0011's finding in a different medium. There, a desktop shell grew 39
hard-coded colour literals because the crate had no geometry to import. Here, a
web app grew a private copy of the logo because the system did not ship one. The
mechanism is the same: **the system did not supply it, so the consumer invented
it.** Meanwhile the desktop front door shows no mark at all.

Bringing the files in found a second copy of the same defect, upstream of any
consumer: **the six shipped SVGs did not agree with each other.** The two white
variants carried `c-0,-35.056` where the other four carried `c0,-35.056` — the
same token, the same character position. Numerically identical, so every renderer
drew the same picture and nothing ever complained, but a negative zero is not a
thing anyone chooses, so the set came from more than one export session. They
were normalised on the way in. Note what that means: the divergence was not
introduced by the consumer copying carelessly. It was *already in the assets*,
and hand-copying propagated it. The gate has to sit on the files themselves, not
only on the consumers.

Bringing the assets in forces a second decision, because the reason to bring them
in now is motion — a loading state where the mark turns like a globe, and a
wordmark that forms — and `guidelines/speed.md` currently forbids that in a
sentence with no exceptions:

> **No decorative animation.** Motion exists only for spatial continuity … and
> stays under ~150 ms. Nothing eases in for delight.

Both proposed assets are decorative under that sentence. The rule is not wrong.
It was written for application chrome, at a time when this system had no brand
surfaces in scope, and it has been doing useful work: it is why the desktop has
no easing anywhere and why `motion.rs` carries two constants instead of twelve.
Loosening it casually would cost more than the animations are worth. But leaving
it unamended means the first reviewer to cite `speed.md` against a marketing
animation is *correct*, and the work cannot ship.

## Considered Options

- **Leave the policy alone; no brand motion** — the mark stays static everywhere
- **Scope the policy to brand surfaces, and cap the assets** — motion is allowed
  where the product is being presented, never where it is being used
- **Relax the policy generally** — allow easing and entrance animation across the
  apps, on taste
- **Put brand motion in the consuming repos** — each app animates its own logo

## Decision Outcome

Chosen: **scope the policy to brand surfaces, and cap the assets at three.**

**Where motion is allowed.** Marketing pages, the install page, OG and social
images, the desktop front door, the README. These are surfaces where the product
is being *presented*. Motion there is doing the job the surface exists for.

**Where it is not.** Application chrome, and anything on a data surface. Not a
panel, not a toolbar, not a chart, not a row. `speed.md`'s budgets are unchanged
inside the apps and this ADR narrows nothing about them: feedback still lands
next frame, spatial continuity still stays under 150 ms, and there is still no
easing for delight anywhere a user is working. *(This clause is narrowed by the
Update of 2026-08-05 below — read them together.)*

**The one in-app exception is not an exception.** The honest-work indicator —
the cue shown when something can exceed ~100 ms — is *required* by `speed.md`
already, under the honesty line. Web has been shipping a hand-rolled version of
it for as long as the dataset explorer has existed. Drawing it with the mark
instead of a generic spinner changes what it looks like, not whether it is
allowed.

**Cap: three assets.** The globe turn; the wordmark forming; the work indicator,
which is the globe turn at reduced amplitude rather than a third animation.
*(Now read as `form`; the lockup forming; the work indicator, which is `form`.
See the updates at the end — the count and the relationship are unchanged.)* A
fourth is earned by a named product requirement that provably cannot be composed
from the three — the same two-door rule ADR 0011 put on primitives, for the same
reason. A library of brand animations is the maintenance burden ADR 0002
correctly rejected, arriving one tasteful addition at a time.

**Brand motion is a brand asset, not a component.** It lives in
`meridian-design/brand/` beside the mark it animates. The consuming app decides
*where* to play it; this repo decides *what it is*. That line is the same one
ADR 0011 drew around information architecture.

### Where the assets live, and under what terms

`meridian-design/brand/`, beside `fonts/`. Inside the crate, not at the
repository root, because the desktop consumer takes `meridian-design` as a Cargo
dependency and needs the bytes at compile time; a root-level directory would not
reach it.

That puts a trademark inside an MIT crate, so the carve-out is explicit and
stated in three places: `brand/LICENSE-BRAND.md`, the root `README.md`, and the
root `LICENSE`. The MIT grant covers the source code; it does not cover
`brand/`. The vendored typefaces already set this precedent one directory over —
MIT code, OFL fonts, each artefact carrying the terms that actually govern it.

### Format splits by consumer, and the split is deliberately asymmetric

**Desktop takes Lottie, rendered by velato.** velato 0.11 targets Vello 0.9,
which is what the desktop app already pins; it is MIT/Apache-2.0, so it raises
none of the licence questions ADR 0011 had to answer for the previous host's
component layer; and the shell already rasterises a `vello::Scene` onto a wgpu
texture inside egui, sharing a single wgpu instance. velato's output *is* a
`vello::Scene`. This adds a parser, not a renderer.

**Web does not take a Lottie runtime.** `lottie-web` is roughly 75 KB gzipped of
JavaScript; the WASM player fetches a ~500 KB engine from a CDN by default. The
site is a static export that ships no motion library at all today, and it has
already done deliberate work to stop fetching a runtime from a CDN. Neither cost
is worth paying for two brand animations when the alternative is a `<style>`
block and six paths. **Web takes animated SVG**, with `prefers-reduced-motion`
honoured — which `speed.md` requires regardless.

The asymmetry is the point. The same motion, expressed twice, from one source.

### The assets are generated, not drawn

The design tool in use has no Lottie export, so *some* pipeline step was always
going to be non-obvious. For this mark, generation is the right answer anyway,
because the mark is already parametric: `brand/sources/Meridian Prime
Components.svg` is one tooth path, placed three times, mirrored once, clipped by
a circle. Six teeth from one primitive and two transforms.

So: one generator, two emitted artefacts, both committed and pinned by a
conformance test. That is the pattern this repo already runs for the colour
pipeline (ADR 0007 — offline, reproducible, output committed rather than
computed at build time) and for `tokens.css` (ADR 0008). A hand-tuned editor
remains the right tool for anything organic enough to need hand-drawn easing;
nothing here is.

### The globe turn

*(This section is retired as a specification by the Update of 2026-08-05 below.
The derivation stands; the animation it names is not the one to build.)*

**The globe turn is derived from what the mark is, which is not what it looks
like.** Decomposing the shipped path segment by segment: each of the six teeth
is a bar running from a vertical axis out to the disc limb, closed at its inner
end by a **cap of constant radius**. The bars alternate side, top to bottom.

The constant radius is the finding. A meridian projected onto a sphere makes a
cap that is widest at the equator and pinches toward the poles; the mark's caps
are radius 100 at every latitude. **So the mark is a stylised globe, not a
projection of one**, and an animation that treats the existing curve as an
ellipse to be scaled would be animating a shape the mark does not contain.

The turn therefore sweeps one parameter per bar — where its cap sits on the axis
— and leaves the cap's radius alone: `x_cap(t) = 300 + side·100 + A·sin(ωt + φ)`,
clipped to the disc. At `A = 0` the output is the shipped mark exactly, which is
the property that makes the generator checkable. Both renderers express it as an
animated path under a circular clip.

Worth stating because the animation shipping today is a `rotate()`, and a rotate
is the wrong transform for this mark either way: the bars are horizontal, so
spinning them reads as a wheel rather than a sphere.

Two free parameters remain, and they are aesthetic rather than derivable: the
phase offset between bars (π reproduces the mark's alternating rhythm; π/3 reads
as a helix) and the sweep amplitude (300 lets a bar cover or clear the whole
disc; 150 keeps every bar partly on screen at all times). Those are settled by
looking, not by argument — see the prototype linked from the pull request.

### What the desktop renderer cannot do, verified in its source

These are authoring constraints, not blockers, and they are cheaper to know now:

- **Repeater is not implemented.** The importer's match arm for it is commented
  out. The mark is six repeated teeth; authored as a repeater it renders as
  nothing at all. Bake the six.
- **Boolean operations are not implemented.** Pre-flatten; do not ship live
  merges.
- **There are no text layers.** The wordmark must be outlined curves. This is
  also why the wordmark animation is web-only for now. *(Second clause lapsed
  2026-08-16 — the wordmark is outlined, so there is no text layer to want and
  the lockup ships in both formats. The first clause stands and is now load
  bearing. See the update at the end.)*
- **Trim path *is* implemented**, in both the importer and the renderer, which
  the upstream README's summary of unsupported "advanced shapes" obscures. Draw-on
  works. Stroke dash — the other way to draw a line on — genuinely is missing.
- **Prefer plain masks to track mattes.** The matte path carries an unresolved
  note in the renderer; plain masks apply cleanly as clip layers.

### Update (2026-08-05 — the data-surface clause narrows; the globe turn retires)

Two amendments, taken together so the record moves once rather than twice.

**1. "Anything on a data surface" narrows to *decorative* motion on a data
surface, and the test is what the motion depicts.**

The clause above bars motion by location. That was the right rule for the
question this ADR was asked — where the *mark* may animate — and it is the wrong
rule for motion in general, because this decision already admits a case that
location cannot explain. The honest-work indicator is allowed inside the apps, in
a data grid among other places, and the reason given for it above is not where it
sits but what it shows.

So the test is what the motion depicts, and it reads the same way everywhere:

- **Admitted: motion that encodes a property of the data.** A dot travelling an
  edge of a graph shows which way the graph flows. Direction is a property of
  the topology and the dot encodes it, as an arrowhead does. That is data ink,
  and it survives on the same ground the honest-work cue survives on.
- **Refused: motion tied to no state.** An ambient field behind a canvas depicts
  nothing about what is drawn on it. `guidelines/speed.md` is unchanged here —
  rendering headroom is spent on more data on screen, not on ornament — and a
  surface that wants one ships it behind a preference a reader can leave off.
- **The ~150 ms ceiling is a continuity budget.** It bounds a transition. A cue
  that runs while a surface is idle is not a transition, so the ceiling does not
  reach it; what governs that cue is whether it depicts something.

**This does not let the mark animate in a toolbar.** What it admits is ink that
says something about the data and happens to be drawn as motion. The
brand-surface list, the three-asset cap, the move of the assets into
`meridian-design/brand/`, the trademark carve-out and the format split are all
untouched — this narrows one sentence.

It is an amendment and not a supersession: the reasoning stands, and the scope
was written when brand motion was the motion in question. `guidelines/speed.md`
carries the same test now, so the sentence quoted in the Context section above is
that page as it read on 2026-07-27, not as it reads today.

**2. The globe turn specified above is retired as a specification. The cap is
not.**

`x_cap(t) = 300 + side·100 + A·sin(ωt + φ)` is derived from the mark's own
geometry, and at `A = 0` it reproduces the shipped mark, which is what makes a
generator for it checkable. Both of those still hold. What did not survive was
the animation itself: built and viewed rather than argued, it does not read as a
globe turning, and the phase and amplitude that section left free are not what
would fix that.

`guidelines/identity.md` binds any replacement: the mark is not redrawn. An
animation that has to mirror the figure, or close it with a straight edge inside
the disc, is redrawing it — so the repairs a sweep of this shape invites are not
available either.

**So that section specifies nothing to build.** The turn and the work indicator
derived from it are still assets the cap allows; their *form* is open, and a
later amendment names it. This changes what an asset looks like, not how many
there are or where they may play.

### Update (2026-08-16 — the first asset is named, and both artefacts exist)

The Update above left the first asset's form open and said a later amendment
would name it. This is that amendment. It names one asset and records that it
is built; the cap, the surfaces and the format split are untouched.

**The asset is `form`, and it is the mark assembling.** An arc laps the disc
outline, right over the top, and collapses into the point where the first stroke
appears. Three strokes then flow through, one per row of the mark, right to
left. The mark fills in behind them row by row, holds, and disperses the same
way. 7.2 s at 25 fps, looping. Every movement carries a coloured wake one frame
behind it, which makes the colour a flash you catch rather than a trail you
watch.

It is derived from the mark the way the retired turn was meant to be, and the
derivation is the reason it reads. The stroke that crosses each row is the
figure the mark repeats three times — two quarter circles of radius 100 meeting
at a pinch with colinear tangents — run along that row's own edges, so each fill
lands exactly on the line that predicted it. And the wipe that reveals a row
does not travel linearly: its edge moves as a meridian on a turning sphere,
`x = 300 + 300·cos(πt)`, nearly still at each limb and quickest across the
centre. That is the foreshortening the eye reads as rotation, and it is what
survived of the globe turn. The mark is never redrawn, mirrored or closed with a
straight edge, so `guidelines/identity.md` binds nothing here.

**It occupies the first slot, and the work cue is still not a third animation.**
The cap stated above reads *the globe turn; the wordmark forming; the work
indicator, which is the globe turn at reduced amplitude*. Read it now as **`form`;
the wordmark forming; the work indicator, which is `form` rather than a third
animation**. The count is unchanged and so is the relationship: the indicator is
a cut of the first asset, not a new one.

`form` is long for a >100 ms cue, and that is a property rather than a problem.
The beats run shortest first: a wait that resolves in a few hundred milliseconds
shows only the arc growing on the disc, which is what a spinner is; a wait long
enough to matter shows the mark assemble. Nothing has to be chosen in advance.
If measurement later shows the long loop is wrong in an application, the cut to
make is the arc alone — frames 0–40, 1.6 s — which is already inside the asset
and needs no new decision.

**Both artefacts exist, and the pin this ADR asked for is two gates rather than
one.** `meridian-design/brand/motion/` holds `form.json` and `form_dark.json`
for the desktop and `form.svg` and `form_dark.svg` for the web, under
`brand/LICENSE-BRAND.md` rather than the crate's MIT grant, embedded by
`src/brand.rs` so the desktop reaches them as a cargo dependency.

This ADR said *"one generator, two emitted artefacts… without the pin it would
be two animations that merely resemble each other."* That is right and it is not
sufficient, because a byte comparison can only say a file has not changed. So
the SVG emitter **reads the built Lottie** — the paths, the trim schedule, the
colours, the loop length — rather than choreographing the same thing twice; the
two files cannot describe different animations, because only one of them
describes an animation at all. `scripts/check-motion.sh` then holds the bytes
against the generator in CI, and `meridian-design/tests/motion.rs` holds what
bytes cannot say: that the teeth are the tracked mark's own path text, that
every colour is a value in `tokens.css`, that both formats loop for the same
time, and that dark is light recoloured rather than redrawn.

**The one authoring constraint above that the built asset does not keep — and
what checking it found.** This ADR advises *"prefer plain masks to track
mattes"*, because velato's matte path carried an unresolved note where its masks
apply cleanly. `form` reveals each row with a track matte, since a Lottie matte
applies to exactly one layer and the three rows arrive on three separate beats.
That left the desktop as the one consumer the asset had not been put through, so
it was put through it.

**velato 0.11 takes the mattes.** Both artefacts import without error, and at
every beat — the arc, the strokes, the mark assembling, holding and dispersing —
all nine matted row precomps produce draw calls, in both themes. Alpha mattes
arrive as `Compose::SrcIn`, which is the operator an alpha matte means. The note
this ADR was written against is a dead `Matte` enum in `runtime/model`, never
matched anywhere; the live path is `mask_layer`, set during import, which is
what the advice above predates rather than describes.

That was measured without a GPU. velato renders through a `RenderSink` trait, so
a sink that records rather than rasterises says exactly what the desktop would
be asked to draw, with the Lottie layer names attached. **The boundary is
therefore compositing, not import**: what has not been exercised here is vello
painting `SrcIn` on the GPU, because velato pulls vello without its renderer.
The desktop shell already rasterises vello scenes for everything else it draws,
so that half is exercised by the app rather than by this repository — but it is
worth knowing which half is which.

The advice stands as advice. If a later shell finds the compositing wrong, the
change is bounded and known: emit Lottie masks instead, and teach the SVG
emitter to read them where it reads mattes today. It already turns every matte
into a `clipPath`, which is the shape a mask arrives in anyway.

**Two consequences worth stating.**

The web artefact settles to the assembled mark under
`prefers-reduced-motion: reduce`. That satisfies the letter of this ADR and of
`guidelines/speed.md`, and it is worth being plain that a still mark says the
brand rather than that work is happening — **a surface using `form` as the work
cue owes a reader who has asked for less motion a textual cue as well.** The
still frame carries the same information only when the surface supplies that
half.

The leader's ink was a picked colour for the whole exploration — `#1a1917` on
light, `#e9e6e0` on dark, neither of them a token value, both close enough to
`--m-ink` to pass every review. The mark beside them was never quite the same
colour as the animation of it. That is ADR 0011's shadow palette a third time,
in a generator rather than an app, and it is why the gate reads `tokens.css`
instead of a list of approved hexes written beside the test.

### Consequences

- Good, because the mark stops being a file in a local folder that reaches
  production by hand-copy, and the drift that already happened gets pinned by a
  test rather than by review attention.
- Good, because the speed policy is narrowed on purpose, in public, with the
  boundary written down — rather than being quietly broken by the first
  animation that ships and then treated as precedent.
- Good, because the cap is set before anyone has drawn anything, which is the
  only time a cap is free.
- Neutral, because the two renderers see different artefacts. One generator and
  a byte-pinned snapshot each is what keeps that honest; without the pin it
  would be two animations that merely resemble each other.
- Bad, because this repo now carries binary artefacts under terms that differ
  from the crate surrounding them, and a contributor reading only the root
  `LICENSE` could get it wrong. Three statements of the carve-out is the
  mitigation, and it is a real cost.
- Bad, because "brand surface" is a judgement call at the edges — a docs page
  header, an empty state that is *nearly* marketing. The cap is what stops that
  ambiguity from mattering much: there are three assets, and adding a fourth is a
  decision someone has to argue for.

### Update (2026-08-16 — the second asset is the lockup, and it is not web-only)

Two amendments, both narrowing clauses that were reasoned correctly and have
since been overtaken by the wordmark acquiring a vector source.

**1. The second slot is *the lockup forming*, not *the wordmark forming*.**

The cap now reads **`form`; the lockup forming; the work indicator, which is
`form` rather than a third animation**. The count is unchanged, and so is the
relationship between the three.

The original wording predates the assets existing. Reading it back with the
brand surfaces this decision itself lists — marketing, install, OG images, the
desktop front door, the README — none of them shows a bare wordmark; they all
show the lockup. An animation of the wordmark alone would have been an asset
with no surface, which is a worse outcome than the cap was defending against.

The relationship that settles it is the one already established for the work
indicator: **build the larger thing and the smaller one is a cut of it.** The
lockup's second beat, frames 33–73, is the wordmark forming on its own; a
surface that wants only that takes the cut, exactly as an application wanting a
shorter work cue takes `form`'s arc alone. The reverse does not hold — a
wordmark animation cannot be grown into a lockup without choreographing the
mark and the placement, which is the whole of the work.

**2. "The wordmark animation is web-only for now" has lapsed.**

That clause sits under *What the desktop renderer cannot do*, beside
"there are no text layers", and it was a correct reading of velato at the time:
with no outlined wordmark in the repository, an animated wordmark would have had
to be live type, which velato cannot draw at all.

`meridian_wordmark.svg` and `meridian_lockup.svg` are outlined curves, so there
is no text layer to want. The constraint the clause depended on no longer
applies, and **the lockup ships in both formats, like `form`.**

One thing was verified rather than assumed before saying so, because the lockup
exercises something `form` never did: the mark's six teeth have no counters, and
R, D and A do. velato parses a fill rule into its schema and then **hardcodes
`Fill::NonZero` in `runtime/vello.rs`**, never reading it back — so a counter is
a hole there only if it winds against its outline. Measured on the artwork: every
letter outline winds clockwise and every counter anticlockwise, so the holes are
real in velato as well as in the browser. `brand.rs` asserts the winding, because
it is a property both renderers depend on and neither declares — and note that a
flipped winding would fail in both at once rather than in one, since the SVG
declares `fill-rule:nonzero` too.

What is *not* covered is unchanged from the 2026-08-16 note above: vello painting
that compositing on the GPU, which the desktop shell exercises for every scene it
draws.

**What does not move.** Brand motion is still confined to brand surfaces, still
capped at three, and the mark still does not move in application chrome or on a
data surface. The lockup is emphatically a front-door asset — `guidelines/speed.md`
admits the honest-work cue as the mark's one in-app appearance, and that cue is
`form`, not this.
