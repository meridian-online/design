//! The semantic layer: what a colour is *for*, in framework-neutral form.
//!
//! Until now this vocabulary existed in exactly one place — the desktop theme
//! emitter, expressed in one framework's key names ("popover.background",
//! "tab_bar.segmented.background", …). That emitter's consumer is going away.
//! The vocabulary is the valuable part and it is not framework-specific, so it
//! lives here as plain `Copy` data (ADR 0003) and the emitters become what
//! they always should have been: renamings.
//!
//! Structure:
//!
//! - [`Surfaces`] — the planes a thing can sit on.
//! - [`Borders`] — the rules between them, including the one that is gated for
//!   contrast because it identifies a control.
//! - [`Text`] — ink slots, from primary down to disabled, plus on-solid and
//!   link.
//! - [`Role`] + [`RoleColours`] — the interaction roles, each carrying three
//!   instantiated [`StateSet`]s (background, foreground, border) across
//!   base/hover/active/selected/focus/disabled.
//! - Component families ([`Rows`], [`Tabs`], [`Scrollbar`], [`Editor`],
//!   [`DragDrop`], [`Feedback`], [`Containers`]) — the rest of the emitter's
//!   vocabulary, grouped by what it describes rather than by framework key.
//!
//! Step semantics follow ADR 0007 (Radix anatomy on Meridian scales): 1–2 app
//! backgrounds, 3–5 component states, 6–8 borders, 9–10 solids, 11–12 text.
//! Where this module deliberately departs from that grammar it says so at the
//! field.
//!
//! Contrast is gated in `tests/chrome_gate.rs` using the crate's own WCAG
//! helper (`validate::contrast`) — no second contrast model (that would
//! contradict ADR 0006/0007).

use crate::chrome::{INK_DARK, INK_LIGHT};
use crate::colour::{Rgba, StateSet};
use crate::scales::*;
use crate::viz::STATUS;

/// True black. Not a token and not on any scale — the one paint that dims a
/// dark plane (every Meridian grey is warm, and a warm grey over a dark plane
/// lightens it). Named here so the dark scrim can say what it is instead of
/// spelling `0x00, 0x00, 0x00` inline.
const BLACK: Rgba = Rgba::from_u8(0x00, 0x00, 0x00, 0xff);

/// The planes. "Raised" here is [`crate::elevation::Elevation::Raised`]: a
/// distinct region on the working plane, separated by a hairline and casting
/// nothing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Surfaces {
    /// The app plane behind everything (step 1–2 territory).
    pub app: Rgba,
    /// Panels, cards, tables, editors — the reading surface.
    pub raised: Rgba,
    /// Recessed regions: inputs on a panel, wells, code blocks.
    pub sunken: Rgba,
    /// Menus, popovers, tooltips, dialogs — the floating plane.
    ///
    /// **Asymmetric by mode, deliberately.** In light this equals
    /// [`Self::raised`]: a popover over a panel has *zero* surface contrast
    /// and is separated by its hairline and its shadow alone. In dark it
    /// takes its own lighter step. That is not an oversight and not a
    /// missing light-mode step — it is the quiet-chrome rule
    /// (`guidelines/identity.md`) applied per mode. Light chrome separates
    /// by edge: the light scale's steps 1–3 span a few percent of luminance,
    /// so a distinct overlay tint reads as a dirty panel rather than as
    /// depth, while the shadow ([`crate::elevation::Elevation::Overlay`])
    /// already carries "floating". Dark chrome cannot separate by shadow —
    /// a dark shadow on a dark plane is invisible — so lightening the
    /// surface is the only signal left, and dark's steps climb fast enough
    /// for one step to read.
    ///
    /// Consequence for consumers: in light mode an overlay **must** paint
    /// its hairline and its shadow. It is the only thing distinguishing it.
    /// Asserted in `tests/chrome_gate.rs`.
    pub overlay: Rgba,
    /// Navigation rail / sidebar plane.
    pub sidebar: Rgba,
    /// Title bar, tab bar, status bar — the chrome bands.
    pub header: Rgba,
    /// The dim behind a modal. Alpha by definition.
    pub scrim: Rgba,
}

