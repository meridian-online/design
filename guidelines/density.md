# Density

Dense by default, no density modes (ADR 0005). Meridian is built for analysts
who want more on screen; density is a value, not a setting.

## Rules

- **12px UI base.** Workspace chrome text (sidebars, tabs, panels, logs) sits
  at 12px; the desktop theme sets `font.size: 12` / `mono_font.size: 12`.
  Chart axis/tick labels sit one step below at 11px.
- **No compact/comfortable toggle.** A mode multiplies every component spec
  by two and the default still has to be chosen. Choose once, dense.
- **The row-height ladder** — pick a rung, never invent a height:
  | rung | px | use |
  |---|---|---|
  | dense | 20 | hot dense lists (leaderboard-class); the unified data table (virtualised grids) |
  | grid | 24 | rows carrying their own inline pointer controls (menu option rows, pickers) |
  | preview | 36 | preview tables |
  | comfortable | 46 | settings, navigation lists |
- **Tabular figures at table scope, never globally.** Wherever digits align
  vertically — tables, tick labels, stat columns — apply tabular numerals +
  slashed zero (`ui-numeric` utility on web; `TNUM` + `ZERO` features in the
  crate). Prose keeps proportional figures.
- **Density comes from the face, alignment from the feature.** The lesson
  Rill learned in production: a monospace UI font is not the way to align
  numbers — the characters are too wide and read as "technical". A narrow
  proportional sans plus `tnum` gives density *and* alignment.
- **Right-align numeric columns.** Tabular figures align digits; right
  alignment aligns magnitudes.

## Evidence

ADR 0005 + its Phase 2 resolution. Values shipped in `meridian-design`
(`typography.rs`, `spacing.rs`); the 12px base confirmed in-app (Brightfield
#62 eyeball).
