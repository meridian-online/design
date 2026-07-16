---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0009. Icons: Tabler everywhere; shadcn internals exempt

## Context and Problem Statement

One icon language across web (React/shadcn) and desktop (SVG→path rendering in
Rust). Lucide is shadcn's default, but a viz authoring tool leans hardest on
exactly the icons Lucide lacks: measured July 2026, Lucide (1,746 glyphs) has
no sankey, treemap, radar, funnel, cohort, or histogram icons, all of which
Tabler (5,093 outline + 1,053 filled, MIT) provides, alongside the deepest
filter/sort/table families of any stroke set. SVG diffing verified the two
sets share identical root geometry (24×24, 2px stroke, round caps/joins,
`currentColor`) — visually one family. A "Lucide first, Tabler for gaps"
policy would put the domain-critical work on the gap-filler.

## Considered Options

- **Tabler everywhere; shadcn internals exempt**
- **Lucide first, Tabler for gaps** — per-icon "is this a gap?" decisions
- **Lucide only** — redraw missing chart glyphs ourselves
- Phosphor (14 months unreleased, baked weights), Material Symbols
  (font-centric, Google look), Remix (custom licence since Jan 2026) —
  eliminated in research

## Decision Outcome

Chosen: Tabler everywhere. Outline is the primary style; filled variants serve
selected/toggle states. Vendored shadcn primitives keep their internal
`lucide-react` imports — chevrons, checks, and X's indistinguishable from
Tabler's — treated as component internals, not a second icon set. The Rust
ingest strips Tabler's invisible bounding-rect path; the web sets
`optimizePackageImports` for the large package. Carbon's icon set is retained
only as a *naming/coverage checklist* when Meridian must draw a glyph Tabler
lacks (guidelines footnote, not a dependency).

### Consequences

- Good, because zero per-icon policy decisions and native coverage for
  mark-type pickers and table/filter chrome.
- Good, because MIT, healthy cadence, and live stroke-width like Lucide.
- Neutral, because Tabler releases roughly bi-monthly vs Lucide's near-weekly
  and is more of a one-maintainer project — acceptable for a vendored,
  path-flattened consumption model.
