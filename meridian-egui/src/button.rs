//! The button: an optional icon, a label, an optional keycap.
//!
//! Three weights — [`Weight::Default`] (the neutral role's fill in its rule),
//! [`Weight::Primary`] (the accent's solid) and [`Weight::Quiet`] (no fill and
//! no rule until the pointer is over it). 24 tall, or 20 with
//! [`Button::dense`]; an icon-only button is square. A button is as wide as its
//! contents, never wider than the space it is given: a label too long for that
//! is cut with an ellipsis and the keycap keeps its place.
//!
//! The text is the caller's: the label, the keystroke (`"⌘↩"`) and the reason
//! an unavailable button gives are strings it passes. Nothing here animates;
//! every state shows on the next frame.
//!
//! An unavailable button ([`Button::unavailable`]) keeps its place, senses the
//! pointer, reports no click and reports its rect when pointed at
//! ([`ButtonResponse::pointed_unavailable`]) so the caller can anchor a popover
//! holding the reason. It takes no keyboard focus, and its accessibility node is
//! disabled with the reason as its description.
//!
//! [`paint_button`] and [`button_size`] are the drawing and the measure, kept
//! apart from the interaction so a component that lays buttons out itself (the
//! ribbon row) draws the same button.

use egui::epaint::text::{FontId, LayoutJob, TextFormat};
use egui::{Color32, CornerRadius, Rect, Sense, Stroke, StrokeKind, Vec2};
use meridian_design::control::{HEIGHT_MD, HEIGHT_SM, ICON_SM};
use meridian_design::typography::UI_SIZE;
use meridian_design::{semantic, Role};

use crate::icons::Icon;
use crate::key_chip::{key_size, paint_key, KeyInk, KeySize};
use crate::theme::to_color32;
use crate::widgets::{focus_ring, focus_ring_on_solid, optically_centred_galley_top};
use crate::MeridianUi;

/// How loud a button is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Weight {
    /// The neutral role's fill, rule and ink.
    #[default]
    Default,
    /// The accent's solid: the one action a surface wants taken.
    Primary,
    /// No fill and no rule at rest; the neutral hover and pressed fills under
    /// the pointer. For commands in a band or a ribbon.
    Quiet,
}

/// One button's configuration. Built with [`Button::new`] or
/// [`Button::icon_only`] and the `#[must_use]` builders, then drawn by
/// [`button`].
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct Button<'a> {
    icon: Option<&'a Icon>,
    label: &'a str,
    icon_only: bool,
    keystroke: Option<&'a str>,
    reason: Option<&'a str>,
    weight: Weight,
    dense: bool,
}

impl<'a> Button<'a> {
    /// A button labelled `label`, default weight, 24 tall.
    pub fn new(label: &'a str) -> Self {
        Self {
            icon: None,
            label,
            icon_only: false,
            keystroke: None,
            reason: None,
            weight: Weight::Default,
            dense: false,
        }
    }

    /// A square button drawing only `icon`. `label` is not drawn: it names the
    /// button to a screen reader.
    pub fn icon_only(icon: &'a Icon, label: &'a str) -> Self {
        Self {
            icon: Some(icon),
            icon_only: true,
            ..Self::new(label)
        }
    }

    /// An icon before the label.
    pub fn icon(mut self, icon: &'a Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// The keystroke that does what this button does, as the caller's string
    /// (`"⌘↩"`), drawn as a keycap at the trailing edge. A dense button has no
    /// room for one: passing it is a debug assertion, and in release it is not
    /// drawn.
    pub fn keystroke(mut self, keystroke: &'a str) -> Self {
        self.keystroke = Some(keystroke);
        self
    }

    /// How loud the button is.
    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    /// The dense rung: 20 tall, `SPACE_3` of padding, no keycap.
    pub fn dense(mut self) -> Self {
        self.dense = true;
        self
    }

    /// The button cannot be used now, for `reason` (the caller's sentence).
    pub fn unavailable(mut self, reason: &'a str) -> Self {
        self.reason = Some(reason);
        self
    }

    /// The keycap that will be drawn: none on a dense or icon-only button.
    fn key(&self) -> Option<&'a str> {
        if self.dense || self.icon_only {
            None
        } else {
            self.keystroke
        }
    }

    fn height(&self) -> f32 {
        if self.dense {
            HEIGHT_SM
        } else {
            HEIGHT_MD
        }
    }
}

/// What [`button`] hands back.
#[derive(Clone, Debug)]
pub struct ButtonResponse {
    /// The button's own response.
    pub response: egui::Response,
    /// The button was clicked. Always false for an unavailable one.
    pub clicked: bool,
    /// The button is unavailable and the pointer is over it: its rect, for the
    /// caller to anchor the reason popover to. `None` otherwise.
    pub pointed_unavailable: Option<Rect>,
}

