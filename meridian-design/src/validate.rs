//! Palette validation maths — our own implementation of the published
//! standards (OKLab, Machado-Oliveira-Fernandes 2009 CVD simulation at
//! severity 1.0, WCAG relative-luminance contrast), matching the method's
//! canonical validator. `tests/palette_gate.rs` uses this to gate every
//! shipped palette in CI. All arithmetic in f64.

use crate::colour::Rgba;

fn s2lin(c: f64) -> f64 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

fn lin(c: Rgba) -> [f64; 3] {
    [s2lin(c.r as f64), s2lin(c.g as f64), s2lin(c.b as f64)]
}

/// Machado, Oliveira & Fernandes (2009) transforms, severity 1.0, linear RGB.
const PROTAN: [[f64; 3]; 3] = [
    [0.152286, 1.052583, -0.204868],
    [0.114503, 0.786281, 0.099216],
    [-0.003882, -0.048116, 1.051998],
];
const DEUTAN: [[f64; 3]; 3] = [
    [0.367322, 0.860646, -0.227968],
    [0.280085, 0.672501, 0.047413],
    [-0.011820, 0.042940, 0.968881],
];

fn simulate(l: [f64; 3], m: &[[f64; 3]; 3]) -> [f64; 3] {
    let mut out = [0.0; 3];
    for (i, row) in m.iter().enumerate() {
        out[i] = (row[0] * l[0] + row[1] * l[1] + row[2] * l[2]).clamp(0.0, 1.0);
    }
    out
}

fn oklab_from_lin(l: [f64; 3]) -> [f64; 3] {
    let a = (0.4122214708 * l[0] + 0.5363325363 * l[1] + 0.0514459929 * l[2]).cbrt();
    let b = (0.2119034982 * l[0] + 0.6806995451 * l[1] + 0.1073969566 * l[2]).cbrt();
    let c = (0.0883024619 * l[0] + 0.2817188376 * l[1] + 0.6299787005 * l[2]).cbrt();
    [
        0.2104542553 * a + 0.7936177850 * b - 0.0040720468 * c,
        1.9779984951 * a - 2.4285922050 * b + 0.4505937099 * c,
        0.0259040371 * a + 0.7827717662 * b - 0.8086757660 * c,
    ]
}

pub fn oklab(c: Rgba) -> [f64; 3] {
    oklab_from_lin(lin(c))
}

/// OKLCH lightness.
pub fn okl(c: Rgba) -> f64 {
    oklab(c)[0]
}

/// OKLCH chroma.
pub fn okc(c: Rgba) -> f64 {
    let [_, a, b] = oklab(c);
    a.hypot(b)
}

/// OKLCH hue in degrees.
pub fn okh(c: Rgba) -> f64 {
    let [_, a, b] = oklab(c);
    (b.atan2(a).to_degrees() % 360.0 + 360.0) % 360.0
}

fn de(a: [f64; 3], b: [f64; 3]) -> f64 {
    100.0 * ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// Euclidean OKLab distance ×100, unsimulated (normal) vision.
pub fn delta_e(c1: Rgba, c2: Rgba) -> f64 {
    de(oklab(c1), oklab(c2))
}

/// Worst-case CVD distance: min of protan/deutan-simulated OKLab ΔE ×100.
pub fn delta_e_cvd(c1: Rgba, c2: Rgba) -> f64 {
    let p = de(
        oklab_from_lin(simulate(lin(c1), &PROTAN)),
        oklab_from_lin(simulate(lin(c2), &PROTAN)),
    );
    let d = de(
        oklab_from_lin(simulate(lin(c1), &DEUTAN)),
        oklab_from_lin(simulate(lin(c2), &DEUTAN)),
    );
    p.min(d)
}

/// WCAG contrast ratio.
pub fn contrast(a: Rgba, b: Rgba) -> f64 {
    let y = |c: Rgba| {
        let l = lin(c);
        0.2126 * l[0] + 0.7152 * l[1] + 0.0722 * l[2]
    };
    let (ya, yb) = (y(a), y(b));
    let (hi, lo) = if ya > yb { (ya, yb) } else { (yb, ya) };
    (hi + 0.05) / (lo + 0.05)
}

/// Minimum adjacent-pair value of `f` over a slice.
pub fn min_adjacent(palette: &[Rgba], f: fn(Rgba, Rgba) -> f64) -> f64 {
    palette
        .windows(2)
        .map(|w| f(w[0], w[1]))
        .fold(f64::INFINITY, f64::min)
}

/// Minimum all-pairs value of `f` over a slice.
pub fn min_all_pairs(palette: &[Rgba], f: fn(Rgba, Rgba) -> f64) -> f64 {
    let mut m = f64::INFINITY;
    for i in 0..palette.len() {
        for j in i + 1..palette.len() {
            m = m.min(f(palette[i], palette[j]));
        }
    }
    m
}
