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

## Phase 2 — Font gate ✅ (2026-07-16 — FAILED for Geist, fallback adopted)

The 11px `tnum` side-by-side ran in a real Brightfield window (branch
`design/0002-font-gate`); Hugh's eyeball: Inter reads better. **Inter +
JetBrains Mono adopted** (ADR 0005 resolution; evidence in `validation/`).
JetBrains Mono requires `CALT_OFF` (ligatures off) on data surfaces.

## Phase 3 — Web adoption ✅ (2026-07-17, web PR #44 → prod)

Shipped: the generated tokens block lives marker-delimited in web
`app/global.css`, byte-pinned to this repo's snapshot by web
`scripts/check-tokens.mjs`; `--chart-1..5` reference the Harbour slots
(dormant until the site's first chart — zero consumers today, by check);
Inter self-hosted from upstream rsms 4.1 builds, Geist dropped, Anybody
display-only; `@utility ui-numeric` (tabular figures + slashed zero, table
scope). Icons had already shipped independently (web #42 — see ADR 0009
update). Web follow-ups noted on the PR: apply `ui-numeric` to the dataset
explorer; map shadcn semantic tokens onto `--m-*`.

## Phase 4 — Brightfield adoption ✅ (2026-07-17, brightfield #62 + #63)

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
