# Component review Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add twelve components and four compositions to `meridian-egui`, relax the chrome's corners to 2, 3 and 3, and replace ADR 0011's cap with an inventory a test holds.

**Architecture:** Values are plain data in `meridian-design` and are emitted in `tokens.css`; anything that names an `egui` type is in `meridian-egui`. Each component is one module with a free function, a builder config and a response struct, and is tested by reading what a frame painted. `components/sheet.html` is the visual reference for every look.

**Tech Stack:** Rust 1.95.0, egui 0.35, kurbo 0.13, egui_kittest for headless frames and accesskit queries.

**Spec:** `docs/superpowers/specs/2026-11-22-component-review-design.md`. Its appendix holds every signature; this plan cites them as "spec §n" and does not repeat them. Read the spec and open `components/sheet.html` in a browser before starting any task.

## Global Constraints

- `meridian-design` keeps an empty `[dependencies]`. `meridian-egui` depends on `meridian-design`, `egui` and `kurbo` only.
- No component takes a colour, width, height or radius parameter. A rung (`.dense()`) and a bar's ink are the only exceptions.
- No literal where a token exists. A value on the sheet with no token gets a token in the task that first needs it.
- Every colour comes from `meridian_design::semantic::semantic(dark)` or a named scale; the sheet's `var(--m-…)` name beside each part says which. Where this plan writes a token as a path such as `borders.control` or `rows.selected_bg`, it means the semantic token the emitter writes under the matching CSS name (`--m-border-control`, `--m-rows-selected-bg`); find the Rust field by that name in `meridian-design/src/emit/mod.rs` rather than trusting the path's spelling.
- Every state appears on the next frame. No animation.
- Text is the caller's. The only copy a component owns is `teaching_message`'s "next time, press".
- A component holds nothing between frames.
- An unavailable button, segmented option or ribbon command is disabled, takes no keyboard focus, senses hover, reports no click, reports its rect, and carries its reason as its accesskit description. An unavailable menu item or picker row takes the cursor.
- Tests read the paint list (`tests/common/mod.rs`, from Task 1) and accesskit nodes. No image goldens.
- A token change regenerates `meridian-design/tests/snapshots/tokens.css` and `reference/tokens.md` in the same commit: `cargo run --example dump_css > tests/snapshots/tokens.css` and `cargo run --example dump_md > ../reference/tokens.md`, from `meridian-design/`.
- This repo is public. No private planning identifiers in code, comments, commit messages or pull request text.
- From Task 2 on, every new public item gets its row in `meridian-egui/INVENTORY.md` in the task that adds it.
- Commit messages end with the attribution line the session supplies.

**The gate.** Before each commit run the task's own test, then from the repo root:

```bash
cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace
```

One cargo build at a time. The full gate takes a few minutes on a warm cache.

## Review Focus

Inputs the spec is silent on that a person will hit. Each has a test in the task that owns the code.

1. **Text longer than its slot.** A long label in `button`, `vertical_tab`, `step_row`, `menu_item` or `section` is cut with "…" on one line and never pushes a keycap, count or close control out of its place or out of the row. (Tasks 7, 10, 12, 16, 17)
2. **Empty and degenerate inputs.** Zero tabs, zero options, zero bins, `total == 0`, `done > total`, an empty groups slice: the component draws its empty form or nothing, and does not panic or divide by zero. (Tasks 11, 13, 14, 15, 18)
3. **Too little width.** A `ribbon_row` narrower than its icons and keys clips at its trailing edge without overlapping; a `Popover` whose anchor is at the screen's edge stays wholly on screen. (Tasks 9, 15)
4. **Out-of-range geometry.** `Timeline` fractions below 0, above 1, reversed or NaN are clamped and ordered; a `Gutter` lane at or above `lanes` is ignored. (Task 16)
5. **Wide keystrokes.** A keystroke of several glyphs (`"⌃⌥⇧K"`, `"F12"`) widens the keycap and never its height, in both keycap sizes. (Task 6)

---

## File structure

`meridian-design/src/`: `radius.rs`, `control.rs`, `spacing.rs`, `typography.rs`, `semantic.rs` and `emit/mod.rs` are modified; `marks.rs` is new.

`meridian-egui/src/`, one new module per component: `button.rs`, `text_field.rs`, `popover.rs`, `menu.rs`, `segmented.rs`, `section.rs`, `distribution.rs`, `ribbon.rs` (tabs, row and `key_tip`), `step_row.rs`, `vertical_tab.rs`, `run_status.rs` (with `teaching_message`). `key_chip.rs`, `picker.rs`, `overlay.rs`, `notify.rs`, `widgets.rs`, `theme.rs`, `tokens.rs`, `lib.rs` and `icons/` are modified.

`meridian-egui/tests/`: `common/mod.rs` is new and holds the paint-list helpers; one new test file per module, named for it; `square_look.rs` becomes `chrome_look.rs`; `inventory.rs` is new.

Docs: `decisions/0011-…md`, `decisions/0013-…md`, `guidelines/chrome.md`, `CLAUDE.md`, `ROADMAP.md`, `README.md`, `meridian-egui/INVENTORY.md`, `components/sheet.html`.

---

### Task 1: Corners return, and the look's tests move to shared helpers

