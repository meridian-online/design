"""CONCEPT: form — the disc is traced, three S's draw, then the mark arrives.

The sequence Hugh asked for, and it turns the loop into a formation:

    an arc laps the disc outline, right over the top, and closes
    one S per row flows THROUGH, right to left, staggered
    the frame empties
    the mark assembles row by row, same direction
    it holds, then disperses the same way, and loops

The orbit is the first beat, and it is the SAME instrument as the S's rather
than a borrowed one. `s_layer` draws its dash with a trim pair — head on
`track(t0 + LAG, DRAW)`, tail on `track(t0, DRAW)` — so what is drawn is the
gap between two copies of one eased ramp, offset in time. Run that pair on a
closed circle instead of an open S and the arc's LENGTH becomes the derivative
of the ramp: it grows out of a point, stretches into a comet as it accelerates
across the top, and contracts back into that same point as it decelerates. No
opacity animation is needed at either end, because the geometry already arrives
from nothing and leaves to nothing.

That is the mechanism in the yqOFSTuUsR reference, arrived at from our own
parts. The reference runs `s` over frames 0–79 and `e` over 8–87 at 60 fps —
one eased ramp and a delayed copy of it — and pairs it with a layer fade it
does not strictly need.

It laps ANTICLOCKWISE ON SCREEN — the head travels leftward across the top,
the direction everything else in this loop travels — and its seam is NOT the
right limb. It is `ORBIT_THETA`, the point where the first S becomes visible,
which sits a row higher. The arc therefore dies exactly where, and exactly
when, the S it hands to appears. Both halves of that are derived rather than
dialled in; see the derivation block below `circle_path`.

Everything eases SYMMETRICALLY now — the orbit and the S's alike, on
cubic-bezier(0.6, 0, 0.4, 1), slow at both limbs and quickest across the
middle. Because a trim pair's dash length is its ramp's speed, that shapes the
strokes as well as their travel: each S enters short, opens to about 68% of its
path at the midpoint, and closes as it leaves. Only the mark's own wipe still
runs the front-loaded EASE_O/EASE_I. The cost of symmetric on both sides of a
handover is that the two meet at their slowest moments, which is what
SEAM_OVERLAP exists to cover.

Two things changed from `trace`:

* ONE dash per row, not a stream. A stream is weather; a single stroke per row
  is a statement, and it lets the mark be the resolution rather than one more
  thing happening.
* The dash passes through and exits rather than drawing on and staying. It is
  the same trim pair as the wind reference — head leads, tail follows — and the
  stroke removes itself, so nothing has to be faded off before the mark lands.
* Right to left throughout — the S's, the mark's arrival and its exit. Each S
  comes in along the lower band, rises through the S, and leaves along the
  upper band; the fill then lands on exactly the boundary that stroke drew.
* The mark assembles ROW BY ROW rather than in one pass, and leaves the same
  way, so the whole thing loops: draw, assemble, hold, disperse, empty.

The mark is the REAL geometry, imported from meridian_black.svg via
python-lottie — six teeth, not a reconstruction. Its reveal is a rectangle
matte whose right edge sweeps 0 -> W, so the fill arrives in the same direction
as the strokes that predicted it.

Slower than `trace` throughout: the draw is 34 frames rather than 30, and
nothing overlaps more than it needs to.
"""
import json
import math
import pathlib

import svg_form
import tokens
from curves import ease_when as _ease_when, flatten as _flatten
from mark import MARK

HERE = pathlib.Path(__file__).resolve().parent

FPS = 25
W = H = 600
R = 100.0
DISC = 300.0
MARK_X = 300.0
K = 0.5522847498307936
OVER = 90.0
STROKE = 12.0             # one pen for the S's and the orbit alike

DRAW = 32                 # frames for the head to cross the whole path
LAG = 11                  # frames the tail trails it -> the dash's length
STAGGER = 9               # between rows, for the S's and for the mark alike

# THE ORBIT — step one, ahead of everything else.
ORBIT_DUR = 36            # frames for the head to lap the disc once
# The tail trails the head by this much, so the arc's peak length is the
# ramp's peak SPEED times this lag — see ORBIT_EASE_O. On the symmetric curve
# the orbit runs, four frames of a thirty-six frame lap open to a measured
# 26.9% of the circle at the midpoint: a comet of 97 degrees, against the
# reference's rather more delicate 69. This is the knob to turn if it wants to
# be finer — it scales the arc almost linearly, and past about 8 it stops
# reading as a travelling head at all and becomes a ring being drawn on.
ORBIT_LAG = 4
ORBIT_R = DISC - 7        # inset so the stroke clears the canvas edge

