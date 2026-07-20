---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-20
---
# 0003. Framework-neutral tokens; thin GPUI adapter; watch Xilem

## Context and Problem Statement

Brightfield's chrome runs on GPUI + gpui-component today, with a foreseen move
to the Linebender stack (chart rendering is already vello/kurbo). Expressing
tokens in gpui-component's theme vocabulary would mean re-translating the
whole system at migration time.

## Considered Options

- **Framework-neutral crate + thin adapters** — plain `Copy` structs; adapters
  at each framework boundary
- **Full gpui-component theme investment** — treat Linebender as a future port
- **Prototype the Masonry adapter now** — assume a near-term move

## Decision Outcome

Chosen option: framework-neutral crate, thin GPUI adapter, revisit at the next
Xilem release (~late 2026). Research (July 2026) found Masonry/Xilem at 0.4.0,
self-described alpha, layout system just replaced, and no tabs/docks/tables
widgets — "port now" is not viable; but its Properties/`DefaultProperties`
styling model confirms the neutral-crate shape maps cleanly to both targets.
GPL urgency is low: gpui and gpui-component are Apache-2.0; GPL enters only
via zed's ztracing/zlog chain, already stubbed and cargo-deny-gated in
Brightfield.

The crate contract: plain `Copy` structs; colours as framework-neutral sRGB
(never `gpui::Hsla`); dimensions as logical-pixel floats; interaction states
(hover/active/disabled) as first-class token slots, not a cascade; typography
carrying OpenType features as `(tag, value)` pairs; **MIT and
dependency-free**, so it sits permanently outside the GPL firewall.

### Update (2026-07-17 — second exit candidate: the rerun/egui stack)

Xilem-only watching risks a hard block (Masonry still ships no
docks/tabs/tables). Researched the stack behind rerun-io/rerun as an
alternative: **egui + eframe + egui_tiles (docking) + egui_table
(virtualised, millions of rows)** — all MIT/Apache-2.0 (the GPL-stub class
of problem disappears entirely), quarterly releases, Rerun-funded, and
rerun itself is the production existence proof of exactly Brightfield's
shape. The vello chart canvas survives byte-identical (shared wgpu
device; native-texture / paint-callback embedding is rerun's own
architecture). egui's `Visuals` is state-slotted plain structs
(noninteractive/inactive/hovered/active/open) — a 1:1 fit for this
crate; an egui adapter is a third thin emitter.

**The historic disqualifier — text — flipped in 2026**: 0.34 replaced
ab_glyph with hinted skrifa rasterisation; 0.35 merged HarfRust GSUB/GPOS
shaping. Remaining gaps, all bounded: (1) **no OpenType feature-tag API**
— `tnum`/`zero`/`liga`-off must be baked into font files
(pyftfeatfreeze; mind OFL Reserved-Font-Name renaming) or contributed
upstream (small unclaimed PR — the shaper already supports features);
(2) the YAML editor pane drops to egui `TextEdit` + custom layouter —
serviceable for a spec pane, below GPUI's editor lineage; (3) the
workspace shell re-expresses in immediate-mode idiom (mitigated: the
keyboard grammar lives in gpui-free `brightfield-keys`, and this token
crate now carries the look).

**Posture: two candidates.** egui/rerun is the nearer-term primary
candidate; Masonry/Xilem remains the watch (~late 2026) — its Parley
text stack is philosophically superior, but egui closed most of the gap
while actually shipping the widgets. Next concrete step when migration
pressure arrives: a spike — egui_tiles + embedded vello canvas + a
feature-frozen Inter table at 11px — gated by the same eyeball
discipline as the font gate.

### Update (2026-07-20 — two clarifications; the watch is retired)

**1. "MIT and dependency-free" scopes the crate, not the repository.** The
contract above binds `meridian-design`, and its own manifest has always said
so — `meridian-design/Cargo.toml`, above an intentionally empty
`[dependencies]`:

> `# Dependency-free by contract (ADR 0003): this crate must remain outside any`
> `# GPL firewall and portable to every consumer (web emission, GPUI, Masonry).`

"This crate", not "this repo". A sibling crate in the same repository may take
dependencies; `meridian-egui` takes `meridian-design` and `egui` (ADR 0011).
The firewall is a property of the token crate's dependency graph, and it is
undisturbed by a neighbour.

**2. The Masonry/Xilem watch is retired.** This ADR held the option open
pending the next Xilem release. Brightfield has since committed to egui and
paid for it: a staged migration, no dual-host in the main tree, and a standing
maintenance tax in the form of a wgpu major-version lock shared with the Vello
canvas. Keeping a port test alive would constrain present design decisions in
order to preserve an option that has already been declined — the worst of both,
since the constraint is paid now and the option is not exercised.

The framework-neutral crate shape is **retained on its own merits** — plain
`Copy` structs, no framework types, colours as sRGB, dimensions as
logical-pixel floats. It is a good shape because it keeps the token data
honest and testable, not because it hedges a port. Retained as a property, not
as a hedge.

Reopening Linebender is therefore a **new decision**, taken on evidence at the
time, not a standing assumption anyone inherits. The port test is replaced by
the **crate-boundary test** of ADR 0011: pure data, or anything the web
`tokens.css` emitter needs, lives in `meridian-design`; anything naming an
`egui` type lives in `meridian-egui`. That test is mechanical, and two live
consumers exercise it continuously.

### Consequences

- Good, because the migration cost of the design system itself becomes ~zero.
- Good, because gpui-component's `ThemeConfig` is plain serde-JSON — the
  "adapter" is mostly an emitter in this repo.
- Bad, because a thin adapter means some gpui-component surfaces keep stock
  styling until the token set grows.
