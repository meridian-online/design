# Chrome

The chrome is square. A rule and a bar do what a corner and a wash did (ADR 0013). Draw each of these from its token; a literal in a consumer is the drift ADR 0011 names.

## Rules

- **Chrome has no corner.** Chips, tags, badges, buttons, fields, rows, panels, cards, popovers and modals read `radius::CHIP`, `CONTROL` and `PANEL` (`--m-radius-*`), and each is 0. A tile that holds an icon, a pill and a toggle's track are square too.
- **Only a mark that holds no icon and no word stays round.** A dot, a ring, a spinner, a slider's thumb: `radius::FULL`.
- **Focus is a 2px rule inside the edge.** In the focus ink (`--m-border-focus`), square, over the control's own fill. Reserve nothing outside the box for it: `--m-focus-ring-bleed` is 0. It lands next frame with no fade (`speed.md`).
- **On a solid fill, the ring changes ink.** 1px in the on-solid ink (`--m-focus-ring-width-on-solid`), 1px in from the edge (`--m-focus-ring-inset`). The focus ink measures under 3:1 on the accent's fill.
- **A key hint is a keycap.** The sunken fill, a hairline in `--m-border-subtle` on its top and sides, and a 2px foot (`--m-keycap-foot-width`) in `--m-border-default`. It keeps the key chip's size.
- **A row has three states.** At rest, no fill. Under the pointer, `--m-rows-hover-bg`. Under the cursor, `--m-rows-cursor-bg` and a 3px bar (`--m-row-bar-width`) on its leading edge in `--m-rows-cursor-bar`. The pointer is a glance; the cursor is where the keys act, and only the cursor has a bar.
- **The selection wash is not the cursor.** `--m-rows-selected-bg` is for a toggle that is on and a range that is brushed, square.
- **Tabs are line tabs.** Words in a row over one rule, and a 2px bar (`--m-tab-bar-width`) in `--m-tabs-active-bar` under the open one.
- **A bar's ink is the focus ink unless the caller passes one.** The row and the tab strip each take the bar's ink from the caller.
- **A field is a sunken fill over a 1px rule.** While it has the keys, the rule is 2px in the focus ink.
- **A card on the plane casts nothing.** A hairline separates it (`elevation.rs`).
- **A card that floats casts a hard shadow.** A 1px rule and a shadow with no blur: an overlay's offset 3 by 3 (`--m-shadow-overlay`), a modal's 6 by 6 and darker (`--m-shadow-modal`). Only overlays and modals cast.

## Evidence

ADR 0013. `tests/chrome_gate.rs` measures the focus ink against every fill a ring or a bar lands on and the on-solid ink against the accent's fill, and fails under 3:1. `tests/conformance.rs` pins the emitted radii, ring, shadows, bar widths and bar inks in both themes.
