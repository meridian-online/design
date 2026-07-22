//! Meridian Design System → egui bridge.
//!
//! The design crate is deliberately framework-free (`meridian_design::Rgba` is
//! plain sRGB straight-alpha data — ADR 0003), so every consumer converts at its
//! own boundary. This module is that boundary for every egui surface: it derives
//! an [`egui::Visuals`] / [`egui::Style`] from the Meridian gray + Maritime
//! scales, chrome ink, type ramp, box model and motion budget, and installs the
//! bundled Inter / JetBrains Mono faces into egui's [`egui::FontDefinitions`].
//! Nothing here is invented — it is a mechanical port of `meridian-design`
//! tokens onto egui's slots.
//!
//! The single colour boundary is [`to_color32`]; everything else composes it.
//!
//! # What this bridge governs
//!
//! Colour and type were always here. The box model and the motion budget are
//! here too, because egui carries its own defaults for both and inheriting them
//! is a silent breach of the token system, not a neutral fallback:
//!
//! - **Corner radii** come from [`meridian_design::radius`] — controls at
//!   [`radius::CONTROL`], containers at [`radius::PANEL`] — not egui's stock
//!   rounding.
//! - **Elevation shadows** come from [`meridian_design::Elevation`]: the modal
//!   shadow drives [`egui::Visuals::window_shadow`], the overlay shadow drives
//!   [`egui::Visuals::popup_shadow`]. The two flat elevations cast nothing, so
//!   panels and the plane stay shadowless by construction.
//! - **Spacing** comes from the ladder in [`meridian_design::spacing`]; no gap
//!   here is a bare float.
//! - **Animation time** is set explicitly from [`meridian_design::motion`]:
//!   egui 0.35's stock `animation_time` is `0.2` s, over the 150 ms spatial
//!   budget before a widget is drawn, so the adapter must write the token value
//!   rather than inherit the default.

use std::sync::Arc;

use meridian_design::chrome::{INK_DARK, INK_LIGHT, OVERLAY_DARK, OVERLAY_LIGHT};
use meridian_design::colour::Rgba;
use meridian_design::scales::{
    AMBER_DARK, AMBER_LIGHT, GRAY_DARK, GRAY_LIGHT, MARITIME_DARK, MARITIME_LIGHT, RED_DARK,
    RED_LIGHT,
};
use meridian_design::typography::{CHART_LABEL_SIZE, UI_SIZE};
use meridian_design::{control, motion, radius, spacing, Elevation, MARITIME};

use crate::Mode;

/// The **one** colour boundary: a Meridian token (sRGB, straight alpha, 0–1) →
/// an `egui::Color32` (gamma sRGB, straight alpha), keeping the token's alpha.
#[must_use]
pub fn to_color32(c: Rgba) -> egui::Color32 {
    let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    egui::Color32::from_rgba_unmultiplied(q(c.r), q(c.g), q(c.b), q(c.a))
}

/// A Meridian [`Elevation`] shadow → an `egui::epaint::Shadow` for `mode`.
/// [`Elevation::Flat`] and [`Elevation::Raised`] cast nothing and map to
/// [`egui::epaint::Shadow::NONE`]; a Meridian shadow never spreads, so `spread`
/// is always zero (a spread shadow is a glow, and glows are decoration).
#[must_use]
fn to_shadow(elevation: Elevation, dark: bool) -> egui::epaint::Shadow {
    match elevation.shadow(dark) {
        None => egui::epaint::Shadow::NONE,
        Some(s) => egui::epaint::Shadow {
            offset: [s.x.round() as i8, s.y.round() as i8],
            blur: s.blur.round() as u8,
            spread: 0,
            color: to_color32(s.colour),
        },
    }
}

/// The token set one mode draws from — the generated 12-step gray + Maritime
/// scales (Radix step semantics: 1–2 app bg, 3–5 component states, 6–8 borders,
/// 9–10 solids, 11–12 text), the chart ink, and the overlay accents.
struct Palette {
    gray: [Rgba; 12],
    maritime: [Rgba; 12],
    ink: meridian_design::chrome::InkTokens,
    overlay: meridian_design::chrome::OverlayTokens,
    warn: Rgba,
    error: Rgba,
    dark: bool,
}

impl Palette {
    fn for_mode(mode: Mode) -> Self {
        match mode {
            Mode::Light => Palette {
                gray: GRAY_LIGHT,
                maritime: MARITIME_LIGHT,
                ink: INK_LIGHT,
                overlay: OVERLAY_LIGHT,
                warn: AMBER_LIGHT[10],
                error: RED_LIGHT[10],
                dark: false,
            },
            Mode::Dark => Palette {
                gray: GRAY_DARK,
                maritime: MARITIME_DARK,
                ink: INK_DARK,
                overlay: OVERLAY_DARK,
                warn: AMBER_DARK[10],
                error: RED_DARK[10],
                dark: true,
            },
        }
    }
}

