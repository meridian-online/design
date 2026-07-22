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
        PathEl::LineTo(Point::new(2.2569999999999997, 17.125)),
        PathEl::CurveTo(Point::new(1.9169613469685325, 17.713865116434476), Point::new(1.914885226729591, 18.438931187329448), Point::new(2.251546074036935, 19.029733933429316)),
        PathEl::CurveTo(Point::new(2.588206921344279, 19.620536679529184), Point::new(3.213051435197391, 19.988360619677174), Point::new(3.8930000000000002, 19.996)),
        PathEl::LineTo(Point::new(20.107, 19.996)),
        PathEl::CurveTo(Point::new(20.78663919682816, 19.98816049024656), Point::new(21.411141291453834, 19.620475187434568), Point::new(21.747738608645864, 19.02998967377862)),
        PathEl::CurveTo(Point::new(22.084335925837898, 18.439504160122674), Point::new(22.08253230458503, 17.714802776761097), Point::new(21.743, 17.126)),
        PathEl::LineTo(Point::new(13.636999999999999, 3.589999999999998)),
        PathEl::CurveTo(Point::new(13.290141525204573, 3.0174864513329194), Point::new(12.669389695875864, 2.667779713444471), Point::new(12.0, 2.667779713444471)),
        PathEl::CurveTo(Point::new(11.330610304124137, 2.667779713444471), Point::new(10.709858474795427, 3.0174864513329194), Point::new(10.363, 3.5899999999999976)),
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
        PathEl::CurveTo(Point::new(3.0, 12.447715250169207), Point::new(3.4477152501692063, 12.0), Point::new(4.0, 12.0)),
        PathEl::LineTo(Point::new(8.0, 12.0)),
        PathEl::CurveTo(Point::new(8.552284749830793, 12.0), Point::new(9.0, 12.447715250169207), Point::new(9.0, 13.0)),
        PathEl::LineTo(Point::new(9.0, 19.0)),
        PathEl::CurveTo(Point::new(9.0, 19.552284749830793), Point::new(8.552284749830793, 20.0), Point::new(8.0, 20.0)),
        PathEl::LineTo(Point::new(4.0, 20.0)),
        PathEl::CurveTo(Point::new(3.4477152501692068, 20.0), Point::new(3.0, 19.552284749830793), Point::new(3.0, 19.0)),
        PathEl::LineTo(Point::new(3.0, 13.0)),
        PathEl::MoveTo(Point::new(15.0, 9.0)),
        PathEl::CurveTo(Point::new(15.0, 8.447715250169207), Point::new(15.447715250169207, 8.0), Point::new(16.0, 8.0)),
        PathEl::LineTo(Point::new(20.0, 8.0)),
        PathEl::CurveTo(Point::new(20.552284749830793, 8.0), Point::new(21.0, 8.447715250169207), Point::new(21.0, 9.0)),
        PathEl::LineTo(Point::new(21.0, 19.0)),
        PathEl::CurveTo(Point::new(21.0, 19.552284749830793), Point::new(20.552284749830793, 20.0), Point::new(20.0, 20.0)),
        PathEl::LineTo(Point::new(16.0, 20.0)),
        PathEl::CurveTo(Point::new(15.447715250169207, 20.0), Point::new(15.0, 19.552284749830793), Point::new(15.0, 19.0)),
        PathEl::LineTo(Point::new(15.0, 9.0)),
        PathEl::MoveTo(Point::new(9.0, 5.0)),
        PathEl::CurveTo(Point::new(9.0, 4.447715250169207), Point::new(9.447715250169207, 4.0), Point::new(10.0, 4.0)),
        PathEl::LineTo(Point::new(14.0, 4.0)),
        PathEl::CurveTo(Point::new(14.552284749830793, 4.0), Point::new(15.0, 4.447715250169207), Point::new(15.0, 5.0)),
        PathEl::LineTo(Point::new(15.0, 19.0)),
        PathEl::CurveTo(Point::new(15.0, 19.552284749830793), Point::new(14.552284749830793, 20.0), Point::new(14.0, 20.0)),
        PathEl::LineTo(Point::new(10.0, 20.0)),
        PathEl::CurveTo(Point::new(9.447715250169207, 20.0), Point::new(9.0, 19.552284749830793), Point::new(9.0, 19.0)),
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
        PathEl::CurveTo(Point::new(7.0, 10.104569499661586), Point::new(7.8954305003384135, 11.0), Point::new(9.0, 11.0)),
        PathEl::CurveTo(Point::new(10.104569499661586, 11.0), Point::new(11.0, 10.104569499661586), Point::new(11.0, 9.0)),
        PathEl::CurveTo(Point::new(11.0, 7.8954305003384135), Point::new(10.104569499661586, 7.0), Point::new(9.0, 7.0)),
        PathEl::CurveTo(Point::new(7.8954305003384135, 7.0), Point::new(7.0, 7.895430500338413), Point::new(7.0, 9.0)),
        PathEl::MoveTo(Point::new(17.0, 7.0)),
        PathEl::CurveTo(Point::new(17.0, 8.104569499661586), Point::new(17.895430500338414, 9.0), Point::new(19.0, 9.0)),
        PathEl::CurveTo(Point::new(20.104569499661586, 9.0), Point::new(21.0, 8.104569499661586), Point::new(21.0, 7.0)),
        PathEl::CurveTo(Point::new(21.0, 5.8954305003384135), Point::new(20.104569499661586, 5.0), Point::new(19.0, 5.0)),
        PathEl::CurveTo(Point::new(17.895430500338414, 5.0), Point::new(17.0, 5.895430500338413), Point::new(17.0, 7.0)),
        PathEl::MoveTo(Point::new(12.0, 15.0)),
        PathEl::CurveTo(Point::new(12.0, 16.104569499661586), Point::new(12.895430500338414, 17.0), Point::new(14.0, 17.0)),
        PathEl::CurveTo(Point::new(15.104569499661586, 17.0), Point::new(16.0, 16.104569499661586), Point::new(16.0, 15.0)),
        PathEl::CurveTo(Point::new(16.0, 13.895430500338414), Point::new(15.104569499661586, 13.0), Point::new(14.0, 13.0)),
        PathEl::CurveTo(Point::new(12.895430500338414, 13.0), Point::new(12.0, 13.895430500338414), Point::new(12.0, 15.0)),
        PathEl::MoveTo(Point::new(10.16, 10.62)),
        PathEl::LineTo(Point::new(12.5, 13.5)),
        PathEl::MoveTo(Point::new(15.088, 13.328)),
        PathEl::LineTo(Point::new(17.925, 8.741999999999999)),
    ],
};

