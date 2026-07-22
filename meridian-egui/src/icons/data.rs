//! Generated Tabler icon path constants — do not edit by hand.
//!
//! source: @tabler/icons 3.45.0, outline style (24×24 grid, stroke-width 2)
//!
//! Regenerate with the committed `gen-tabler-icons` bin (the manifest header
//! carries the full refresh command); `tests/icon_drift.rs` fails when this
//! file and `tabler.manifest` disagree.
//!
//! Icon artwork: Tabler Icons — MIT — © Paweł Kuna and contributors —
//! <https://tabler.io/icons>.

// Arc→cubic conversion emits whatever coordinates the maths produces; some
// happen to approximate well-known constants, which means nothing here.
#![allow(clippy::approx_constant)]

use kurbo::{PathEl, Point};

use super::Icon;

/// Tabler outline `alert-triangle`.
#[rustfmt::skip]
pub const ALERT_TRIANGLE: Icon = Icon {
    name: "alert-triangle",
    els: &[
        PathEl::MoveTo(Point::new(12.0, 9.0)),
        PathEl::LineTo(Point::new(12.0, 13.0)),
        PathEl::MoveTo(Point::new(10.363, 3.591)),
        PathEl::LineTo(Point::new(2.257, 17.125)),
        PathEl::CurveTo(Point::new(1.916961, 17.713865), Point::new(1.914885, 18.438931), Point::new(2.251546, 19.029734)),
        PathEl::CurveTo(Point::new(2.588207, 19.620537), Point::new(3.213051, 19.988361), Point::new(3.893, 19.996)),
        PathEl::LineTo(Point::new(20.107, 19.996)),
        PathEl::CurveTo(Point::new(20.786639, 19.98816), Point::new(21.411141, 19.620475), Point::new(21.747739, 19.02999)),
        PathEl::CurveTo(Point::new(22.084336, 18.439504), Point::new(22.082532, 17.714803), Point::new(21.743, 17.126)),
        PathEl::LineTo(Point::new(13.637, 3.59)),
        PathEl::CurveTo(Point::new(13.290142, 3.017486), Point::new(12.66939, 2.66778), Point::new(12.0, 2.66778)),
        PathEl::CurveTo(Point::new(11.33061, 2.66778), Point::new(10.709858, 3.017486), Point::new(10.363, 3.59)),
        PathEl::MoveTo(Point::new(12.0, 16.0)),
        PathEl::LineTo(Point::new(12.01, 16.0)),
    ],
};

/// Tabler outline `arrows-sort`.
#[rustfmt::skip]
pub const ARROWS_SORT: Icon = Icon {
    name: "arrows-sort",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 9.0)),
        PathEl::LineTo(Point::new(7.0, 5.0)),
        PathEl::LineTo(Point::new(11.0, 9.0)),
        PathEl::MoveTo(Point::new(7.0, 5.0)),
        PathEl::LineTo(Point::new(7.0, 19.0)),
        PathEl::MoveTo(Point::new(21.0, 15.0)),
        PathEl::LineTo(Point::new(17.0, 19.0)),
        PathEl::LineTo(Point::new(13.0, 15.0)),
        PathEl::MoveTo(Point::new(17.0, 19.0)),
        PathEl::LineTo(Point::new(17.0, 5.0)),
    ],
};

/// Tabler outline `chart-area`.
#[rustfmt::skip]
pub const CHART_AREA: Icon = Icon {
    name: "chart-area",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 19.0)),
        PathEl::LineTo(Point::new(20.0, 19.0)),
        PathEl::MoveTo(Point::new(4.0, 15.0)),
        PathEl::LineTo(Point::new(8.0, 9.0)),
        PathEl::LineTo(Point::new(12.0, 11.0)),
        PathEl::LineTo(Point::new(16.0, 6.0)),
        PathEl::LineTo(Point::new(20.0, 10.0)),
        PathEl::LineTo(Point::new(20.0, 15.0)),
        PathEl::LineTo(Point::new(4.0, 15.0)),
    ],
};

/// Tabler outline `chart-bar`.
#[rustfmt::skip]
pub const CHART_BAR: Icon = Icon {
    name: "chart-bar",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 13.0)),
        PathEl::CurveTo(Point::new(3.0, 12.447715), Point::new(3.447715, 12.0), Point::new(4.0, 12.0)),
        PathEl::LineTo(Point::new(8.0, 12.0)),
        PathEl::CurveTo(Point::new(8.552285, 12.0), Point::new(9.0, 12.447715), Point::new(9.0, 13.0)),
        PathEl::LineTo(Point::new(9.0, 19.0)),
        PathEl::CurveTo(Point::new(9.0, 19.552285), Point::new(8.552285, 20.0), Point::new(8.0, 20.0)),
        PathEl::LineTo(Point::new(4.0, 20.0)),
        PathEl::CurveTo(Point::new(3.447715, 20.0), Point::new(3.0, 19.552285), Point::new(3.0, 19.0)),
        PathEl::LineTo(Point::new(3.0, 13.0)),
        PathEl::MoveTo(Point::new(15.0, 9.0)),
        PathEl::CurveTo(Point::new(15.0, 8.447715), Point::new(15.447715, 8.0), Point::new(16.0, 8.0)),
        PathEl::LineTo(Point::new(20.0, 8.0)),
        PathEl::CurveTo(Point::new(20.552285, 8.0), Point::new(21.0, 8.447715), Point::new(21.0, 9.0)),
        PathEl::LineTo(Point::new(21.0, 19.0)),
        PathEl::CurveTo(Point::new(21.0, 19.552285), Point::new(20.552285, 20.0), Point::new(20.0, 20.0)),
        PathEl::LineTo(Point::new(16.0, 20.0)),
        PathEl::CurveTo(Point::new(15.447715, 20.0), Point::new(15.0, 19.552285), Point::new(15.0, 19.0)),
        PathEl::LineTo(Point::new(15.0, 9.0)),
        PathEl::MoveTo(Point::new(9.0, 5.0)),
        PathEl::CurveTo(Point::new(9.0, 4.447715), Point::new(9.447715, 4.0), Point::new(10.0, 4.0)),
        PathEl::LineTo(Point::new(14.0, 4.0)),
        PathEl::CurveTo(Point::new(14.552285, 4.0), Point::new(15.0, 4.447715), Point::new(15.0, 5.0)),
        PathEl::LineTo(Point::new(15.0, 19.0)),
        PathEl::CurveTo(Point::new(15.0, 19.552285), Point::new(14.552285, 20.0), Point::new(14.0, 20.0)),
        PathEl::LineTo(Point::new(10.0, 20.0)),
        PathEl::CurveTo(Point::new(9.447715, 20.0), Point::new(9.0, 19.552285), Point::new(9.0, 19.0)),
        PathEl::LineTo(Point::new(9.0, 5.0)),
        PathEl::MoveTo(Point::new(4.0, 20.0)),
        PathEl::LineTo(Point::new(18.0, 20.0)),
    ],
};

/// Tabler outline `chart-dots`.
#[rustfmt::skip]
pub const CHART_DOTS: Icon = Icon {
    name: "chart-dots",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 3.0)),
        PathEl::LineTo(Point::new(3.0, 21.0)),
        PathEl::LineTo(Point::new(21.0, 21.0)),
        PathEl::MoveTo(Point::new(7.0, 9.0)),
        PathEl::CurveTo(Point::new(7.0, 10.104569), Point::new(7.895431, 11.0), Point::new(9.0, 11.0)),
        PathEl::CurveTo(Point::new(10.104569, 11.0), Point::new(11.0, 10.104569), Point::new(11.0, 9.0)),
        PathEl::CurveTo(Point::new(11.0, 7.895431), Point::new(10.104569, 7.0), Point::new(9.0, 7.0)),
        PathEl::CurveTo(Point::new(7.895431, 7.0), Point::new(7.0, 7.895431), Point::new(7.0, 9.0)),
        PathEl::MoveTo(Point::new(17.0, 7.0)),
        PathEl::CurveTo(Point::new(17.0, 8.104569), Point::new(17.895431, 9.0), Point::new(19.0, 9.0)),
        PathEl::CurveTo(Point::new(20.104569, 9.0), Point::new(21.0, 8.104569), Point::new(21.0, 7.0)),
        PathEl::CurveTo(Point::new(21.0, 5.895431), Point::new(20.104569, 5.0), Point::new(19.0, 5.0)),
        PathEl::CurveTo(Point::new(17.895431, 5.0), Point::new(17.0, 5.895431), Point::new(17.0, 7.0)),
        PathEl::MoveTo(Point::new(12.0, 15.0)),
        PathEl::CurveTo(Point::new(12.0, 16.104569), Point::new(12.895431, 17.0), Point::new(14.0, 17.0)),
        PathEl::CurveTo(Point::new(15.104569, 17.0), Point::new(16.0, 16.104569), Point::new(16.0, 15.0)),
        PathEl::CurveTo(Point::new(16.0, 13.895431), Point::new(15.104569, 13.0), Point::new(14.0, 13.0)),
        PathEl::CurveTo(Point::new(12.895431, 13.0), Point::new(12.0, 13.895431), Point::new(12.0, 15.0)),
        PathEl::MoveTo(Point::new(10.16, 10.62)),
        PathEl::LineTo(Point::new(12.5, 13.5)),
        PathEl::MoveTo(Point::new(15.088, 13.328)),
        PathEl::LineTo(Point::new(17.925, 8.742)),
    ],
};