MARK_DUR = 22             # frames for one row's wipe
HOLD = 24                 # frames resting on the whole mark
OUT_DUR = 18              # leaves quicker than it arrived
# The wipe uncovers nothing on its own first frame: its edge starts at the
# limb, where the cos sampling in `turn` is slowest, so the first tooth pixel
# appears about three frames after MARK_IN.
WIPE_LATENCY = 3
# Frames of deliberate overlap at the S -> mark handover, so it is continuous
# rather than merely abutting.
MARK_OVER = 2

#: The leader's ink, per theme — `--m-ink`, the colour the mark is drawn in on
#: that surface.
#:
#: Read from the crate rather than typed. It WAS typed, for most of the
#: exploration: `#1a1917` on light and `#e9e6e0` on dark, and neither appears
#: anywhere in `tokens.css`. They were close enough to `--m-ink` to look right
#: and were never the same colour as the mark beside them. That is ADR 0011's
#: finding a third time — the shadow palette a consumer grows when nothing
#: makes it read the real one — arriving in a generator instead of in an app,
#: which is why the wake has always come from `tokens.py` and the leader now
#: does too.
def leader(theme):
    return tokens.colour("m-ink", theme)

# THE TRAIL. Every travelling stroke here is drawn by a trim pair, so a ghost
# of it is not a copy that has been moved — it is the SAME path with the SAME
# trim, run a few frames late. Nothing about the geometry or the schedule
# changes; the wake is the stroke's own past.
#
# The gap between one ghost and the next is the stroke's speed times the lag,
# which is the same relationship the dash length already has to the ramp. So
# the wake stretches where the stroke is quick and closes up where it is slow,
# without being told to.
#: Frames between one ghost and the next, and the knob that decides whether
#: the colour is a wake or a FLASH. The visible wake is speed x lag, so a small
#: lag means colour appears only where the stroke is quickest and is gone at
#: the limbs — you catch it rather than watch it, which is what the 5uvsTa4DXp
#: reference does with four copies one frame apart at 25 fps.
#:
#: At 3 the wake was continuously visible and read as a coloured stroke. At 1
#: each ghost sits 1/11 of a dash length behind, so the colour is a tip that
#: opens across the middle of a travel and closes at both ends.
#: Per-variant; this is the ceiling LOOP is sized against.
TRAIL_LAG = 2
#: Ghosts BEHIND the leader. Three, as the reference uses.
TRAIL_N = 3

#: Each entry is (family, light steps, dark steps). The leader is always the
#: theme's own ink — black on light, paper on dark — so a scheme only ever
#: describes what happens BEHIND it.
#:
#: The 12-step scales flip with the theme, so one list of steps serves both:
#: light runs pale to deep, dark runs deep to pale, with 9 the accent in each.
#: The chart ramps do NOT. `m-seq` and `m-div-blue` are defined once, in
#: `:root` only, because a data ramp is the same ramp whichever theme surrounds
#: it — so the dark wake has to walk them the other way round to recede into
#: its background rather than out of it.
#:
#: `m-seq` respects its own ordinal guidance: on light nothing paler than step
#: 250, on dark nothing deeper than 600.
#: A scheme is the wake, front to back — the leader is always the theme's own
#: ink, so the COLOUR COUNT of a variant is len(wake) + 1. Wakes need not all
#: be the same length: a shorter one simply finishes earlier inside the same
#: LOOP, which is sized against the longest.
#:
#: The categorical set is eight discrete hues, not a ramp. Every slot sits at
#: roughly the same weight — that is the point of a categorical palette, since
#: no series should outrank another — so a wake built from three of them does
#: not recede, it reads as three equal bands. Where a categorical slot appears
#: below it is spent ONCE, as a spark at the tail, with sequential blue doing
#: the fading in front of it.
#: A scheme is the wake, front to back — the leader is always the theme's own
#: ink, so a variant's COLOUR COUNT is len(wake) + 1. Wakes need not be the same
#: length: a shorter one finishes earlier inside the same LOOP, which is sized
#: against the longest.
#:
#: ORDER IS BY LUMINANCE, not by taste. A wake has to recede, so each step is
#: closer to the surface than the one in front of it — lighter on light, darker
#: on dark. For the 12-step scales that falls out of the scale itself. For the
#: categorical slots it does not, because they are eight discrete hues chosen
#: to sit at roughly equal weight, so the order has to be measured:
#:
#:     light   gold .385  >  orange .297  >  violet .130
#:     dark    gold .297  >  violet .184  >  orange .174
#:
#: which is why the warm schemes below reverse between themes, and why violet
#: leads on light but trails on dark.
#: A scheme is the wake, front to back — the leader is always the theme's own
#: ink, so a variant's COLOUR COUNT is len(wake) + 1.
#:
#: ORDER IS BY LUMINANCE, not by taste. A wake has to recede, so each step sits
#: closer to the surface than the one in front of it — lighter on light, darker
#: on dark. The 12-step scales and `m-seq` do that by construction. The
#: CATEGORICAL slots do not: they are eight discrete hues tuned to roughly
#: equal weight, so that no data series outranks another, and that is exactly
#: the property that stops them stacking into a fade. Their order has to be
#: measured, and it differs by theme:
#:
#:     light   gold .385  >  orange .297  >  violet .130
#:     dark    gold .297  >  violet .184  >  orange .174
#:
#: which is why `blue-violet` puts violet behind the blue on light and keeps it
#: there on dark, where violet is still the dimmer of the two.
SCHEMES = {
    # CHOSEN 2026-08-16 — see README. Ink and two steps of the Meridian
    # blue-240 sequential ramp; three colours in total.
    "seq3": (["m-seq-500", "m-seq-300"],
             ["m-seq-350", "m-seq-550"]),
    # Runner-up. Blue leads and violet trails, so the mark's own family starts
    # the wake and the categorical slot is the thing you catch at the end.
    "blue-violet": (["m-seq-500", "m-cat-5"], ["m-seq-350", "m-cat-5"]),
}
#: The mark's rows take a SHORTER lag than the strokes do, because a fill
#: carries its colour as area where a stroke carries it as a line: at the
#: stroke's lag each row came out 41% coloured, which read as colour-blocking
#: rather than as a trail. Derived from the stroke's lag rather than set
#: independently, so turning one knob keeps the two in proportion.
def mark_lag(trail_lag):
    return max(1, round(trail_lag * 2 / 3))


