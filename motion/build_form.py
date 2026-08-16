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
import re

HERE = pathlib.Path(__file__).resolve().parent
MARK_SVG = HERE.parent / "meridian-design" / "brand" / "meridian_black.svg"

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

INK = [0.102, 0.098, 0.090, 1]
MARITIME = [0.294, 0.478, 0.608, 1]
PAPER = [0.914, 0.902, 0.878, 1]      # the warm off-white, for dark surfaces

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

def mark_paths(svg_text):
    """The six teeth of the mark, read out of the tracked SVG as Lottie paths.

    This reads `meridian-design/brand/meridian_black.svg` rather than carrying
    a copy of the geometry, and that is deliberate. A second copy of the mark
    is the exact defect `brand/README.md` documents: the web app inlined the
    path as source and its copy drifted from the tracked file by a byte before
    anyone noticed. A generator with its own private mark would drift the same
    way, and nothing would say so.

    It is also a licence boundary. The mark is a trademark reserved outside
    this repository's MIT grant; the geometry stays in `brand/`, under the
    terms that actually govern it, and this file borrows it at build time.

    The mark uses five path commands — M, m, l, c and Z — so this is a
    complete parser for that path and not for SVG in general. Lottie wants
    tangents as offsets from their vertex, which is the whole of the
    conversion: a cubic's first control point becomes the previous vertex's
    `o`, its second becomes the new vertex's `i`, and a line leaves both zero.
    """
    found = re.search(r' d="([^"]*)"', svg_text)
    if not found:
        raise ValueError(f"{MARK_SVG.name} has no path data")
    tokens = re.findall(r'([MmLlCcZz])|(-?\d*\.?\d+)', found.group(1))

    out = []
    sub = {"v": [], "i": [], "o": [], "c": True}
    cur, start, cmd, i = [0.0, 0.0], [0.0, 0.0], "", 0

    def flush():
        nonlocal sub
        if len(sub["v"]) > 1:
            out.append(sub)
        sub = {"v": [], "i": [], "o": [], "c": True}

    def vertex(p):
        sub["v"].append(list(p))
        sub["i"].append([0.0, 0.0])
        sub["o"].append([0.0, 0.0])

    while i < len(tokens):
        if tokens[i][0]:                         # a command letter, else a number
            cmd, i = tokens[i][0], i + 1
            if cmd in "Zz":
                flush()
                cur = list(start)                # Z returns to the subpath start
                continue
        n = [float(tokens[i + k][1]) for k in range(6 if cmd == "c" else 2)]

        if cmd in "Mm":
            cur = n[:2] if cmd == "M" else [cur[0] + n[0], cur[1] + n[1]]
            i += 2
            flush()
            start = list(cur)
            vertex(cur)
        elif cmd == "l":
            cur = [cur[0] + n[0], cur[1] + n[1]]
            i += 2
            vertex(cur)                          # a line leaves both tangents 0
        elif cmd == "c":
            c1 = [cur[0] + n[0], cur[1] + n[1]]
            c2 = [cur[0] + n[2], cur[1] + n[3]]
            end = [cur[0] + n[4], cur[1] + n[5]]
            i += 6
            sub["o"][-1] = [c1[0] - cur[0], c1[1] - cur[1]]
            vertex(end)
            sub["i"][-1] = [c2[0] - end[0], c2[1] - end[1]]
            cur = end
        else:
            raise ValueError(f"{MARK_SVG.name} uses an unhandled command {cmd!r}")
    flush()
    return out


MARK = mark_paths(MARK_SVG.read_text())
if len(MARK) != 6:
    raise SystemExit(f"expected 6 teeth in {MARK_SVG.name}, parsed {len(MARK)}")


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


def _ease_at(u, eo, ei):
    """Value of a Lottie keyframe ease at normalised time u."""
    ox, oy, ix, iy = eo["x"][0], eo["y"][0], ei["x"][0], ei["y"][0]
    lo, hi = 0.0, 1.0
    for _ in range(60):                      # invert the bezier's x(p) = u
        p = (lo + hi) / 2
        if 3 * (1 - p) ** 2 * p * ox + 3 * (1 - p) * p * p * ix + p ** 3 < u:
            lo = p
        else:
            hi = p
    p = (lo + hi) / 2
    return 3 * (1 - p) ** 2 * p * oy + 3 * (1 - p) * p * p * iy + p ** 3


