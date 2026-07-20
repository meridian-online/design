//! Emitters — every downstream artefact is generated from this crate and
//! pinned by a conformance test (ADR 0008): `tokens.css` for the web, and
//! the gpui-component `ThemeConfig` JSON pair for Brightfield.

mod theme;
pub use theme::{theme_config, ThemeMode};

use crate::chrome::{InkTokens, INK_DARK, INK_LIGHT};
use crate::colour::Rgba;
use crate::scales::*;
use crate::viz::*;

fn scale(css: &mut String, name: &str, s: &[Rgba; 12]) {
    for (i, c) in s.iter().enumerate() {
        css.push_str(&format!("  --m-{}-{}: {};\n", name, i + 1, c.hex()));
    }
}

fn ink(css: &mut String, t: &InkTokens) {
    css.push_str(&format!("  --m-surface: {};\n", t.surface.hex()));
    css.push_str(&format!("  --m-page: {};\n", t.page.hex()));
    css.push_str(&format!("  --m-ink: {};\n", t.ink_primary.hex()));
    css.push_str(&format!(
        "  --m-ink-secondary: {};\n",
        t.ink_secondary.hex()
    ));
    css.push_str(&format!("  --m-ink-muted: {};\n", t.ink_muted.hex()));
    css.push_str(&format!("  --m-gridline: {};\n", t.gridline.hex()));
    css.push_str(&format!("  --m-baseline: {};\n", t.baseline.hex()));
    css.push_str(&format!("  --m-focus: {};\n", t.focus.hex()));
}

fn mode_block(css: &mut String, dark: bool) {
    scale(css, "gray", if dark { &GRAY_DARK } else { &GRAY_LIGHT });
    scale(
        css,
        "maritime",
        if dark {
            &MARITIME_DARK
        } else {
            &MARITIME_LIGHT
        },
    );
    scale(css, "red", if dark { &RED_DARK } else { &RED_LIGHT });
    scale(css, "amber", if dark { &AMBER_DARK } else { &AMBER_LIGHT });
    scale(css, "green", if dark { &GREEN_DARK } else { &GREEN_LIGHT });
    scale(css, "blue", if dark { &BLUE_DARK } else { &BLUE_LIGHT });
    let cat = if dark {
        &CATEGORICAL_DARK
    } else {
        &CATEGORICAL_LIGHT
    };
    for (i, c) in cat.iter().enumerate() {
        css.push_str(&format!("  --m-cat-{}: {};\n", i + 1, c.hex()));
    }
    css.push_str(&format!(
        "  --m-null-ink: {};\n",
        if dark { NULL_INK_DARK } else { NULL_INK_LIGHT }.hex()
    ));
    css.push_str(&format!(
        "  --m-div-mid: {};\n",
        if dark {
            DIVERGING_MID_DARK
        } else {
            DIVERGING_MID_LIGHT
        }
        .hex()
    ));
    ink(css, if dark { &INK_DARK } else { &INK_LIGHT });
    semantic_block(css, dark);
}

/// Format a logical-pixel dimension: integral values lose their `.0`, so the
/// artefact reads like hand-written CSS and diffs stay legible.
fn px(v: f32) -> String {
    if v.fract() == 0.0 {
        format!("{}px", v as i64)
    } else {
        format!("{v}px")
    }
}

fn decl(css: &mut String, name: &str, value: &str) {
    css.push_str(&format!("  --m-{name}: {value};\n"));
}

