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

/// Interaction states as first-class token slots. Both current targets want
/// them flat — gpui-component as separate theme fields, Masonry as separate
/// named property types — so the crate never models a cascade (ADR 0003).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateSet {
    pub base: Rgba,
    pub hover: Rgba,
    pub active: Rgba,
    pub disabled: Rgba,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trips_u8_construction() {
        assert_eq!(Rgba::from_u8(0x4b, 0x7a, 0x9b, 0xff).hex(), "#4b7a9b");
        assert_eq!(Rgba::from_u8(0x4b, 0x7a, 0x9b, 0x80).hex(), "#4b7a9b80");
    }
}
