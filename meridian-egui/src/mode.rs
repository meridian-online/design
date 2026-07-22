//! Light or dark chrome.

/// Which chrome a frame is drawn in.
///
/// This is a *theme selector*, not application state: it exists so a single
/// boolean — dark or not — reaches every Meridian token accessor
/// (`semantic(dark)`, `Elevation::shadow(dark)`) from one place. It lives in the
/// egui adapter rather than in a consuming shell because the adapter is what
/// resolves tokens through it; a shell holds *its* mode as a copy of this enum.
///
/// [`Mode::Light`] is the default: the Vello chart canvas is light-first this
/// phase, and dark chrome around a white chart reads as broken until dark chart
/// ink lands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Light chrome (the current default).
    #[default]
    Light,
    /// Dark chrome (tokens ready; chart ink follows in a later phase).
    Dark,
}

impl Mode {
    /// Whether this is the dark mode — the argument every design-token accessor
    /// takes (`semantic(dark)`, `Elevation::shadow(dark)`).
    #[must_use]
    pub const fn is_dark(self) -> bool {
        matches!(self, Mode::Dark)
    }

    /// The other mode.
    #[must_use]
    pub const fn toggled(self) -> Self {
        match self {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        }
    }
}
