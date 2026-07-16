---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0007. Semantic colours: Radix-shaped scales generated from Meridian seeds

## Context and Problem Statement

The system needs semantic colours (success/warning/error/info) and full
neutral/accent scales with well-defined usage steps, coherent with warm
neutrals (OKLCH hue ~60–80) and Maritime — in light and dark.

## Considered Options

- **Radix Colors scales adopted as-is** — proven 12-step anatomy, but its warm
  gray ("sand") measures OKLCH hue ~85–107: slightly yellow-green of the
  Meridian cream
- **Radix-shaped scales generated from our own seeds** — vendor the MIT
  `generateRadixColors` function (radix-ui/website repo; pure function,
  computes internally in OKLCH; takes accent + gray + background seeds)
- **Fully bespoke ramps** — maximum coherence, most work, no proven anatomy

## Decision Outcome

Chosen: generate Radix-shaped scales from Meridian seeds — Maritime as accent,
a true hue-70 warm cream as the gray seed, plus red/amber/green/blue role
seeds. We inherit Radix's proven 12-step semantics (1–2 app backgrounds, 3–5
component states, 6–8 borders, 9–10 solids, 11–12 text) and identical
light/dark step meaning, while every scale is actually Meridian's. The
generator is vendored into `validation/` and run offline; outputs are
committed, not computed at build time. Caveat carried from Radix: bright
scales (yellow/amber/sky/mint) design step 9 for black foreground text —
warning badges must respect this.

### Consequences

- Good, because step semantics give the token layer its usage grammar for
  free, and theme switching is a scale swap with stable step meaning.
- Good, because the generator is a single MIT file with two small deps —
  scriptable, no service dependency.
- Neutral, because `@radix-ui/colors` itself is in stable/maintenance mode
  (3.0.0 since Oct 2023) — acceptable since we vendor the generator, not the
  palette.