/// Rules between planes.
///
/// **Border steps, canonised.** ADR 0007 and `scales.rs` both document steps
/// 6–8 as the border band, and the two emitters had drifted off it in opposite
/// directions (one used step 4 for its main border and step 5 for input
/// borders; the CSS side emitted no chrome border at all and used step 3 for
/// gridlines). The band is the ground truth, so: [`Self::subtle`] = step 6,
/// [`Self::default_`] = step 7, [`Self::strong`] = step 8, asserted in
/// `tests/chrome_gate.rs`.
///
/// Two slots sit outside the band on purpose, both documented at the field:
/// [`Self::divider`] (below it) and [`Self::control`] (above it).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Borders {
    /// Step 6 — panel edges, header rules, window border. The default
    /// hairline.
    pub subtle: Rgba,
    /// Step 7 — the visible edge: input outlines, resting control borders,
    /// anywhere the rule must be found rather than felt. `default_` because
    /// `default` is a keyword-adjacent trap in most consumers.
    pub default_: Rgba,
    /// Step 8 — emphasis: hovered control edges, a drag target's outline.
    pub strong: Rgba,
    /// **Above the border band, deliberately** — step 9 in light, step 10 in
    /// dark. WCAG 1.4.11 asks 3:1 of any boundary that identifies a control,
    /// and the whole 6–8 band tops out near 2:1 against the surfaces. The dark
    /// scale needs the extra step because it climbs more slowly and this
    /// boundary must also survive the selection wash. Use this for the
    /// boundary a user must find to know a control exists (an empty input, a
    /// segmented group). Gated at 3:1 in `tests/chrome_gate.rs`.
    pub control: Rgba,
    /// The focus ring colour — Maritime, because focus is the purest case of
    /// "Maritime is a verb" (`guidelines/identity.md`). Gated at 3:1 against
    /// every surface. Geometry lives in [`crate::focus`].
    pub focus: Rgba,
    /// Step 3 — **below the border band, deliberately.** A rule that repeats
    /// at every row (table row rules, chart gridlines) reads as a lattice if
    /// drawn at step 6.
    ///
    /// This *is* the value the chart layer calls a gridline: it is assigned
    /// from [`crate::chrome`]'s `gridline` rather than restated, so a UI
    /// divider and a chart gridline are one definition and cannot drift.
    /// `tests/chrome_gate.rs` also pins that definition to step 3, so a
    /// chrome edit that moved the gridline off the band would redden rather
    /// than silently drag every table rule with it.
    pub divider: Rgba,
}

/// Ink slots.
///
/// [`Self::primary`], [`Self::secondary`], [`Self::link`] and the on-solid
/// pair are gated at 4.5:1. [`Self::muted`] and [`Self::placeholder`] are
/// gated at 3:1 only, and that is a real limit, not an oversight: on Radix
/// anatomy only steps 11–12 are text steps, so any slot below them buys
/// quietness with contrast. Rule of use — **if losing the text loses meaning,
/// it wears `primary` or `secondary`.** `muted` is for supporting annotation
/// that repeats information available elsewhere; `placeholder` for hint text
/// inside an empty control; `disabled` is ungated (WCAG exempts inactive
/// controls) and must never be the only signal that something is unavailable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text {
    /// Step 12 — titles, values, anything load-bearing. Assigned from
    /// [`crate::chrome`]'s `ink_primary`: chrome is the anchor for every ink
    /// slot that also exists inside a chart.
    pub primary: Rgba,
    /// Step 11 — labels, column headers, secondary values. Assigned from
    /// chrome's `ink_secondary`.
    pub secondary: Rgba,
    /// Step 10 — supporting annotation.
    ///
    /// **The one ink slot with no chrome counterpart, deliberately.** Chrome
    /// has three inks (primary, secondary, muted) because a chart needs
    /// three; the UI needs four, because it has both quiet annotation and
    /// placeholder text and they cannot be the same value. So the two layers
    /// line up like this, and it is checked in `tests/chrome_gate.rs`:
    ///
    /// | slot | step | chrome |
    /// |---|---|---|
    /// | [`Self::primary`] | 12 | `ink_primary` |
    /// | [`Self::secondary`] | 11 | `ink_secondary` |
    /// | `muted` (this) | 10 | *none — one step above chrome's muted* |
    /// | [`Self::placeholder`] | 9 | `ink_muted` |
    ///
    /// Chrome's `ink_muted` is the *quietest* ink in its layer, and it is
    /// step 9 — the same step this layer calls `placeholder`. That is the
    /// identity, and `placeholder` is assigned from it. `muted` then sits one
    /// step *up* from it in both modes (which is why the raw hexes appear to
    /// move in opposite directions: light steps darken as they climb, dark
    /// steps lighten, and the step relation is what is stable). The extra
    /// step is bought for a reason — chrome's muted ink only ever sits on the
    /// chart surface, whereas UI muted text sits on hovered rows, the
    /// selection wash and the sunken plane, and step 9 does not survive all
    /// of those at 3:1.
    pub muted: Rgba,
    /// Step 9 — placeholder/hint inside an empty control. Assigned from
    /// chrome's `ink_muted`, which is the same step: see [`Self::muted`] for
    /// why this layer has four ink slots where chrome has three.
    pub placeholder: Rgba,
    /// Step 8 — disabled ink.
    pub disabled: Rgba,
    /// Ink on a dark solid (the accent, danger, success, info roles). The
    /// warm near-white surface colour, not pure white — pure white on warm
    /// chrome reads blue.
    pub on_solid: Rgba,
    /// Ink on a *bright* solid — the warning role. ADR 0007 carries the Radix
    /// caveat: bright scales design their solid for black foreground text.
    pub on_solid_bright: Rgba,
    /// Link at rest.
    pub link: Rgba,
    /// Link hovered.
    pub link_hover: Rgba,
    /// Link pressed. This equals [`Self::link_hover`] in **both** modes, and
    /// that is a measured limit rather than an oversight: a link must clear
    /// 4.5:1 over the selection wash as well as over every plane, and only two
    /// steps of the accent scale do. The pressed state is therefore carried by
    /// the focus ring and the pointer, not by a third ink.
    pub link_active: Rgba,
}

