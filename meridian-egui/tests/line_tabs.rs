//! The line-tab strip, read out of the paint list and the accessibility tree.
//!
//! A strip is the words in a row over one rule, and the open one is marked by a
//! bar under it. Every claim below is made of what a frame painted or what the
//! tree carries, because a token can read `2` while the painter draws a `1`, a
//! click can be reported for the tab that is already open, and a word can be
//! described to a screen reader as a button whatever it looks like — none of
//! those is visible from the tokens.
//!
//! The expectations are laid out from the design crate's named constants and
//! inks, never asked of the code under test, and the literal widths the look
//! states (1 and 2) are held beside the constants so a token drifting cannot
//! take the claim with it.
//!
//! CPU tessellation only — green on a runner with no GPU, like its siblings.

use egui::accesskit::Role as A11yRole;
use egui::epaint::RectShape;
use egui::{Color32, CornerRadius, Rect, Stroke};
use egui_kittest::kittest::{NodeT, Queryable};
use egui_kittest::Harness;
use meridian_design::colour::Rgba;
use meridian_design::control::TAB_BAR_WIDTH;
use meridian_design::semantic::semantic;
use meridian_egui::{line_tabs, theme, LineTabs, LineTabsResponse, Mode, TOKENS};

const EPS: f32 = 0.01;

const MODES: [Mode; 2] = [Mode::Light, Mode::Dark];

/// Three names, so "the second is open" has a first and a third beside it.
const NAMES: [&str; 3] = ["columns", "settings", "set"];

fn ink(t: Rgba) -> Color32 {
    theme::to_color32(t)
}

fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

fn near_rect(a: Rect, b: Rect) -> bool {
    near(a.min.x, b.min.x)
        && near(a.min.y, b.min.y)
        && near(a.max.x, b.max.x)
        && near(a.max.y, b.max.y)
}

// ─── driving a strip ────────────────────────────────────────────────────────

/// What the harness holds between frames: what the caller passes in, and what
/// the strip reported.
struct Frame {
    open: usize,
    bar: Option<Color32>,
    strip: Rect,
    clicks: Vec<usize>,
}

/// A strip of [`NAMES`] drawn and run once. `open` is the index the caller
/// passes; `bar` is the ink it passes, if any.
fn drawn(mode: Mode, open: usize, bar: Option<Color32>) -> Harness<'static, Frame> {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 120.0))
        .build_ui_state(
            move |ui, f: &mut Frame| {
                theme::apply(ui.ctx(), mode);
                let mut tabs = LineTabs::new(&NAMES, f.open);
                if let Some(ink) = f.bar {
                    tabs = tabs.bar_ink(ink);
                }
                let out: LineTabsResponse = line_tabs(ui, tabs);
                f.strip = out.response.rect;
                if let Some(i) = out.clicked {
                    f.clicks.push(i);
                }
            },
            Frame {
                open,
                bar,
                strip: Rect::NOTHING,
                clicks: Vec::new(),
            },
        );
    harness.run();
    harness
}

// ─── reading the paint list ─────────────────────────────────────────────────

/// One rect the frame painted, with everything the look is stated in.
#[derive(Clone, Copy, Debug)]
struct Painted {
    rect: Rect,
    fill: Color32,
    stroke: Stroke,
    radius: CornerRadius,
    blur: f32,
}

impl Painted {
    fn from(r: &RectShape) -> Self {
        Self {
            rect: r.rect,
            fill: r.fill,
            stroke: r.stroke,
            radius: r.corner_radius,
            blur: r.blur_width,
        }
    }

    fn is_stroked(&self) -> bool {
        self.stroke.width > 0.0 && self.stroke.color != Color32::TRANSPARENT
    }
}

/// One word the frame painted: its text, the ink it was drawn in and where its
/// layout box starts.
#[derive(Clone, Debug)]
struct Word {
    text: String,
    ink: Color32,
    left: f32,
}

