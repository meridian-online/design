//! Bundled font bytes (ADR 0005 resolution): Inter 4.1 (upstream rsms static
//! builds) and JetBrains Mono 2.304 — both SIL OFL 1.1, licence files ship
//! alongside in `fonts/`. One source, two sinks: the egui app registers these
//! through the `meridian-egui` adapter (`install_fonts`), the render crate
//! parses them directly. Never substitute a stripped webfont build.

/// The UI sans family name these bytes register as.
pub const SANS: &str = "Inter";
/// The mono family name these bytes register as.
pub const MONO: &str = "JetBrains Mono";

pub const INTER_REGULAR: &[u8] = include_bytes!("../fonts/Inter-Regular.ttf");
pub const INTER_MEDIUM: &[u8] = include_bytes!("../fonts/Inter-Medium.ttf");
pub const INTER_SEMIBOLD: &[u8] = include_bytes!("../fonts/Inter-SemiBold.ttf");
pub const INTER_BOLD: &[u8] = include_bytes!("../fonts/Inter-Bold.ttf");
pub const JBM_REGULAR: &[u8] = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");
pub const JBM_MEDIUM: &[u8] = include_bytes!("../fonts/JetBrainsMono-Medium.ttf");
pub const JBM_BOLD: &[u8] = include_bytes!("../fonts/JetBrainsMono-Bold.ttf");
pub const JBM_ITALIC: &[u8] = include_bytes!("../fonts/JetBrainsMono-Italic.ttf");

/// Every bundled face, for bulk registration.
pub const ALL: [&[u8]; 8] = [
    INTER_REGULAR,
    INTER_MEDIUM,
    INTER_SEMIBOLD,
    INTER_BOLD,
    JBM_REGULAR,
    JBM_MEDIUM,
    JBM_BOLD,
    JBM_ITALIC,
];
