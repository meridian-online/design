//! Helpers every look test shares: reading a frame's paint list, and asserting
//! what corner its shapes were drawn with.
//!
//! A test file starts with `mod common;` and takes what it needs. Not every
//! file uses every helper.
#![allow(dead_code)]

use egui::epaint::RectShape;
use egui::{Color32, CornerRadius, Rect, Stroke, StrokeKind};
use egui_kittest::Harness;
use meridian_design::colour::Rgba;
use meridian_egui::{theme, Mode};

/// Float noise tolerance and nothing else: every figure here is a whole number
/// of logical pixels, and the mutations these tests are built to catch move a
/// number by a whole pixel.
pub const EPS: f32 = 0.01;

pub fn ink(t: Rgba) -> Color32 {
    theme::to_color32(t)
}

pub fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

pub fn near_rect(a: Rect, b: Rect) -> bool {
    near(a.min.x, b.min.x)
        && near(a.min.y, b.min.y)
        && near(a.max.x, b.max.x)
        && near(a.max.y, b.max.y)
}

// ─── reading the paint list ─────────────────────────────────────────────────

/// One rect the frame painted, with everything the look is stated in.
#[derive(Clone, Copy, Debug)]
pub struct Painted {
    pub rect: Rect,
    pub fill: Color32,
    pub stroke: Stroke,
    pub kind: StrokeKind,
    pub radius: CornerRadius,
    pub blur: f32,
}

impl Painted {
    pub fn from(r: &RectShape) -> Self {
        Self {
            rect: r.rect,
            fill: r.fill,
            stroke: r.stroke,
            kind: r.stroke_kind,
            radius: r.corner_radius,
            blur: r.blur_width,
        }
    }

    /// The box everything this shape paints lies in: the fill, and the stroke
    /// on the side of the edge its kind puts it on.
    pub fn outer(&self) -> Rect {
        if self.stroke.width <= 0.0 || self.stroke.color == Color32::TRANSPARENT {
            return self.rect;
        }
        match self.kind {
            StrokeKind::Inside => self.rect,
            StrokeKind::Middle => self.rect.expand(self.stroke.width / 2.0),
            StrokeKind::Outside => self.rect.expand(self.stroke.width),
        }
    }

    pub fn is_stroked(&self) -> bool {
        self.stroke.width > 0.0 && self.stroke.color != Color32::TRANSPARENT
    }
}

/// Every rect a frame painted, in paint order, flattened out of the shape tree
/// (an `egui::Frame` nests its background under a `Shape::Vec` when it also
/// casts a shadow).
///
/// **Less the harness's own page.** A kittest harness paints the page surface
/// behind whatever it hosts, as a rect the size of the hosted content — so a
/// one-widget frame carries a rect that is the widget's own extent and is not the
/// widget's. It is the one fill in `mode` that is the page's, and it is left
/// out; nothing a primitive draws is the page's fill.
pub fn painted<S>(harness: &Harness<'_, S>, mode: Mode) -> Vec<Painted> {
    pub fn walk(shape: &egui::Shape, out: &mut Vec<Painted>) {
        match shape {
            egui::Shape::Rect(r) => out.push(Painted::from(r)),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &harness.output().shapes {
        walk(&clipped.shape, &mut out);
    }
    let page = theme::meridian_visuals(mode).panel_fill;
    out.retain(|r| r.fill != page);
    out
}

/// The rects filled with `fill`.
pub fn filled(rects: &[Painted], fill: Color32) -> Vec<Painted> {
    rects.iter().filter(|r| r.fill == fill).copied().collect()
}

/// Every rect in `rects` is drawn with the corner `radius`, as a logical pixel
/// count, on all four corners.
pub fn assert_corner(rects: &[Painted], radius: f32, what: &str) {
    assert!(
        !rects.is_empty(),
        "{what}: the frame painted no rect at all"
    );
    let want = CornerRadius::same(radius.round() as u8);
    for r in rects {
        assert_eq!(
            r.radius, want,
            "{what}: a rect at {:?} filled {:?} is drawn with corners {:?}, not {radius}",
            r.rect, r.fill, r.radius
        );
    }
}

/// Every rect in `rects` is square: a corner of zero.
pub fn assert_square(rects: &[Painted], what: &str) {
    assert_corner(rects, meridian_design::radius::NONE, what);
}
