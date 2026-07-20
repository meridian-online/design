# design — the Meridian Design System

This repo defines how Meridian looks, reads, and feels. It is the **single
source of truth** for token values: colours, the type ramp, spacing and the box
model, chart ink, and the artefacts emitted from them. Two consumers today —
[`meridian-online/web`](https://github.com/meridian-online/web) and
[`meridian-online/brightfield`](https://github.com/meridian-online/brightfield).

If you are about to hard-code a colour, a radius, a control height, a duration
or a font size in a consuming repo: stop. It belongs here, and the consumer
imports it. ADR 0011 exists because that rule was not enforced and a desktop
shell grew a private shadow palette.

## Layout

| Path | Contents |
|---|---|
| `meridian-design/` | The token crate. MIT, **dependency-free by contract** (ADR 0003). The only place token values are defined, plus the emitters. |
| `decisions/` | ADRs 0001–0011 — the scoping decisions and every amendment to them. `_template.md` is the shape. |
| `guidelines/` | Six citable pages: identity, density, speed, colour, typography, icons. The rules tokens cannot carry. |
| `validation/` | Colour maths and its evidence: the reproducible `.mts` pipeline, the vendored Radix scale generator, the approved-palette record and review gallery. |
| `scripts/` | Repo gates that are not Rust tests — currently the public-hygiene check. |

Toolchain is pinned by `rust-toolchain.toml` (1.95.0). CI: `.github/workflows/ci.yml`.

## The crate is layered — reach for the highest layer that fits

1. **raw** — `scales` (12-step ramps), `viz` (chart palettes), `chrome` (the
   ink/overlay tokens the renderer reads).
2. **geometry** — `spacing`, `radius`, `control`, `focus`, `elevation`,
   `motion`. The box model, stated once.
3. **semantic** — `semantic`: what a colour is *for*, framework-neutral, with an
   instantiated `StateSet` per interaction role.
4. **contract** — `a11y`: role and keyboard intent — the half of a component
   spec that colour tokens cannot carry.
5. **emitters** — `emit`, one per consumer, each pinned by a snapshot.

Reach for the semantic layer first. Drop to a raw scale only when the thing
being coloured genuinely has no semantic name yet — and consider whether the
right fix is to give it one.

Colours are designed in OKLCH and **stored as their sRGB conversion**, so no
consumer needs colour-space maths (ADR 0008). Values are plain `Copy` structs;
colours are framework-neutral sRGB (never a host colour type), dimensions are
logical-pixel floats, interaction states are first-class token slots rather than
a cascade, and typography carries OpenType features as `(tag, value)` pairs.

`meridian-design` itself stays MIT and dependency-free — that is what keeps it
permanently outside the GPL firewall (ADR 0003, ADR 0011). **That contract binds
the crate, not the repository.** Sibling crates live here too and may take
dependencies of their own; the first is `meridian-egui`, the egui adapter and
desktop primitives (ADR 0011).

## Emitters and their pinned snapshots

Every downstream artefact is generated from the crate and pinned byte-for-byte
by a conformance test, so drift fails CI instead of shipping:

| Artefact | Emitter | Snapshot | Regenerate with |
|---|---|---|---|
| Web `tokens.css` | `emit::tokens_css` | `tests/snapshots/tokens.css` | `cargo run --example dump_css > tests/snapshots/tokens.css` |
| Desktop `ThemeConfig` (light/dark) | `emit::theme_config` | `tests/snapshots/theme-{light,dark}.json` | `cargo run --example dump_theme light > tests/snapshots/theme-light.json` (and `dark`) |

Rules:

- **An intentional token change regenerates its snapshot in the same commit.**
  A red conformance test means a consumer would have drifted; it is never fixed
  by loosening the test.
- **A snapshot only catches a change, never a missing one.** `conformance.rs`
  carries extra structural tests for exactly this reason — one caught the theme
  emitter restating chrome anchors as literal hexes instead of reading
  `chrome.rs`, which made editing a chrome token a silent no-op. When you add an
  emitter, add the test that proves it *reads* the tokens.
- Palette maths is gated by `tests/palette_gate.rs` (the Rust port of the
  validator in `src/validate.rs`), never approved by eye.
- The `validation/*.mts` scripts are an offline, reproducible pipeline; their
  output is committed into the crate, never computed at build time (ADR 0007).

Run the crate's gates from `meridian-design/`: `cargo build` and `cargo test`.

## ADR conventions

- Numbered `NNNN-kebab-title.md`, sequential, never renumbered. Frontmatter:
  `status` (`proposed` | `accepted` | `superseded`), `date-created`,
  `date-modified`. Body follows `_template.md`: context → considered options →
  decision outcome → consequences.
- **Amend in place.** When the world changes under a decision that was
  nonetheless reasoned correctly, append an `### Update (YYYY-MM-DD — what
  changed)` section, bump `date-modified`, and say plainly which clauses narrow
  and which still stand. ADR 0002's desktop-component amendment and ADR 0003's
  retirement of the Xilem watch are the worked examples.
- **`superseded-by` is for genuine supersession only** — the reasoning was
  wrong, not merely overtaken. Set it on the old record and point the new one
  back; do not use it to record an amendment, and do not silently rewrite a
  decision's history.
- Don't re-litigate a settled ADR. If the premise has changed, write the
  amendment or the successor.
- Guidelines pages distil ADRs into something citable in review — "this violates
  `speed.md`" should be a complete review comment. Keep the ADR authoritative
  and the guideline short.

## This repo is public

Public on GitHub, MIT, and read by people outside the project. Concretely:

- **No private planning identifiers.** Decision-record refs, task ids, milestone
  ids, acceptance-criterion shorthand, document ids, card ids and spec AC ids
  from the private planning tracker mean nothing here and leak the shape of
  private work. `scripts/check-public-hygiene.sh` enforces this; it runs first in
  CI and takes seconds. Run it locally before pushing — no arguments needed.
  If it fires, delete the pointer and, where it carried meaning, write the actual
  rationale in plain English. The allowlist is a last resort and every entry
  must explain itself.
- Cross-repo references are fine when they resolve publicly: ADR numbers in this
  repo, `meridian-online/*` PR numbers, upstream issue links.
- Commit messages, comments and PR text are as public as the code.

## Where the boundary sits

- **Tokens and adapters live here. Components mostly do not.** Web keeps its own
  components (shadcn on Base UI) permanently — no npm package and no TypeScript
  build path in this repo. Desktop is the exception: with no host widget library
  left after the move off GPUI, a *capped* set of egui primitives lands here as
  `meridian-egui` (ADR 0011). Capped, not a gallery — the maintenance concern in
  ADR 0002 was correct.
- **An application's information architecture never moves here.** Brightfield's
  dock model, its pane/panel/toolbar/status contracts, its pickers and modal
  layers stay in Brightfield. A design system that owns an application's IA is
  the same mistake as one that owns nothing.
- **Framework adapters are thin emitters and they live here**, not in the
  consuming app — so a host change re-translates an adapter rather than the
  system (ADR 0003).
- Consumers pin what they take: web checks the emitted CSS block byte-for-byte
  on its own side; Brightfield takes the crate as a cargo dependency. A token
  release therefore has a bump ritual at each consumer — expect it.
