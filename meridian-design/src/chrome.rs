//! Chrome tokens — the surfaces, hairlines, and ink around the data
//! (values approved 2026-07-16, drawn from the generated gray/maritime
//! scales). Two deliberately separate groups, because they migrate in
//! separate Brightfield PRs (ROADMAP Phase 4):
//!
//! - **ink** reaches the Vello scene and therefore rendered PNGs — changing
//!   it is a sanctioned example-PNG re-baseline event.
//! - **overlay** is GPUI-side quads that never enter the Vello scene — safe
//!   to change with PNGs byte-identical.

use crate::colour::Rgba;

/// Colours that reach the rendered chart itself.
pub struct InkTokens {
    /// Chart surface (the plot background).
    pub surface: Rgba,
    /// Page plane behind the chart surface.
    pub page: Rgba,
    /// Titles, primary labels.
    pub ink_primary: Rgba,
    /// Secondary labels, legend text.
    pub ink_secondary: Rgba,
    /// Axis tick labels, muted annotations.
    pub ink_muted: Rgba,
    /// Gridline hairline.
    pub gridline: Rgba,
    /// Axis baseline / domain line.
    pub baseline: Rgba,
    /// Interactive emphasis inside the chart (hover crosshair etc.).
    pub focus: Rgba,
}

pub const INK_LIGHT: InkTokens = InkTokens {
    surface: Rgba::from_u8(0xfc, 0xfc, 0xfb, 0xff),
    page: Rgba::from_u8(0xfb, 0xfa, 0xf9, 0xff),
    ink_primary: Rgba::from_u8(0x23, 0x1f, 0x1c, 0xff),
    ink_secondary: Rgba::from_u8(0x60, 0x5c, 0x58, 0xff),
    ink_muted: Rgba::from_u8(0x8a, 0x84, 0x7f, 0xff),
    gridline: Rgba::from_u8(0xeb, 0xea, 0xe9, 0xff),
    baseline: Rgba::from_u8(0xd4, 0xd2, 0xcf, 0xff),
    focus: Rgba::from_u8(0x4b, 0x7a, 0x9b, 0xff),
};

pub const INK_DARK: InkTokens = InkTokens {
    surface: Rgba::from_u8(0x16, 0x14, 0x13, 0xff),
    page: Rgba::from_u8(0x0e, 0x0c, 0x0b, 0xff),
    ink_primary: Rgba::from_u8(0xef, 0xee, 0xec, 0xff),
    ink_secondary: Rgba::from_u8(0xb7, 0xb2, 0xae, 0xff),
    ink_muted: Rgba::from_u8(0x71, 0x6c, 0x67, 0xff),
    gridline: Rgba::from_u8(0x23, 0x22, 0x21, 0xff),
    baseline: Rgba::from_u8(0x3c, 0x3a, 0x38, 0xff),
    focus: Rgba::from_u8(0x8f, 0xc1, 0xe4, 0xff),
};

/// GPUI-side interaction quads (never in the Vello scene).
pub struct OverlayTokens {
    /// Committed brush interior wash.
    pub brush_fill: Rgba,
    /// Committed brush border.
    pub brush_border: Rgba,
    /// Keyboard focus ring.
    pub focus_ring: Rgba,
}

pub const OVERLAY_LIGHT: OverlayTokens = OverlayTokens {
    brush_fill: Rgba::from_u8(0x23, 0x1f, 0x1c, 0x1a),
    brush_border: Rgba::from_u8(0x60, 0x5c, 0x58, 0x99),
    focus_ring: Rgba::from_u8(0x4b, 0x7a, 0x9b, 0xff),
};

pub const OVERLAY_DARK: OverlayTokens = OverlayTokens {
    brush_fill: Rgba::from_u8(0xef, 0xee, 0xec, 0x1a),
    brush_border: Rgba::from_u8(0xb7, 0xb2, 0xae, 0x99),
    focus_ring: Rgba::from_u8(0x8f, 0xc1, 0xe4, 0xff),
};