/// Tabler outline `chart-funnel`.
#[rustfmt::skip]
pub const CHART_FUNNEL: Icon = Icon {
    name: "chart-funnel",
    els: &[
        PathEl::MoveTo(Point::new(4.387, 3.0)),
        PathEl::LineTo(Point::new(19.613, 3.0)),
        PathEl::CurveTo(Point::new(19.93426721556051, 3.0002438719955182), Point::new(20.235854271423918, 3.1548257332996283), Point::new(20.423634890156467, 3.415500094324978)),
        PathEl::CurveTo(Point::new(20.611415508889014, 3.6761744553503277), Point::new(20.662520561927217, 4.011194719905421), Point::new(20.561, 4.316)),
        PathEl::LineTo(Point::new(15.456, 19.632)),
        PathEl::CurveTo(Point::new(15.183856133307454, 20.44908550872326), Point::new(14.419214822060669, 21.000207381107668), Point::new(13.558, 21.0)),
        PathEl::LineTo(Point::new(10.442, 21.0)),
        PathEl::CurveTo(Point::new(9.580785177939331, 21.000207381107668), Point::new(8.816143866692546, 20.44908550872326), Point::new(8.544, 19.632)),
        PathEl::LineTo(Point::new(3.4400000000000004, 4.316000000000001)),
        PathEl::CurveTo(Point::new(3.3385346595291354, 4.011361864463792), Point::new(3.389524955398377, 3.6765303973796284), Point::new(3.5770722771640995, 3.415904974524622)),
        PathEl::CurveTo(Point::new(3.764619598929822, 3.1552795516696164), Point::new(4.065909218352009, 3.0005646950753), Point::new(4.387, 3.000000000000001)),
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
        PathEl::CurveTo(Point::new(5.670017245941445, 4.203790646682006), Point::new(2.721258054040172, 8.218229826970713), Point::new(3.0584421563683435, 12.650233120510547)),
        PathEl::CurveTo(Point::new(3.395626258696515, 17.082236414050385), Point::new(6.917763585949622, 20.60437374130349), Point::new(11.349766879489458, 20.941557843631657)),
        PathEl::CurveTo(Point::new(15.781770173029292, 21.278741945959823), Point::new(19.796209353317998, 18.32998275405855), Point::new(20.8, 13.999999999999993)),
        PathEl::CurveTo(Point::new(20.8, 13.447715250169207), Point::new(20.352284749830794, 13.0), Point::new(19.8, 13.0)),
        PathEl::LineTo(Point::new(13.0, 13.0)),
        PathEl::CurveTo(Point::new(11.895430500338414, 13.0), Point::new(11.0, 12.104569499661586), Point::new(11.0, 11.0)),
        PathEl::LineTo(Point::new(11.0, 4.0)),
        PathEl::CurveTo(Point::new(10.97490466160835, 3.7606916225926765), Point::new(10.85502682942116, 3.5413973700516777), Point::new(10.667133704139147, 3.3910828698260675)),
        PathEl::CurveTo(Point::new(10.479240578857134, 3.240768369600457), Point::new(10.238980320532141, 3.171952149731709), Point::new(10.0, 3.1999999999999997)),
        PathEl::MoveTo(Point::new(15.0, 3.5)),
        PathEl::CurveTo(Point::new(17.57180575834304, 4.4055633875312274), Point::new(19.594436612468773, 6.428194241656962), Point::new(20.5, 9.0)),
        PathEl::LineTo(Point::new(16.0, 9.0)),
        PathEl::CurveTo(Point::new(15.447715250169207, 9.0), Point::new(15.0, 8.552284749830793), Point::new(15.0, 8.0)),
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
        PathEl::CurveTo(Point::new(10.943999999999999, 6.0), Point::new(13.056, 14.0), Point::new(20.0, 14.0)),
        PathEl::MoveTo(Point::new(4.0, 12.0)),
        PathEl::CurveTo(Point::new(10.370000000000001, 12.0), Point::new(13.63, 18.0), Point::new(20.0, 18.0)),
        PathEl::MoveTo(Point::new(20.0, 6.0)),
        PathEl::CurveTo(Point::new(12.474, 6.0), Point::new(12.094999999999999, 18.0), Point::new(4.0, 18.0)),
    ],
};