#: The ceiling, used to size LOOP so every variant is the same length and can
#: be compared frame for frame.
MARK_TRAIL_LAG = mark_lag(TRAIL_LAG)

# Rows on the mark's ladder. Each S sits where that row's real edge sits.
ROWS = [200.0, 400.0, 600.0]
# Which imported teeth make up each row, from their y extents: tooth 0 spans
# 7..100 and tooth 1 spans 100..200, so they are the top pair, and so on.
ROW_TEETH = [(0, 1), (2, 3), (5, 4)]

# The S's used to run a front-loaded ease, cubic-bezier(0.32, 0, 0.2, 1). They
# now run the same symmetric curve as the orbit, so the stroke is slow at both
# limbs and quickest across the middle. Kept here because the mark's own wipe
# still uses it, and because the difference is the point of the note below.
EASE_O = {"x": [0.32], "y": [0]}       # a gentle start
EASE_I = {"x": [0.2], "y": [1]}        # and a soft landing

# The orbit eases SYMMETRICALLY, and that is not a preference — it is what the
# arc-length mechanism needs. The pair above is front-loaded: with x2 = 0.2 the
# curve is near its target a fifth of the way through, so a head running it
# peaks at a third of the lap and then crawls. Measured on the circle, that put
# the widest arc at frame 12 of 40 and left the last twenty frames as a dash
# under twelve degrees creeping from ten o'clock to twelve — a visible stall,
# because the arc's LENGTH is that curve's speed and the speed had gone.
# cubic-bezier(0.6, 0, 0.4, 1) is symmetric (x2 = 1 - x1), so the peak sits at
# the midpoint and the growth and contraction mirror each other. It is also
# what the yqOFSTuUsR reference runs.
ORBIT_EASE_O = {"x": [0.6], "y": [0]}
ORBIT_EASE_I = {"x": [0.4], "y": [1]}

XF = {"ty": "tr", "p": {"a": 0, "k": [0, 0]}, "a": {"a": 0, "k": [0, 0]},
      "s": {"a": 0, "k": [100, 100]}, "r": {"a": 0, "k": 0},
      "o": {"a": 0, "k": 100}, "sk": {"a": 0, "k": 0}, "sa": {"a": 0, "k": 0},
      "nm": "Transform"}


def s_path(cy):
    """Right to left: in along the lower band, UP through the S, out left.

    Traversal order is the direction of travel — trim runs along the path — so
    reversing the vertices is the whole of the direction change. The tangents
    reverse with them: a reversed cubic swaps its in and out handles, which is
    why `o` here is the old `i` read backwards and vice versa.
    """
    x0, x1 = -OVER, W + OVER
    return {"v": [[x1, cy], [MARK_X + R, cy],
                  [MARK_X, cy - R], [MARK_X - R, cy - 2 * R], [x0, cy - 2 * R]],
            "o": [[0, 0], [-R * K, 0], [0, -R * K], [0, 0], [0, 0]],
            "i": [[0, 0], [0, 0], [0, R * K], [R * K, 0], [0, 0]],
            "c": False}


