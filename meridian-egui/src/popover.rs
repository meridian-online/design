//! The popover: an overlay card anchored to a rect.
//!
//! It is the card [`crate::overlay`] draws for a modal (overlay surface, 1px
//! rule, panel corner) minus the scrim, floated at `SPACE_2` from the rect it
//! belongs to and cast with the overlay's hard shadow. There is no arrow.
//! Menus are a popover whose body is menu items ([`Popover::show_menu`]); a
//! greyed command's reason is its one-sentence form ([`Popover::show_reason`]).
//!
//! Stateless, like [`crate::ModalLayer`]: whether a popover is open is the
//! caller's state, and it reports `dismissed` so the caller can close it. The
//! card's size is egui's memory (the `Area` remembers it from the last frame),
//! which is what the flip decision reads; on the first frame the size is not
//! known yet, so the card is placed below/beside and egui's own sizing pass
//! keeps that frame invisible, and the next frame flips and clamps with the
//! measured size.
//!
//! **Dismissal.** A press outside is read from the pointer's interact
//! position: with no pointer there is no outside dismissal. A press on another
//! popover's layer (a submenu over its menu) is not outside.
//!
//! Escape closes only the topmost dismissable popover; no other reports it.
//! egui keeps its layer order private, so "topmost" is read from the order
//! popovers are shown in. The one expected on top is the last dismissable
//! popover shown in the previous pass that did not report `dismissed` there
//! (one that did is being closed by its caller), and it reports the Escape as
//! soon as it is shown. A dismissable popover shown before it in the pass
//! takes the key out of the queue and hands it on, so a modal or picker shown
//! after them never sees it.
//!
//! When the expected popover is not shown this pass (its caller closed it for
//! another reason), or none was expected (each was opened this pass), the
//! Escape goes to the last dismissable popover shown this pass. Which one that
//! is is known only once the pass ends, so the popover left holding the key
//! reports `dismissed` on the next pass, one frame late.
//!
//! A reason is never dismissable and never reads the input queue, so Escape
//! reaches whatever is underneath it.

use std::hash::Hash;

use egui::{Id, Key, LayerId, Modifiers, Order, PointerButton, Pos2, Rect, Vec2};
use meridian_design::semantic;

use crate::key_chip::{chip_width, key_chip_with, KeyInk, KeySize};
use crate::overlay::overlay_surface;
use crate::theme::to_color32;
use crate::tokens::Tokens;
use crate::MeridianUi;

/// Which side of the anchor the card opens on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor {
    /// Below the anchor, leading edges aligned; above when there is no room.
    Below,
    /// Past the anchor's trailing edge, tops aligned; on the leading side when
    /// there is no room. A submenu opens this way.
    Beside,
}

/// What a popover hands back.
pub struct PopoverResponse<R> {
    /// The body closure's return value.
    pub inner: R,
    /// Escape was pressed with this popover topmost (on the pass after the
    /// press, when which popover was topmost was known only once the pass
    /// ended; see the module doc), or a primary press landed outside the card
    /// and outside the anchor (a press on the anchor is the caller's toggle).
    /// The caller stops showing the popover in response.
    pub dismissed: bool,
}

/// The floating card anchored to a rect.
pub struct Popover;

/// Where the card's top left goes: on the preferred side, flipped when the
/// card does not fit within `screen`, then clamped wholly onto it.
fn place(anchor: Rect, size: Vec2, side: Anchor, screen: Rect, gap: f32) -> Pos2 {
    let mut pos = match side {
        Anchor::Below => {
            let mut y = anchor.max.y + gap;
            if y + size.y > screen.max.y {
                y = anchor.min.y - gap - size.y;
            }
            Pos2::new(anchor.min.x, y)
        }
        Anchor::Beside => {
            let mut x = anchor.max.x + gap;
            if x + size.x > screen.max.x {
                x = anchor.min.x - gap - size.x;
            }
            Pos2::new(x, anchor.min.y)
        }
    };
    pos.x = pos.x.min(screen.max.x - size.x).max(screen.min.x);
    pos.y = pos.y.min(screen.max.y - size.y).max(screen.min.y);
    pos
}

/// How the card's body is padded and sized.
#[derive(Clone, Copy)]
enum Form {
    /// As wide as the body, no padding.
    General,
    /// 240 wide, vertical padding only.
    Menu,
    /// 240 wide, padded on all sides.
    Reason,
}

