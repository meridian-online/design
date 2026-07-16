//! Conformance: emitted artefacts are pinned exactly. If an intentional token
//! change lands, regenerate the snapshot in the same commit — a diff here in
//! CI means a consumer would have drifted.

#[test]
fn tokens_css_matches_snapshot() {
    let expected = include_str!("snapshots/tokens.css");
    assert_eq!(
        meridian_design::emit::tokens_css(),
        expected,
        "emit::tokens_css() no longer matches tests/snapshots/tokens.css — \
         regenerate the snapshot if the change is intentional"
    );
}
