//! The geometry of the two chip primitives, read out of the paint list — the
//! horizontal rhythm, and the vertical the glyphs actually sit on.
//!
//! **Nothing held the horizontal before.** Reverting either `status_pill` or
//! `key_chip` to the raw `space[2]` it used to spend left the whole workspace
//! suite green — 14 test binaries, no failures, under each mutation
//! separately. The accessibility tree carries the label, not the capsule, and
//! the capsule is the thing whose width moved; `tests/overlay_picker.rs` reads
//! the paint list for chips but asks it only about height. So the axis this
//! file started as was unguarded in a design system whose whole argument is
//! that its own components are not off.
//!
//! **Nor did anything hold the vertical**, and the pill was wrong on it.
//! `status_pill` used to place its label by centring the galley's *font box* —
//! ascent and descent, the same height for every string in the face — which
//! centres the metrics and lets the glyphs fall where they may inside them.
//! Nothing here or anywhere else in the workspace could see the difference,
//! because the box is exactly where the box-centring code says it is however
//! the glyphs sit in it. The vertical tests below measure the glyphs instead,
//! and they measure them off the triangles the frame would draw.
//!
//! Both chips are under test on the vertical now. The keycap's claim is made
//! twice, at two altitudes, because for the keycap they are not the same claim.
//! The first is where the widget's arithmetic puts the glyphs, read off the
//! galley's own row meshes. The second is where epaint *draws* them, read off
//! the primitive `Context::tessellate` returns — and epaint snaps a galley's
//! origin to a whole physical pixel on the way, which the first altitude cannot
//! see. The pill's geometry makes the snap a no-op; the keycap's does not, and
//! a suite that only measured layout would report the keycap centred when the
//! drawn keystroke is half a physical pixel off.
//!
//! Every expectation below is laid out from `meridian-design` constants here,
//! never asked of the code under test, and every measurement comes back out of
//! the shapes the frame actually painted. The vertical claims go one step
//! further and are not stated from a constant at all: they are relations
//! between two independently measured drawn things — the box the chip painted,
//! and the triangles its text tessellated into — so there is no expression
//! they could be quietly copying from the widget.

use egui_kittest::Harness;
use meridian_design::control::{HEIGHT_XS, ICON_XS};
use meridian_design::semantic::{semantic, Role};
use meridian_design::spacing::{CHIP_PADDING_X, ICON_LABEL_GAP, SPACE_1};
use meridian_egui::{icons, key_chip, theme, widgets, Mode};

/// The hairline every box in this crate is stroked with. Named here so the
/// insets below are spelled out term by term rather than borrowed from the
/// code under test — the same term `tests/overlay_picker.rs` spends on the
/// vertical axis.
const HAIRLINE: f32 = 1.0;

/// Logical pixels are exact multiples of the ladder here, so this is float
/// noise tolerance and nothing else. The horizontal mutations these tests are
/// built to catch move a number by 2.0; the vertical ones move it by 0.5 at
/// the tightest, which is still fifty times this.
const EPS: f32 = 0.01;

/// The two pixel densities a Meridian surface is drawn at. Both are exercised
/// because the vertical defect is a *rasterisation* difference and not only a
/// layout one: the glyph atlas is rebuilt per density, so the room a face
/// leaves above its ascenders and below its baseline is not the same fraction
/// of a point at 1.0 as it is at 2.0.
const DENSITIES: [f32; 2] = [1.0, 2.0];

fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

/// Is `v` (in points) on a whole physical pixel at `pixels_per_point`?
fn on_a_physical_pixel(v: f32, pixels_per_point: f32) -> bool {
    let pixels = v * pixels_per_point;
    (pixels - pixels.round()).abs() < EPS
}

/// Every rect, text run and stroked path the frame painted, flattened out of
/// the shape tree. A deliberately separate walk from the one in
/// `overlay_picker.rs`: if the two shared a reader, a bug in the reader would
/// hide from both.
///
/// Text is kept as the whole painted shape rather than as one rect, because
/// the three questions this file asks of a label want different things out of
/// it: the horizontal claims want the font box egui laid out, the vertical ones
/// want the glyphs, and [`tessellated_glyph_box`] wants the shape itself so it
/// can hand epaint the very shape the frame emitted rather than a
/// reconstruction of it. Paths are kept because that is what an icon is —
/// `Icon::paint` flattens its curves and adds them as stroked polylines, so the
/// only way to ask where the icon was actually drawn is to read its points
/// back.
struct Painted {
    rects: Vec<(egui::Rect, egui::Color32, egui::CornerRadius)>,
    texts: Vec<egui::epaint::TextShape>,
    paths: Vec<Vec<egui::Pos2>>,
}

