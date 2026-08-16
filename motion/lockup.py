"""The lockup — mark and wordmark, placed — parsed from the tracked SVG.

`mark.py` reads `meridian_black.svg` for the mark alone. This reads
`meridian_lockup.svg`, which carries the mark AND the wordmark already sitting
where the brand puts them, and that is the whole reason it exists as a separate
reader rather than as a placement computed here.

## Why the composite file rather than the two components

The alternative is to read `meridian_black.svg` and `meridian_wordmark.svg`
separately and apply a scale and an offset in this file. That would put the
lockup's geometry in Python — a third copy of a placement that already exists
in two tracked SVGs — and the first time the lockup moved, the animation would
keep the old one and nothing would say so. It also costs the property the SVG
emitter depends on: `svg_form.py` writes each filled shape using the tracked
file's own `d` text rather than re-serialising vertices, so the animated
artefact carries the artwork to the character. Geometry transformed in Python
has no `d` text to be verbatim about.

Reading the composite gives both for free. Every shape is already in one
coordinate space, so every Lottie layer keeps an identity transform — which is
also the only thing `svg_form.py` translates — and every filled path has
verbatim text behind it.

What that costs is a dependency on the lockup being *made of* the tracked
components rather than redrawn. That is not assumed: `tests/motion.rs` registers
the lockup's mark against `meridian_black.svg` and its wordmark against
`meridian_wordmark.svg`, so a redrawn or distorted lockup fails there rather
than animating quietly.

## What the file turns out to be

Nine drawable elements on a 336x84 canvas. The mark is one `<path id="prime_black">`
of six teeth. The wordmark is eight letters, M-E-R-I-D-I-A-N, each its own
element carrying its own counters as subpaths.

Two export shapes bite anyone reading this file, and both are asserted here
rather than discovered later:

**The two I's are `<rect>`, not `<path>`.** Affinity optimises axis-aligned
rectangles that way. A reader that looks only at `d` attributes drops them and
animates MER DAN without raising. They are converted to four-vertex subpaths,
which is exact rather than an approximation, and `_rect_d` writes the `d` text
that a path-shaped export would have carried.

**The mark is LAST in document order despite being leftmost.** Reading order
maps to letter order for the wordmark but not for the file as a whole, so the
mark is found by its id and the letters are ordered by their own left edges
rather than by position in the file.

## The counters are holes because of how they wind, not because anyone said so

Both renderers fill non-zero: the SVG declares `fill-rule:nonzero`, and velato
parses a fill rule into its schema and then hardcodes `Fill::NonZero` in
`runtime/vello.rs` without reading it back. Under that rule a counter is a hole
only if it winds against its outline. Every outline here winds clockwise and
every counter anticlockwise, so R, D and A come out with holes — and
`_check_windings` asserts it, because it is the one property both renderers
depend on and neither declares.
"""
import pathlib
import re

from curves import flatten
from mark import mark_paths

HERE = pathlib.Path(__file__).resolve().parent
LOCKUP_SVG = HERE.parent / "meridian-design" / "brand" / "meridian_lockup.svg"

#: The word, in reading order. Used to name layers and to check the count.
WORD = "MERIDIAN"

#: The id Affinity keeps on the mark's path. The mark is found by this rather
#: than by position, because it is last in the file and leftmost on the canvas.
MARK_ID = "prime_black"

#: Which letters carry counters, and how many subpaths each element has.
#: R is three — outline, leg and counter — because the leg is drawn as its own
#: shape and unioned by the fill rule rather than being part of the outline.
SUBPATHS = {"M": 1, "E": 1, "R": 3, "I": 1, "D": 2, "A": 2, "N": 1}


def _num(x):
    s = f"{x:.4f}".rstrip("0").rstrip(".")
    return "0" if s in ("", "-", "-0") else s


def _rect_d(x, y, w, h):
    """A `<rect>` as the `d` text a path-shaped export would have carried.

    Exact, not approximate: a rectangle is four vertices and four straight
    lines, and the relative form matches the shape of every other subpath in
    the file so the emitted SVG reads consistently.
    """
    return f"M{_num(x)},{_num(y)}l{_num(w)},0l0,{_num(h)}l{_num(-w)},0Z"


