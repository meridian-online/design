//! Notifications: persistent, id-deduplicated banners and transient toasts.
//!
//! Two layers, one rule each:
//!
//! - [`NotificationLayer`] holds **persistent banners keyed by
//!   [`NotificationId`]**. Raising an id that is already present *replaces*
//!   that banner in place — it never stacks. The id is composite
//!   ([`NotificationId::composite`]) so a recurring source names itself per
//!   step: a pipeline that fails the same step five times shows one current
//!   banner, not five stale ones. Banners live until dismissed (by the user
//!   or programmatically).
//! - [`ToastLayer`] holds **transient, self-expiring toasts**. No identity,
//!   no dedup — a toast is a moment, and stacking moments is correct.
//!
//! Both draw from the semantic tokens only: overlay surface, subtle
//! hairline, panel radius, a severity dot from the matching interaction
//! role. Severity maps to a token [`Role`] ([`Severity::role`]) rather than
//! to any colour named here.

use egui::{Align, Align2, Layout, Margin, RichText};
use meridian_design::{semantic, Role, Semantic};

use crate::theme::to_color32;
use crate::MeridianUi;

/// Identity for a persistent banner. Equality is what drives replacement,
/// so two banners about the same condition must construct the same id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NotificationId(egui::Id);

impl NotificationId {
    /// An id from one source value.
    pub fn new(source: impl std::hash::Hash + std::fmt::Debug) -> Self {
        Self(egui::Id::new(source))
    }

    /// A composite id: source plus step. A pipeline raising per-step banners
    /// uses one source with a step key per stage, so a re-failing stage
    /// replaces its own banner and never touches its siblings'.
    pub fn composite(
        source: impl std::hash::Hash + std::fmt::Debug,
        step: impl std::hash::Hash + std::fmt::Debug,
    ) -> Self {
        Self(egui::Id::new(source).with(step))
    }
}

/// How loudly a notification speaks. Maps onto the semantic interaction
/// roles — no colour is named in this module.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Neutral information — never used for "good".
    Info,
    /// Confirmation and healthy state.
    Success,
    /// Caution.
    Warning,
    /// Failure.
    Error,
}

impl Severity {
    /// The interaction role whose tokens this severity draws with.
    #[must_use]
    pub fn role(self) -> Role {
        match self {
            Severity::Info => Role::Info,
            Severity::Success => Role::Success,
            Severity::Warning => Role::Warning,
            Severity::Error => Role::Danger,
        }
    }
}

/// One persistent banner.
#[derive(Clone, Debug)]
pub struct Notification {
    /// Identity — what replacement keys on.
    pub id: NotificationId,
    /// Severity, mapped to a token role for the accent.
    pub severity: Severity,
    /// Headline.
    pub title: String,
    /// Optional supporting line, muted.
    pub body: Option<String>,
}

impl Notification {
    /// A banner with a headline and no body.
    pub fn new(id: NotificationId, severity: Severity, title: impl Into<String>) -> Self {
        Self {
            id,
            severity,
            title: title.into(),
            body: None,
        }
    }

    /// Add the supporting line.
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

/// Persistent, id-deduplicated banners, drawn anchored to the top-right of
/// the viewport. The host owns the instance and calls [`NotificationLayer::show`]
/// every frame.
#[derive(Default)]
pub struct NotificationLayer {
    items: Vec<Notification>,
}

impl NotificationLayer {
    /// An empty layer.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Raise a banner. If a banner with the same id is already showing, it
    /// is **replaced in place** — same position in the stack, new content —
    /// rather than added again.
    pub fn raise(&mut self, notification: Notification) {
        match self.items.iter_mut().find(|n| n.id == notification.id) {
            Some(existing) => *existing = notification,
            None => self.items.push(notification),
        }
    }

    /// Remove the banner with `id`. Returns whether one was showing.
    pub fn dismiss(&mut self, id: NotificationId) -> bool {
        let before = self.items.len();
        self.items.retain(|n| n.id != id);
        self.items.len() != before
    }

    /// Remove every banner.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// How many banners are showing.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether no banners are showing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The banners, oldest first.
    pub fn iter(&self) -> impl Iterator<Item = &Notification> {
        self.items.iter()
    }