**Files:**
- Modify: `meridian-design/src/radius.rs`, `meridian-design/tests/conformance.rs`, `meridian-design/tests/snapshots/tokens.css`, `reference/tokens.md`
- Modify: `meridian-egui/src/key_chip.rs`, `widgets.rs` (status pill), `overlay.rs`, `notify.rs`, `theme.rs`
- Create: `meridian-egui/tests/common/mod.rs`; rename `tests/square_look.rs` to `tests/chrome_look.rs`
- Modify: `decisions/0013-square-chrome.md`, `guidelines/chrome.md`

**Interfaces:**
- Produces: `radius::CHIP == 2.0`, `radius::CONTROL == 3.0`, `radius::PANEL == 3.0`; `NONE` and `FULL` unchanged.
- Produces: `tests/common/mod.rs` with `Painted`, `painted(&Harness, Mode) -> Vec<Painted>`, `filled`, `ink`, `near`, `near_rect`, moved unchanged out of `square_look.rs`, plus `assert_corner(rects: &[Painted], radius: f32, what: &str)` and `assert_square(rects, what)`. Every later test file starts with `mod common;`.

- [ ] **Step 1: Rewrite the tests to the new look, and watch them fail.** In `radius.rs` replace `every_chrome_rung_is_square` with `the_chrome_rungs_are_two_three_and_three` asserting the three values and `NONE == 0.0`. In `conformance.rs` rename `tokens_css_draws_the_square_look` to `tokens_css_draws_the_chrome_look` and assert `--m-radius-chip: 2px`, `--m-radius-control: 3px`, `--m-radius-panel: 3px`, keeping its assertions about bars, the ring and the hard shadows. In `chrome_look.rs` replace the three "drawn without a corner" tests with: `a_key_and_a_pill_draw_the_chip_corner` (2); `the_modal_card_the_banner_and_the_toast_draw_the_panel_corner` (3); `egui_own_buttons_fields_windows_and_menus_draw_their_token_corner` (3); `a_row_the_query_line_line_tabs_and_the_pane_header_stay_square` (0); and extend the two ring tests to assert the ring's corner equals `focus::ring_radius(radius::CONTROL)`.

Run: `cargo test --workspace` — Expected: the renamed tests FAIL on the values.

- [ ] **Step 2: Change the three constants; make each primitive pass its own token's radius.** The key chip is drawn as five square rects today; redraw it as one rect with `radius::CHIP`, the hairline on three sides and the 2px foot, so the corner is the shape's and the foot follows it. The list row, query line, line tabs and pane header keep `0`.
- [ ] **Step 3: Regenerate `tokens.css` and `reference/tokens.md`.**
- [ ] **Step 4: Amend ADR 0013 and `guidelines/chrome.md`** as spec decision 2 says: an `### Update (2026-11-22 — a small corner returns)` section with `date-modified` bumped, stating what narrows, what stands and what stays square; and in `chrome.md` replace "Chrome has no corner" with the three radii, what takes each, and the list that stays square.
- [ ] **Step 5: Run the gate. Commit:** `Corners return at 2, 3 and 3; ADR 0013 amended`

### Task 2: The inventory, and ADR 0011's amendment

**Files:**
- Create: `meridian-egui/INVENTORY.md`, `meridian-egui/tests/inventory.rs`
- Modify: `decisions/0011-desktop-component-code-and-the-licence-firewall.md`, `CLAUDE.md`, `ROADMAP.md`, `README.md`

**Interfaces:**
- Produces: `INVENTORY.md`, one Markdown table with columns `Item | Kind | Door | Call sites or requirement`. `Kind` is `component`, `composition` or `helper`. `Item` is the public name exactly as `lib.rs` re-exports it or as its module exports it.

- [ ] **Step 1: Write `tests/inventory.rs`, failing.** `every_public_item_has_a_row_and_every_row_names_a_public_item`: read `src/lib.rs` and each `pub mod` it declares as text, collect every `pub fn`, `pub struct`, `pub enum` and `pub trait` name reachable from the crate root, read the table's `Item` column, and assert the two sets are equal, printing both differences. `every_component_row_states_a_door`: `Door` is `call sites` or `requirement` for a component, and the fourth column is not empty. Types that are a component's own config or response (for example `ListRow`, `ListRowResponse`) are listed in their component's `Item` cell, comma-separated, so one row covers them.
- [ ] **Step 2: Write `INVENTORY.md` for the surface that exists today.** Components: modal chrome, query line, list row, key chip, picker, line tabs, notification layer, toast layer, field row, pane header, status pill; each with `call sites` and the Brightfield surfaces that call it in plain words (the command palette, the help sheet, the sources browser, the inspector, the status band). Helpers: `focus_ring`, `focus_ring_for`, `focus_ring_on_solid`, `focus_ring_on_solid_for`, `slider_colours`, `SliderColours`, `align_scope`, `LayoutInfo`, `tooltip_for_action`, `optically_centred_galley_top`, `overlay_frame`, `mode_of`, `Mode`, `Tokens`, `TOKENS`, `MeridianUi`, `Icon`, the `theme` functions.
- [ ] **Step 3: Amend ADR 0011** with `### Update (2026-11-22 — the cap becomes an inventory)` per spec decision 1, including the definition of a variant parameter and the three kinds of row. Correct "Capped at 15" in `CLAUDE.md`, and "a capped set" in `CLAUDE.md`'s layout table, `ROADMAP.md` and `README.md`, to "an inventoried set" with a pointer to `meridian-egui/INVENTORY.md`.
- [ ] **Step 4: Run the gate. Commit:** `Replace the primitive cap with an inventory a test holds`

