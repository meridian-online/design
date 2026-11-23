# meridian-egui inventory

Every public function, struct, enum, trait and static of this crate has a row here, and `tests/inventory.rs` fails when the crate and this file disagree in either direction (ADR 0011). A component's own config and response types share its row. Every `pub const` (the icon glyphs and `icons::ALL`, `GRID`, `STROKE_WIDTH`, `PROMPT_GLYPH`, `DEFAULT_TOAST_SECONDS`) is data and has no row of its own; the `Icon` row covers the glyphs. A public type alias or union would need a row like any other item; there are none today.

- **component**: admitted through one of ADR 0011's two doors. `call sites` means at least three independent production call sites; a gallery call does not count, and a call that arrives only through another component of this crate is not independent. Every other component is `requirement`: a named need of Brightfield that cannot be composed from the rest. The last column names the real callers.
- **composition**: composed from other rows, and says from what.
- **helper**: a public function or type that draws no component of its own.

The callers were searched in Brightfield at revision 7b6ddba1 (`crates/`, tests excluded). The components below predate the inventory and were admitted under ADR 0011.

| Item | Kind | Door | Call sites or requirement |
|---|---|---|---|
| ModalChrome, ModalLayer, ModalLayerResponse, ModalWidth | component | call sites | Every Brightfield modal shares one frame. Callers: the overlays in `brightfield-shell/src/window.rs` (commands, keyboard help, jump to asset, close question, new protocol, which protocol, organise joins, preferences, default app) and the SQL card in `typed_card.rs` |
| query_line, QueryLineResponse | component | requirement | Brightfield needs one query line with the prompt glyph above every filtered list. No production caller at 7b6ddba1 outside this crate's `Picker`, which draws it for the commands palette, keyboard help and jump to asset; the gallery also shows it; admitted under ADR 0011 before the inventory |
| list_row, ListRow, ListRowResponse, RowHeight, RowState | component | requirement | Brightfield needs one row treatment, with a selection bar, for every list. Callers: the sources browser (`brightfield-shell/src/sources_browser_view.rs`) directly, and the commands palette, keyboard help and jump to asset through `Picker`, which is not an independent call site |
| button, Button, ButtonResponse, Weight | component | call sites | Brightfield's toolbars, dialogs and forms hand-build their buttons: 32 of them, none through this crate yet. Three weights (default, primary, quiet), an optional icon and keycap, and an unavailable state that carries its reason |
| text_field, TextField, TextFieldResponse | component | call sites | Brightfield's forms and filters hand-build their text inputs: 14 of them, none through this crate yet. A sunken field over a rule, an optional icon and unit, a message line for help or an error, and egui's own `TextEdit` doing the editing |
| key_chip, key_chip_with, KeySize, KeyInk | component | call sites | Callers: `overlays.rs`, `protocol.rs`, `shelf.rs`, `sources_browser_view.rs` and `typed_card.rs`, all in `brightfield-shell/src/` |
| Picker, PickerDelegate, PickerEvent, PickerHint, PickerOutcome, PickerResponse, PickerRow | component | call sites | Callers: three delegates in `brightfield-shell/src/window.rs`, the commands palette, the keyboard help sheet and jump to asset. `PickerRow::unavailable(reason)` is a composition of this row with `menu_item`'s unavailable treatment: the reason as the row's detail, greyed, the key dimmed, still taking the cursor and picking nothing |
| Anchor, Popover, PopoverResponse | component | requirement | The chrome's reason for a greyed command, the collapsed ribbon's dropped row, and menus (a popover of menu items). Anchors below a rect, above when there is no room, or beside it for a submenu; the caller holds whether it is open |
| menu_item, MenuItem, MenuItemResponse | component | requirement | Brightfield needs the list row as a menu's item: a check column, a label, a right-aligned key and a chevron for a submenu, with an unavailable state that still takes the cursor. Callers: the inspector's commands and the chart encoding's rows, inside `Popover::show_menu` |
| segmented, Segmented, SegmentedOption, SegmentedResponse | component | requirement | Brightfield needs a pane's activity switcher: mutually exclusive options in one ruled box, each with an optional key, the selected one in the selection wash |
| menu_separator | composition | n/a | The hairline between a menu's groups: a 1px `borders.divider` rule with `SPACE_2` above and below, composed from the design crate's divider token and the popover's menu spacing |
| line_tabs, LineTabs, LineTabsResponse | component | requirement | Brightfield needs a strip of words over one rule, with a bar under the open one, for its workbench tabs. One caller: `brightfield-workbench/src/chrome.rs` |
| NotificationLayer, Notification, NotificationId, Severity | component | requirement | Brightfield needs notifications that stay until dismissed, one per source. One caller: `brightfield-shell/src/window.rs` (the gallery also shows it) |
| ToastLayer, Toast | component | requirement | Brightfield needs short messages that expire on their own. One caller: `brightfield-shell/src/window.rs` (the gallery also shows it) |
| field_row | component | requirement | Brightfield needs a label and value row for property panes. No production caller at 7b6ddba1; shown in the gallery only; admitted under ADR 0011 before the inventory |
| pane_header, pane_header_with | component | requirement | Brightfield needs one header band for a pane, with a leading icon and a trailing slot. No production caller at 7b6ddba1; shown in the gallery only; admitted under ADR 0011 before the inventory |
| status_pill | component | requirement | Brightfield needs a small icon-and-word state mark. One caller: `brightfield-shell/src/chart_item.rs` (the gallery also shows it) |
| focus_ring, focus_ring_for, focus_ring_on_solid, focus_ring_on_solid_for | helper | n/a | The focus treatment every interactive piece draws |
| slider_colours, SliderColours | helper | n/a | Slider colours for the host's own sliders |
| align_scope, LayoutInfo | helper | n/a | Aligns label and value columns inside a scope |
| tooltip_for_action | helper | n/a | A tooltip that shows an action's keystroke through the key chip |
| optically_centred_galley_top | helper | n/a | Centres a line of text on its row |
| overlay_frame | helper | n/a | The frame a modal or an overlay draws inside |
| mode_of, Mode | helper | n/a | Light or dark chrome, the argument every token accessor takes |
| Tokens, TOKENS, MeridianUi | helper | n/a | The geometry tokens, reached as `ui.tokens()` |
| Icon | helper | n/a | An icon glyph and the way it is drawn |
| apply, install_fonts, meridian_style, meridian_visuals, to_color32 | helper | n/a | The theme: tokens installed onto an egui context |
| derive, manifest_entry_from_svg | helper | n/a | The icon generator's library half, for the offline binary |