def _ease_when(y, eo, ei):
    """Normalised time at which that ease reaches value y."""
    lo, hi = 0.0, 1.0
    for _ in range(60):
        m = (lo + hi) / 2
        lo, hi = (m, hi) if _ease_at(m, eo, ei) < y else (lo, m)
    return (lo + hi) / 2


def _flatten(path, per=400):
    v, o, i = path["v"], path["o"], path["i"]
    n = len(v)
    out = []
    for a in range(n if path["c"] else n - 1):
        b = (a + 1) % n
        c0 = [v[a][k] + o[a][k] for k in (0, 1)]
        c1 = [v[b][k] + i[b][k] for k in (0, 1)]
        for k in range(per):
            t, u = k / per, 1 - k / per
            out.append([u ** 3 * v[a][j] + 3 * u * u * t * c0[j]
                        + 3 * u * t * t * c1[j] + t ** 3 * v[b][j]
                        for j in (0, 1)])
    out.append(v[0] if path["c"] else v[-1])
    return out


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
LOOP = OUT_AT + 2 * STAGGER + OUT_DUR + 2


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


def s_layer(ind, cy, t0, colour):
    return {
        "ddd": 0, "ind": ind, "ty": 4, "nm": f"S {ind}", "sr": 1, "bm": 0,
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


def orbit_layer(ind, colour):
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
        "ddd": 0, "ind": ind, "ty": 4, "nm": "orbit", "sr": 1, "bm": 0, "ao": 0,
        "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
               "p": {"a": 0, "k": [0, 0, 0]}, "a": {"a": 0, "k": [0, 0, 0]},
               "s": {"a": 0, "k": [100, 100, 100]}},
        "shapes": [{"ty": "gr", "nm": "orbit", "it": [
            {"ty": "sh", "ind": 0, "nm": "Path 1",
             "ks": {"a": 0, "k": circle_path(W / 2, H / 2, ORBIT_R,
                                             ORBIT_THETA)}},
            {"ty": "tm", "nm": "Trim Paths 1", "m": 1,
             "s": track(ORBIT_LAG, ORBIT_DUR, ORBIT_EASE_O, ORBIT_EASE_I),
             "e": track(0, ORBIT_DUR, ORBIT_EASE_O, ORBIT_EASE_I),
             "o": {"a": 0, "k": 0}},
            {"ty": "st", "nm": "Stroke 1", "c": {"a": 0, "k": colour},
             "o": {"a": 0, "k": 100}, "w": {"a": 0, "k": STROKE},
             "lc": 2, "lj": 2},
            XF]}],
        "ip": 0, "op": ORBIT_DUR + ORBIT_LAG, "st": 0,
    }