/// Every rect and every word a frame painted, in paint order, less the
/// harness's own page — the one fill in `mode` that is the page's and not the
/// strip's.
fn read(harness: &Harness<'_, Frame>, mode: Mode) -> (Vec<Painted>, Vec<Word>) {
    fn walk(shape: &egui::Shape, rects: &mut Vec<Painted>, words: &mut Vec<Word>) {
        match shape {
            egui::Shape::Rect(r) => rects.push(Painted::from(r)),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| walk(s, rects, words)),
            egui::Shape::Text(t) => {
                // The ink the tessellator draws the section in: the override if
                // there is one, the fallback where the layout left the colour to
                // the painter, and the section's own otherwise.
                let own = t.galley.job.sections[0].format.color;
                let drawn = t
                    .override_text_color
                    .unwrap_or(if own == Color32::PLACEHOLDER {
                        t.fallback_color
                    } else {
                        own
                    });
                words.push(Word {
                    text: t.galley.text().to_owned(),
                    ink: drawn,
                    left: t.pos.x,
                });
            }
            _ => {}
        }
    }
    let (mut rects, mut words) = (Vec::new(), Vec::new());
    for clipped in &harness.output().shapes {
        walk(&clipped.shape, &mut rects, &mut words);
    }
    let page = theme::meridian_visuals(mode).panel_fill;
    rects.retain(|r| r.fill != page);
    (rects, words)
}

fn filled(rects: &[Painted], fill: Color32) -> Vec<Painted> {
    rects.iter().filter(|r| r.fill == fill).copied().collect()
}

// ─── reading the tree ───────────────────────────────────────────────────────

/// The rect of each tab, in the order the names were passed.
fn tab_rects(harness: &Harness<'_, Frame>) -> Vec<Rect> {
    NAMES
        .iter()
        .map(|name| harness.get_by_role_and_label(A11yRole::Tab, name).rect())
        .collect()
}

// ─── the look ───────────────────────────────────────────────────────────────

#[test]
fn three_names_with_the_second_open_are_words_over_one_rule_and_the_second_has_a_bar() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let (divider, bar_ink, primary, secondary) = (
            ink(sem.borders.divider),
            ink(sem.tabs.active_bar),
            ink(sem.text.primary),
            ink(sem.text.secondary),
        );
        let harness = drawn(mode, 1, None);
        let strip = harness.state().strip;
        let (rects, words) = read(&harness, mode);
        let tabs = tab_rects(&harness);

        // One rule, 1px, in the divider ink, along the bottom of the strip and
        // across all of it.
        let rules = filled(&rects, divider);
        assert_eq!(rules.len(), 1, "{mode:?}: one rule; painted {rects:?}");
        let rule = Rect::from_min_max(egui::pos2(strip.left(), strip.bottom() - 1.0), strip.max);
        assert!(
            near_rect(rules[0].rect, rule),
            "{mode:?}: the rule is {:?}, not {rule:?}",
            rules[0].rect
        );

        // One bar, TAB_BAR_WIDTH (2px) tall, in the tab bar's ink, on the foot
        // of the strip and exactly as wide as the second tab.
        let bars = filled(&rects, bar_ink);
        assert_eq!(bars.len(), 1, "{mode:?}: one bar; painted {rects:?}");
        let bar = bars[0].rect;
        assert!(
            near(bar.height(), TAB_BAR_WIDTH) && near(bar.height(), 2.0),
            "{mode:?}: the bar is {} tall",
            bar.height()
        );
        assert!(
            near(bar.bottom(), strip.bottom()),
            "{mode:?}: the bar is not on the foot of the strip"
        );
        assert!(
            near(bar.left(), tabs[1].left()) && near(bar.right(), tabs[1].right()),
            "{mode:?}: the bar spans {}..{} and the second tab {}..{}",
            bar.left(),
            bar.right(),
            tabs[1].left(),
            tabs[1].right()
        );

        // The bar goes down over the rule.
        let at = |p: &Painted| {
            rects
                .iter()
                .position(|r| r.rect == p.rect && r.fill == p.fill)
        };
        assert!(
            at(&rules[0]).unwrap() < at(&bars[0]).unwrap(),
            "{mode:?}: the rule covers the bar"
        );

        // Three words in a row, in the order they were passed: the open one in
        // the primary ink and the others in the secondary.
        let drawn_words: Vec<(&str, Color32)> =
            words.iter().map(|w| (w.text.as_str(), w.ink)).collect();
        assert_eq!(
            drawn_words,
            vec![
                (NAMES[0], secondary),
                (NAMES[1], primary),
                (NAMES[2], secondary)
            ],
            "{mode:?}: the words and the inks they were drawn in"
        );

        // The row: the tabs run left to right from the strip's edge, each one
        // starting where the last ended, all the strip's height, and each word
        // sits its padding in from the tab it is in.
        assert!(near(tabs[0].left(), strip.left()), "{mode:?}");
        for pair in tabs.windows(2) {
            assert!(near(pair[0].right(), pair[1].left()), "{mode:?}: {tabs:?}");
        }
        for (tab, word) in tabs.iter().zip(&words) {
            assert!(
                near(tab.top(), strip.top()) && near(tab.bottom(), strip.bottom()),
                "{mode:?}: a tab is {tab:?} in a strip {strip:?}"
            );
            assert!(
                near(word.left, tab.left() + TOKENS.space[4]),
                "{mode:?}: {:?} starts at {} in a tab starting at {}",
                word.text,
                word.left,
                tab.left()
            );
        }
    }
}

