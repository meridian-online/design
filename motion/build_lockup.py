"""CONCEPT: lockup — one edge crosses the brand, and the brand is behind it.

ADR 0012's second capped asset. `form` draws the mark alone; this draws the
mark and the wordmark together, at the placement `meridian_lockup.svg` holds:

    an edge enters at the left of the disc
    the mark's three rows arrive behind it, each a beat later than the last
    the edge carries on across the gap and writes the word
    it holds on the whole lockup, then leaves the same way, and loops

## One instrument, four times

Everything here is the same thing: a rectangular matte whose edge sweeps, with
a wake of ghost copies that uncover slightly more than the ink in front of
them. That is `form`'s mark-arrival mechanism exactly — `build_form.build`
builds each row this way — and it is applied here to the mark's three rows and
then to the word.

Using one instrument is the point rather than an economy. A lockup is a mark
and a word that belong together; drawing them with two different gestures
would say the opposite. The only difference between the mark's beats and the
word's is duration, because the word is four times as wide.

## It travels LEFT TO RIGHT, which is the opposite of `form`

`form` runs right to left throughout. This runs left to right, and the reason
is not aesthetic: a word is read left to right, and the lockup is the mark
*then* the word. An edge that crossed this canvas right to left would write
the word backwards and hand the eye to the mark last. The direction follows
what the artwork is, so the two assets differ here and should.

Mechanically it is the rect's RIGHT edge that leads here, where `form` leads
with its left. The static leg between arrival and departure survives the
change: the rect is wider than the canvas, so it still covers the artwork at
both ends of the leg, and the hold costs no keyframes.

## Each beat's sweep is scoped to its own ink

`form` sweeps its canvas, which is right there because its mark fills it. Here
the mark occupies 58 units of a 336-wide canvas, so a canvas-wide sweep spends
a fifth of each row's duration on the row and the rest travelling over empty
space — a row that finished early and then waited. `matte` therefore takes the
beat's own extent, and the whole of a duration is spent crossing ink. The wake
depends on it too: its width is the edge's speed times its lag, so an edge
going five times too fast turned the colour from a band into a block.

## The geometry is read, not placed

Every path comes from `lockup.py`, which parses `meridian_lockup.svg`. Nothing
in this file knows where the mark sits or how big the word is — the mark's
rows are split by the same tooth grouping `form` uses, and the word's extent
comes from its own vertices. A lockup that moved would move this animation
with it, and `tests/motion.rs` holds the composite to its components so a
lockup that was *redrawn* fails instead.

## What is deliberately not here

No orbit and no S-strokes. `form` opens with an arc lapping the disc and three
strokes flowing through, and both are about the mark's own construction; they
have nothing to say about a word. Borrowing them would have made this a longer
`form` with a wordmark appended rather than its own asset, and it would have
pushed the loop past ten seconds for a front door.
"""
import json
import pathlib

import lockup
import svg_form
import tokens
from build_form import (
    ROW_TEETH, SCHEMES, TRAIL_N, XF, leader, mark_lag, seq, trail_colours,
    turn,
)

HERE = pathlib.Path(__file__).resolve().parent

FPS = 25
#: The lockup's own canvas, so every path can be used exactly as the tracked
#: file draws it and every layer keeps an identity transform.
W = lockup.CANVAS_W
H = lockup.CANVAS_H

# ------------------------------------------------------------------ the beats
#
# Frames at 25 fps. The mark is 58.6 units wide and the word is 236, so the
# word's sweep is longer — but not four times longer, because an edge crossing
# a word does not have to travel at the mark's pace to read as the same edge.
# WORD_DUR was set by eye against the row duration and then checked: at 40 the
# edge crosses about six units per frame, which is a fifth of a letter width,
# so no letter appears in a single frame.
MARK_DUR = 18             # frames for one row's wipe
STAGGER = 7               # between rows — the mark still assembles row by row
#: The word starts before the mark has finished, so the edge reads as one
#: continuous travel rather than as two gestures with a seam. Three frames of
#: overlap is the same order as `form`'s SEAM_OVERLAP and for the same reason:
#: abutting two eased beats exactly leaves them meeting at their slowest.
HANDOVER = 3
WORD_DUR = 40             # frames for the edge to cross the word
HOLD = 45                 # frames resting on the whole lockup
OUT_DUR = 16              # leaves quicker than it arrived