/// Tabler outline `chart-funnel`.
#[rustfmt::skip]
pub const CHART_FUNNEL: Icon = Icon {
    name: "chart-funnel",
    els: &[
        PathEl::MoveTo(Point::new(4.387, 3.0)),
        PathEl::LineTo(Point::new(19.613, 3.0)),
        PathEl::CurveTo(Point::new(19.934267, 3.000244), Point::new(20.235854, 3.154826), Point::new(20.423635, 3.4155)),
        PathEl::CurveTo(Point::new(20.611416, 3.676174), Point::new(20.662521, 4.011195), Point::new(20.561, 4.316)),
        PathEl::LineTo(Point::new(15.456, 19.632)),
        PathEl::CurveTo(Point::new(15.183856, 20.449086), Point::new(14.419215, 21.000207), Point::new(13.558, 21.0)),
        PathEl::LineTo(Point::new(10.442, 21.0)),
        PathEl::CurveTo(Point::new(9.580785, 21.000207), Point::new(8.816144, 20.449086), Point::new(8.544, 19.632)),
        PathEl::LineTo(Point::new(3.44, 4.316)),
        PathEl::CurveTo(Point::new(3.338535, 4.011362), Point::new(3.389525, 3.67653), Point::new(3.577072, 3.415905)),
        PathEl::CurveTo(Point::new(3.76462, 3.15528), Point::new(4.065909, 3.000565), Point::new(4.387, 3.0)),
        PathEl::MoveTo(Point::new(5.0, 9.0)),
        PathEl::LineTo(Point::new(19.0, 9.0)),
        PathEl::MoveTo(Point::new(7.0, 15.0)),
        PathEl::LineTo(Point::new(17.0, 15.0)),
    ],
};

/// Tabler outline `chart-histogram`.
#[rustfmt::skip]
pub const CHART_HISTOGRAM: Icon = Icon {
    name: "chart-histogram",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 3.0)),
        PathEl::LineTo(Point::new(3.0, 21.0)),
        PathEl::LineTo(Point::new(21.0, 21.0)),
        PathEl::MoveTo(Point::new(20.0, 18.0)),
        PathEl::LineTo(Point::new(20.0, 21.0)),
        PathEl::MoveTo(Point::new(16.0, 16.0)),
        PathEl::LineTo(Point::new(16.0, 21.0)),
        PathEl::MoveTo(Point::new(12.0, 13.0)),
        PathEl::LineTo(Point::new(12.0, 21.0)),
        PathEl::MoveTo(Point::new(8.0, 16.0)),
        PathEl::LineTo(Point::new(8.0, 21.0)),
        PathEl::MoveTo(Point::new(3.0, 11.0)),
        PathEl::CurveTo(Point::new(9.0, 11.0), Point::new(8.0, 6.0), Point::new(12.0, 6.0)),
        PathEl::CurveTo(Point::new(16.0, 6.0), Point::new(15.0, 11.0), Point::new(21.0, 11.0)),
    ],
};

/// Tabler outline `chart-line`.
#[rustfmt::skip]
pub const CHART_LINE: Icon = Icon {
    name: "chart-line",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 19.0)),
        PathEl::LineTo(Point::new(20.0, 19.0)),
        PathEl::MoveTo(Point::new(4.0, 15.0)),
        PathEl::LineTo(Point::new(8.0, 9.0)),
        PathEl::LineTo(Point::new(12.0, 11.0)),
        PathEl::LineTo(Point::new(16.0, 6.0)),
        PathEl::LineTo(Point::new(20.0, 10.0)),
    ],
};

/// Tabler outline `chart-pie`.
#[rustfmt::skip]
pub const CHART_PIE: Icon = Icon {
    name: "chart-pie",
    els: &[
        PathEl::MoveTo(Point::new(10.0, 3.2)),
        PathEl::CurveTo(Point::new(5.670017, 4.203791), Point::new(2.721258, 8.21823), Point::new(3.058442, 12.650233)),
        PathEl::CurveTo(Point::new(3.395626, 17.082236), Point::new(6.917764, 20.604374), Point::new(11.349767, 20.941558)),
        PathEl::CurveTo(Point::new(15.78177, 21.278742), Point::new(19.796209, 18.329983), Point::new(20.8, 14.0)),
        PathEl::CurveTo(Point::new(20.8, 13.447715), Point::new(20.352285, 13.0), Point::new(19.8, 13.0)),
        PathEl::LineTo(Point::new(13.0, 13.0)),
        PathEl::CurveTo(Point::new(11.895431, 13.0), Point::new(11.0, 12.104569), Point::new(11.0, 11.0)),
        PathEl::LineTo(Point::new(11.0, 4.0)),
        PathEl::CurveTo(Point::new(10.974905, 3.760692), Point::new(10.855027, 3.541397), Point::new(10.667134, 3.391083)),
        PathEl::CurveTo(Point::new(10.479241, 3.240768), Point::new(10.23898, 3.171952), Point::new(10.0, 3.2)),
        PathEl::MoveTo(Point::new(15.0, 3.5)),
        PathEl::CurveTo(Point::new(17.571806, 4.405563), Point::new(19.594437, 6.428194), Point::new(20.5, 9.0)),
        PathEl::LineTo(Point::new(16.0, 9.0)),
        PathEl::CurveTo(Point::new(15.447715, 9.0), Point::new(15.0, 8.552285), Point::new(15.0, 8.0)),
        PathEl::LineTo(Point::new(15.0, 3.5)),
    ],
};

/// Tabler outline `chart-radar`.
#[rustfmt::skip]
pub const CHART_RADAR: Icon = Icon {
    name: "chart-radar",
    els: &[
        PathEl::MoveTo(Point::new(12.0, 3.0)),
        PathEl::LineTo(Point::new(21.5, 10.0)),
        PathEl::LineTo(Point::new(18.0, 21.0)),
        PathEl::LineTo(Point::new(6.0, 21.0)),
        PathEl::LineTo(Point::new(2.5, 10.0)),
        PathEl::LineTo(Point::new(12.0, 3.0)),
        PathEl::MoveTo(Point::new(12.0, 7.5)),
        PathEl::LineTo(Point::new(17.5, 11.5)),
        PathEl::LineTo(Point::new(15.0, 17.0)),
        PathEl::LineTo(Point::new(8.5, 17.0)),
        PathEl::LineTo(Point::new(6.5, 11.5)),
        PathEl::LineTo(Point::new(12.0, 7.5)),
        PathEl::MoveTo(Point::new(2.5, 10.0)),
        PathEl::LineTo(Point::new(12.0, 13.0)),
        PathEl::LineTo(Point::new(21.5, 10.0)),
        PathEl::MoveTo(Point::new(12.0, 3.0)),
        PathEl::LineTo(Point::new(12.0, 13.0)),
        PathEl::LineTo(Point::new(18.0, 21.0)),
        PathEl::MoveTo(Point::new(6.0, 21.0)),
        PathEl::LineTo(Point::new(12.0, 13.0)),
    ],
};

