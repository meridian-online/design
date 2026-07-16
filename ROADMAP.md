# Roadmap

## Phase 0 — Scaffold (this repo state)

Repo skeleton, ADRs 0001–0010, `meridian-design` crate skeleton (type shapes,
brand constant, emitter stubs), CI.

## Phase 1 — Palette design

The creative core. Seed the vendored `generateRadixColors` (MIT, from
radix-ui/website) with Maritime + a true hue-70 warm cream gray → 12-step
neutral/accent/semantic scales, light + dark. Then the viz set: 8 ordered
categorical slots tuned to the warm surfaces (ordering derived by enumerating
for maximum adjacent CVD ΔE), sequential ramp, diverging pair, status palette —
validated by script against **our** surfaces in both modes, output checked in
as the CI gate. Chrome/ink table falls out of the neutral scale. Design calls
made here: whether the sequential ramp is Maritime-anchored; the diverging
warm pole.

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
