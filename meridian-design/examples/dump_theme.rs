//! Regenerate the ThemeConfig snapshots:
//! `cargo run --example dump_theme light > tests/snapshots/theme-light.json`
//! `cargo run --example dump_theme dark  > tests/snapshots/theme-dark.json`

use meridian_design::emit::{theme_config, ThemeMode};

fn main() {
    let mode = match std::env::args().nth(1).as_deref() {
        Some("dark") => ThemeMode::Dark,
        _ => ThemeMode::Light,
    };
    print!("{}", theme_config(mode));
}
