//! The column a picker row's keycap takes, and what the description does with
//! what is left.
//!
//! **The defect these tests were written against is not a missing ellipsis.**
//! egui emits one, epaint draws it, and then the keycap is painted on top of
//! it. `Picker::show_list` used to add the detail label — with
//! `egui::Label::truncate()`, whose `max_width` is `ui.available_width()` at
//! the moment the label is added — and only afterwards add the chip, at which
//! point `key_chip` painted an opaque fill and a hairline over glyphs that were
//! already on the canvas. Every description in the light picker golden ended
//! mid-word with no marker because the marker was underneath the keycap.
//!
//! That is why the claims below are made at two altitudes and why neither one
//! alone would do:
//!
//! - **Layout** says where the galley was placed. It can tell a description
//!   that was handed the whole row from one handed the column the chip leaves,
//!   which is the reservation.
//! - **Drawn** says which triangles epaint emitted and, on a GPU, which pixels
//!   came back. It is the only altitude at which a glyph that was drawn and
//!   then covered is distinguishable from a glyph that was never drawn at all —
//!   at layout the two are the same picture, and a suite that only measured
//!   layout would report a truncation the reader is looking at as an overlap.
//!
//! The pixel half needs a GPU adapter through `wgpu` and is `#[ignore]`d for
//! that reason, exactly as the sibling montage is. Everything else here runs on
//! the CPU tessellation path and is green on a headless runner.

use egui_kittest::Harness;
use meridian_design::radius;
use meridian_design::semantic::semantic;
use meridian_design::spacing::{ICON_LABEL_GAP, ROW_GRID};
use meridian_egui::{theme, Mode, Picker, PickerDelegate, PickerOutcome, PickerRow};

/// Logical points here are exact multiples of the ladder, so this is float
/// noise tolerance and nothing else. The mutations these tests exist to catch
/// move a number by 6.0 at the tightest — the named gap — and by 19 to 39 where
/// the defect actually bit.
const EPS: f32 = 0.01;

/// The window every claim below is measured in: the picker at the width the
/// consuming shell's gallery specimen draws it, in logical points. Narrow
/// enough that a real description overflows, which is the whole condition
/// under test — a picker wide enough for its strings has no column contention
/// and proves nothing.
const WINDOW: egui::Vec2 = egui::vec2(380.0, 260.0);

/// Two pixel densities, because the drawn claims are rasterisation claims: the
/// glyph atlas is rebuilt per density and epaint snaps a galley's origin to a
/// whole physical pixel on the way to the screen.
const DENSITIES: [f32; 2] = [1.0, 2.0];

/// The rows every claim is made over: label, description, keystroke.
///
/// Three of the four descriptions are longer than the column the keycap leaves
/// at [`WINDOW`], so three rows exercise the cut. The fourth is deliberately
/// short and is the control: a description that fits must be drawn whole, with
/// no marker, or "clears the keycap" would be satisfied by a widget that simply
/// stopped drawing descriptions.
///
/// The keystrokes span the width range a keycap actually takes — a single
/// letter, a three-letter cap, a five-letter one — because the column the row
/// has to reserve is the chip's own width and a fix that happened to hold for
/// one keycap width is not the relation.
const ROWS: &[Row] = &[
    Row {
        label: "queue.drain",
        detail: "Drain the queue into the consumer when the producer is idle",
        keystroke: "d",
        cut: true,
    },
    Row {
        label: "queue.fill",
        detail: "Fill the queue from the producer whenever the consumer is idle",
        keystroke: "Esc",
        cut: true,
    },
    Row {
        label: "stack.pop",
        detail: "Pop one level",
        keystroke: "p",
        cut: false,
    },
    Row {
        label: "stack.push",
        detail: "Push the current selection onto the traversal stack",
        keystroke: "Enter",
        cut: true,
    },
];

struct Row {
    label: &'static str,
    detail: &'static str,
    keystroke: &'static str,
    /// Whether this row's description is expected to overflow the column the
    /// keycap leaves. Asserted, never assumed — see
    /// [`the_fixture_cuts_three_descriptions_and_leaves_one_whole`].
    cut: bool,
}