fn painted<S>(harness: &Harness<'_, S>) -> Painted {
    fn walk(shape: &egui::Shape, out: &mut Painted) {
        match shape {
            egui::Shape::Rect(r) => out.rects.push((r.rect, r.fill, r.corner_radius)),
            egui::Shape::Text(t) => out.texts.push(t.clone()),
            egui::Shape::Path(p) => out.paths.push(p.points.clone()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Painted {
        rects: Vec::new(),
        texts: Vec::new(),
        paths: Vec::new(),
    };
    for clipped in &harness.output().shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// The box around the triangles a text shape would put on the screen.
///
/// Built from the mesh vertices rather than from `Galley::mesh_bounds`, which
/// is the same figure cached on the galley: the widget under test reads that
/// cached field, so a test that also read it would be asking the code to
/// confirm its own arithmetic. Walking the vertices is a second derivation of
/// the same thing, and a stale or mis-set `mesh_bounds` would show up as a
/// disagreement rather than as agreement with itself.
///
/// `glyph_vertex_range` is what keeps this to glyphs: a row's mesh can also
/// carry a background quad and underline/strikethrough geometry, and none of
/// those is what the eye centres on.
fn glyph_box(pos: egui::Pos2, galley: &egui::Galley) -> egui::Rect {
    let mut box_ = egui::Rect::NOTHING;
    for placed in &galley.rows {
        let visuals = &placed.row.visuals;
        for vertex in &visuals.mesh.vertices[visuals.glyph_vertex_range.clone()] {
            box_.extend_with(vertex.pos + placed.pos.to_vec2() + pos.to_vec2());
        }
    }
    box_
}

/// The box around the triangles a text shape puts on the screen **after epaint
/// has placed it**, as against where layout put it.
///
/// [`glyph_box`] above reads the galley's own row meshes, which is where the
/// widget's arithmetic landed. This runs the text shape the frame actually
/// painted back through [`egui::Context::tessellate`] — the same call a backend
/// makes before it hands anything to a GPU — and reads the vertices out of the
/// primitive that comes back. The difference between the two is epaint's
/// whole-pixel snap of the galley origin
/// (`epaint::TessellationOptions::round_text_to_pixels`, on by default), which
/// is invisible at layout and is what a reader sees.
///
/// The vertex count is checked against the galley's own glyph count rather than
/// assumed: a row mesh can also carry a background quad and underline
/// geometry, and if either ever appeared this box would be wider than the
/// glyphs and every claim measured off it would be measuring something else.
fn tessellated_glyph_box(
    ctx: &egui::Context,
    pixels_per_point: f32,
    text: &egui::epaint::TextShape,
) -> egui::Rect {
    let clipped = egui::epaint::ClippedShape {
        clip_rect: egui::Rect::EVERYTHING,
        shape: egui::Shape::Text(text.clone()),
    };
    let mut box_ = egui::Rect::NOTHING;
    let mut vertices = 0usize;
    for clipped_primitive in ctx.tessellate(vec![clipped], pixels_per_point) {
        if let egui::epaint::Primitive::Mesh(mesh) = clipped_primitive.primitive {
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
        "the tessellated text carries {vertices} vertices against the galley's \
         {glyph_vertices} glyph vertices — this box is not the glyphs"
    );
    box_
}

impl Painted {
    /// The one box painted with `fill` at the chip radius. Filtering on the
    /// pair is what addresses a chip exactly: the surface behind it carries the
    /// same rect in a one-widget frame, and is drawn square.
    fn chip_box(&self, fill: egui::Color32) -> egui::Rect {
        let radius = egui::CornerRadius::from(meridian_design::radius::CHIP);
        let found: Vec<egui::Rect> = self
            .rects
            .iter()
            .filter(|(_, f, r)| *f == fill && *r == radius)
            .map(|(rect, _, _)| *rect)
            .collect();
        assert_eq!(found.len(), 1, "exactly one chip box painted");
        found[0]
    }

    /// The one text run painted, as the font box egui laid it out in. Each
    /// harness below draws a single chip, so a second run would mean the frame
    /// is not the frame under test.
    fn only_text(&self) -> egui::Rect {
        let text = self.only_text_shape();
        egui::Rect::from_min_size(text.pos, text.galley.size())
    }

    /// The one text run painted, as the glyphs it would actually draw.
    fn only_glyphs(&self) -> egui::Rect {
        let text = self.only_text_shape();
        glyph_box(text.pos, &text.galley)
    }

    fn only_text_shape(&self) -> &egui::epaint::TextShape {
        assert_eq!(self.texts.len(), 1, "exactly one text run painted");
        &self.texts[0]
    }

    /// The box around every stroked path the frame drew. In these harnesses
    /// that is the icon and nothing else — the capsule and the keycap are
    /// rects, and the hairline round them is a rect stroke, not a path.
    fn paths_box(&self) -> egui::Rect {
        let mut box_ = egui::Rect::NOTHING;
        for path in &self.paths {
            for point in path {
                box_.extend_with(*point);
            }
        }
        box_
    }
}

/// What one drawn chip measured: its own box, its label's font box, its
/// label's glyphs as laid out and as tessellated, and — for a pill — the icon's
/// drawn strokes.
///
/// `glyphs` and `drawn_glyphs` are the same box a pipeline stage apart, and the
/// stage between them is epaint's whole-pixel snap of the galley origin. They
/// agree whenever the snap is a no-op, which is most of the time and is not
/// something a test may assume.
struct Drawn {
    chip: egui::Rect,
    text: egui::Rect,
    glyphs: egui::Rect,
    drawn_glyphs: egui::Rect,
    icon: egui::Rect,
}

/// Draw one status pill on its own and measure it.
fn drawn_pill_at(mode: Mode, label: &'static str, role: Role, density: f32) -> Drawn {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 200.0))
        .with_pixels_per_point(density)
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), mode);
            widgets::status_pill(ui, &icons::CLOCK, label, role);
        });
    harness.run();
    let p = painted(&harness);
    let fill = theme::to_color32(semantic(mode.is_dark()).role(role).background.base);
    Drawn {
        chip: p.chip_box(fill),
        text: p.only_text(),
        glyphs: p.only_glyphs(),
        drawn_glyphs: tessellated_glyph_box(&harness.ctx, density, p.only_text_shape()),
        icon: p.paths_box(),
    }
}

fn drawn_pill(mode: Mode, label: &'static str, role: Role) -> Drawn {
    drawn_pill_at(mode, label, role, 1.0)
}

/// Draw one keycap chip on its own and measure it.
fn drawn_key_chip_at(mode: Mode, keystroke: &'static str, density: f32) -> Drawn {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 200.0))
        .with_pixels_per_point(density)
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), mode);
            key_chip(ui, keystroke);
        });
    harness.run();
    let p = painted(&harness);
    let fill = theme::to_color32(semantic(mode.is_dark()).surfaces.sunken);
    Drawn {
        chip: p.chip_box(fill),
        text: p.only_text(),
        glyphs: p.only_glyphs(),
        drawn_glyphs: tessellated_glyph_box(&harness.ctx, density, p.only_text_shape()),
        icon: p.paths_box(),
    }
}