def circle_path(cx, cy, r, theta0=0.0):
    """A circle traversed ANTICLOCKWISE ON SCREEN from angle `theta0`.

    Anticlockwise on screen (y is down) means the head runs LEFTWARD across the
    top — the direction the S's flow and the direction the mark assembles.
    `theta0` is measured from the right limb, positive upwards, so the caller
    can put the seam wherever the choreography needs it; the arc grows out of
    that point and contracts back into it.

    Written out as four vertices rather than reached for as an `el` primitive
    because an ellipse fixes both its start point and its winding, and here
    both are the content. K is the circular bezier constant, so these four
    cubics are a circle to within a ten-thousandth of the radius.
    """
    v, o, i = [], [], []
    for n in range(4):
        a = theta0 + n * math.pi / 2
        ca, sa = math.cos(a), math.sin(a)
        v.append([cx + r * ca, cy - r * sa])
        # Travel direction at angle a is d/da (cos a, -sin a) = (-sin a, -cos a).
        o.append([-r * K * sa, -r * K * ca])
        i.append([r * K * sa, r * K * ca])
    return {"v": v, "o": o, "i": i, "c": True}


# ---------------------------------------------------------------- derivation
#
# What follows is measured off the geometry above rather than typed in. Two
# constants here were hand-computed once and both went stale the moment an
# easing curve changed — the entry and exit fractions of s_path are properties
# of the PATH, but the frames they land on are properties of the RAMP, and
# nothing connected the two. Deriving them is what keeps a change to
# S_EASE_O from silently reopening the eleven-frame hole described below.

S_EASE_O, S_EASE_I = ORBIT_EASE_O, ORBIT_EASE_I   # the S's now ease like the arc


def _visible_span(path):
    """Where along `path` the stroke is inside the disc, as arc-length
    fractions, plus the point it first becomes visible at.

    Everything in the `flow` precomp is clipped to the disc, and an S runs from
    x = W + OVER to x = -OVER, so a tenth of it at each end is never seen. That
    is the whole reason a naive `S_GONE` overstated the exit by twelve frames.
    """
    pts = _flatten(path)
    run = [0.0]
    for a in range(1, len(pts)):
        run.append(run[-1] + math.hypot(pts[a][0] - pts[a - 1][0],
                                        pts[a][1] - pts[a - 1][1]))
    inside = [a for a, p in enumerate(pts)
              if math.hypot(p[0] - W / 2, p[1] - H / 2) <= DISC]
    first, last = inside[0], inside[-1]
    return run[first] / run[-1], run[last] / run[-1], pts[first]


# The rows are NOT alike, because the disc crops them differently. Row 0's
# upper band runs along y = 0 and row 2's lower band along y = H, and both of
# those are TANGENT to the disc — they touch at a single point and are
# otherwise outside it. So row 0 is the first to appear (it enters along
# y = 200, crossing the disc at x = 582.7) but it vanishes early, at the
# tangent; and row 2 barely exists until it has risen, but exits along y = 400
# and stays visible almost to the end of its path.
#
# First in and last out are therefore different rows, and using one row for
# both — which is what this did at first — puts MARK_IN some frames early. It
# happens to be the safe direction to be wrong in, which is exactly why it
# needed measuring rather than assuming.
S_IN_F, _, S_IN_PT = _visible_span(s_path(ROWS[0]))
_, S_OUT_F, _ = _visible_span(s_path(ROWS[-1]))
S_IN_U = _ease_when(S_IN_F, S_EASE_O, S_EASE_I)    # normalised time, not frames
S_OUT_U = _ease_when(S_OUT_F, S_EASE_O, S_EASE_I)

# THE SEAM. The orbit grows from and collapses into one point, and that point
# is where the first S becomes visible — not the right limb, which sits a whole
# row lower. The arc therefore dies exactly where the S is about to appear.
ORBIT_THETA = math.asin((H / 2 - S_IN_PT[1]) / ORBIT_R)

# ...and it dies WHEN the S arrives, too, so the two events read as one event.
# Note this is not the frame the S LAYER starts: the head needs S_IN_U of its
# ramp to cross the off-canvas stretch first, and on the symmetric ease that is
# slower than it was on the front-loaded one.
#
# SEAM_OVERLAP is not a fudge. Abutting them exactly — orbit's last frame, then
# the S's first — leaves a one-frame hole, because the frame the head CROSSES
# the disc is the frame it has zero length: it is a boundary, not a mark. Ink
# counting found it immediately at frame 40. Three frames of overlap mean the
# arc's last wisp and the S's first stroke share the seam instead.
ORBIT_END = ORBIT_DUR + ORBIT_LAG
SEAM_OVERLAP = 3
LEAD = round(ORBIT_END - S_IN_U * DRAW) - SEAM_OVERLAP