#[test]
fn the_bar_is_under_whichever_tab_is_open() {
    for mode in MODES {
        let bar_ink = ink(semantic(mode.is_dark()).tabs.active_bar);
        for open in 0..NAMES.len() {
            let harness = drawn(mode, open, None);
            let (rects, words) = read(&harness, mode);
            let tabs = tab_rects(&harness);
            let bars = filled(&rects, bar_ink);
            assert_eq!(bars.len(), 1, "{mode:?}, open {open}: one bar");
            assert!(
                near(bars[0].rect.left(), tabs[open].left())
                    && near(bars[0].rect.right(), tabs[open].right()),
                "{mode:?}, open {open}: the bar is at {:?}, the tab at {:?}",
                bars[0].rect,
                tabs[open]
            );
            let primary = ink(semantic(mode.is_dark()).text.primary);
            let in_primary: Vec<&str> = words
                .iter()
                .filter(|w| w.ink == primary)
                .map(|w| w.text.as_str())
                .collect();
            assert_eq!(in_primary, vec![NAMES[open]], "{mode:?}, open {open}");
        }
    }
}

#[test]
fn an_index_past_the_end_leaves_every_tab_closed() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let harness = drawn(mode, NAMES.len(), None);
        let (rects, words) = read(&harness, mode);
        assert!(
            filled(&rects, ink(sem.tabs.active_bar)).is_empty(),
            "{mode:?}: a bar under no tab"
        );
        assert!(
            words.iter().all(|w| w.ink == ink(sem.text.secondary)),
            "{mode:?}: a word is open"
        );
        for name in NAMES {
            let tab = harness.get_by_role_and_label(A11yRole::Tab, name);
            assert_ne!(
                tab.accesskit_node().is_selected(),
                Some(true),
                "{mode:?}: {name:?} reads selected"
            );
        }
    }
}

#[test]
fn a_caller_can_pass_the_bars_ink_and_only_the_bars() {
    let hue = Color32::from_rgb(0xd0, 0x40, 0x80);
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        let harness = drawn(mode, 1, Some(hue));
        let (rects, words) = read(&harness, mode);
        let tabs = tab_rects(&harness);

        let bars = filled(&rects, hue);
        assert_eq!(bars.len(), 1, "{mode:?}: one bar in the caller's ink");
        assert!(
            near(bars[0].rect.left(), tabs[1].left())
                && near(bars[0].rect.right(), tabs[1].right())
                && near(bars[0].rect.height(), 2.0),
            "{mode:?}: the bar is {:?}",
            bars[0].rect
        );
        assert!(
            filled(&rects, ink(sem.tabs.active_bar)).is_empty(),
            "{mode:?}: the token's bar is painted as well"
        );

        // The ink is the bar's: the rule and the words keep theirs.
        assert_eq!(
            filled(&rects, ink(sem.borders.divider)).len(),
            1,
            "{mode:?}: the rule is not in the divider ink"
        );
        let inks: Vec<Color32> = words.iter().map(|w| w.ink).collect();
        assert_eq!(
            inks,
            vec![
                ink(sem.text.secondary),
                ink(sem.text.primary),
                ink(sem.text.secondary)
            ],
            "{mode:?}: a word took the caller's ink or lost its own"
        );
    }
}

