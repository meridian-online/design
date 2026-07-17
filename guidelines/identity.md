# Identity

How Meridian looks when it's being itself: quiet, warm, precise. The brand is
felt in restraint, not colour volume (ADR 0004).

## Rules

- **Chrome is quiet.** Surfaces, panels, and bars wear the warm neutral scale
  (`--m-gray-*`, OKLCH hue ~60–80 cream) — never pure grey, never blue-grey.
  The warmth *is* the signature; if a surface samples as a cool grey, it is
  unthemed and that's a bug.
- **Maritime is a verb, not a decoration.** `#4b7a9b` (and its scale) appears
  only on things the user can act on or has selected: focus rings, links,
  primary actions, active list items, selection washes, drag affordances. It
  never tints static chrome and it is never a data series.
- **Colour on the canvas belongs to data.** Around a chart, ink; inside a
  chart, the viz palettes (see `colour.md`). If chrome competes with marks
  for attention, chrome loses.
- **Colour is reserved for the exception.** A healthy number is neutral ink;
  only the deviation earns colour (a negative delta, a failing check). Don't
  paint the normal case.
- **Light-first.** The warm-light theme is the flagship — screenshots,
  marketing, defaults. Dark is derived from the same scales with identical
  step semantics, never a naive inversion.
- **Anybody is display-only.** The brand display face appears on marketing
  and web display surfaces (headlines, wordmark lockups, OG images) and
  never inside the apps.
- **The mark.** Brand assets (the prime pattern) live in the web repo's
  `brand/` directory — dark-on-transparent for light contexts, white for
  dark, `whiteob` for high-contrast/favicon.

## Evidence

Decision record: ADR 0004. Shipped proof: web PR #44; Brightfield #62
(chrome) + #63 (canvas) — chrome samples byte-exact against the token scale.
