//! `ui.tokens()` — the Meridian box model, reachable from any egui context.
//!
//! rerun's `re_ui` established the idiom this follows: an extension trait on
//! `egui::Ui` (and here `egui::Context` too) so a widget reads shared design
//! data through the `Ui` it already holds, instead of every function growing a
//! `tokens: &Tokens` parameter that is threaded from the top of the frame. The
//! tokens are a `&'static` constant, so this borrows nothing and allocates
//! nothing — it is pure sugar over [`crate::TOKENS`].

use crate::tokens::{Tokens, TOKENS};

/// Meridian design-token access for egui contexts. Implemented for both
/// [`egui::Ui`] and [`egui::Context`] so a call site can reach the box model
/// from whichever it holds.
pub trait MeridianUi {
    /// The Meridian box-model tokens (spacing, radii, row and control heights,
    /// modal widths). A shared `&'static` — the same instance every time.
    fn tokens(&self) -> &'static Tokens;
}

impl MeridianUi for egui::Ui {
    fn tokens(&self) -> &'static Tokens {
        &TOKENS
    }
}

impl MeridianUi for egui::Context {
    fn tokens(&self) -> &'static Tokens {
        &TOKENS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_hands_back_the_shared_tokens() {
        let ctx = egui::Context::default();
        assert!(std::ptr::eq(ctx.tokens(), &TOKENS));
    }

    #[test]
    fn ui_hands_back_the_shared_tokens() {
        let mut got: Option<*const Tokens> = None;
        // egui's built-in CPU-only test harness is enough to obtain a `Ui` and
        // call the trait method.
        egui::__run_test_ui(|ui| {
            got = Some(ui.tokens() as *const Tokens);
        });
        assert_eq!(got, Some(&TOKENS as *const Tokens));
    }
}