#: Ghosts behind each wiping edge, and how far behind. Taken from `form` rather
#: than chosen again: the wake is the same instrument, so it takes the same
#: setting, and `mark_lag` is what keeps a fill's lag proportional to a
#: stroke's.
TRAIL_LAG = 1
ROW_LAG = mark_lag(TRAIL_LAG)

#: The chosen scheme, shared with `form` so the two assets wake the same
#: colour. Reading it from `build_form.SCHEMES` rather than restating the steps
#: is the same rule `tokens.py` keeps one level down.
CHOSEN = "seq3"

#: Ghosts actually emitted. `TRAIL_N` is the ceiling `form` sizes against, but
#: a scheme supplies its own steps and `seq3` supplies two — so taking the
#: ceiling here would reserve frames the wake never uses.
WAKE_N = min(len(SCHEMES[CHOSEN][0]), TRAIL_N)

#: The first beat starts here rather than at frame 0, because a wake runs
#: EARLIER than the ink it trails: ghost `k` opens `k * ROW_LAG` frames before
#: its leader. At frame 0 that put the outermost ghost of the first row at
#: frame -2, where the composition clips it — so the first row entered with a
#: truncated wake, and the two lost frames reappeared as a stray blue sliver at
#: the very end of the loop, wrapped around. Nothing is now asked to happen
#: before the composition starts, and nothing waits that does not have to.
LEAD_IN = WAKE_N * ROW_LAG

MARK_ALL_IN = LEAD_IN + 2 * STAGGER + MARK_DUR
WORD_IN = MARK_ALL_IN - HANDOVER
WORD_ALL_IN = WORD_IN + WORD_DUR
OUT_AT = WORD_ALL_IN + HOLD
#: The mark leaves first because it is leftmost and the departure travels the
#: same way the arrival did. The word's exit is longer for the same reason its
#: entrance was.
WORD_OUT = OUT_AT
MARK_OUT = OUT_AT + 4
WORD_OUT_DUR = round(OUT_DUR * WORD_DUR / MARK_DUR)
#: The tail is the mirror of `LEAD_IN`: a ghost leaves LATER than its leader,
#: so the loop has to outlast the last one by as many frames as the wake is
#: deep, plus a beat of rest before it starts again.
LOOP = max(MARK_OUT + 2 * STAGGER + OUT_DUR,
           WORD_OUT + WORD_OUT_DUR) + WAKE_N * ROW_LAG + 2


#: How far past a beat's ink each sweep runs, in canvas units.
#:
#: A clip edge that comes to rest EXACTLY on an ink edge leaks a sub-pixel
#: column of that ink through the renderer's antialiasing. Measured: with the
#: departure ending at the word's own right edge, a hairline of the N's right
#: stem stayed on screen for the 22% of the loop the clip spends parked there —
#: visible in an `<img>` at ordinary sizes, and not something a filmstrip of the
#: interesting frames would ever show, because it is only there while nothing is
#: supposed to be.
#:
#: One unit of 336 is a third of a percent of the canvas, so it costs nothing in
#: pacing, and it is far larger than any antialiasing footprint at any size this
#: is rendered at.
CLEAR = 1.0


def wake_colours(theme):
    light_steps, dark_steps = SCHEMES[CHOSEN]
    steps = light_steps if theme == "light" else dark_steps
    return [tokens.colour(step, theme) for step in steps][:TRAIL_N]


