---
status: accepted
date-created: 2026-07-27
date-modified: 2026-08-05
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
which is the globe turn at reduced amplitude rather than a third animation. A
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
  also why the wordmark animation is web-only for now.
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