/// Mode-invariant, non-colour tokens: the box model, the type sizes, the
/// motion budget. Emitted once in `:root` — none of them changes with theme.
fn geometry(css: &mut String) {
    use crate::control::*;
    use crate::focus;
    use crate::motion;
    use crate::radius;
    use crate::spacing::*;
    use crate::typography::{CHART_LABEL_SIZE, MONO_FAMILY, SANS_FAMILY, UI_SIZE};

    decl(css, "font-sans", &format!("\"{SANS_FAMILY}\""));
    decl(css, "font-mono", &format!("\"{MONO_FAMILY}\""));
    decl(css, "font-size-ui", &px(UI_SIZE));
    decl(css, "font-size-chart", &px(CHART_LABEL_SIZE));

    decl(css, "radius-none", &px(radius::NONE));
    decl(css, "radius-chip", &px(radius::CHIP));
    decl(css, "radius-control", &px(radius::CONTROL));
    decl(css, "radius-panel", &px(radius::PANEL));
    decl(css, "radius-full", &px(radius::FULL));

    for (i, s) in SPACE.iter().enumerate() {
        decl(css, &format!("space-{i}"), &px(*s));
    }
    decl(css, "panel-padding", &px(PANEL_PADDING));
    decl(css, "section-gap", &px(SECTION_GAP));
    decl(css, "pane-gap", &px(PANE_GAP));
    decl(css, "icon-label-gap", &px(ICON_LABEL_GAP));
    decl(css, "control-gap", &px(CONTROL_GAP));
    decl(css, "modal-padding", &px(MODAL_PADDING));
    decl(css, "modal-width-narrow", &px(MODAL_WIDTH_NARROW));
    decl(css, "modal-width-default", &px(MODAL_WIDTH_DEFAULT));

    decl(css, "row-dense", &px(ROW_DENSE));
    decl(css, "row-grid", &px(ROW_GRID));
    decl(css, "row-preview", &px(ROW_PREVIEW));
    decl(css, "row-comfortable", &px(ROW_COMFORTABLE));

    decl(css, "control-xs", &px(HEIGHT_XS));
    decl(css, "control-sm", &px(HEIGHT_SM));
    decl(css, "control-md", &px(HEIGHT_MD));
    decl(css, "control-lg", &px(HEIGHT_LG));

    decl(css, "icon-xs", &px(ICON_XS));
    decl(css, "icon-sm", &px(ICON_SM));
    decl(css, "icon-md", &px(ICON_MD));
    decl(css, "icon-lg", &px(ICON_LG));
    decl(css, "icon-xl", &px(ICON_XL));

    decl(css, "focus-ring-width", &px(focus::RING_WIDTH));
    decl(css, "focus-ring-offset", &px(focus::RING_OFFSET));
    decl(css, "focus-ring-inset", &px(focus::RING_INSET));
    decl(css, "focus-ring-bleed", &px(focus::RING_BLEED));

    decl(css, "motion-spatial", &format!("{}ms", motion::SPATIAL_MS));
    decl(
        css,
        "motion-animation-time",
        &format!("{}s", motion::ANIMATION_TIME),
    );
}