fn drawn_key_chip(mode: Mode, keystroke: &'static str) -> Drawn {
    drawn_key_chip_at(mode, keystroke, 1.0)
}

/// The labels the gallery's pills carry, two of them with descenders — the
/// defect on the *vertical* axis is descender-dependent, and a horizontal claim
/// that only held for `ok` would be worth nothing.
const PILL_LABELS: [(&str, Role); 4] = [
    ("ready", Role::Info),
    ("ok", Role::Success),
    ("waiting", Role::Neutral),
    ("failing", Role::Danger),
];

/// The keystrokes every keycap claim below is made over.
///
/// Four labels chosen to span the two independent ways a glyph box can sit
/// inside the font box it was laid out in — where the ink starts, and where it
/// ends. `Esc` inks from cap height down to the baseline; `Ctrl+K` from
/// ascender height down to the baseline; `Pg Dn` from cap height down into the
/// descender; and `y` is the extreme, x-height above and descender below, with
/// no cap and no ascender in it at all.
///
/// That the four really do differ is not assumed — see
/// [`the_keystrokes_under_test_ink_boxes_that_start_and_end_differently`]. It
/// is what makes the vertical claims below able to tell centring on the glyphs
/// apart from centring on the font box *and* from lifting every keystroke by a
/// constant, which is the mutation a single-label test cannot see.
const KEYSTROKES: [&str; 4] = ["Esc", "Ctrl+K", "Pg Dn", "y"];

