# Component review: twelve components for `meridian-egui`, a relaxed corner, and an inventory in place of the cap

Date: 2026-11-22. Status: for approval.

## Intent

Brightfield's chrome is about to be assembled: a ribbon, a step list, an inspector, open places, a title band. The owner ruled that the design system's components are built first and the chrome is assembled from them, because the last time an application drew its own chrome it grew a private palette, five focus treatments and four copies of one modal (ADR 0011).

The outcomes the owner expects are three. The interface is consistent, because each shape is drawn by one function. It looks better, because the look of each component is designed once, with care, and applied everywhere. And the design can change cheaply, because a change is one edit here and a version bump in the consumer.

The second outcome does not follow from having components. It follows from deciding how each one looks. That was done on the component sheet, `components/sheet.html`, where every component is drawn in every state in both themes and the owner ruled on each look. **The sheet is this design's visual reference. Where this document and the sheet disagree about a look, the sheet is right.**

## Goals

- Twelve new components in `meridian-egui`, each matching its drawing on the sheet in every listed state, in light and dark.
- Four pieces composed from those and from existing primitives, exported so Brightfield never draws them itself.
- The tokens those components need, defined in `meridian-design` and emitted in `tokens.css`.
- Corners relaxed from square to a small radius, across new and existing components alike.
- ADR 0011's numeric cap replaced by an inventory that a test holds.
- The 27 icons the chrome names, added to the Tabler manifest.

## Non-goals

- **Adoption in Brightfield.** Replacing its hand-built buttons and text inputs, and a gate that refuses a direct `egui::Button` in the shell, are Brightfield's work and are tracked there.
- **Patterns with no surface asking for them yet.** Stat tiles, an object header and a hover card were in the mockups that prompted this review. They enter by ADR 0011's doors when a surface needs them.
- **Controls with too few call sites.** Checkbox, radio, combo box and date picker stay egui's own, themed by `theme::apply`.
- **The data grid.** It is Brightfield's canvas and stays there.
- **Web components.** Web keeps shadcn on Base UI and takes token values only (ADR 0011).
- **The modal layer's sheet width and banners with action buttons**, which are asked for separately. The popover here answers only the request for a layer anchored to a rect.

## Anti-goals

These would make the work a failure even if every test passed.

- A component whose look was guessed by the builder and not taken from the sheet.
- A component named for where Brightfield puts it. `activity_switcher` is wrong; `segmented` is right.
- Application structure moving into this repo. Each component draws what it is handed. Which steps, which places, which commands and every word stay Brightfield's.
- A look parameter. No component takes a colour, a width, a height or a radius. The one exception is a bar's ink, which `line_tabs` and `list_row` already take.
- A literal where a token belongs.

## Constraints