def matte(t_in, dur, t_out, out_dur, x0, x1):
    """A rect whose right edge uncovers this beat from the left, then leaves.

    **The sweep is scoped to the beat's own artwork, not to the canvas**, and
    that is the difference between a duration and a duration that does
    something. The rect is `2W` wide, so its right edge sits at `p.x + W`:
    running `p.x` from `x0 - W` to `x1 - W` walks that edge from the left of
    this beat's ink to the right of it, and the whole of `dur` is spent
    crossing ink.

    Sweeping the canvas instead — which is what `form` does, correctly, because
    its mark fills its canvas — costs most of the beat here. The mark occupies
    x 11 to 69.6 of 336, so a canvas-wide edge cleared it at 21% of the sweep
    and the remaining 79% was a fully-drawn row waiting for a wipe to finish
    somewhere off to the right. It also flattened the wake, whose width is the
    edge's speed times its lag: an edge crossing five times the distance in the
    same time is five times too fast, and the colour became a block rather than
    a band.

    Both ends run `CLEAR` past the ink rather than stopping on it, so a resting
    clip edge never coincides with an edge of the artwork.

    Departure is the mirror: the LEFT edge is `p.x - W`, so `x0 + W` to
    `x1 + W` empties the beat in the same direction it filled. Between the two
    legs the rect covers `[x0, x1]` at both ends — `[x1 - 2W, x1]` and
    `[x0, x0 + 2W]` both contain it — so the hold costs no keyframes, which is
    the same trick `form` uses one canvas up.
    """
    return {
        "ddd": 0, "ind": 1, "ty": 4, "nm": "wipe", "sr": 1, "bm": 0, "ao": 0,
        "td": 1,
        "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
               "a": {"a": 0, "k": [0, 0, 0]},
               "s": {"a": 0, "k": [100, 100, 100]},
               "p": {"a": 1, "k": seq(
                   turn(t_in, dur, x0 - CLEAR - W, x1 + CLEAR - W, y=H / 2)
                   + turn(t_out, out_dur, x0 - CLEAR + W, x1 + CLEAR + W,
                          y=H / 2))}},
        "shapes": [{"ty": "gr", "nm": "wipe", "it": [
            {"ty": "rc", "nm": "Rect", "p": {"a": 0, "k": [0, 0]},
             "s": {"a": 0, "k": [2 * W, 2 * H]}, "r": {"a": 0, "k": 0}},
            {"ty": "fl", "nm": "Fill", "c": {"a": 0, "k": [0, 0, 0, 1]},
             "o": {"a": 0, "k": 100}, "r": 1}, XF]}],
        "ip": t_in, "op": t_out + out_dur + 1, "st": 0,
    }


def inked(paths, ink, t_in, t_out, out_dur, nm):
    """The artwork this beat reveals, filled flat and matted by the wipe above."""
    return {
        "ddd": 0, "ind": 2, "ty": 4, "nm": nm, "sr": 1, "bm": 0, "ao": 0,
        "tt": 1,
        "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
               "p": {"a": 0, "k": [0, 0, 0]}, "a": {"a": 0, "k": [0, 0, 0]},
               "s": {"a": 0, "k": [100, 100, 100]}},
        "shapes": [{"ty": "gr", "nm": "ink", "it": (
            [{"ty": "sh", "ind": i, "nm": f"path {i}", "ks": {"a": 0, "k": p}}
             for i, p in enumerate(paths)]
            + [{"ty": "fl", "nm": "Fill", "c": {"a": 0, "k": ink},
                "o": {"a": 0, "k": 100}, "r": 1}, XF])}],
        "ip": t_in, "op": t_out + out_dur + 1, "st": 0,
    }


def beat(rid, nm, paths, ink, t_in, dur, t_out, out_dur):
    """One wiped reveal as its own precomp, with the flow layer that places it.

    A track matte in Lottie applies to exactly one layer, so a beat that
    staggers independently needs a precomp of its own — the same constraint
    `form` documents, and the reason the asset count here is beats times wake
    copies rather than one matte serving everything.

    The sweep's extent is read off the paths this beat draws, so a beat covers
    its own ink and nothing else has to be told where that is.
    """
    x0, _, x1, _ = lockup.bounds(paths)
    asset = {"id": rid, "layers": [matte(t_in, dur, t_out, out_dur, x0, x1),
                                   inked(paths, ink, t_in, t_out, out_dur, nm)]}
    flow = {"ddd": 0, "ind": 0, "ty": 0, "nm": nm, "refId": rid, "sr": 1,
            "bm": 0, "ao": 0, "w": W, "h": H,
            "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                   "p": {"a": 0, "k": [W / 2, H / 2, 0]},
                   "a": {"a": 0, "k": [W / 2, H / 2, 0]},
                   "s": {"a": 0, "k": [100, 100, 100]}},
            "ip": t_in, "op": t_out + out_dur + 1, "st": 0}
    return asset, flow


