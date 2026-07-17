# Typography

Three faces, each with one job (ADR 0005 + Phase 2 resolution).

## The stack

- **Inter** — the sans. All UI chrome, web body, chart labels/titles. Chosen
  over Geist by the Phase 2 font gate: a real in-app 11px `tnum` table,
  side by side, eyeballed — Inter read better (evidence:
  `validation/font-gate-2026-07-16.png`; harness kept on the Brightfield
  branch `design/0002-font-gate` for future font questions).
- **JetBrains Mono** — the mono. YAML/SQL editors, code, anywhere the grid
  matters. **`calt` must be off** (`CALT_OFF`): it ships coding ligatures on
  by default and `=>` must read as two glyphs on data surfaces.
- **Anybody (600)** — brand display only, marketing/web surfaces, never
  in-app.

## Rules

- **Bundle upstream builds only.** Web self-hosts the rsms variable woff2s;
  desktop embeds the static TTFs from `meridian_design::fonts` (OFL licences
  ship alongside). Webfont-service builds strip OpenType features and are
  banned — a missing `tnum` fails silently.
- **Features by scope**: `tnum` + `zero` wherever digits align (table scope,
  tick labels — see `density.md`); `calt 0` on all mono data surfaces.
- **Sizes**: 12px UI base, 12px mono, 11px chart labels (dense-by-default).
- **Weights**: Regular for running text; Medium for small-size emphasis and
  dense table text (small sizes want the slightly heavier cut); SemiBold+
  for headings. Don't synthesise weights.
- **One face per job.** No per-surface font choices; if a surface seems to
  need a different face, the surface is misclassified — reread the list
  above.

## Evidence

ADR 0005 (decision + gate resolution). Shipped: web #44 (self-hosted Inter,
Geist dropped), Brightfield #62 (UI/editor faces via theme) + #63 (Inter
chart text).