# When the last S stops being visible — twelve-odd frames before its tail
# reaches the end of the path. Setting MARK_IN from the path end instead once
# left ELEVEN fully empty frames here, and every filmstrip of it looked fine.
S_VANISH = round(LEAD + 2 * STAGGER + LAG + S_OUT_U * DRAW)
MARK_IN = S_VANISH - WIPE_LATENCY - MARK_OVER
# Each row wipes on its own beat, so the mark assembles rather than arriving as
# one block. It exits the way it came — uncovering from the left, same
# direction, same stagger — and lands on an empty frame so the loop restarts.
MARK_ALL_IN = MARK_IN + 2 * STAGGER + MARK_DUR
OUT_AT = MARK_ALL_IN + HOLD
LOOP = OUT_AT + 2 * STAGGER + TRAIL_N * MARK_TRAIL_LAG + OUT_DUR + 2


LIN_I = {"x": [1], "y": [1]}
LIN_O = {"x": [0], "y": [0]}


def seq(kfs):
    """Drop coincident keyframes and give every non-terminal one handles."""
    out = []
    for kf in kfs:
        if out and abs(out[-1]["t"] - kf["t"]) < 1e-6:
            out[-1] = kf
        else:
            out.append(kf)
    for kf in out[:-1]:
        kf.setdefault("i", LIN_I)
        kf.setdefault("o", LIN_O)
    out[-1].pop("i", None)
    out[-1].pop("o", None)
    return out


def turn(t0, dur, x_from, x_to, n=9):
    """Keyframes walking the wipe edge the way a meridian crosses a sphere.

    Half a cosine: still at one limb, quickest at the centre, still at the
    other. Linear between samples, because the sampling already carries the
    curve. The last keyframe of the pair is left for the caller to follow.
    """
    out = []
    for k in range(n):
        u = k / (n - 1)
        x = x_from + (x_to - x_from) * (1 - math.cos(math.pi * u)) / 2
        kf = {"t": t0 + dur * u, "s": [x, H / 2, 0]}
        if k < n - 1:
            kf["i"], kf["o"] = LIN_I, LIN_O
        out.append(kf)
    return out


def track(t0, dur, ease_o=EASE_O, ease_i=EASE_I):
    return {"a": 1, "k": [
        {"t": t0, "s": [0], "i": ease_i, "o": ease_o},
        {"t": t0 + dur, "s": [100]}]}


def s_layer(ind, cy, t0, colour, nm="S"):
    return {
        "ddd": 0, "ind": ind, "ty": 4, "nm": f"{nm} {ind}", "sr": 1, "bm": 0,
        "ao": 0,
        "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
               "p": {"a": 0, "k": [0, 0, 0]}, "a": {"a": 0, "k": [0, 0, 0]},
               "s": {"a": 0, "k": [100, 100, 100]}},
        "shapes": [{"ty": "gr", "nm": "S", "it": [
            {"ty": "sh", "ind": 0, "nm": "Path 1", "ks": {"a": 0, "k": s_path(cy)}},
            # Head leads by LAG, tail follows: the dash enters at the right,
            # crosses, and leaves at the left under its own steam. Letting it
            # exit rather than fade is what makes it read as flowing THROUGH
            # rather than being drawn and rubbed out.
            {"ty": "tm", "nm": "Trim Paths 1", "m": 1,
             "s": track(t0 + LAG, DRAW, S_EASE_O, S_EASE_I),
             "e": track(t0, DRAW, S_EASE_O, S_EASE_I),
             "o": {"a": 0, "k": 0}},
            {"ty": "st", "nm": "Stroke 1", "c": {"a": 0, "k": colour},
             "o": {"a": 0, "k": 100}, "w": {"a": 0, "k": STROKE},
             "lc": 2, "lj": 2},
            XF]}],
        "ip": t0, "op": t0 + DRAW + LAG + 2, "st": 0,
    }


def trail_colours(leader, wake):
    """The leader's own colour, then the wake's, front to back.

    The leader keeps the variant's ink so the stroke that predicts the mark is
    still the mark's colour; only what follows it carries the scheme. That
    ordering matters for drawing too: layers[0] is topmost in Lottie, so the
    leader is emitted FIRST and the wake falls in behind it. Reversing that
    buries the stroke meant to be read under its own ghosts.
    """
    return [leader] + list(wake)


