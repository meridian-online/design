//! Two-pass column alignment: the mechanism that lets every field in a panel
//! share one aligned value column instead of each row guessing its own split.
//!
//! Ported from the rerun viewer's `re_ui` crate — the `list_item` scope module
//! (<https://github.com/rerun-io/rerun>, `crates/viewer/re_ui/src/list_item/scope.rs>`),
//! © the rerun contributors, licensed MIT OR Apache-2.0. The port keeps rerun's
//! two-pass architecture and its width heuristics; it drops the statistics that
//! are specific to rerun's `ListItem` content types (the action-button gutter
//! and the property-content width), which have no counterpart here.
//!
//! # How it works
//!
//! Immediate-mode UIs lay out top-to-bottom in a single pass, so a row cannot
//! know the widest label *below* it. The fix is temporal, not spatial:
//!
//! - On frame *n*, every row inside an [`align_scope`] measures the width its
//!   left column wants and registers it
//!   ([`LayoutInfo::register_desired_left_column_width`]). The maximum
//!   accumulates in egui temp memory, keyed by the scope's id.
//! - On frame *n + 1*, [`align_scope`] reads the accumulated maximum, resets
//!   the accumulator, and hands the value to every row through [`LayoutInfo`]
//!   — so all rows split at the same x, and the accumulation restarts for the
//!   frame after.
//!
//! The very first frame has no accumulated statistics, so the scope requests a
//! discarded pass ([`egui::Context::request_discard`]) rather than show one
//! jittery mis-aligned frame; [`LayoutInfo::left_column_width`] is `None` in
//! that pass and rows fall back to their own width.
//!
//! Scopes nest: each [`align_scope`] pushes its [`LayoutInfo`] onto a stack in
//! egui memory and pops it on the way out, so an inner scope aligns
//! independently of the outer one. Sibling scopes with different ids do not
//! share statistics.

use egui::NumExt as _;

/// Layout statistics accumulated during frame *n* to drive frame *n + 1*.
///
/// Stored in egui temp memory against the scope id; reset by [`align_scope`]
/// at the start of every frame after reading.
#[derive(Debug, Clone, Default)]
struct LayoutStatistics {
    /// Maximum desired left-column width registered this frame, measured from
    /// [`LayoutInfo::left_x`].
    max_desired_left_column_width: Option<f32>,

    /// Maximum item width registered this frame, measured from
    /// [`LayoutInfo::left_x`] to the right edge of the item.
    max_item_width: Option<f32>,
}

impl LayoutStatistics {
    /// Reset the accumulator for a new frame.
    fn reset(ctx: &egui::Context, scope_id: egui::Id) {
        ctx.data_mut(|writer| {
            writer.insert_temp(scope_id, Self::default());
        });
    }

    /// Read the value accumulated over the previous frame.
    fn read(ctx: &egui::Context, scope_id: egui::Id) -> Self {
        if let Some(slf) = ctx.data(|reader| reader.get_temp(scope_id)) {
            slf
        } else {
            // First frame in this scope: layout would be jittery-wrong, so ask
            // egui to discard this pass and lay out again with the statistics
            // this pass is about to gather (rerun does exactly this).
            ctx.request_discard("meridian_egui::align first-frame statistics");
            Self::default()
        }
    }

    /// Update the accumulator. Used by [`LayoutInfo`]'s registration methods.
    fn update(ui: &egui::Ui, scope_id: egui::Id, update: impl FnOnce(&mut Self)) {
        ui.data_mut(|writer| {
            let stats: &mut Self = writer.get_temp_mut_or_default(scope_id);
            update(stats);
        });
    }
}

/// Layout information prepared by [`align_scope`] for the rows inside it.
///
/// Two purposes, exactly as in rerun:
///
/// - read-only layout state for drawing this frame (where the shared column
///   splits), and
/// - the registration API whose values become *next* frame's layout state.
///
/// Rows obtain the current scope's instance with [`LayoutInfo::current`].
#[derive(Debug, Clone)]
pub struct LayoutInfo {
    /// Left-most x of the scope — the reference every registered width is
    /// measured from. Set by [`align_scope`] from `ui.max_rect()`.
    left_x: f32,

