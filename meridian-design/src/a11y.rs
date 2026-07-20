//! Accessibility metadata as plain `&'static` data: what a control *is*, and
//! what the keyboard is expected to do to it.
//!
//! This is the half of a component contract that colour tokens cannot carry.
//! A design system that ships a "primary button" and says nothing about its
//! role or its key handling ships half a component, and every consumer
//! re-invents the other half differently.
//!
//! **No dependency, by construction** (the crate's `[dependencies]` is empty
//! and stays empty, ADR 0003). The shape here is deliberately enums plus
//! `&'static str` tables returned from `const fn`s: no derive, no map type, no
//! serialisation. Anything richer would have wanted a serde derive, and the
//! answer to that is to keep the data out of this crate, not to take the
//! dependency.
//!
//! The role names are the ARIA vocabulary because it is the only cross-
//! platform role vocabulary that exists, and because the web half of Meridian
//! consumes it directly. A native accessibility layer maps them at its own
//! boundary — the same posture the colour types take (ADR 0003).

/// How a role receives keyboard focus.
///
/// A single `focusable: bool` cannot describe a composite widget and gets the
/// answer wrong in both directions. A grid row is never a Tab stop, yet it
/// holds focus while the user arrows through the grid and must answer Return;
/// a tab list is never a Tab stop either, but its *selected tab* is the stop
/// and the list is what interprets the arrow keys. Collapsing those onto one
/// boolean is what produced "cannot take focus, must answer Activate", which
/// is not implementable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    /// In the normal tab order: Tab reaches it from outside.
    ///
    /// A composite that manages roving focus is a single stop — Tab enters it
    /// once and the arrow keys move within. A transient container (a menu, a
    /// modal) is a stop *while it is open*: opening it moves focus in, and
    /// the tab order is scoped to it until it closes.
    Tab,
    /// Focusable only inside the composite that owns it — roving tabindex or
    /// `aria-activedescendant`. It holds focus and answers keys, but Tab
    /// never lands on it directly; its owner is the tab stop.
    Roving,
    /// Never focused. Live regions (announced, not visited) and pure
    /// structure. A role here must answer no key intent — see
    /// `a_role_answers_keys_only_if_it_can_hold_focus`.
    Never,
}

/// Declares an enum together with its complete roster and its stable string
/// name, from **one** list.
///
/// The roster is the point. A hand-written `const ALL: [Role; 25]` looks like
/// a gate and is not one: adding a variant breaks the exhaustive `match`es
/// (which is what actually forces the author into this file), but nothing
/// makes them extend `ALL`, so the new variant silently escapes every
/// `for r in ALL` test — verified, it stays green. Generating the enum, the
/// roster and the name table from the same token list closes that: the
/// variant cannot exist without appearing in all three.
///
/// `ALL` stays a fixed-size array, sized from the list rather than typed by
/// hand, so a consumer can still build a binding table with no allocation.
macro_rules! roster {
    (
        $(#[$emeta:meta])*
        pub enum $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident = $text:literal ),* $(,)?
        }
        $(#[$fnmeta:meta])*
        pub fn $namefn:ident;
    ) => {
        $(#[$emeta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $( $(#[$vmeta])* $variant, )*
        }

        impl $name {
            /// Every variant, in declaration order — generated from the same
            /// list as the enum, so it cannot fall out of sync with it.
            pub const ALL: [$name; [$( stringify!($variant) ),*].len()] =
                [$( $name::$variant ),*];

            $(#[$fnmeta])*
            pub const fn $namefn(self) -> &'static str {
                match self {
                    $( $name::$variant => $text, )*
                }
            }
        }
    };
}

roster! {
    /// What a control is, for assistive technology. The variant and its ARIA
    /// name are declared together so neither can exist without the other.
    pub enum WidgetRole {
        Button = "button",
        Link = "link",
        Checkbox = "checkbox",
        Radio = "radio",
        Switch = "switch",
        Tab = "tab",
        TabList = "tablist",
        MenuItem = "menuitem",
        Menu = "menu",
        ComboBox = "combobox",
        ListBox = "listbox",
        Option_ = "option",
        TextBox = "textbox",
        Slider = "slider",
        ProgressBar = "progressbar",
        Grid = "grid",
        GridCell = "gridcell",
        ColumnHeader = "columnheader",
        Row = "row",
        Dialog = "dialog",
        Tooltip = "tooltip",
        Status = "status",
        Alert = "alert",
        Separator = "separator",
        Toolbar = "toolbar",
    }
    /// The ARIA role name.
    pub fn aria;
}

