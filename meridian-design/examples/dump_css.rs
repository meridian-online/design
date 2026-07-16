//! Regenerate the conformance snapshot:
//! `cargo run --example dump_css > tests/snapshots/tokens.css`

fn main() {
    print!("{}", meridian_design::emit::tokens_css());
}
