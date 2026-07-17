---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
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

### Consequences

- Good, because the migration cost of the design system itself becomes ~zero.
- Good, because gpui-component's `ThemeConfig` is plain serde-JSON — the
  "adapter" is mostly an emitter in this repo.
- Bad, because a thin adapter means some gpui-component surfaces keep stock
  styling until the token set grows.