/// Tabler outline `chart-sankey`.
#[rustfmt::skip]
pub const CHART_SANKEY: Icon = Icon {
    name: "chart-sankey",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::CurveTo(Point::new(10.944, 6.0), Point::new(13.056, 14.0), Point::new(20.0, 14.0)),
        PathEl::MoveTo(Point::new(4.0, 12.0)),
        PathEl::CurveTo(Point::new(10.37, 12.0), Point::new(13.63, 18.0), Point::new(20.0, 18.0)),
        PathEl::MoveTo(Point::new(20.0, 6.0)),
        PathEl::CurveTo(Point::new(12.474, 6.0), Point::new(12.095, 18.0), Point::new(4.0, 18.0)),
    ],
};

/// Tabler outline `chart-treemap`.
#[rustfmt::skip]
pub const CHART_TREEMAP: Icon = Icon {
    name: "chart-treemap",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::CurveTo(Point::new(4.0, 4.895431), Point::new(4.895431, 4.0), Point::new(6.0, 4.0)),
        PathEl::LineTo(Point::new(18.0, 4.0)),
        PathEl::CurveTo(Point::new(19.104569, 4.0), Point::new(20.0, 4.895431), Point::new(20.0, 6.0)),
        PathEl::LineTo(Point::new(20.0, 18.0)),
        PathEl::CurveTo(Point::new(20.0, 19.104569), Point::new(19.104569, 20.0), Point::new(18.0, 20.0)),
        PathEl::LineTo(Point::new(6.0, 20.0)),
        PathEl::CurveTo(Point::new(4.895431, 20.0), Point::new(4.0, 19.104569), Point::new(4.0, 18.0)),
        PathEl::LineTo(Point::new(4.0, 6.0)),
        PathEl::MoveTo(Point::new(12.0, 4.0)),
        PathEl::LineTo(Point::new(12.0, 20.0)),
        PathEl::MoveTo(Point::new(4.0, 15.0)),
        PathEl::LineTo(Point::new(12.0, 15.0)),
        PathEl::MoveTo(Point::new(12.0, 12.0)),
        PathEl::LineTo(Point::new(20.0, 12.0)),
        PathEl::MoveTo(Point::new(16.0, 12.0)),
        PathEl::LineTo(Point::new(16.0, 20.0)),
        PathEl::MoveTo(Point::new(16.0, 16.0)),
        PathEl::LineTo(Point::new(20.0, 16.0)),
    ],
};

/// Tabler outline `check`.
#[rustfmt::skip]
pub const CHECK: Icon = Icon {
    name: "check",
    els: &[
        PathEl::MoveTo(Point::new(5.0, 12.0)),
        PathEl::LineTo(Point::new(10.0, 17.0)),
        PathEl::LineTo(Point::new(20.0, 7.0)),
    ],
};

/// Tabler outline `chevron-down`.
#[rustfmt::skip]
pub const CHEVRON_DOWN: Icon = Icon {
    name: "chevron-down",
    els: &[
        PathEl::MoveTo(Point::new(6.0, 9.0)),
        PathEl::LineTo(Point::new(12.0, 15.0)),
        PathEl::LineTo(Point::new(18.0, 9.0)),
    ],
};

/// Tabler outline `chevron-left`.
#[rustfmt::skip]
pub const CHEVRON_LEFT: Icon = Icon {
    name: "chevron-left",
    els: &[
        PathEl::MoveTo(Point::new(15.0, 6.0)),
        PathEl::LineTo(Point::new(9.0, 12.0)),
        PathEl::LineTo(Point::new(15.0, 18.0)),
    ],
};

/// Tabler outline `chevron-right`.
#[rustfmt::skip]
pub const CHEVRON_RIGHT: Icon = Icon {
    name: "chevron-right",
    els: &[
        PathEl::MoveTo(Point::new(9.0, 6.0)),
        PathEl::LineTo(Point::new(15.0, 12.0)),
        PathEl::LineTo(Point::new(9.0, 18.0)),
    ],
};

/// Tabler outline `chevron-up`.
#[rustfmt::skip]
pub const CHEVRON_UP: Icon = Icon {
    name: "chevron-up",
    els: &[
        PathEl::MoveTo(Point::new(6.0, 15.0)),
        PathEl::LineTo(Point::new(12.0, 9.0)),
        PathEl::LineTo(Point::new(18.0, 15.0)),
    ],
};

/// Tabler outline `circle-check`.
#[rustfmt::skip]
pub const CIRCLE_CHECK: Icon = Icon {
    name: "circle-check",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 12.0)),
        PathEl::CurveTo(Point::new(3.0, 16.970563), Point::new(7.029437, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970563, 21.0), Point::new(21.0, 16.970563), Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(21.0, 7.029437), Point::new(16.970563, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.029437, 3.0), Point::new(3.0, 7.029437), Point::new(3.0, 12.0)),
        PathEl::MoveTo(Point::new(9.0, 12.0)),
        PathEl::LineTo(Point::new(11.0, 14.0)),
        PathEl::LineTo(Point::new(15.0, 10.0)),
    ],
};

/// Tabler outline `circle-x`.
#[rustfmt::skip]
pub const CIRCLE_X: Icon = Icon {
    name: "circle-x",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 12.0)),
        PathEl::CurveTo(Point::new(3.0, 16.970563), Point::new(7.029437, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970563, 21.0), Point::new(21.0, 16.970563), Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(21.0, 7.029437), Point::new(16.970563, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.029437, 3.0), Point::new(3.0, 7.029437), Point::new(3.0, 12.0)),
        PathEl::MoveTo(Point::new(10.0, 10.0)),
        PathEl::LineTo(Point::new(14.0, 14.0)),
        PathEl::MoveTo(Point::new(14.0, 10.0)),
        PathEl::LineTo(Point::new(10.0, 14.0)),
    ],
};

/// Tabler outline `clock`.
#[rustfmt::skip]
pub const CLOCK: Icon = Icon {
    name: "clock",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 12.0)),
        PathEl::CurveTo(Point::new(3.0, 16.970563), Point::new(7.029437, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970563, 21.0), Point::new(21.0, 16.970563), Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(21.0, 7.029437), Point::new(16.970563, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.029437, 3.0), Point::new(3.0, 7.029437), Point::new(3.0, 12.0)),
        PathEl::MoveTo(Point::new(12.0, 7.0)),
        PathEl::LineTo(Point::new(12.0, 12.0)),
        PathEl::LineTo(Point::new(15.0, 15.0)),
    ],
};

/// Tabler outline `copy`.
#[rustfmt::skip]
pub const COPY: Icon = Icon {
    name: "copy",
    els: &[
        PathEl::MoveTo(Point::new(7.0, 9.667)),
        PathEl::CurveTo(Point::new(7.0, 8.194057), Point::new(8.194057, 7.0), Point::new(9.667, 7.0)),
        PathEl::LineTo(Point::new(18.333, 7.0)),
        PathEl::CurveTo(Point::new(19.805943, 7.0), Point::new(21.0, 8.194057), Point::new(21.0, 9.667)),
        PathEl::LineTo(Point::new(21.0, 18.333)),
        PathEl::CurveTo(Point::new(21.0, 19.805943), Point::new(19.805943, 21.0), Point::new(18.333, 21.0)),
        PathEl::LineTo(Point::new(9.667, 21.0)),
        PathEl::CurveTo(Point::new(8.194057, 21.0), Point::new(7.0, 19.805943), Point::new(7.0, 18.333)),
        PathEl::LineTo(Point::new(7.0, 9.667)),
        PathEl::MoveTo(Point::new(4.012, 16.737)),
        PathEl::CurveTo(Point::new(3.387801, 16.381153), Point::new(3.001732, 15.718504), Point::new(3.0, 15.0)),
        PathEl::LineTo(Point::new(3.0, 5.0)),
        PathEl::CurveTo(Point::new(3.0, 3.9), Point::new(3.9, 3.0), Point::new(5.0, 3.0)),
        PathEl::LineTo(Point::new(15.0, 3.0)),
        PathEl::CurveTo(Point::new(15.75, 3.0), Point::new(16.158, 3.385), Point::new(16.5, 4.0)),
    ],
};

