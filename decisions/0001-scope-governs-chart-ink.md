---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0001. The design system governs web, desktop chrome, and chart ink

## Context and Problem Statement

Meridian spans a marketing/docs web surface and the Brightfield desktop
authoring app. The most visible surface of a visualisation tool is the chart
output itself, yet at decision time the web's `--chart-1..5` tokens were
untouched shadcn defaults and Brightfield shipped d3/Mosaic parity colours
(Tableau10 steel blue, viridis). Does the design system stop at UI chrome, or
does it own the canvas?

## Considered Options

- **Everything, including chart ink** — web, desktop chrome, AND default chart
  palettes/typography
- **Chrome only** — charts keep d3/Mosaic defaults for ecosystem familiarity
- **Chrome now, charts later** — defer the viz palette to its own effort

## Decision Outcome

Chosen option: "Everything, including chart ink". The viz defaults are the
most visible brand surface for a viz tool; leaving them generic forfeits the
system's biggest win. Portability is preserved by shipping the palette as
renderer defaults plus explicit-range spec attributes (ADR 0006).

### Consequences

- Good, because the canvas — the product — carries the identity, not just the
  frame around it.
- Good, because tokenising chart ink fixes known gaps in one move (untouched
  web chart tokens; Brightfield's NULL-fill rendering as a high value on blue
  schemes).
- Bad, because chart-ink changes in Brightfield move rendered output and
  require a sanctioned example-PNG re-baseline.