/// The interaction roles. Small on purpose — a role is a *meaning*, and the
/// list of meanings a data tool needs is short.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Default controls: secondary buttons, list rows, toolbar items. Wears
    /// chrome, not colour.
    Neutral,
    /// The primary action, selection, links-as-buttons. Maritime.
    Accent,
    /// Destructive and error.
    Danger,
    /// Confirmation and healthy state.
    Success,
    /// Caution. Takes bright-solid ink ([`Text::on_solid_bright`]).
    Warning,
    /// Neutral information — never used for "good".
    Info,
}

impl Role {
    /// Every role, in the order [`Semantic::roles`] stores them.
    pub const ALL: [Role; 6] = [
        Role::Neutral,
        Role::Accent,
        Role::Danger,
        Role::Success,
        Role::Warning,
        Role::Info,
    ];

    /// Index into [`Semantic::roles`].
    pub const fn index(self) -> usize {
        match self {
            Role::Neutral => 0,
            Role::Accent => 1,
            Role::Danger => 2,
            Role::Success => 3,
            Role::Warning => 4,
            Role::Info => 5,
        }
    }

    /// Whether this role paints a filled solid (and therefore needs on-solid
    /// ink) or wears chrome.
    pub const fn is_solid(self) -> bool {
        !matches!(self, Role::Neutral)
    }

    /// Stable machine name, used by the CSS emitter and by any adapter that
    /// needs a key.
    pub const fn name(self) -> &'static str {
        match self {
            Role::Neutral => "neutral",
            Role::Accent => "accent",
            Role::Danger => "danger",
            Role::Success => "success",
            Role::Warning => "warning",
            Role::Info => "info",
        }
    }
}

/// One role's three instantiated [`StateSet`]s.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoleColours {
    /// Fill.
    pub background: StateSet,
    /// Ink on that fill.
    pub foreground: StateSet,
    /// Edge. For solid roles this tracks the fill so a consumer that always
    /// strokes gets no seam; `border.focus` is always the ring colour.
    pub border: StateSet,
}

/// List and table surfaces — one vocabulary, because a table is a list with
/// columns and giving them separate tokens is how they drift apart.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rows {
    pub background: Rgba,
    pub header_background: Rgba,
    pub header_foreground: Rgba,
    pub footer_background: Rgba,
    pub footer_foreground: Rgba,
    /// Zebra striping (the even row).
    pub stripe_background: Rgba,
    pub hover_background: Rgba,
    pub selected_background: Rgba,
    pub selected_border: Rgba,
    /// The repeated rule between rows — [`Borders::divider`].
    pub row_border: Rgba,
}

