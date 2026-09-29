//! The chrome gate — the semantic layer's equivalent of the palette gate.
//!
//! `tests/palette_gate.rs` proves the *data* colours are legible. This proves
//! the *chrome* colours are: text on every surface, ink on every solid, the
//! boundary that identifies a control, and the focus ring against everything
//! it can ever be drawn over.
//!
//! Method: WCAG relative-luminance contrast, from the crate's own
//! `validate::contrast`. No second contrast model — ADRs 0006/0007 settle the
//! method for this system, and mixing a second one would mean two answers to
//! the same question.

use meridian_design::chrome::{INK_DARK, INK_LIGHT};
use meridian_design::scales::{GRAY_DARK, GRAY_LIGHT, MARITIME_DARK, MARITIME_LIGHT};
use meridian_design::semantic::{semantic, Role, Semantic};
use meridian_design::validate::contrast;
use meridian_design::Rgba;

/// The two floors now live in `validate.rs`, because the reference sheet
/// publishes them beside the ratios it measures and a threshold stated twice
/// is a threshold that can disagree with itself.
use meridian_design::validate::{NON_TEXT_MIN as NON_TEXT, TEXT_MIN as TEXT};

fn modes() -> [(&'static str, &'static Semantic); 2] {
    [("light", semantic(false)), ("dark", semantic(true))]
}

/// Every background chrome can sit on: the planes plus the row states,
/// because a hovered or selected row is where contrast quietly dies.
fn planes(s: &Semantic) -> Vec<(&'static str, Rgba)> {
    vec![
        ("app", s.surfaces.app),
        ("raised", s.surfaces.raised),
        ("sunken", s.surfaces.sunken),
        ("overlay", s.surfaces.overlay),
        ("sidebar", s.surfaces.sidebar),
        ("header", s.surfaces.header),
    ]
}

/// Every background chrome can sit on: the planes plus the row states,
/// because a hovered or selected row is where contrast quietly dies.
fn chrome_backgrounds(s: &Semantic) -> Vec<(&'static str, Rgba)> {
    let mut v = planes(s);
    v.extend([
        ("row-hover", s.rows.hover_background),
        ("row-selected", s.rows.selected_background),
        ("row-stripe", s.rows.stripe_background),
    ]);
    v
}

/// Every fill the focus ink is drawn over as a ring or a bar. The ring is a
/// rule inside the control's edge (`focus::RING_OFFSET` is 0), so it lands on
/// the control's own fill — a field's sunken fill, a row under the pointer or
/// the cursor, a default control in each of its states — and the bars land on
/// the row under the cursor and on the tab strip. A solid fill is not here: the
/// ring there takes the on-solid ink, which the test after this one measures.
fn ring_fills(s: &Semantic) -> Vec<(&'static str, Rgba)> {
    let neutral = s.role(Role::Neutral).background;
    let mut v = chrome_backgrounds(s);
    v.extend([
        ("row-cursor", s.rows.cursor_background),
        ("neutral", neutral.base),
        ("neutral-hover", neutral.hover),
        ("neutral-active", neutral.active),
        ("tabs-bar", s.tabs.bar_background),
        ("tabs", s.tabs.background),
        ("tabs-active", s.tabs.active_background),
    ]);
    v
}

/// The chrome backgrounds plus the text-selection wash and the row under the
/// cursor. Real text can be selected; a control border or a placeholder
/// cannot, which is why the two lists differ.
///
/// The row under the cursor carries text and the focus ink's bar, and both are
/// gated on it (here and in [`ring_fills`]). It is not in
/// [`chrome_backgrounds`], so `borders.control` is not held to 3:1 on it: on
/// the light cursor fill that border measures under the floor, and no control
/// with a drawn boundary sits in a row under the cursor today.
fn text_backgrounds(s: &Semantic) -> Vec<(&'static str, Rgba)> {
    let mut v = chrome_backgrounds(s);
    v.push(("editor-selection", s.editor.selection));
    v.push(("row-cursor", s.rows.cursor_background));
    v
}