### Task 3: Geometry and type tokens the components need

**Files:**
- Modify: `meridian-design/src/control.rs`, `spacing.rs`, `typography.rs`, `emit/mod.rs`, `tests/conformance.rs`, `tests/snapshots/tokens.css`, `reference/tokens.md`; `meridian-egui/src/tokens.rs`

**Interfaces:**
- Produces, in `control`: `KEYCAP_HEIGHT: f32 = 21.0`, `KEYCAP_HEIGHT_IN_CONTROL: f32 = 16.0`, `KEYCAP_INSET_IN_CONTROL: f32 = SPACE_2` (4), `GUTTER_LANE: f32 = 12.0`, `STEP_MARK_CELL: f32 = 24.0`, `STEP_MARK: f32 = 7.0`, `STEP_MARK_STROKE: f32 = 1.5`, `TIMELINE_WIDTH: f32 = 72.0`, `TIMELINE_BAR_HEIGHT: f32 = 12.0`, `DURATION_WIDTH: f32 = 40.0`, `DOT: f32 = 6.0`.
- Produces, in `spacing`: `BAND_HEIGHT: f32 = ROW_GRID + 2.0 * SPACE_2` (32), `POPOVER_WIDTH: f32 = 240.0`.
- Produces, in `typography`: `LABEL_SIZE: f32 = 11.0`, `LABEL_TRACKING_EM: f32 = 0.06`, `LABEL_UPPERCASE: bool = true`.
- Produces CSS names: `--m-keycap-height`, `--m-keycap-height-in-control`, `--m-keycap-inset-in-control`, `--m-gutter-lane`, `--m-step-mark-cell`, `--m-step-mark`, `--m-timeline-width`, `--m-timeline-bar-height`, `--m-duration-width`, `--m-dot`, `--m-band-height`, `--m-popover-width`, `--m-label-size`, `--m-label-tracking`.

- [ ] **Step 1: Extend `tokens_css_carries_the_geometry_layer`** to assert each new declaration with its value. Add `the_keycap_in_a_control_clears_the_focus_ring_evenly`: `(HEIGHT_MD - KEYCAP_HEIGHT_IN_CONTROL) / 2 == KEYCAP_INSET_IN_CONTROL` and `KEYCAP_INSET_IN_CONTROL - focus::RING_WIDTH == 2.0`. Run it: FAIL.
- [ ] **Step 2: Add the constants and their `decl` lines in `emit::geometry`; mirror them in `meridian-egui`'s `Tokens`.**
- [ ] **Step 3: Regenerate both snapshots. Run the gate. Commit:** `Add the geometry and label tokens the new components read`

### Task 4: The state mark as data

**Files:**
- Create: `meridian-design/src/marks.rs`; modify `meridian-design/src/lib.rs`

**Interfaces:**
- Produces:

```rust
pub enum StepMark { NotRun, Running, Ok, Failed, Stale }
pub enum MarkShape { Ring, HalfDisc, Disc, Diamond, RingWithCentre }
pub struct MarkGlyph { pub shape: MarkShape, pub ink: Rgba, pub fill: Option<Rgba> }
pub const STEP_MARKS: [StepMark; 5];
pub fn glyph(mark: StepMark, dark: bool) -> MarkGlyph;
pub fn word(mark: StepMark) -> &'static str;   // "not run", "running", "ok", "failed", "stale": the accesskit state word
```