/// What [`paint_button`] draws the button in. A caller that lays a button out
/// itself sets what it needs: a ribbon row draws a quiet button with its label
/// dropped when it is narrow, and with no key when a key tip replaces it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ButtonPaint {
    /// The pointer is over the button. Ignored when it is unavailable.
    pub hovered: bool,
    /// The primary button is down on it. Ignored when it is unavailable.
    pub pressed: bool,
    /// The button has keyboard focus. Ignored when it is unavailable.
    pub focused: bool,
    /// Draw the label. Off, the button is the size `button_size(.., true)`
    /// reports: the icon and the key only.
    pub label: bool,
    /// Draw the keycap, if the button has one.
    pub key: bool,
}

impl ButtonPaint {
    /// A button at rest with everything drawn.
    pub(crate) const REST: Self = Self {
        hovered: false,
        pressed: false,
        focused: false,
        label: true,
        key: true,
    };
}

/// The label's ink and the icon's, the box's fill and rule, the keycap's ink.
struct Inks {
    fill: Color32,
    rule: Color32,
    label: Color32,
    icon: Color32,
    key: KeyInk,
}

fn inks(ui: &egui::Ui, button: &Button<'_>, p: &ButtonPaint) -> Inks {
    let sem = semantic(ui.visuals().dark_mode);
    let off = button.reason.is_some();
    let neutral = sem.role(Role::Neutral);
    let c = to_color32;
    match (button.weight, off) {
        (Weight::Default, true) => Inks {
            fill: c(neutral.background.disabled),
            rule: c(neutral.border.disabled),
            label: c(sem.text.disabled),
            icon: c(sem.text.disabled),
            key: KeyInk::Dimmed,
        },
        (Weight::Default, false) => {
            let (fill, rule) = if p.pressed {
                (neutral.background.active, neutral.border.active)
            } else if p.hovered {
                (neutral.background.hover, neutral.border.hover)
            } else {
                (neutral.background.base, neutral.border.base)
            };
            Inks {
                fill: c(fill),
                rule: c(rule),
                label: c(neutral.foreground.base),
                icon: c(sem.text.secondary),
                key: KeyInk::Plain,
            }
        }
        (Weight::Primary, true) => {
            let a = sem.role(Role::Accent);
            Inks {
                fill: c(a.background.disabled),
                rule: c(a.border.disabled),
                label: c(a.foreground.disabled),
                icon: c(a.foreground.disabled),
                key: KeyInk::Dimmed,
            }
        }
        (Weight::Primary, false) => {
            let a = sem.role(Role::Accent);
            let (fill, rule) = if p.pressed {
                (a.background.active, a.border.active)
            } else if p.hovered {
                (a.background.hover, a.border.hover)
            } else {
                (a.background.base, a.border.base)
            };
            Inks {
                fill: c(fill),
                rule: c(rule),
                label: c(a.foreground.base),
                icon: c(a.foreground.base),
                key: KeyInk::OnSolid,
            }
        }
        (Weight::Quiet, true) => Inks {
            fill: Color32::TRANSPARENT,
            rule: Color32::TRANSPARENT,
            label: c(sem.text.disabled),
            icon: c(sem.text.disabled),
            key: KeyInk::Dimmed,
        },
        (Weight::Quiet, false) => {
            let fill = if p.pressed {
                c(neutral.background.active)
            } else if p.hovered {
                c(neutral.background.hover)
            } else {
                Color32::TRANSPARENT
            };
            Inks {
                fill,
                rule: Color32::TRANSPARENT,
                label: c(neutral.foreground.base),
                icon: c(sem.text.secondary),
                key: KeyInk::Plain,
            }
        }
    }
}

/// The label on one line, cut with an ellipsis at `max_width` when it does not
/// fit.
fn label_galley(
    ui: &egui::Ui,
    text: &str,
    ink: Color32,
    max_width: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = LayoutJob::single_section(
        text.to_owned(),
        TextFormat::simple(FontId::proportional(UI_SIZE), ink),
    );
    job.wrap.max_width = max_width;
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    ui.painter().layout_job(job)
}

/// The leading padding, and the trailing one when there is no keycap.
fn padding(ui: &egui::Ui, button: &Button<'_>) -> f32 {
    let t = ui.tokens();
    if button.dense {
        t.space[3]
    } else {
        t.space[4]
    }
}

/// Whether the button is only its icon: square, centred.
fn is_square(button: &Button<'_>, drop_label: bool) -> bool {
    button.icon.is_some() && (button.icon_only || (drop_label && button.key().is_none()))
}