/// Tab strips and segmented controls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tabs {
    pub bar_background: Rgba,
    pub segmented_background: Rgba,
    pub background: Rgba,
    pub foreground: Rgba,
    pub active_background: Rgba,
    pub active_foreground: Rgba,
}

/// Scrollbars.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scrollbar {
    pub track: Rgba,
    pub thumb: Rgba,
    pub thumb_hover: Rgba,
}

/// Text editing surfaces (the spec editor, inline cell edit).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Editor {
    pub caret: Rgba,
    /// Text selection wash.
    pub selection: Rgba,
}

/// Drag and drop affordances.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragDrop {
    /// Wash over a valid drop area. Alpha by definition.
    pub drop_target: Rgba,
    /// Outline of the thing being dragged / its insertion point.
    pub drag_border: Rgba,
}

/// Progress, loading and slider chrome.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Feedback {
    /// Loading placeholder block.
    pub skeleton: Rgba,
    pub progress_track: Rgba,
    pub progress_bar: Rgba,
    pub slider_track: Rgba,
    pub slider_thumb: Rgba,
}

/// Named containers that are not roles and not rows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Containers {
    pub sidebar_background: Rgba,
    pub sidebar_border: Rgba,
    pub title_bar_background: Rgba,
    pub status_bar_background: Rgba,
    /// The tiling/dock background behind panels.
    pub tiles_background: Rgba,
    pub group_box_background: Rgba,
    pub group_box_title_foreground: Rgba,
    pub description_label_background: Rgba,
    pub description_label_foreground: Rgba,
    pub accordion_background: Rgba,
    pub accordion_hover_background: Rgba,
    pub popover_background: Rgba,
    pub window_border: Rgba,
}

/// One mode's complete semantic layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Semantic {
    pub surfaces: Surfaces,
    pub borders: Borders,
    pub text: Text,
    /// Indexed by [`Role::index`]; use [`Semantic::role`].
    pub roles: [RoleColours; 6],
    pub rows: Rows,
    pub tabs: Tabs,
    pub scrollbar: Scrollbar,
    pub editor: Editor,
    pub drag_drop: DragDrop,
    pub feedback: Feedback,
    pub containers: Containers,
}

impl Semantic {
    pub const fn role(&self, role: Role) -> &RoleColours {
        &self.roles[role.index()]
    }

    /// Every surface a control or text can sit on — what the contrast gate
    /// iterates.
    pub const fn planes(&self) -> [Rgba; 5] {
        [
            self.surfaces.app,
            self.surfaces.raised,
            self.surfaces.sunken,
            self.surfaces.overlay,
            self.surfaces.sidebar,
        ]
    }
}

/// The mode's layer. `dark` mirrors light with identical slot meaning — never
/// a naive inversion (`guidelines/identity.md`).
pub const fn semantic(dark: bool) -> &'static Semantic {
    if dark {
        &DARK
    } else {
        &LIGHT
    }
}

// ---------------------------------------------------------------------------
// Light
// ---------------------------------------------------------------------------