/// Tabler outline `database`.
#[rustfmt::skip]
pub const DATABASE: Icon = Icon {
    name: "database",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::CurveTo(Point::new(4.0, 7.656854), Point::new(7.581722, 9.0), Point::new(12.0, 9.0)),
        PathEl::CurveTo(Point::new(16.418278, 9.0), Point::new(20.0, 7.656854), Point::new(20.0, 6.0)),
        PathEl::CurveTo(Point::new(20.0, 4.343146), Point::new(16.418278, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.581722, 3.0), Point::new(4.0, 4.343146), Point::new(4.0, 6.0)),
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::LineTo(Point::new(4.0, 12.0)),
        PathEl::CurveTo(Point::new(4.0, 13.656854), Point::new(7.581722, 15.0), Point::new(12.0, 15.0)),
        PathEl::CurveTo(Point::new(16.418278, 15.0), Point::new(20.0, 13.656854), Point::new(20.0, 12.0)),
        PathEl::LineTo(Point::new(20.0, 6.0)),
        PathEl::MoveTo(Point::new(4.0, 12.0)),
        PathEl::LineTo(Point::new(4.0, 18.0)),
        PathEl::CurveTo(Point::new(4.0, 19.656854), Point::new(7.581722, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.418278, 21.0), Point::new(20.0, 19.656854), Point::new(20.0, 18.0)),
        PathEl::LineTo(Point::new(20.0, 12.0)),
    ],
};

/// Tabler outline `dots-vertical`.
#[rustfmt::skip]
pub const DOTS_VERTICAL: Icon = Icon {
    name: "dots-vertical",
    els: &[
        PathEl::MoveTo(Point::new(11.0, 12.0)),
        PathEl::CurveTo(Point::new(11.0, 12.552285), Point::new(11.447715, 13.0), Point::new(12.0, 13.0)),
        PathEl::CurveTo(Point::new(12.552285, 13.0), Point::new(13.0, 12.552285), Point::new(13.0, 12.0)),
        PathEl::CurveTo(Point::new(13.0, 11.447715), Point::new(12.552285, 11.0), Point::new(12.0, 11.0)),
        PathEl::CurveTo(Point::new(11.447715, 11.0), Point::new(11.0, 11.447715), Point::new(11.0, 12.0)),
        PathEl::MoveTo(Point::new(11.0, 19.0)),
        PathEl::CurveTo(Point::new(11.0, 19.552285), Point::new(11.447715, 20.0), Point::new(12.0, 20.0)),
        PathEl::CurveTo(Point::new(12.552285, 20.0), Point::new(13.0, 19.552285), Point::new(13.0, 19.0)),
        PathEl::CurveTo(Point::new(13.0, 18.447715), Point::new(12.552285, 18.0), Point::new(12.0, 18.0)),
        PathEl::CurveTo(Point::new(11.447715, 18.0), Point::new(11.0, 18.447715), Point::new(11.0, 19.0)),
        PathEl::MoveTo(Point::new(11.0, 5.0)),
        PathEl::CurveTo(Point::new(11.0, 5.552285), Point::new(11.447715, 6.0), Point::new(12.0, 6.0)),
        PathEl::CurveTo(Point::new(12.552285, 6.0), Point::new(13.0, 5.552285), Point::new(13.0, 5.0)),
        PathEl::CurveTo(Point::new(13.0, 4.447715), Point::new(12.552285, 4.0), Point::new(12.0, 4.0)),
        PathEl::CurveTo(Point::new(11.447715, 4.0), Point::new(11.0, 4.447715), Point::new(11.0, 5.0)),
    ],
};

/// Tabler outline `download`.
#[rustfmt::skip]
pub const DOWNLOAD: Icon = Icon {
    name: "download",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 17.0)),
        PathEl::LineTo(Point::new(4.0, 19.0)),
        PathEl::CurveTo(Point::new(4.0, 20.104569), Point::new(4.895431, 21.0), Point::new(6.0, 21.0)),
        PathEl::LineTo(Point::new(18.0, 21.0)),
        PathEl::CurveTo(Point::new(19.104569, 21.0), Point::new(20.0, 20.104569), Point::new(20.0, 19.0)),
        PathEl::LineTo(Point::new(20.0, 17.0)),
        PathEl::MoveTo(Point::new(7.0, 11.0)),
        PathEl::LineTo(Point::new(12.0, 16.0)),
        PathEl::LineTo(Point::new(17.0, 11.0)),
        PathEl::MoveTo(Point::new(12.0, 4.0)),
        PathEl::LineTo(Point::new(12.0, 16.0)),
    ],
};

/// Tabler outline `external-link`.
#[rustfmt::skip]
pub const EXTERNAL_LINK: Icon = Icon {
    name: "external-link",
    els: &[
        PathEl::MoveTo(Point::new(12.0, 6.0)),
        PathEl::LineTo(Point::new(6.0, 6.0)),
        PathEl::CurveTo(Point::new(4.895431, 6.0), Point::new(4.0, 6.895431), Point::new(4.0, 8.0)),
        PathEl::LineTo(Point::new(4.0, 18.0)),
        PathEl::CurveTo(Point::new(4.0, 19.104569), Point::new(4.895431, 20.0), Point::new(6.0, 20.0)),
        PathEl::LineTo(Point::new(16.0, 20.0)),
        PathEl::CurveTo(Point::new(17.104569, 20.0), Point::new(18.0, 19.104569), Point::new(18.0, 18.0)),
        PathEl::LineTo(Point::new(18.0, 12.0)),
        PathEl::MoveTo(Point::new(11.0, 13.0)),
        PathEl::LineTo(Point::new(20.0, 4.0)),
        PathEl::MoveTo(Point::new(15.0, 4.0)),
        PathEl::LineTo(Point::new(20.0, 4.0)),
        PathEl::LineTo(Point::new(20.0, 9.0)),
    ],
};

/// Tabler outline `eye`.
#[rustfmt::skip]
pub const EYE: Icon = Icon {
    name: "eye",
    els: &[
        PathEl::MoveTo(Point::new(10.0, 12.0)),
        PathEl::CurveTo(Point::new(10.0, 13.104569), Point::new(10.895431, 14.0), Point::new(12.0, 14.0)),
        PathEl::CurveTo(Point::new(13.104569, 14.0), Point::new(14.0, 13.104569), Point::new(14.0, 12.0)),
        PathEl::CurveTo(Point::new(14.0, 10.895431), Point::new(13.104569, 10.0), Point::new(12.0, 10.0)),
        PathEl::CurveTo(Point::new(10.895431, 10.0), Point::new(10.0, 10.895431), Point::new(10.0, 12.0)),
        PathEl::MoveTo(Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(18.6, 16.0), Point::new(15.6, 18.0), Point::new(12.0, 18.0)),
        PathEl::CurveTo(Point::new(8.4, 18.0), Point::new(5.4, 16.0), Point::new(3.0, 12.0)),
        PathEl::CurveTo(Point::new(5.4, 8.0), Point::new(8.4, 6.0), Point::new(12.0, 6.0)),
        PathEl::CurveTo(Point::new(15.6, 6.0), Point::new(18.6, 8.0), Point::new(21.0, 12.0)),
    ],
};

/// Tabler outline `file`.
#[rustfmt::skip]
pub const FILE: Icon = Icon {
    name: "file",
    els: &[
        PathEl::MoveTo(Point::new(14.0, 3.0)),
        PathEl::LineTo(Point::new(14.0, 7.0)),
        PathEl::CurveTo(Point::new(14.0, 7.552285), Point::new(14.447715, 8.0), Point::new(15.0, 8.0)),
        PathEl::LineTo(Point::new(19.0, 8.0)),
        PathEl::MoveTo(Point::new(17.0, 21.0)),
        PathEl::LineTo(Point::new(7.0, 21.0)),
        PathEl::CurveTo(Point::new(5.895431, 21.0), Point::new(5.0, 20.104569), Point::new(5.0, 19.0)),
        PathEl::LineTo(Point::new(5.0, 5.0)),
        PathEl::CurveTo(Point::new(5.0, 3.895431), Point::new(5.895431, 3.0), Point::new(7.0, 3.0)),
        PathEl::LineTo(Point::new(14.0, 3.0)),
        PathEl::LineTo(Point::new(19.0, 8.0)),
        PathEl::LineTo(Point::new(19.0, 19.0)),
        PathEl::CurveTo(Point::new(19.0, 20.104569), Point::new(18.104569, 21.0), Point::new(17.0, 21.0)),
    ],
};

