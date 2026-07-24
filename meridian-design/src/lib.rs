//! Meridian design tokens.
//!
//! The single definition point for how Meridian looks: colours, type ramp,
//! spacing, and chart palettes, as plain `Copy` data with no framework types
//! (ADR 0003). Consumers:
//!
//! - the web takes the emitted `tokens.css` ([`emit::tokens_css`]), pinned by
//!   a conformance test;
//! - the desktop app consumes tokens through the egui adapter, a sibling
//!   crate that reads the token values directly;
//! - that adapter is a thin emitter that lives in this repo — a host change
//!   re-translates the adapter, not the system (ADR 0003; ADR 0011 for the
//!   egui adapter).
//!
//! Colours are designed in OKLCH and stored as their sRGB conversion so no
//! consumer needs colour-space maths (ADR 0008). The crate carries the full
//! palette — neutral, accent and semantic scales, the categorical chart set,
//! and the sequential and diverging ramps — alongside the type ramp, the box
//! model, and the brand signature.
//!
//! The crate is layered:
//!
//! - **raw** — [`scales`] (12-step ramps), [`viz`] (chart palettes), [`chrome`]
//!   (the ink/overlay tokens the renderer reads);
//! - **geometry** — [`spacing`], [`radius`], [`control`], [`focus`],
//!   [`elevation`], [`motion`], [`input_widgets`]: the box model, stated once;
//! - **semantic** — [`semantic`]: what a colour is *for*, framework-neutral,
//!   with an instantiated [`StateSet`] per interaction role;
//! - **contract** — [`a11y`]: role and keyboard intent, the half of a component
//!   spec that colour tokens cannot carry;
//! - **emitters** — [`emit`], one per consumer, each pinned by a snapshot.
//!
//! Reach for the semantic layer first; drop to a raw scale only when the thing
//! being coloured genuinely has no semantic name yet.

pub mod a11y;
pub mod chrome;
pub mod colour;
pub mod control;
pub mod elevation;
pub mod emit;
pub mod focus;
pub mod fonts;
pub mod input_widgets;
pub mod motion;
pub mod radius;
pub mod scales;
pub mod semantic;
pub mod spacing;
pub mod typography;
pub mod validate;
pub mod viz;

pub use colour::{Rgba, StateSet};
pub use elevation::Elevation;
pub use semantic::{semantic, Role, Semantic};

/// Maritime — the Meridian brand signature, `hsl(205, 35%, 45%)` / `#4b7a9b`
/// (web decision 0010). Reserved for interactive, focus, and selection states;
/// chrome stays warm-neutral and canvas colour belongs to data (ADR 0004).
pub const MARITIME: Rgba = Rgba::from_u8(0x4b, 0x7a, 0x9b, 0xff);
