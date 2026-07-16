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

### Consequences

- Good, because the migration cost of the design system itself becomes ~zero.
- Good, because gpui-component's `ThemeConfig` is plain serde-JSON — the
  "adapter" is mostly an emitter in this repo.
- Bad, because a thin adapter means some gpui-component surfaces keep stock
  styling until the token set grows.