/// Tabler outline `filter`.
#[rustfmt::skip]
pub const FILTER: Icon = Icon {
    name: "filter",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 4.0)),
        PathEl::LineTo(Point::new(20.0, 4.0)),
        PathEl::LineTo(Point::new(20.0, 6.172)),
        PathEl::CurveTo(Point::new(19.999887, 6.70239), Point::new(19.789099, 7.211015), Point::new(19.414, 7.586)),
        PathEl::LineTo(Point::new(15.0, 12.0)),
        PathEl::LineTo(Point::new(15.0, 19.0)),
        PathEl::LineTo(Point::new(9.0, 21.0)),
        PathEl::LineTo(Point::new(9.0, 12.5)),
        PathEl::LineTo(Point::new(4.52, 7.572)),
        PathEl::CurveTo(Point::new(4.185447, 7.203926), Point::new(4.000053, 6.724397), Point::new(4.0, 6.227)),
        PathEl::LineTo(Point::new(4.0, 4.0)),
    ],
};

/// Tabler outline `folder`.
#[rustfmt::skip]
pub const FOLDER: Icon = Icon {
    name: "folder",
    els: &[
        PathEl::MoveTo(Point::new(5.0, 4.0)),
        PathEl::LineTo(Point::new(9.0, 4.0)),
        PathEl::LineTo(Point::new(12.0, 7.0)),
        PathEl::LineTo(Point::new(19.0, 7.0)),
        PathEl::CurveTo(Point::new(20.104569, 7.0), Point::new(21.0, 7.895431), Point::new(21.0, 9.0)),
        PathEl::LineTo(Point::new(21.0, 17.0)),
        PathEl::CurveTo(Point::new(21.0, 18.104569), Point::new(20.104569, 19.0), Point::new(19.0, 19.0)),
        PathEl::LineTo(Point::new(5.0, 19.0)),
        PathEl::CurveTo(Point::new(3.895431, 19.0), Point::new(3.0, 18.104569), Point::new(3.0, 17.0)),
        PathEl::LineTo(Point::new(3.0, 6.0)),
        PathEl::CurveTo(Point::new(3.0, 4.895431), Point::new(3.895431, 4.0), Point::new(5.0, 4.0)),
    ],
};

/// Tabler outline `grip-vertical`.
#[rustfmt::skip]
pub const GRIP_VERTICAL: Icon = Icon {
    name: "grip-vertical",
    els: &[
        PathEl::MoveTo(Point::new(8.0, 5.0)),
        PathEl::CurveTo(Point::new(8.0, 5.552285), Point::new(8.447715, 6.0), Point::new(9.0, 6.0)),
        PathEl::CurveTo(Point::new(9.552285, 6.0), Point::new(10.0, 5.552285), Point::new(10.0, 5.0)),
        PathEl::CurveTo(Point::new(10.0, 4.447715), Point::new(9.552285, 4.0), Point::new(9.0, 4.0)),
        PathEl::CurveTo(Point::new(8.447715, 4.0), Point::new(8.0, 4.447715), Point::new(8.0, 5.0)),
        PathEl::MoveTo(Point::new(8.0, 12.0)),
        PathEl::CurveTo(Point::new(8.0, 12.552285), Point::new(8.447715, 13.0), Point::new(9.0, 13.0)),
        PathEl::CurveTo(Point::new(9.552285, 13.0), Point::new(10.0, 12.552285), Point::new(10.0, 12.0)),
        PathEl::CurveTo(Point::new(10.0, 11.447715), Point::new(9.552285, 11.0), Point::new(9.0, 11.0)),
        PathEl::CurveTo(Point::new(8.447715, 11.0), Point::new(8.0, 11.447715), Point::new(8.0, 12.0)),
        PathEl::MoveTo(Point::new(8.0, 19.0)),
        PathEl::CurveTo(Point::new(8.0, 19.552285), Point::new(8.447715, 20.0), Point::new(9.0, 20.0)),
        PathEl::CurveTo(Point::new(9.552285, 20.0), Point::new(10.0, 19.552285), Point::new(10.0, 19.0)),
        PathEl::CurveTo(Point::new(10.0, 18.447715), Point::new(9.552285, 18.0), Point::new(9.0, 18.0)),
        PathEl::CurveTo(Point::new(8.447715, 18.0), Point::new(8.0, 18.447715), Point::new(8.0, 19.0)),
        PathEl::MoveTo(Point::new(14.0, 5.0)),
        PathEl::CurveTo(Point::new(14.0, 5.552285), Point::new(14.447715, 6.0), Point::new(15.0, 6.0)),
        PathEl::CurveTo(Point::new(15.552285, 6.0), Point::new(16.0, 5.552285), Point::new(16.0, 5.0)),
        PathEl::CurveTo(Point::new(16.0, 4.447715), Point::new(15.552285, 4.0), Point::new(15.0, 4.0)),
        PathEl::CurveTo(Point::new(14.447715, 4.0), Point::new(14.0, 4.447715), Point::new(14.0, 5.0)),
        PathEl::MoveTo(Point::new(14.0, 12.0)),
        PathEl::CurveTo(Point::new(14.0, 12.552285), Point::new(14.447715, 13.0), Point::new(15.0, 13.0)),
        PathEl::CurveTo(Point::new(15.552285, 13.0), Point::new(16.0, 12.552285), Point::new(16.0, 12.0)),
        PathEl::CurveTo(Point::new(16.0, 11.447715), Point::new(15.552285, 11.0), Point::new(15.0, 11.0)),
        PathEl::CurveTo(Point::new(14.447715, 11.0), Point::new(14.0, 11.447715), Point::new(14.0, 12.0)),
        PathEl::MoveTo(Point::new(14.0, 19.0)),
        PathEl::CurveTo(Point::new(14.0, 19.552285), Point::new(14.447715, 20.0), Point::new(15.0, 20.0)),
        PathEl::CurveTo(Point::new(15.552285, 20.0), Point::new(16.0, 19.552285), Point::new(16.0, 19.0)),
        PathEl::CurveTo(Point::new(16.0, 18.447715), Point::new(15.552285, 18.0), Point::new(15.0, 18.0)),
        PathEl::CurveTo(Point::new(14.447715, 18.0), Point::new(14.0, 18.447715), Point::new(14.0, 19.0)),
    ],
};

/// Tabler outline `info-circle`.
#[rustfmt::skip]
pub const INFO_CIRCLE: Icon = Icon {
    name: "info-circle",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 12.0)),
        PathEl::CurveTo(Point::new(3.0, 16.970563), Point::new(7.029437, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970563, 21.0), Point::new(21.0, 16.970563), Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(21.0, 7.029437), Point::new(16.970563, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.029437, 3.0), Point::new(3.0, 7.029437), Point::new(3.0, 12.0)),
        PathEl::MoveTo(Point::new(12.0, 9.0)),
        PathEl::LineTo(Point::new(12.01, 9.0)),
        PathEl::MoveTo(Point::new(11.0, 12.0)),
        PathEl::LineTo(Point::new(12.0, 12.0)),
        PathEl::LineTo(Point::new(12.0, 16.0)),
        PathEl::LineTo(Point::new(13.0, 16.0)),
    ],
};

