# Meridian Design System

The single source of truth for how Meridian looks, reads, and feels — across the
web ([meridian-online/web](https://github.com/meridian-online/web)), the
Brightfield desktop app
([meridian-online/brightfield](https://github.com/meridian-online/brightfield)),
and the chart ink both of them render.

## What lives here

| Path | Contents |
|---|---|
| `meridian-design/` | The token crate — MIT, dependency-free, framework-neutral Rust. Colours, type ramp, spacing, chart palettes, and the emitters. The **only** place token values are defined. |
| `decisions/` | Architecture decision records, ADRs 0001–0011 — the scoping decisions that shaped the system, and the record of every amendment to them. |
| `guidelines/` | Six citable pages: identity, density, speed budgets, colour method, typography, icons. |
| `validation/` | Palette gates and evidence — colour maths runs in CI, never by eye. |

`meridian-design` is the token crate, and its dependency-free,
framework-neutral contract binds **that crate**, not the repository (ADR 0003).
Sibling crates live here too and may take dependencies of their own — the first
is `meridian-egui`, the egui adapter and desktop primitives (ADR 0011).

## How it is consumed

- **Web** takes the emitted `tokens.css` (CSS custom properties, light + dark),
  pinned byte-for-byte at both ends — `tests/conformance.rs`
  (`tokens_css_matches_snapshot`) gates the emitted bytes in this repo's CI,
  and a conformance check on the web side pins what ships.
- **Brightfield** takes `meridian-design` as a cargo dependency. The renderer
  reads token values directly; the egui app shell is themed through the
  `meridian-egui` adapter (ADR 0011).
- **Framework adapters are thin and they live here**, not in the consuming app
  — tokens are plain `Copy` structs with framework-neutral sRGB colours and
  logical-pixel dimensions, so a host change re-translates the adapter rather
  than the system (ADR 0003).

## Status

Phases 0–5 are shipped and the system is live on both web and desktop — see
[ROADMAP.md](ROADMAP.md) for what each phase delivered. In short: the full
palette (neutral, accent, semantic, the categorical chart set, sequential and
diverging ramps) generated and CI-gated; Inter + JetBrains Mono adopted after a
failed font gate for Geist; Tabler adopted as the one icon language and shipped
on web; tokens live in production web CSS and in the desktop chrome and chart
ink; six guideline pages written.

The **desktop component layer** lives here as `meridian-egui`: the egui adapter
and a capped set of primitives, alongside the geometry and state tokens they
consume (ADR 0011). Brightfield's move off GPUI onto egui left no host widget
library to defer to, so those primitives live in the design system. With the
desktop app on egui, the earlier gpui-component theme emitter has been retired —
the crate now emits `tokens.css` for the web, and `meridian-egui` themes the
desktop. The token crate's contract is unchanged by any of it.