/// One popover shown in a pass.
#[derive(Clone, Copy)]
struct Shown {
    layer: LayerId,
    dismissable: bool,
    /// It reported `dismissed` in that pass, so its caller is closing it.
    dismissed: bool,
}

/// The popover layers shown this pass and the last, in the order they were
/// shown, and the popover holding an Escape that is still to be reported.
#[derive(Clone, Default)]
struct Registry {
    pass: u64,
    prev: Vec<Shown>,
    cur: Vec<Shown>,
    /// The popover that held an Escape when the previous pass ended: it
    /// reports `dismissed` when it is shown this pass.
    prev_held: Option<LayerId>,
    /// The popover holding an Escape this pass, waiting to learn whether a
    /// popover shown after it is higher.
    held: Option<LayerId>,
}

fn registry<T>(ctx: &egui::Context, f: impl FnOnce(&mut Registry) -> T) -> T {
    let key = Id::new("meridian_popover_registry");
    let now = ctx.cumulative_pass_nr();
    ctx.data_mut(|d| {
        let reg = d.get_temp_mut_or_default::<Registry>(key);
        if reg.pass != now {
            // What was recorded belongs to the previous pass only if that is
            // the pass it was recorded in: a pass that showed no popover
            // never opened the registry.
            let previous = reg.pass + 1 == now;
            reg.prev = std::mem::take(&mut reg.cur);
            reg.prev_held = reg.held.take();
            if !previous {
                reg.prev.clear();
                reg.prev_held = None;
            }
            reg.pass = now;
        }
        f(reg)
    })
}

fn show_card<R>(
    ctx: &egui::Context,
    id_salt: impl Hash + std::fmt::Debug,
    anchor: Rect,
    side: Anchor,
    form: Form,
    dismissable: bool,
    body: impl FnOnce(&mut egui::Ui) -> R,
) -> (egui::Response, R, bool) {
    let t: &Tokens = ctx.tokens();
    let style = ctx.style_of(ctx.theme());
    let gap = t.space[2];
    let id = Id::new(("meridian_popover", id_salt));
    let size = egui::AreaState::load(ctx, id)
        .and_then(|s| s.size)
        .unwrap_or(Vec2::ZERO);
    let pos = place(anchor, size, side, ctx.content_rect(), gap);

    let margin = match form {
        Form::General => egui::Margin::ZERO,
        Form::Menu => egui::Margin::symmetric(0, gap as i8),
        Form::Reason => egui::Margin::symmetric(t.space[5] as i8, t.space[4] as i8),
    };
    let frame = overlay_surface(&style, t)
        .inner_margin(margin)
        .shadow(style.visuals.popup_shadow);
    let rule = 1.0;
    let width = match form {
        Form::General => None,
        Form::Menu => Some(t.popover_width - 2.0 * rule),
        Form::Reason => Some(t.popover_width - 2.0 * rule - 2.0 * t.space[5]),
    };

    let area = egui::Area::new(id)
        .order(Order::Foreground)
        .fixed_pos(pos)
        .constrain(false)
        .show(ctx, |ui| {
            frame
                .show(ui, |ui| {
                    if let Some(w) = width {
                        ui.set_width(w);
                    }
                    if matches!(form, Form::Menu) {
                        // Menu items and separators abut: a row's own height is
                        // the whole of its place.
                        ui.spacing_mut().item_spacing.y = 0.0;
                    }
                    body(ui)
                })
                .inner
        });

    let layer = area.response.layer_id;
    if !dismissable {
        registry(ctx, |r| {
            r.cur.push(Shown {
                layer,
                dismissable: false,
                dismissed: false,
            });
        });
        return (area.response, area.inner, false);
    }
    // The popover expected on top: the last dismissable one shown in the
    // previous pass whose caller was not told to close it.
    let (expected, late) = registry(ctx, |r| {
        let expected = r
            .prev
            .iter()
            .rev()
            .find(|e| e.dismissable && !e.dismissed)
            .map(|e| e.layer);
        r.cur.push(Shown {
            layer,
            dismissable: true,
            dismissed: false,
        });
        (expected, r.prev_held == Some(layer))
    });
    let card = area.response.rect;
    let pressed = ctx.input(|i| {
        i.pointer
            .button_pressed(PointerButton::Primary)
            .then(|| i.pointer.interact_pos())
            .flatten()
    });
    let outside = pressed.is_some_and(|p| {
        let on_popover = ctx
            .layer_id_at(p)
            .is_some_and(|l| registry(ctx, |r| r.prev.iter().chain(&r.cur).any(|e| e.layer == l)));
        !card.contains(p) && !anchor.contains(p) && !on_popover
    });
    let consume = || ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape));
    let esc = if expected == Some(layer) {
        // The topmost: it takes the Escape, from the queue or from a popover
        // shown before it this pass that was holding it in case it did not
        // show.
        let held = registry(ctx, |r| r.held.take().is_some());
        held || consume()
    } else {
        // Not expected on top, but it may be the highest shown this pass:
        // hold the Escape, taking it from the queue or from a popover shown
        // before it, until the pass shows whether one shown later is higher.
        let held = registry(ctx, |r| r.held.is_some()) || consume();
        if held {
            registry(ctx, |r| r.held = Some(layer));
            ctx.request_repaint();
        }
        false
    };
    let dismissed = esc || outside || late;
    if dismissed {
        registry(ctx, |r| {
            if let Some(e) = r.cur.iter_mut().rev().find(|e| e.layer == layer) {
                e.dismissed = true;
            }
        });
    }
    (area.response, area.inner, dismissed)
}