/// Tabler outline `json`.
#[rustfmt::skip]
pub const JSON: Icon = Icon {
    name: "json",
    els: &[
        PathEl::MoveTo(Point::new(20.0, 16.0)),
        PathEl::LineTo(Point::new(20.0, 8.0)),
        PathEl::LineTo(Point::new(23.0, 16.0)),
        PathEl::LineTo(Point::new(23.0, 8.0)),
        PathEl::MoveTo(Point::new(15.0, 8.0)),
        PathEl::CurveTo(Point::new(16.104569, 8.0), Point::new(17.0, 8.895431), Point::new(17.0, 10.0)),
        PathEl::LineTo(Point::new(17.0, 14.0)),
        PathEl::CurveTo(Point::new(17.0, 15.104569), Point::new(16.104569, 16.0), Point::new(15.0, 16.0)),
        PathEl::CurveTo(Point::new(13.895431, 16.0), Point::new(13.0, 15.104569), Point::new(13.0, 14.0)),
        PathEl::LineTo(Point::new(13.0, 10.0)),
        PathEl::CurveTo(Point::new(13.0, 8.895431), Point::new(13.895431, 8.0), Point::new(15.0, 8.0)),
        PathEl::MoveTo(Point::new(1.0, 8.0)),
        PathEl::LineTo(Point::new(4.0, 8.0)),
        PathEl::LineTo(Point::new(4.0, 14.5)),
        PathEl::CurveTo(Point::new(4.0, 15.328427), Point::new(3.328427, 16.0), Point::new(2.5, 16.0)),
        PathEl::CurveTo(Point::new(1.671573, 16.0), Point::new(1.0, 15.328427), Point::new(1.0, 14.5)),
        PathEl::LineTo(Point::new(1.0, 14.0)),
        PathEl::MoveTo(Point::new(7.0, 15.0)),
        PathEl::CurveTo(Point::new(7.0, 15.552285), Point::new(7.447715, 16.0), Point::new(8.0, 16.0)),
        PathEl::LineTo(Point::new(9.0, 16.0)),
        PathEl::CurveTo(Point::new(9.552285, 16.0), Point::new(10.0, 15.552285), Point::new(10.0, 15.0)),
        PathEl::LineTo(Point::new(10.0, 13.0)),
        PathEl::CurveTo(Point::new(10.0, 12.447715), Point::new(9.552285, 12.0), Point::new(9.0, 12.0)),
        PathEl::LineTo(Point::new(8.0, 12.0)),
        PathEl::CurveTo(Point::new(7.447715, 12.0), Point::new(7.0, 11.552285), Point::new(7.0, 11.0)),
        PathEl::LineTo(Point::new(7.0, 9.0)),
        PathEl::CurveTo(Point::new(7.0, 8.447715), Point::new(7.447715, 8.0), Point::new(8.0, 8.0)),
        PathEl::LineTo(Point::new(9.0, 8.0)),
        PathEl::CurveTo(Point::new(9.552285, 8.0), Point::new(10.0, 8.447715), Point::new(10.0, 9.0)),
    ],
};

/// Tabler outline `layout-sidebar`.
#[rustfmt::skip]
pub const LAYOUT_SIDEBAR: Icon = Icon {
    name: "layout-sidebar",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::CurveTo(Point::new(4.0, 4.895431), Point::new(4.895431, 4.0), Point::new(6.0, 4.0)),
        PathEl::LineTo(Point::new(18.0, 4.0)),
        PathEl::CurveTo(Point::new(19.104569, 4.0), Point::new(20.0, 4.895431), Point::new(20.0, 6.0)),
        PathEl::LineTo(Point::new(20.0, 18.0)),
        PathEl::CurveTo(Point::new(20.0, 19.104569), Point::new(19.104569, 20.0), Point::new(18.0, 20.0)),
        PathEl::LineTo(Point::new(6.0, 20.0)),
        PathEl::CurveTo(Point::new(4.895431, 20.0), Point::new(4.0, 19.104569), Point::new(4.0, 18.0)),
        PathEl::LineTo(Point::new(4.0, 6.0)),
        PathEl::MoveTo(Point::new(9.0, 4.0)),
        PathEl::LineTo(Point::new(9.0, 20.0)),
    ],
};

/// Tabler outline `loader-2`.
#[rustfmt::skip]
pub const LOADER_2: Icon = Icon {
    name: "loader-2",
    els: &[
        PathEl::MoveTo(Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.029437, 3.0), Point::new(3.0, 7.029437), Point::new(3.0, 12.0)),
        PathEl::CurveTo(Point::new(3.0, 16.970563), Point::new(7.029437, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970563, 21.0), Point::new(21.0, 16.970563), Point::new(21.0, 12.0)),
    ],
};

/// Tabler outline `lock`.
#[rustfmt::skip]
pub const LOCK: Icon = Icon {
    name: "lock",
    els: &[
        PathEl::MoveTo(Point::new(5.0, 13.0)),
        PathEl::CurveTo(Point::new(5.0, 11.895431), Point::new(5.895431, 11.0), Point::new(7.0, 11.0)),
        PathEl::LineTo(Point::new(17.0, 11.0)),
        PathEl::CurveTo(Point::new(18.104569, 11.0), Point::new(19.0, 11.895431), Point::new(19.0, 13.0)),
        PathEl::LineTo(Point::new(19.0, 19.0)),
        PathEl::CurveTo(Point::new(19.0, 20.104569), Point::new(18.104569, 21.0), Point::new(17.0, 21.0)),
        PathEl::LineTo(Point::new(7.0, 21.0)),
        PathEl::CurveTo(Point::new(5.895431, 21.0), Point::new(5.0, 20.104569), Point::new(5.0, 19.0)),
        PathEl::LineTo(Point::new(5.0, 13.0)),
        PathEl::MoveTo(Point::new(11.0, 16.0)),
        PathEl::CurveTo(Point::new(11.0, 16.552285), Point::new(11.447715, 17.0), Point::new(12.0, 17.0)),
        PathEl::CurveTo(Point::new(12.552285, 17.0), Point::new(13.0, 16.552285), Point::new(13.0, 16.0)),
        PathEl::CurveTo(Point::new(13.0, 15.447715), Point::new(12.552285, 15.0), Point::new(12.0, 15.0)),
        PathEl::CurveTo(Point::new(11.447715, 15.0), Point::new(11.0, 15.447715), Point::new(11.0, 16.0)),
        PathEl::MoveTo(Point::new(8.0, 11.0)),
        PathEl::LineTo(Point::new(8.0, 7.0)),
        PathEl::CurveTo(Point::new(8.0, 4.790861), Point::new(9.790861, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(14.209139, 3.0), Point::new(16.0, 4.790861), Point::new(16.0, 7.0)),
        PathEl::LineTo(Point::new(16.0, 11.0)),
    ],
};

/// Tabler outline `maximize`.
#[rustfmt::skip]
pub const MAXIMIZE: Icon = Icon {
    name: "maximize",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 8.0)),
        PathEl::LineTo(Point::new(4.0, 6.0)),
        PathEl::CurveTo(Point::new(4.0, 4.895431), Point::new(4.895431, 4.0), Point::new(6.0, 4.0)),
        PathEl::LineTo(Point::new(8.0, 4.0)),
        PathEl::MoveTo(Point::new(4.0, 16.0)),
        PathEl::LineTo(Point::new(4.0, 18.0)),
        PathEl::CurveTo(Point::new(4.0, 19.104569), Point::new(4.895431, 20.0), Point::new(6.0, 20.0)),
        PathEl::LineTo(Point::new(8.0, 20.0)),
        PathEl::MoveTo(Point::new(16.0, 4.0)),
        PathEl::LineTo(Point::new(18.0, 4.0)),
        PathEl::CurveTo(Point::new(19.104569, 4.0), Point::new(20.0, 4.895431), Point::new(20.0, 6.0)),
        PathEl::LineTo(Point::new(20.0, 8.0)),
        PathEl::MoveTo(Point::new(16.0, 20.0)),
        PathEl::LineTo(Point::new(18.0, 20.0)),
        PathEl::CurveTo(Point::new(19.104569, 20.0), Point::new(20.0, 19.104569), Point::new(20.0, 18.0)),
        PathEl::LineTo(Point::new(20.0, 16.0)),
    ],
};

/// Tabler outline `minimize`.
#[rustfmt::skip]
pub const MINIMIZE: Icon = Icon {
    name: "minimize",
    els: &[
        PathEl::MoveTo(Point::new(15.0, 19.0)),
        PathEl::LineTo(Point::new(15.0, 17.0)),
        PathEl::CurveTo(Point::new(15.0, 15.895431), Point::new(15.895431, 15.0), Point::new(17.0, 15.0)),
        PathEl::LineTo(Point::new(19.0, 15.0)),
        PathEl::MoveTo(Point::new(15.0, 5.0)),
        PathEl::LineTo(Point::new(15.0, 7.0)),
        PathEl::CurveTo(Point::new(15.0, 8.104569), Point::new(15.895431, 9.0), Point::new(17.0, 9.0)),
        PathEl::LineTo(Point::new(19.0, 9.0)),
        PathEl::MoveTo(Point::new(5.0, 15.0)),
        PathEl::LineTo(Point::new(7.0, 15.0)),
        PathEl::CurveTo(Point::new(8.104569, 15.0), Point::new(9.0, 15.895431), Point::new(9.0, 17.0)),
        PathEl::LineTo(Point::new(9.0, 19.0)),
        PathEl::MoveTo(Point::new(5.0, 9.0)),
        PathEl::LineTo(Point::new(7.0, 9.0)),
        PathEl::CurveTo(Point::new(8.104569, 9.0), Point::new(9.0, 8.104569), Point::new(9.0, 7.0)),
        PathEl::LineTo(Point::new(9.0, 5.0)),
    ],
};