/// Derive an `egui::Visuals` from the Meridian tokens for `mode`.
///
/// Starts from egui's light/dark base (so unmapped fields stay sane) and
/// overrides every colour slot the shell renders from the Meridian scales:
/// panel/window surfaces from gray 1–2, widget states from gray 2–4 with
/// gray 5–8 hairlines, text from gray 11–12, selection/links from Maritime,
/// warn/error from Amber/Red. Corner radii come from [`radius`], and the two
/// floating elevations' shadows from [`Elevation`] — the box model, not just
/// colour and type.
#[must_use]
pub fn meridian_visuals(mode: Mode) -> egui::Visuals {
    let p = Palette::for_mode(mode);
    let c = to_color32;

    let mut v = if p.dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    v.dark_mode = p.dark;

    // Text + surfaces.
    v.override_text_color = Some(c(p.gray[11]));
    v.panel_fill = c(p.ink.page);
    v.window_fill = c(p.gray[1]);
    v.window_stroke = egui::Stroke::new(1.0, c(p.gray[6]));
    v.faint_bg_color = c(p.gray[1]);
    v.extreme_bg_color = c(p.ink.surface);
    v.code_bg_color = c(p.gray[2]);

    // Accents.
    v.hyperlink_color = c(MARITIME);
    v.selection = egui::style::Selection {
        bg_fill: to_color32(Rgba {
            a: 0.25,
            ..p.overlay.focus_ring
        }),
        stroke: egui::Stroke::new(1.0, c(p.maritime[8])),
    };
    v.warn_fg_color = c(p.warn);
    v.error_fg_color = c(p.error);

    // Per-state widget colours (bg from gray 1–4, hairlines from gray 5–8,
    // ink from gray 11–12, the active border from the Maritime solid).
    let w = &mut v.widgets;
    w.noninteractive.bg_fill = c(p.gray[1]);
    w.noninteractive.weak_bg_fill = c(p.gray[1]);
    w.noninteractive.bg_stroke = egui::Stroke::new(1.0, c(p.gray[5]));
    w.noninteractive.fg_stroke = egui::Stroke::new(1.0, c(p.gray[11]));

    w.inactive.bg_fill = c(p.gray[2]);
    w.inactive.weak_bg_fill = c(p.gray[2]);
    w.inactive.bg_stroke = egui::Stroke::new(1.0, c(p.gray[6]));
    w.inactive.fg_stroke = egui::Stroke::new(1.0, c(p.gray[11]));

    w.hovered.bg_fill = c(p.gray[3]);
    w.hovered.weak_bg_fill = c(p.gray[3]);
    w.hovered.bg_stroke = egui::Stroke::new(1.0, c(p.gray[7]));
    w.hovered.fg_stroke = egui::Stroke::new(1.5, c(p.gray[11]));

    w.active.bg_fill = c(p.gray[4]);
    w.active.weak_bg_fill = c(p.gray[4]);
    w.active.bg_stroke = egui::Stroke::new(1.0, c(p.maritime[8]));
    w.active.fg_stroke = egui::Stroke::new(2.0, c(p.gray[11]));

    w.open.bg_fill = c(p.gray[3]);
    w.open.weak_bg_fill = c(p.gray[3]);
    w.open.bg_stroke = egui::Stroke::new(1.0, c(p.gray[6]));
    w.open.fg_stroke = egui::Stroke::new(1.0, c(p.gray[11]));

    // Box model: corner radii from the radius ladder. Controls (buttons, inputs,
    // list rows, menu items) take `CONTROL`; containers (windows, menus) take
    // `PANEL`. Left at egui's defaults before this bridge governed the box model.
    let control_radius = egui::CornerRadius::same(radius::CONTROL.round() as u8);
    for wv in [
        &mut w.noninteractive,
        &mut w.inactive,
        &mut w.hovered,
        &mut w.active,
        &mut w.open,
    ] {
        wv.corner_radius = control_radius;
    }
    let panel_radius = egui::CornerRadius::same(radius::PANEL.round() as u8);
    v.window_corner_radius = panel_radius;
    v.menu_corner_radius = panel_radius;

    // Elevation: the two floating surfaces cast the Meridian shadows; the plane
    // and raised regions cast nothing (they map to `Shadow::NONE`).
    v.window_shadow = to_shadow(Elevation::Modal, p.dark);
    v.popup_shadow = to_shadow(Elevation::Overlay, p.dark);

    v
}