def _rect_path(x, y, w, h):
    """The same rectangle as a Lottie path — four vertices, no tangents."""
    v = [[x, y], [x + w, y], [x + w, y + h], [x, y + h]]
    return {"v": v, "i": [[0.0, 0.0]] * 4, "o": [[0.0, 0.0]] * 4, "c": True}


def _subpath_texts(d):
    """One `d` string split into per-subpath `d` strings, absolute-opened.

    The same seam `mark.mark_subpaths` cuts, and for the same reason: subpaths
    after the first open with a relative `m` taken from where `Z` left the pen,
    so a subpath drawn on its own needs its opening move rewritten as the
    absolute point the parser already computed. Everything from the first
    drawing command onwards is the file's own characters, because every later
    command in these exports is relative.
    """
    parsed = mark_paths(f' d="{d}"')
    pieces = [p for p in d.split("Z") if p.strip()]
    if len(pieces) != len(parsed):
        raise ValueError(f"{len(parsed)} parsed subpaths against {len(pieces)} "
                         "text pieces — the two readings disagree")
    out = []
    for piece, path in zip(pieces, parsed):
        at = min((i for i in (piece.find("l"), piece.find("c")) if i >= 0),
                 default=-1)
        if at < 0:
            raise ValueError("a subpath with no drawing command after its move")
        x, y = path["v"][0]
        out.append(f"M{_num(x)},{_num(y)}{piece[at:]}Z")
    return parsed, out


def _signed_area(path):
    pts = flatten(path, per=200)
    n = len(pts)
    return 0.5 * sum(pts[k][0] * pts[(k + 1) % n][1]
                     - pts[(k + 1) % n][0] * pts[k][1] for k in range(n))


def _check_windings(letters):
    """Every counter winds against its outline, so non-zero fill leaves a hole.

    Asserted rather than trusted because nothing in either artefact declares
    it. The SVG says `fill-rule:nonzero` and velato hardcodes the same rule, so
    a re-export that flipped a winding would fill R, D and A solid in both —
    consistently wrong, which is worse than one of them failing loudly.

    The largest subpath by absolute area is taken as the outline; anything
    enclosed by it must wind the other way. R's leg is the exception the rule
    needs: it winds WITH the outline because it is unioned into the letter
    rather than punched out of it, so this checks enclosure, not merely sign.
    """
    for name, paths in letters:
        if len(paths) == 1:
            continue
        areas = [_signed_area(p) for p in paths]
        outer = max(range(len(areas)), key=lambda i: abs(areas[i]))
        ox = [v[0] for v in paths[outer]["v"]]
        oy = [v[1] for v in paths[outer]["v"]]
        for i, area in enumerate(areas):
            if i == outer:
                continue
            xs = [v[0] for v in paths[i]["v"]]
            ys = [v[1] for v in paths[i]["v"]]
            enclosed = (min(xs) > min(ox) and max(xs) < max(ox)
                        and min(ys) > min(oy) and max(ys) < max(oy))
            if enclosed and (area > 0) == (areas[outer] > 0):
                raise ValueError(
                    f"{name}: an enclosed subpath winds with its outline, so "
                    "non-zero fill will not punch it out — the counter would "
                    "render solid in both the SVG and velato")


def _drawables(text):
    """Every `<path>`/`<rect>`, as (id, parsed subpaths, per-subpath `d`)."""
    out = []
    for m in re.finditer(r"<(path|rect)\b([^>]*?)/?>", text):
        tag, attrs = m.group(1), m.group(2)
        found = re.search(r'\bid="([^"]*)"', attrs)
        ident = found.group(1) if found else ""
        if tag == "rect":
            def g(k):
                hit = re.search(rf'\b{k}="([-\d.]+)"', attrs)
                if not hit:
                    raise ValueError(f"a <rect> with no {k}")
                return float(hit.group(1))
            x, y, w, h = g("x"), g("y"), g("width"), g("height")
            out.append((ident, [_rect_path(x, y, w, h)], [_rect_d(x, y, w, h)]))
            continue
        # The leading space matters. Matching `d="` alone also matches the `d`
        # in `id="`, which is how the mark's own element reads as path data
        # spelling `prime_black`.
        hit = re.search(r' d="([^"]*)"', attrs)
        if not hit:
            raise ValueError("a <path> with no d attribute")
        parsed, texts = _subpath_texts(hit.group(1))
        out.append((ident, parsed, texts))
    return out