/// The pill is inset from its capsule by the named chip padding, on both edges.
///
/// This is the defect the token was introduced for, stated as the picture
/// shows it: the outer inset used to be 4.0 against a 6.0 gap between the icon
/// and the label, so the group sat looser in its middle than it sat inside the
/// capsule. The trailing inset is measured directly; the leading one is what is
/// left of the run from the capsule's edge to the label once the icon and the
/// gap are taken out of it.
#[test]
fn the_pill_spends_the_named_chip_padding_on_both_outer_edges() {
    for mode in [Mode::Light, Mode::Dark] {
        for (label, role) in PILL_LABELS {
            let pill = drawn_pill(mode, label, role);
            let (capsule, text) = (pill.chip, pill.text);

            let trailing = capsule.right() - text.right();
            let leading = (text.left() - capsule.left()) - ICON_XS - ICON_LABEL_GAP;

            assert!(
                near(trailing, CHIP_PADDING_X),
                "{mode:?} {label}: trailing inset {trailing} is not CHIP_PADDING_X \
                 ({CHIP_PADDING_X})"
            );
            assert!(
                near(leading, CHIP_PADDING_X),
                "{mode:?} {label}: leading inset {leading} is not CHIP_PADDING_X \
                 ({CHIP_PADDING_X})"
            );
            assert!(
                trailing + EPS >= ICON_LABEL_GAP,
                "{mode:?} {label}: the rhythm is inverted — the capsule insets the \
                 group by {trailing} while the group is {ICON_LABEL_GAP} loose in \
                 its middle"
            );
        }
    }
}

/// The capsule is exactly the width its four terms add up to, and nothing else
/// has crept into the row.
///
/// Sizes in points, decoded off the sign-off montage at 2.0 px/pt: `ready`
/// 62.5, `ok` 43.5, `waiting` 71.0, `failing` 64.0 — each 4.0 wider than it was
/// before this token existed. Those figures are not asserted here because they
/// are font metrics; the equation that produced them is.
#[test]
fn the_capsule_is_its_terms_and_no_more() {
    for (label, role) in PILL_LABELS {
        let pill = drawn_pill(Mode::Light, label, role);
        let (capsule, text) = (pill.chip, pill.text);
        let expected = CHIP_PADDING_X + ICON_XS + ICON_LABEL_GAP + text.width() + CHIP_PADDING_X;
        assert!(
            near(capsule.width(), expected),
            "{label}: the capsule drew {} wide against the {expected} its terms \
             add up to",
            capsule.width()
        );
    }
}

/// The keycap spends the same named padding, which is the half of the token
/// that makes it shared rather than a pill constant with a general name.
///
/// The hairline is in the expectation because the chip's box is a stroked
/// `egui::Frame` and the painted rect carries the stroke outside the margin —
/// the same term `overlay_picker.rs` spends when it derives the chip's height.
#[test]
fn the_keycap_spends_the_same_named_chip_padding() {
    for mode in [Mode::Light, Mode::Dark] {
        for keystroke in KEYSTROKES {
            let keycap = drawn_key_chip(mode, keystroke);
            let (chip, text) = (keycap.chip, keycap.text);

            let leading = text.left() - chip.left();
            let trailing = chip.right() - text.right();
            let expected = CHIP_PADDING_X + HAIRLINE;

            assert!(
                near(leading, expected),
                "{mode:?} {keystroke}: leading inset {leading} is not \
                 CHIP_PADDING_X + hairline ({expected})"
            );
            assert!(
                near(trailing, expected),
                "{mode:?} {keystroke}: trailing inset {trailing} is not \
                 CHIP_PADDING_X + hairline ({expected})"
            );
            assert!(
                near(chip.width(), text.width() + 2.0 * expected),
                "{mode:?} {keystroke}: the keycap drew {} wide against the {} its \
                 terms add up to",
                chip.width(),
                text.width() + 2.0 * expected
            );
        }
    }
}

/// The vertical ladder does not move, and this is the test that says so.
///
/// The pill's capsule is a control rung; the keycap is its content plus the
/// hairline gap the ladder's smallest step allows, top and bottom. Both are
/// stated from the constants, so an edit that reached the height ladder by
/// accident reddens here rather than in a picture nobody looks at.
///
/// This is about the *rungs*, not about where a label sits inside one. Where
/// the pill's glyphs sit is
/// [`the_pill_centres_its_label_on_the_glyphs_not_on_the_font_box`], and the
/// two are independent: moving a label within its chip must leave every figure
/// below untouched, which is most of what this test is worth.
#[test]
fn neither_chip_moves_on_the_vertical_ladder() {
    for (label, role) in PILL_LABELS {
        let capsule = drawn_pill(Mode::Light, label, role).chip;
        assert!(
            near(capsule.height(), HEIGHT_XS),
            "{label}: the capsule drew {} tall against the {HEIGHT_XS} rung it \
             sits on",
            capsule.height()
        );
    }
    for keystroke in KEYSTROKES {
        let keycap = drawn_key_chip(Mode::Light, keystroke);
        let (chip, text) = (keycap.chip, keycap.text);
        let expected = text.height() + 2.0 * SPACE_1 + 2.0 * HAIRLINE;
        assert!(
            near(chip.height(), expected),
            "{keystroke}: the keycap drew {} tall against the {expected} its \
             terms add up to",
            chip.height()
        );
    }
}