/// Tabler outline `chart-treemap`.
#[rustfmt::skip]
pub const CHART_TREEMAP: Icon = Icon {
    name: "chart-treemap",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::CurveTo(Point::new(4.0, 4.8954305003384135), Point::new(4.895430500338413, 4.0), Point::new(6.0, 4.0)),
        PathEl::LineTo(Point::new(18.0, 4.0)),
        PathEl::CurveTo(Point::new(19.104569499661586, 4.0), Point::new(20.0, 4.8954305003384135), Point::new(20.0, 6.0)),
        PathEl::LineTo(Point::new(20.0, 18.0)),
        PathEl::CurveTo(Point::new(20.0, 19.104569499661586), Point::new(19.104569499661586, 20.0), Point::new(18.0, 20.0)),
        PathEl::LineTo(Point::new(6.0, 20.0)),
        PathEl::CurveTo(Point::new(4.8954305003384135, 20.0), Point::new(4.0, 19.104569499661586), Point::new(4.0, 18.0)),
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
        PathEl::CurveTo(Point::new(3.0000000000000018, 16.970562748477143), Point::new(7.02943725152286, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970562748477143, 21.0), Point::new(21.0, 16.970562748477143), Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(21.0, 7.029437251522859), Point::new(16.970562748477143, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.02943725152286, 3.0), Point::new(3.0, 7.029437251522858), Point::new(3.0, 11.999999999999998)),
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
        PathEl::CurveTo(Point::new(3.0000000000000018, 16.970562748477143), Point::new(7.02943725152286, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970562748477143, 21.0), Point::new(21.0, 16.970562748477143), Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(21.0, 7.029437251522859), Point::new(16.970562748477143, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.02943725152286, 3.0), Point::new(3.0, 7.029437251522858), Point::new(3.0, 11.999999999999998)),
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
        PathEl::CurveTo(Point::new(3.0000000000000018, 16.970562748477143), Point::new(7.02943725152286, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970562748477143, 21.0), Point::new(21.0, 16.970562748477143), Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(21.0, 7.029437251522859), Point::new(16.970562748477143, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.02943725152286, 3.0), Point::new(3.0, 7.029437251522858), Point::new(3.0, 11.999999999999998)),
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
        PathEl::CurveTo(Point::new(7.000000000000002, 8.194056572201276), Point::new(8.194056572201275, 7.000000000000002), Point::new(9.667000000000002, 7.000000000000002)),
        PathEl::LineTo(Point::new(18.333, 7.0)),
        PathEl::CurveTo(Point::new(19.805943427798724, 7.000000000000001), Point::new(21.0, 8.194056572201275), Point::new(21.0, 9.667000000000002)),
        PathEl::LineTo(Point::new(21.0, 18.333)),
        PathEl::CurveTo(Point::new(21.0, 19.805943427798724), Point::new(19.805943427798724, 21.0), Point::new(18.333, 21.0)),
        PathEl::LineTo(Point::new(9.666999999999998, 21.0)),
        PathEl::CurveTo(Point::new(8.194056572201271, 21.0), Point::new(6.999999999999998, 19.805943427798724), Point::new(6.999999999999998, 18.333)),
        PathEl::LineTo(Point::new(6.999999999999998, 9.666999999999998)),
        PathEl::MoveTo(Point::new(4.012, 16.737)),
        PathEl::CurveTo(Point::new(3.3878011554713194, 16.381153449113793), Point::new(3.0017324930474945, 15.718503976159065), Point::new(2.999999999999999, 14.999999999999998)),
        PathEl::LineTo(Point::new(2.9999999999999996, 4.999999999999998)),
        PathEl::CurveTo(Point::new(2.9999999999999996, 3.899999999999998), Point::new(3.8999999999999995, 2.9999999999999982), Point::new(5.0, 2.9999999999999982)),
        PathEl::LineTo(Point::new(15.0, 2.9999999999999982)),
        PathEl::CurveTo(Point::new(15.75, 2.9999999999999982), Point::new(16.158, 3.384999999999998), Point::new(16.5, 3.9999999999999982)),
    ],
};

/// Tabler outline `database`.
#[rustfmt::skip]
pub const DATABASE: Icon = Icon {
    name: "database",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::CurveTo(Point::new(4.000000000000001, 7.656854249492381), Point::new(7.581722001353653, 9.0), Point::new(12.0, 9.0)),
        PathEl::CurveTo(Point::new(16.41827799864635, 9.0), Point::new(20.0, 7.656854249492381), Point::new(20.0, 6.0)),
        PathEl::CurveTo(Point::new(20.0, 4.343145750507619), Point::new(16.41827799864635, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.581722001353653, 3.0), Point::new(4.0, 4.343145750507619), Point::new(4.0, 6.0)),
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::LineTo(Point::new(4.0, 12.0)),
        PathEl::CurveTo(Point::new(4.000000000000001, 13.656854249492381), Point::new(7.581722001353653, 15.0), Point::new(12.0, 15.0)),
        PathEl::CurveTo(Point::new(16.41827799864635, 15.0), Point::new(20.0, 13.65685424949238), Point::new(20.0, 12.0)),
        PathEl::LineTo(Point::new(20.0, 6.0)),
        PathEl::MoveTo(Point::new(4.0, 12.0)),
        PathEl::LineTo(Point::new(4.0, 18.0)),
        PathEl::CurveTo(Point::new(4.000000000000001, 19.65685424949238), Point::new(7.581722001353653, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.41827799864635, 21.0), Point::new(20.0, 19.65685424949238), Point::new(20.0, 18.0)),
        PathEl::LineTo(Point::new(20.0, 12.0)),
    ],
};

/// Tabler outline `dots-vertical`.
#[rustfmt::skip]
pub const DOTS_VERTICAL: Icon = Icon {
    name: "dots-vertical",
    els: &[
        PathEl::MoveTo(Point::new(11.0, 12.0)),
        PathEl::CurveTo(Point::new(11.0, 12.552284749830793), Point::new(11.447715250169207, 13.0), Point::new(12.0, 13.0)),
        PathEl::CurveTo(Point::new(12.552284749830793, 13.0), Point::new(13.0, 12.552284749830793), Point::new(13.0, 12.0)),
        PathEl::CurveTo(Point::new(13.0, 11.447715250169207), Point::new(12.552284749830793, 11.0), Point::new(12.0, 11.0)),
        PathEl::CurveTo(Point::new(11.447715250169207, 11.0), Point::new(11.0, 11.447715250169207), Point::new(11.0, 12.0)),
        PathEl::MoveTo(Point::new(11.0, 19.0)),
        PathEl::CurveTo(Point::new(11.0, 19.552284749830793), Point::new(11.447715250169207, 20.0), Point::new(12.0, 20.0)),
        PathEl::CurveTo(Point::new(12.552284749830793, 20.0), Point::new(13.0, 19.552284749830793), Point::new(13.0, 19.0)),
        PathEl::CurveTo(Point::new(13.0, 18.447715250169207), Point::new(12.552284749830793, 18.0), Point::new(12.0, 18.0)),
        PathEl::CurveTo(Point::new(11.447715250169207, 18.0), Point::new(11.0, 18.447715250169207), Point::new(11.0, 19.0)),
        PathEl::MoveTo(Point::new(11.0, 5.0)),
        PathEl::CurveTo(Point::new(11.0, 5.552284749830793), Point::new(11.447715250169207, 6.0), Point::new(12.0, 6.0)),
        PathEl::CurveTo(Point::new(12.552284749830793, 6.0), Point::new(13.0, 5.552284749830793), Point::new(13.0, 5.0)),
        PathEl::CurveTo(Point::new(13.0, 4.447715250169207), Point::new(12.552284749830793, 4.0), Point::new(12.0, 4.0)),
        PathEl::CurveTo(Point::new(11.447715250169207, 4.0), Point::new(11.0, 4.447715250169207), Point::new(11.0, 5.0)),
    ],
};

