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

- **Defined:** a variant parameter is an argument that selects among looks of one component: a weight, a rung. Content (an icon, a keycap, a count) and state (unavailable, selected, open) are not variants. By that reading `button` has two, its weight and its rung, and no component here has three.

The inventory has three kinds of row, so that the test can cover the crate's whole public surface:

- **component**: its door, and its call sites or its requirement in plain words. The twelve here name the part of Brightfield's chrome that requires them. The primitives that exist today are entered as admitted under ADR 0011 before the inventory, each with the Brightfield surfaces that call it.
- **composition**: what it is composed from.
- **helper**: a public function that draws no component of its own, such as `focus_ring`, `slider_colours`, `align_scope`, `tooltip_for_action`, `optically_centred_galley_top` and `overlay_frame`.

The figure also appears in `CLAUDE.md` ("Capped at 15") and `ROADMAP.md`, which are corrected in the same change. ADR 0002's update mentions "a capped set of ~15 primitives"; it is left as written, and ADR 0011's update is what governs.

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

Of the primitives that exist today, these take a corner: the key chip and the status pill (chip), the modal card, the banner and the toast (panel), and egui's own buttons, fields, windows and menus through `theme::apply`. These stay square: the list row, line tabs, the pane header, and the query line, which is a band running edge to edge inside the picker's card.

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

The signatures are in the appendix. Points the table leaves open are settled as follows.

- **A button's rung.** 24 unless the caller asks for the dense rung with `.dense()`, which is 20. A dense button draws no keycap; passing one is a debug assertion. A rung is a ladder step, as `list_row`'s `RowHeight` is, not a height parameter.
- **Widths.** A button is as wide as its contents. A text field, a section, a step row, a menu item and a vertical tab fill the width they are given. The popover's reason form is 240; its general form is as wide as its body.
- **A text field's label and message.** `text_field` draws the field and, beneath it, one message line: help in the muted ink, or the error in the danger text ink when one is passed. The label is laid out by the caller's form, in the left column `align_scope` already provides. The existing `field_row` is a read-only label and value and is not changed.
- **Menus.** A menu is a `Popover` whose body is `menu_item`s. `menu_separator(ui)` draws the hairline between groups and is listed as a composition. `Popover::show_menu` gives the body the menu's width of 240 and its vertical padding. A submenu's item draws its chevron and reports its rect and that it was chosen; the caller opens a second `Popover` with `Anchor::Beside`, so `Popover` anchors below, above or beside a rect.
- **Who holds state.** The caller holds everything: which section is open, which tab is selected, where the cursor is, whether a popover is open. A component reports what was clicked and holds nothing between frames.
- **Unavailable controls and the keyboard.** An unavailable button, segmented option or ribbon command is disabled: it takes no keyboard focus, and its accesskit node is disabled with the reason as its description. Its reason reaches a keyboard user because Brightfield shows the reason popover for a few seconds when the command's key is pressed. An unavailable menu item or picker row does take the cursor, since a list is walked with the keys.
- **Tokens beyond section 6.** The builder adds a token for any value on the sheet that has none, rather than a literal. The on-solid keycap's two rule inks are new colours: they become semantic tokens and take an entry in `tests/chrome_gate.rs`.

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
- What ADR 0011's "variant parameter" means, in decision 1.
- A text field draws its own message line and leaves its label to the caller's form.
- A menu is a popover of menu items with a separator; there is no menu component.
- The query line stays square.

## Left to the builder

- Module layout and file names in `meridian-egui`.
- Token names, within each module's convention.
- How the inventory test reads the crate's public surface.
- The exact paint of each state where the sheet's CSS and egui's painter differ in mechanism. The result must match the sheet.
- The order of work, subject to these dependencies: tokens and the two ADR amendments first; `button` before `ribbon_row` and `segmented`; `Popover` before `menu_item`'s submenu use; `marks` before `step_row` and `run_status`.

## How it is verified