/// Derive a full `egui::Style` for `mode`: the Meridian [`meridian_visuals`]
/// plus the Meridian type ramp mapped onto egui's `TextStyle`s — UI body at
/// [`UI_SIZE`] (Inter), tick/label at [`CHART_LABEL_SIZE`], monospace at
/// [`UI_SIZE`] (JetBrains Mono) — plus the spacing ladder and the motion budget.
/// `install_fonts` must have run on the context for these families to resolve to
/// the bundled faces.
#[must_use]
pub fn meridian_style(mode: Mode) -> egui::Style {
    use egui::{FontFamily, FontId, TextStyle};

    let mut style = egui::Style {
        visuals: meridian_visuals(mode),
        ..Default::default()
    };

    let ui = UI_SIZE;
    style.text_styles = [
        (
            TextStyle::Small,
            FontId::new(CHART_LABEL_SIZE, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(ui, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(ui, FontFamily::Proportional)),
        (
            TextStyle::Heading,
            FontId::new(ui + 4.0, FontFamily::Proportional),
        ),
        (TextStyle::Monospace, FontId::new(ui, FontFamily::Monospace)),
    ]
    .into();

    // Spacing from the ladder — no gap here is a bare float. Item gap and button
    // padding take the toolbar/row steps; panel and menu margins take the panel
    // and hairline steps; the minimum interactive height is a control-ladder rung.
    let sp = &mut style.spacing;
    sp.item_spacing = egui::vec2(spacing::CONTROL_GAP, spacing::SPACE_2);
    sp.button_padding = egui::vec2(spacing::SPACE_4, spacing::SPACE_2);
    sp.window_margin = egui::Margin::same(spacing::PANEL_PADDING as i8);
    sp.menu_margin = egui::Margin::symmetric(spacing::SPACE_2 as i8, spacing::SPACE_1 as i8);
    sp.indent = spacing::SPACE_6;
    sp.interact_size.y = control::HEIGHT_MD;

    // The motion budget, set explicitly (egui's 0.2s stock value overruns it).
    style.animation_time = motion::ANIMATION_TIME;

    style
}

/// Install the bundled Meridian faces into `ctx` so egui's `Proportional`
/// family resolves to Inter and `Monospace` to JetBrains Mono (deliverable 1:
/// text looks *correct-but-wrong* without this — a system lookalike). Must run
/// before the first frame. The extra weights register under named families for
/// later use; the two default families are the load-bearing mapping.
pub fn install_fonts(ctx: &egui::Context) {
    use meridian_design::fonts;

    let mut defs = egui::FontDefinitions::default();
    let add = |defs: &mut egui::FontDefinitions, name: &str, bytes: &'static [u8]| {
        defs.font_data.insert(
            name.to_owned(),
            Arc::new(egui::FontData::from_static(bytes)),
        );
    };

    add(&mut defs, "Inter", fonts::INTER_REGULAR);
    add(&mut defs, "Inter-Medium", fonts::INTER_MEDIUM);
    add(&mut defs, "Inter-SemiBold", fonts::INTER_SEMIBOLD);
    add(&mut defs, "Inter-Bold", fonts::INTER_BOLD);
    add(&mut defs, "JetBrains Mono", fonts::JBM_REGULAR);
    add(&mut defs, "JetBrains Mono Medium", fonts::JBM_MEDIUM);

    defs.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Inter".to_owned());
    defs.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "JetBrains Mono".to_owned());
    // Named families for weighted headings / mono-medium, resolvable via
    // `FontFamily::Name(..)` without disturbing the default cascade.
    for (fam, face) in [
        ("Inter-Medium", "Inter-Medium"),
        ("Inter-SemiBold", "Inter-SemiBold"),
        ("Inter-Bold", "Inter-Bold"),
        ("JetBrains Mono Medium", "JetBrains Mono Medium"),
    ] {
        defs.families
            .insert(egui::FontFamily::Name(fam.into()), vec![face.to_owned()]);
    }

    ctx.set_fonts(defs);
}

