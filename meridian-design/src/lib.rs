//! Meridian design tokens.
//!
//! The single definition point for how Meridian looks: colours, type ramp,
//! spacing, and chart palettes, as plain `Copy` data with no framework types
//! (ADR 0003). Consumers:
//!
//! - the web takes the emitted `tokens.css` ([`emit::tokens_css`]), pinned by
//!   a conformance test;
//! - Brightfield's render crate reads token values directly; its app shell
//!   applies the emitted gpui-component `ThemeConfig` JSON;
//! - future Linebender/Masonry chrome consumes the same structs.
//!
//! Colours are designed in OKLCH and stored as their sRGB conversion so no
//! consumer needs colour-space maths (ADR 0008). Phase 0 carries the type
//! shapes and the brand signature; scale and palette values land in Phase 1.

pub mod chrome;
pub mod colour;
pub mod emit;
pub mod scales;
pub mod spacing;
pub mod typography;
pub mod validate;
pub mod viz;

pub use colour::{Rgba, StateSet};

/// Maritime — the Meridian brand signature, `hsl(205, 35%, 45%)` / `#4b7a9b`
/// (web decision 0010). Reserved for interactive, focus, and selection states;
/// chrome stays warm-neutral and canvas colour belongs to data (ADR 0004).
pub const MARITIME: Rgba = Rgba::from_u8(0x4b, 0x7a, 0x9b, 0xff);