/// Tabler outline `download`.
#[rustfmt::skip]
pub const DOWNLOAD: Icon = Icon {
    name: "download",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 17.0)),
        PathEl::LineTo(Point::new(4.0, 19.0)),
        PathEl::CurveTo(Point::new(4.0, 20.104569499661586), Point::new(4.8954305003384135, 21.0), Point::new(6.0, 21.0)),
        PathEl::LineTo(Point::new(18.0, 21.0)),
        PathEl::CurveTo(Point::new(19.104569499661586, 21.0), Point::new(20.0, 20.104569499661586), Point::new(20.0, 19.0)),
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
        PathEl::CurveTo(Point::new(4.8954305003384135, 6.0), Point::new(4.0, 6.895430500338413), Point::new(4.0, 8.0)),
        PathEl::LineTo(Point::new(4.0, 18.0)),
        PathEl::CurveTo(Point::new(4.0, 19.104569499661586), Point::new(4.8954305003384135, 20.0), Point::new(6.0, 20.0)),
        PathEl::LineTo(Point::new(16.0, 20.0)),
        PathEl::CurveTo(Point::new(17.104569499661586, 20.0), Point::new(18.0, 19.104569499661586), Point::new(18.0, 18.0)),
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
        PathEl::CurveTo(Point::new(10.0, 13.104569499661586), Point::new(10.895430500338414, 14.0), Point::new(12.0, 14.0)),
        PathEl::CurveTo(Point::new(13.104569499661586, 14.0), Point::new(14.0, 13.104569499661586), Point::new(14.0, 12.0)),
        PathEl::CurveTo(Point::new(14.0, 10.895430500338414), Point::new(13.104569499661586, 10.0), Point::new(12.0, 10.0)),
        PathEl::CurveTo(Point::new(10.895430500338414, 10.0), Point::new(10.0, 10.895430500338414), Point::new(10.0, 12.0)),
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
        PathEl::CurveTo(Point::new(14.0, 7.552284749830793), Point::new(14.447715250169207, 8.0), Point::new(15.0, 8.0)),
        PathEl::LineTo(Point::new(19.0, 8.0)),
        PathEl::MoveTo(Point::new(17.0, 21.0)),
        PathEl::LineTo(Point::new(7.0, 21.0)),
        PathEl::CurveTo(Point::new(5.8954305003384135, 21.0), Point::new(5.0, 20.104569499661586), Point::new(5.0, 19.0)),
        PathEl::LineTo(Point::new(5.0, 5.0)),
        PathEl::CurveTo(Point::new(5.0, 3.8954305003384135), Point::new(5.895430500338413, 3.0), Point::new(7.0, 3.0)),
        PathEl::LineTo(Point::new(14.0, 3.0)),
        PathEl::LineTo(Point::new(19.0, 8.0)),
        PathEl::LineTo(Point::new(19.0, 19.0)),
        PathEl::CurveTo(Point::new(19.0, 20.104569499661586), Point::new(18.104569499661586, 21.0), Point::new(17.0, 21.0)),
    ],
};

/// Tabler outline `filter`.
#[rustfmt::skip]
pub const FILTER: Icon = Icon {
    name: "filter",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 4.0)),
        PathEl::LineTo(Point::new(20.0, 4.0)),
        PathEl::LineTo(Point::new(20.0, 6.172000000000001)),
        PathEl::CurveTo(Point::new(19.999886725189008, 6.702389626957629), Point::new(19.789098749621903, 7.211014537285084), Point::new(19.414, 7.586)),
        PathEl::LineTo(Point::new(15.000000000000002, 12.0)),
        PathEl::LineTo(Point::new(15.000000000000002, 19.0)),
        PathEl::LineTo(Point::new(9.000000000000002, 21.0)),
        PathEl::LineTo(Point::new(9.000000000000002, 12.5)),
        PathEl::LineTo(Point::new(4.520000000000001, 7.572)),
        PathEl::CurveTo(Point::new(4.18544742213182, 7.203926293246256), Point::new(4.000053159351604, 6.72439690201666), Point::new(4.000000000000002, 6.227)),
        PathEl::LineTo(Point::new(4.000000000000002, 4.0)),
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
        PathEl::CurveTo(Point::new(20.104569499661586, 7.0), Point::new(21.0, 7.8954305003384135), Point::new(21.0, 9.0)),
        PathEl::LineTo(Point::new(21.0, 17.0)),
        PathEl::CurveTo(Point::new(21.0, 18.104569499661586), Point::new(20.104569499661586, 19.0), Point::new(19.0, 19.0)),
        PathEl::LineTo(Point::new(5.0, 19.0)),
        PathEl::CurveTo(Point::new(3.8954305003384135, 19.0), Point::new(3.0, 18.104569499661586), Point::new(3.0, 17.0)),
        PathEl::LineTo(Point::new(3.0, 6.0)),
        PathEl::CurveTo(Point::new(3.0, 4.8954305003384135), Point::new(3.8954305003384126, 4.0), Point::new(5.0, 4.0)),
    ],
};