/// The semantic layer, per mode. Names mirror the Rust field paths so a
/// reviewer can move between the two without a lookup table.
fn semantic_block(css: &mut String, dark: bool) {
    use crate::elevation::Elevation;
    use crate::semantic::{semantic, Role};

    let s = semantic(dark);

    decl(css, "surface-app", &s.surfaces.app.hex());
    decl(css, "surface-raised", &s.surfaces.raised.hex());
    decl(css, "surface-sunken", &s.surfaces.sunken.hex());
    decl(css, "surface-overlay", &s.surfaces.overlay.hex());
    decl(css, "surface-sidebar", &s.surfaces.sidebar.hex());
    decl(css, "surface-header", &s.surfaces.header.hex());
    decl(css, "surface-scrim", &s.surfaces.scrim.hex());

    decl(css, "border-subtle", &s.borders.subtle.hex());
    decl(css, "border-default", &s.borders.default_.hex());
    decl(css, "border-strong", &s.borders.strong.hex());
    decl(css, "border-control", &s.borders.control.hex());
    decl(css, "border-focus", &s.borders.focus.hex());
    decl(css, "border-divider", &s.borders.divider.hex());

    decl(css, "text-primary", &s.text.primary.hex());
    decl(css, "text-secondary", &s.text.secondary.hex());
    decl(css, "text-muted", &s.text.muted.hex());
    decl(css, "text-placeholder", &s.text.placeholder.hex());
    decl(css, "text-disabled", &s.text.disabled.hex());
    decl(css, "text-on-solid", &s.text.on_solid.hex());
    decl(css, "text-on-solid-bright", &s.text.on_solid_bright.hex());
    decl(css, "text-link", &s.text.link.hex());
    decl(css, "text-link-hover", &s.text.link_hover.hex());
    decl(css, "text-link-active", &s.text.link_active.hex());

    const STATES: [&str; 6] = ["", "-hover", "-active", "-selected", "-focus", "-disabled"];
    for role in Role::ALL {
        let rc = s.role(role);
        for (channel, set) in [
            ("bg", rc.background),
            ("fg", rc.foreground),
            ("border", rc.border),
        ] {
            for (state, colour) in STATES.iter().zip(set.all()) {
                decl(
                    css,
                    &format!("{}-{channel}{state}", role.name()),
                    &colour.hex(),
                );
            }
        }
    }

    decl(css, "rows-bg", &s.rows.background.hex());
    decl(css, "rows-head-bg", &s.rows.header_background.hex());
    decl(css, "rows-head-fg", &s.rows.header_foreground.hex());
    decl(css, "rows-foot-bg", &s.rows.footer_background.hex());
    decl(css, "rows-foot-fg", &s.rows.footer_foreground.hex());
    decl(css, "rows-stripe-bg", &s.rows.stripe_background.hex());
    decl(css, "rows-hover-bg", &s.rows.hover_background.hex());
    decl(css, "rows-selected-bg", &s.rows.selected_background.hex());
    decl(css, "rows-selected-border", &s.rows.selected_border.hex());
    decl(css, "rows-border", &s.rows.row_border.hex());

    decl(css, "tabs-bar-bg", &s.tabs.bar_background.hex());
    decl(css, "tabs-segmented-bg", &s.tabs.segmented_background.hex());
    decl(css, "tabs-bg", &s.tabs.background.hex());
    decl(css, "tabs-fg", &s.tabs.foreground.hex());
    decl(css, "tabs-active-bg", &s.tabs.active_background.hex());
    decl(css, "tabs-active-fg", &s.tabs.active_foreground.hex());

    decl(css, "scrollbar-track", &s.scrollbar.track.hex());
    decl(css, "scrollbar-thumb", &s.scrollbar.thumb.hex());
    decl(css, "scrollbar-thumb-hover", &s.scrollbar.thumb_hover.hex());

    decl(css, "editor-caret", &s.editor.caret.hex());
    decl(css, "editor-selection", &s.editor.selection.hex());

    decl(css, "drop-target", &s.drag_drop.drop_target.hex());
    decl(css, "drag-border", &s.drag_drop.drag_border.hex());

    decl(css, "skeleton", &s.feedback.skeleton.hex());
    decl(css, "progress-track", &s.feedback.progress_track.hex());
    decl(css, "progress-bar", &s.feedback.progress_bar.hex());
    decl(css, "slider-track", &s.feedback.slider_track.hex());
    decl(css, "slider-thumb", &s.feedback.slider_thumb.hex());

    let c = &s.containers;
    decl(css, "sidebar-bg", &c.sidebar_background.hex());
    decl(css, "sidebar-border", &c.sidebar_border.hex());
    decl(css, "title-bar-bg", &c.title_bar_background.hex());
    decl(css, "status-bar-bg", &c.status_bar_background.hex());
    decl(css, "tiles-bg", &c.tiles_background.hex());
    decl(css, "group-box-bg", &c.group_box_background.hex());
    decl(
        css,
        "group-box-title-fg",
        &c.group_box_title_foreground.hex(),
    );
    decl(
        css,
        "description-label-bg",
        &c.description_label_background.hex(),
    );
    decl(
        css,
        "description-label-fg",
        &c.description_label_foreground.hex(),
    );
    decl(css, "accordion-bg", &c.accordion_background.hex());
    decl(
        css,
        "accordion-hover-bg",
        &c.accordion_hover_background.hex(),
    );
    decl(css, "popover-bg", &c.popover_background.hex());
    decl(css, "window-border", &c.window_border.hex());

    // Elevation: only the two planes above the working plane cast anything,
    // so only two shadow tokens exist. `Flat` and `Raised` are absent on
    // purpose — see `elevation.rs`.
    for (name, elevation) in [
        ("shadow-overlay", Elevation::Overlay),
        ("shadow-modal", Elevation::Modal),
    ] {
        let sh = elevation.shadow(dark).expect("overlay planes cast");
        decl(
            css,
            name,
            &format!(
                "{} {} {} {}",
                px(sh.x),
                px(sh.y),
                px(sh.blur),
                sh.colour.hex()
            ),
        );
    }
}

/// Emit the CSS custom-properties artefact (light `:root`, dark under
/// `.dark`, matching the web repo's theme class). Deterministic by
/// construction; `tests/conformance.rs` pins the exact output.
pub fn tokens_css() -> String {
    let mut css = String::new();
    css.push_str("/* Generated by meridian-design — do not edit by hand. */\n");
    css.push_str(":root {\n");
    css.push_str(&format!(
        "  --meridian-maritime: {};\n",
        crate::MARITIME.hex()
    ));
    mode_block(&mut css, false);
    // Mode-invariant values.
    for (i, c) in SEQUENTIAL_MERIDIAN.iter().enumerate() {
        css.push_str(&format!("  --m-seq-{}: {};\n", 100 + i * 50, c.hex()));
    }
    for (i, c) in DIVERGING_BLUE_ARM.iter().enumerate() {
        css.push_str(&format!("  --m-div-blue-{}: {};\n", i + 1, c.hex()));
    }
    for (i, c) in DIVERGING_RED_ARM.iter().enumerate() {
        css.push_str(&format!("  --m-div-red-{}: {};\n", i + 1, c.hex()));
    }
    css.push_str(&format!("  --m-status-good: {};\n", STATUS.good.hex()));
    css.push_str(&format!(
        "  --m-status-warning: {};\n",
        STATUS.warning.hex()
    ));
    css.push_str(&format!(
        "  --m-status-serious: {};\n",
        STATUS.serious.hex()
    ));
    css.push_str(&format!(
        "  --m-status-critical: {};\n",
        STATUS.critical.hex()
    ));
    geometry(&mut css);
    css.push_str("}\n.dark {\n");
    mode_block(&mut css, true);
    css.push_str("}\n");
    css
}
