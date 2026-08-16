# Reference

Generated reference material. Nothing in this directory is written by hand, and editing a file here is always the wrong move — the edit will be overwritten the next time somebody regenerates, and until then the file will disagree with the crate that is the actual source of truth.

| File | Emitter | Regenerate with |
|---|---|---|
| `tokens.md` | `meridian_design::emit::tokens_md` | `cd meridian-design && cargo run --example dump_md > ../reference/tokens.md` |

The published file **is** the pin. `tests/conformance.rs` compares `emit::tokens_md()` against `reference/tokens.md` byte-for-byte, so an intentional token change regenerates this directory in the same commit, exactly as it regenerates `tests/snapshots/tokens.css`. There is no second copy under `tests/snapshots/`, deliberately: a snapshot nobody reads would end up being the only version anyone checked.

## What keeps it honest

The sheet reads `emit::tokens_css()` rather than the token modules, so it restates no name and no value — the same rule `motion/svg_form.py` follows when it reads the built Lottie instead of re-choreographing the schedule. What is written by hand in `src/emit/markdown.rs` is editorial only: which heading a family of tokens sits under, and the sentence explaining what that family is for.

Four gates in `tests/conformance.rs` hold the rest, and each one exists because a snapshot cannot say it:

- **Every token the CSS emits reaches the sheet.** A snapshot pinned against the emitter's own output would happily absorb a token that stopped being rendered, and a reference that silently omits a token reads as a token that does not exist.
- **Nothing lands in `## Unclassified`.** Declarations are filed by name prefix and anything unclaimed is printed under that heading rather than dropped, so a new family of tokens fails CI until somebody decides where it belongs and what to say about it. That judgement is the one part of this the generator cannot make, and forcing it is the point.
- **Every contrast ratio is recomputed, not typed.** The expectation is derived from `semantic.rs` through the same `validate::contrast` the chrome gate uses. The floors come from `validate::TEXT_MIN` and `validate::NON_TEXT_MIN`, which both the sheet and `tests/chrome_gate.rs` read, so a published floor cannot disagree with an enforced one.
- **The colours in the sheet are exactly the colours in the CSS** — set equality, in both directions. That catches a hex typed into a sentence as readily as a scale that quietly stopped being drawn.

## What the sheet deliberately does not do

It publishes no contrast number for a pair no gate defends, because a measurement without an assertion behind it is a claim that can stop being true without anything going red. Where a reader might reasonably expect one and there is none — the diverging ramps, most of the sequential steps, the status set against each other under simulated colour-vision deficiency — the honest fix is to add the gate first.

It also declines to measure anything translucent. `validate::contrast` reads only the colour channels, so handing it `surfaces.scrim` would measure the ink the scrim is made from rather than the result of painting it over something. The crate has no compositing maths and no gate for one, so those cells print a dash.

GitHub renders no colour swatch for a hex code in a repository file — verified, not assumed — so `tokens.md` is numbers. A visual palette sheet would have to be SVG, which GitHub does serve as an image with inline `<style>` permitted.
