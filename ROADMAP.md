# Roadmap

## Phase 0 — Scaffold (this repo state)

Repo skeleton, ADRs 0001–0010, `meridian-design` crate skeleton (type shapes,
brand constant, emitter stubs), CI.

## Phase 1 — Palette design ✅ (2026-07-16)

Shipped: 12-step neutral/accent/semantic scales generated from Meridian seeds
(Maritime + true hue-70 warm cream); the approved "Harbour" 8-slot categorical
set (ordering by exhaustive joint-mode CVD search; first four validate
all-pairs); the Meridian blue-240 sequential ramp as an **opt-in** named
scheme (default stays viridis — Hugh's call); diverging blue ↔ brick red;
status palette; `null_ink`; the chart ink table. All values in
`meridian-design`, gated by `tests/palette_gate.rs`; canonical validator
record + reproducible pipeline + review gallery in `validation/`.

## Phase 2 — Font gate (early, cheap, blocking)

Bundle the upstream Geist builds (not the Google Fonts build — it strips
stylistic sets), render an 11px `tnum` table in Brightfield, eyeball it.
Pass → typography tokens lock. Fail → Inter + JetBrains Mono, ADR 0005
amended, nothing else changes.

## Phase 3 — Web adoption

Swap `global.css` to the emitted `tokens.css`; replace the untouched shadcn
`--chart-1..5` with the real viz palette; drop Inter from the layout; add the
table-scope `tnum` utility. Conformance test pins web against the crate.

## Phase 4 — Brightfield adoption (two PRs)

- **PR A — overlay** (PNGs byte-identical): apply the emitted `ThemeConfig`
  pair; UI/editor fonts via config; tokenise GPUI-only overlay colours (brush,
  focus ring, slider); collapse duplicated `rgba` helpers.
- **PR B — ink** (one sanctioned re-baseline behind a before/after gallery):
  chart background/axis/grid/legend/label ink, categorical palette, Meridian
  sequential default (booked as a DEV entry), `null_ink`, Geist chart-font
  bytes.
- Folded chore: implement `colorRange`/`colorDomain` consumption; teach
  `serialise_spec` to expand `colorScheme: meridian` to explicit ranges on
  export.

## Phase 5 — Guidelines

Density (12px base; row ladder 22/24/36/46), speed budgets, icon policy,
brand application.

## Standing

Revisit Masonry/Xilem at its next release (~late 2026). GPL exposure in the
current stack is contained (stub + cargo-deny gate in Brightfield), so the
Linebender move stays strategic, not urgent.
