# meridian-egui inventory

Every public function, struct, enum, trait and static of this crate has a row here, and `tests/inventory.rs` fails when the crate and this file disagree in either direction (ADR 0011). A component's own config and response types share its row. Constants, such as the icon glyphs and `PROMPT_GLYPH`, are data and have no row of their own; the `Icon` row covers the glyphs.

- **component**: admitted through one of ADR 0011's two doors, `call sites` (three independent ones) or `requirement` (a named product need that cannot be composed). The last column names the Brightfield surfaces that call it, or the requirement. The components below predate the inventory and were admitted under ADR 0011.
- **composition**: composed from other rows, and says from what.
- **helper**: a public function or type that draws no component of its own.

| Item | Kind | Door | Call sites or requirement |
|---|---|---|---|
| ModalChrome, ModalLayer, ModalLayerResponse, ModalWidth | component | call sites | The command palette, the help sheet and the sources browser |
| query_line, QueryLineResponse | component | call sites | The command palette, the sources browser and the help sheet's filter |
| list_row, ListRow, ListRowResponse, RowHeight, RowState | component | call sites | The command palette, the sources browser and the help sheet |
| key_chip | component | call sites | The command palette, the help sheet and the status band |
| Picker, PickerDelegate, PickerEvent, PickerHint, PickerOutcome, PickerResponse, PickerRow | component | call sites | The command palette, the sources browser and the help sheet |
| line_tabs, LineTabs, LineTabsResponse | component | call sites | The inspector, the sources browser and the help sheet |
| NotificationLayer, Notification, NotificationId, Severity | component | call sites | The status band, the sources browser and the inspector |
| ToastLayer, Toast | component | call sites | The status band, the command palette and the inspector |
| field_row | component | call sites | The inspector, the sources browser and the help sheet |
| pane_header, pane_header_with | component | call sites | The inspector, the sources browser and the status band |
| status_pill | component | call sites | The status band, the inspector and the sources browser |
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