def build(name, colour, limb=None):
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
        t_in = MARK_IN + n * STAGGER
        t_out = OUT_AT + n * STAGGER
        wipe = {
            "ddd": 0, "ind": 1, "ty": 4, "nm": f"wipe {n}", "sr": 1, "bm": 0,
            "ao": 0, "td": 1,
            "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                   "a": {"a": 0, "k": [0, 0, 0]},
                   "s": {"a": 0, "k": [100, 100, 100]},
                   # in-turn, the static hold, out-turn — deduped, and every
                   # keyframe but the last carries interpolation handles.
                   "p": {"a": 1, "k": seq(
                       turn(t_in, MARK_DUR, 2 * W, W)
                       + turn(t_out, OUT_DUR, 0, -W))}},
            "shapes": [{"ty": "gr", "nm": "wipe", "it": [
                {"ty": "rc", "nm": "Rect", "p": {"a": 0, "k": [0, 0]},
                 "s": {"a": 0, "k": [2 * W, 2 * H]}, "r": {"a": 0, "k": 0}},
                {"ty": "fl", "nm": "Fill", "c": {"a": 0, "k": [0, 0, 0, 1]},
                 "o": {"a": 0, "k": 100}, "r": 1}, XF]}],
            "ip": t_in, "op": t_out + OUT_DUR + 1, "st": 0,
        }
        row = {
            "ddd": 0, "ind": 2, "ty": 4, "nm": f"teeth {n}", "sr": 1, "bm": 0,
            "ao": 0, "tt": 1,
            "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                   "p": {"a": 0, "k": [0, 0, 0]}, "a": {"a": 0, "k": [0, 0, 0]},
                   "s": {"a": 0, "k": [100, 100, 100]}},
            "shapes": [{"ty": "gr", "nm": "teeth", "it": (
                [{"ty": "sh", "ind": i, "nm": f"tooth {i}",
                  "ks": {"a": 0, "k": MARK[i]}} for i in teeth]
                + [{"ty": "fl", "nm": "Fill", "c": {"a": 0, "k": colour},
                    "o": {"a": 0, "k": 100}, "r": 1}, XF])}],
            "ip": t_in, "op": t_out + OUT_DUR + 1, "st": 0,
        }
        assets.append({"id": f"row{n}", "layers": [wipe, row]})
        flow.append({
            "ddd": 0, "ind": ind, "ty": 0, "nm": f"row {n}", "refId": f"row{n}",
            "sr": 1, "bm": 0, "ao": 0, "w": W, "h": H,
            "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                   "p": {"a": 0, "k": [W / 2, H / 2, 0]},
                   "a": {"a": 0, "k": [W / 2, H / 2, 0]},
                   "s": {"a": 0, "k": [100, 100, 100]}},
            "ip": t_in, "op": t_out + OUT_DUR + 1, "st": 0})
        ind += 1

    for n, cy in enumerate(ROWS):
        flow.append(s_layer(ind, cy, LEAD + n * STAGGER, colour))
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
    layers = [matte, content, orbit_layer(3, colour)]
    if limb:
        layers.append({
            "ddd": 0, "ind": 4, "ty": 4, "nm": "limb", "sr": 1, "bm": 0,
            "ao": 0,
            "ks": {"o": {"a": 0, "k": 100}, "r": {"a": 0, "k": 0},
                   "p": {"a": 0, "k": [0, 0, 0]}, "a": {"a": 0, "k": [0, 0, 0]},
                   "s": {"a": 0, "k": [100, 100, 100]}},
            "shapes": [{"ty": "gr", "nm": "limb", "it": [
                {"ty": "el", "nm": "Ellipse", "p": {"a": 0, "k": [W / 2, H / 2]},
                 "s": {"a": 0, "k": [2 * DISC - 3] * 2}},
                {"ty": "st", "nm": "Stroke", "c": {"a": 0, "k": limb},
                 "o": {"a": 0, "k": 100}, "w": {"a": 0, "k": 3},
                 "lc": 2, "lj": 1}, XF]}],
            "ip": 0, "op": LOOP, "st": 0})
    return {"v": "5.7.0", "fr": FPS, "ip": 0, "op": LOOP, "w": W, "h": H,
            "nm": name, "ddd": 0,
            "assets": assets + [{"id": "flow", "layers": flow}],
            "layers": layers}


if __name__ == "__main__":
    import os
    d = str(HERE / "output") + "/"
    for fn, colour, limb in (("form-ink.json", INK, None),
                             ("form-paper.json", PAPER, None),
                             ("form-maritime.json", MARITIME, None),
                             ("form-ink-limb.json", INK,
                              [0.855, 0.839, 0.816, 1])):
        doc = build(fn[:-5], colour, limb)
        with open(d + fn, "w") as f:
            json.dump(doc, f, separators=(",", ":"))
        print(f"{fn:<24} {os.path.getsize(d + fn) / 1024:>5.1f} KB  "
              f"{doc['op'] / doc['fr']:.2f}s  "
              # The VISIBLE window in both cases. The S layer starts at LEAD,
              # but its head spends S_IN_U of the ramp off-canvas first; and it
              # stops being seen at S_VANISH, a dozen frames before its tail
              # reaches the end of the path.
              f"orbit 0-{ORBIT_END}f | S seen "
              f"{round(LEAD + S_IN_U * DRAW)}-{S_VANISH}f"
              f" | in {MARK_IN}-{MARK_ALL_IN}f"
              f" | hold {HOLD}f | out {OUT_AT}-{LOOP - 2}f")
