---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0008. Pipeline: the Rust crate is the source of truth; OKLCH definitions; emitted artefacts pinned by tests

## Context and Problem Statement

Token values must reach CSS custom properties (web), Rust constants
(Brightfield render), and gpui-component `ThemeConfig` JSON (Brightfield app)
without drift. How much machinery does that take at two-consumer scale?

## Considered Options

- **Rust crate as source; emitters + conformance tests** — no external token
  tooling
- **W3C DTCG JSON + Style Dictionary/Terrazzo** — industry machinery,
  future-proof, more to babysit
- **Hand-synced tokens.css + tokens.rs** — least machinery, most discipline

## Decision Outcome

Chosen: the `meridian-design` crate is the single definition point. Colours
are designed in OKLCH (the space used for ramp generation) and stored as their
sRGB conversion so consumers need no colour-space maths. The crate's emitters
generate `tokens.css` (light + dark custom properties) and the gpui-component
`ThemeConfig` pair; checked-in conformance tests pin the emitted artefacts so
any drift fails CI. External token tooling (DTCG/Style Dictionary) is deferred
until a third consumer materialises.

### Consequences

- Good, because "source of truth" is enforced by the compiler and CI, not
  convention.
- Good, because emitted JSON means the gpui adapter in Brightfield is ~3 lines
  of `apply_config`.
- Bad, because non-Rust contributors edit token values in Rust source — the
  values are plain consts, so the barrier is low but non-zero.