def _load():
    text = LOCKUP_SVG.read_text()

    box = re.search(r'viewBox="0 0 ([\d.]+) ([\d.]+)"', text)
    if not box:
        raise ValueError(f"{LOCKUP_SVG.name} has no origin-anchored viewBox")
    w, h = float(box.group(1)), float(box.group(2))

    if re.search(r'transform="', text) or "<use" in text or "<text" in text:
        raise ValueError(f"{LOCKUP_SVG.name} carries a transform, a <use> or a "
                         "<text>; this reader translates none of them, and a "
                         "silently ignored one moves the artwork")

    found = _drawables(text)
    marks = [f for f in found if f[0] == MARK_ID]
    if len(marks) != 1:
        raise ValueError(f"expected one element with id={MARK_ID!r}, "
                         f"found {len(marks)}")
    _, mark_paths_, mark_texts = marks[0]
    if len(mark_paths_) != 6:
        raise ValueError(f"the mark parsed as {len(mark_paths_)} teeth, not 6")

    rest = [f for f in found if f[0] != MARK_ID]
    if len(rest) != len(WORD):
        raise ValueError(f"{len(rest)} wordmark elements against {len(WORD)} "
                         f"letters in {WORD!r}")
    # By their own left edge, not by document order: the letters happen to be
    # in reading order here, but that is a property of this export rather than
    # of SVG, and sorting makes it one the file cannot quietly lose.
    rest.sort(key=lambda f: min(v[0] for p in f[1] for v in p["v"]))

    letters, letters_d = [], []
    for glyph, (_, paths, texts) in zip(WORD, rest):
        want = SUBPATHS[glyph]
        if len(paths) != want:
            raise ValueError(f"{glyph} parsed as {len(paths)} subpaths, "
                             f"expected {want}")
        letters.append((glyph, paths))
        letters_d.append((glyph, texts))
    _check_windings(letters)

    return w, h, mark_paths_, mark_texts, letters, letters_d


CANVAS_W, CANVAS_H, MARK, MARK_D, LETTERS, LETTERS_D = _load()

#: Every filled path the lockup can draw, keyed by its parsed geometry's
#: identity, to the tracked file's own `d` text. `svg_form.py` looks shapes up
#: here so the emitted SVG carries the artwork to the character.
VERBATIM = list(zip(MARK, MARK_D)) + [
    (path, text)
    for (_, paths), (_, texts) in zip(LETTERS, LETTERS_D)
    for path, text in zip(paths, texts)
]


def bounds(paths):
    """The vertex bounding box of a list of parsed subpaths.

    Vertices only, so a curve that bulges past its endpoints is understated.
    That is the right measure for everything here — the wipe edges are driven
    by letter extents, and a letter's ink is bounded by its own vertices to
    well within a stroke width.
    """
    vs = [v for p in paths for v in p["v"]]
    return (min(v[0] for v in vs), min(v[1] for v in vs),
            max(v[0] for v in vs), max(v[1] for v in vs))


#: The mark's disc, derived from its own geometry rather than restated.
MARK_X0, MARK_Y0, MARK_X1, MARK_Y1 = bounds(MARK)
MARK_R = (MARK_X1 - MARK_X0) / 2
MARK_CX = (MARK_X0 + MARK_X1) / 2
MARK_CY = (MARK_Y0 + MARK_Y1) / 2

#: The wordmark's ink, and the gap the lockup puts between the two.
WORD_X0, WORD_Y0, WORD_X1, WORD_Y1 = bounds([p for _, ps in LETTERS for p in ps])
GAP = WORD_X0 - (MARK_CX + MARK_R)