- [ ] **Step 1: Tests in `marks.rs`, failing.** `every_state_has_its_own_shape` (five distinct `MarkShape`s, in the spec decision 5 table's order); `each_ink_is_the_named_token_in_both_modes` (not run: `borders.control`; running: `borders.focus`; ok, failed, stale: the status good, critical and warning inks the emitter writes as `--m-status-good`, `--m-status-critical`, `--m-status-warning`); `only_the_disc_and_the_diamond_are_filled`.
- [ ] **Step 2: Implement. Run the gate. Commit:** `State the step mark's shape and ink per state as data`

### Task 5: The 27 icons

**Files:**
- Modify: `meridian-egui/src/icons/tabler.manifest`, `icons/data.rs`, `icons/gen.rs` (whatever the generator rewrites)

- [ ] **Step 1:** In a scratch directory outside the repo, `npm pack @tabler/icons@3.45.0` and unpack it. Copy the 27 outline SVGs the spec's decision 7 names beside the manifest's existing glyphs' sources, then run `cargo run -p meridian-egui --bin gen-tabler-icons -- <svg-dir> 3.45.0`.
- [ ] **Step 2:** Add to `tests/icon_drift.rs` `the_chrome_s_icons_are_in_the_manifest`: `Icon::by_name(name).is_some()` for each of the 27 names.
- [ ] **Step 3: Run the gate. Commit:** `Add the 27 Tabler glyphs the chrome names`

### Task 6: The keycap inside a control

**Files:**
- Modify: `meridian-egui/src/key_chip.rs`, `meridian-design/src/semantic.rs`, `emit/mod.rs`, `tests/chrome_gate.rs`, both snapshots, `guidelines/chrome.md`; `tests/chip_geometry.rs`

**Interfaces:**
- Produces:

```rust
pub enum KeySize { Row, Control }          // 21 and 16 tall
pub enum KeyInk { Plain, Dimmed, OnSolid } // Dimmed: an unavailable command's; OnSolid: on an accent fill
pub fn key_chip_with(ui: &mut egui::Ui, keystroke: &str, size: KeySize, ink: KeyInk) -> egui::Response;
pub(crate) fn key_size(ui: &egui::Ui, keystroke: &str, size: KeySize) -> egui::Vec2;
pub(crate) fn paint_key(ui: &egui::Ui, rect: egui::Rect, keystroke: &str, size: KeySize, ink: KeyInk);
```

`key_chip(ui, keystroke)` stays and is `key_chip_with(ui, keystroke, KeySize::Row, KeyInk::Plain)`.
- Produces semantic inks `keycap_on_solid_rule` and `keycap_on_solid_foot` (the on-solid text ink at 45% and 80% over the accent fill, stored as their flattened sRGB), emitted as `--m-keycap-on-solid-rule` and `--m-keycap-on-solid-foot`.

- [ ] **Step 1: Tests, failing.** In `chip_geometry.rs`: `a_key_in_a_control_is_sixteen_tall` and `a_key_in_a_row_is_still_twenty_one`; `a_wide_keystroke_widens_the_key_and_never_its_height` for `"⌃⌥⇧K"` and `"F12"` at both sizes (Review Focus 5); `a_dimmed_key_draws_its_keystroke_in_the_disabled_ink_and_keeps_its_box`; `a_key_on_a_solid_fill_has_no_face_and_draws_in_the_on_solid_inks`. In `chrome_gate.rs`: `the_key_on_a_solid_fill_is_findable`, both new inks against `accent.bg` in every visible state at the non-text floor the gate already uses.
- [ ] **Step 2: Implement.** The 16 key is the hairline on three sides, 1px of padding above a 12px text box, and the 2px foot.
- [ ] **Step 3: Rewrite `chrome.md`'s keycap line** to name the two sizes and where each is used. Add the INVENTORY rows. Regenerate both snapshots.
- [ ] **Step 4: Run the gate. Commit:** `A keycap inside a control is 16 tall and clears the focus ring`

### Task 7: `button`

**Files:** Create `meridian-egui/src/button.rs`, `tests/button.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:**
- Consumes: Task 6's `paint_key`, `key_size`; `focus_ring_for`, `focus_ring_on_solid_for`.
- Produces: spec §1 (`Weight`, `Button`, `ButtonResponse`, `button`), and `pub(crate) fn paint_button(ui, rect, &Button, state) ` plus `pub(crate) fn button_size(ui, &Button, drop_label: bool) -> Vec2`, which Task 15 uses.

Look (sheet §1, classes `.btn`, `.primary`, `.quiet`): 24 tall, `SPACE_4` leading padding, `ICON_LABEL_GAP` between icon (14), label and key; with a key the trailing inset is `KEYCAP_INSET_IN_CONTROL`. Default: `neutral` role's bg, border and fg per state. Primary: `accent` role, key in `KeyInk::OnSolid`, focus by `focus_ring_on_solid_for`. Quiet: no fill or rule at rest, `neutral.bg_hover` and `neutral.bg_active` under the pointer and pressed. Unavailable: disabled fill and border, text and icon in `text.disabled`, key `KeyInk::Dimmed`. Dense: 20 tall, `SPACE_3` padding.

- [ ] **Step 1: Tests, failing.** One paint-list test per weight asserting fill, stroke and corner (3) at rest, hover, pressed and focus, in both modes; `a_key_sits_four_from_the_top_bottom_and_trailing_edge`; `an_unavailable_button_reports_no_click_and_reports_its_rect_when_pointed_at`; `an_unavailable_button_is_disabled_with_its_reason_as_its_description` (kittest `get_by_role_and_label`); `a_dense_button_is_twenty_tall_and_draws_no_key`; `a_long_label_is_cut_with_an_ellipsis_and_the_key_keeps_its_place` in a 120-wide ui (Review Focus 1); `an_icon_only_button_is_square_and_named_by_its_label`.
- [ ] **Step 2: Implement. Add the INVENTORY row** (`component`, `call sites`: Brightfield's toolbars, dialogs and forms, 32 hand-built buttons).
- [ ] **Step 3: Run the gate. Commit:** `Add button: icon, label and key in three weights`

### Task 8: `text_field`

**Files:** Create `src/text_field.rs`, `tests/text_field.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:** Produces spec §2.

Look (sheet §2 A, `.fld`): 24 tall (20 dense), fills its width, `surfaces.sunken` fill with the top corners at 3 and the bottom square, a 1px bottom rule in `borders.control`; hovered, the rule is `text.primary`; with the keys, 2px in `borders.focus`; with an error, 2px in the danger border. Leading icon and unit in `text.muted`; placeholder in `text.placeholder`. Beneath, one 11px line: help in `text.muted`, or the error in the red scale's step 11. The editing itself is `egui::TextEdit` with its frame off.

- [ ] **Step 1: Tests, failing.** Paint-list tests for rest, hover, keys, error and unavailable in both modes; `the_rule_thickens_without_moving_the_text`; `an_error_replaces_the_help_line`; `typing_changes_the_string_and_reports_changed`; `enter_reports_submitted`; `the_field_fills_the_width_it_is_given`; accesskit: role `TextInput`, the error as description.
- [ ] **Step 2: Implement; INVENTORY row** (`call sites`: 14 hand-built inputs).
- [ ] **Step 3: Run the gate. Commit:** `Add text_field over egui's text editing`

### Task 9: `Popover`

**Files:** Create `src/popover.rs`, `tests/popover.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:**
- Consumes: `Elevation::Overlay` shadow, `spacing::POPOVER_WIDTH`, Task 6's `key_chip_with`.
- Produces: spec §5 (`Anchor`, `PopoverResponse`, `Popover::{show, show_menu, show_reason}`).

Look (sheet §5, `.pop`): `surfaces.overlay` fill, 1px `borders.default`, corner 3, the 3 by 3 hard shadow, no arrow, `SPACE_2` between anchor and card. `show_reason`: 240 wide, padding `SPACE_4` by `SPACE_5`, the sentence wrapped, an optional 21 key at the top right. `show_menu`: 240 wide, `SPACE_2` above and below, no side padding.

- [ ] **Step 1: Tests, failing.** `it_opens_below_its_anchor_and_above_when_there_is_no_room`; `beside_opens_to_the_trailing_side_and_flips_when_there_is_no_room`; `an_anchor_at_the_screen_s_corner_keeps_the_card_wholly_on_screen` (Review Focus 3); `esc_and_a_click_outside_report_dismissed`; `a_click_inside_does_not`; `the_card_is_ruled_with_a_hard_shadow_three_by_three_and_a_three_corner`; `a_reason_is_two_hundred_and_forty_wide_and_wraps`; accesskit `Tooltip` for the reason.
- [ ] **Step 2: Implement** on `egui::Area` with the overlay frame's fill and stroke. INVENTORY row (`requirement`: the chrome's reason for a greyed command, the collapsed ribbon's dropped row, menus).
- [ ] **Step 3: Run the gate. Commit:** `Add Popover: an overlay card anchored to a rect`

### Task 10: `menu_item`, `menu_separator`, and the picker row's unavailable state

**Files:** Create `src/menu.rs`, `tests/menu.rs`; modify `src/picker.rs`, `tests/overlay_picker.rs`, `lib.rs`, `INVENTORY.md`

**Interfaces:**
- Consumes: `list_row`'s state fills and bar; `key_chip_with`.
- Produces: spec §9 and `PickerRow::unavailable`.

Look (sheet §9, `.mi`): the list row at 24 with `SPACE_4` leading and `SPACE_2` trailing padding, a 14 check column when `checked` was called, the label, a right-aligned 21 key, a 14 chevron for a submenu. Unavailable: label, check and detail in `text.disabled`, key dimmed. The separator is a 1px `borders.divider` rule with `SPACE_2` above and below.

- [ ] **Step 1: Tests, failing.** Paint-list tests for rest, hover, pressed, cursor (fill and 3px bar), unavailable and unavailable-with-cursor; `a_checked_item_draws_its_check_and_an_unchecked_one_reserves_the_column`; `an_unavailable_item_reports_no_click_and_reports_its_rect`; `a_long_label_is_cut_and_the_key_keeps_its_column` (Review Focus 1); accesskit `MenuItem`, disabled with its reason. For the picker: `an_unavailable_row_draws_its_reason_as_its_detail_in_the_disabled_ink_and_keeps_its_key`, `it_still_takes_the_cursor`.
- [ ] **Step 2: Implement; INVENTORY rows** (`menu_item` component by requirement: the inspector's commands and the chart encoding's rows; `menu_separator` composition).
- [ ] **Step 3: Run the gate. Commit:** `Add menu_item and the picker row's unavailable state`

### Task 11: `segmented`

**Files:** Create `src/segmented.rs`, `tests/segmented.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:** Produces spec §6.

Look (sheet §6 B, `.seg.b`): one box ruled 1px in `neutral.border` with corner 3; options divided by 1px rules; each option `SPACE_4` leading, label, then a 16 key at `KEYCAP_INSET_IN_CONTROL`. Selected: `neutral.bg_selected` fill, `neutral.border_selected` rule, `neutral.fg_selected` text. Hover `neutral.bg_hover`, pressed `neutral.bg_active`. 24 tall when any option has a keystroke, else 20 with `SPACE_4` padding each side.

- [ ] **Step 1: Tests, failing.** Paint-list tests per state in both modes; `only_the_outer_corners_round`; `it_is_twenty_tall_with_no_keystrokes_and_twenty_four_with_one`; `an_unavailable_option_reports_its_rect_and_no_click`; `no_options_draws_nothing_and_does_not_panic`, `a_selected_index_past_the_end_selects_nothing` (Review Focus 2); accesskit `RadioGroup` of `RadioButton`s, the selected one checked.
- [ ] **Step 2: Implement; INVENTORY row** (`requirement`: a pane's activity switcher).
- [ ] **Step 3: Run the gate. Commit:** `Add segmented: exclusive options in one ruled box`

### Task 12: `section`

**Files:** Create `src/section.rs`, `tests/section.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:** Produces spec §7.

Look (sheet §7 A, `.sec.a`): a 24 header with a 1px `borders.divider` rule on its top edge, `SPACE_2` leading and `SPACE_4` trailing padding, a 14 chevron (down when open, right when closed) in `text.muted`, the title in the label style (11px, capitals, 0.06em tracking, `text.secondary`), the count right-aligned in `text.secondary` at 12px. Hover fills the header with the accordion hover fill. The body is inset `SPACE_6` leading, `SPACE_4` trailing, `SPACE_2` above and `SPACE_5` below, and is not laid out while closed.

- [ ] **Step 1: Tests, failing.** `a_closed_section_does_not_run_its_body` (`inner.is_none()`, body closure not called); `a_click_on_the_header_reports_toggled_and_changes_nothing_itself`; `the_title_is_drawn_in_capitals_whatever_the_caller_passed`; `a_count_sits_at_the_trailing_edge_and_a_long_title_is_cut_before_it` (Review Focus 1); paint-list tests for rest, hover, focus; accesskit: the header is a `Button` named by the title, with its expanded state.
- [ ] **Step 2: Implement; INVENTORY row** (`requirement`: the inspector's stacked sections).
- [ ] **Step 3: Run the gate. Commit:** `Add section: a labelled, countable, collapsible group`

### Task 13: `distribution_bar`

**Files:** Create `src/distribution.rs`, `tests/distribution.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:** Produces spec §8.

Look (sheet §8 A, `.dist`): 36 tall (20 dense), fills its width, a 1px `baseline` rule along the foot. Bars are equal in width with 1px between, heights proportional to the largest count, at least 2 tall when the count is above zero, in the first categorical chart ink; square. For `Top`: the values in the order given, a `SPACE_2` gap, the rest bar in `ink_muted`. Then for both forms a `SPACE_2` gap and the missing bar in the null ink, its height proportional to `missing` against the same largest count. The hovered bar is the sequential ramp's 600 step.

- [ ] **Step 1: Tests, failing.** `bars_are_equal_in_width_and_proportional_in_height`; `a_non_zero_count_is_at_least_two_tall_and_a_zero_count_draws_nothing`; `the_rest_and_the_missing_bars_follow_a_gap_and_are_the_last_two_indices`; `hovering_a_bar_reports_its_index_and_darkens_only_it`; `bars_are_square`; `no_bins_draws_the_baseline_alone`, `a_total_of_zero_does_not_divide` (Review Focus 2); `dense_is_twenty_tall`.
- [ ] **Step 2: Implement; INVENTORY row** (`requirement`: the inspector's column summary).
- [ ] **Step 3: Run the gate. Commit:** `Add distribution_bar`

### Task 14: `ribbon_tabs` and `key_tip`

**Files:** Create `src/ribbon.rs`, `tests/ribbon_tabs.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:**
- Consumes: `paint_key`, the overlay shadow.
- Produces: spec §3 and `key_tip`.

Look (sheet §3 A, `.rib.a`): a 24 strip on the title-bar surface, `SPACE_4` side padding, a 1px `borders.subtle` rule along its foot. A tab is its label with `SPACE_5` each side, in `text.secondary`; hover `text.primary`; pressed `neutral.bg_active`; focus the ring. The open tab is a cell: `surfaces.raised` fill, 1px `borders.subtle` on top and sides with the top corners at 3, no bottom rule, and it covers the strip's rule beneath itself; its label is Medium in `text.primary`. Selected while collapsed: Medium, `text.primary`, no cell. The contextual tab follows a 1px divider inset `SPACE_2` top and bottom with `SPACE_4` each side, and is filled with Maritime step 3 with Maritime step 11 text; open, it is the raised cell with a 2px Maritime step 9 bar on its top edge. `key_tip` paints a 21 key on a foreground layer with the overlay shadow, its trailing bottom corner offset from the target's by a quarter of its width and half its height.

- [ ] **Step 1: Tests, failing.** Paint-list tests per tab state in both modes; `the_open_tab_covers_the_strip_s_rule_beneath_it`; `a_collapsed_strip_draws_no_cell`; `clicks_and_double_clicks_report_the_tab_including_the_selected_one`; `rects_holds_every_tab`; `key_tips_draw_one_key_per_tab_with_a_keystroke_and_do_not_move_the_tabs` (rects equal with tips on and off); `no_stages_and_no_contextual_draws_the_empty_strip` (Review Focus 2); accesskit `TabList` of `Tab`s, the open one selected.
- [ ] **Step 2: Implement; INVENTORY rows** (`ribbon_tabs` component by requirement; `key_tip` composition from the key chip).
- [ ] **Step 3: Run the gate. Commit:** `Add ribbon_tabs and key_tip`

### Task 15: `ribbon_row`

**Files:** Modify `src/ribbon.rs`; create `tests/ribbon_row.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:**
- Consumes: Task 7's `Button`, `paint_button`, `button_size`; Task 14's `key_tip`.
- Produces: spec §4.

Look (sheet §4, `.rrow`): a `BAND_HEIGHT` band on the raised surface with a 1px `borders.subtle` rule along its foot and `SPACE_4` side padding; buttons drawn in the quiet weight with `SPACE_3` leading padding, `SPACE_1` between; groups divided by a 1px rule 16 tall with `SPACE_3` each side. Fitting: measure every command with its label; while the total exceeds the width, drop the label of the last command that still has one. With key tips on, each command's own key is not drawn and its tip is.

- [ ] **Step 1: Tests, failing.** `at_full_width_every_label_is_drawn_and_icon_only_is_empty`; `labels_are_dropped_from_the_trailing_end_until_it_fits` (assert the exact `icon_only` list at two widths); `narrower_than_every_icon_and_key_it_clips_at_the_trailing_edge_and_no_two_commands_overlap` (Review Focus 3); `an_unavailable_command_reports_pointed_unavailable_and_never_clicked`; `key_tips_replace_each_command_s_own_key`; `no_groups_draws_the_empty_band`, `an_empty_group_draws_no_divider` (Review Focus 2); accesskit `Toolbar` of `Button`s.
- [ ] **Step 2: Implement; INVENTORY row** (`requirement`: the ribbon's commands; Run in the title band is a one-command row).
- [ ] **Step 3: Run the gate. Commit:** `Add ribbon_row: quiet buttons that drop labels to fit`

### Task 16: `step_row` and `step_mark`

**Files:** Create `src/step_row.rs`, `tests/step_row.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:**
- Consumes: Task 4's `marks`; Task 3's gutter, timeline and mark tokens.
- Produces: spec §10, and `pub(crate) fn paint_mark(painter, centre: Pos2, mark: StepMark, mode: Mode)`, which Task 18 uses.

Look (sheet §10, `.srow`, timeline B): a 24 row filling its width. The gutter is `5 + lanes × GUTTER_LANE + 5` wide; lane `l`'s centre is `5 + l × 12 + 6` from the row's leading edge. A through lane is a vertical line the row's height; an arriving lane runs from the row's top to the mark's centre and a leaving lane from the centre to the row's foot, straight when on the mark's own lane and a cubic curve otherwise; lines are 1.25 wide in `borders.strong`. The mark is drawn over the lines at its lane's centre. Then, `SPACE_3` apart: the fold control (a 12 chevron and, when folded, the member count, in `text.muted`); the name, filling, cut with "…", in `text.primary` for a step and `text.secondary` for a table; the output right-aligned in `text.secondary`; the timeline column, `TIMELINE_WIDTH` wide and the row's height, with a 1px gridline at each half and a bar `TIMELINE_BAR_HEIGHT` tall from `start` to `end`, at least 2 wide, with a 1px rule and the chip corner, in `borders.strong` ruled `borders.control`, or the critical ink for a failed step and the progress ink for a running one; the duration, `DURATION_WIDTH` wide, right-aligned, `text.muted`; `SPACE_4` trailing. Hover and pressed are the list row's. Cursor is the list row's fill and 3px bar. Focused is `rows.selected_bg` with a 1px inside rule in `rows.selected_border`. Both: the selected fill, the rule and the bar. `step_mark` is a `STEP_MARK_CELL` square cell with the mark centred and the focused treatment.

- [ ] **Step 1: Tests, failing.** `each_state_draws_its_shape_in_its_ink` for the five marks in both modes; `the_mark_sits_on_its_lane` (centre x for lanes 0 and 1); `every_row_s_gutter_is_one_width_for_a_given_lane_count`; `a_lane_at_or_past_the_lane_count_is_ignored`; `a_timeline_is_clamped_ordered_and_never_narrower_than_two` for `(-0.5, 0.2)`, `(0.9, 1.7)`, `(0.6, 0.4)` and `(NAN, 0.5)`, the last drawing no bar (Review Focus 4); `a_table_row_draws_no_timeline_or_duration_but_keeps_the_columns`; `a_folded_row_shows_its_member_count_and_reports_fold_clicked_apart_from_clicked`; paint-list tests for rest, hover, pressed, focused, cursor and both; `a_long_name_is_cut_and_the_output_bar_and_duration_keep_their_columns` (Review Focus 1); accesskit `Row` labelled `"<name>, <state word>"`.
- [ ] **Step 2: Implement; INVENTORY rows** (`step_row` component by requirement: the spine's list; `step_mark` composition: the spine's thin form).
- [ ] **Step 3: Run the gate. Commit:** `Add step_row and step_mark`

### Task 17: `vertical_tab`

**Files:** Create `src/vertical_tab.rs`, `tests/vertical_tab.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:** Produces spec §11.

Look (sheet §11 A, `.vtabs.a`): a 24 row filling its width, `SPACE_4` leading and `SPACE_2` trailing padding, `SPACE_2` between parts. The label is the node, then " · " and the activity in `text.muted`, cut with "…"; `text.secondary` at rest, `text.primary` under the pointer. The dot is `DOT` across, round, in `text.secondary`. The close control is an 18 square holding a 12 "x" in `text.muted`, drawn only under the pointer or on the current place; under the pointer itself it takes `neutral.bg_active` and `text.primary`. Current: `surfaces.raised` fill, 1px `borders.subtle` on the top, bottom and leading side with the leading corners at 3, no trailing rule, extended 1px past the trailing edge so it covers the rail's rule; the node in Medium `text.primary`, the activity in `text.secondary`.

- [ ] **Step 1: Tests, failing.** Paint-list tests for rest, hover, current, current-and-hovered, close-hovered and focus in both modes; `the_current_tab_extends_one_pixel_past_its_trailing_edge`; `the_close_control_is_drawn_only_under_the_pointer_or_on_the_current_place`; `a_click_on_close_reports_close_clicked_and_not_clicked`; `a_long_node_name_is_cut_and_the_dot_and_close_keep_their_places` (Review Focus 1); accesskit `Tab`, selected when current, with a child `Button` named "Close".
- [ ] **Step 2: Implement; INVENTORY row** (`requirement`: the open places).
- [ ] **Step 3: Run the gate. Commit:** `Add vertical_tab`

### Task 18: `run_status` and `teaching_message`

**Files:** Create `src/run_status.rs`, `tests/run_status.rs`; modify `lib.rs`, `INVENTORY.md`

**Interfaces:**
- Consumes: Task 16's `paint_mark`; `key_chip`.
- Produces: spec §12 and `teaching_message`.

Look (sheet §12 A, `.rs.a`): 24 tall, as wide as its contents, `SPACE_4` side padding, `SPACE_3` between parts, corner 3 on its hover fill (`neutral.bg_hover`) and focus ring. Running: the running mark, the step in Medium `text.primary` when given, `"{done} of {total}"` in `text.secondary`, and along its foot a 2px bar the area's full width, `feedback.progress_track` with `feedback.progress_bar` over `done / total` of it, square. Ran: the mark, `when` in `text.secondary`, `age` in `text.muted`. Not run: the not-run mark and the caller's `label` in `text.muted`. `teaching_message` is one line: the command in `text.primary`, " · next time, press" in `text.secondary`, `SPACE_3`, a 21 key.

- [ ] **Step 1: Tests, failing.** `running_draws_the_bar_along_the_foot_at_done_over_total`; `done_past_total_fills_the_bar_and_a_total_of_zero_draws_the_track_alone` (Review Focus 2); `ran_draws_the_mark_it_was_given`; `not_run_draws_the_ring_and_the_caller_s_label`; `the_area_is_the_same_width_running_with_and_without_progress`; hover and focus paint; a click is reported; accesskit `Status`, with a `ProgressBar` child while running; `the_teaching_message_reads_as_one_sentence_to_accesskit`.
- [ ] **Step 2: Implement; INVENTORY rows** (`run_status` component by requirement: the title band's activity area; `teaching_message` composition).
- [ ] **Step 3: Run the gate. Commit:** `Add run_status and teaching_message`

### Task 19: The gallery

**Files:** Modify `meridian-egui/examples/gallery.rs`

- [ ] **Step 1:** Rewrite the example to show every component and composition in the sheet's order, each state drawn fixed where the component allows it (the caller passes selected, open, cursor, focused, unavailable) and live otherwise, with a light and dark toggle that calls `theme::apply`. Group by the sheet's section numbers and titles.
- [ ] **Step 2:** Extend `tests/primitives.rs`'s `every_primitive_renders_headlessly_in_both_modes` to call each new component once in both modes.
- [ ] **Step 3:** Run `cargo run --example gallery -p meridian-egui` beside `components/sheet.html`, compare section by section, and fix any component that differs from the sheet in the task's own module, with a test for what differed.
- [ ] **Step 4: Run the gate. Commit:** `The gallery shows every component in the sheet's order`

### Task 20: The sheet reads the tokens alone; the docs take the outcome

**Files:** Modify `components/sheet.html`, `CLAUDE.md`, `README.md`, `ROADMAP.md`; delete `docs/superpowers/specs/2026-11-22-component-review-design.md` and this plan

- [ ] **Step 1: The sheet.** Remove the corner switch, its script and the `setCorners` call, so the radii come from `tokens.css`. Replace each `--x-` value with the token Task 3 or Task 6 added and delete the `:root` block and the "Values with no token yet" section. Leave the lettered candidates and the rulings.
- [ ] **Step 2: Check it.** `grep -c -- '--x-' components/sheet.html` prints `0`; the page opened in a browser matches its state before the change at the ruled corners.
- [ ] **Step 3: Fold the spec into the repo's docs.** `CLAUDE.md`: the layout table's `meridian-egui/` row names the inventory; the boundary section replaces "Capped at 15, not a gallery" with the inventory and the general-shape rule and their reason; a short "Components" section lists the twelve and four by name, says `components/sheet.html` is the look's reference, records the two keycap sizes and why, the corner values and what stays square, and the rulings in the spec's "Calls made on the owner's behalf" with the reason for each. `ROADMAP.md` and `README.md`: one paragraph each on what landed.
- [ ] **Step 4: Delete the spec and this plan.** Run the gate. Commit: `Fold the component review into the repo's docs`

---

## Dependencies

Tasks 1 to 6 in order. Task 7 needs 6. Task 10 needs 9. Task 15 needs 7 and 14. Task 16 needs 3 and 4. Task 18 needs 16. Tasks 8, 9, 11, 12, 13, 14 and 17 need only 1 to 6. Task 19 needs 7 to 18. Task 20 is last.
