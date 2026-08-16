//! Regenerate the reference sheet:
//! `cargo run --example dump_md > ../reference/tokens.md`

fn main() {
    print!("{}", meridian_design::emit::tokens_md());
}