/// Tabler outline `grip-vertical`.
#[rustfmt::skip]
pub const GRIP_VERTICAL: Icon = Icon {
    name: "grip-vertical",
    els: &[
        PathEl::MoveTo(Point::new(8.0, 5.0)),
        PathEl::CurveTo(Point::new(8.0, 5.552284749830793), Point::new(8.447715250169207, 6.0), Point::new(9.0, 6.0)),
        PathEl::CurveTo(Point::new(9.552284749830793, 6.0), Point::new(10.0, 5.552284749830793), Point::new(10.0, 5.0)),
        PathEl::CurveTo(Point::new(10.0, 4.447715250169207), Point::new(9.552284749830793, 4.0), Point::new(9.0, 4.0)),
        PathEl::CurveTo(Point::new(8.447715250169207, 4.0), Point::new(8.0, 4.447715250169207), Point::new(8.0, 5.0)),
        PathEl::MoveTo(Point::new(8.0, 12.0)),
        PathEl::CurveTo(Point::new(8.0, 12.552284749830793), Point::new(8.447715250169207, 13.0), Point::new(9.0, 13.0)),
        PathEl::CurveTo(Point::new(9.552284749830793, 13.0), Point::new(10.0, 12.552284749830793), Point::new(10.0, 12.0)),
        PathEl::CurveTo(Point::new(10.0, 11.447715250169207), Point::new(9.552284749830793, 11.0), Point::new(9.0, 11.0)),
        PathEl::CurveTo(Point::new(8.447715250169207, 11.0), Point::new(8.0, 11.447715250169207), Point::new(8.0, 12.0)),
        PathEl::MoveTo(Point::new(8.0, 19.0)),
        PathEl::CurveTo(Point::new(8.0, 19.552284749830793), Point::new(8.447715250169207, 20.0), Point::new(9.0, 20.0)),
        PathEl::CurveTo(Point::new(9.552284749830793, 20.0), Point::new(10.0, 19.552284749830793), Point::new(10.0, 19.0)),
        PathEl::CurveTo(Point::new(10.0, 18.447715250169207), Point::new(9.552284749830793, 18.0), Point::new(9.0, 18.0)),
        PathEl::CurveTo(Point::new(8.447715250169207, 18.0), Point::new(8.0, 18.447715250169207), Point::new(8.0, 19.0)),
        PathEl::MoveTo(Point::new(14.0, 5.0)),
        PathEl::CurveTo(Point::new(14.0, 5.552284749830793), Point::new(14.447715250169207, 6.0), Point::new(15.0, 6.0)),
        PathEl::CurveTo(Point::new(15.552284749830793, 6.0), Point::new(16.0, 5.552284749830793), Point::new(16.0, 5.0)),
        PathEl::CurveTo(Point::new(16.0, 4.447715250169207), Point::new(15.552284749830793, 4.0), Point::new(15.0, 4.0)),
        PathEl::CurveTo(Point::new(14.447715250169207, 4.0), Point::new(14.0, 4.447715250169207), Point::new(14.0, 5.0)),
        PathEl::MoveTo(Point::new(14.0, 12.0)),
        PathEl::CurveTo(Point::new(14.0, 12.552284749830793), Point::new(14.447715250169207, 13.0), Point::new(15.0, 13.0)),
        PathEl::CurveTo(Point::new(15.552284749830793, 13.0), Point::new(16.0, 12.552284749830793), Point::new(16.0, 12.0)),
        PathEl::CurveTo(Point::new(16.0, 11.447715250169207), Point::new(15.552284749830793, 11.0), Point::new(15.0, 11.0)),
        PathEl::CurveTo(Point::new(14.447715250169207, 11.0), Point::new(14.0, 11.447715250169207), Point::new(14.0, 12.0)),
        PathEl::MoveTo(Point::new(14.0, 19.0)),
        PathEl::CurveTo(Point::new(14.0, 19.552284749830793), Point::new(14.447715250169207, 20.0), Point::new(15.0, 20.0)),
        PathEl::CurveTo(Point::new(15.552284749830793, 20.0), Point::new(16.0, 19.552284749830793), Point::new(16.0, 19.0)),
        PathEl::CurveTo(Point::new(16.0, 18.447715250169207), Point::new(15.552284749830793, 18.0), Point::new(15.0, 18.0)),
        PathEl::CurveTo(Point::new(14.447715250169207, 18.0), Point::new(14.0, 18.447715250169207), Point::new(14.0, 19.0)),
    ],
};

/// Tabler outline `info-circle`.
#[rustfmt::skip]
pub const INFO_CIRCLE: Icon = Icon {
    name: "info-circle",
    els: &[
        PathEl::MoveTo(Point::new(3.0, 12.0)),
        PathEl::CurveTo(Point::new(3.0000000000000018, 16.970562748477143), Point::new(7.02943725152286, 21.0), Point::new(12.0, 21.0)),
        PathEl::CurveTo(Point::new(16.970562748477143, 21.0), Point::new(21.0, 16.970562748477143), Point::new(21.0, 12.0)),
        PathEl::CurveTo(Point::new(21.0, 7.029437251522859), Point::new(16.970562748477143, 3.0), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(7.02943725152286, 3.0), Point::new(3.0, 7.029437251522858), Point::new(3.0, 11.999999999999998)),
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
        PathEl::CurveTo(Point::new(16.104569499661586, 8.0), Point::new(17.0, 8.895430500338414), Point::new(17.0, 10.0)),
        PathEl::LineTo(Point::new(17.0, 14.0)),
        PathEl::CurveTo(Point::new(17.0, 15.104569499661586), Point::new(16.104569499661586, 16.0), Point::new(15.0, 16.0)),
        PathEl::CurveTo(Point::new(13.895430500338414, 16.0), Point::new(13.0, 15.104569499661586), Point::new(13.0, 14.0)),
        PathEl::LineTo(Point::new(13.0, 10.0)),
        PathEl::CurveTo(Point::new(13.0, 8.895430500338414), Point::new(13.895430500338414, 8.0), Point::new(15.0, 8.0)),
        PathEl::MoveTo(Point::new(1.0, 8.0)),
        PathEl::LineTo(Point::new(4.0, 8.0)),
        PathEl::LineTo(Point::new(4.0, 14.5)),
        PathEl::CurveTo(Point::new(4.0, 15.32842712474619), Point::new(3.3284271247461903, 16.0), Point::new(2.5, 16.0)),
        PathEl::CurveTo(Point::new(1.67157287525381, 16.0), Point::new(1.0000000000000002, 15.32842712474619), Point::new(1.0, 14.5)),
        PathEl::LineTo(Point::new(1.0, 14.0)),
        PathEl::MoveTo(Point::new(7.0, 15.0)),
        PathEl::CurveTo(Point::new(7.0, 15.552284749830793), Point::new(7.447715250169207, 16.0), Point::new(8.0, 16.0)),
        PathEl::LineTo(Point::new(9.0, 16.0)),
        PathEl::CurveTo(Point::new(9.552284749830793, 16.0), Point::new(10.0, 15.552284749830793), Point::new(10.0, 15.0)),
        PathEl::LineTo(Point::new(10.0, 13.0)),
        PathEl::CurveTo(Point::new(10.0, 12.447715250169207), Point::new(9.552284749830793, 12.0), Point::new(9.0, 12.0)),
        PathEl::LineTo(Point::new(8.0, 12.0)),
        PathEl::CurveTo(Point::new(7.447715250169207, 12.0), Point::new(7.0, 11.552284749830793), Point::new(7.0, 11.0)),
        PathEl::LineTo(Point::new(7.0, 9.0)),
        PathEl::CurveTo(Point::new(7.0, 8.447715250169207), Point::new(7.447715250169207, 8.0), Point::new(8.0, 8.0)),
        PathEl::LineTo(Point::new(9.0, 8.0)),
        PathEl::CurveTo(Point::new(9.552284749830793, 8.0), Point::new(10.0, 8.447715250169207), Point::new(10.0, 9.0)),
    ],
};

