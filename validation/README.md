# Validation

Colour maths runs in CI, never by eye.

## Palette gate

Every shipped palette is checked in BOTH modes against the Meridian surfaces:
lightness band, chroma floor, adjacent-pair CVD separation (OKLab ΔE ≥ 8,
Machado 2009 severity-1.0 simulation), the normal-vision floor (≥ 15, hard
fail), first-4 all-pairs, ordinal ramp bounds, and surface contrast. The
standing CI gate is the Rust port in `meridian-design/src/validate.rs` +
`tests/palette_gate.rs` — our own implementation of the published maths, so
the repo owns its gate end-to-end. `record-2026-07-16.txt` is the canonical
validator's output for the approved Phase 1 palettes (32 PASS lines);
`gallery-2026-07-16.html` is the review page those approvals were made on.

## Reproducible pipeline

- `scales.mts` — seeds → 12-step scales (drives the vendored generator)
- `design.mts` — categorical candidate spec → metrics → exhaustive joint-mode
  ordering search (the order is a safety mechanism, not cosmetics)
- `ramps.mts` — sequential/diverging/status/null construction
- `gen-rust.mts` — emits the Rust const blocks for `meridian-design`
- `npm install` here, then `node <script>.mts` (Node ≥ 24 strips types natively)

## Scale generator

`generateRadixColors` — vendored from the MIT-licensed
[radix-ui/website](https://github.com/radix-ui/website) repo
(`components/generate-radix-colors.tsx`; deps: colorjs.io, bezier-easing,
@radix-ui/colors) — generates the 12-step neutral/accent/semantic scales from
Meridian seeds (Maritime accent, hue-70 warm cream gray). Run offline; outputs
are committed into `meridian-design`, never computed at build time (ADR 0007).
