---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0006. Meridian-native viz palette; Mosaic portability via renderer default + explicit ranges

## Context and Problem Statement

Brightfield ships d3/Mosaic parity colours (Tableau10 categorical, viridis
sequential); the web's chart tokens are unthemed shadcn defaults. A
Meridian-native palette is the system's most visible win — but a
Brightfield-authored spec must render identically in vanilla Mosaic, and
research settled the mechanism: Observable Plot's scheme registry is **closed**
(no registration API; unknown scheme names throw at render time; Mosaic's JSON
schema enumerates the legal names). `colorScheme: meridian` can never be a
portable spec value.

## Considered Options

- **Native full set** — categorical + sequential + diverging + status, OKLCH
- **Hybrid** — native categorical, keep viridis/turbo sequential
- **Keep d3/Mosaic defaults** — zero identity
- Portability: register a custom scheme in Plot (impossible — closed registry)
  vs renderer default + explicit ranges

## Decision Outcome

Chosen: native full set, carried in two layers.

1. **Renderer default, zero spec surface**: a spec that says nothing about
   colour renders Meridian in Brightfield and Plot defaults in vanilla Mosaic
   — the already-accepted deviation pattern (Brightfield's viridis-vs-turbo
   DEV-0003). Booked as a DEV entry.
2. **Portable pinning via explicit literals**: `colorDomain` + `colorRange`
   arrays, which vanilla Mosaic accepts today. `colorScheme: meridian` is
   permitted as Brightfield-local sugar, but the spec export path expands it
   to explicit ranges.

Palette design follows the computed method (Phase 1): 8 ordered categorical
slots tuned to the warm surfaces — the ordering derived by enumerating for
maximum adjacent CVD ΔE — validated by script in both modes against our own
surfaces, with the validation output checked in as a CI gate. A dedicated
`null_ink` token gives NULL values a light neutral that cannot impersonate a
scheme value.

### Consequences

- Good, because the canvas carries the identity while portable specs stay
  byte-honest in vanilla Mosaic.
- Good, because `null_ink` closes the known NULL-renders-as-steel-blue bug as
  a side effect.
- Bad, because Brightfield must first implement `colorRange`/`colorDomain`
  consumption (currently parsed but ignored) — a prerequisite chore in
  Phase 4.