- `meridian-design` stays MIT and dependency-free (ADR 0003). Anything that is pure data lives there; anything that names an `egui` type lives in `meridian-egui` (ADR 0011's crate-boundary test).
- No code, enum bodies or token values are taken from Zed's GPL crates (ADR 0011). Blueprint and MotherDuck were read for measurements and shapes only.
- A token change regenerates `tests/snapshots/tokens.css` and `reference/tokens.md` in the same commit.
- `meridian-egui`'s tests read the paint list, not pictures (`tests/square_look.rs` is the model). The crate keeps no image goldens. A new component's tests assert what it painted: rects, fills, strokes, corners, text.
- No motion. Every state appears on the next frame (`guidelines/speed.md`).
- This repo is public. Nothing here names a private planning identifier.

## Approaches

Three ways to treat ADR 0011's cap of 15 were considered. The crate holds about thirteen primitives today and this work adds twelve.

1. **Keep 15 and build only the general shapes here**, leaving the ribbon, the step row and the rest to Brightfield. Rejected: it contradicts the ruling that components come first, and it recreates the conditions ADR 0011 was written to end.
2. **Raise the figure to 25 and change nothing else.** A small amendment. Rejected: the figure was a forecast made in July, before the chrome existed, and a figure revised the first time a real requirement meets it limits nothing. It also leaves the product-requirement door as loose as it is.
3. **Drop the figure, keep both doors, tighten the second, and add an inventory a test holds.** Chosen by the owner.

The reasoning for the third: ADR 0011's own claim is that a gallery arrives one reasonable addition at a time, and that only a rule which can refuse a proposal stops it. The two doors already refuse a speculative component. What the figure added was a limit on the product-requirement door, through which one call site is enough. The replacement limit is that a component admitted by that door is a general shape under a general name, and that every addition is a visible row in an inventory a reviewer can challenge.

## Decisions

### 1. ADR 0011 is amended in place

An `### Update (2026-11-22 — the cap becomes an inventory)` section, with `date-modified` bumped, saying:

- **Narrows:** "Cap: 15 primitives" is removed.
- **Stands:** the two doors (three independent call sites, or a named product requirement that provably cannot be composed), and the rule that a third variant parameter means the component is inlined back and deleted in the same pull request.
- **Added:** a component admitted by a product requirement is a general shape, named for what it is.
- **Added:** `meridian-egui/INVENTORY.md` lists every component and every composed piece, one row each, with its door and its call sites or its requirement. A test fails when the crate's public surface and the inventory disagree in either direction.

`CLAUDE.md` and `README.md` say "capped at 15" in several places and are corrected in the same change.

### 2. Corners: ADR 0013 is amended in place

The owner's ruling: "I made an earlier ruling that we need to be completely at right angles on all elements. As I look more closely at Quiver I think we can relax this somewhat." The amount was ruled from the sheet's corner switch, with measurements from MotherDuck's app, whose small controls round by 2px and everything else by 3px.

| Token | Was | Is | Rounds |
|---|---|---|---|
| `radius::CHIP` | 0 | 2 | keycap, status pill, timeline bar, a tab's close control |
| `radius::CONTROL` | 0 | 3 | button, text field, segmented control, focus ring |
| `radius::PANEL` | 0 | 3 | popover, menu, modal card, banner, toast, an inspector or step list as a card |
| `radius::FULL` | 9999 | 9999 | a dot, a ring, a spinner, a slider's thumb |

An `### Update (2026-11-22 — a small corner returns)` section on ADR 0013 says:

- **Narrows:** "no corners". Chips, controls and cards take the radii above. These are smaller than the 3, 6 and 8 the ADR retired.
- **Stands:** everything else the ADR decided. The focus ring is a 2px rule inside the control's edge and follows the control's corner. Overlays cast a hard shadow with no blur. A cursor is a fill and a bar. Tabs are line tabs.
- **Stays square:** rows and their hover, cursor and selection fills; every bar (a line tab's, a cursor's, a progress bar along a foot); every hairline; data marks, including the distribution bar's bins; bands that run edge to edge.
- **Joined cells round only their outer corners**: the ribbon's open tab at the top, a vertical tab's current place on the side away from the canvas. A text field rounds its top corners and keeps its rule straight.

Consequences inside this repo: `radius.rs`'s test that every radius is 0 is rewritten to pin the three values; `tests/square_look.rs` is rewritten so that each primitive is asserted to draw its token's corner, and rows and bars are asserted square; `guidelines/chrome.md` replaces "Chrome has no corner" with what rounds and what does not; `theme::apply` passes the control and panel radii to egui's own widgets, windows and menus.

Consequences outside: `tokens.css` changes three values, so web's pinned block changes at its next bump, and Brightfield's gallery goldens change at its bump.

### 3. The keycap has two sizes

The keycap is 21 tall. A 24 control draws its focus ring 2px inside its edge and leaves 20, so the ring ran through the cap. The owner ruled for a smaller cap inside a control.

- **In a row** (list row, menu item, picker row, a status band): 21, as `key_chip` draws today.
- **Inside a control** (button, segmented option, ribbon command): 16 tall, placed 4 (`SPACE_2`) from the control's top, bottom and trailing edge alike. That clears the ring by 2 on all three sides and lands on whole pixels.

Both keep the keycap's look: the sunken face, the hairline on three sides, the 2px foot. `guidelines/chrome.md`'s "It keeps the key chip's size" is rewritten to name the two sizes and where each is used. A consequence: a control that carries a keycap is at least 24 tall, and so is any band that holds a keycap in a row.

### 4. The components

Each follows the crate's existing call shape: a free function `fn widget(ui: &mut egui::Ui, config: Config<'_>) -> WidgetResponse`, where `Config` is built with `new(..)` and `#[must_use]` builder methods and the response carries `response: egui::Response` and what was acted on. A floating layer is a unit struct with `show(ctx, id_salt, …)`, as `ModalLayer` is. Text is the caller's: a keystroke is a caller string such as `"⌘↩"`, a reason is a caller sentence. An unavailable part is passed its reason, keeps its place, senses hover, reports no click, and reports its rect so the caller can anchor a popover.