#[test]
fn load_bearing_text_clears_aa_on_every_surface() {
    for (mode, s) in modes() {
        for (slot, ink) in [
            ("primary", s.text.primary),
            ("secondary", s.text.secondary),
            ("link", s.text.link),
            ("link-hover", s.text.link_hover),
            ("link-active", s.text.link_active),
        ] {
            for (name, bg) in text_backgrounds(s) {
                let c = contrast(ink, bg);
                assert!(
                    c >= TEXT,
                    "{mode}: text.{slot} on {name} is {c:.2}:1, below {TEXT}"
                );
            }
        }
    }
}

#[test]
fn quiet_ink_clears_the_non_text_floor_and_is_documented_as_quiet() {
    // muted/placeholder are deliberately below the text floor — see the
    // `Text` doc comment. They must still clear 3:1 everywhere, and they must
    // stay in rank order so "quieter" always means quieter.
    for (mode, s) in modes() {
        for (slot, ink, bgs) in [
            ("muted", s.text.muted, text_backgrounds(s)),
            // Placeholder text lives inside an empty control, so its
            // background is that control's plane — never a row wash and
            // never the selection wash (a placeholder cannot be selected).
            ("placeholder", s.text.placeholder, planes(s)),
        ] {
            for (name, bg) in bgs {
                let c = contrast(ink, bg);
                assert!(
                    c >= NON_TEXT,
                    "{mode}: text.{slot} on {name} is {c:.2}:1, below {NON_TEXT}"
                );
            }
        }
        let plane = s.surfaces.raised;
        let rank = [
            contrast(s.text.primary, plane),
            contrast(s.text.secondary, plane),
            contrast(s.text.muted, plane),
            contrast(s.text.placeholder, plane),
            contrast(s.text.disabled, plane),
        ];
        for w in rank.windows(2) {
            assert!(w[0] > w[1], "{mode}: ink slots are out of rank order");
        }
    }
}

#[test]
fn ink_on_every_solid_clears_aa_in_every_visible_state() {
    for (mode, s) in modes() {
        for role in Role::ALL {
            let rc = s.role(role);
            // `disabled` is exempt (WCAG exempts inactive controls) and
            // `focus` repeats `base` by construction; the rest must hold.
            for (state, bg, fg) in [
                ("base", rc.background.base, rc.foreground.base),
                ("hover", rc.background.hover, rc.foreground.hover),
                ("active", rc.background.active, rc.foreground.active),
                ("selected", rc.background.selected, rc.foreground.selected),
            ] {
                let c = contrast(fg, bg);
                assert!(
                    c >= TEXT,
                    "{mode}: {}.{state} ink is {c:.2}:1, below {TEXT}",
                    role.name()
                );
            }
        }
    }
}

#[test]
fn the_control_boundary_is_findable_everywhere() {
    // WCAG 1.4.11: a boundary that identifies a control needs 3:1. This is
    // exactly why `borders.control` sits above the 6–8 band.
    for (mode, s) in modes() {
        for (name, bg) in chrome_backgrounds(s) {
            let c = contrast(s.borders.control, bg);
            assert!(
                c >= NON_TEXT,
                "{mode}: borders.control on {name} is {c:.2}:1, below {NON_TEXT}"
            );
        }
    }
}

#[test]
fn the_focus_ring_is_visible_against_everything_it_can_land_on() {
    for (mode, s) in modes() {
        // The ring and the two bars share one ink, and the measurements below
        // are taken on that ink, so a bar token that left it would go
        // unmeasured. Hold the bars to it first.
        let ring = s.borders.focus;
        assert_eq!(
            s.rows.cursor_bar, ring,
            "{mode}: the row's bar left the focus ink"
        );
        assert_eq!(
            s.tabs.active_bar, ring,
            "{mode}: the tab's bar left the focus ink"
        );

        for (name, bg) in ring_fills(s) {
            let c = contrast(ring, bg);
            assert!(
                c >= NON_TEXT,
                "{mode}: the focus ink on {name} is {c:.2}:1, below {NON_TEXT}"
            );
        }
    }
}

