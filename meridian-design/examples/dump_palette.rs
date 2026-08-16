//! Regenerate the palette sheets:
//! `cargo run --example dump_palette light > ../reference/palette.svg`
//! `cargo run --example dump_palette dark  > ../reference/palette_dark.svg`

fn main() {
    let dark = std::env::args().nth(1).as_deref() == Some("dark");
    print!("{}", meridian_design::emit::palette_svg(dark));
}