Every interactive part publishes an accesskit node by role and label, and an unavailable part is disabled with its reason as the node's description. Each component reports the rects of its parts.

Section numbers are the sheet's.

| § | Component | Draws | States | Door |
|---|---|---|---|---|
| 1 | `button` | An optional icon, a label, an optional keycap. Three weights: default (sunken fill in a rule), primary (accent fill), quiet (no fill or rule until pointed at). 24 tall; 20 without a keycap; icon-only is square. | rest, hover, pressed, keyboard focus, unavailable | call sites |
| 2 | `text_field` | A sunken fill over a 1px rule, with an optional leading icon and unit. Wraps egui's `TextEdit`, which keeps text editing, selection and input methods. | rest, hover, has the keys (2px rule in the focus ink), error (2px rule in the danger ink), unavailable | call sites |
| 3 | `ribbon_tabs` | A row of tabs, then an optional contextual tab set apart by a divider and the Maritime wash. The open tab is a cell joined to the row beneath. | per tab: rest, hover, pressed, open, selected while collapsed, keyboard focus; key tips on or off | requirement |
| 4 | `ribbon_row` | Quiet buttons in groups divided by a rule, in a 32 band with contents at the 24 rung. When narrower than its contents, labels are dropped from the right until it fits; the icon and the key remain. | per command: button's states, plus icon-only and key tip shown | requirement |
| 5 | `Popover` | An overlay card anchored below a rect, or above when there is no room below, holding any body. The overlay shadow, a 1px rule, no arrow. A reason form holds one sentence and an optional keycap at a fixed width of 240. | open; dismissed by Esc or a click outside; the caller holds whether it is open | requirement |
| 6 | `segmented` | Mutually exclusive options in one ruled box, each with an optional keycap. The selected option takes the selection wash. | per option: rest, hover, pressed, selected, keyboard focus, unavailable | requirement |
| 7 | `section` | A header row (disclosure, a small capital label, an optional count at the right) over a hairline, and a body drawn only while open. | header: rest, hover, keyboard focus; open or closed | requirement |
| 8 | `distribution_bar` | Bars in chart ink: equal bins low to high, or a category's top values with the others as one bar; then the missing count in the null ink. 36 tall, or 20 for a column header. | rest; one bar hovered and reported | requirement |
| 9 | `menu_item` | A list row with a check column, a label, a right-aligned 21 keycap, and a chevron for a submenu. | rest, hover, pressed, cursor, checked, unavailable, unavailable with the cursor | requirement |
| 10 | `step_row` | A graph gutter with the state mark on its lane, an optional fold control with a member count, a name, what it produced, and for a step a timeline bar and a duration. | rest, hover, pressed, focused (selection wash), cursor (fill and bar), both | requirement |
| 11 | `vertical_tab` | A row naming a node and an activity, a dot while it holds an unsaved draft, a close control shown under the pointer and on the current place. The current place is a cell joined to the canvas. | rest, hover, current, current and hovered, close hovered, keyboard focus | requirement |
| 12 | `run_status` | Running: the mark, the step, "n of m", and a 2px progress bar along its foot. Ran: the mark, when, how long ago. Not run: a quiet line. Clickable. | three cases; hover, keyboard focus | requirement |

Composed pieces, exported and listed in the inventory as compositions:

- `key_tip(ui, target, keystroke)`: a 21 keycap at its target's trailing bottom corner, on a layer above, casting the overlay shadow. `ribbon_tabs` and `ribbon_row` draw it themselves when key tips are on, and a command's own keycap gives way to it.
- `step_mark(ui, mark, focused)`: the state mark alone in a 24 cell, for the step list's thin form.
- `teaching_message(ui, command, keystroke)`: `<command> · next time, press [key]`, the one piece of copy a component owns.
- `PickerRow::unavailable(reason)`: the menu item's unavailable treatment on the existing picker row, with the reason as its detail. The row still takes the cursor.