impl Popover {
    /// Show a card as wide as its body, no padding.
    pub fn show<R>(
        ctx: &egui::Context,
        id_salt: impl Hash + std::fmt::Debug,
        anchor: Rect,
        side: Anchor,
        body: impl FnOnce(&mut egui::Ui) -> R,
    ) -> PopoverResponse<R> {
        let (_, inner, dismissed) =
            show_card(ctx, id_salt, anchor, side, Form::General, true, body);
        PopoverResponse { inner, dismissed }
    }

    /// Show a menu card: 240 wide, `SPACE_2` above and below, no side padding.
    /// The body's vertical item spacing is zero, so rows and separators abut.
    pub fn show_menu<R>(
        ctx: &egui::Context,
        id_salt: impl Hash + std::fmt::Debug,
        anchor: Rect,
        side: Anchor,
        body: impl FnOnce(&mut egui::Ui) -> R,
    ) -> PopoverResponse<R> {
        let (_, inner, dismissed) = show_card(ctx, id_salt, anchor, side, Form::Menu, true, body);
        PopoverResponse { inner, dismissed }
    }

    /// Show a greyed command's reason: one wrapped sentence in a 240 card and
    /// an optional key at the top right. Never dismissed from here; the caller
    /// decides when it shows. Published as an accesskit `Tooltip`.
    pub fn show_reason(
        ctx: &egui::Context,
        id_salt: impl Hash + std::fmt::Debug,
        anchor: Rect,
        reason: &str,
        keystroke: Option<&str>,
    ) -> egui::Response {
        let (response, (), _) = show_card(
            ctx,
            id_salt,
            anchor,
            Anchor::Below,
            Form::Reason,
            false,
            |ui| {
                let t = ui.tokens();
                let dark = ui.visuals().dark_mode;
                let ink = to_color32(semantic(dark).text.primary);
                let full = ui.available_width();
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let label_w = match keystroke {
                        Some(k) => full - chip_width(ui, k) - t.space[4],
                        None => full,
                    };
                    ui.scope(|ui| {
                        ui.set_width(label_w);
                        ui.add(egui::Label::new(egui::RichText::new(reason).color(ink)).wrap());
                    });
                    if let Some(k) = keystroke {
                        ui.add_space(t.space[4]);
                        key_chip_with(ui, k, KeySize::Row, KeyInk::Plain);
                    }
                });
            },
        );
        let reason = reason.to_owned();
        ctx.accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Tooltip);
            node.set_label(reason);
        });
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN: Rect = Rect::from_min_max(Pos2::ZERO, Pos2::new(400.0, 300.0));

    #[test]
    fn below_flips_above_only_when_it_does_not_fit() {
        let size = Vec2::new(100.0, 80.0);
        let roomy = Rect::from_min_size(Pos2::new(50.0, 50.0), Vec2::new(40.0, 20.0));
        assert_eq!(
            place(roomy, size, Anchor::Below, SCREEN, 4.0),
            Pos2::new(50.0, 74.0)
        );
        let low = Rect::from_min_size(Pos2::new(50.0, 250.0), Vec2::new(40.0, 20.0));
        assert_eq!(
            place(low, size, Anchor::Below, SCREEN, 4.0),
            Pos2::new(50.0, 166.0)
        );
    }
}