    /// Draw the banners. Each carries its own dismiss control; dismissals
    /// are applied before returning.
    pub fn show(&mut self, ctx: &egui::Context) {
        if self.items.is_empty() {
            return;
        }
        let t = ctx.tokens();
        let style = ctx.style_of(ctx.theme());
        let dark = style.visuals.dark_mode;
        let sem = semantic(dark);
        let mut dismissed: Vec<NotificationId> = Vec::new();

        egui::Area::new(egui::Id::new("meridian_notification_layer"))
            .anchor(Align2::RIGHT_TOP, egui::vec2(-t.pane_gap, t.pane_gap))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                for n in &self.items {
                    card_frame(&style, t).show(ui, |ui| {
                        ui.set_width(t.modal_width_narrow - 2.0 * t.panel_padding);
                        ui.horizontal(|ui| {
                            severity_dot(ui, sem, n.severity);
                            ui.add_space(t.icon_label_gap);
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&n.title).strong());
                                if let Some(body) = &n.body {
                                    ui.label(RichText::new(body).color(to_color32(sem.text.muted)));
                                }
                            });
                            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                if ui.small_button("×").clicked() {
                                    dismissed.push(n.id);
                                }
                            });
                        });
                    });
                    ui.add_space(t.control_gap);
                }
            });

        for id in dismissed {
            self.dismiss(id);
        }
    }
}

/// One transient toast.
#[derive(Clone, Debug)]
pub struct Toast {
    /// Severity, mapped to a token role for the accent.
    pub severity: Severity,
    /// The line to show.
    pub text: String,
}

impl Toast {
    /// A toast.
    pub fn new(severity: Severity, text: impl Into<String>) -> Self {
        Self {
            severity,
            text: text.into(),
        }
    }
}

struct ToastEntry {
    toast: Toast,
    /// Wall-clock second (the host clock handed to [`ToastLayer::tick`])
    /// past which this toast is gone. `None` until first shown — the
    /// lifetime starts when the toast becomes visible, not when it was
    /// pushed.
    expires_at: Option<f64>,
}

/// How long a toast lives once visible, in seconds, unless the layer is
/// configured otherwise.
pub const DEFAULT_TOAST_SECONDS: f64 = 4.0;

/// Transient toasts, drawn anchored to the bottom-right of the viewport,
/// self-expiring. The host owns the instance and calls [`ToastLayer::show`]
/// every frame.
pub struct ToastLayer {
    items: Vec<ToastEntry>,
    duration: f64,
}

impl Default for ToastLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl ToastLayer {
    /// An empty layer with the default lifetime.
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            duration: DEFAULT_TOAST_SECONDS,
        }
    }

    /// An empty layer whose toasts live `seconds` once visible.
    #[must_use]
    pub fn with_duration(seconds: f64) -> Self {
        Self {
            items: Vec::new(),
            duration: seconds,
        }
    }

    /// Queue a toast. Toasts have no identity and stack in arrival order.
    pub fn push(&mut self, toast: Toast) {
        self.items.push(ToastEntry {
            toast,
            expires_at: None,
        });
    }

    /// How many toasts are queued or visible.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether nothing is queued or visible.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Advance the clock: start the lifetime of anything newly visible and
    /// drop what has expired. [`ToastLayer::show`] calls this with egui's
    /// clock; it is public so a host (or a test) can drive expiry with its
    /// own.
    pub fn tick(&mut self, now: f64) {
        for entry in &mut self.items {
            entry.expires_at.get_or_insert(now + self.duration);
        }
        self.items
            .retain(|e| e.expires_at.is_some_and(|at| at > now));
    }

    /// Draw the toasts, expiring as it goes, and ask for a repaint at the
    /// next expiry so toasts leave on time even with no input.
    pub fn show(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|i| i.time);
        self.tick(now);
        if self.items.is_empty() {
            return;
        }

        let t = ctx.tokens();
        let style = ctx.style_of(ctx.theme());
        let sem = semantic(style.visuals.dark_mode);

        egui::Area::new(egui::Id::new("meridian_toast_layer"))
            .anchor(Align2::RIGHT_BOTTOM, egui::vec2(-t.pane_gap, -t.pane_gap))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                for entry in &self.items {
                    egui::Frame::new()
                        .fill(to_color32(sem.surfaces.overlay))
                        .stroke(egui::Stroke::new(1.0, to_color32(sem.borders.subtle)))
                        .corner_radius(t.radius_panel)
                        .inner_margin(Margin::symmetric(t.space[4] as i8, t.space[2] as i8))
                        .shadow(style.visuals.popup_shadow)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                severity_dot(ui, sem, entry.toast.severity);
                                ui.add_space(t.icon_label_gap);
                                ui.label(&entry.toast.text);
                            });
                        });
                    ui.add_space(t.control_gap);
                }
            });

        if let Some(next) = self
            .items
            .iter()
            .filter_map(|e| e.expires_at)
            .min_by(f64::total_cmp)
        {
            let wait = (next - now).max(0.0);
            ctx.request_repaint_after(std::time::Duration::from_secs_f64(wait));
        }
    }
}