/// Geometry is not a theme artefact. The card's pixel decoding found the dark
/// golden identical to the light one box for box; this holds the two primitives
/// to that.
#[test]
fn the_two_modes_draw_the_same_boxes() {
    for (label, role) in PILL_LABELS {
        let l = drawn_pill(Mode::Light, label, role);
        let d = drawn_pill(Mode::Dark, label, role);
        let (light, light_text) = (l.chip, l.text);
        let (dark, dark_text) = (d.chip, d.text);
        assert!(
            near(light.width(), dark.width()) && near(light.height(), dark.height()),
            "{label}: the pill is {:?} in light and {:?} in dark",
            light.size(),
            dark.size()
        );
        assert!(
            near(
                light_text.left() - light.left(),
                dark_text.left() - dark.left()
            ),
            "{label}: the label sits at a different inset in the two modes"
        );
    }
    for keystroke in KEYSTROKES {
        let light = drawn_key_chip(Mode::Light, keystroke).chip;
        let dark = drawn_key_chip(Mode::Dark, keystroke).chip;
        assert!(
            near(light.width(), dark.width()) && near(light.height(), dark.height()),
            "{keystroke}: the keycap is {:?} in light and {:?} in dark",
            light.size(),
            dark.size()
        );
    }
}

// ---------------------------------------------------------------------------
// The vertical: where the glyphs sit inside the chip.
// ---------------------------------------------------------------------------

/// The pill centres its label on the glyphs it draws, not on the font box
/// those glyphs were laid out in.
///
/// A galley's box is the face's metrics — one ascent above the baseline and
/// one descent below it, identical for every string set in that face. Centring
/// it centres the metrics, and the descent is reserved whether the string
/// spends it or not: `ok` spends none and landed centred, `failing` hangs its
/// `g` into the reserved room and sat low by half of it. That is why the
/// defect read as inconsistent rather than as uniformly off — it was a
/// function of the string.
///
/// Both terms here are measured off the painted frame and neither is stated
/// from a constant: the capsule is the rect the widget filled, and the glyph
/// box is walked out of the mesh the text tessellated into. The claim is that
/// those two centres coincide, which is a relation between two drawn things
/// rather than a restatement of any expression in the widget.
///
/// Both densities, because the atlas is rebuilt per density and the room a
/// face leaves round its glyphs is not the same fraction of a point at each.
#[test]
fn the_pill_centres_its_label_on_the_glyphs_not_on_the_font_box() {
    for density in DENSITIES {
        for mode in [Mode::Light, Mode::Dark] {
            for (label, role) in PILL_LABELS {
                let pill = drawn_pill_at(mode, label, role, density);
                let off = pill.glyphs.center().y - pill.chip.center().y;
                assert!(
                    off.abs() < EPS,
                    "{mode:?} {density}x {label}: the label's glyphs are centred \
                     {off} off the capsule's centreline — the glyphs run \
                     {:?} and the capsule {:?}",
                    pill.glyphs.y_range(),
                    pill.chip.y_range()
                );
            }
        }
    }
}

/// The icon and the label answer to one vertical reference, which is the whole
/// of what "the group is centred" means for a chip that carries both.
///
/// The icon was never the wrong half — it has always been centred on the box
/// it draws in — so this reads as a test of the label, and that is exactly
/// what it is: it pins the *pair*, so neither half can be corrected in
/// isolation and leave the chip reading crooked. The icon's position is
/// recovered from the polyline points `Icon::paint` flattens its curves into,
/// which is a completely different shape kind from the text and shares no
/// reader with it.
///
/// The bound on the icon's extent is what makes the measurement addressed
/// rather than lucky: it says the strokes found really are an icon inside this
/// pill, not some other path the frame happened to draw.
#[test]
fn the_icon_and_the_label_share_the_pills_centreline() {
    for density in DENSITIES {
        for (label, role) in PILL_LABELS {
            let pill = drawn_pill_at(Mode::Light, label, role, density);

            assert!(
                pill.icon.is_positive()
                    && pill.icon.width() <= ICON_XS + EPS
                    && pill.icon.height() <= ICON_XS + EPS
                    && pill.chip.contains_rect(pill.icon),
                "{density}x {label}: {:?} is not an icon's worth of stroked path \
                 inside the capsule {:?}",
                pill.icon,
                pill.chip
            );

            let apart = pill.glyphs.center().y - pill.icon.center().y;
            assert!(
                apart.abs() < EPS,
                "{density}x {label}: the label's glyphs and the icon's strokes \
                 are {apart} apart on the vertical — one group, two references"
            );
        }
    }
}

