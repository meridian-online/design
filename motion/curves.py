"""The two curves this animation is made of, evaluated once for every reader.

Lottie stores both as cubic beziers, and both are read in two places:

* an **ease** — the two control points of a unit cubic, `o` the out-handle and
  `i` the in-handle, both carried on the *first* keyframe of a segment.
  `build_form.py` needs it to derive the schedule before any document exists,
  since the frame an S becomes visible is a property of its ramp rather than of
  its path; `svg_form.py` needs it to sample that document back out, because
  CSS cannot express "the same ease, offset in time" as one animated value.
* a **path** — the vertices and tangents of a drawn curve. `build_form.py`
  flattens it to find where a stroke enters and leaves the disc; `svg_form.py`
  flattens it to say how far its own sampling sits off the Lottie's, in canvas
  units rather than in fractions of a path nobody has measured.

Both readings have to be the same reading, or the two emitted artefacts drift
in a way no snapshot would catch: each would be self-consistent.
"""


def ease_at(u, eo, ei):
    """The value of a Lottie ease at normalised time `u`, in 0..1.

    Bisects x(p) = u rather than solving it, because sixty halvings is exact to
    a part in 10^18 and this runs a few thousand times in a build.
    """
    ox, oy, ix, iy = eo["x"][0], eo["y"][0], ei["x"][0], ei["y"][0]
    lo, hi = 0.0, 1.0
    for _ in range(60):
        p = (lo + hi) / 2
        if 3 * (1 - p) ** 2 * p * ox + 3 * (1 - p) * p * p * ix + p ** 3 < u:
            lo = p
        else:
            hi = p
    p = (lo + hi) / 2
    return 3 * (1 - p) ** 2 * p * oy + 3 * (1 - p) * p * p * iy + p ** 3


def ease_when(y, eo, ei):
    """The normalised time at which that ease reaches value `y`."""
    lo, hi = 0.0, 1.0
    for _ in range(60):
        m = (lo + hi) / 2
        lo, hi = (m, hi) if ease_at(m, eo, ei) < y else (lo, m)
    return (lo + hi) / 2


def flatten(path, per=400):
    """A Lottie path `{v, o, i, c}` as a polyline, `per` samples per segment.

    Lottie holds tangents as offsets from their own vertex, so the control
    points are recovered before each cubic is walked.
    """
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


def length(path):
    """The flattened arc length of a Lottie path, in its own units."""
    pts = flatten(path)
    return sum(((pts[a][0] - pts[a - 1][0]) ** 2
                + (pts[a][1] - pts[a - 1][1]) ** 2) ** 0.5
               for a in range(1, len(pts)))