/// Tabler outline `layout-sidebar`.
#[rustfmt::skip]
pub const LAYOUT_SIDEBAR: Icon = Icon {
    name: "layout-sidebar",
    els: &[
        PathEl::MoveTo(Point::new(4.0, 6.0)),
        PathEl::CurveTo(Point::new(4.0, 4.8954305003384135), Point::new(4.895430500338413, 4.0), Point::new(6.0, 4.0)),
        PathEl::LineTo(Point::new(18.0, 4.0)),
        PathEl::CurveTo(Point::new(19.104569499661586, 4.0), Point::new(20.0, 4.8954305003384135), Point::new(20.0, 6.0)),
        PathEl::LineTo(Point::new(20.0, 18.0)),
        PathEl::CurveTo(Point::new(20.0, 19.104569499661586), Point::new(19.104569499661586, 20.0), Point::new(18.0, 20.0)),
        PathEl::LineTo(Point::new(6.0, 20.0)),
        PathEl::CurveTo(Point::new(4.8954305003384135, 20.0), Point::new(4.0, 19.104569499661586), Point::new(4.0, 18.0)),
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
        PathEl::CurveTo(Point::new(7.02943725152286, 3.0), Point::new(3.0, 7.029437251522858), Point::new(3.0, 11.999999999999998)),
        PathEl::CurveTo(Point::new(3.0, 16.97056274847714), Point::new(7.029437251522857, 21.0), Point::new(11.999999999999998, 21.0)),
        PathEl::CurveTo(Point::new(16.97056274847714, 21.0), Point::new(21.0, 16.970562748477143), Point::new(21.0, 12.000000000000002)),
    ],
};

/// Tabler outline `lock`.
#[rustfmt::skip]
pub const LOCK: Icon = Icon {
    name: "lock",
    els: &[
        PathEl::MoveTo(Point::new(5.0, 13.0)),
        PathEl::CurveTo(Point::new(5.0, 11.895430500338414), Point::new(5.895430500338413, 11.0), Point::new(7.0, 11.0)),
        PathEl::LineTo(Point::new(17.0, 11.0)),
        PathEl::CurveTo(Point::new(18.104569499661586, 11.0), Point::new(19.0, 11.895430500338414), Point::new(19.0, 13.0)),
        PathEl::LineTo(Point::new(19.0, 19.0)),
        PathEl::CurveTo(Point::new(19.0, 20.104569499661586), Point::new(18.104569499661586, 21.0), Point::new(17.0, 21.0)),
        PathEl::LineTo(Point::new(7.0, 21.0)),
        PathEl::CurveTo(Point::new(5.8954305003384135, 21.0), Point::new(5.0, 20.104569499661586), Point::new(5.0, 19.0)),
        PathEl::LineTo(Point::new(5.0, 13.0)),
        PathEl::MoveTo(Point::new(11.0, 16.0)),
        PathEl::CurveTo(Point::new(11.0, 16.552284749830793), Point::new(11.447715250169207, 17.0), Point::new(12.0, 17.0)),
        PathEl::CurveTo(Point::new(12.552284749830793, 17.0), Point::new(13.0, 16.552284749830793), Point::new(13.0, 16.0)),
        PathEl::CurveTo(Point::new(13.0, 15.447715250169207), Point::new(12.552284749830793, 15.0), Point::new(12.0, 15.0)),
        PathEl::CurveTo(Point::new(11.447715250169207, 15.0), Point::new(11.0, 15.447715250169207), Point::new(11.0, 16.0)),
        PathEl::MoveTo(Point::new(8.0, 11.0)),
        PathEl::LineTo(Point::new(8.0, 7.0)),
        PathEl::CurveTo(Point::new(8.0, 4.790861000676827), Point::new(9.790861000676825, 3.0000000000000004), Point::new(12.0, 3.0)),
        PathEl::CurveTo(Point::new(14.209138999323173, 2.999999999999999), Point::new(16.0, 4.790861000676825), Point::new(16.0, 6.999999999999999)),
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
        PathEl::CurveTo(Point::new(4.0, 4.8954305003384135), Point::new(4.895430500338413, 4.0), Point::new(6.0, 4.0)),
        PathEl::LineTo(Point::new(8.0, 4.0)),
        PathEl::MoveTo(Point::new(4.0, 16.0)),
        PathEl::LineTo(Point::new(4.0, 18.0)),
        PathEl::CurveTo(Point::new(4.0, 19.104569499661586), Point::new(4.8954305003384135, 20.0), Point::new(6.0, 20.0)),
        PathEl::LineTo(Point::new(8.0, 20.0)),
        PathEl::MoveTo(Point::new(16.0, 4.0)),
        PathEl::LineTo(Point::new(18.0, 4.0)),
        PathEl::CurveTo(Point::new(19.104569499661586, 4.0), Point::new(20.0, 4.8954305003384135), Point::new(20.0, 6.0)),
        PathEl::LineTo(Point::new(20.0, 8.0)),
        PathEl::MoveTo(Point::new(16.0, 20.0)),
        PathEl::LineTo(Point::new(18.0, 20.0)),
        PathEl::CurveTo(Point::new(19.104569499661586, 20.0), Point::new(20.0, 19.104569499661586), Point::new(20.0, 18.0)),
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
        PathEl::CurveTo(Point::new(15.0, 15.895430500338414), Point::new(15.895430500338414, 15.0), Point::new(17.0, 15.0)),
        PathEl::LineTo(Point::new(19.0, 15.0)),
        PathEl::MoveTo(Point::new(15.0, 5.0)),
        PathEl::LineTo(Point::new(15.0, 7.0)),
        PathEl::CurveTo(Point::new(15.0, 8.104569499661586), Point::new(15.895430500338414, 9.0), Point::new(17.0, 9.0)),
        PathEl::LineTo(Point::new(19.0, 9.0)),
        PathEl::MoveTo(Point::new(5.0, 15.0)),
        PathEl::LineTo(Point::new(7.0, 15.0)),
        PathEl::CurveTo(Point::new(8.104569499661586, 15.0), Point::new(9.0, 15.895430500338414), Point::new(9.0, 17.0)),
        PathEl::LineTo(Point::new(9.0, 19.0)),
        PathEl::MoveTo(Point::new(5.0, 9.0)),
        PathEl::LineTo(Point::new(7.0, 9.0)),
        PathEl::CurveTo(Point::new(8.104569499661586, 9.0), Point::new(9.0, 8.104569499661586), Point::new(9.0, 7.0)),
        PathEl::LineTo(Point::new(9.0, 5.0)),
    ],
};