Signatures. Brightfield's chrome plan proposed a signature for each of these under its own names. The landed signatures keep those shapes under the names above, with three differences the builder must apply: `ribbon_row` takes groups of the same `Button` config that `button` takes, not a separate command type; the reason popover and the anchored layer are one `Popover` with `show` for any body and `show_reason` for a sentence and keystroke; `step_row`'s gutter carries the mark, so there is no separate mark column. Where a signature detail is not fixed here, the builder follows the nearest existing primitive.

### 5. The state mark is data

`meridian_design::marks` holds `StepMark { NotRun, Running, Ok, Failed, Stale }` and `glyph(mark, dark) -> MarkGlyph { shape, ink, fill }`. `step_row`, `step_mark`, `run_status` and Brightfield's graph renderer all read it.

| State | Shape | Ink |
|---|---|---|
| not run | a ring | the control border |
| running | a ring, its right half filled | the focus ink |
| ok | a disc | status good |
| failed | a diamond | status critical |
| stale | a ring with a centre | status warning |

The mark is 7 across with a 1.5 stroke, inside a 14 box. Shape tells the states apart without colour.

### 6. New tokens

All in `meridian-design`, emitted in `tokens.css`. Names are the builder's to settle within each module's existing convention.

| Value | For |
|---|---|
| 2, 3, 3 | the three radii, above |
| 16 | the keycap's height inside a control |
| 32 | a band height, `ROW_GRID + 2 × SPACE_2`, for the title band and the ribbon row |
| 240 | the popover's reason width |
| 12 | one lane of the step row's gutter |
| 24 | the thin form's mark cell |
| 72, 40 | the step row's timeline and duration columns |
| 12 | the timeline bar's height in a 24 row |
| 6 | the unsaved dot |
| capitals, 0.06em tracking, 11px | the section label, as a style in the type ramp |

### 7. Icons

Added to `meridian-egui/src/icons/tabler.manifest` with the existing generator: `adjustments`, `axis-x`, `axis-y`, `palette`, `zoom-in`, `zoom-out`, `arrow-back-up`, `list`, `schema`, `hierarchy`, `link`, `unlink`, `key`, `fold`, `trash`, `layout-columns`, `eye-off`, `cursor-text`, `math-function`, `transform`, `bucket-droplet`, `copy-off`, `list-numbers`, `sum`, `arrows-join`, `row-insert-bottom`, `file-import`. All 27 exist in Tabler's outline set.

### 8. The sheet stays, and is kept true

`components/sheet.html` reads `tokens.css` by relative path. It overrides the three radius tokens on the page so the ruled corners can be seen before the tokens change. Once the tokens change, the override and the switch are removed and the page reads the tokens alone. Its list of "values with no token yet" is emptied as each token lands. Whether the sheet joins the published evidence site is left open; publishing it publishes what it reaches for.

## Calls made on the owner's behalf

Shown on the sheet and not objected to, but not ruled on individually:

- The contextual ribbon tab is tinted with the Maritime wash, though Maritime otherwise means interaction.
- A key tip sits at its target's trailing bottom corner and partly covers what is beside it.
- The state mark sits on its lane in the gutter. MotherDuck's Flight Explainer, measured live, draws it the same way at the same size.
- State marks carry an ink per state as well as a shape. Flight Explainer uses one grey.
- A key inside a control is 16 tall, where the candidate first shown was 17; 16 is what makes the inset even.
- Component names: `segmented`, `section`, `step_row`, `vertical_tab`, `run_status`, `Popover`, `menu_item`.

## Left to the builder

- Module layout and file names in `meridian-egui`.
- Token names, within each module's convention.
- How the inventory test reads the crate's public surface.
- The exact paint of each state where the sheet's CSS and egui's painter differ in mechanism. The result must match the sheet.
- The order of work, subject to these dependencies: tokens and the two ADR amendments first; `button` before `ribbon_row` and `segmented`; `Popover` before `menu_item`'s submenu use; `marks` before `step_row` and `run_status`.

## How it is verified

- `cargo test` in `meridian-design` and `meridian-egui`. Each component has paint-list tests for every state in the table above, in both modes, in the style of `tests/square_look.rs`, and an accesskit test by role and label.
- The inventory test.
- `cargo run --example gallery -p meridian-egui` shows every component beside the sheet for a person to compare. The owner compares them before the branch is called ready.
- The conformance snapshots for `tokens.css` and `reference/tokens.md` are regenerated and pass.