pub const LIGHT: Semantic = {
    let g = GRAY_LIGHT;
    let m = MARITIME_LIGHT;
    let r = RED_LIGHT;
    let n = GREEN_LIGHT;
    let a = AMBER_LIGHT;
    let b = BLUE_LIGHT;

    // Ink on a solid is the light surface colour in BOTH modes — it is a
    // paint, not a mode-dependent slot. Sourced from `chrome.rs` so a change
    // there propagates instead of being restated.
    let on_solid = INK_LIGHT.surface;
    let on_bright = INK_LIGHT.ink_primary;
    let disabled_fill = g[4];
    let disabled_ink = g[9];

    let surfaces = Surfaces {
        app: INK_LIGHT.page,
        raised: INK_LIGHT.surface,
        sunken: g[1],
        overlay: INK_LIGHT.surface,
        sidebar: g[0],
        header: g[1],
        // The primary ink at 20% — a scrim is the page's own ink dimming the
        // page, not a fourth grey. Read from chrome, never restated.
        scrim: INK_LIGHT.ink_primary.with_alpha_u8(0x33),
    };

    let borders = Borders {
        subtle: g[5],
        default_: g[6],
        strong: g[7],
        control: g[8],
        focus: m[8],
        // One definition with the chart layer's gridline — see the field doc.
        divider: INK_LIGHT.gridline,
    };

    let text = Text {
        primary: INK_LIGHT.ink_primary,
        secondary: INK_LIGHT.ink_secondary,
        // Step 10. The only ink slot chrome has no counterpart for, and the
        // reason is at `Text::muted`.
        muted: g[9],
        placeholder: INK_LIGHT.ink_muted,
        disabled: g[7],
        on_solid,
        on_solid_bright: on_bright,
        link: m[10],
        link_hover: m[11],
        link_active: m[11],
    };

    // Solid roles: base/hover/active pick the scale steps that keep on-solid
    // ink at or above 4.5:1 (see `tests/chrome_gate.rs`). That is one step
    // darker than the Radix step-9 solid for the accent and success scales,
    // whose step 9 lands at 4.49 and 4.27 — under AA for body-size ink.
    let neutral = RoleColours {
        background: StateSet {
            base: g[1],
            hover: g[2],
            active: g[3],
            selected: m[2],
            focus: g[1],
            disabled: g[1],
        },
        foreground: StateSet {
            base: g[11],
            hover: g[11],
            active: g[11],
            selected: g[11],
            focus: g[11],
            disabled: disabled_ink,
        },
        border: StateSet {
            base: borders.default_,
            hover: borders.strong,
            active: borders.control,
            selected: m[5],
            focus: borders.focus,
            disabled: g[4],
        },
    };

    let accent = solid_role(
        m[9],
        m[10],
        m[11],
        m[5],
        on_solid,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );
    let danger = solid_role(
        r[8],
        r[9],
        r[11],
        r[5],
        on_solid,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );
    let success = solid_role(
        n[9],
        n[10],
        n[11],
        n[5],
        on_solid,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );
    // Warning lifts on hover and sinks on press: the bright scale's darker
    // steps cannot carry the dark ink (step 11 falls to 3.3:1), so the usual
    // darken-on-hover progression is unavailable. ADR 0007's bright-scale
    // caveat, made concrete.
    let warning = solid_role(
        a[8],
        a[7],
        a[9],
        a[5],
        on_bright,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );
    let info = solid_role(
        b[8],
        b[9],
        b[11],
        b[5],
        on_solid,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );

    Semantic {
        surfaces,
        borders,
        text,
        roles: [neutral, accent, danger, success, warning, info],
        rows: Rows {
            background: surfaces.raised,
            header_background: g[1],
            header_foreground: g[10],
            footer_background: g[1],
            footer_foreground: g[10],
            stripe_background: g[0],
            hover_background: g[2],
            selected_background: m[2],
            selected_border: m[5],
            row_border: borders.divider,
        },
        tabs: Tabs {
            bar_background: g[1],
            segmented_background: g[2],
            background: g[1],
            foreground: g[10],
            active_background: surfaces.raised,
            active_foreground: g[11],
        },
        scrollbar: Scrollbar {
            track: g[1],
            thumb: g[5],
            thumb_hover: g[6],
        },
        editor: Editor {
            caret: g[11],
            selection: m[3],
        },
        drag_drop: DragDrop {
            // The Maritime anchor at 15% — the accent as a verb, washed.
            // Read from the anchor so it cannot drift off the brand colour.
            drop_target: crate::MARITIME.with_alpha_u8(0x26),
            drag_border: m[7],
        },
        feedback: Feedback {
            skeleton: g[2],
            progress_track: g[4],
            progress_bar: m[9],
            slider_track: g[4],
            slider_thumb: m[9],
        },
        containers: Containers {
            sidebar_background: g[0],
            sidebar_border: borders.subtle,
            title_bar_background: surfaces.app,
            status_bar_background: surfaces.app,
            tiles_background: surfaces.app,
            group_box_background: g[1],
            group_box_title_foreground: g[10],
            description_label_background: g[1],
            description_label_foreground: g[10],
            accordion_background: surfaces.raised,
            accordion_hover_background: g[2],
            popover_background: surfaces.overlay,
            window_border: borders.subtle,
        },
    }
};

// ---------------------------------------------------------------------------
// Dark
// ---------------------------------------------------------------------------