impl WidgetRole {
    /// How this role receives keyboard focus.
    ///
    /// [`Focus::Roving`] is the answer for the members of a composite — a
    /// grid cell, a grid row, a column header, a tab, a menu item, a listbox
    /// option. They hold focus and answer keys; they are simply not Tab
    /// stops, because their owner is. [`Focus::Never`] is reserved for live
    /// regions and structure, and those answer no keys at all.
    pub const fn focus(self) -> Focus {
        match self {
            // Composite members: focus arrives by arrow key, not by Tab.
            Self::Tab
            | Self::MenuItem
            | Self::Option_
            | Self::GridCell
            | Self::Row
            | Self::ColumnHeader => Focus::Roving,
            // Announced or structural — never focused, and never keyed.
            Self::ProgressBar | Self::Tooltip | Self::Status | Self::Alert | Self::Separator => {
                Focus::Never
            }
            // Everything else is a Tab stop, including the composites
            // (`TabList`, `Menu`, `ListBox`, `Toolbar`, `Grid`) — one stop
            // each, entered by Tab and traversed by arrow.
            _ => Focus::Tab,
        }
    }

    /// Whether Tab reaches this role from outside. Composite members answer
    /// `false` even though they hold focus — use [`Self::takes_focus`] for
    /// "can this ever be the focused element".
    pub const fn tab_stop(self) -> bool {
        matches!(self.focus(), Focus::Tab)
    }

    /// Whether this role can ever hold keyboard focus, whether by Tab or by
    /// its owner's roving cursor. **This is the one that pairs with
    /// [`Self::intents`]**: a role that answers a key must be able to be the
    /// focused element when that key arrives, and a role that can be focused
    /// must answer at least one key. Both directions are gated in the tests.
    pub const fn takes_focus(self) -> bool {
        !matches!(self.focus(), Focus::Never)
    }

    /// The keyboard intents a control of this role must answer, *while it is
    /// the focused element*. A consumer that implements fewer has shipped a
    /// mouse-only control.
    ///
    /// For a composite, the intents are the ones the composite itself
    /// interprets while focus is inside it — a `ListBox` answers the arrows
    /// even though the focused descendant is an `Option_`.
    pub const fn intents(self) -> &'static [KeyIntent] {
        use KeyIntent::*;
        match self {
            Self::Button | Self::MenuItem | Self::Option_ => &[Activate],
            Self::Link => &[Activate],
            Self::Checkbox | Self::Switch => &[Toggle],
            Self::Radio | Self::Tab => &[Activate, MovePrev, MoveNext],
            Self::TabList | Self::Toolbar => &[MovePrev, MoveNext, MoveFirst, MoveLast],
            Self::Menu | Self::ListBox => {
                &[MovePrev, MoveNext, MoveFirst, MoveLast, Activate, Dismiss]
            }
            Self::ComboBox => &[Expand, Activate, MovePrev, MoveNext, Dismiss],
            Self::TextBox => &[Commit, Dismiss],
            Self::Slider => &[Decrement, Increment, MoveFirst, MoveLast],
            Self::Grid | Self::GridCell => &[
                MovePrev, MoveNext, MoveUp, MoveDown, MoveFirst, MoveLast, PageUp, PageDown,
                Activate, Extend,
            ],
            Self::Dialog => &[Dismiss, Commit],
            Self::Row => &[Activate, Extend],
            Self::ColumnHeader => &[Activate],
            Self::ProgressBar | Self::Tooltip | Self::Status | Self::Alert | Self::Separator => &[],
        }
    }
}

