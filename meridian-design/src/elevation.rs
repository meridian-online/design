//! Elevation — the typed form of quiet chrome.
//!
//! `guidelines/identity.md`: chrome is quiet, and "if chrome competes with
//! marks for attention, chrome loses". A drop shadow is chrome competing.
//! So the rule is not "how much shadow" but **whether the thing is on the
//! working plane at all**:
//!
//! - Separation *on* the plane is a hairline. Panels, cards, toolbars, docked
//!   containers, table headers — all flat, all separated by a one-pixel
//!   border from the border scale. They cast nothing.
//! - Shadow exists only *above* the plane, for the two things that genuinely
//!   float and must be read as temporary: overlays (menus, popovers,
//!   tooltips, autocomplete) and modals.
//!
//! [`Elevation::Flat`] and [`Elevation::Raised`] therefore both return `None`
//! from [`Elevation::shadow`]. That is the point of the type: "raised" is a
//! statement about hairline treatment and stacking, not an invitation to a
//! soft shadow. If a surface seems to need a shadow to be legible, its
//! background is wrong, not its elevation.

use crate::chrome::INK_LIGHT;
use crate::colour::Rgba;

/// True black — the only paint that dims a dark plane, since every Meridian
/// grey is warm and a warm grey laid over a dark plane lightens it rather
/// than shading it. Named rather than spelled inline: a raw hex in new code
/// is the drift this crate exists to remove.
const BLACK: Rgba = Rgba::from_u8(0x00, 0x00, 0x00, 0xff);

/// A drop shadow, framework-neutral: offsets and blur in logical pixels,
/// colour as straight-alpha sRGB. No spread — a spread shadow is a glow, and
/// glows are decoration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub colour: Rgba,
}

/// Where a surface sits relative to the working plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevation {
    /// Flush with the plane: the app background, page chrome, table body.
    /// No hairline, no shadow — it *is* the plane.
    Flat,
    /// On the plane but a distinct region: panels, cards, toolbars, headers.
    /// A hairline separates it. **Casts nothing.**
    Raised,
    /// Above the plane and temporary: menus, popovers, tooltips, dropdowns.
    Overlay,
    /// Above everything, with a scrim beneath it: modals, dialogs.
    Modal,
}

impl Elevation {
    /// The shadow this elevation casts, if any. `None` for [`Self::Flat`] and
    /// [`Self::Raised`] — by design, not by omission.
    ///
    /// Shadow paint is never its own colour. In light it is the primary ink
    /// read from [`crate::chrome`] at low alpha — a shadow is the page's own
    /// ink, so a chrome re-anchor moves it too. In dark it is [`BLACK`], for
    /// the reason given at that constant. The two anchors are asserted in
    /// [`tests`] below, so neither can quietly become a fifth grey.
    pub const fn shadow(self, dark: bool) -> Option<Shadow> {
        match self {
            Self::Flat | Self::Raised => None,
            Self::Overlay => Some(Shadow {
                x: 0.0,
                y: 2.0,
                blur: 8.0,
                colour: if dark {
                    BLACK.with_alpha_u8(0x66)
                } else {
                    INK_LIGHT.ink_primary.with_alpha_u8(0x1f)
                },
            }),
            Self::Modal => Some(Shadow {
                x: 0.0,
                y: 8.0,
                blur: 24.0,
                colour: if dark {
                    BLACK.with_alpha_u8(0x8c)
                } else {
                    INK_LIGHT.ink_primary.with_alpha_u8(0x29)
                },
            }),
        }
    }

    /// Whether this elevation is separated by a hairline border. Everything
    /// except the plane itself is.
    pub const fn hairline(self) -> bool {
        !matches!(self, Self::Flat)
    }

    /// Whether a scrim dims what is behind. Modals only — an overlay that
    /// dims the app is a modal wearing an overlay's clothes.
    pub const fn scrim(self) -> bool {
        matches!(self, Self::Modal)
    }
}

/// Every elevation, in plane order.
pub const ELEVATIONS: [Elevation; 4] = [
    Elevation::Flat,
    Elevation::Raised,
    Elevation::Overlay,
    Elevation::Modal,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_working_plane_casts_nothing() {
        for dark in [false, true] {
            assert!(Elevation::Flat.shadow(dark).is_none());
            assert!(
                Elevation::Raised.shadow(dark).is_none(),
                "raised must stay flat — separation on the plane is a hairline"
            );
            assert!(Elevation::Overlay.shadow(dark).is_some());
            assert!(Elevation::Modal.shadow(dark).is_some());
        }
    }

    #[test]
    fn shadows_grow_monotonically_and_never_spread_sideways() {
        for dark in [false, true] {
            let o = Elevation::Overlay.shadow(dark).unwrap();
            let m = Elevation::Modal.shadow(dark).unwrap();
            assert_eq!(o.x, 0.0);
            assert_eq!(m.x, 0.0);
            assert!(m.y > o.y && m.blur > o.blur);
            assert!(m.colour.a > o.colour.a);
        }
    }

    /// The anchor gate: shadow paint is the chrome ink (light) or black
    /// (dark), never a restated hex. Re-anchor `chrome.rs` and the shadows
    /// follow; introduce a literal here and this reddens.
    #[test]
    fn shadow_paint_is_an_anchor_not_a_literal() {
        for e in [Elevation::Overlay, Elevation::Modal] {
            let light = e.shadow(false).unwrap().colour;
            assert!(
                light.same_paint(&INK_LIGHT.ink_primary),
                "{e:?} light shadow drifted off the primary ink anchor"
            );
            let dark = e.shadow(true).unwrap().colour;
            assert!(
                dark.same_paint(&BLACK),
                "{e:?} dark shadow must be black — a warm grey lightens a \
                 dark plane instead of shading it"
            );
            // A shadow is a wash by definition; a fully opaque one is a fill.
            assert!(light.a < 1.0 && dark.a < 1.0);
        }
    }

    #[test]
    fn only_the_modal_dims_the_app() {
        assert!(!Elevation::Flat.hairline());
        assert!(Elevation::Raised.hairline());
        assert!(!Elevation::Overlay.scrim());
        assert!(Elevation::Modal.scrim());
    }
}