def orbit_layer(ind, colour, t0=0):
    """One lap of the disc outline, drawn by the same trim pair as an S.

    Lives at the TOP level, outside the disc matte, because the arc sits on
    the limb: matted, its outer half would be cut away and it would read as a
    six-wide stroke that thins wherever the antialiasing softened. The `limb`
    guide ring already sits out here for the same reason.

    Both ends erase themselves. Before `s` starts moving the drawn segment is
    [0, e], growing out of the right limb; after `e` reaches the end it is
    [s, 1], the last of the lap contracting back into that same point.

    `op` lands ON the frame the trim closes rather than after it. A round cap
    on a zero-length segment is not nothing — lottie-web draws it as a dot of
    the full stroke width, which showed up in the render as a stray pip
    sitting at the right limb after the arc had gone.
    """
    return {
        "ddd": 0, "ind": ind, "ty": 4,
        "nm": "orbit" if not t0 else f"orbit wake {t0}", "sr": 1, "bm": 0,
        "ao": 0,
        "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
               "p": {"a": 0, "k": [0, 0, 0]}, "a": {"a": 0, "k": [0, 0, 0]},
               "s": {"a": 0, "k": [100, 100, 100]}},
        "shapes": [{"ty": "gr", "nm": "orbit", "it": [
            {"ty": "sh", "ind": 0, "nm": "Path 1",
             "ks": {"a": 0, "k": circle_path(W / 2, H / 2, ORBIT_R,
                                             ORBIT_THETA)}},
            {"ty": "tm", "nm": "Trim Paths 1", "m": 1,
             "s": track(t0 + ORBIT_LAG, ORBIT_DUR, ORBIT_EASE_O, ORBIT_EASE_I),
             "e": track(t0, ORBIT_DUR, ORBIT_EASE_O, ORBIT_EASE_I),
             "o": {"a": 0, "k": 0}},
            {"ty": "st", "nm": "Stroke 1", "c": {"a": 0, "k": colour},
             "o": {"a": 0, "k": 100}, "w": {"a": 0, "k": STROKE},
             "lc": 2, "lj": 2},
            XF]}],
        "ip": t0, "op": t0 + ORBIT_DUR + ORBIT_LAG, "st": 0,
    }