pub const DARK: Semantic = {
    let g = GRAY_DARK;
    let m = MARITIME_DARK;
    let r = RED_DARK;
    let n = GREEN_DARK;
    let a = AMBER_DARK;
    let b = BLUE_DARK;

    let on_solid = INK_LIGHT.surface;
    let on_bright = INK_LIGHT.ink_primary;
    let disabled_fill = g[4];
    let disabled_ink = g[9];

    let surfaces = Surfaces {
        app: INK_DARK.page,
        raised: INK_DARK.surface,
        sunken: g[1],
        overlay: g[2],
        sidebar: g[0],
        header: g[1],
        // True black at 40%, and this one is *not* the primary ink: dimming
        // a dark plane with its own near-white ink lightens it. Black is the
        // only paint that dims here, so it is stated once, as itself.
        scrim: BLACK.with_alpha_u8(0x66),
    };

    let borders = Borders {
        subtle: g[5],
        default_: g[6],
        strong: g[7],
        // Step 10, not 9 — see the `Borders::control` doc comment.
        control: g[9],
        focus: m[10],
        // One definition with the chart layer's gridline — see the field doc.
        divider: INK_DARK.gridline,
    };

    let text = Text {
        primary: INK_DARK.ink_primary,
        secondary: INK_DARK.ink_secondary,
        // Step 10. The only ink slot chrome has no counterpart for, and the
        // reason is at `Text::muted`.
        muted: g[9],
        placeholder: INK_DARK.ink_muted,
        disabled: g[7],
        on_solid,
        on_solid_bright: on_bright,
        link: m[10],
        link_hover: m[11],
        link_active: m[11],
    };

    let neutral = RoleColours {
        background: StateSet {
            base: g[1],
            hover: g[2],
            active: g[3],
            selected: m[2],
            focus: g[1],
            disabled: g[1],
        },
        foreground: StateSet {
            base: g[11],
            hover: g[11],
            active: g[11],
            selected: g[11],
            focus: g[11],
            disabled: disabled_ink,
        },
        border: StateSet {
            base: borders.default_,
            hover: borders.strong,
            active: borders.control,
            selected: m[5],
            focus: borders.focus,
            disabled: g[4],
        },
    };

    // Dark solids brighten on hover and darken on press — the inverse of
    // light — because the scales' steps 11–12 are text steps and unavailable
    // as fills.
    let accent = solid_role(
        m[9],
        m[7],
        m[6],
        m[5],
        on_solid,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );
    let danger = solid_role(
        r[8],
        r[9],
        r[6],
        r[5],
        on_solid,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );
    let success = solid_role(
        n[9],
        n[7],
        n[6],
        n[5],
        on_solid,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );
    let warning = solid_role(
        a[8],
        a[10],
        a[9],
        a[5],
        on_bright,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );
    let info = solid_role(
        b[8],
        b[9],
        b[6],
        b[5],
        on_solid,
        disabled_fill,
        disabled_ink,
        borders.focus,
    );

    Semantic {
        surfaces,
        borders,
        text,
        roles: [neutral, accent, danger, success, warning, info],
        rows: Rows {
            background: surfaces.raised,
            header_background: g[1],
            header_foreground: g[10],
            footer_background: g[1],
            footer_foreground: g[10],
            stripe_background: g[0],
            hover_background: g[2],
            selected_background: m[2],
            selected_border: m[5],
            row_border: borders.divider,
        },
        tabs: Tabs {
            bar_background: g[1],
            segmented_background: g[2],
            background: g[1],
            foreground: g[10],
            active_background: surfaces.raised,
            active_foreground: g[11],
        },
        scrollbar: Scrollbar {
            track: g[1],
            thumb: g[5],
            thumb_hover: g[6],
        },
        editor: Editor {
            caret: g[11],
            // Accent step 3, one step below the light mode's wash: the dark
            // accent scale climbs faster, and step 4 pushes quiet ink under
            // the 3:1 floor for selected text.
            selection: m[2],
        },
        drag_drop: DragDrop {
            // The Maritime anchor at 15% — the accent as a verb, washed.
            // Read from the anchor so it cannot drift off the brand colour.
            drop_target: crate::MARITIME.with_alpha_u8(0x26),
            drag_border: m[7],
        },
        feedback: Feedback {
            skeleton: g[2],
            progress_track: g[4],
            progress_bar: m[9],
            slider_track: g[4],
            slider_thumb: m[9],
        },
        containers: Containers {
            sidebar_background: g[0],
            sidebar_border: borders.subtle,
            title_bar_background: surfaces.app,
            status_bar_background: surfaces.app,
            tiles_background: surfaces.app,
            group_box_background: g[1],
            group_box_title_foreground: g[10],
            description_label_background: g[1],
            description_label_foreground: g[10],
            accordion_background: surfaces.raised,
            accordion_hover_background: g[2],
            popover_background: surfaces.overlay,
            window_border: borders.subtle,
        },
    }
};