struct Palette;

impl PickerDelegate for Palette {
    fn update_query(&mut self, _query: &str) {}
    fn match_count(&self) -> usize {
        ROWS.len()
    }
    fn row(&self, index: usize) -> PickerRow {
        let r = &ROWS[index];
        PickerRow::new(r.label)
            .detail(r.detail)
            .keystroke(r.keystroke)
    }
    fn confirm(&mut self, _index: Option<usize>, _query: &str) -> PickerOutcome {
        PickerOutcome::Close
    }
}

/// One shape the frame painted, tagged with its position in the paint order.
///
/// Paint order is the question this file exists to answer, so the walk keeps a
/// single ordered sequence rather than sorting rects and text into separate
/// lists the way `tests/chip_geometry.rs` does. Order does not matter to the
/// claims there; here, a glyph inside the keycap's box is *occluded* if the box
/// was painted after it and merely ugly if before, and a reader that had thrown
/// the ordering away could not tell those apart.
enum Shape {
    Rect(egui::Rect, egui::Color32, egui::CornerRadius),
    Text(egui::epaint::TextShape),
}

fn paint_list<S>(harness: &Harness<'_, S>) -> Vec<Shape> {
    fn walk(shape: &egui::Shape, out: &mut Vec<Shape>) {
        match shape {
            egui::Shape::Rect(r) => out.push(Shape::Rect(r.rect, r.fill, r.corner_radius)),
            egui::Shape::Text(t) => out.push(Shape::Text(t.clone())),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &harness.output().shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// What one row drew: the keycap's box, the description's galley, and where
/// each sits in the paint order.
struct DrawnRow {
    /// The index of the row in [`ROWS`].
    index: usize,
    /// The keycap's painted box — fill and hairline, the thing that covered the
    /// text.
    chip: egui::Rect,
    /// Where the keycap's box sits in the frame's paint order.
    chip_order: usize,
    /// The description as it was laid out and painted.
    detail: egui::epaint::TextShape,
    /// Where the description sits in the frame's paint order.
    detail_order: usize,
}

impl DrawnRow {
    /// The description's font box — where layout put the galley.
    fn detail_rect(&self) -> egui::Rect {
        egui::Rect::from_min_size(self.detail.pos, self.detail.galley.size())
    }

    /// The characters the galley actually kept, marker and all.
    fn kept(&self) -> String {
        self.detail.galley.rows[0].row.text()
    }

    /// The mean advance of the glyphs the description drew — "one glyph" in the
    /// only sense a proportional face admits of one. Read off the glyphs this
    /// very galley carries rather than stated as a number, because it is a
    /// property of the face and the size and neither is this file's to pin.
    fn mean_glyph_advance(&self) -> f32 {
        let advances: Vec<f32> = self
            .detail
            .galley
            .rows
            .iter()
            .flat_map(|placed| placed.row.glyphs.iter())
            .map(|glyph| glyph.advance_width)
            .collect();
        assert!(!advances.is_empty(), "the description drew no glyphs");
        advances.iter().sum::<f32>() / advances.len() as f32
    }
}

/// Draw the picker and pair every keycap with the description beside it.
///
/// The pairing is by *content*, not by index: a description is the text run
/// whose kept characters are a prefix of one of [`ROWS`]' descriptions, and its
/// keycap is the chip box that overlaps it vertically. Pairing by order would
/// have quietly survived the probe run that built this file, where an empty
/// text run from the query line shifted every row against the wrong chip and
/// reported a defect that was not there.
fn drawn_rows<S>(harness: &Harness<'_, S>, mode: Mode) -> Vec<DrawnRow> {
    let shapes = paint_list(harness);
    let chip_fill = theme::to_color32(semantic(mode.is_dark()).surfaces.sunken);
    let chip_radius = egui::CornerRadius::from(radius::CHIP);

    let chips: Vec<(usize, egui::Rect)> = shapes
        .iter()
        .enumerate()
        .filter_map(|(n, s)| match s {
            Shape::Rect(rect, fill, cr) if *fill == chip_fill && *cr == chip_radius => {
                Some((n, *rect))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        chips.len(),
        ROWS.len(),
        "the frame painted {} keycaps for {} rows",
        chips.len(),
        ROWS.len()
    );

    let mut out = Vec::new();
    for (n, shape) in shapes.iter().enumerate() {
        let Shape::Text(text) = shape else { continue };
        let kept = text.galley.rows[0].row.text();
        let body = kept.trim_end_matches('\u{2026}');
        if body.is_empty() {
            continue;
        }
        let Some(index) = ROWS.iter().position(|r| r.detail.starts_with(body)) else {
            continue;
        };
        let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
        let (chip_order, chip) = *chips
            .iter()
            .find(|(_, c)| c.center().y > rect.top() && c.center().y < rect.bottom())
            .unwrap_or_else(|| panic!("no keycap on the row of {:?}", ROWS[index].label));
        out.push(DrawnRow {
            index,
            chip,
            chip_order,
            detail: text.clone(),
            detail_order: n,
        });
    }
    out.sort_by_key(|r| r.index);
    assert_eq!(
        out.iter().map(|r| r.index).collect::<Vec<_>>(),
        (0..ROWS.len()).collect::<Vec<_>>(),
        "every row's description must be found exactly once"
    );
    out
}

fn draw(mode: Mode, density: f32) -> Harness<'static, Picker<Palette>> {
    let mut harness = Harness::builder()
        .with_size(WINDOW)
        .with_pixels_per_point(density)
        .build_ui_state(
            move |ui, picker: &mut Picker<Palette>| {
                theme::apply(ui.ctx(), mode);
                picker.show(ui);
            },
            Picker::new(Palette),
        );
    harness.run();
    harness
}

/// The box around the triangles a text shape puts on the screen after epaint
/// has placed it.
///
/// Not the galley's font box and not its cached `mesh_bounds`: the shape the
/// frame emitted is run back through the same `tessellate` call a backend makes
/// before it hands anything to a GPU, and the box comes out of the vertices
/// that come back. That is what makes the occlusion claim below a claim about
/// something drawn rather than about something laid out.
fn tessellated_glyph_box(
    ctx: &egui::Context,
    density: f32,
    text: &egui::epaint::TextShape,
) -> egui::Rect {
    let clipped = egui::epaint::ClippedShape {
        clip_rect: egui::Rect::EVERYTHING,
        shape: egui::Shape::Text(text.clone()),
    };
    let mut box_ = egui::Rect::NOTHING;
    let mut vertices = 0usize;
    for primitive in ctx.tessellate(vec![clipped], density) {
        if let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive {
            vertices += mesh.vertices.len();
            for vertex in &mesh.vertices {
                box_.extend_with(vertex.pos);
            }
        }
    }
    let glyph_vertices: usize = text
        .galley
        .rows
        .iter()
        .map(|placed| placed.row.visuals.glyph_vertex_range.len())
        .sum();
    assert_eq!(
        vertices, glyph_vertices,
        "the tessellated description carries {vertices} vertices against the \
         galley's {glyph_vertices} glyph vertices — this box is not the glyphs"
    );
    box_
}

/// The fixture cuts what it claims to cut.
///
/// Every claim below is quantified over rows, and three of the four are only
/// interesting on a row whose description does not fit. If the window ever
/// widened, or the face ever narrowed, the sweep would go green on four rows
/// that all fit and the file would be testing nothing. This is the assertion
/// that stops that being silent.
#[test]
fn the_fixture_cuts_three_descriptions_and_leaves_one_whole() {
    for mode in [Mode::Light, Mode::Dark] {
        let harness = draw(mode, 1.0);
        for row in drawn_rows(&harness, mode) {
            let expected = &ROWS[row.index];
            let kept = row.kept();
            let was_cut = kept != expected.detail;
            assert_eq!(
                was_cut, expected.cut,
                "{:?} in {mode:?}: the fixture says cut={}, the frame drew {kept:?}",
                expected.label, expected.cut
            );
        }
    }
}

/// **AC1 — the row reserves the keycap's column before the description is laid
/// out against it.**
///
/// Stated as the distance between two independently measured drawn things: the
/// right edge of the galley the description was laid out into, and the left
/// edge of the box the keycap painted. The bar is the one named gap the row
/// spends between its own elements, so the claim is "a description never gets
/// nearer the keycap than two of the row's own parts get to each other" rather
/// than a pixel count.
///
/// Before the fix this ran to −19.1 pt on the first row and −39.2 pt on the
/// fourth: the galley was laid out straight through the keycap, because
/// `Label::truncate()` had been handed `ui.available_width()` at a moment when
/// the chip had claimed nothing.
#[test]
fn the_description_is_laid_out_inside_the_column_the_keycap_leaves() {
    for mode in [Mode::Light, Mode::Dark] {
        for density in DENSITIES {
            let harness = draw(mode, density);
            for row in drawn_rows(&harness, mode) {
                let clearance = row.chip.left() - row.detail_rect().right();
                assert!(
                    clearance >= ICON_LABEL_GAP - EPS,
                    "{:?} in {mode:?} at {density}x: the description's right edge \
                     is {:.2} pt from the keycap's left border, and the row's own \
                     named gap is {ICON_LABEL_GAP} pt",
                    ROWS[row.index].label,
                    clearance
                );
            }
        }
    }
}

/// **AC2, on the triangles epaint emits — no description glyph is covered by
/// the keycap.**
///
/// Two things have to hold together for this to be about occlusion rather than
/// about layout. The keycap has to be painted *after* the description, which is
/// the condition under which its opaque fill hides anything; and no glyph
/// triangle may fall inside the box it paints. Assert only the second and a
/// frame that painted the chip first would read as fixed while the reader still
/// saw text through a keycap; assert only the first and nothing about the
/// picture has been said at all.
///
/// This is the altitude at which the original defect is even visible. A
/// truncated glyph emits no triangles, so a suite reading layout alone sees the
/// same picture whether the description was cut short or drawn in full and
/// buried — and the goldens showed glyphs sliced vertically by the chip's
/// border, which is the second.
#[test]
fn no_description_glyph_is_drawn_under_the_keycap() {
    for mode in [Mode::Light, Mode::Dark] {
        for density in DENSITIES {
            let harness = draw(mode, density);
            for row in drawn_rows(&harness, mode) {
                assert!(
                    row.chip_order > row.detail_order,
                    "{:?} in {mode:?}: the keycap is painted before the \
                     description, so this test would pass on a frame that \
                     showed text through a keycap",
                    ROWS[row.index].label
                );
                let glyphs = tessellated_glyph_box(&harness.ctx, density, &row.detail);
                assert!(
                    glyphs.right() <= row.chip.left() + EPS,
                    "{:?} in {mode:?} at {density}x: the description's drawn \
                     glyphs reach {:.2} pt and the keycap the frame paints over \
                     them starts at {:.2} pt — {:.2} pt of the description is \
                     under the keycap",
                    ROWS[row.index].label,
                    glyphs.right(),
                    row.chip.left(),
                    glyphs.right() - row.chip.left()
                );
            }
        }
    }
}

/// **AC3 — an overflowing description ends between words with a visible marker,
/// and the marker is not itself under the keycap.**
///
/// The card proposed `break_anywhere = false` as the mechanism, and it is not
/// one. epaint breaks the row at the last word boundary that fits and *then*
/// pops glyphs back off it to make room for the marker, which puts the cut
/// inside a word again — `TextWrapping::break_anywhere`'s own documentation
/// warns about exactly that — and where it does not, it keeps the space it
/// broke on and the row reads `producer \u{2026}` with the marker hanging off a
/// gap. Both readings are asserted against below, so the setting the widget
/// actually ships is pinned in both directions rather than assumed.
///
/// The claim is made against the source string: what was kept has to be a
/// prefix of the description, it has to end on a non-space, and the description
/// has to carry on with a space at exactly that point. A mid-word cut is a
/// prefix too, and a cut with a trailing space ends between words too, which is
/// why all three clauses are here and none of them is the prefix alone.
#[test]
fn an_overflowing_description_ends_between_words_with_a_visible_marker() {
    for mode in [Mode::Light, Mode::Dark] {
        for density in DENSITIES {
            let harness = draw(mode, density);
            for row in drawn_rows(&harness, mode) {
                let expected = &ROWS[row.index];
                let kept = row.kept();
                if !expected.cut {
                    assert_eq!(
                        kept, expected.detail,
                        "{:?} in {mode:?}: a description that fits is drawn whole",
                        expected.label
                    );
                    assert!(
                        !kept.contains('\u{2026}'),
                        "{:?} in {mode:?}: a description that fits carries no marker",
                        expected.label
                    );
                    continue;
                }

                let body = kept.strip_suffix('\u{2026}').unwrap_or_else(|| {
                    panic!(
                        "{:?} in {mode:?}: the cut description {kept:?} ends with \
                         no marker",
                        expected.label
                    )
                });
                assert!(
                    expected.detail.starts_with(body),
                    "{:?} in {mode:?}: {body:?} is not a prefix of the description",
                    expected.label
                );
                assert!(
                    !body.ends_with(char::is_whitespace),
                    "{:?} in {mode:?}: the cut keeps the space it broke on, so \
                     the marker hangs off a gap: {kept:?}",
                    expected.label
                );
                assert!(
                    expected.detail[body.len()..].starts_with(char::is_whitespace),
                    "{:?} in {mode:?}: the cut lands inside {:?}, not between words",
                    expected.label,
                    expected.detail[body.len()..]
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                );

                // And the marker is drawn where it can be read. The galley's
                // right edge is the marker's, since the marker is its last
                // glyph.
                assert!(
                    row.detail_rect().right() <= row.chip.left() + EPS,
                    "{:?} in {mode:?}: the marker is under the keycap",
                    expected.label
                );
            }
        }
    }
}

/// **AC5 — this card is horizontal only: the match rows still sit on the named
/// ladder rung, and nothing the description does spills off it.**
///
/// Two halves, because the rung can move in two ways and the diff on this card
/// could have caused either. The row's own painted box has to measure the
/// ladder value the rung names — the same claim the consuming shell's gallery
/// gate makes on the accessibility node, made here on the box the frame
/// painted, in the repo that owns the ladder. And no description glyph may be
/// drawn outside the row it belongs to, which is what a description allowed to
/// wrap to a second line would do: the wrap alternative this card ruled out
/// leaves the row's *box* on its rung and puts the text through the row below.
#[test]
fn the_match_rows_still_sit_on_their_named_ladder_rung() {
    for mode in [Mode::Light, Mode::Dark] {
        for density in DENSITIES {
            let harness = draw(mode, density);
            let shapes = paint_list(&harness);
            let selected = theme::to_color32(semantic(mode.is_dark()).rows.selected_background);
            let control = egui::CornerRadius::from(radius::CONTROL);
            let boxes: Vec<egui::Rect> = shapes
                .iter()
                .filter_map(|s| match s {
                    Shape::Rect(rect, fill, cr) if *fill == selected && *cr == control => {
                        Some(*rect)
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(
                boxes.len(),
                1,
                "the picker draws exactly one selected row, and it is the only \
                 row that paints a box to measure"
            );
            assert!(
                (boxes[0].height() - ROW_GRID).abs() < EPS,
                "in {mode:?} at {density}x the selected match row measures {:.2} \
                 pt and the Grid rung names {ROW_GRID}",
                boxes[0].height()
            );

            // The first row is the selected one, so its box is the band every
            // description of that row must stay inside.
            for row in drawn_rows(&harness, mode) {
                if row.index != 0 {
                    continue;
                }
                let glyphs = tessellated_glyph_box(&harness.ctx, density, &row.detail);
                assert!(
                    glyphs.top() >= boxes[0].top() - EPS
                        && glyphs.bottom() <= boxes[0].bottom() + EPS,
                    "in {mode:?} at {density}x the description draws from {:.2} to \
                     {:.2} pt and its row runs from {:.2} to {:.2} — the text is \
                     off the rung",
                    glyphs.top(),
                    glyphs.bottom(),
                    boxes[0].top(),
                    boxes[0].bottom()
                );
            }
        }
    }
}

/// **AC2, on real pixels — the drawn description clears the keycap by more than
/// a glyph.**
///
/// The card's evidence was read off a rendered golden, and this is that
/// measurement taken in the repo that causes it: render the frame through
/// `wgpu`, find the rightmost column of description ink in the band between
/// where the description starts and where the keycap's border is, and measure
/// back to the border.
///
/// It has to be pixels and not triangles. The triangle box above is the
/// *geometry* epaint emits; what a reader sees is that geometry composited, and
/// the number the card reported — 0.5 pt of clearance on four of six rows —
/// is a distance between two things on a screen. Rendered, the failing case
/// does not report a negative clearance at all: the chip's fill has already
/// covered the glyphs, so the rightmost visible ink lands *on* the border and
/// the clearance reads 0.4 pt, which is the picture the card described.
///
/// `#[ignore]` because it needs a GPU adapter through `wgpu` and the CI runner
/// for this repo has not got one — the same reason and the same treatment as
/// the sibling repo's montage. **Run it by hand when this geometry changes:**
/// `cargo test -p meridian-egui --test picker_chip_column -- --ignored`.
#[test]
#[ignore = "needs a GPU adapter through wgpu"]
fn the_drawn_description_clears_the_keycap_by_more_than_a_glyph() {
    for mode in [Mode::Light, Mode::Dark] {
        let density = 2.0;
        let mut harness = Harness::builder()
            .with_size(WINDOW)
            .with_pixels_per_point(density)
            .wgpu()
            .build_ui_state(
                move |ui, picker: &mut Picker<Palette>| {
                    theme::apply(ui.ctx(), mode);
                    picker.show(ui);
                },
                Picker::new(Palette),
            );
        harness.run();
        let rows = drawn_rows(&harness, mode);
        let image = harness
            .render()
            .expect("a wgpu adapter and a rendered frame");

        for row in rows {
            let rect = row.detail_rect();
            let left = (rect.left() * density).floor() as u32;
            let right = ((row.chip.left() * density).floor() as u32).min(image.width());
            let top = (rect.top() * density).floor() as u32;
            let bottom = ((rect.bottom() * density).ceil() as u32).min(image.height());
            assert!(left < right, "the band to scan is empty");

            // The band's background is whatever most of it is: the row fill,
            // selected or not. Taken from the image rather than from a token so
            // the measurement does not depend on knowing which state the row
            // drew in, and so antialiased edges are ink by construction.
            let mut counts = std::collections::HashMap::new();
            for y in top..bottom {
                for x in left..right {
                    *counts.entry(image.get_pixel(x, y).0).or_insert(0usize) += 1;
                }
            }
            let background = *counts
                .iter()
                .max_by_key(|(_, count)| **count)
                .expect("a non-empty band")
                .0;

            let mut rightmost: Option<u32> = None;
            for y in top..bottom {
                for x in left..right {
                    let pixel = image.get_pixel(x, y).0;
                    let distance: i32 = (0..3)
                        .map(|c| (pixel[c] as i32 - background[c] as i32).abs())
                        .sum();
                    if distance > 12 {
                        rightmost = Some(rightmost.map_or(x, |r: u32| r.max(x)));
                    }
                }
            }
            let ink_right = (rightmost.expect("the description drew ink") + 1) as f32 / density;
            let clearance = row.chip.left() - ink_right;
            let glyph = row.mean_glyph_advance();
            assert!(
                clearance >= glyph,
                "{:?} in {mode:?}: {clearance:.2} pt between the rightmost \
                 description ink and the keycap's border, against a mean glyph \
                 advance of {glyph:.2} pt",
                ROWS[row.index].label
            );
        }
    }
}