/// The size a button takes at its natural width. `drop_label` measures it
/// without its label, as a ribbon row draws it when it is too narrow: the icon
/// and the keycap remain.
pub(crate) fn button_size(ui: &egui::Ui, button: &Button<'_>, drop_label: bool) -> Vec2 {
    let height = button.height();
    if is_square(button, drop_label) {
        return Vec2::splat(height);
    }
    let gap = ui.tokens().icon_label_gap;
    let pad = padding(ui, button);
    let mut parts: Vec<f32> = Vec::new();
    if button.icon.is_some() {
        parts.push(ICON_SM);
    }
    if !drop_label {
        parts.push(
            label_galley(ui, button.label, Color32::PLACEHOLDER, f32::INFINITY)
                .size()
                .x,
        );
    }
    let key = button.key().map(|k| key_size(ui, k, KeySize::Control).x);
    parts.extend(key);
    let gaps = gap * parts.len().saturating_sub(1) as f32;
    let trailing = if key.is_some() {
        ui.tokens().keycap_inset_in_control
    } else {
        pad
    };
    Vec2::new(pad + parts.iter().sum::<f32>() + gaps + trailing, height)
}

/// Paint `button` in `rect` in the state `p` describes: its box, the focus
/// ring, the icon, the label (cut with an ellipsis to the room the key leaves)
/// and the keycap. `rect` is the button's box; a caller that laid it out
/// itself passes the rect it chose, and the content keeps its insets from the
/// edges.
pub(crate) fn paint_button(ui: &egui::Ui, rect: Rect, button: &Button<'_>, p: ButtonPaint) {
    let off = button.reason.is_some();
    let p = ButtonPaint {
        hovered: p.hovered && !off,
        pressed: p.pressed && !off,
        focused: p.focused && !off,
        ..p
    };
    let ink = inks(ui, button, &p);
    let t = ui.tokens();
    let painter = ui.painter();

    if ink.fill != Color32::TRANSPARENT || ink.rule != Color32::TRANSPARENT {
        painter.rect(
            rect,
            CornerRadius::same(t.radius_control.round() as u8),
            ink.fill,
            Stroke::new(1.0, ink.rule),
            StrokeKind::Inside,
        );
    }
    if p.focused {
        if button.weight == Weight::Primary {
            focus_ring_on_solid(ui, rect, t.radius_control);
        } else {
            focus_ring(ui, rect, t.radius_control);
        }
    }

    let key = button.key().filter(|_| p.key);
    let draw_label = p.label && !button.icon_only;
    let gap = t.icon_label_gap;

    if is_square(button, !p.label) {
        if let Some(icon) = button.icon {
            let r = Rect::from_center_size(rect.center(), Vec2::splat(ICON_SM));
            icon.paint(painter, r, ink.icon);
        }
        return;
    }

    let mut x = rect.left() + padding(ui, button);
    let mut right = rect.right() - padding(ui, button);
    if let Some(k) = key {
        let size = key_size(ui, k, KeySize::Control);
        let inset = t.keycap_inset_in_control;
        let key_rect = Rect::from_min_size(
            egui::pos2(rect.right() - inset - size.x, rect.top() + inset),
            size,
        );
        paint_key(ui, key_rect, k, KeySize::Control, ink.key);
        right = key_rect.left() - gap;
    }
    if let Some(icon) = button.icon {
        let r = Rect::from_center_size(
            egui::pos2(x + ICON_SM / 2.0, rect.center().y),
            Vec2::splat(ICON_SM),
        );
        icon.paint(painter, r, ink.icon);
        x += ICON_SM + gap;
    }
    if draw_label {
        let galley = label_galley(ui, button.label, ink.label, (right - x).max(0.0));
        let top = optically_centred_galley_top(&galley, rect.center().y);
        painter.galley(egui::pos2(x, top), galley, ink.label);
    }
}

/// Draw a button and report what was done to it.
///
/// The button is as wide as its contents and no wider than the width `ui` has
/// left; [`paint_button`] cuts a label that does not fit.
pub fn button(ui: &mut egui::Ui, button: Button<'_>) -> ButtonResponse {
    debug_assert!(
        !(button.dense && button.keystroke.is_some()),
        "a dense button has no room for a keycap"
    );
    let mut size = button_size(ui, &button, false);
    size.x = size.x.min(ui.available_width().max(button.height()));
    let off = button.reason.is_some();
    let sense = if off { Sense::hover() } else { Sense::click() };
    let (rect, response) = ui.allocate_exact_size(size, sense);

    let label = button.label;
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, !off, label));
    if let Some(reason) = button.reason {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_disabled();
            node.set_description(reason);
        });
    }

    if ui.is_rect_visible(rect) {
        paint_button(
            ui,
            rect,
            &button,
            ButtonPaint {
                hovered: response.hovered(),
                pressed: response.is_pointer_button_down_on(),
                focused: response.has_focus(),
                ..ButtonPaint::REST
            },
        );
    }

    ButtonResponse {
        clicked: !off && response.clicked(),
        pointed_unavailable: (off && response.hovered()).then_some(rect),
        response,
    }
}