/// A label that spends none of the descent does not move, and this is the cost
/// of the correction stated as a guard.
///
/// The two boxes coincide for such a string in the proportional face — the
/// room above its ascenders happens to be the room below its baseline — so
/// centring on the glyphs returns what centring on the font box returned, to
/// the bit. That is worth pinning for two reasons. It is the reason the change
/// is a correction and not a nudge: the labels that already read right are
/// untouched. And it is what separates centring on the glyphs from lifting
/// every label by a constant, which would centre the descenders by pushing
/// everything else up and would pass a test that only ever looked at
/// `failing`.
///
/// It says nothing about the mono face, where the two boxes do *not* coincide
/// and every keystroke moved. That is deliberate: this is a claim about one
/// face's metrics, and stating it for both would be stating something untrue.
#[test]
fn a_pill_label_with_no_descender_still_sits_where_its_font_box_put_it() {
    for density in DENSITIES {
        for mode in [Mode::Light, Mode::Dark] {
            let pill = drawn_pill_at(mode, "ok", Role::Success, density);
            let off = pill.text.center().y - pill.chip.center().y;
            assert!(
                off.abs() < EPS,
                "{mode:?} {density}x: `ok` has been moved — its font box is now \
                 {off} off the capsule's centreline, and it was on it"
            );
        }
    }
}

