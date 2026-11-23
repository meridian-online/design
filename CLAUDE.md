# design — the Meridian Design System

This repo defines how Meridian looks, reads, and feels. It is the **single source of truth** for token values: colours, the type ramp, spacing and the box model, chart ink, and the artefacts emitted from them. Two consumers today — [`meridian-online/web`](https://github.com/meridian-online/web) and [`meridian-online/brightfield`](https://github.com/meridian-online/brightfield).

If you are about to hard-code a colour, a radius, a control height, a duration or a font size in a consuming repo: stop. It belongs here, and the consumer imports it. ADR 0011 exists because that rule was not enforced and a desktop shell grew a private shadow palette.

## Layout

| Path | Contents |
|---|---|
| `meridian-design/` | The token crate. MIT, **dependency-free by contract** (ADR 0003). The only place token values are defined, plus the emitters (`src/emit/`), the bundled `fonts/`, and `brand/`. |
| `meridian-design/brand/` | The mark, the wordmark, their Affinity sources, and the motion generated from them (ADR 0012). **Reserved, not MIT** — `brand/LICENSE-BRAND.md`, echoed in the root `LICENSE`. Inside the crate because the desktop takes it as a cargo dependency and needs the bytes at compile time. |
| `meridian-egui/` | The egui adapter and the inventoried desktop components (ADR 0011; `meridian-egui/INVENTORY.md`). Takes dependencies of its own. |
| `decisions/` | ADRs 0001–0013 — the scoping decisions and every amendment to them. `_template.md` is the shape. |
| `guidelines/` | Seven citable pages — identity, density, speed, colour, typography, icons, chrome — plus `README.md`, their index. The rules tokens cannot carry. |
| `validation/` | Colour maths and its evidence: the reproducible `.mts` pipeline (with its own private `package.json`), the vendored Radix scale generator, the approved-palette record and the review gallery. |
| `components/` | The component sheet: one hand-written HTML page that draws each `meridian-egui` component in every state, in both themes, over the emitted `tokens.css`, with the ruling on each look recorded on it. It is the reference a component is built and reviewed against. It reads `tokens.css` and the bundled fonts by relative path and restates neither. |
| `motion/` | The offline, dependency-free Python generator for brand motion, plus preview and measurement pages. It emits **into `meridian-design/brand/motion/`** — ADR 0012's home — in both formats, reading the mark from `brand/` and every colour from `tokens.css` rather than copying either. `motion/output/` is untracked scratch for schemes that lost. |
| `reference/` | Generated reference material, never hand-written. `tokens.md` today — every token, both themes, and every contrast ratio a gate defends. The published file *is* the pin; `reference/README.md` carries the regeneration ritual and what each gate holds. |
| `scripts/` | Repo gates that are not Rust tests — the brand-motion pin, and `build-pages.py`, which assembles the published evidence site and refuses to publish a dangling reference. |
| `README.md`, `ROADMAP.md`, `LICENSE` | The public face, the sequence of work, and the MIT grant. |

Toolchain is pinned by `rust-toolchain.toml` (1.95.0). CI: `.github/workflows/ci.yml`.

## The crate is layered — reach for the highest layer that fits

1. **raw** — `scales` (12-step ramps), `viz` (chart palettes), `chrome` (the ink/overlay tokens the renderer reads).
2. **geometry** — `spacing`, `radius`, `control`, `focus`, `elevation`, `motion`. The box model, stated once.
3. **semantic** — `semantic`: what a colour is *for*, framework-neutral, with an instantiated `StateSet` per interaction role.
4. **contract** — `a11y`: role and keyboard intent — the half of a component spec that colour tokens cannot carry.
5. **emitters** — `emit`, one per consumer, each pinned by a snapshot.

Reach for the semantic layer first. Drop to a raw scale only when the thing being coloured genuinely has no semantic name yet — and consider whether the right fix is to give it one.

Colours are designed in OKLCH and **stored as their sRGB conversion**, so no consumer needs colour-space maths (ADR 0008). Values are plain `Copy` structs; colours are framework-neutral sRGB (never a host colour type), dimensions are logical-pixel floats, interaction states are first-class token slots rather than a cascade, and typography carries OpenType features as `(tag, value)` pairs.

`meridian-design` itself stays MIT and dependency-free — that is what keeps it permanently outside the GPL firewall (ADR 0003, ADR 0011). **That contract binds the crate, not the repository.** Sibling crates live here and take dependencies of their own: `meridian-egui` is the first, the egui adapter and the inventoried desktop components ADR 0011 called for (`meridian-egui/INVENTORY.md`).

One thing in the crate is **not** under its MIT grant: `brand/`. The mark and wordmark are trademarks, reserved, carved out in the root `LICENSE` and in `brand/LICENSE-BRAND.md` — the same shape `fonts/` already uses for OFL. Adding files there means adding them under those terms, not the crate's.

## Emitters and their pinned snapshots

Every downstream artefact is generated from the crate and pinned byte-for-byte by a conformance test, so drift fails CI instead of shipping:

| Artefact | Emitter | Pinned by | Regenerate with |
|---|---|---|---|
| Web `tokens.css` | `emit::tokens_css` | `tests/snapshots/tokens.css` | `cargo run --example dump_css > tests/snapshots/tokens.css` |
| `reference/tokens.md` | `emit::tokens_md`, **from `tokens_css` rather than from the token modules** | `reference/tokens.md` itself — the published file is the pin | `cargo run --example dump_md > ../reference/tokens.md` |
| `reference/palette{,_dark}.svg` | `emit::palette_svg`, from `tokens_css` likewise | the published files, plus a gate that every fill is a token value in that mode | `cargo run --example dump_palette light > ../reference/palette.svg` (and `dark`) |
| `brand/motion/form{,_dark}.json` | `motion/build_form.py` | `scripts/check-motion.sh` + `tests/motion.rs` | `cd motion && python3 build_form.py` |
| `brand/motion/form{,_dark}.svg` | `motion/svg_form.py`, from the Lottie above | the same two | the same command |
| `brand/motion/lockup{,_dark}.json` | `motion/build_lockup.py` | the same two | `cd motion && python3 build_lockup.py` |
| `brand/motion/lockup{,_dark}.svg` | `motion/svg_form.py`, from the Lottie above | the same two | the same command |

The desktop app is themed through the `meridian-egui` adapter (ADR 0011), not a JSON theme artefact; the gpui-component `ThemeConfig` emitter was retired once that shell cut over to egui, so `tokens.css` is the only token artefact a *consumer* takes. `reference/tokens.md` is the second emitter and has no consuming repository — its reader is a person. It is generated all the same, and from `tokens_css` rather than from the token modules, because a hand-written reference is right on the day it is written and silently wrong afterwards.

**The brand-motion emitter is Python, so its byte pin is a script rather than a snapshot test.** `cargo test` must not need a Python interpreter, so `scripts/check-motion.sh` runs `build_form.py --check` as its own CI job and `tests/motion.rs` carries what a byte comparison cannot say — that the mark is the tracked mark, that every colour is a token value, and that the two formats describe one animation. Neither gate replaces the other. The SVG emitter reads the built Lottie rather than re-choreographing it, which is the same "read it, do not restate it" rule the crate's emitters keep, applied to the schedule.

Rules:

- **An intentional token change regenerates its snapshot in the same commit.** A red conformance test means a consumer would have drifted; it is never fixed by loosening the test.
- **A snapshot only catches a change, never a missing one.** `conformance.rs` carries extra structural tests for exactly this reason — an emitter that restates a token as a literal instead of reading its module makes editing that token a silent no-op, and a snapshot pinned against the emitter's own output cannot notice. When you add an emitter, add the test that proves it *reads* the tokens.
- Palette maths is gated by `tests/palette_gate.rs` and chrome contrast by `tests/chrome_gate.rs` — both built on `src/validate.rs`, never approved by eye.
- The `validation/*.mts` scripts are an offline, reproducible pipeline; their output is committed into the crate, never computed at build time (ADR 0007).

Run the crate's gates from `meridian-design/`: `cargo build` and `cargo test`.

## Components

`meridian-egui` draws twelve components and four compositions that Brightfield's chrome is assembled from, so that each shape is drawn by one function, its look is decided once, and a change to it is one edit here and a version bump in the consumer. The owner ruled that the components come first and the chrome is assembled from them, because the last time an application drew its own chrome it grew a private palette, five focus treatments and four copies of one modal (ADR 0011).

- **Components:** `button`, `text_field`, `Popover`, `menu_item`, `segmented`, `section`, `distribution_bar`, `ribbon_tabs`, `ribbon_row`, `step_row`, `vertical_tab`, `run_status`. Named for what they are, not where Brightfield puts them: `segmented`, not an activity switcher.
- **Compositions:** `key_tip`, `step_mark`, `teaching_message` and `menu_separator`. The picker row's unavailable state, `PickerRow::unavailable`, is composed the same way but belongs to the picker's inventory row. A menu is a `Popover` of `menu_item`s; there is no menu component.
- **Left out until a surface asks:** stat tiles, an object header, a hover card. Checkbox, radio, combo box and date picker stay egui's own, themed by `theme::apply`. The data grid stays in Brightfield; web takes token values only.

**`components/sheet.html` is the look's reference.** It draws every component in every state in both themes, reads `tokens.css` alone, and keeps the lettered candidates the owner ruled between. Where code and sheet disagree about a look, the sheet is right, and a change of look is made on the sheet first. `cargo run --example gallery -p meridian-egui` draws the components in the sheet's order for comparing the two.

How each component is built:

- A free function taking a builder config and returning a response that carries `egui::Response` and what was acted on; a floating layer is a struct with `show`, as `ModalLayer` is. The caller holds every state (open, selected, cursor, whether a popover is up); a component holds nothing between frames, and every state appears on the next frame.
- No component takes a colour, width, height or radius. A rung (`.dense()`) and a bar's ink are the only exceptions. Text is the caller's; the one phrase a component owns is `teaching_message`'s "next time, press".
- An unavailable button, segmented option or ribbon command is disabled: it takes no keyboard focus, senses hover, reports no click, reports its rect so the caller can anchor the reason popover, and carries its reason as its accesskit description. An unavailable menu item or picker row still takes the cursor, because a list is walked with the keys.
- Tests read the paint list and accesskit nodes (`tests/common/mod.rs`). There are no image goldens.
- Every public item has a row in `meridian-egui/INVENTORY.md` with its door, and `tests/inventory.rs` fails when the crate and the list disagree. ADR 0011's figure of 15 was removed because it was a forecast made before the chrome existed, and a figure revised the first time a real requirement meets it limits nothing. Both doors stand (three independent call sites, or a named product requirement that cannot be composed), and one admitted by a requirement must be a general shape under a general name. A third variant parameter still means the component is inlined back and deleted.

The rulings behind the look, with their reasons:

- **Corners are 2 for chips, 3 for controls and 3 for cards** (ADR 0013's update), the amounts MotherDuck's app measures. The owner relaxed the square chrome after looking closely at Palantir's Quiver. Rows and their fills, bars, rules, data marks and edge-to-edge bands stay square; joined cells round only their outer corners.
- **The keycap has two sizes.** 21 in a row; 16 inside a control, 4 from its top, bottom and trailing edge, because the 21 cap ran through a 24 control's 2px-inset focus ring. 16 rather than the first candidate's 17 makes the inset even on all three sides.
- **On a solid accent fill**, the keycap's rule is the on-solid ink at 70% and its foot at full strength, so both clear 3:1 against every accent state in both modes.
- **Every state mark clears 3:1 on every row ground in both modes.** A mark below it moves the smallest step along its own scale: light not-run takes grey step 10, light stale amber step 11, dark failed red step 11. `tests/chrome_gate.rs` holds it. This keeps status as chrome ink separate from status as chart ink.
- **A text field with an error** shows a 2px danger foot at rest, and while it has the keys the same danger ink closes into a 2px ring on all four sides. Focus blue and danger red on one field contradicted each other.
- **A vertical tab's close control** shows under the pointer, on the current place, and while the tab or the control has keyboard focus, so a keyboard or screen-reader user can close any place.
- **Labels in a row share one baseline.** Text is placed with the face's cap height centred on its band (`cap_centred_galley_top`), so a word with a descender sits level with one without; a row of line tabs showed the step when each word was centred on its own ink. Per-glyph centring stays only for a keystroke alone in its keycap.
- **Calls made on the owner's behalf**, shown on the sheet and not objected to: the contextual ribbon tab takes the Maritime wash; a key tip sits at its target's trailing bottom corner and may cover a neighbour; the state mark sits on its lane in the graph gutter, as MotherDuck's Flight Explainer draws it; state marks carry an ink per state as well as a shape, so shape alone still tells them apart; a text field draws its own message line and leaves its label to the caller's form; the query line stays square.
- **Taken during the build**, each undoable: an unavailable primary button dims its icon with its label; a press on one popover never dismisses another, Esc closes only the topmost, and a reason popover takes no input; a segmented control too narrow for its labels shrinks the longest first and clips to its room; ribbon tabs are their label plus 12 each side; a pressed contextual tab shows the pressed fill over its tint; key tips are drawn only for targets wholly on screen; the run status bar stays square; rules are 1px (`control::HAIRLINE`, `--m-hairline`).
- **A divider on an overlay has its own ink** (`borders.divider_overlay`, `--m-border-divider-overlay`). In dark mode the overlay surface sits on the divider's own grey step, so a menu's separator vanished; dark takes grey step 5, light keeps the divider. `tests/chrome_gate.rs` holds it to the divider's contrast on the raised plane, where the gridline was approved.
- **Where the gallery differs from the sheet, and why it stands:** ribbon key tips replace each command's keycap in place, and nothing moves, so the target stays where the eye is (the sheet closes the row up; open for the owner); a reason popover's wrapped lines are about 4px tighter, egui's line height against the page's; picker rows keep the settled picker's 4px gap.
- **Known limit:** counts do not use tabular figures, because egui 0.35 has no switch for OpenType features.

## ADR conventions

- Numbered `NNNN-kebab-title.md`, sequential, never renumbered. Frontmatter: `status` (`proposed` | `accepted` | `superseded`), `date-created`, `date-modified`. Body follows `_template.md`: context → considered options → decision outcome → consequences.
- **Amend in place.** When the world changes under a decision that was nonetheless reasoned correctly, append an `### Update (YYYY-MM-DD — what changed)` section, bump `date-modified`, and say plainly which clauses narrow and which still stand. ADR 0002's desktop-component amendment and ADR 0003's retirement of the Xilem watch are the worked examples.
- **`superseded-by` is for genuine supersession only** — the reasoning was wrong, not merely overtaken. Set it on the old record and point the new one back; do not use it to record an amendment, and do not silently rewrite a decision's history.
- Don't re-litigate a settled ADR. If the premise has changed, write the amendment or the successor.
- Guidelines pages distil ADRs into something citable in review — "this violates `speed.md`" should be a complete review comment. Keep the ADR authoritative and the guideline short.

## This repo is public

Public on GitHub, MIT, and read by people outside the project — and since the evidence site went up, public as a *rendered* surface too, not only as source. `.github/workflows/pages.yml` publishes the palette review gallery and the six brand-motion harnesses from `main`, assembled by `scripts/build-pages.py`. Anything those pages reach for is published with them, so a file becoming a preview page's dependency is a decision to publish it. The build runs on pull requests as well, because a reference that dangles once published is invisible to every other gate here: the file is tracked, the HTML is valid, CI is green, and the page is broken for everyone who has not got the repo checked out.

Concretely:

- **No private planning identifiers.** Decision-record refs, task ids, milestone ids, acceptance-criterion shorthand, document ids, card ids and spec AC ids from the private planning tracker mean nothing here and leak the shape of private work. If one slips in, delete the pointer and, where it carried meaning, write the actual rationale in plain English.
- Cross-repo references are fine when they resolve publicly: ADR numbers in this repo, `meridian-online/*` PR numbers, upstream issue links.
- Commit messages, comments and PR text are as public as the code.

## Where the boundary sits

- **Tokens and adapters live here. Components mostly do not.** Web keeps its own components (shadcn on Base UI) permanently — this repo publishes no npm package and has no TypeScript build path for consumers. (`validation/` does carry TypeScript, but it is a private, offline colour pipeline whose output is committed; nothing downstream builds it.) Desktop is the exception: with no host widget library left after the move off GPUI, an *inventoried* set of egui components landed here as `meridian-egui` (ADR 0011). Not a gallery — the maintenance concern in ADR 0002 was correct, and the answer is `meridian-egui/INVENTORY.md`: every public item has a row with its door, and a test fails when the crate and the list disagree.
- **Brand assets live here; where they are used does not.** The mark, the wordmark and their motion are the system's (ADR 0012). *Which* surface plays an animation, and when, is the consuming app's call. Brand motion is capped at three assets and confined to brand surfaces — `guidelines/speed.md`'s no-decorative-motion budget is unchanged inside the apps.
- **An application's information architecture never moves here.** Brightfield's dock model, its pane/panel/toolbar/status contracts, its pickers and modal layers stay in Brightfield. A design system that owns an application's IA is the same mistake as one that owns nothing.
- **The egui adapter is a thin emitter and lives here**, not in the consuming app — so a host change re-translates the adapter rather than the system (ADR 0003).
- Consumers pin what they take: web checks the emitted CSS block byte-for-byte on its own side; Brightfield takes the crate as a cargo dependency. A token release therefore has a bump ritual at each consumer — expect it.
