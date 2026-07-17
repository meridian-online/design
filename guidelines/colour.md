# Colour

Colour is assigned by the job it does, and the colour part is computable —
so it is computed (ADRs 0006/0007). No palette ships on taste.

## The four jobs

- **Categorical (identity)** — the Harbour 8: blue, gold, teal, red, violet,
  orange, plum, green. **The order is a colourblind-safety mechanism**
  (derived by exhaustive search over CVD-simulated ΔE) — assign slots in
  fixed order, never re-sort, never cycle a 9th hue. A 9th series folds into
  "Other", facets, or gets direct labels.
- **Sequential (magnitude)** — default stays **viridis** (scientific-colormap
  familiarity; Hugh's call). The Maritime-anchored `meridian` ramp is an
  opt-in named scheme. One hue, light→dark; never rainbow.
- **Diverging (polarity)** — Maritime blue ↔ brick red with a warm-neutral
  midpoint, so "nothing" reads as nothing. Equal steps per arm; never a hue
  at the midpoint.
- **Status (state)** — good/warning/serious/critical, fixed across modes,
  **reserved**: never reused as series colours, never colour-alone (always
  icon + label). Warning takes dark text (bright-scale rule).

## Hard rules

- **All-pairs cap**: only the first four Harbour slots validate all-pairs
  (scatter, choropleth, small multiples). Past four: fold, facet, or label.
- **Relief rule**: light-mode gold/teal/orange sit below 3:1 on the surface —
  legal only with visible direct labels or a table view.
- **`null_ink`** (`#dcdad8` / `#32302f`): NULL renders as a warm grey below
  the series chroma floor — it can never impersonate data. Any new mark
  family with NULL semantics must use it.
- **Colour follows the entity, never its rank.** Filtering that changes the
  series count must not repaint survivors.
- **Text wears ink tokens, never series colour.**

## The gate

Every palette change must re-clear `meridian-design/tests/palette_gate.rs`
(the Rust port of the method's validator: lightness band, chroma floor,
adjacent CVD ΔE ≥ 8, normal-vision floor ≥ 15, first-4 all-pairs, ordinal
bounds, contrast). Regenerate values with the `validation/` pipeline; the
canonical validator record and the approval gallery live there too.

## Mosaic portability

Meridian palettes are **renderer defaults** (zero spec surface; booked as a
DEV entry). A spec that must pin colours portably carries explicit
`colorDomain` + `colorRange` literals — never the name `meridian`, which
vanilla Mosaic rejects (Observable Plot's scheme registry is closed).
Brightfield's export path expands `colorScheme: meridian` into explicit
stops automatically.