/// Tabler outline `player-pause`.
#[rustfmt::skip]
pub const PLAYER_PAUSE: Icon = Icon {
    name: "player-pause",
    els: &[
        PathEl::MoveTo(Point::new(6.0, 6.0)),
        PathEl::CurveTo(Point::new(6.0, 5.447715250169207), Point::new(6.447715250169207, 5.0), Point::new(7.0, 5.0)),
        PathEl::LineTo(Point::new(9.0, 5.0)),
        PathEl::CurveTo(Point::new(9.552284749830793, 5.0), Point::new(10.0, 5.447715250169207), Point::new(10.0, 6.0)),
        PathEl::LineTo(Point::new(10.0, 18.0)),
        PathEl::CurveTo(Point::new(10.0, 18.552284749830793), Point::new(9.552284749830793, 19.0), Point::new(9.0, 19.0)),
        PathEl::LineTo(Point::new(7.0, 19.0)),
        PathEl::CurveTo(Point::new(6.447715250169207, 19.0), Point::new(6.0, 18.552284749830793), Point::new(6.0, 18.0)),
        PathEl::LineTo(Point::new(6.0, 6.0)),
        PathEl::MoveTo(Point::new(14.0, 6.0)),
        PathEl::CurveTo(Point::new(14.0, 5.447715250169207), Point::new(14.447715250169207, 5.0), Point::new(15.0, 5.0)),
        PathEl::LineTo(Point::new(17.0, 5.0)),
        PathEl::CurveTo(Point::new(17.552284749830793, 5.0), Point::new(18.0, 5.447715250169207), Point::new(18.0, 6.0)),
        PathEl::LineTo(Point::new(18.0, 18.0)),
        PathEl::CurveTo(Point::new(18.0, 18.552284749830793), Point::new(17.552284749830793, 19.0), Point::new(17.0, 19.0)),
        PathEl::LineTo(Point::new(15.0, 19.0)),
        PathEl::CurveTo(Point::new(14.447715250169207, 19.0), Point::new(14.0, 18.552284749830793), Point::new(14.0, 18.0)),
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
        PathEl::CurveTo(Point::new(5.0, 5.8954305003384135), Point::new(5.895430500338413, 5.0), Point::new(7.0, 5.0)),
        PathEl::LineTo(Point::new(17.0, 5.0)),
        PathEl::CurveTo(Point::new(18.104569499661586, 5.0), Point::new(19.0, 5.8954305003384135), Point::new(19.0, 7.0)),
        PathEl::LineTo(Point::new(19.0, 17.0)),
        PathEl::CurveTo(Point::new(19.0, 18.104569499661586), Point::new(18.104569499661586, 19.0), Point::new(17.0, 19.0)),
        PathEl::LineTo(Point::new(7.0, 19.0)),
        PathEl::CurveTo(Point::new(5.8954305003384135, 19.0), Point::new(5.0, 18.104569499661586), Point::new(5.0, 17.0)),
        PathEl::LineTo(Point::new(5.0, 7.0)),
    ],
};