#[test]
fn nothing_in_a_strip_has_a_corner_and_every_width_is_the_tokens() {
    for mode in MODES {
        let sem = semantic(mode.is_dark());
        for open in [1, NAMES.len()] {
            let harness = drawn(mode, open, None);
            let (rects, _) = read(&harness, mode);

            // A rule, and a bar when a tab is open: nothing else is a rect.
            let expected = if open < NAMES.len() { 2 } else { 1 };
            assert_eq!(
                rects.len(),
                expected,
                "{mode:?}, open {open}: the strip painted {rects:?}"
            );
            for r in &rects {
                assert_eq!(
                    r.radius,
                    CornerRadius::ZERO,
                    "{mode:?}, open {open}: a rect at {:?} is drawn with corners {:?}",
                    r.rect,
                    r.radius
                );
                assert!(
                    !r.is_stroked() && near(r.blur, 0.0),
                    "{mode:?}, open {open}: a rect at {:?} is stroked or blurred",
                    r.rect
                );
            }

            // The rule is a 1px hairline and the bar is the token's width.
            let rule = filled(&rects, ink(sem.borders.divider));
            assert!(near(rule[0].rect.height(), 1.0), "{mode:?}, open {open}");
            if open < NAMES.len() {
                let bar = filled(&rects, ink(sem.tabs.active_bar));
                assert!(
                    near(bar[0].rect.height(), TAB_BAR_WIDTH),
                    "{mode:?}, open {open}"
                );
            }
        }
    }
}

// ─── the click ──────────────────────────────────────────────────────────────

#[test]
fn a_click_on_a_closed_tab_reports_it_and_a_click_on_the_open_one_reports_none() {
    for mode in MODES {
        for (open, open_name) in NAMES.iter().enumerate() {
            for (target, target_name) in NAMES.iter().enumerate() {
                let mut harness = drawn(mode, open, None);
                assert!(
                    harness.state().clicks.is_empty(),
                    "{mode:?}: a click before any click"
                );
                harness
                    .get_by_role_and_label(A11yRole::Tab, target_name)
                    .click();
                harness.run();
                let expected: Vec<usize> = if target == open { vec![] } else { vec![target] };
                assert_eq!(
                    harness.state().clicks,
                    expected,
                    "{mode:?}: clicking {target_name:?} with {open_name:?} open"
                );
            }
        }
    }
}

// ─── the tree ───────────────────────────────────────────────────────────────

#[test]
fn each_word_is_a_tab_inside_one_tab_list_and_the_open_one_reads_selected() {
    for mode in MODES {
        let harness = drawn(mode, 1, None);

        let lists: Vec<_> = harness.get_all_by_role(A11yRole::TabList).collect();
        assert_eq!(lists.len(), 1, "{mode:?}: one tab list");
        let list = lists[0].accesskit_node().id();

        let tabs: Vec<_> = harness.get_all_by_role(A11yRole::Tab).collect();
        assert_eq!(tabs.len(), NAMES.len(), "{mode:?}: one tab per name");
        assert!(
            harness.query_all_by_role(A11yRole::Button).next().is_none(),
            "{mode:?}: a word is described as a button"
        );

        for (i, name) in NAMES.iter().enumerate() {
            let tab = harness.get_by_role_and_label(A11yRole::Tab, name);
            let parent = tab.accesskit_node().parent().map(|p| p.id());
            assert_eq!(
                parent,
                Some(list),
                "{mode:?}: {name:?} is not inside the tab list"
            );
            let selected = tab.accesskit_node().is_selected();
            if i == 1 {
                assert_eq!(selected, Some(true), "{mode:?}: {name:?} is the open tab");
            } else {
                assert_ne!(selected, Some(true), "{mode:?}: {name:?} reads selected");
            }
        }
    }
}

// ─── the signature a consumer calls ─────────────────────────────────────────

/// The strip called the way a consumer calls it. A compile-time claim — a
/// changed signature fails to build here — with a run behind it.
#[test]
fn the_strip_is_called_the_way_a_consumer_calls_it() {
    let mut harness = Harness::builder()
        .with_size(egui::vec2(400.0, 120.0))
        .build_ui(move |ui| {
            theme::apply(ui.ctx(), Mode::Light);
            let names: &[&str] = &["a", "b"];
            let out: LineTabsResponse = line_tabs(ui, LineTabs::new(names, 0));
            let _: Option<usize> = out.clicked;
            let _: egui::Response = out.response;
            let hue = Color32::from_rgb(1, 2, 3);
            let _ = line_tabs(ui, LineTabs::new(names, 1).bar_ink(hue));
        });
    harness.run();
}