    /// The shared left-column width to lay out with this frame, measured from
    /// [`Self::left_x`] (so it includes any indentation between the scope's
    /// left edge and the row).
    ///
    /// `None` during the scope's first frame, when nothing has been
    /// accumulated yet — rows must fall back to their own natural width.
    left_column_width: Option<f32>,

    /// The scope this info belongs to; keys the [`LayoutStatistics`].
    scope_id: egui::Id,
}

impl Default for LayoutInfo {
    fn default() -> Self {
        Self {
            left_x: 0.0,
            left_column_width: None,
            scope_id: egui::Id::NULL,
        }
    }
}

impl LayoutInfo {
    /// Left-most x coordinate of the scope — the origin registered widths are
    /// measured from.
    #[must_use]
    pub fn left_x(&self) -> f32 {
        self.left_x
    }

    /// The shared left-column width for this frame, or `None` on the scope's
    /// first frame (fall back to the row's own width).
    #[must_use]
    pub fn left_column_width(&self) -> Option<f32> {
        self.left_column_width
    }

    /// Register the width this row's left column wants, measured from
    /// [`Self::left_x`]. Every aligned row should call this once per frame;
    /// the maximum becomes next frame's [`Self::left_column_width`].
    pub fn register_desired_left_column_width(&self, ui: &egui::Ui, desired_width: f32) {
        LayoutStatistics::update(ui, self.scope_id, |stats| {
            stats.max_desired_left_column_width = stats
                .max_desired_left_column_width
                .map(|v| v.max(desired_width))
                .or(Some(desired_width));
        });
    }

    /// Register the full width of an item, measured from [`Self::left_x`] to
    /// its right edge. Feeds the cap that stops a runaway label column from
    /// eating the whole row.
    pub fn register_max_item_width(&self, ui: &egui::Ui, width: f32) {
        LayoutStatistics::update(ui, self.scope_id, |stats| {
            stats.max_item_width = stats.max_item_width.map(|v| v.max(width)).or(Some(width));
        });
    }

    /// The innermost enclosing scope's layout info, or `None` when the caller
    /// is not inside an [`align_scope`].
    ///
    /// rerun treats "outside a scope" as a bug and warns; here it is a
    /// supported degradation — a row drawn outside any scope aligns by its own
    /// width (alignment-by-convention) instead of by the shared column.
    #[must_use]
    pub fn current(ctx: &egui::Context) -> Option<Self> {
        LayoutInfoStack::top(ctx)
    }
}

/// Stack of [`LayoutInfo`]s in egui memory, so [`align_scope`]s can nest.
///
/// Keyed at [`egui::Id::NULL`] like rerun's original — egui's temp memory is
/// keyed by `(Id, TypeId)`, and this private type keeps the slot isolated.
#[derive(Debug, Clone, Default)]
struct LayoutInfoStack(Vec<LayoutInfo>);

impl LayoutInfoStack {
    fn push(ctx: &egui::Context, state: LayoutInfo) {
        ctx.data_mut(|writer| {
            let stack: &mut Self = writer.get_temp_mut_or_default(egui::Id::NULL);
            stack.0.push(state);
        });
    }

    fn pop(ctx: &egui::Context) -> Option<LayoutInfo> {
        ctx.data_mut(|writer| {
            let stack: &mut Self = writer.get_temp_mut_or_default(egui::Id::NULL);
            stack.0.pop()
        })
    }

    fn top(ctx: &egui::Context) -> Option<LayoutInfo> {
        ctx.data_mut(|writer| {
            let stack: &mut Self = writer.get_temp_mut_or_default(egui::Id::NULL);
            stack.0.last().cloned()
        })
    }
}

