---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0005. Typography: Geist + Geist Mono (gated), Anybody display-only, dense by default

## Context and Problem Statement

The web loaded three families simultaneously (Inter as body class, Geist bound
to `--font-sans`, Anybody 600 for display) — a merge, not a decision. A data
tool also needs a mono (YAML/SQL editor, table cells, axis ticks) and a
density posture.

## Considered Options

- **Geist + Geist Mono** — one-family story, OFL, Google Fonts, distinctive
- **Inter + JetBrains Mono** — what Grafana/Superset/Rill ship; strongest
  small-size record
- **Density**: dense-by-default / Carbon-style modes / comfortable

## Decision Outcome

Chosen: Geist + Geist Mono; Inter dropped; Anybody retained for
marketing/display surfaces only (never inside the apps). Dense by default, no
density modes — the Rill 0.17 posture (12px UI base, 11px chart labels, a
row-height ladder instead of one height). Tabular figures (`tnum`) are
mandatory wherever numbers align, applied at table scope. Rill's own retreat
from a monospace UI font ("characters too wide") confirms: density comes from
a narrow proportional sans, alignment from `tnum`.

**Gate (Phase 2):** verification found two documented risks at 11–12px — Geist
needs weight/tracking compensation below 16px, and its l-vs-1 ambiguity issue
remains open. The choice locks only after an 11px `tnum` table renders
acceptably in Brightfield (Medium-ish weight, `zero` enabled). Named fallback:
Inter + JetBrains Mono. Always embed the upstream Geist builds — the Google
Fonts build strips stylistic sets. Note: no italics exist in the Geist family.

### Resolution (2026-07-16, Phase 2 gate run — FALLBACK ADOPTED)

The gate ran same-day: a Brightfield GPUI window (branch
`design/0002-font-gate`, `examples/font_gate.rs`) rendering identical 11px
Medium `tnum`+`zero` data tables in Geist (left) beside Inter (right), plus
Geist Mono beside JetBrains Mono, on the Phase 1 light surface. Hugh's
verdict: **"the Inter column reads better."** Evidence:
`validation/font-gate-2026-07-16.png` — Inter's slashed zero and l·1·I
distinction hold visibly better at 11px, consistent with Geist's documented
sub-16px weakness and open l-vs-1 issue.

**Adopted: Inter + JetBrains Mono.** Geist is dropped entirely. Anybody
(display-only) and the density posture are unchanged. Two riders:

- JetBrains Mono defaults its coding ligatures ON (`calt`) — the gate
  screenshot shows `⇒ ≠ ≤` — so the token layer carries `CALT_OFF`, required
  on every data/editor surface.
- The web cleanup direction flips: Phase 3 now KEEPS Inter (already loaded)
  and removes Geist — the reverse of the original plan.

### Consequences

- Good, because one family across sans+mono, OFL, bundleable into the desktop
  app (fontique registers in-memory blobs).
- Good, because dense-by-default fits the analyst author; no ×2 component-spec
  multiplier from density modes.
- Bad, because the small-size risk is real; the gate exists so failure costs
  one ADR amendment, not a system rework.
