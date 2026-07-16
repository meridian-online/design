---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0010. Speed and motion budgets are first-class policy

## Context and Problem Statement

"A good experience is always fast and intuitive" is a Meridian value. Values
that live only in culture erode; values written as budgets can be cited in
reviews like any other token.

## Considered Options

- **Budgets as a first-class guidelines page** — citable, testable where
  possible
- **Informal principle** — stated in an intro, enforced by nobody

## Decision Outcome

Chosen: budgets as policy (`guidelines/speed.md`, Phase 5). Starting stance,
to be refined there: interaction feedback lands next frame; no decorative
animation; motion exists only for spatial continuity (a panel opening, a brush
moving) and never exceeds the duration the eye needs; anything slower than a
budget must show its progress honestly. The Rill-style "instant" feel is the
benchmark; GPU rendering is the enabler, not the excuse.

### Consequences

- Good, because PRs and design reviews get a concrete reference — "this
  violates the speed policy" beats "this feels slow".
- Good, because it differentiates: most design systems specify how things
  look, few specify how fast they must feel.
- Neutral, because some budgets are only eyeball-testable today; instrumenting
  them is future work.