/// Create a scope in which rows share one aligned left column.
///
/// Reads the statistics the previous frame accumulated, resets the
/// accumulator, and exposes the resulting [`LayoutInfo`] to everything inside
/// `content` (via [`LayoutInfo::current`]). Scopes may nest — each keeps its
/// own statistics — and two sibling scopes with different `id_salt`s are
/// independent.
///
/// Mirrors rerun's `list_item_scope`:
///
/// - the scope id derives from `id_salt` combined with the `Ui`'s id, so the
///   salt only needs to be unique among siblings;
/// - the content runs under [`egui::Ui::push_id`], so equal salts in sibling
///   uis do not clash;
/// - vertical item spacing is zeroed inside the scope (rows own their air);
/// - the shared column is capped at 70% of the widest item (or of the
///   available width before any item has registered), so a pathological label
///   cannot push every value off the row.
pub fn align_scope<R>(
    ui: &mut egui::Ui,
    id_salt: impl egui::AsId,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    let id_salt = egui::Id::new(id_salt);
    let scope_id = ui.id().with(id_salt);

    // Read the previous frame's statistics and reset for this frame.
    let layout_stats = LayoutStatistics::read(ui.ctx(), scope_id);
    LayoutStatistics::reset(ui.ctx(), scope_id);

    // Prepare this frame's layout info from them.
    let left_column_width =
        if let Some(max_desired_left_column_width) = layout_stats.max_desired_left_column_width {
            // Rerun's heuristic, kept as-is: never let the left column claim
            // more than 70% of the widest item (falling back to the available
            // width when no item has registered yet).
            let available_width = layout_stats
                .max_item_width
                .unwrap_or_else(|| ui.available_width());
            Some(max_desired_left_column_width.at_most(0.7 * available_width))
        } else {
            None
        };
    let state = LayoutInfo {
        left_x: ui.max_rect().left(),
        left_column_width,
        scope_id,
    };

    // Push, run, pop.
    LayoutInfoStack::push(ui.ctx(), state);
    let response = ui.push_id(id_salt, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        content(ui)
    });
    LayoutInfoStack::pop(ui.ctx());

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A context whose screen rect is fixed and which never re-runs a frame on
    /// `request_discard`, so the two-pass behaviour is observable frame by
    /// frame.
    fn single_pass_ctx() -> (egui::Context, egui::RawInput) {
        let ctx = egui::Context::default();
        ctx.options_mut(|o| o.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(800.0, 600.0),
            )),
            ..Default::default()
        };
        (ctx, input)
    }

    /// Frame n registers widths; frame n+1 consumes their maximum. The first
    /// frame exposes no width at all.
    #[test]
    fn second_frame_consumes_the_first_frames_maximum() {
        let (ctx, input) = single_pass_ctx();
        let mut widths_seen: Vec<Option<f32>> = Vec::new();

        for _ in 0..3 {
            let _ = ctx.run_ui(input.clone(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    align_scope(ui, "fields", |ui| {
                        let info = LayoutInfo::current(ui.ctx()).expect("inside a scope");
                        widths_seen.push(info.left_column_width());
                        info.register_desired_left_column_width(ui, 80.0);
                        info.register_desired_left_column_width(ui, 120.0);
                        info.register_desired_left_column_width(ui, 60.0);
                    });
                });
            });
        }

        assert_eq!(widths_seen[0], None, "first frame has nothing accumulated");
        assert_eq!(widths_seen[1], Some(120.0), "second frame gets the maximum");
        assert_eq!(widths_seen[2], Some(120.0), "steady state holds");
    }

    /// The accumulator resets every frame: when the wide row disappears, the
    /// column narrows on the following frame rather than sticking.
    #[test]
    fn column_narrows_after_the_wide_row_leaves() {
        let (ctx, input) = single_pass_ctx();
        let mut widths_seen: Vec<Option<f32>> = Vec::new();

        for frame in 0..3 {
            let _ = ctx.run_ui(input.clone(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    align_scope(ui, "fields", |ui| {
                        let info = LayoutInfo::current(ui.ctx()).expect("inside a scope");
                        widths_seen.push(info.left_column_width());
                        info.register_desired_left_column_width(ui, 50.0);
                        if frame == 0 {
                            info.register_desired_left_column_width(ui, 200.0);
                        }
                    });
                });
            });
        }

        assert_eq!(widths_seen[1], Some(200.0), "wide row from frame 0 applies");
        assert_eq!(widths_seen[2], Some(50.0), "frame 1 no longer saw it");
    }

    /// The shared column is capped at 70% of the widest registered item.
    #[test]
    fn left_column_is_capped_against_the_widest_item() {
        let (ctx, input) = single_pass_ctx();
        let mut widths_seen: Vec<Option<f32>> = Vec::new();

        for _ in 0..2 {
            let _ = ctx.run_ui(input.clone(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    align_scope(ui, "fields", |ui| {
                        let info = LayoutInfo::current(ui.ctx()).expect("inside a scope");
                        widths_seen.push(info.left_column_width());
                        info.register_desired_left_column_width(ui, 500.0);
                        info.register_max_item_width(ui, 400.0);
                    });
                });
            });
        }

        assert_eq!(
            widths_seen[1],
            Some(0.7 * 400.0),
            "desired width is clamped to 70% of the widest item"
        );
    }

    /// Sibling scopes with different ids accumulate independently.
    #[test]
    fn sibling_scopes_do_not_share_statistics() {
        let (ctx, input) = single_pass_ctx();
        let mut widths_a: Vec<Option<f32>> = Vec::new();
        let mut widths_b: Vec<Option<f32>> = Vec::new();

        for _ in 0..2 {
            let _ = ctx.run_ui(input.clone(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    align_scope(ui, "a", |ui| {
                        let info = LayoutInfo::current(ui.ctx()).expect("inside a scope");
                        widths_a.push(info.left_column_width());
                        info.register_desired_left_column_width(ui, 100.0);
                    });
                    align_scope(ui, "b", |ui| {
                        let info = LayoutInfo::current(ui.ctx()).expect("inside a scope");
                        widths_b.push(info.left_column_width());
                        info.register_desired_left_column_width(ui, 40.0);
                    });
                });
            });
        }

        assert_eq!(widths_a[1], Some(100.0));
        assert_eq!(widths_b[1], Some(40.0));
    }

    /// Nested scopes stack: the inner scope's info shadows the outer one's
    /// while it is open, and the outer one is restored after.
    #[test]
    fn nested_scopes_stack_and_restore() {
        let (ctx, input) = single_pass_ctx();
        let mut outer_widths: Vec<Option<f32>> = Vec::new();
        let mut inner_widths: Vec<Option<f32>> = Vec::new();
        let mut restored_widths: Vec<Option<f32>> = Vec::new();

        for _ in 0..2 {
            let _ = ctx.run_ui(input.clone(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    align_scope(ui, "outer", |ui| {
                        let outer = LayoutInfo::current(ui.ctx()).expect("outer scope");
                        outer_widths.push(outer.left_column_width());
                        outer.register_desired_left_column_width(ui, 150.0);

                        align_scope(ui, "inner", |ui| {
                            let inner = LayoutInfo::current(ui.ctx()).expect("inner scope");
                            inner_widths.push(inner.left_column_width());
                            inner.register_desired_left_column_width(ui, 30.0);
                        });

                        let restored = LayoutInfo::current(ui.ctx()).expect("outer restored");
                        restored_widths.push(restored.left_column_width());
                    });
                });
            });
        }

        assert_eq!(outer_widths[1], Some(150.0));
        assert_eq!(inner_widths[1], Some(30.0), "inner accumulates on its own");
        assert_eq!(restored_widths[1], Some(150.0), "outer info is restored");
    }

    /// Outside any scope there is no layout info — the supported degradation
    /// rows fall back on.
    #[test]
    fn outside_a_scope_there_is_no_current_info() {
        let ctx = egui::Context::default();
        assert!(LayoutInfo::current(&ctx).is_none());
    }
}
