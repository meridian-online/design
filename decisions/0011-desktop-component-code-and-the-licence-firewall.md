---
status: accepted
date-created: 2026-07-20
date-modified: 2026-07-20
---
# 0011. Desktop component code lives here; the licence firewall holds

## Context and Problem Statement

ADR 0002 declined a component layer at two-consumer scale and assigned desktop
components to the host library (gpui-component). Brightfield is now migrating
off GPUI onto egui + Vello, which voids that premise on the desktop half:
egui ships roughly fifteen primitive widgets and no system at all, so there is
no host library left to defer to.

An audit of the desktop shell shows what the resulting vacuum produced. A
private shadow palette of 34 hard-coded hex literals across 11 distinct values,
with **zero** occurrences of Maritime — the interaction accent had drifted to a
generic azure at ~83% saturation where Maritime sits at 35%. One concept,
focus/selection, rendered five different ways. Seven empty states with seven
treatments. One modal card copied four times at widths 560/480/520/480.
`meridian_design::spacing` imported zero times, with row heights redeclared in
four places that agree only by coincidence. Zero Tabler icons against ADR 0009.

The root cause is upstream of the app: `meridian-design/src/spacing.rs` still
carries an unfulfilled "Phase 1" TODO. The crate holds 214 colour values and
six dimension values — no radii, no control heights, no focus geometry, no
elevation, no durations. A consumer cannot import geometry that was never
defined, so it invents it, eleven values at a time.

## Considered Options

- **A second crate here** — `meridian-egui`, adapter + primitives, alongside
  the token crate
- **Primitives in the application repo** — each app owns its own widget layer
- **Nothing; grow tokens only** — assume better tokens close the gap alone
- **A cross-platform component system** — one abstraction serving web and
  desktop

## Decision Outcome

Chosen: a new sibling crate **`meridian-egui`** in this repo, depending on
`meridian-design` and `egui` and nothing else. ADR 0003 already anticipated the
egui adapter as "a third thin emitter", and emitters live here. The token
crate's own contract is unchanged; it grows a geometry/state/semantic layer,
but stays plain `Copy` structs, MIT, dependency-free.

**Tokens alone were never going to close it.** Eleven invented colour values
are a symptom; five focus treatments and four modal widths are not values a
token file can supply, because the decision they encode is a *shape*, not a
number. Radii and control heights belong in the token crate; the widget that
consumes them consistently does not.

**What does not move.** The application's information architecture — its dock
model, its pane/panel/toolbar/status contracts, its picker and modal layers —
stays in Brightfield and is never exported. A design system that owns an
application's IA is the same mistake as a design system that owns nothing:
both put the decision in the wrong repo. **Web keeps its own components**
(shadcn on Base UI), explicitly and permanently. No npm package, no TypeScript
build path in this repo. A cross-platform component abstraction serving both
targets is rejected outright — the two hosts share a look, not a widget model.

### The licence firewall

Zed's `ui`, `ui_macros`, `theme`, `workspace`, `component` and
`component_preview` crates are **GPL-3.0-or-later** (verified 2026-07-20
against each crate's `Cargo.toml`; `crates/ui/` carries its own `LICENSE-GPL`).
Their API shapes and interaction patterns may be read and reimplemented from
understanding. **No code, no enum bodies, no token values may be copied**, in
any quantity, including "just the variant names". GPUI itself is Apache-2.0;
the firewall is around the component layer built on it, which is where the
temptation lives.

rerun's `re_ui` is harvestable, with one correction to the obvious reading:
its declared licence is `(MIT OR Apache-2.0) AND OFL-1.1` (verified 2026-07-20
on crates.io and in the crate's `Cargo.toml`). The **code** is dual
MIT/Apache-2.0 and may be adapted **with attribution**; the OFL-1.1 term is
there for the font binary the crate vendors (`data/Inter-Medium.otf`, shipped
with its own `OFL.txt`), which we have no reason to take — Brightfield already
carries its own Inter and JetBrains Mono under those fonts' own OFL terms
(ADR 0005). Take patterns and code; leave the vendored font where it is.

### The primitive cap

**Cap: 15 primitives.** A primitive is earned by **three independent call
sites**, or by a named product requirement that provably cannot be composed
from existing primitives. Those are the only two doors.

A primitive that needs a third variant parameter was the wrong abstraction:
inline it back to its call sites and delete it. This is a rule, not a
sentiment — write the deletion into the same PR that discovers the third
variant.

Without the cap this layer becomes exactly the maintenance burden ADR 0002
correctly rejected, arriving one reasonable-sounding addition at a time. The
cap is the mechanism that lets us say yes to components without saying yes to
a component gallery.

### The crate-boundary test

Anything that is **pure data**, or that the web `tokens.css` emitter needs,
lives in `meridian-design`. Anything that **names an `egui` type** lives in
`meridian-egui`. The test is mechanical — a compiler error, not a judgement
call — and it is exercised continuously by two live consumers rather than by
appeal to a hypothetical future port.

### Consequences

- Good, because the shadow palette has somewhere to go: one accent, one focus
  treatment, one empty state, one modal width, defined once.
- Good, because the firewall is written down before the first widget is
  drafted, when honouring it is free.
- Good, because the boundary is enforced by the build rather than by review
  attention.
- Neutral, because this repo comes to ship two crates rather than one, and the
  release ritual, CI matrix and README have to account for both.
- Bad, because a component layer is a standing maintenance cost that ADR 0002
  was right to fear. The cap is the whole answer to that, and it only works if
  it is enforced against a proposal someone wants.