def build(name, colour, wake, lag=TRAIL_LAG):
    flow = []
    ind = 1

    # Each row is its own precomp: a wipe matte over that row's two teeth.
    # A matte in Lottie applies to exactly one layer, so per-row staggering
    # means per-row precomps — one wipe cannot serve three independent beats.
    #
    # The rect is 2W wide centred at p.x, covering [p.x-W, p.x+W]:
    #   p.x = 2W  nothing        W or 0  fully covered        -W  nothing again
    # so 2W -> W wipes ON from the right (its left edge sweeps W -> 0) and
    # 0 -> -W wipes OFF from the right (its right edge sweeps W -> 0). The
    # W -> 0 leg between them is visually static; it is what holds the row.
    #
    # THE TURN. Those edges do not travel linearly — they travel like a
    # meridian on a turning sphere, which is the whole difference between a
    # wipe and a globe. A meridian at longitude t projects to
    #
    #     x = cx + R.cos(pi.t)
    #
    # so the edge is nearly still at each limb and quickest across the centre.
    # That is the foreshortening the eye reads as rotation, and it is the same
    # projection the terminator work earlier in this session used. Sampling it
    # into keyframes is exact; easing between two keyframes can only approximate
    # it, and approximating it is what made the arrival read as a wipe.
    assets = []
    for n, teeth in enumerate(ROW_TEETH):
        # THE MARK'S WAKE IS BUILT THE OTHER WAY ROUND. A row is revealed by a
        # matte, not drawn by a trim, so a ghost that simply starts late is
        # hidden: it has uncovered LESS than the ink in front of it, and the
        # ink covers it completely. Colour only shows where a ghost has got
        # somewhere the ink has not.
        #
        # So each ghost is on screen slightly LONGER than the copy in front of
        # it, at both ends — earlier in, later out. On the way in its edge
        # leads, so colour runs ahead of the ink; on the way out it lingers, so
        # colour is left behind. One band travelling with the wiping edge in
        # both directions.
        #
        # Its width is the edge's speed times the lag, and `turn` makes that
        # edge slowest at the limbs and quickest across the centre — so the
        # band is thin where the row starts and ends, widest as it crosses.
        # The same relationship the dash and the arc already have.
        for k, ink in enumerate(trail_colours(colour, wake)):
            t_in = MARK_IN + n * STAGGER - k * mark_lag(lag)
            t_out = OUT_AT + n * STAGGER + k * mark_lag(lag)
            rid = f"row{n}_{k}"
            wipe = {
                "ddd": 0, "ind": 1, "ty": 4, "nm": f"wipe {n}.{k}", "sr": 1,
                "bm": 0, "ao": 0, "td": 1,
                "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                       "a": {"a": 0, "k": [0, 0, 0]},
                       "s": {"a": 0, "k": [100, 100, 100]},
                       # in-turn, the static hold, out-turn — deduped, and
                       # every keyframe but the last carries handles.
                       "p": {"a": 1, "k": seq(
                           turn(t_in, MARK_DUR, 2 * W, W)
                           + turn(t_out, OUT_DUR, 0, -W))}},
                "shapes": [{"ty": "gr", "nm": "wipe", "it": [
                    {"ty": "rc", "nm": "Rect", "p": {"a": 0, "k": [0, 0]},
                     "s": {"a": 0, "k": [2 * W, 2 * H]},
                     "r": {"a": 0, "k": 0}},
                    {"ty": "fl", "nm": "Fill", "c": {"a": 0, "k": [0, 0, 0, 1]},
                     "o": {"a": 0, "k": 100}, "r": 1}, XF]}],
                "ip": t_in, "op": t_out + OUT_DUR + 1, "st": 0,
            }
            row = {
                "ddd": 0, "ind": 2, "ty": 4, "nm": f"teeth {n}.{k}", "sr": 1,
                "bm": 0, "ao": 0, "tt": 1,
                "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                       "p": {"a": 0, "k": [0, 0, 0]},
                       "a": {"a": 0, "k": [0, 0, 0]},
                       "s": {"a": 0, "k": [100, 100, 100]}},
                "shapes": [{"ty": "gr", "nm": "teeth", "it": (
                    [{"ty": "sh", "ind": i, "nm": f"tooth {i}",
                      "ks": {"a": 0, "k": MARK[i]}} for i in teeth]
                    + [{"ty": "fl", "nm": "Fill", "c": {"a": 0, "k": ink},
                        "o": {"a": 0, "k": 100}, "r": 1}, XF])}],
                "ip": t_in, "op": t_out + OUT_DUR + 1, "st": 0,
            }
            assets.append({"id": rid, "layers": [wipe, row]})
            flow.append({
                "ddd": 0, "ind": ind, "ty": 0, "nm": f"row {n}.{k}",
                "refId": rid, "sr": 1, "bm": 0, "ao": 0, "w": W, "h": H,
                "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                       "p": {"a": 0, "k": [W / 2, H / 2, 0]},
                       "a": {"a": 0, "k": [W / 2, H / 2, 0]},
                       "s": {"a": 0, "k": [100, 100, 100]}},
                "ip": t_in, "op": t_out + OUT_DUR + 1, "st": 0})
            ind += 1

    # Each row is a leader and its wake: the same S, the same trim, started
    # TRAIL_LAG frames later per ghost. Emitted leader-first so the wake falls
    # in behind it rather than over it.
    for n, cy in enumerate(ROWS):
        for k, ink in enumerate(trail_colours(colour, wake)):
            flow.append(s_layer(ind, cy, LEAD + n * STAGGER + k * lag,
                                ink, "S" if k == 0 else "S wake"))
            ind += 1

    matte = {
        "ddd": 0, "ind": 1, "ty": 4, "nm": "disc", "sr": 1, "bm": 0, "ao": 0,
        "td": 1,
        "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
               "p": {"a": 0, "k": [0, 0, 0]}, "a": {"a": 0, "k": [0, 0, 0]},
               "s": {"a": 0, "k": [100, 100, 100]}},
        "shapes": [{"ty": "gr", "nm": "disc", "it": [
            {"ty": "el", "nm": "Ellipse", "p": {"a": 0, "k": [W / 2, H / 2]},
             "s": {"a": 0, "k": [2 * DISC, 2 * DISC]}},
            {"ty": "fl", "nm": "Fill", "c": {"a": 0, "k": [0, 0, 0, 1]},
             "o": {"a": 0, "k": 100}, "r": 1}, XF]}],
        "ip": 0, "op": LOOP, "st": 0,
    }
    content = {"ddd": 0, "ind": 2, "ty": 0, "nm": "flow", "refId": "flow",
               "sr": 1, "bm": 0, "ao": 0, "tt": 1, "w": W, "h": H,
               "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                      "p": {"a": 0, "k": [W / 2, H / 2, 0]},
                      "a": {"a": 0, "k": [W / 2, H / 2, 0]},
                      "s": {"a": 0, "k": [100, 100, 100]}},
               "ip": 0, "op": LOOP, "st": 0}
    # The matte has to sit immediately above the layer it mattes, so the orbit
    # goes AFTER the pair rather than on top of it. It draws under the mark,
    # which costs nothing: by the time any tooth is on screen the arc is gone.
    orbit = [orbit_layer(3 + k, ink, k * lag)
             for k, ink in enumerate(trail_colours(colour, wake))]
    layers = [matte, content] + orbit
    return {"v": "5.7.0", "fr": FPS, "ip": 0, "op": LOOP, "w": W, "h": H,
            "nm": name, "ddd": 0,
            "assets": assets + [{"id": "flow", "layers": flow}],
            "layers": layers}


