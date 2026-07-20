//! Meridian design tokens.
//!
//! The single definition point for how Meridian looks: colours, type ramp,
//! spacing, and chart palettes, as plain `Copy` data with no framework types
//! (ADR 0003). Consumers:
//!
//! - the web takes the emitted `tokens.css` ([`emit::tokens_css`]), pinned by
//!   a conformance test;
//! - Brightfield's renderer reads token values directly; its app shell applies
//!   the emitted theme;
//! - framework adapters are thin emitters and they live in this repo, as
//!   sibling crates — a host change re-translates the adapter, not the system
//!   (ADR 0003; ADR 0011 for the egui adapter).
//!
//! Colours are designed in OKLCH and stored as their sRGB conversion so no
//! consumer needs colour-space maths (ADR 0008). The crate carries the full
//! palette — neutral, accent and semantic scales, the categorical chart set,
//! and the sequential and diverging ramps — alongside the type ramp, spacing
//! and the brand signature.

pub mod chrome;
pub mod colour;
pub mod emit;
pub mod fonts;
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