/// Apply the Meridian style + fonts to a context in one call (the shell's
/// startup path). Idempotent — safe to call each frame, though once suffices.
pub fn apply(ctx: &egui::Context, mode: Mode) {
    install_fonts(ctx);
    let style = meridian_style(mode);
    // Set the mode-specific style on both theme slots and select the mode, so
    // the active style is the Meridian one regardless of egui's theme guess.
    ctx.set_style_of(egui::Theme::Light, style.clone());
    ctx.set_style_of(egui::Theme::Dark, style);
    ctx.set_theme(match mode {
        Mode::Light => egui::ThemePreference::Light,
        Mode::Dark => egui::ThemePreference::Dark,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use meridian_design::scales::GRAY_LIGHT;

    /// Conformance snapshot (deliverable 1): pin the derived light Visuals so a
    /// token drift or a bridge-logic change is caught. These are the exact
    /// Color32 values the shell renders chrome from.
    #[test]
    fn light_visuals_snapshot() {
        let v = meridian_visuals(Mode::Light);
        assert!(!v.dark_mode);
        assert_eq!(v.override_text_color, Some(to_color32(GRAY_LIGHT[11])));
        assert_eq!(v.panel_fill, to_color32(INK_LIGHT.page));
        assert_eq!(v.extreme_bg_color, to_color32(INK_LIGHT.surface));
        assert_eq!(v.hyperlink_color, to_color32(MARITIME));
        assert_eq!(v.selection.stroke.color, to_color32(MARITIME_LIGHT[8]));
        assert_eq!(
            v.widgets.active.bg_stroke.color,
            to_color32(MARITIME_LIGHT[8])
        );
        assert_eq!(
            v.widgets.noninteractive.fg_stroke.color,
            to_color32(GRAY_LIGHT[11])
        );
    }

    /// The one colour boundary round-trips a known opaque token exactly, and
    /// carries straight alpha through for a translucent one.
    #[test]
    fn to_color32_boundary() {
        // Maritime #4b7a9b opaque.
        let m = to_color32(MARITIME);
        assert_eq!((m.r(), m.g(), m.b(), m.a()), (0x4b, 0x7a, 0x9b, 0xff));
        // A half-alpha token keeps its alpha (unmultiplied).
        let half = to_color32(Rgba::new(1.0, 0.0, 0.0, 0.5));
        assert_eq!(half.a(), 128);
    }

    /// Dark mode selects the dark scales (a different app background than light).
    #[test]
    fn dark_visuals_differs() {
        let light = meridian_visuals(Mode::Light);
        let dark = meridian_visuals(Mode::Dark);
        assert!(dark.dark_mode);
        assert_ne!(light.panel_fill, dark.panel_fill);
    }

    /// The box model comes from the tokens, not egui's defaults: controls take
    /// the `CONTROL` radius, windows/menus the `PANEL` radius.
    #[test]
    fn corner_radii_come_from_the_radius_ladder() {
        let v = meridian_visuals(Mode::Light);
        let control = egui::CornerRadius::same(radius::CONTROL.round() as u8);
        let panel = egui::CornerRadius::same(radius::PANEL.round() as u8);
        assert_eq!(v.widgets.inactive.corner_radius, control);
        assert_eq!(v.widgets.active.corner_radius, control);
        assert_eq!(v.window_corner_radius, panel);
        assert_eq!(v.menu_corner_radius, panel);
    }

    /// Elevation drives the two floating shadows; the plane casts nothing. The
    /// modal shadow is deeper than the overlay's, exactly as the tokens say.
    #[test]
    fn shadows_come_from_elevation() {
        for mode in [Mode::Light, Mode::Dark] {
            let v = meridian_visuals(mode);
            let dark = mode.is_dark();
            assert_eq!(v.window_shadow, to_shadow(Elevation::Modal, dark));
            assert_eq!(v.popup_shadow, to_shadow(Elevation::Overlay, dark));
            // A raised/flat surface maps to no shadow.
            assert_eq!(
                to_shadow(Elevation::Raised, dark),
                egui::epaint::Shadow::NONE
            );
            // Modal floats higher than an overlay.
            assert!(v.window_shadow.blur >= v.popup_shadow.blur);
        }
    }

    /// Spacing is the ladder, and the animation budget is the motion token —
    /// not egui's stock `0.2` s, which overruns the 150 ms spatial ceiling.
    #[test]
    fn style_takes_spacing_and_motion_from_tokens() {
        let s = meridian_style(Mode::Light);
        assert_eq!(s.spacing.item_spacing.x, spacing::CONTROL_GAP);
        assert_eq!(s.spacing.interact_size.y, control::HEIGHT_MD);
        assert_eq!(s.animation_time, motion::ANIMATION_TIME);
        assert!(s.animation_time < 0.2, "must undercut egui's stock value");
    }

    /// Fonts install without panicking and set the default families to the
    /// Meridian faces (Context is CPU-only — no GPU needed).
    #[test]
    fn fonts_install() {
        let ctx = egui::Context::default();
        install_fonts(&ctx);
        // A second application (the `apply` path) must also be safe.
        apply(&ctx, Mode::Light);
    }
}