# ------------------------------------------------------------------ the build
#
# The shipped asset and the exploration go to different places on purpose.
#
# `meridian-design/brand/motion/` is where ADR 0012 puts brand motion, inside
# the crate so the desktop can take the bytes as a cargo dependency, under
# `brand/LICENSE-BRAND.md` rather than the crate's MIT grant. Four files: the
# chosen scheme in both themes, each as Lottie for the desktop and as animated
# SVG for the web. `tests/motion.rs` pins them, `scripts/check-motion.sh`
# proves they are still what this generator produces.
#
# `motion/output/` is the scratch the comparison pages read, and it is
# untracked. The runner-up and the slower cut belong to a decision that has
# already been made; keeping them committed would leave files in the tree that
# look like assets, that nothing pins, and that a consumer could take.
ASSETS = HERE.parent / "meridian-design" / "brand" / "motion"
SCRATCH = HERE / "output"

#: The scheme that ships, and the lag that makes its colour a flash rather than
#: a wake. Everything else `SCHEMES` still knows how to build is exploration.
CHOSEN = "seq3"
CHOSEN_LAG = 1


def artefacts():
    """The four files that ship, as `{name: bytes}`.

    Both formats come from one document per theme, which is what makes them
    one animation rather than two that resemble each other.
    """
    out, error = {}, 0.0
    for theme, steps in zip(("light", "dark"), SCHEMES[CHOSEN]):
        stem = "form" if theme == "light" else "form_dark"
        wake = [tokens.colour(n, theme) for n in steps]
        doc = build(stem, leader(theme), wake, CHOSEN_LAG)
        svg, err = svg_form.emit(doc)
        error = max(error, err)
        out[f"{stem}.json"] = json.dumps(doc, separators=(",", ":")) + "\n"
        out[f"{stem}.svg"] = svg
    return out, error


def explore():
    """Every scheme and lag in `SCHEMES`, into the untracked scratch directory.

    Two axes: what the wake is made of, and how fleeting it is. Every file is
    the same length, so they can be stopped on the same frame and compared.
    """
    SCRATCH.mkdir(exist_ok=True)
    for lag, tag in ((1, "flash"), (2, "quick")):
        for name, (light, dark) in SCHEMES.items():
            if lag == 2 and name != CHOSEN:
                continue              # the slower cut, on the chosen scheme only
            for theme, steps in (("light", light), ("dark", dark)):
                suffix = "" if theme == "light" else "-dark"
                fn = f"form-{name}-{tag}{suffix}.json"
                wake = [tokens.colour(n, theme) for n in steps]
                doc = build(fn[:-5], leader(theme), wake, lag)
                (SCRATCH / fn).write_text(json.dumps(doc, separators=(",", ":")))
                swatch = " ".join(tokens.hex_of(n, theme) for n in steps)
                print(f"  {fn:<30} lag {lag}f  {len(steps) + 1} colours  {swatch}")


if __name__ == "__main__":
    import sys

    check = "--check" in sys.argv
    built, sampling = artefacts()

    if check:
        # What CI asks: are the committed artefacts still what this generator
        # produces? A snapshot catches an edit to the file; this catches an
        # edit to the generator that nobody regenerated for — the direction a
        # committed artefact actually drifts.
        stale = [fn for fn, text in built.items()
                 if not (ASSETS / fn).is_file()
                 or (ASSETS / fn).read_text() != text]
        for fn in stale:
            print(f"  STALE  brand/motion/{fn}")
        if stale:
            sys.exit("\nbrand/motion/ no longer matches build_form.py — run it, "
                     "and commit the artefacts with the change that moved them")
        print(f"brand/motion/ matches build_form.py ({len(built)} artefacts)")
        sys.exit(0)

    ASSETS.mkdir(exist_ok=True)
    for fn, text in built.items():
        (ASSETS / fn).write_text(text)
        print(f"brand/motion/{fn:<20} {len(text) / 1024:>5.1f} KB")
    print(f"\n  ink       {tokens.hex_of('m-ink')} / {tokens.hex_of('m-ink', 'dark')}"
          f"   wake {' '.join(tokens.hex_of(n) for n in SCHEMES[CHOSEN][0])}"
          f" / {' '.join(tokens.hex_of(n, 'dark') for n in SCHEMES[CHOSEN][1])}")
    print(f"  schedule  orbit 0-{ORBIT_END}f | S seen "
          f"{round(LEAD + S_IN_U * DRAW)}-{S_VANISH}f | in {MARK_IN}-"
          f"{MARK_ALL_IN}f | hold {HOLD}f | out {OUT_AT}-{LOOP - 2}f | "
          f"{LOOP / FPS:.2f}s at {FPS}fps")
    # The one place the SVG cannot be exact: a dash length is the difference of
    # two eased ramps, which no CSS timing function expresses, so it is sampled
    # per frame. Printed rather than asserted in a comment.
    print(f"  sampling  SVG dash ends land at most {sampling:.3f} units from "
          f"the Lottie's continuous ease, on a {W}-unit canvas "
          f"({sampling / STROKE:.1%} of a stroke width)")

    if "--explore" in sys.argv:
        print("\n  scratch (untracked, for motion/preview/schemes.html)")
        explore()