/// Tabler outline `player-pause`.
#[rustfmt::skip]
pub const PLAYER_PAUSE: Icon = Icon {
    name: "player-pause",
    els: &[
        PathEl::MoveTo(Point::new(6.0, 6.0)),
        PathEl::CurveTo(Point::new(6.0, 5.447715), Point::new(6.447715, 5.0), Point::new(7.0, 5.0)),
        PathEl::LineTo(Point::new(9.0, 5.0)),
        PathEl::CurveTo(Point::new(9.552285, 5.0), Point::new(10.0, 5.447715), Point::new(10.0, 6.0)),
        PathEl::LineTo(Point::new(10.0, 18.0)),
        PathEl::CurveTo(Point::new(10.0, 18.552285), Point::new(9.552285, 19.0), Point::new(9.0, 19.0)),
        PathEl::LineTo(Point::new(7.0, 19.0)),
        PathEl::CurveTo(Point::new(6.447715, 19.0), Point::new(6.0, 18.552285), Point::new(6.0, 18.0)),
        PathEl::LineTo(Point::new(6.0, 6.0)),
        PathEl::MoveTo(Point::new(14.0, 6.0)),
        PathEl::CurveTo(Point::new(14.0, 5.447715), Point::new(14.447715, 5.0), Point::new(15.0, 5.0)),
        PathEl::LineTo(Point::new(17.0, 5.0)),
        PathEl::CurveTo(Point::new(17.552285, 5.0), Point::new(18.0, 5.447715), Point::new(18.0, 6.0)),
        PathEl::LineTo(Point::new(18.0, 18.0)),
        PathEl::CurveTo(Point::new(18.0, 18.552285), Point::new(17.552285, 19.0), Point::new(17.0, 19.0)),
        PathEl::LineTo(Point::new(15.0, 19.0)),
        PathEl::CurveTo(Point::new(14.447715, 19.0), Point::new(14.0, 18.552285), Point::new(14.0, 18.0)),
        PathEl::LineTo(Point::new(14.0, 6.0)),
    ],
};

/// Tabler outline `player-play`.
#[rustfmt::skip]
pub const PLAYER_PLAY: Icon = Icon {
    name: "player-play",
    els: &[
        PathEl::MoveTo(Point::new(7.0, 4.0)),
        PathEl::LineTo(Point::new(7.0, 20.0)),
        PathEl::LineTo(Point::new(20.0, 12.0)),
        PathEl::LineTo(Point::new(7.0, 4.0)),
    ],
};

/// Tabler outline `player-stop`.
#[rustfmt::skip]
pub const PLAYER_STOP: Icon = Icon {
    name: "player-stop",
    els: &[
        PathEl::MoveTo(Point::new(5.0, 7.0)),
        PathEl::CurveTo(Point::new(5.0, 5.895431), Point::new(5.895431, 5.0), Point::new(7.0, 5.0)),
        PathEl::LineTo(Point::new(17.0, 5.0)),
        PathEl::CurveTo(Point::new(18.104569, 5.0), Point::new(19.0, 5.895431), Point::new(19.0, 7.0)),
        PathEl::LineTo(Point::new(19.0, 17.0)),
        PathEl::CurveTo(Point::new(19.0, 18.104569), Point::new(18.104569, 19.0), Point::new(17.0, 19.0)),
        PathEl::LineTo(Point::new(7.0, 19.0)),
        PathEl::CurveTo(Point::new(5.895431, 19.0), Point::new(5.0, 18.104569), Point::new(5.0, 17.0)),
        PathEl::LineTo(Point::new(5.0, 7.0)),
    ],
};

/// Tabler outline `refresh`.
#[rustfmt::skip]
pub const REFRESH: Icon = Icon {
    name: "refresh",
    els: &[
        PathEl::MoveTo(Point::new(20.0, 11.0)),
        PathEl::CurveTo(Point::new(19.497276, 7.382537), Point::new(16.635869, 4.548933), Point::new(13.013671, 4.081553)),
        PathEl::CurveTo(Point::new(9.391472, 3.614172), Point::new(5.904508, 5.628632), Point::new(4.5, 9.0)),
        PathEl::MoveTo(Point::new(4.0, 5.0)),
        PathEl::LineTo(Point::new(4.0, 9.0)),
        PathEl::LineTo(Point::new(8.0, 9.0)),
        PathEl::MoveTo(Point::new(4.0, 13.0)),
        PathEl::CurveTo(Point::new(4.502724, 16.617463), Point::new(7.364131, 19.451067), Point::new(10.986329, 19.918447)),
        PathEl::CurveTo(Point::new(14.608528, 20.385828), Point::new(18.095492, 18.371368), Point::new(19.5, 15.0)),
        PathEl::MoveTo(Point::new(20.0, 19.0)),
        PathEl::LineTo(Point::new(20.0, 15.0)),
        PathEl::LineTo(Point::new(16.0, 15.0)),
    ],
};

/// Tabler outline `search`.
#[rustfmt::skip]
pub const SEARCH: Icon = Icon {
    name: "search",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 10.0)),
        PathEl::CurveTo(Point::new(3.0, 13.865993), Point::new(6.134007, 17.0), Point::new(10.0, 17.0)),
        PathEl::CurveTo(Point::new(13.865993, 17.0), Point::new(17.0, 13.865993), Point::new(17.0, 10.0)),
        PathEl::CurveTo(Point::new(17.0, 6.134007), Point::new(13.865993, 3.0), Point::new(10.0, 3.0)),
        PathEl::CurveTo(Point::new(6.134007, 3.0), Point::new(3.0, 6.134007), Point::new(3.0, 10.0)),
        PathEl::MoveTo(Point::new(21.0, 21.0)),
        PathEl::LineTo(Point::new(15.0, 15.0)),
    ],
};

