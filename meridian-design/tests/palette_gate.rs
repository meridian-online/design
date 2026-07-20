//! The palette gate (ADR 0006): every shipped palette must pass the method's
//! checks in both modes — this is the CI enforcement of the validation run
//! recorded in `validation/record-2026-07-16.txt`. If a palette edit lands,
//! these assertions are the contract it must re-clear.

use meridian_design::chrome::{INK_DARK, INK_LIGHT};
use meridian_design::validate::*;
use meridian_design::viz::*;

const LIGHT_BAND: (f64, f64) = (0.43, 0.77);
const DARK_BAND: (f64, f64) = (0.48, 0.67);
const CHROMA_FLOOR: f64 = 0.10;
const CVD_TARGET: f64 = 8.0;
const NORMAL_FLOOR: f64 = 15.0;

#[test]
fn categorical_light_passes_all_gates() {
    let p = CATEGORICAL_LIGHT;
    for (i, c) in p.iter().enumerate() {
        let l = okl(*c);
        assert!(
            l >= LIGHT_BAND.0 && l <= LIGHT_BAND.1,
            "slot {i} outside light band: L{l:.3}"
        );
        assert!(okc(*c) >= CHROMA_FLOOR, "slot {i} below chroma floor");
    }
    assert!(min_adjacent(&p, delta_e_cvd) >= CVD_TARGET);
    assert!(min_adjacent(&p, delta_e) >= NORMAL_FLOOR);
    // Relief slots are exactly gold(1), teal(2), orange(5): sub-3:1 by design,
    // mitigated by direct labels / table view. Everything else clears 3:1.
    for (i, c) in p.iter().enumerate() {
        let ct = contrast(*c, INK_LIGHT.surface);
        if [1, 2, 5].contains(&i) {
            assert!(
                ct < 3.0 && ct >= 2.0,
                "slot {i} left the documented relief band: {ct:.2}"
            );
        } else {
            assert!(ct >= 3.0, "slot {i} lost 3:1 on the light surface: {ct:.2}");
        }
    }
}

#[test]
fn categorical_dark_passes_all_gates() {
    let p = CATEGORICAL_DARK;
    for (i, c) in p.iter().enumerate() {
        let l = okl(*c);
        assert!(
            l >= DARK_BAND.0 && l <= DARK_BAND.1,
            "slot {i} outside dark band: L{l:.3}"
        );
        assert!(okc(*c) >= CHROMA_FLOOR, "slot {i} below chroma floor");
        assert!(
            contrast(*c, INK_DARK.surface) >= 3.0,
            "slot {i} below 3:1 on dark"
        );
    }
    assert!(min_adjacent(&p, delta_e_cvd) >= CVD_TARGET);
    assert!(min_adjacent(&p, delta_e) >= NORMAL_FLOOR);
}

#[test]
fn first_four_validate_all_pairs_in_both_modes() {
    for p in [&CATEGORICAL_LIGHT[..4], &CATEGORICAL_DARK[..4]] {
        assert!(min_all_pairs(p, delta_e_cvd) >= CVD_TARGET);
        assert!(min_all_pairs(p, delta_e) >= NORMAL_FLOOR);
    }
}

#[test]
fn sequential_meridian_is_a_single_hue_monotone_ramp() {
    let ramp = SEQUENTIAL_MERIDIAN;
    for w in ramp.windows(2) {
        assert!(okl(w[0]) > okl(w[1]), "lightness must strictly descend");
    }
    let hues: Vec<f64> = ramp.iter().map(|c| okh(*c)).collect();
    let (min, max) = hues
        .iter()
        .fold((f64::MAX, f64::MIN), |(a, b), h| (a.min(*h), b.max(*h)));
    assert!(
        max - min < 4.0,
        "hue drift {:.1}° — sequential must stay one hue",
        max - min
    );
}

#[test]
fn sequential_ordinal_bounds_hold() {
    // Documented ordinal bounds: light starts no lighter than step 250
    // (index 3), dark ends no darker than step 600 (index 10); the step
    // nearest the surface must clear 2:1 (canonical validator, 2026-07-16).
    assert!(contrast(SEQUENTIAL_MERIDIAN[3], INK_LIGHT.surface) >= 2.0);
    assert!(contrast(SEQUENTIAL_MERIDIAN[10], INK_DARK.surface) >= 2.0);
}

#[test]
fn null_ink_can_never_read_as_data() {
    // Below the series chroma floor in both modes — reads "no data".
    assert!(okc(NULL_INK_LIGHT) < CHROMA_FLOOR);
    assert!(okc(NULL_INK_DARK) < CHROMA_FLOOR);
}

#[test]
fn status_colours_clear_dark_surface_and_stay_off_series_slots() {
    for c in [STATUS.good, STATUS.warning, STATUS.serious, STATUS.critical] {
        assert!(contrast(c, INK_DARK.surface) >= 3.0);
    }
    // Status must never equal a categorical slot exactly (impersonation guard).
    for s in [STATUS.good, STATUS.warning, STATUS.serious, STATUS.critical] {
        for c in CATEGORICAL_LIGHT.iter().chain(CATEGORICAL_DARK.iter()) {
            assert!(delta_e(s, *c) > 1.0, "a status colour equals a series slot");
        }
    }
}

#[test]
fn diverging_arms_descend_toward_their_poles() {
    // Blue arm: pole → lightest; red arm: lightest → pole.
    for w in DIVERGING_BLUE_ARM.windows(2) {
        assert!(
            okl(w[0]) < okl(w[1]),
            "blue arm must lighten toward the midpoint"
        );
    }
    for w in DIVERGING_RED_ARM.windows(2) {
        assert!(okl(w[0]) > okl(w[1]), "red arm must darken toward the pole");
    }
    // Midpoints are neutral (below chroma floor) so "nothing" reads as nothing.
    assert!(okc(DIVERGING_MID_LIGHT) < CHROMA_FLOOR);
    assert!(okc(DIVERGING_MID_DARK) < CHROMA_FLOOR);
}