roster! {
    /// What the user means, independent of which key they pressed. Bindings
    /// are a platform concern (Return vs Space, Home vs ⌘↑); the *intent* is
    /// the part a design system can fix.
    pub enum KeyIntent {
        /// Do the thing.
        Activate = "activate",
        /// Flip a binary state.
        Toggle = "toggle",
        /// Open a popup owned by this control.
        Expand = "expand",
        /// Close without committing — always available, always non-destructive.
        Dismiss = "dismiss",
        /// Commit an edit.
        Commit = "commit",
        MovePrev = "move-prev",
        MoveNext = "move-next",
        MoveUp = "move-up",
        MoveDown = "move-down",
        MoveFirst = "move-first",
        MoveLast = "move-last",
        PageUp = "page-up",
        PageDown = "page-down",
        /// Grow a selection from the anchor.
        Extend = "extend",
        Decrement = "decrement",
        Increment = "increment",
    }
    /// Stable machine name — for a keymap file, a shortcut sheet, or a test.
    pub fn name;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aria_names_are_unique_and_lowercase() {
        let names: Vec<&str> = WidgetRole::ALL.iter().map(|r| r.aria()).collect();
        for (i, a) in names.iter().enumerate() {
            assert!(!a.is_empty() && a.chars().all(|c| c.is_ascii_lowercase()));
            for b in &names[i + 1..] {
                assert_ne!(a, b, "duplicate ARIA role name");
            }
        }
    }

    /// Forward direction: focus without keys is a mouse-only control.
    #[test]
    fn every_focusable_role_answers_at_least_one_intent() {
        for r in WidgetRole::ALL {
            if r.takes_focus() {
                assert!(
                    !r.intents().is_empty(),
                    "{} takes focus but answers no key",
                    r.aria()
                );
            }
        }
    }

    /// Reverse direction — the one the forward test cannot see, and the one
    /// that was wrong: a role that must answer a key has to be able to be the
    /// focused element when that key arrives. "Cannot take focus, must answer
    /// Activate" is not implementable by anyone.
    #[test]
    fn a_role_answers_keys_only_if_it_can_hold_focus() {
        for r in WidgetRole::ALL {
            if !r.intents().is_empty() {
                assert!(
                    r.takes_focus(),
                    "{} answers {:?} but can never hold focus",
                    r.aria(),
                    r.intents()
                );
            }
            if r.focus() == Focus::Never {
                assert!(
                    r.intents().is_empty(),
                    "{} is never focused yet claims key intents",
                    r.aria()
                );
            }
        }
    }

    /// A composite member is not a Tab stop, but it is still focusable — the
    /// distinction the single boolean could not carry.
    #[test]
    fn composite_members_hold_focus_without_being_tab_stops() {
        for r in [
            WidgetRole::Row,
            WidgetRole::ColumnHeader,
            WidgetRole::GridCell,
            WidgetRole::Tab,
            WidgetRole::MenuItem,
            WidgetRole::Option_,
        ] {
            assert_eq!(r.focus(), Focus::Roving, "{} moved off roving", r.aria());
            assert!(!r.tab_stop());
            assert!(r.takes_focus());
        }
        // …and the composites that own them are the stops.
        for r in [
            WidgetRole::Grid,
            WidgetRole::TabList,
            WidgetRole::Menu,
            WidgetRole::ListBox,
            WidgetRole::Toolbar,
        ] {
            assert!(r.tab_stop(), "{} must be the tab stop", r.aria());
        }
    }

    #[test]
    fn anything_that_can_be_opened_can_be_dismissed() {
        for r in WidgetRole::ALL {
            let intents = r.intents();
            if intents.contains(&KeyIntent::Expand) {
                assert!(
                    intents.contains(&KeyIntent::Dismiss),
                    "{} opens but never closes",
                    r.aria()
                );
            }
        }
    }

    #[test]
    fn intent_names_are_unique() {
        let all = KeyIntent::ALL;
        for (i, a) in all.iter().enumerate() {
            assert!(
                !a.name().is_empty()
                    && a.name().chars().all(|c| c.is_ascii_lowercase() || c == '-'),
                "{:?} has an unusable machine name",
                a
            );
            for b in &all[i + 1..] {
                assert_ne!(a.name(), b.name());
            }
        }
    }

    /// Every intent must be reachable: an intent no role answers is either a
    /// missing role binding or a variant that should not exist.
    #[test]
    fn every_intent_is_answered_by_some_role() {
        for i in KeyIntent::ALL {
            assert!(
                WidgetRole::ALL.iter().any(|r| r.intents().contains(&i)),
                "no role answers {}",
                i.name()
            );
        }
    }
}