/// Tabler outline `settings`.
#[rustfmt::skip]
pub const SETTINGS: Icon = Icon {
    name: "settings",
    els: &[
        PathEl::MoveTo(Point::new(10.325, 4.317)),
        PathEl::CurveTo(Point::new(10.751, 2.561), Point::new(13.249, 2.561), Point::new(13.675, 4.317)),
        PathEl::CurveTo(Point::new(13.804643, 4.852059), Point::new(14.182034, 5.293024), Point::new(14.690652, 5.503746)),
        PathEl::CurveTo(Point::new(15.199269, 5.714467), Point::new(15.777941, 5.669601), Point::new(16.248, 5.383)),
        PathEl::CurveTo(Point::new(17.791, 4.443), Point::new(19.558, 6.209), Point::new(18.618, 7.753)),
        PathEl::CurveTo(Point::new(18.331812, 8.222854), Point::new(18.287055, 8.801081), Point::new(18.497528, 9.309379)),
        PathEl::CurveTo(Point::new(18.708001, 9.817677), Point::new(19.148429, 10.195001), Point::new(19.683, 10.325)),
        PathEl::CurveTo(Point::new(21.439, 10.751), Point::new(21.439, 13.249), Point::new(19.683, 13.675)),
        PathEl::CurveTo(Point::new(19.147941, 13.804643), Point::new(18.706976, 14.182034), Point::new(18.496254, 14.690652)),
        PathEl::CurveTo(Point::new(18.285533, 15.199269), Point::new(18.330399, 15.777941), Point::new(18.617, 16.248)),
        PathEl::CurveTo(Point::new(19.557, 17.791), Point::new(17.791, 19.558), Point::new(16.247, 18.618)),
        PathEl::CurveTo(Point::new(15.777146, 18.331812), Point::new(15.198919, 18.287055), Point::new(14.690621, 18.497528)),
        PathEl::CurveTo(Point::new(14.182323, 18.708001), Point::new(13.804999, 19.148429), Point::new(13.675, 19.683)),
        PathEl::CurveTo(Point::new(13.249, 21.439), Point::new(10.751, 21.439), Point::new(10.325, 19.683)),
        PathEl::CurveTo(Point::new(10.195357, 19.147941), Point::new(9.817966, 18.706976), Point::new(9.309348, 18.496254)),
        PathEl::CurveTo(Point::new(8.800731, 18.285533), Point::new(8.222059, 18.330399), Point::new(7.752, 18.617)),
        PathEl::CurveTo(Point::new(6.209, 19.557), Point::new(4.442, 17.791), Point::new(5.382, 16.247)),
        PathEl::CurveTo(Point::new(5.668188, 15.777146), Point::new(5.712945, 15.198919), Point::new(5.502472, 14.690621)),
        PathEl::CurveTo(Point::new(5.291999, 14.182323), Point::new(4.851571, 13.804999), Point::new(4.317, 13.675)),
        PathEl::CurveTo(Point::new(2.561, 13.249), Point::new(2.561, 10.751), Point::new(4.317, 10.325)),
        PathEl::CurveTo(Point::new(4.852059, 10.195357), Point::new(5.293024, 9.817966), Point::new(5.503746, 9.309348)),
        PathEl::CurveTo(Point::new(5.714467, 8.800731), Point::new(5.669601, 8.222059), Point::new(5.383, 7.752)),
        PathEl::CurveTo(Point::new(4.443, 6.209), Point::new(6.209, 4.442), Point::new(7.753, 5.382)),
        PathEl::CurveTo(Point::new(8.753, 5.99), Point::new(10.049, 5.452), Point::new(10.325, 4.317)),
        PathEl::MoveTo(Point::new(9.0, 12.0)),
        PathEl::CurveTo(Point::new(9.0, 13.656854), Point::new(10.343146, 15.0), Point::new(12.0, 15.0)),
        PathEl::CurveTo(Point::new(13.656854, 15.0), Point::new(15.0, 13.656854), Point::new(15.0, 12.0)),
        PathEl::CurveTo(Point::new(15.0, 10.343146), Point::new(13.656854, 9.0), Point::new(12.0, 9.0)),
        PathEl::CurveTo(Point::new(10.343146, 9.0), Point::new(9.0, 10.343146), Point::new(9.0, 12.0)),
    ],
};

/// Tabler outline `sort-ascending`.
#[rustfmt::skip]
pub const SORT_ASCENDING: Icon = Icon {
    name: "sort-ascending",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::LineTo(Point::new(11.0, 6.0)),
        PathEl::MoveTo(Point::new(4.0, 12.0)),
        PathEl::LineTo(Point::new(11.0, 12.0)),
        PathEl::MoveTo(Point::new(4.0, 18.0)),
        PathEl::LineTo(Point::new(13.0, 18.0)),
        PathEl::MoveTo(Point::new(15.0, 9.0)),
        PathEl::LineTo(Point::new(18.0, 6.0)),
        PathEl::LineTo(Point::new(21.0, 9.0)),
        PathEl::MoveTo(Point::new(18.0, 6.0)),
        PathEl::LineTo(Point::new(18.0, 18.0)),
    ],
};

/// Tabler outline `sort-descending`.
#[rustfmt::skip]
pub const SORT_DESCENDING: Icon = Icon {
    name: "sort-descending",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::LineTo(Point::new(13.0, 6.0)),
        PathEl::MoveTo(Point::new(4.0, 12.0)),
        PathEl::LineTo(Point::new(11.0, 12.0)),
        PathEl::MoveTo(Point::new(4.0, 18.0)),
        PathEl::LineTo(Point::new(11.0, 18.0)),
        PathEl::MoveTo(Point::new(15.0, 15.0)),
        PathEl::LineTo(Point::new(18.0, 18.0)),
        PathEl::LineTo(Point::new(21.0, 15.0)),
        PathEl::MoveTo(Point::new(18.0, 6.0)),
        PathEl::LineTo(Point::new(18.0, 18.0)),
    ],
};

/// Tabler outline `table`.
#[rustfmt::skip]
pub const TABLE: Icon = Icon {
    name: "table",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 5.0)),
        PathEl::CurveTo(Point::new(3.0, 3.895431), Point::new(3.895431, 3.0), Point::new(5.0, 3.0)),
        PathEl::LineTo(Point::new(19.0, 3.0)),
        PathEl::CurveTo(Point::new(20.104569, 3.0), Point::new(21.0, 3.895431), Point::new(21.0, 5.0)),
        PathEl::LineTo(Point::new(21.0, 19.0)),
        PathEl::CurveTo(Point::new(21.0, 20.104569), Point::new(20.104569, 21.0), Point::new(19.0, 21.0)),
        PathEl::LineTo(Point::new(5.0, 21.0)),
        PathEl::CurveTo(Point::new(3.895431, 21.0), Point::new(3.0, 20.104569), Point::new(3.0, 19.0)),
        PathEl::LineTo(Point::new(3.0, 5.0)),
        PathEl::MoveTo(Point::new(3.0, 10.0)),
        PathEl::LineTo(Point::new(21.0, 10.0)),
        PathEl::MoveTo(Point::new(10.0, 3.0)),
        PathEl::LineTo(Point::new(10.0, 21.0)),
    ],
};

/// Tabler outline `upload`.
#[rustfmt::skip]
pub const UPLOAD: Icon = Icon {
    name: "upload",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 17.0)),
        PathEl::LineTo(Point::new(4.0, 19.0)),
        PathEl::CurveTo(Point::new(4.0, 20.104569), Point::new(4.895431, 21.0), Point::new(6.0, 21.0)),
        PathEl::LineTo(Point::new(18.0, 21.0)),
        PathEl::CurveTo(Point::new(19.104569, 21.0), Point::new(20.0, 20.104569), Point::new(20.0, 19.0)),
        PathEl::LineTo(Point::new(20.0, 17.0)),
        PathEl::MoveTo(Point::new(7.0, 9.0)),
        PathEl::LineTo(Point::new(12.0, 4.0)),
        PathEl::LineTo(Point::new(17.0, 9.0)),
        PathEl::MoveTo(Point::new(12.0, 4.0)),
        PathEl::LineTo(Point::new(12.0, 16.0)),
    ],
};

/// Tabler outline `x`.
#[rustfmt::skip]
pub const X: Icon = Icon {
    name: "x",
    els: &[
        PathEl::MoveTo(Point::new(18.0, 6.0)),
        PathEl::LineTo(Point::new(6.0, 18.0)),
        PathEl::MoveTo(Point::new(6.0, 6.0)),
        PathEl::LineTo(Point::new(18.0, 18.0)),
    ],
};

/// Every icon in this set, sorted by name (binary-searchable).
#[rustfmt::skip]
pub const ALL: &[Icon] = &[
    ALERT_TRIANGLE,
    ARROWS_SORT,
    CHART_AREA,
    CHART_BAR,
    CHART_DOTS,
    CHART_FUNNEL,
    CHART_HISTOGRAM,
    CHART_LINE,
    CHART_PIE,
    CHART_RADAR,
    CHART_SANKEY,
    CHART_TREEMAP,
    CHECK,
    CHEVRON_DOWN,
    CHEVRON_LEFT,
    CHEVRON_RIGHT,
    CHEVRON_UP,
    CIRCLE_CHECK,
    CIRCLE_X,
    CLOCK,
    COPY,
    DATABASE,
    DOTS_VERTICAL,
    DOWNLOAD,
    EXTERNAL_LINK,
    EYE,
    FILE,
    FILTER,
    FOLDER,
    GRIP_VERTICAL,
    INFO_CIRCLE,
    JSON,
    LAYOUT_SIDEBAR,
    LOADER_2,
    LOCK,
    MAXIMIZE,
    MINIMIZE,
    PLAYER_PAUSE,
    PLAYER_PLAY,
    PLAYER_STOP,
    REFRESH,
    SEARCH,
    SETTINGS,
    SORT_ASCENDING,
    SORT_DESCENDING,
    TABLE,
    UPLOAD,
    X,
];
