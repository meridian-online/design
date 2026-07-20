//! Framework-neutral colour types.

/// An sRGB colour with straight (non-premultiplied) alpha, components in
/// `0.0..=1.0`. Deliberately not `gpui::Hsla`, not `peniko::Color`: consumers
/// convert at their own boundary (ADR 0003).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Construct from 8-bit channels. Token values defined from hex use this
    /// so [`Rgba::hex`] round-trips exactly.
    pub const fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// The same colour at a different opacity.
    ///
    /// Every wash in this system — a scrim, a drop shadow, a drop-target
    /// tint — is an anchor colour at low alpha. This exists so those slots
    /// name *which* anchor they are
    /// (`chrome::INK_LIGHT.ink_primary.with_alpha_u8(0x33)`) instead of
    /// restating its hex, which is exactly how the two silently drift apart.
    pub const fn with_alpha_u8(self, a: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: a as f32 / 255.0,
        }
    }

    /// Whether two colours are the same paint ignoring opacity — the question
    /// a gate asks of a wash ("is this scrim the primary ink?").
    pub fn same_paint(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }

    /// CSS hex form: `#rrggbb`, or `#rrggbbaa` when alpha < 1.
    pub fn hex(&self) -> String {
        let ch = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        if self.a >= 1.0 {
            format!("#{:02x}{:02x}{:02x}", ch(self.r), ch(self.g), ch(self.b))
        } else {
            format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                ch(self.r),
                ch(self.g),
                ch(self.b),
                ch(self.a)
            )
        }
    }
}

/// Interaction states as first-class token slots. Every target wants them
/// flat — a theme config wants separate fields, an immediate-mode `Visuals`
/// wants separate buckets — so the crate never models a cascade (ADR 0003).
///
/// Six states, and the two additions are the ones frameworks reliably fail to
/// supply: `selected` (persistent, unlike `active`, which is the transient
/// press) and `focus` (keyboard, which the immediate-mode desktop framework
/// has no bucket for at all — see [`crate::focus`]). A `StateSet` is
/// instantiated per role in [`crate::semantic`]; nothing else should build one
/// ad hoc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateSet {
    /// At rest.
    pub base: Rgba,
    /// Pointer over the target.
    pub hover: Rgba,
    /// Pressed — transient, lasts as long as the button is down.
    pub active: Rgba,
    /// Selected — persistent, survives the pointer leaving.
    pub selected: Rgba,
    /// Keyboard focus. For a background slot this is usually equal to `base`:
    /// the ring carries focus, the fill does not move.
    pub focus: Rgba,
    /// Non-interactive.
    pub disabled: Rgba,
}

impl StateSet {
    /// Every state in a fixed order, so gates and emitters can visit all of
    /// them without hand-listing fields.
    pub const fn all(&self) -> [Rgba; 6] {
        [
            self.base,
            self.hover,
            self.active,
            self.selected,
            self.focus,
            self.disabled,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trips_u8_construction() {
        assert_eq!(Rgba::from_u8(0x4b, 0x7a, 0x9b, 0xff).hex(), "#4b7a9b");
        assert_eq!(Rgba::from_u8(0x4b, 0x7a, 0x9b, 0x80).hex(), "#4b7a9b80");
    }

    #[test]
    fn with_alpha_keeps_the_paint_and_moves_only_the_opacity() {
        let anchor = Rgba::from_u8(0x23, 0x1f, 0x1c, 0xff);
        let wash = anchor.with_alpha_u8(0x33);
        assert!(wash.same_paint(&anchor));
        assert_eq!(wash.hex(), "#231f1c33");
        assert_eq!(wash.with_alpha_u8(0xff), anchor);
        assert!(!wash.same_paint(&Rgba::from_u8(0x4b, 0x7a, 0x9b, 0x33)));
    }
}
