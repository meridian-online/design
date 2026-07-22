//! The egui emitter of the Meridian Design System.
//!
//! `meridian-design` is framework-neutral by contract (ADR 0003): its tokens are
//! plain sRGB `Copy` data with an empty `[dependencies]`, so every consumer
//! converts them onto its own framework's types at its own boundary. This crate
//! is that boundary for egui — "a third thin emitter" alongside the web's
//! `tokens.css` and the desktop theme JSON (ADR 0003; ADR 0011 places the egui
//! adapter here, beside the tokens, rather than in the consuming shell).
//!
//! Three things cross the boundary and nothing else:
//!
//! - [`theme`] — the token → `egui::Style`/`Visuals`/`FontDefinitions` bridge.
//!   Colour and type from the Meridian scales and ramp; the box model (spacing,
//!   corner radii, elevation shadows) and the animation budget from the geometry
//!   and motion tokens. [`theme::apply`] installs all of it on a context.
//! - [`Mode`] — light or dark chrome, the one argument every token accessor
//!   takes (`semantic(dark)`, `Elevation::shadow(dark)`). It lives with the
//!   adapter because it is a theme selector, not application state.
//! - [`Tokens`] + [`MeridianUi`] — the geometry a surface reaches for, behind
//!   `ui.tokens()`, so a call site spells `SPACE_4` by name instead of threading
//!   a token bundle through every signature (the rerun `UiExt` idiom).
//!
//! What does **not** live here is the shell contract — `Item`, `Panel`, the dock
//! model, the `Subject` push contract. Information architecture is
//! application-shaped and stays in its host repo; only the token adapter and the
//! reusable primitives it grows are portable enough to belong to the system
//! (decision recorded in the design ADR set).

pub mod ext;
pub mod mode;
pub mod theme;
pub mod tokens;

pub use ext::MeridianUi;
pub use mode::Mode;
pub use tokens::{Tokens, TOKENS};

pub mod align;
pub mod icons;

pub use icons::Icon;
