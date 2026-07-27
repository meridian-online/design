# Roadmap

## Phase 0 — Scaffold ✅ (2026-07-16)

Repo skeleton, the first ten ADRs (0001–0010), `meridian-design` crate skeleton
(type shapes, brand constant, emitter stubs), CI. Later phases filled the
skeleton in; ADR 0011 followed on 2026-07-20.

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

## Phase 5 — Guidelines ✅ (2026-07-17)

Six citable pages shipped in `guidelines/`: identity, density, speed
budgets, colour, typography, icons — each distilled from the ADRs and the
shipped phases, with evidence links. The roadmap is complete; the system is
live on web and desktop.

## Standing

The Masonry/Xilem watch is **retired** (ADR 0003 update, 2026-07-20):
Brightfield committed to egui and paid the migration cost, so reopening
Linebender is a new decision rather than a standing assumption. The
framework-neutral crate shape is kept on its own merits.

## Phase 6 — Desktop components ✅

Brightfield's move off GPUI left no host widget library to defer to, so a capped
set of egui primitives and the egui adapter landed here as a second crate,
`meridian-egui`, alongside the geometry/state token layer they consume
(ADR 0011). The token crate's contract is unchanged.

With the desktop app on egui, the gpui-component `ThemeConfig` emitter that
Phase 4 introduced has been **retired** — it had no remaining consumer once the
shell cut over. The crate now emits a single artefact, `tokens.css` for the web
(pinned by `tests/conformance.rs`); the desktop is themed through
`meridian-egui`.

## Phase 7 — Brand assets and motion 🚧 (2026-07-27)

The mark and wordmark come under version control as `meridian-design/brand/`,
reserved rather than MIT, with the carve-out stated in the root `LICENSE`, the
root `README` and `brand/LICENSE-BRAND.md`. They had been reaching production by
hand-copy from a local folder, and web had already stopped copying the file and
started copying the *path* — an inline mark that has diverged from the tracked
SVG by a byte.

ADR 0012 settles the motion question the assets raised: brand motion is allowed
on brand surfaces (marketing, install, OG, the desktop front door, the README),
never in app chrome or on a data surface, capped at three assets. Desktop takes
Lottie through velato — which targets the Vello version the app already pins;
web takes animated SVG rather than a 75 KB JS runtime or a CDN WASM fetch. Both
are emitted from one generator and pinned, on the `tokens.css` model.

Outstanding: the generator and its two artefacts; the wordmark has **no vector
source** and needs an outlined SVG export before it can be animated at all; and
the web-side check pinning the inlined glyph to `brand/` is a change in that
repo, not this one.
