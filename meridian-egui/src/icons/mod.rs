//! The Meridian icon set: Tabler outline glyphs as committed kurbo path
//! constants (ADR 0009 — Tabler everywhere, SVG→path ingest, the invisible
//! bounding-rect path stripped).
//!
//! Icon artwork is from [Tabler Icons](https://tabler.io/icons), MIT,
//! © Paweł Kuna and contributors. The vendored path data lives in
//! `tabler.manifest` beside this module; `data.rs` holds the generated
//! constants. Both are outputs of the committed `gen-tabler-icons` bin —
//! **regenerate, never hand-edit** (the discipline the token scales already
//! use), and `tests/icon_drift.rs` fails if the two disagree.
//!
//! Rendering: a Tabler glyph is stroke-only artwork on a 24×24 grid at
//! stroke-width 2 with the stroke scaling live ([`Icon::paint`] keeps that
//! ratio at any size). The rung an icon should be drawn at comes from the
//! design tokens' icon ladder, not from ad-hoc sizes.
//!
//! ```no_run
//! # egui::__run_test_ui(|ui| {
//! use meridian_egui::icons;
//! icons::CHART_BAR.show(ui, 16.0, egui::Color32::BLACK);
//! # });
//! ```

mod data;

#[doc(hidden)]
pub mod gen;

pub use data::*;

/// The native Tabler grid: glyphs are authored on a 24×24 viewBox.
pub const GRID: f32 = 24.0;

/// The native Tabler stroke width at [`GRID`] size. Stroke width is a live
/// attribute: it scales with the glyph, so a 12 px icon is stroked at 1 px.
pub const STROKE_WIDTH: f32 = 2.0;

/// Curve-flattening tolerance in logical pixels. Well under half a pixel at
/// every rung of the icon ladder, so flattening is never the visible artefact.
const FLATTEN_TOLERANCE: f64 = 0.1;

/// One Tabler outline glyph: its icon name and its path elements on the
/// [`GRID`]. The constants in this module ([`ALL`] lists them) are generated —
/// see the module docs.
#[derive(Debug, Clone, Copy)]
pub struct Icon {
    /// The Tabler icon name (kebab-case), e.g. `"chart-bar"`.
    pub name: &'static str,
    /// The glyph's path elements on the 24×24 grid, arcs already converted to
    /// cubics by the generator's kurbo ingest.
    pub els: &'static [kurbo::PathEl],
}

impl Icon {
    /// Look an icon up by its Tabler name. [`ALL`] is sorted by name, so this
    /// is a binary search.
    #[must_use]
    pub fn by_name(name: &str) -> Option<Self> {
        ALL.binary_search_by(|icon| icon.name.cmp(name))
            .ok()
            .map(|i| ALL[i])
    }

    /// The glyph as an owned [`kurbo::BezPath`] on the native [`GRID`] — the
    /// form a kurbo/Vello consumer strokes directly.
    #[must_use]
    pub fn bez_path(&self) -> kurbo::BezPath {
        self.els.iter().copied().collect()
    }

    /// Stroke the glyph into `rect` on an egui painter in `colour`.
    ///
    /// The glyph scales uniformly from the [`GRID`] to the shorter side of
    /// `rect` and keeps Tabler's stroke-to-size ratio ([`STROKE_WIDTH`] at
    /// [`GRID`]). egui strokes are butt-capped where Tabler's are round — at
    /// icon sizes the difference is sub-pixel.
    pub fn paint(&self, painter: &egui::Painter, rect: egui::Rect, colour: egui::Color32) {
        let scale = f64::from(rect.width().min(rect.height())) / f64::from(GRID);
        let affine = kurbo::Affine::translate((f64::from(rect.left()), f64::from(rect.top())))
            * kurbo::Affine::scale(scale);
        let stroke = egui::Stroke::new(STROKE_WIDTH * scale as f32, colour);

        fn flush(
            painter: &egui::Painter,
            stroke: egui::Stroke,
            points: &mut Vec<egui::Pos2>,
            closed: bool,
        ) {
            if points.len() >= 2 {
                let points = std::mem::take(points);
                if closed {
                    painter.add(egui::Shape::closed_line(points, stroke));
                } else {
                    painter.add(egui::Shape::line(points, stroke));
                }
            } else {
                points.clear();
            }
        }

        let mut points: Vec<egui::Pos2> = Vec::new();
        kurbo::flatten(affine * self.bez_path(), FLATTEN_TOLERANCE, |el| match el {
            kurbo::PathEl::MoveTo(p) => {
                flush(painter, stroke, &mut points, false);
                points.push(egui::pos2(p.x as f32, p.y as f32));
            }
            kurbo::PathEl::LineTo(p) => {
                points.push(egui::pos2(p.x as f32, p.y as f32));
            }
            kurbo::PathEl::ClosePath => flush(painter, stroke, &mut points, true),
            // `flatten` only emits the three elements above.
            _ => {}
        });
        flush(painter, stroke, &mut points, false);
    }

    /// Allocate a `size`×`size` region in `ui` and paint the glyph into it.
    /// Non-interactive (`Sense::hover`) — wrap it yourself when an icon is a
    /// button.
    pub fn show(&self, ui: &mut egui::Ui, size: f32, colour: egui::Color32) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            self.paint(ui.painter(), rect, colour);
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_name_finds_every_icon_and_rejects_strangers() {
        for icon in ALL {
            let found = Icon::by_name(icon.name).expect("listed icon resolves");
            assert_eq!(found.name, icon.name);
        }
        assert!(Icon::by_name("no-such-glyph").is_none());
    }

    #[test]
    fn painting_is_cpu_only_and_panic_free() {
        egui::__run_test_ui(|ui| {
            for icon in ALL {
                icon.show(ui, 16.0, egui::Color32::BLACK);
            }
        });
    }
}