/// On the accent's solid fill the ring takes the on-solid ink, one pixel in
/// from the edge (`focus::RING_WIDTH_ON_SOLID`, `focus::RING_INSET`). This
/// holds that ink to 3:1 on the fill in every state a focused control can be
/// in, and holds the reason it exists: the focus ink on that fill measures
/// under 3:1, so a ring in the focus ink there would not be seen.
#[test]
fn the_ring_on_the_accent_takes_the_on_solid_ink_and_is_seen() {
    for (mode, s) in modes() {
        let accent = s.role(Role::Accent).background;
        for (state, fill) in [
            ("base", accent.base),
            ("hover", accent.hover),
            ("active", accent.active),
            ("focus", accent.focus),
        ] {
            let c = contrast(s.text.on_solid, fill);
            assert!(
                c >= NON_TEXT,
                "{mode}: the on-solid ring on accent.{state} is {c:.2}:1, below {NON_TEXT}"
            );
        }
        let lost = contrast(s.borders.focus, accent.base);
        assert!(
            lost < NON_TEXT,
            "{mode}: the focus ink on the accent is {lost:.2}:1 — a ring in the \
             focus ink would now be seen there, and the on-solid ring's reason is gone"
        );
    }
}

#[test]
fn border_steps_are_the_documented_band() {
    // Ground truth: ADR 0007 and `scales.rs` both document steps 6–8 as the
    // border band. Canonised here as subtle=6, default=7, strong=8, with two
    // documented departures: divider below (step 3) and control above (9).
    for (mode, s, g) in [
        ("light", semantic(false), GRAY_LIGHT),
        ("dark", semantic(true), GRAY_DARK),
    ] {
        assert_eq!(s.borders.subtle, g[5], "{mode}: subtle must be step 6");
        assert_eq!(s.borders.default_, g[6], "{mode}: default must be step 7");
        assert_eq!(s.borders.strong, g[7], "{mode}: strong must be step 8");
        // The one slot whose step differs by mode, and the reason is measured
        // rather than aesthetic: 3:1 over the selection wash (see below).
        let control_step = if mode == "light" { g[8] } else { g[9] };
        assert_eq!(
            s.borders.control, control_step,
            "{mode}: control step moved"
        );
        assert_eq!(s.borders.divider, g[2], "{mode}: divider must be step 3");

        // …and the band must actually read as a progression on the surface.
        let plane = s.surfaces.raised;
        let ladder = [
            contrast(s.borders.divider, plane),
            contrast(s.borders.subtle, plane),
            contrast(s.borders.default_, plane),
            contrast(s.borders.strong, plane),
            contrast(s.borders.control, plane),
        ];
        for w in ladder.windows(2) {
            assert!(w[1] > w[0], "{mode}: border steps are not a progression");
        }
    }
}

#[test]
fn the_focus_ring_is_maritime_and_nothing_else_is() {
    // `guidelines/identity.md`: Maritime is reserved for interaction. The ring
    // is the purest case, so it must come from the accent scale; chrome
    // surfaces and borders must not.
    for (s, accent, gray) in [
        (semantic(false), MARITIME_LIGHT, GRAY_LIGHT),
        (semantic(true), MARITIME_DARK, GRAY_DARK),
    ] {
        assert!(
            accent.contains(&s.borders.focus),
            "the focus ring must be an accent-scale step"
        );
        for chrome in [
            s.surfaces.sunken,
            s.surfaces.sidebar,
            s.surfaces.header,
            s.borders.subtle,
            s.borders.default_,
            s.borders.strong,
            s.borders.control,
            s.borders.divider,
        ] {
            assert!(
                gray.contains(&chrome),
                "static chrome must come from the warm neutral scale"
            );
        }
    }
}

