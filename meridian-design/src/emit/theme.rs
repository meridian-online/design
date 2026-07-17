//! gpui-component `ThemeConfig` emitter (ADR 0002/0008): maps the Meridian
//! token layer onto gpui-component's theme vocabulary (schema field names as
//! of pin b7e63cc2) as a serde-compatible JSON string. Brightfield parses
//! this with `serde_json::from_str::<ThemeConfig>` and applies it via
//! `Theme::apply_config` — the whole GPUI adapter is those ~3 lines.
//!
//! Unset fields cascade to gpui-component's stock theme; the field map below
//! deliberately covers every family the Brightfield shell renders so no
//! stock blue survives. Snapshot-pinned by `tests/conformance.rs`.

use crate::colour::Rgba;
use crate::scales::*;
use crate::viz::{CATEGORICAL_DARK, CATEGORICAL_LIGHT, STATUS};

/// Theme appearance for [`theme_config`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

pub fn theme_config(mode: ThemeMode) -> String {
    let dark = mode == ThemeMode::Dark;
    // Scale accessors take Radix STEP numbers (1..=12), not indices.
    let gray = if dark { &GRAY_DARK } else { &GRAY_LIGHT };
    let mar = if dark { &MARITIME_DARK } else { &MARITIME_LIGHT };
    let red = if dark { &RED_DARK } else { &RED_LIGHT };
    let green = if dark { &GREEN_DARK } else { &GREEN_LIGHT };
    let amber = if dark { &AMBER_DARK } else { &AMBER_LIGHT };
    let blue = if dark { &BLUE_DARK } else { &BLUE_LIGHT };
    let cat = if dark { &CATEGORICAL_DARK } else { &CATEGORICAL_LIGHT };
    let g = |step: usize| gray[step - 1].hex();
    let m = |step: usize| mar[step - 1].hex();

    // Chrome anchors (chrome.rs INK tokens, restated per mode).
    let page = if dark { "#0e0c0b" } else { "#fbfaf9" };
    let surface = if dark { "#161413" } else { "#fcfcfb" };
    let ink = g(12);
    // On-solid text: light cream on maritime/red/green/blue solids in both
    // modes; warning takes dark text (bright-scale caveat, ADR 0007).
    let on_solid = "#fcfcfb";
    let on_warning = "#231f1c";

    let mut c: Vec<(&str, String)> = Vec::new();
    c.push(("background", page.into()));
    c.push(("foreground", ink.clone()));
    c.push(("muted.background", g(3)));
    c.push(("muted.foreground", g(11)));
    c.push(("accent.background", g(3)));
    c.push(("accent.foreground", ink.clone()));
    c.push(("border", g(4)));
    c.push(("input.border", g(5)));
    c.push(("ring", m(8)));
    c.push(("primary.background", m(9)));
    c.push(("primary.foreground", on_solid.into()));
    c.push(("primary.hover.background", m(10)));
    c.push(("primary.active.background", if dark { m(7) } else { m(11) }));
    c.push(("secondary.background", g(2)));
    c.push(("secondary.foreground", g(11)));
    c.push(("secondary.hover.background", g(3)));
    c.push(("secondary.active.background", g(4)));
    c.push(("danger.background", STATUS.critical.hex()));
    c.push(("danger.foreground", on_solid.into()));
    c.push(("danger.hover.background", red[9].hex()));
    c.push(("danger.active.background", red[10].hex()));
    c.push(("success.background", STATUS.good.hex()));
    c.push(("success.foreground", on_solid.into()));
    c.push(("success.hover.background", green[9].hex()));
    c.push(("success.active.background", green[10].hex()));
    c.push(("warning.background", STATUS.warning.hex()));
    c.push(("warning.foreground", on_warning.into()));
    c.push(("warning.hover.background", amber[9].hex()));
    c.push(("warning.active.background", amber[10].hex()));
    c.push(("info.background", blue[8].hex()));
    c.push(("info.foreground", on_solid.into()));
    c.push(("info.hover.background", blue[9].hex()));
    c.push(("info.active.background", blue[10].hex()));
    c.push(("sidebar.background", g(1)));
    c.push(("sidebar.foreground", ink.clone()));
    c.push(("sidebar.border", g(4)));
    c.push(("sidebar.accent.background", g(3)));
    c.push(("sidebar.accent.foreground", ink.clone()));
    c.push(("sidebar.primary.background", m(9)));
    c.push(("sidebar.primary.foreground", on_solid.into()));
    c.push(("title_bar.background", page.into()));
    c.push(("title_bar.border", g(4)));
    c.push(("tab_bar.background", g(2)));
    c.push(("tab_bar.segmented.background", g(3)));
    c.push(("tab.background", g(2)));
    c.push(("tab.foreground", g(11)));
    c.push(("tab.active.background", surface.into()));
    c.push(("tab.active.foreground", ink.clone()));
    c.push(("list.background", surface.into()));
    c.push(("list.head.background", g(2)));
    c.push(("list.even.background", g(1)));
    c.push(("list.hover.background", g(3)));
    c.push(("list.active.background", m(3)));
    c.push(("list.active.border", m(6)));
    c.push(("table.background", surface.into()));
    c.push(("table.head.background", g(2)));
    c.push(("table.head.foreground", g(11)));
    c.push(("table.foot.background", g(2)));
    c.push(("table.foot.foreground", g(11)));
    c.push(("table.even.background", g(1)));
    c.push(("table.hover.background", g(3)));
    c.push(("table.active.background", m(3)));
    c.push(("table.active.border", m(6)));
    c.push(("table.row.border", g(3)));
    c.push(("popover.background", surface.into()));
    c.push(("popover.foreground", ink.clone()));
    c.push(("selection.background", m(4)));
    c.push(("caret", ink.clone()));
    c.push(("link", if dark { m(11) } else { m(9) }));
    c.push(("link.hover", if dark { m(12) } else { m(10) }));
    c.push(("link.active", if dark { m(12) } else { m(11) }));
    c.push(("scrollbar.background", g(2)));
    c.push(("scrollbar.thumb.background", g(6)));
    c.push(("scrollbar.thumb.hover.background", g(7)));
    c.push(("window.border", g(4)));
    c.push((
        "overlay",
        if dark {
            Rgba::from_u8(0x00, 0x00, 0x00, 0x66).hex()
        } else {
            Rgba::from_u8(0x23, 0x1f, 0x1c, 0x33).hex()
        },
    ));
    c.push(("drop_target.background", {
        let s = mar[8];
        let ch = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        Rgba::from_u8(ch(s.r), ch(s.g), ch(s.b), 0x26).hex()
    }));
    c.push(("drag.border", m(8)));
    c.push(("skeleton.background", g(3)));
    c.push(("progress.bar.background", m(9)));
    c.push(("slider.background", g(5)));
    c.push(("slider.thumb.background", m(9)));
    c.push(("status_bar.background", page.into()));
    c.push(("status_bar.border", g(4)));
    c.push(("tiles.background", page.into()));
    c.push(("accordion.background", surface.into()));
    c.push(("accordion.hover.background", g(3)));
    c.push(("group_box.background", g(2)));
    c.push(("group_box.foreground", ink.clone()));
    c.push(("group_box.title.foreground", g(11)));
    c.push(("description_list.label.background", g(2)));
    c.push(("description_list.label.foreground", g(11)));
    c.push(("button.background", surface.into()));
    c.push(("button.foreground", ink.clone()));
    c.push(("button.hover.background", g(3)));
    c.push(("button.active.background", g(4)));
    c.push(("button.primary.background", m(9)));
    c.push(("button.primary.foreground", on_solid.into()));
    c.push(("button.primary.hover.background", m(10)));
    c.push(("button.primary.active.background", if dark { m(7) } else { m(11) }));
    c.push(("button.secondary.background", g(2)));
    c.push(("button.secondary.foreground", ink.clone()));
    c.push(("button.secondary.hover.background", g(3)));
    c.push(("button.secondary.active.background", g(4)));
    c.push(("button.danger.background", STATUS.critical.hex()));
    c.push(("button.danger.foreground", on_solid.into()));
    c.push(("button.danger.hover.background", red[9].hex()));
    c.push(("button.danger.active.background", red[10].hex()));
    for (i, colour) in cat.iter().take(5).enumerate() {
        // chart.1..5 — the same Harbour hookup the web's --chart-N carries.
        let key: &'static str = match i {
            0 => "chart.1",
            1 => "chart.2",
            2 => "chart.3",
            3 => "chart.4",
            _ => "chart.5",
        };
        c.push((key, colour.hex()));
    }
    c.push(("chart_bullish", STATUS.good.hex()));
    c.push(("chart_bearish", STATUS.critical.hex()));

    let mut json = String::new();
    json.push_str("{\n");
    json.push_str(&format!(
        "  \"name\": \"Meridian {}\",\n",
        if dark { "Dark" } else { "Light" }
    ));
    json.push_str(&format!(
        "  \"mode\": \"{}\",\n",
        if dark { "dark" } else { "light" }
    ));
    json.push_str("  \"font.family\": \"Inter\",\n");
    json.push_str("  \"font.size\": 12,\n");
    json.push_str("  \"mono_font.family\": \"JetBrains Mono\",\n");
    json.push_str("  \"mono_font.size\": 12,\n");
    json.push_str("  \"radius\": 6,\n");
    json.push_str("  \"radius.lg\": 8,\n");
    json.push_str("  \"colors\": {\n");
    for (i, (k, v)) in c.iter().enumerate() {
        let comma = if i + 1 == c.len() { "" } else { "," };
        json.push_str(&format!("    \"{k}\": \"{v}\"{comma}\n"));
    }
    json.push_str("  }\n}\n");
    json
}
