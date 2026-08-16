<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="meridian-design/brand/motion/lockup_dark.svg">
    <img src="meridian-design/brand/motion/lockup.svg" alt="Meridian" width="420" height="105">
  </picture>
</p>

# Meridian Design System

The single source of truth for how Meridian looks, reads, and feels — across the web ([meridian-online/web](https://github.com/meridian-online/web)), the Brightfield desktop app ([meridian-online/brightfield](https://github.com/meridian-online/brightfield)), and the chart ink both of them render.

## What lives here

| Path | Contents |
|---|---|
| `meridian-design/` | The token crate — MIT, dependency-free, framework-neutral Rust. Colours, type ramp, spacing, chart palettes, and the emitters. The **only** place token values are defined. |
| `meridian-design/brand/` | The mark, the wordmark, and their Affinity sources. **Not MIT** — see below. |
| `meridian-egui/` | The egui adapter and a capped set of desktop primitives (ADR 0011). Takes dependencies of its own; the token crate's contract is unaffected. |
| `decisions/` | Architecture decision records, ADRs 0001–0012 — the scoping decisions that shaped the system, and the record of every amendment to them. |
| `guidelines/` | Six citable pages: identity, density, speed budgets, colour method, typography, icons. |
| `reference/` | [`tokens.md`](reference/tokens.md) — every token, both themes, generated. Not written by hand and not to be edited by hand. |
| `validation/` | Palette gates and evidence — colour maths runs in CI, never by eye. |
| `motion/` | The offline generator for brand motion. What it emits lives in `meridian-design/brand/motion/` — two formats from one source, pinned by CI. |

`meridian-design` is the token crate, and its dependency-free, framework-neutral contract binds **that crate**, not the repository (ADR 0003). Sibling crates live here too and may take dependencies of their own — the first is `meridian-egui`, the egui adapter and desktop primitives (ADR 0011).

## How it is consumed

- **Web** takes the emitted `tokens.css` (CSS custom properties, light + dark), pinned byte-for-byte at both ends — `tests/conformance.rs` (`tokens_css_matches_snapshot`) gates the emitted bytes in this repo's CI, and a conformance check on the web side pins what ships.
- **Brightfield** takes `meridian-design` as a cargo dependency. The renderer reads token values directly; the egui app shell is themed through the `meridian-egui` adapter (ADR 0011).
- **Framework adapters are thin and they live here**, not in the consuming app — tokens are plain `Copy` structs with framework-neutral sRGB colours and logical-pixel dimensions, so a host change re-translates the adapter rather than the system (ADR 0003).

## The palette

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="reference/palette_dark.svg">
  <img src="reference/palette.svg" alt="The Meridian palette: six twelve-step ramps, the categorical chart set, the sequential and diverging ramps, and the status colours">
</picture>

Generated from the crate, not drawn — every chip's fill is the string the web consumes, and a CI gate fails if one is a colour `tokens.css` does not emit. It has a dark twin because the system does; both are pinned byte-for-byte.

## The evidence

These pages are published at **<https://meridian-online.github.io/design/>** — the palette review gallery ADR 0006 rests on, and the six harnesses the brand motion was measured with. They were always in the repo and always unreadable without cloning it and serving from the repository root, which is a strange place to keep the evidence for a decision anyone might want to check.

## Looking things up

[`reference/tokens.md`](reference/tokens.md) is every value the system defines, in both themes: the six 12-step ramps, the chart palettes, the semantic layer, all six interaction roles across their six states, the box model, and the keyboard-and-role contract.

It is generated, and generated from `tokens.css` rather than from the token modules — so the page and the artefact the web ships cannot disagree, and a token added to the system reaches the reference without anyone remembering to put it there. Its contrast section publishes measured WCAG ratios, and only for pairs a CI gate already holds to a floor: a number with no assertion behind it is a claim that can stop being true without anything going red. The floors it prints and the floors `tests/chrome_gate.rs` enforces are the same two constants.

For the reasoning rather than the values, `guidelines/` has six short citable pages, and `decisions/` has the records they distil.

## Status

Phases 0–5 are shipped and the system is live on both web and desktop — see [ROADMAP.md](ROADMAP.md) for what each phase delivered. In short: the full palette (neutral, accent, semantic, the categorical chart set, sequential and diverging ramps) generated and CI-gated; Inter + JetBrains Mono adopted after a failed font gate for Geist; Tabler adopted as the one icon language and shipped on web; tokens live in production web CSS and in the desktop chrome and chart ink; six guideline pages written.

The **desktop component layer** lives here as `meridian-egui`: the egui adapter and a capped set of primitives, alongside the geometry and state tokens they consume (ADR 0011). Brightfield's move off GPUI onto egui left no host widget library to defer to, so those primitives live in the design system. With the desktop app on egui, the earlier gpui-component theme emitter has been retired — the crate now emits `tokens.css` for the web, and `meridian-egui` themes the desktop. The token crate's contract is unchanged by any of it.

The **brand assets** live in `meridian-design/brand/` (ADR 0012), which is where consumers should reference the mark from — inlining its path is how the one divergence we have already found got there. Brand motion is capped at three animations and confined to brand surfaces; the apps' no-decorative-motion budget is unchanged. Two of the three are built, each emitted as Lottie for the desktop and as animated SVG for the web, because the two consumers want different things and the site takes no motion runtime: `brand/motion/form.*`, the mark assembling, which doubles as the honest-work cue once a wait passes 100 ms; and `brand/motion/lockup.*`, the animation playing at the top of this file, where one edge crosses the brand left to right, the mark's rows arriving behind it before it writes the word. The third slot was never a third — ADR 0012 reads the work indicator as a cut of `form` rather than a new animation.

The README is one of the brand surfaces ADR 0012 allows, and the hero above is that permission being used rather than a new one: it references the tracked file by path, the way every other consumer must. Its dark twin is served through `prefers-color-scheme`, it stops under `prefers-reduced-motion` and rests on the finished lockup, and `tests/motion.rs` fails if the path it points at stops resolving.

## Licence

MIT for the code — with two carve-outs, both stated in [LICENSE](LICENSE): `meridian-design/brand/` holds Meridian trademarks, all rights reserved, including the generated animations in `brand/motion/` that embed the mark; and `meridian-design/fonts/` holds Inter and JetBrains Mono under the SIL Open Font License 1.1. Neither is covered by the MIT grant. The motion generator in `motion/` is MIT; what it emits into `brand/` is not.
