//! The one modal chrome.
//!
//! The audit behind ADR 0011 found one conceptual modal card re-declared with
//! widths, max-heights, paddings and escape hints that agreed with nothing.
//! This module is the single copy: [`overlay_frame`] draws the card chrome
//! (surface, hairline, panel radius, modal shadow, title row, keystroke hint
//! footer) around any body, and [`ModalLayer`] floats that card over a
//! token-scrim backdrop with escape / click-outside dismissal.
//!
//! Width is a rung of the modal-width scale ([`ModalWidth`]) resolved through
//! `ui.tokens()` — there is no pixel parameter, so a third width cannot be
//! introduced from a call site. Height is content-driven, capped so the card
//! always keeps the largest ladder gap of breathing room against the screen
//! edge.
//!
//! The escape/enter affordances are part of the chrome, not the body: every
//! modal states them the same way (keycap chip + muted verb, right-aligned in
//! the footer), so "how do I leave this?" never varies by surface again.

use std::hash::Hash;

use egui::{Align, Layout, RichText};
use meridian_design::semantic;

use crate::key_chip::key_chip;
use crate::theme::to_color32;
use crate::tokens::Tokens;
use crate::MeridianUi;

/// A rung of the modal-width scale. Two rungs exist and there is no third
/// (see the token crate's spacing module): a modal is narrow or default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ModalWidth {
    /// Confirmations, single-field prompts.
    Narrow,
    /// Forms, pickers, detail cards.
    #[default]
    Default,
}

impl ModalWidth {
    /// The rung's card width in logical pixels, resolved from the shared
    /// tokens.
    #[must_use]
    pub fn px(self, tokens: &Tokens) -> f32 {
        match self {
            ModalWidth::Narrow => tokens.modal_width_narrow,
            ModalWidth::Default => tokens.modal_width_default,
        }
    }
}

/// What a modal card says about itself: title, width rung, and the
/// keystroke affordance hints in the footer.
///
/// The escape hint defaults to "close" — present unless a caller explicitly
/// removes it, so the inconsistency this chrome retires (three different
/// escape hints, including none) cannot quietly return.
#[derive(Clone, Debug)]
pub struct ModalChrome {
    title: Option<String>,
    width: ModalWidth,
    esc_hint: Option<String>,
    enter_hint: Option<String>,
}

impl Default for ModalChrome {
    fn default() -> Self {
        Self::new()
    }
}

impl ModalChrome {
    /// Default chrome: no title, default width, an "Esc close" hint, no
    /// enter hint.
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            width: ModalWidth::Default,
            esc_hint: Some("close".to_owned()),
            enter_hint: None,
        }
    }

    /// Set the title row.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Pick a width rung.
    #[must_use]
    pub fn width(mut self, width: ModalWidth) -> Self {
        self.width = width;
        self
    }

    /// Shorthand for the narrow rung.
    #[must_use]
    pub fn narrow(mut self) -> Self {
        self.width = ModalWidth::Narrow;
        self
    }

    /// Replace the escape hint's verb ("close", "cancel", …).
    #[must_use]
    pub fn esc_hint(mut self, verb: impl Into<String>) -> Self {
        self.esc_hint = Some(verb.into());
        self
    }

    /// Remove the escape hint (for chrome whose body already narrates its
    /// exits — use sparingly).
    #[must_use]
    pub fn without_esc_hint(mut self) -> Self {
        self.esc_hint = None;
        self
    }

    /// Add an enter hint with its verb ("run", "apply", "jump", …).
    #[must_use]
    pub fn enter_hint(mut self, verb: impl Into<String>) -> Self {
        self.enter_hint = Some(verb.into());
        self
    }
}

/// The card frame itself: overlay surface, subtle hairline, panel radius,
/// modal padding, and the modal elevation shadow the theme derived from the
/// tokens. In light mode the hairline and shadow are the *only* separation an
/// overlay has from the plane below (the surfaces are deliberately equal), so
/// both are always painted.
fn card_frame(style: &egui::Style, tokens: &Tokens) -> egui::Frame {
    let sem = semantic(style.visuals.dark_mode);
    egui::Frame::new()
        .fill(to_color32(sem.surfaces.overlay))
        .stroke(egui::Stroke::new(1.0, to_color32(sem.borders.subtle)))
        .corner_radius(tokens.radius_panel)
        .inner_margin(egui::Margin::same(tokens.modal_padding as i8))
        .shadow(style.visuals.window_shadow)
}

/// One keystroke affordance: keycap chip, then its verb in muted small ink.
/// Laid out right-to-left (the footer's direction), so the verb is added
/// first to land to the chip's right when read left-to-right.
fn hint_pair(ui: &mut egui::Ui, key: &str, verb: &str) {
    let t = ui.tokens();
    let dark = ui.visuals().dark_mode;
    let muted = to_color32(semantic(dark).text.muted);
    ui.label(RichText::new(verb).small().color(muted));
    ui.add_space(t.space[1]);
    key_chip(ui, key);
}

