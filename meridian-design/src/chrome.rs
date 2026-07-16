//! Chrome tokens — the surfaces, hairlines, and ink around the data.
//!
//! Values land in Phase 1, derived from the generated neutral scale, in two
//! deliberately separate groups (they migrate in separate Brightfield PRs —
//! ROADMAP Phase 4):
//!
//! - **`ink`** — colours that reach the Vello scene and therefore rendered
//!   PNGs: chart surface, axis/tick ink, gridline hairline, label/title ink,
//!   legend panel/border. Changing any of these is a sanctioned example-PNG
//!   re-baseline event in Brightfield.
//! - **`overlay`** — GPUI-side quads that never enter the Vello scene:
//!   brush fills/borders, focus ring, live slider track/thumb. Safe to change
//!   with PNGs byte-identical.
//!
//! Each group carries light and dark values from the same scales (ADR 0004:
//! warm neutrals everywhere, light-first).