/// The anchor gate. `chrome.rs` holds the values approved for the rendered
/// chart; this layer must *read* them, never restate them. Every equality
/// below is a claim made in a doc comment somewhere in `semantic.rs`, made
/// checkable here — a doc comment that promises "named once" and is not
/// gated is worth less than no comment at all.
#[test]
fn the_semantic_layer_reads_the_chrome_anchors_rather_than_restating_them() {
    for (mode, s, ink) in [
        ("light", semantic(false), &INK_LIGHT),
        ("dark", semantic(true), &INK_DARK),
    ] {
        // A UI divider and a chart gridline are one value, not two that
        // happen to agree. (The step-3 half of the claim is asserted in
        // `border_steps_are_the_documented_band`.)
        assert_eq!(
            s.borders.divider, ink.gridline,
            "{mode}: divider and gridline have drifted apart"
        );
        // Planes.
        assert_eq!(s.surfaces.app, ink.page, "{mode}: app plane");
        assert_eq!(s.surfaces.raised, ink.surface, "{mode}: raised plane");
        // Ink slots that exist in both layers.
        assert_eq!(s.text.primary, ink.ink_primary, "{mode}: primary ink");
        assert_eq!(s.text.secondary, ink.ink_secondary, "{mode}: secondary ink");
        assert_eq!(
            s.text.placeholder, ink.ink_muted,
            "{mode}: chrome's quietest ink and the placeholder slot are the \
             same step and must be the same value"
        );
    }
}

/// The one ink slot with no chrome counterpart, gated as a *relationship*
/// rather than left to look like an accident. `semantic.text.muted` is one
/// step above `chrome::INK_*.ink_muted` in both modes — which in raw hex
/// looks like the two layers diverging in opposite directions, and is in fact
/// the same rule applied to two scales that climb in opposite directions.
#[test]
fn semantic_muted_ink_is_one_step_quieter_than_chrome_allows_itself() {
    for (mode, s, ink, g) in [
        ("light", semantic(false), &INK_LIGHT, GRAY_LIGHT),
        ("dark", semantic(true), &INK_DARK, GRAY_DARK),
    ] {
        // Chrome's muted ink is step 9; the UI's muted ink is step 10.
        assert_eq!(ink.ink_muted, g[8], "{mode}: chrome muted must be step 9");
        assert_eq!(s.text.muted, g[9], "{mode}: semantic muted must be step 10");
        // And the point of the extra step: more contrast, in both modes, on
        // the surface both layers share.
        let plane = s.surfaces.raised;
        let ui = contrast(s.text.muted, plane);
        let chart = contrast(ink.ink_muted, plane);
        assert!(
            ui > chart,
            "{mode}: UI muted ink ({ui:.2}:1) must be louder than chart muted \
             ink ({chart:.2}:1) — it sits on row washes, the chart ink does not"
        );
    }
}

/// Washes are anchors at low alpha, not new colours. This is the gate for
/// the raw-hex class of drift: every translucent value in the system must
/// still be recognisably one of the anchors.
#[test]
fn every_wash_is_an_anchor_at_low_alpha() {
    use meridian_design::elevation::Elevation;

    let light = semantic(false);
    assert!(
        light.surfaces.scrim.same_paint(&INK_LIGHT.ink_primary),
        "the light scrim must be the page's own ink"
    );
    // Dark is the documented exception: warm grey over a dark plane lightens
    // it, so black is the only paint that dims.
    let dark = semantic(true);
    let black = Rgba::from_u8(0x00, 0x00, 0x00, 0xff);
    assert!(
        dark.surfaces.scrim.same_paint(&black),
        "the dark scrim must be true black"
    );
    for s in [&light, &dark] {
        assert!(
            s.drag_drop
                .drop_target
                .same_paint(&meridian_design::MARITIME),
            "the drop wash must be the Maritime anchor"
        );
    }
    for e in [Elevation::Overlay, Elevation::Modal] {
        assert!(e
            .shadow(false)
            .unwrap()
            .colour
            .same_paint(&INK_LIGHT.ink_primary));
        assert!(e.shadow(true).unwrap().colour.same_paint(&black));
    }
}

