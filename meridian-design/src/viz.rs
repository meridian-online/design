//! Chart palettes (ADR 0006). Values land in Phase 1 via the computed method
//! (see `validation/`): designed in OKLCH against the warm surfaces, gated by
//! the palette validator in both modes, validation output checked in.
//!
//! Planned exports:
//!
//! - **Categorical**: 8 ordered slots — the ORDER is the colourblind-safety
//!   mechanism (derived by enumerating orderings for maximum adjacent CVD ΔE)
//!   and is therefore data, never cosmetic. Assign in fixed order; fold a 9th
//!   series into "Other".
//! - **Sequential**: the default magnitude ramp as explicit stops (Brightfield
//!   scheme-stop table shape).
//! - **Diverging**: two poles + a neutral midpoint from the warm gray scale.
//! - **Status**: good/warning/serious/critical — reserved, never reused as
//!   series colours, never colour-alone (icon + label pairing).
//! - **`null_ink`**: a light neutral for NULL values that cannot impersonate
//!   any scheme value (closes Brightfield's NULL-renders-as-steel-blue bug).
//!
//! Portability contract: these are renderer DEFAULTS (zero spec surface).
//! Specs that must pin the palette portably carry explicit
//! `colorDomain`/`colorRange` literals — never the name "meridian", which
//! vanilla Mosaic would reject (closed Observable Plot scheme registry).