def build(name, theme):
    """The whole composition: three mark rows, then the word, each with a wake."""
    inks = trail_colours(leader(theme), wake_colours(theme))

    # Each beat is (label, paths, when it starts, how long, when it leaves,
    # how long that takes). The mark's rows are grouped by the same tooth
    # pairs `form` uses; the word is one beat because one edge crosses it.
    beats = []
    for n, teeth in enumerate(ROW_TEETH):
        beats.append((f"row{n}", [lockup.MARK[i] for i in teeth],
                      LEAD_IN + n * STAGGER, MARK_DUR,
                      MARK_OUT + n * STAGGER, OUT_DUR))
    beats.append(("word", [p for _, paths in lockup.LETTERS for p in paths],
                  WORD_IN, WORD_DUR, WORD_OUT, WORD_OUT_DUR))

    assets, flow, ind = [], [], 1
    for label, paths, t_in, dur, t_out, out_dur in beats:
        # A ghost is on screen LONGER than the copy in front of it, at both
        # ends — earlier in, later out — because a matte reveals rather than
        # draws. Colour shows only where a ghost has uncovered something the
        # ink in front of it has not, so the visible band is the edge's speed
        # times the lag: thin at the limbs, widest across the middle.
        for k, ink in enumerate(inks):
            asset, layer = beat(f"{label}_{k}", f"{label}.{k}", paths, ink,
                                t_in - k * ROW_LAG, dur,
                                t_out + k * ROW_LAG, out_dur)
            layer["ind"] = ind
            assets.append(asset)
            flow.append(layer)
            ind += 1

    # The canvas is written as integers. It is integral — the lockup's viewBox
    # is `0 0 336 84` — and `lockup.py` only carries it as a float because that
    # is what parsing a viewBox gives you. Writing `336.0` would leave the two
    # formats stating the same canvas in two notations, which the conformance
    # test comparing them has to know about rather than simply compare.
    canvas = {"w": int(W), "h": int(H)}
    assert (canvas["w"], canvas["h"]) == (W, H), "a non-integral canvas"
    return {"v": "5.7.0", "fr": FPS, "ip": 0, "op": LOOP, **canvas,
            "nm": name, "ddd": 0, "assets": assets, "layers": flow}


# ------------------------------------------------------------------ the build
ASSETS = HERE.parent / "meridian-design" / "brand" / "motion"
SCRATCH = HERE / "output"


def artefacts(errors=None):
    """The four shipped files: two themes, each as Lottie and animated SVG."""
    out = {}
    errors = {} if errors is None else errors
    for theme, stem in (("light", "lockup"), ("dark", "lockup_dark")):
        doc = build(stem, theme)
        out[f"{stem}.json"] = json.dumps(doc, separators=(",", ":")) + "\n"
        svg, err = svg_form.emit(doc, lockup.VERBATIM)
        out[f"{stem}.svg"] = svg
        errors[stem] = err
    return out


def main(argv):
    check = "--check" in argv
    errors = {}
    made = artefacts(errors)
    bad = []
    for name, text in sorted(made.items()):
        path = ASSETS / name
        if check:
            if not path.exists() or path.read_text() != text:
                bad.append(name)
        else:
            path.write_text(text)
    if check:
        if bad:
            raise SystemExit(
                f"brand/motion/ no longer matches build_lockup.py: "
                f"{', '.join(bad)} — run `python3 build_lockup.py`")
        print(f"brand/motion/ matches build_lockup.py ({len(made)} artefacts)")
    else:
        for name in sorted(made):
            print(f"wrote {ASSETS / name}")
        worst = max(errors.values()) if errors else 0.0
        print(f"{LOOP} frames at {FPS} fps — {LOOP / FPS:.2f}s; "
              f"worst SVG sampling error {worst * 100:.4f}% of a path")


if __name__ == "__main__":
    import sys
    main(sys.argv[1:])
