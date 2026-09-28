# Colour

Colour is assigned by the job it does, and the colour part is computable — so it is computed (ADRs 0006/0007). No palette ships on taste.

## The four jobs

- **Categorical (identity)** — the Harbour 8: blue, gold, teal, red, violet, orange, plum, green. **The order is a colourblind-safety mechanism** (derived by exhaustive search over CVD-simulated ΔE) — assign slots in fixed order, never re-sort, never cycle a 9th hue. A 9th series folds into "Other", facets, or gets direct labels.
- **Sequential (magnitude)** — default stays **viridis** (scientific-colormap familiarity; Hugh's call). The Maritime-anchored `meridian` ramp is an opt-in named scheme. One hue, light→dark; never rainbow.
- **Diverging (polarity)** — Maritime blue ↔ brick red with a warm-neutral midpoint, so "nothing" reads as nothing. Equal steps per arm; never a hue at the midpoint.
- **Status (state)** — good/warning/serious/critical, fixed across modes, **reserved**: never reused as series colours, never colour-alone (always icon + label). Warning takes dark text (bright-scale rule).

## Hard rules

- **All-pairs cap**: only the first four Harbour slots validate all-pairs (scatter, choropleth, small multiples). Past four: fold, facet, or label.
- **Relief rule**: light-mode gold/teal/orange sit below 3:1 on the surface — legal only with visible direct labels or a table view.
- **`null_ink`** (`#dcdad8` / `#32302f`): NULL renders as a warm grey below the series chroma floor — it can never impersonate data. Any new mark family with NULL semantics must use it.
- **Colour follows the entity, never its rank.** Filtering that changes the series count must not repaint survivors.
- **Text wears ink tokens, never series colour.**

## A hue in chrome

A categorical hue may mark a kind of data in chrome: the head of a column, the band a field's name sits in. That is the categorical job spent on chrome, not a fifth job, and it takes one of three forms:

- **a tint under its name**: the hue at `viz::CHROME_TINT_ALPHA_LIGHT` in light and `viz::CHROME_TINT_ALPHA_DARK` in dark (`viz::chrome_tint` applies the one for the mode), with the name in a text ink token on top of it;
- **a bar on its edge**: the hue at full strength on one edge of the thing it marks;
- **a marker beside it**: a dot or a pip next to the name.

Three limits bind it:

- **It is not text ink.** The name, the glyphs and the figures on a tinted header wear `text.*` tokens, as they do on a bare one; `tests/chrome_gate.rs` holds the ink over each of the eight hues, in both modes, to the floors it holds it to over a bare surface.
- **It is not spent on chrome that stands for no data.** A frame, a toolbar, a divider, a menu and a button take no categorical hue. A control's own accent is Maritime (`guidelines/identity.md`), and that job is not this one.
- **The hard rules above still bind it.** Slots go in their fixed order, the relief rule holds for a bar or a marker in gold, teal or orange (the name beside it is its direct label), and a status colour is not borrowed to mark a kind of data.

**Which kind of data takes which hue is the consuming app's to name, not the design system's.** The system supplies the palette, the tint's strength and this rule. The app decides which of its kinds of data takes which slot and paints it where its own band is built; changing that assignment is the app's change and touches no token here.

## The gate

Every palette change must re-clear `meridian-design/tests/palette_gate.rs` (the Rust port of the method's validator: lightness band, chroma floor, adjacent CVD ΔE ≥ 8, normal-vision floor ≥ 15, first-4 all-pairs, ordinal bounds, contrast). Regenerate values with the `validation/` pipeline; the canonical validator record and the approval gallery live there too.

## Mosaic portability

Meridian palettes are **renderer defaults** (zero spec surface; booked as a DEV entry). A spec that must pin colours portably carries explicit `colorDomain` + `colorRange` literals — never the name `meridian`, which vanilla Mosaic rejects (Observable Plot's scheme registry is closed). Brightfield's export path expands `colorScheme: meridian` into explicit stops automatically.