/// Build a solid role from four fills plus the shared disabled/ink/ring
/// values. Solid roles are all shaped identically; writing them out six times
/// twice is how they drift.
#[allow(clippy::too_many_arguments)]
const fn solid_role(
    base: Rgba,
    hover: Rgba,
    active: Rgba,
    selected_border: Rgba,
    ink: Rgba,
    disabled_fill: Rgba,
    disabled_ink: Rgba,
    ring: Rgba,
) -> RoleColours {
    RoleColours {
        background: StateSet {
            base,
            hover,
            active,
            // A selected solid reads as held down.
            selected: active,
            // The ring carries focus; the fill does not move.
            focus: base,
            disabled: disabled_fill,
        },
        foreground: StateSet {
            base: ink,
            hover: ink,
            active: ink,
            selected: ink,
            focus: ink,
            disabled: disabled_ink,
        },
        border: StateSet {
            base,
            hover,
            active,
            selected: selected_border,
            focus: ring,
            disabled: disabled_fill,
        },
    }
}

/// The status colours, restated here so a consumer reading the semantic layer
/// does not have to know they live with the chart palettes. Identical values —
/// `viz::STATUS` remains the definition (`guidelines/colour.md`: status is
/// reserved and fixed across modes).
pub const STATUS_GOOD: Rgba = STATUS.good;
pub const STATUS_WARNING: Rgba = STATUS.warning;
pub const STATUS_SERIOUS: Rgba = STATUS.serious;
pub const STATUS_CRITICAL: Rgba = STATUS.critical;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_indices_round_trip() {
        for (i, r) in Role::ALL.iter().enumerate() {
            assert_eq!(r.index(), i);
            assert_eq!(LIGHT.role(*r), &LIGHT.roles[i]);
            assert_eq!(DARK.role(*r), &DARK.roles[i]);
        }
    }

    #[test]
    fn only_the_neutral_role_wears_chrome() {
        assert!(!Role::Neutral.is_solid());
        for r in Role::ALL {
            if !matches!(r, Role::Neutral) {
                assert!(r.is_solid(), "{} should be solid", r.name());
            }
        }
    }

    #[test]
    fn every_state_slot_is_instantiated_and_states_differ() {
        for s in [&LIGHT, &DARK] {
            for r in Role::ALL {
                let rc = s.role(r);
                // A StateSet whose states are all one colour is a StateSet
                // that was never filled in.
                let bg = rc.background.all();
                assert!(
                    bg.iter().any(|c| *c != bg[0]),
                    "{} background never changes state",
                    r.name()
                );
                assert_ne!(rc.background.base, rc.background.hover);
                assert_ne!(rc.background.hover, rc.background.active);
                assert_ne!(rc.border.focus, rc.border.base);
            }
        }
    }

    #[test]
    fn dark_mirrors_light_slot_for_slot() {
        // Same vocabulary in both modes: no slot exists in one and not the
        // other (the type system guarantees the shape; this guards values
        // being left equal across modes, which means one was forgotten).
        assert_ne!(LIGHT.surfaces.app, DARK.surfaces.app);
        assert_ne!(LIGHT.text.primary, DARK.text.primary);
        assert_ne!(LIGHT.borders.subtle, DARK.borders.subtle);
        assert_ne!(LIGHT.rows.hover_background, DARK.rows.hover_background);
    }

    #[test]
    fn light_is_lighter_than_dark_on_every_plane() {
        for (l, d) in LIGHT.planes().iter().zip(DARK.planes().iter()) {
            assert!(crate::validate::okl(*l) > crate::validate::okl(*d));
        }
    }
}
