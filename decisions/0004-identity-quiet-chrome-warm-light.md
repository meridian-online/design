---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0004. Identity: quiet chrome, Maritime as interaction accent, light-first, warm neutrals everywhere

## Context and Problem Statement

Meridian has a brand palette (Maritime, `hsl(205, 35%, 45%)` — web decision
0010) and warm-tinted OKLCH neutrals (hue ~60–80) on the web. How loudly does
the brand appear inside the tools, which theme leads, and do the warm neutrals
extend to the desktop?

## Considered Options

- **Quiet chrome / branded chrome / near-monochrome**
- **Light-first / dark-first / strict parity**
- **Warm neutrals everywhere / warm web + grey desktop / grey everywhere**

## Decision Outcome

Chosen: quiet chrome — warm-neutral surfaces with Maritime reserved for
interactive, focus, and selection states; **colour on the canvas belongs to
data** (the Zed/Rill posture, and where web decision 0010's "subtlety in
application matters" caveat points). Light-first with dark derived from the
same scales and shipping close behind. Warm neutrals extend to desktop chrome
and chart surfaces — the same move the admired Claude-artifact palettes make,
and distinctive against every blue-grey dev tool.

### Consequences

- Good, because chrome never competes with data ink in a viz tool.
- Good, because one neutral temperature reads as one product across surfaces.
- Bad, because warm chart surfaces re-baseline Brightfield's example PNGs
  (sanctioned, once — Phase 4 PR B).
- Neutral, because dark mode is derived, not co-designed — parity effort is
  deferred until the light theme settles.
