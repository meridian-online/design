# Chrome

The chrome takes a small corner, and rules and bars still do the work of a wash (ADR 0013). Draw each of these from its token; a literal in a consumer is the drift ADR 0011 names.

## Rules

- **Three radii, and what takes each.** `radius::CHIP` (2, `--m-radius-chip`): a keycap, a status pill, a timeline bar, a tab's close control. `radius::CONTROL` (3): a button, a text field, a segmented control, and the focus ring that follows them. `radius::PANEL` (3): a popover, a menu, a modal card, a banner, a toast, an inspector or step list drawn as a card.
- **Rows, bars and bands stay square.** A row and its hover, cursor and selection fills, every bar, every hairline, data marks and a band that runs edge to edge read `radius::NONE` (`--m-radius-none`), never a rounding rung. Joined cells round only their outer corners, and a text field rounds its top corners and keeps its rule straight.
- **Only a mark that holds no icon and no word stays round.** A dot, a ring, a spinner, a slider's thumb: `radius::FULL`.
- **Focus is a 2px rule inside the edge.** In the focus ink (`--m-border-focus`), following the control's corner, over the control's own fill. Reserve nothing outside the box for it: `--m-focus-ring-bleed` is 0. It lands next frame with no fade (`speed.md`).
- **On a solid fill, the ring changes ink.** 1px in the on-solid ink (`--m-focus-ring-width-on-solid`), 1px in from the edge (`--m-focus-ring-inset`). The focus ink measures under 3:1 on the accent's fill.
- **A key hint is a keycap.** The sunken fill, a hairline in `--m-border-subtle` on its top and sides, and a 2px foot (`--m-keycap-foot-width`) in `--m-border-default`, rounded at its bottom corners with the box. It comes in two sizes, a rung and not a height. In a row, a menu item, a picker row or a status band it is 21 tall (`--m-keycap-height`). Inside a control (a button, a segmented option, a ribbon command) it is 16 tall (`--m-keycap-height-in-control`), placed 4 from the control's top, bottom and trailing edge, which clears the 2px-inset focus ring; a control that carries a key is therefore at least 24 tall. A wide keystroke widens the key and never its height. A key for an unavailable command takes the disabled text ink and keeps its box. On a solid accent fill it has no face: the rule and the foot are `--m-keycap-on-solid-rule` and `--m-keycap-on-solid-foot`, and the keystroke is in the on-solid text ink.
- **A row has three states.** At rest, no fill. Under the pointer, `--m-rows-hover-bg`. Under the cursor, `--m-rows-cursor-bg` and a 3px bar (`--m-row-bar-width`) on its leading edge in `--m-rows-cursor-bar`. The pointer is a glance; the cursor is where the keys act, and only the cursor has a bar.
- **The selection wash is not the cursor.** `--m-rows-selected-bg` is for a toggle that is on and a range that is brushed, and it is square.
- **Tabs are line tabs.** Words in a row over one rule, and a 2px bar (`--m-tab-bar-width`) in `--m-tabs-active-bar` under the open one.
- **A bar's ink is the focus ink unless the caller passes one.** The row and the tab strip each take the bar's ink from the caller.
- **A field is a sunken fill over a 1px rule.** While it has the keys, the rule is 2px in the focus ink.
- **A card on the plane casts nothing.** A hairline separates it (`elevation.rs`).
- **A card that floats casts a hard shadow.** A 1px rule and a shadow with no blur: an overlay's offset 3 by 3 (`--m-shadow-overlay`), a modal's 6 by 6 and darker (`--m-shadow-modal`). Only overlays and modals cast.

## Evidence

ADR 0013. `tests/chrome_gate.rs` measures the focus ink against every fill a ring or a bar lands on and the on-solid ink against the accent's fill, and fails under 3:1. `tests/conformance.rs` pins the emitted radii, ring, shadows, bar widths and bar inks in both themes.