/// Tabler outline `refresh`.
#[rustfmt::skip]
pub const REFRESH: Icon = Icon {
    name: "refresh",
    els: &[
        PathEl::MoveTo(Point::new(20.0, 11.0)),
        PathEl::CurveTo(Point::new(19.497276164480365, 7.382537335257956), Point::new(16.63586927077693, 4.548932968344975), Point::new(13.013670645094836, 4.081552500515027)),
        PathEl::CurveTo(Point::new(9.391472019412742, 3.6141720326850795), Point::new(5.9045084600021305, 5.628631825002695), Point::new(4.500000000000001, 8.999999999999996)),
        PathEl::MoveTo(Point::new(4.0, 5.0)),
        PathEl::LineTo(Point::new(4.0, 9.0)),
        PathEl::LineTo(Point::new(8.0, 9.0)),
        PathEl::MoveTo(Point::new(4.0, 13.0)),
        PathEl::CurveTo(Point::new(4.502723835519634, 16.617462664742046), Point::new(7.364130729223071, 19.451067031655025), Point::new(10.986329354905166, 19.918447499484973)),
        PathEl::CurveTo(Point::new(14.608527980587262, 20.385827967314917), Point::new(18.095491539997873, 18.3713681749973), Point::new(19.5, 15.0)),
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
        PathEl::CurveTo(Point::new(3.000000000000001, 13.865993248815556), Point::new(6.134006751184446, 17.0), Point::new(10.0, 17.0)),
        PathEl::CurveTo(Point::new(13.865993248815554, 17.0), Point::new(17.0, 13.865993248815554), Point::new(17.0, 10.0)),
        PathEl::CurveTo(Point::new(17.0, 6.134006751184446), Point::new(13.865993248815554, 3.0), Point::new(10.0, 3.0)),
        PathEl::CurveTo(Point::new(6.134006751184446, 3.0), Point::new(3.0, 6.134006751184445), Point::new(3.0, 10.0)),
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
        PathEl::CurveTo(Point::new(10.751, 2.561), Point::new(13.248999999999999, 2.561), Point::new(13.674999999999999, 4.317)),
        PathEl::CurveTo(Point::new(13.804643040345798, 4.852059202645093), Point::new(14.182033738711, 5.293024242378385), Point::new(14.690651560332133, 5.503745811693631)),
        PathEl::CurveTo(Point::new(15.199269381953266, 5.714467381008877), Point::new(15.777941082169054, 5.669601259615009), Point::new(16.247999999999998, 5.383)),
        PathEl::CurveTo(Point::new(17.790999999999997, 4.443), Point::new(19.557999999999996, 6.209), Point::new(18.618, 7.753)),
        PathEl::CurveTo(Point::new(18.331812471358425, 8.222853901224077), Point::new(18.28705461054723, 8.80108106823386), Point::new(18.497527967466095, 9.309379165882042)),
        PathEl::CurveTo(Point::new(18.70800132438496, 9.817677263530225), Point::new(19.148428792169092, 10.195001485379033), Point::new(19.683000000000003, 10.325)),
        PathEl::CurveTo(Point::new(21.439, 10.751), Point::new(21.439, 13.248999999999999), Point::new(19.683, 13.674999999999999)),
        PathEl::CurveTo(Point::new(19.147940797354906, 13.804643040345798), Point::new(18.706975757621613, 14.182033738711002), Point::new(18.49625418830637, 14.690651560332133)),
        PathEl::CurveTo(Point::new(18.285532618991123, 15.199269381953266), Point::new(18.33039874038499, 15.777941082169054), Point::new(18.617, 16.247999999999998)),
        PathEl::CurveTo(Point::new(19.557000000000002, 17.790999999999997), Point::new(17.791, 19.557999999999996), Point::new(16.247, 18.618)),
        PathEl::CurveTo(Point::new(15.777146098775923, 18.331812471358425), Point::new(15.19891893176614, 18.28705461054723), Point::new(14.690620834117958, 18.497527967466095)),
        PathEl::CurveTo(Point::new(14.182322736469775, 18.70800132438496), Point::new(13.804998514620967, 19.148428792169092), Point::new(13.675, 19.683)),
        PathEl::CurveTo(Point::new(13.249, 21.439), Point::new(10.751000000000001, 21.439), Point::new(10.325000000000001, 19.683)),
        PathEl::CurveTo(Point::new(10.1953569596542, 19.147940797354906), Point::new(9.817966261288996, 18.706975757621613), Point::new(9.309348439667863, 18.496254188306366)),
        PathEl::CurveTo(Point::new(8.80073061804673, 18.285532618991123), Point::new(8.222058917830942, 18.33039874038499), Point::new(7.752000000000001, 18.617)),
        PathEl::CurveTo(Point::new(6.2090000000000005, 19.557000000000002), Point::new(4.442, 17.791), Point::new(5.382000000000001, 16.247)),
        PathEl::CurveTo(Point::new(5.668187528641575, 15.777146098775923), Point::new(5.71294538945277, 15.19891893176614), Point::new(5.502472032533907, 14.690620834117958)),
        PathEl::CurveTo(Point::new(5.291998675615044, 14.182322736469777), Point::new(4.851571207830911, 13.804998514620967), Point::new(4.317, 13.675)),
        PathEl::CurveTo(Point::new(2.561, 13.249), Point::new(2.561, 10.751000000000001), Point::new(4.317, 10.325000000000001)),
        PathEl::CurveTo(Point::new(4.852059202645094, 10.195356959654202), Point::new(5.293024242378385, 9.817966261288998), Point::new(5.503745811693632, 9.309348439667865)),
        PathEl::CurveTo(Point::new(5.714467381008877, 8.800730618046732), Point::new(5.669601259615009, 8.222058917830942), Point::new(5.382999999999999, 7.752000000000001)),
        PathEl::CurveTo(Point::new(4.443, 6.2090000000000005), Point::new(6.209, 4.442), Point::new(7.753, 5.382000000000001)),
        PathEl::CurveTo(Point::new(8.753, 5.99), Point::new(10.049, 5.452000000000001), Point::new(10.325, 4.317)),
        PathEl::MoveTo(Point::new(9.0, 12.0)),
        PathEl::CurveTo(Point::new(9.0, 13.656854249492381), Point::new(10.34314575050762, 15.0), Point::new(12.0, 15.0)),
        PathEl::CurveTo(Point::new(13.65685424949238, 15.0), Point::new(15.0, 13.65685424949238), Point::new(15.0, 12.0)),
        PathEl::CurveTo(Point::new(15.0, 10.34314575050762), Point::new(13.65685424949238, 9.0), Point::new(12.0, 9.0)),
        PathEl::CurveTo(Point::new(10.34314575050762, 9.0), Point::new(9.0, 10.343145750507619), Point::new(9.0, 12.0)),
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
        PathEl::CurveTo(Point::new(3.0, 3.8954305003384135), Point::new(3.8954305003384126, 3.0), Point::new(5.0, 3.0)),
        PathEl::LineTo(Point::new(19.0, 3.0)),
        PathEl::CurveTo(Point::new(20.104569499661586, 3.0), Point::new(21.0, 3.895430500338413), Point::new(21.0, 5.0)),
        PathEl::LineTo(Point::new(21.0, 19.0)),
        PathEl::CurveTo(Point::new(21.0, 20.104569499661586), Point::new(20.104569499661586, 21.0), Point::new(19.0, 21.0)),
        PathEl::LineTo(Point::new(5.0, 21.0)),
        PathEl::CurveTo(Point::new(3.8954305003384135, 21.0), Point::new(3.0, 20.104569499661586), Point::new(3.0, 19.0)),
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
        PathEl::CurveTo(Point::new(4.0, 20.104569499661586), Point::new(4.8954305003384135, 21.0), Point::new(6.0, 21.0)),
        PathEl::LineTo(Point::new(18.0, 21.0)),
        PathEl::CurveTo(Point::new(19.104569499661586, 21.0), Point::new(20.0, 20.104569499661586), Point::new(20.0, 19.0)),
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