- `cargo test` in `meridian-design` and `meridian-egui`. Each component has paint-list tests for every state in the table above, in both modes, in the style of `tests/square_look.rs`, and an accesskit test by role and label.
- The inventory test.
- `cargo run --example gallery -p meridian-egui` shows every component in every state, laid out in the sheet's order, for a person to compare with the sheet. The example is a stub today that shows only egui's own controls, so this is new work and is planned as such. The owner compares the two before the branch is called ready.
- The conformance snapshots for `tokens.css` and `reference/tokens.md` are regenerated and pass.

## Appendix: signatures

These keep the shapes Brightfield's chrome plan proposed, under the names above. Lifetimes and derives follow the crate's existing primitives. `Icon` is `meridian_egui::Icon`.

```rust
// 1. button
pub enum Weight { Default, Primary, Quiet }
pub struct Button<'a> { /* icon, label, keystroke, reason, weight, dense */ }
impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self;
    pub fn icon_only(icon: &'a Icon, label: &'a str) -> Self;   // the label names the accesskit node
    pub fn icon(self, icon: &'a Icon) -> Self;
    pub fn keystroke(self, keystroke: &'a str) -> Self;
    pub fn weight(self, weight: Weight) -> Self;
    pub fn dense(self) -> Self;
    pub fn unavailable(self, reason: &'a str) -> Self;
}
pub struct ButtonResponse { pub response: egui::Response, pub clicked: bool, pub pointed_unavailable: Option<egui::Rect> }
pub fn button(ui: &mut egui::Ui, button: Button<'_>) -> ButtonResponse;

// 2. text field
pub struct TextField<'a> { /* text, placeholder, icon, unit, error, help, dense, unavailable */ }
impl<'a> TextField<'a> {
    pub fn new(text: &'a mut String) -> Self;
    pub fn placeholder(self, text: &'a str) -> Self;
    pub fn icon(self, icon: &'a Icon) -> Self;
    pub fn unit(self, unit: &'a str) -> Self;
    pub fn help(self, text: &'a str) -> Self;
    pub fn error(self, text: &'a str) -> Self;      // replaces help
    pub fn dense(self) -> Self;
    pub fn unavailable(self) -> Self;
}
pub struct TextFieldResponse { pub response: egui::Response, pub changed: bool, pub submitted: bool }
pub fn text_field(ui: &mut egui::Ui, field: TextField<'_>) -> TextFieldResponse;

// 3. ribbon tabs
pub struct RibbonTab<'a> { /* label, keystroke */ }
impl<'a> RibbonTab<'a> { pub fn new(label: &'a str) -> Self; pub fn keystroke(self, keystroke: &'a str) -> Self; }
pub enum TabPick { Stage(usize), Contextual }
pub struct RibbonTabs<'a> { /* stages, contextual, selected, collapsed, key_tips */ }
impl<'a> RibbonTabs<'a> {
    pub fn new(stages: &'a [RibbonTab<'a>], selected: Option<TabPick>) -> Self;
    pub fn contextual(self, tab: RibbonTab<'a>) -> Self;
    pub fn collapsed(self, collapsed: bool) -> Self;
    pub fn key_tips(self, shown: bool) -> Self;
}
pub struct RibbonTabsResponse {
    pub response: egui::Response,
    pub clicked: Option<TabPick>,          // any tab, the selected one included
    pub double_clicked: Option<TabPick>,
    pub rects: Vec<(TabPick, egui::Rect)>,
}
pub fn ribbon_tabs(ui: &mut egui::Ui, tabs: RibbonTabs<'_>) -> RibbonTabsResponse;

// 4. ribbon row: groups of buttons, drawn in the quiet weight whatever weight they carry
pub struct CommandAt { pub group: usize, pub index: usize }
pub struct RibbonRow<'a> { /* groups, key_tips */ }
impl<'a> RibbonRow<'a> { pub fn new(groups: &'a [&'a [Button<'a>]]) -> Self; pub fn key_tips(self, shown: bool) -> Self; }
pub struct RibbonRowResponse {
    pub response: egui::Response,
    pub clicked: Option<CommandAt>,                           // available commands only
    pub pointed_unavailable: Option<(CommandAt, egui::Rect)>,
    pub rects: Vec<(CommandAt, egui::Rect)>,
    pub icon_only: Vec<CommandAt>,                            // the commands whose label was dropped
}
pub fn ribbon_row(ui: &mut egui::Ui, row: RibbonRow<'_>) -> RibbonRowResponse;

// 5. popover
pub enum Anchor { Below, Beside }     // Below falls back to above, Beside to the other side, when there is no room
pub struct PopoverResponse<R> { pub inner: R, pub dismissed: bool }
pub struct Popover;
impl Popover {
    pub fn show<R>(ctx: &egui::Context, id_salt: impl Hash + Debug, anchor: egui::Rect, side: Anchor,
                   body: impl FnOnce(&mut egui::Ui) -> R) -> PopoverResponse<R>;
    pub fn show_menu<R>(ctx: &egui::Context, id_salt: impl Hash + Debug, anchor: egui::Rect, side: Anchor,
                        body: impl FnOnce(&mut egui::Ui) -> R) -> PopoverResponse<R>;
    pub fn show_reason(ctx: &egui::Context, id_salt: impl Hash + Debug, anchor: egui::Rect,
                       reason: &str, keystroke: Option<&str>) -> egui::Response;    // not dismissable; the caller decides when it shows
}

// 6. segmented control
pub struct SegmentedOption<'a> { /* label, keystroke, reason */ }
impl<'a> SegmentedOption<'a> {
    pub fn new(label: &'a str) -> Self;
    pub fn keystroke(self, keystroke: &'a str) -> Self;
    pub fn unavailable(self, reason: &'a str) -> Self;
}
pub struct Segmented<'a> { /* options, selected */ }
impl<'a> Segmented<'a> { pub fn new(options: &'a [SegmentedOption<'a>], selected: usize) -> Self; }
pub struct SegmentedResponse { pub response: egui::Response, pub clicked: Option<usize>, pub pointed_unavailable: Option<(usize, egui::Rect)> }
pub fn segmented(ui: &mut egui::Ui, control: Segmented<'_>) -> SegmentedResponse;   // 24 when any option has a keystroke, else 20

// 7. section
pub struct Section<'a> { /* title, count, open */ }
impl<'a> Section<'a> { pub fn new(title: &'a str, open: bool) -> Self; pub fn count(self, count: usize) -> Self; }
pub struct SectionResponse<R> { pub response: egui::Response, pub toggled: bool, pub inner: Option<R> }
pub fn section<R>(ui: &mut egui::Ui, section: Section<'_>, body: impl FnOnce(&mut egui::Ui) -> R) -> SectionResponse<R>;

// 8. distribution bar
pub enum Bars<'a> { Bins(&'a [u64]), Top { values: &'a [(&'a str, u64)], rest: u64 } }
pub struct DistributionBar<'a> { /* bars, missing, total, dense */ }
impl<'a> DistributionBar<'a> { pub fn new(bars: Bars<'a>, missing: u64, total: u64) -> Self; pub fn dense(self) -> Self; }
pub struct DistributionResponse { pub response: egui::Response, pub hovered: Option<usize> }   // rest and missing are the last two indices
pub fn distribution_bar(ui: &mut egui::Ui, bar: DistributionBar<'_>) -> DistributionResponse;

// 9. menu item
pub struct MenuItem<'a> { /* label, keystroke, reason, checked, submenu, cursor */ }
impl<'a> MenuItem<'a> {
    pub fn new(label: &'a str) -> Self;
    pub fn keystroke(self, keystroke: &'a str) -> Self;
    pub fn unavailable(self, reason: &'a str) -> Self;
    pub fn checked(self, on: bool) -> Self;      // any call reserves the check column
    pub fn submenu(self) -> Self;
    pub fn cursor(self, cursor: bool) -> Self;
}
pub struct MenuItemResponse { pub response: egui::Response, pub clicked: bool, pub pointed_unavailable: Option<egui::Rect> }
pub fn menu_item(ui: &mut egui::Ui, item: MenuItem<'_>) -> MenuItemResponse;
pub fn menu_separator(ui: &mut egui::Ui);

// 10. step row
pub use meridian_design::marks::StepMark;
pub struct Gutter<'a> {
    pub lanes: u8,               // how many lanes the list's widest row uses, so every row's gutter is one width
    pub node: Option<u8>,        // the lane this row's mark sits on; None for a row with no node
    pub through: &'a [u8],       // lanes passing this row without stopping
    pub from_above: &'a [u8],    // lanes arriving at this node
    pub to_below: &'a [u8],      // lanes leaving this node
}
pub struct Timeline { pub start: f32, pub end: f32 }     // 0.0..=1.0 of the run's span
pub enum Fold { Folded { members: usize }, Unfolded }
pub enum StepEntry { Step, Table }
pub struct StepRow<'a> { /* every field above */ }
impl<'a> StepRow<'a> {
    pub fn new(entry: StepEntry, gutter: Gutter<'a>, mark: StepMark, label: &'a str) -> Self;
    pub fn output(self, text: &'a str) -> Self;
    pub fn timeline(self, timeline: Timeline, duration: &'a str) -> Self;
    pub fn fold(self, fold: Fold) -> Self;
    pub fn focused(self, focused: bool) -> Self;
    pub fn cursor(self, cursor: bool) -> Self;
}
pub struct StepRowResponse { pub response: egui::Response, pub clicked: bool, pub fold_clicked: bool }
pub fn step_row(ui: &mut egui::Ui, row: StepRow<'_>) -> StepRowResponse;
pub fn step_mark(ui: &mut egui::Ui, mark: StepMark, focused: bool) -> egui::Response;

// 11. vertical tab
pub struct VerticalTab<'a> { /* node, activity, current, dirty */ }
impl<'a> VerticalTab<'a> {
    pub fn new(node: &'a str, activity: &'a str) -> Self;
    pub fn current(self, current: bool) -> Self;
    pub fn dirty(self, dirty: bool) -> Self;
}
pub struct VerticalTabResponse { pub response: egui::Response, pub clicked: bool, pub close_clicked: bool }
pub fn vertical_tab(ui: &mut egui::Ui, tab: VerticalTab<'_>) -> VerticalTabResponse;

// 12. run status
pub enum RunStatus<'a> {
    Running { step: Option<&'a str>, done: usize, total: usize },
    Ran { when: &'a str, age: &'a str, mark: StepMark },      // Ok, Stale or Failed
    NotRun { label: &'a str },                                // the caller's words, such as "Not run yet"
}
pub fn run_status(ui: &mut egui::Ui, status: RunStatus<'_>) -> egui::Response;

// compositions
pub fn key_tip(ui: &egui::Ui, target: egui::Rect, keystroke: &str);
pub fn teaching_message(ui: &mut egui::Ui, command: &str, keystroke: &str) -> egui::Response;
impl PickerRow { pub fn unavailable(self, reason: impl Into<String>) -> Self; }   // and a public `unavailable: Option<String>` field
```

Accesskit roles: `button` is `Button`; `text_field` is `TextInput`; `ribbon_tabs` is a `TabList` of `Tab`s with the open one selected; `ribbon_row` is a `Toolbar`; the reason popover is a `Tooltip`; `segmented` is a `RadioGroup` of `RadioButton`s; `section`'s header is a `Button` with its expanded state; `menu_item` is a `MenuItem`; `step_row` is a `Row` labelled with its name and state word; `vertical_tab` is a `Tab` in a vertical `TabList`; `run_status` is a `Status` with a `ProgressBar` child while running; `key_tip` is a `Label`; `teaching_message` is a `Status` labelled with the whole sentence.
