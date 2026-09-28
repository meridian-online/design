---
status: accepted
date-created: 2026-09-29
date-modified: 2026-09-29
---
# 0013. Square chrome: a rule and a bar do what a corner and a wash did

## Context and Problem Statement

The chrome drew corners of 3, 6 and 8 px on a chip, a control and a panel. Its focus ring sat 1px outside the control, rounded to stay concentric with it. A card that floats cast a soft shadow, 2 down with 8 of blur, and a modal's was 8 down with 24. A row under the cursor was an accent wash with a 1px accent border, and tabs were a segmented capsule or words over a thin rule.

A prototype of the desktop app was built in that look and in a square one and driven in both, and the website was captured page by page in both. The square look was chosen. The system carried nothing that could draw it: every radius was above zero, the ring's geometry assumed it sat outside the control, the shadows were blurred, and there was no token for a row's bar, a tab's bar or a keycap's foot. A consumer that wanted the square look would have had to invent those values, which is the drift ADR 0011 exists to stop.

## Considered Options

- **Keep the rounded look** — corners, an outset ring, soft shadows, an accent wash under the cursor.
- **Square chrome** — no corners; a rule, a bar and a hard shadow carry what the corners and the washes carried.

## Decision Outcome

Chosen: square chrome. ADR 0004 stands — chrome stays quiet, and the accent stays on what the user can act on or has selected. The elevation rule stands too: the working plane casts nothing, and only an overlay and a modal cast a shadow, a modal's more than an overlay's.

| Thing | The square look | Token |
|---|---|---|
| A corner | 0 on a chip, a control and a panel | `radius::{CHIP, CONTROL, PANEL}`, `--m-radius-*` |
| What stays round | A mark that holds no icon and no word: a dot, a ring, a spinner, a slider's thumb | `radius::FULL` |
| The focus ring | 2px in the focus ink, inside the control's edge, square. Nothing is reserved outside the box | `focus::RING_WIDTH`, `RING_OFFSET` and `RING_BLEED` at 0 |
| The ring on a solid fill | 1px in the on-solid ink, 1px in from the edge | `focus::RING_WIDTH_ON_SOLID`, `focus::RING_INSET` |
| A key hint | A keycap: the sunken fill, a hairline on its top and sides, a 2px foot a step darker | `control::KEYCAP_FOOT_WIDTH`; `borders.subtle`, `borders.default_` |
| A row at rest | No fill | — |
| A row under the pointer | The hover fill | `rows.hover_background` |
| A row under the cursor | Grey step 4, and a 3px bar on its leading edge in the focus ink | `rows.cursor_background`, `rows.cursor_bar`, `control::ROW_BAR_WIDTH` |
| Tabs | Words in a row over one rule, a 2px bar under the open one | `tabs.active_bar`, `control::TAB_BAR_WIDTH` |
| A field | The sunken fill over a 1px rule; 2px in the focus ink while it has the keys | `surfaces.sunken`, `borders.focus` |
| A card on the plane | A hairline, no shadow | `Elevation::Raised` |
| A card that floats | A 1px rule and a shadow with no blur, offset 3 by 3; a modal's 6 by 6 and darker | `Elevation::{Overlay, Modal}`, `--m-shadow-*` |

The radius rungs keep their names and read 0. A consumer that reads a rung in its own painter goes square with no edit, and the rung is the slot a later look would fill. The ring is drawn inside the edge, so it lands on the control's own fill rather than on the plane, and `tests/chrome_gate.rs` measures the focus ink against every fill a ring or a bar lands on. On the accent's solid fill the focus ink measures under 3:1, which is why the ring there changes ink, and the gate holds both halves of that. The bars take the focus ink where a caller passes none; a caller may pass its own.

The selection wash (`rows.selected_background`) stays, square, for a toggle that is on and a range that is brushed. It is no longer how the cursor is drawn.

### Consequences

- Good, because a consumer that reads the rungs, the ring and the elevations goes square by taking the new revision, with no edit of its own.
- Good, because a ring drawn inside the edge cannot be clipped by a parent or overlap a neighbour, so a layout reserves nothing for it and there is one ring rather than an outset one and an inset exception.
- Good, because the square look is stated here once, and a consumer draws it from tokens rather than from a copy of a prototype's values.
- Bad, because the website's token block is compared byte for byte with this repository's `main`, so its check fails from the moment this lands until the block is spliced again.
- Bad, because a pill and a tag read alike once both are square; they differ by size and by the pointer.
- Bad, because the keycap's foot is faint: `borders.default_` on the sunken fill measures 1.53:1 in light and 1.92:1 in dark. A key hint takes no input, so the 3:1 floor for a control's boundary does not bind it.
- Bad, because `borders.control` on the fill under the cursor measures 2.86:1 in light, under the 3:1 floor. No control with a drawn boundary sits in a row under the cursor today, so the gate holds text and the focus ink on that fill and not the control boundary; a control placed there reopens this.
- Neutral, because the website's own corners come from its own `--radius`, not from these tokens, and follow in the website's own change.