/// The banner/toast card: overlay surface, subtle hairline, panel radius,
/// overlay elevation shadow.
fn card_frame(style: &egui::Style, t: &crate::tokens::Tokens) -> egui::Frame {
    let sem = semantic(style.visuals.dark_mode);
    egui::Frame::new()
        .fill(to_color32(sem.surfaces.overlay))
        .stroke(egui::Stroke::new(1.0, to_color32(sem.borders.subtle)))
        .corner_radius(t.radius_panel)
        .inner_margin(Margin::same(t.panel_padding as i8))
        .shadow(style.visuals.popup_shadow)
}

/// The severity accent: a small filled dot in the role's solid, sized off
/// the spacing ladder.
fn severity_dot(ui: &mut egui::Ui, sem: &Semantic, severity: Severity) {
    let t = ui.tokens();
    let size = t.space[4];
    let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter().circle_filled(
            rect.center(),
            size / 2.0,
            to_color32(sem.role(severity.role()).background.base),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn banner(source: &str, step: &str, title: &str) -> Notification {
        Notification::new(
            NotificationId::composite(source, step),
            Severity::Error,
            title,
        )
    }

    #[test]
    fn raising_the_same_id_replaces_instead_of_stacking() {
        let mut layer = NotificationLayer::new();
        layer.raise(banner("pipeline", "fetch", "fetch failed"));
        layer.raise(banner("pipeline", "fetch", "fetch failed again"));
        assert_eq!(layer.len(), 1, "one id, one banner");
        assert_eq!(
            layer.iter().next().map(|n| n.title.as_str()),
            Some("fetch failed again"),
            "the newest content wins"
        );
    }

    #[test]
    fn replacement_keeps_the_banner_position() {
        let mut layer = NotificationLayer::new();
        layer.raise(banner("pipeline", "fetch", "fetch failed"));
        layer.raise(banner("pipeline", "parse", "parse failed"));
        layer.raise(banner("pipeline", "fetch", "fetch failed again"));
        let titles: Vec<&str> = layer.iter().map(|n| n.title.as_str()).collect();
        assert_eq!(titles, ["fetch failed again", "parse failed"]);
    }

    #[test]
    fn different_steps_of_one_source_are_different_banners() {
        let mut layer = NotificationLayer::new();
        layer.raise(banner("pipeline", "fetch", "fetch failed"));
        layer.raise(banner("pipeline", "parse", "parse failed"));
        assert_eq!(layer.len(), 2);
        assert_ne!(
            NotificationId::composite("pipeline", "fetch"),
            NotificationId::composite("pipeline", "parse")
        );
        assert_eq!(
            NotificationId::composite("pipeline", "fetch"),
            NotificationId::composite("pipeline", "fetch")
        );
    }

    #[test]
    fn dismiss_removes_exactly_its_banner() {
        let mut layer = NotificationLayer::new();
        layer.raise(banner("pipeline", "fetch", "fetch failed"));
        layer.raise(banner("pipeline", "parse", "parse failed"));
        assert!(layer.dismiss(NotificationId::composite("pipeline", "fetch")));
        assert_eq!(layer.len(), 1);
        assert!(
            !layer.dismiss(NotificationId::composite("pipeline", "fetch")),
            "dismissing an absent id reports false"
        );
    }

    #[test]
    fn severity_maps_onto_token_roles() {
        assert_eq!(Severity::Info.role(), Role::Info);
        assert_eq!(Severity::Success.role(), Role::Success);
        assert_eq!(Severity::Warning.role(), Role::Warning);
        assert_eq!(Severity::Error.role(), Role::Danger);
    }

    #[test]
    fn toasts_expire_after_their_duration_once_visible() {
        let mut layer = ToastLayer::with_duration(2.0);
        layer.push(Toast::new(Severity::Info, "saved"));
        layer.push(Toast::new(Severity::Info, "exported"));
        assert_eq!(layer.len(), 2);

        layer.tick(10.0); // both become visible at t=10
        assert_eq!(layer.len(), 2);
        layer.tick(11.9);
        assert_eq!(layer.len(), 2, "still inside the lifetime");
        layer.tick(12.1);
        assert_eq!(layer.len(), 0, "both expired together");
    }

    #[test]
    fn a_toast_lifetime_starts_when_first_visible_not_when_pushed() {
        let mut layer = ToastLayer::with_duration(2.0);
        layer.push(Toast::new(Severity::Info, "first"));
        layer.tick(10.0);
        layer.push(Toast::new(Severity::Info, "second")); // visible from the next tick
        layer.tick(11.0);
        layer.tick(12.5); // first (10+2) is gone; second (11+2) survives
        assert_eq!(layer.len(), 1);
        layer.tick(13.5);
        assert_eq!(layer.len(), 0);
    }

    #[test]
    fn toasts_stack_without_dedup() {
        let mut layer = ToastLayer::new();
        layer.push(Toast::new(Severity::Info, "saved"));
        layer.push(Toast::new(Severity::Info, "saved"));
        assert_eq!(layer.len(), 2, "a toast is a moment; moments stack");
    }
}