/// The light-mode overlay asymmetry, asserted so it reads as a decision.
/// See `Surfaces::overlay`.
#[test]
fn the_light_overlay_is_separated_by_edge_and_the_dark_one_by_surface() {
    let light = semantic(false);
    assert_eq!(
        light.surfaces.overlay, light.surfaces.raised,
        "light overlay is deliberately flush with the panel plane — if this \
         changes, the doc at `Surfaces::overlay` must change with it"
    );
    // …which is only safe because the overlay carries a shadow and a
    // hairline, and both must exist.
    assert!(meridian_design::elevation::Elevation::Overlay
        .shadow(false)
        .is_some());
    assert!(meridian_design::elevation::Elevation::Overlay.hairline());

    let dark = semantic(true);
    assert_ne!(
        dark.surfaces.overlay, dark.surfaces.raised,
        "dark overlay must take its own step — a dark shadow on a dark plane \
         is invisible, so surface is the only signal left"
    );
    assert!(
        contrast(dark.surfaces.overlay, dark.surfaces.raised) > 1.0,
        "the dark overlay step must actually be lighter than the panel"
    );
}

#[test]
fn the_scrim_is_translucent_and_the_planes_are_not() {
    for (mode, s) in modes() {
        assert!(
            s.surfaces.scrim.a < 1.0,
            "{mode}: a scrim must be see-through"
        );
        assert!(
            s.drag_drop.drop_target.a < 1.0,
            "{mode}: a drop wash must be see-through"
        );
        for p in s.planes() {
            assert_eq!(p.a, 1.0, "{mode}: a plane must be opaque");
        }
    }
}

/// A categorical hue over a surface, in the gamma space the painter blends in
/// (straight alpha, as `Rgba` carries it).
fn over(top: Rgba, under: Rgba) -> Rgba {
    let mix = |t: f32, u: f32| t.mul_add(top.a, u * (1.0 - top.a));
    Rgba::new(
        mix(top.r, under.r),
        mix(top.g, under.g),
        mix(top.b, under.b),
        1.0,
    )
}

/// A categorical hue marks a kind of data in chrome as a tint under its name
/// (`guidelines/colour.md`, "A hue in chrome"), so the ink that names it sits
/// on the tint and not on the bare surface. The tint is gated as what it does
/// to that ink: primary and secondary keep the text floor, muted keeps the
/// non-text floor it is held to on a bare surface, for all eight hues in both
/// modes over every background chrome can sit on.
#[test]
fn a_chrome_tint_leaves_the_ink_over_it_legible() {
    use meridian_design::viz::{chrome_tint, CATEGORICAL_DARK, CATEGORICAL_LIGHT};
    for (dark, mode, s) in [
        (false, "light", semantic(false)),
        (true, "dark", semantic(true)),
    ] {
        let palette = if dark {
            CATEGORICAL_DARK
        } else {
            CATEGORICAL_LIGHT
        };
        for (name, bg) in chrome_backgrounds(s) {
            for (slot, hue) in palette.iter().enumerate() {
                let tinted = over(chrome_tint(*hue, dark), bg);
                for (ink_name, ink, floor) in [
                    ("primary", s.text.primary, TEXT),
                    ("secondary", s.text.secondary, TEXT),
                    ("muted", s.text.muted, NON_TEXT),
                ] {
                    let c = contrast(ink, tinted);
                    assert!(
                        c >= floor,
                        "{mode}: text.{ink_name} on {name} under hue slot {slot} is \
                         {c:.2}:1, below {floor}"
                    );
                }
            }
        }
    }
}