/// A label with nothing to ink falls back to its font box, and the fallback is
/// a real answer rather than an arithmetic accident.
///
/// `optically_centred_galley_top` has a second branch. A galley whose glyphs
/// tessellate into no triangles at all — an empty label, or one that is all
/// whitespace — has mesh bounds of [`egui::Rect::NOTHING`], whose centre is
/// not a number. There is nothing to centre on the glyphs in that case, so the
/// only defensible answer is the font box, and the branch has to both take
/// that answer and take it without letting an infinity or a NaN through into a
/// paint position.
///
/// The first assertion is what makes this a test *of the branch* and not just
/// another centring test: it establishes that the frame really did draw a text
/// run with no glyphs in it, so the second assertion is measuring the fallback
/// and cannot be quietly satisfied by the glyph path.
#[test]
fn a_label_with_nothing_to_ink_falls_back_to_its_font_box() {
    for density in DENSITIES {
        for label in ["", " ", "   "] {
            let pill = drawn_pill_at(Mode::Light, label, Role::Neutral, density);

            assert!(
                !pill.glyphs.is_positive(),
                "{density}x {label:?}: this label was supposed to ink nothing, and \
                 it drew glyphs over {:?}",
                pill.glyphs
            );
            assert!(
                pill.text.min.y.is_finite() && pill.text.max.y.is_finite(),
                "{density}x {label:?}: the label was painted at {:?} — the empty \
                 mesh bounds have leaked into the paint position",
                pill.text.y_range()
            );

            let off = pill.text.center().y - pill.chip.center().y;
            assert!(
                off.abs() < EPS,
                "{density}x {label:?}: with no glyphs to centre, the font box is \
                 the answer, and it is sitting {off} off the capsule's centreline"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The vertical: where the keystroke sits inside the keycap.
// ---------------------------------------------------------------------------

/// The four keystrokes under test really do ink boxes that start in different
/// places and end in different places.
///
/// Everything below turns on this. A set of labels that all ink the same box
/// cannot tell centring on the glyphs apart from centring on the font box —
/// the two answers differ by a constant for such a set, and a constant is
/// exactly what a wrong implementation would also produce. So this is a test of
/// the *fixture*, and it belongs beside the claims it licenses rather than in a
/// comment above them: if a face, a size or a label ever changed such that the
/// spread collapsed, the vertical tests would keep passing while quietly losing
/// the mutations they were written to catch, and nothing else would say so.
///
/// Stated as a spread rather than as figures, because the figures are font
/// metrics and this file states no font metric as an expectation.
#[test]
fn the_keystrokes_under_test_ink_boxes_that_start_and_end_differently() {
    for density in DENSITIES {
        let boxes: Vec<egui::Rect> = KEYSTROKES
            .iter()
            .map(|k| drawn_key_chip_at(Mode::Light, k, density).glyphs)
            .collect();
        let tops: Vec<f32> = boxes.iter().map(|b| b.min.y).collect();
        let bottoms: Vec<f32> = boxes.iter().map(|b| b.max.y).collect();
        let spread = |v: &[f32]| {
            v.iter().cloned().fold(f32::MIN, f32::max) - v.iter().cloned().fold(f32::MAX, f32::min)
        };
        assert!(
            spread(&tops) > EPS,
            "{density}x: every keystroke in {KEYSTROKES:?} starts its ink at the \
             same height ({tops:?}) — the set cannot see a mis-centring that \
             depends on where the ink starts"
        );
        assert!(
            spread(&bottoms) > EPS,
            "{density}x: every keystroke in {KEYSTROKES:?} ends its ink at the \
             same height ({bottoms:?}) — the set cannot see a mis-centring that \
             depends on where the ink ends"
        );
    }
}

/// The keycap centres its keystroke on the glyphs it draws, not on the font box
/// those glyphs were laid out in.
///
/// This is the pill's claim made of the second primitive that shares the
/// helper, and the mono face makes it a bigger claim than the pill's. The
/// proportional face leaves an empty band above its ascenders exactly as deep
/// as the descent it reserves below its baseline, so a string spending none of
/// the descent lands centred by coincidence and does not move. The bundled mono
/// face has no such symmetry — measured at 1.0 px/pt, a keystroke with no
/// descender sat 0.5 above the capsule's centreline and one with a descender
/// 0.5 below it, and the correction moves both.
///
/// Both terms are measured off the painted frame and neither is stated from a
/// constant: the capsule is the rect the widget filled, and the glyph box is
/// walked out of the mesh the text tessellated into. The claim is that the two
/// centres coincide, which is a relation between two drawn things rather than a
/// restatement of any expression in the widget — `key_chip` reads
/// `Galley::mesh_bounds`, and this reads the vertices.
///
/// Both densities, because the atlas is rebuilt per density and the room a face
/// leaves round its glyphs is not the same fraction of a point at each. Both
/// modes, because geometry is not a theme artefact and a claim that held in one
/// would be worth half of nothing.
#[test]
fn the_keycap_centres_its_keystroke_on_the_glyphs_not_on_the_font_box() {
    for density in DENSITIES {
        for mode in [Mode::Light, Mode::Dark] {
            for keystroke in KEYSTROKES {
                let keycap = drawn_key_chip_at(mode, keystroke, density);
                let off = keycap.glyphs.center().y - keycap.chip.center().y;
                assert!(
                    off.abs() < EPS,
                    "{mode:?} {density}x {keystroke}: the keystroke's glyphs are \
                     centred {off} off the keycap's centreline — the glyphs run \
                     {:?} and the keycap {:?}",
                    keycap.glyphs.y_range(),
                    keycap.chip.y_range()
                );
            }
        }
    }
}

/// Where the keystroke is **drawn** does not depend on which keystroke it is,
/// and it is seated as close to the centreline as the pixel grid allows.
///
/// This is the claim above one stage further down the pipeline, and the stage
/// is not free. epaint snaps a galley's origin to a whole physical pixel before
/// it tessellates (`epaint::TessellationOptions::round_text_to_pixels`), so the
/// position `key_chip` computes is rounded before a triangle exists. The
/// keycap's capsule is an *odd* number of physical pixels tall at both
/// densities — its height is a galley plus two spacing rungs plus two hairlines
/// and that is what it comes to — while its glyph box is an even number, so no
/// paint position centres the drawn glyphs. Half a physical pixel is left over
/// whatever the widget asks for.
///
/// So the bound asserted here is half a physical pixel and not zero. Zero is
/// not reachable, and a test claiming it would be claiming something the code
/// cannot do; moving the residual to zero means moving the height ladder, which
/// [`neither_chip_moves_on_the_vertical_ladder`] exists to stop happening by
/// accident.
///
/// **The first assertion is what the correction buys at this altitude**: one
/// offset for every keystroke. Under font-box centring the drawn offset is a
/// function of the string — measured at 1.0 px/pt it was -0.5 for `Esc` and
/// `Ctrl+K`, +0.5 for `Pg Dn` and +1.5 for `y` — and a keycap whose label sits
/// somewhere different depending on which key it names is the thing a reader
/// sees. Afterwards the four agree, to the bit, at both densities.
#[test]
fn the_drawn_keystroke_sits_at_one_offset_whatever_the_keystroke_is() {
    for density in DENSITIES {
        for mode in [Mode::Light, Mode::Dark] {
            let offsets: Vec<(&str, f32)> = KEYSTROKES
                .iter()
                .map(|k| {
                    let keycap = drawn_key_chip_at(mode, k, density);
                    (*k, keycap.drawn_glyphs.center().y - keycap.chip.center().y)
                })
                .collect();

            // Every figure below has to have come through the tessellator, and
            // this is what says so. epaint snaps a galley's origin to a whole
            // physical pixel and the glyph quads sit at whole-pixel offsets from
            // it, so a drawn glyph box lands on the grid by construction — while
            // the laid-out one this widget asks for does not, and cannot, for
            // the same odd-capsule reason the residual exists. A measurement
            // that quietly read layout instead of tessellation would agree with
            // everything else here and disagree with this.
            for keystroke in KEYSTROKES {
                let drawn = drawn_key_chip_at(mode, keystroke, density).drawn_glyphs;
                assert!(
                    on_a_physical_pixel(drawn.min.y, density)
                        && on_a_physical_pixel(drawn.max.y, density),
                    "{mode:?} {density}x `{keystroke}`: the drawn glyphs run {:?}, \
                     which is not on the physical pixel grid — this is a laid-out \
                     box, not a tessellated one",
                    drawn.y_range()
                );
            }

            let (first_keystroke, first) = offsets[0];
            for (keystroke, off) in &offsets {
                assert!(
                    near(*off, first),
                    "{mode:?} {density}x: `{keystroke}` is drawn {off} off the \
                     keycap's centreline against `{first_keystroke}`'s {first} — \
                     the keystroke's position is a function of the string, which \
                     is the defect"
                );
            }

            // Half a physical pixel, in points. The capsule is an odd number of
            // physical pixels tall and the glyph box an even number, so this is
            // the floor the snap imposes, not a tolerance.
            let half_a_physical_pixel = 0.5 / density;
            for (keystroke, off) in &offsets {
                assert!(
                    off.abs() <= half_a_physical_pixel + EPS,
                    "{mode:?} {density}x: `{keystroke}` is drawn {off} off the \
                     keycap's centreline, past the {half_a_physical_pixel} the \
                     pixel snap can account for"
                );
            }
        }
    }
}

/// A keystroke with nothing to ink falls back to its font box, and the fallback
/// is a real answer rather than an arithmetic accident.
///
/// `optically_centred_galley_top` has a second branch, and `key_chip` is the
/// second caller to reach it. A galley whose glyphs tessellate into no triangles
/// at all — an empty keystroke, or one that is all whitespace — has mesh bounds
/// of [`egui::Rect::NOTHING`], whose centre is not a number. There is nothing to
/// centre on the glyphs in that case, so the only defensible answer is the font
/// box, and the branch has to both take that answer and take it without letting
/// an infinity or a NaN through into a paint position.
///
/// The first assertion is what makes this a test *of the branch* and not just
/// another centring test: it establishes that the frame really did draw a text
/// run with no glyphs in it, so the assertions after it are measuring the
/// fallback and cannot be quietly satisfied by the glyph path. A keycap is the
/// harder case of the two callers, too — the pill's capsule is a fixed rung, but
/// the keycap sizes itself from the galley, so an empty keystroke is also the
/// narrowest chip the primitive ever draws.
#[test]
fn a_keystroke_with_nothing_to_ink_falls_back_to_its_font_box() {
    for density in DENSITIES {
        for mode in [Mode::Light, Mode::Dark] {
            for keystroke in ["", " ", "   "] {
                let keycap = drawn_key_chip_at(mode, keystroke, density);

                assert!(
                    !keycap.glyphs.is_positive(),
                    "{mode:?} {density}x {keystroke:?}: this keystroke was supposed \
                     to ink nothing, and it drew glyphs over {:?}",
                    keycap.glyphs
                );
                assert!(
                    keycap.text.min.y.is_finite() && keycap.text.max.y.is_finite(),
                    "{mode:?} {density}x {keystroke:?}: the keystroke was painted at \
                     {:?} — the empty mesh bounds have leaked into the paint position",
                    keycap.text.y_range()
                );

                let off = keycap.text.center().y - keycap.chip.center().y;
                assert!(
                    off.abs() < EPS,
                    "{mode:?} {density}x {keystroke:?}: with no glyphs to centre, the \
                     font box is the answer, and it is sitting {off} off the keycap's \
                     centreline"
                );
            }
        }
    }
}