/// The chrome around a body: width, height cap, title row, body, hint
/// footer. Shared by [`overlay_frame`] and [`ModalLayer`] so the two can
/// never drift.
fn chrome_contents<R>(
    ui: &mut egui::Ui,
    chrome: &ModalChrome,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let t = ui.tokens();
    ui.set_width(chrome.width.px(t) - 2.0 * t.modal_padding);
    let max_h = ui.ctx().content_rect().height() - 2.0 * t.space[9] - 2.0 * t.modal_padding;
    ui.set_max_height(max_h.max(0.0));

    if let Some(title) = &chrome.title {
        ui.label(RichText::new(title).strong());
        ui.add_space(t.section_gap);
    }

    let inner = body(ui);

    if chrome.esc_hint.is_some() || chrome.enter_hint.is_some() {
        ui.add_space(t.section_gap);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if let Some(verb) = &chrome.esc_hint {
                hint_pair(ui, "Esc", verb);
            }
            if let Some(verb) = &chrome.enter_hint {
                if chrome.esc_hint.is_some() {
                    ui.add_space(t.control_gap);
                }
                hint_pair(ui, "Enter", verb);
            }
        });
    }

    inner
}

/// Draw the modal card chrome inline, wherever the caller already is — no
/// backdrop, no centring. This is the card [`ModalLayer`] floats; it is also
/// usable on its own for docked overlay surfaces that want the same chrome.
pub fn overlay_frame<R>(
    ui: &mut egui::Ui,
    chrome: &ModalChrome,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let frame = card_frame(ui.style(), ui.tokens());
    frame.show(ui, |ui| chrome_contents(ui, chrome, body)).inner
}

/// What [`ModalLayer::show`] hands back.
pub struct ModalLayerResponse<R> {
    /// The body closure's return value.
    pub inner: R,
    /// The user asked to leave: escape was pressed (and not already consumed
    /// by the body — a [`crate::Picker`] inside the modal consumes it first
    /// and reports its own dismissal), or the backdrop was clicked. The
    /// caller stops showing the modal in response; the layer holds no
    /// open/closed state of its own.
    pub dismissed: bool,
}

/// The floating modal plane: token-scrim backdrop, centred [`overlay_frame`]
/// card, escape / click-outside dismissal.
///
/// Stateless by design — in immediate mode "a modal is open" is the caller's
/// state (it either calls [`ModalLayer::show`] this frame or it does not),
/// and *which* modals exist is application information architecture that
/// stays host-side (ADR 0011). This layer owns only the chrome.
pub struct ModalLayer;

impl ModalLayer {
    /// Show a modal this frame: scrim, centred card with `chrome`, `body`
    /// inside it. `id_salt` distinguishes concurrent modals (egui stacks
    /// them; escape dismisses the topmost first).
    pub fn show<R>(
        ctx: &egui::Context,
        id_salt: impl Hash + std::fmt::Debug,
        chrome: &ModalChrome,
        body: impl FnOnce(&mut egui::Ui) -> R,
    ) -> ModalLayerResponse<R> {
        let t = ctx.tokens();
        let style = ctx.style_of(ctx.theme());
        let scrim = to_color32(semantic(style.visuals.dark_mode).surfaces.scrim);

        let response = egui::Modal::new(egui::Id::new(id_salt))
            .backdrop_color(scrim)
            .frame(card_frame(&style, t))
            .show(ctx, |ui| chrome_contents(ui, chrome, body));

        ModalLayerResponse {
            dismissed: response.should_close(),
            inner: response.inner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TOKENS;

    #[test]
    fn width_rungs_resolve_to_the_token_scale() {
        assert_eq!(ModalWidth::Narrow.px(&TOKENS), TOKENS.modal_width_narrow);
        assert_eq!(ModalWidth::Default.px(&TOKENS), TOKENS.modal_width_default);
        assert_eq!(ModalWidth::default(), ModalWidth::Default);
    }

    #[test]
    fn default_chrome_always_carries_an_escape_hint() {
        let chrome = ModalChrome::new();
        assert_eq!(chrome.esc_hint.as_deref(), Some("close"));
        assert!(chrome.enter_hint.is_none());
        let renamed = ModalChrome::new().esc_hint("cancel").enter_hint("run");
        assert_eq!(renamed.esc_hint.as_deref(), Some("cancel"));
        assert_eq!(renamed.enter_hint.as_deref(), Some("run"));
        assert!(ModalChrome::new().without_esc_hint().esc_hint.is_none());
    }
}
