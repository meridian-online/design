"""`form`, re-emitted as animated SVG from the Lottie that ships beside it.

ADR 0012 splits the formats by consumer: the desktop takes Lottie and renders
it through velato, and **the web takes animated SVG**, because `lottie-web` is
~75 KB gzipped of JavaScript and the site ships no motion library at all. Same
motion, expressed twice, from one source. This is the second expression.

## It translates the document; it does not re-choreograph it

Everything here is read out of the built Lottie: the paths, the trim schedule,
the colours, the stroke width, the frame rate, the loop length. Nothing about
the animation is stated twice. That is the whole design of this file, and the
reason is the one `CLAUDE.md` gives for the crate's emitters — an emitter that
restates a value instead of reading it makes editing that value a silent no-op,
and a snapshot pinned against the emitter's own output cannot notice. Here the
value at risk is the *schedule*: change `SEAM_OVERLAP` in a re-choreographing
emitter and one artefact moves while the other does not, and both still look
plausible on their own.

The mark is the exception, and in the same direction: the teeth come from
`mark.MARK_D`, which is the tracked SVG's own `d` text, so the animated file
carries the mark to the character rather than to four decimal places.

## It is a translator for what `build_form.py` emits, not a Lottie renderer

Anything outside that subset **raises**. A skipped layer is the failure mode
that would leave the two artefacts disagreeing while each still rendered, so
the emitter refuses rather than doing its best.

Supported: shape layers carrying a trimmed stroked path or filled paths;
precomp references with an identity transform; alpha track mattes (`td`/`tt`)
whose matte is a rectangle or an ellipse. Not supported, and asserted against:
opacity/rotation/scale animation, non-identity precomp transforms, masks,
gradients, repeaters, and any keyframe interpolation other than the two-point
bezier `track()` emits and the linear sampling `turn()` emits.

## How the mechanisms map

**A trim pair becomes a dash.** `pathLength="1"` normalises every path to unit
length, so trim percentages are dash fractions directly and no measurement has
to agree between this file and the renderer. A trim from `s` to `e` is
`stroke-dasharray: e-s 1` with `stroke-dashoffset: -s`: one dash, and a gap
long enough that the pattern cannot repeat inside a path of length 1.

The dash *length* is the only thing here that cannot be written as a single
eased ramp — it is the difference of two copies of one ramp offset in time,
which is exactly the mechanism the whole animation is built on and exactly what
a CSS timing function cannot express. So it is sampled, one stop per frame,
linear between. `build_form.turn()` already establishes that sampling as the
house answer for a curve a keyframe pair cannot carry, and one stop per frame
is where it stops because that is the composition's own time resolution. The
build prints what the compromise costs: a dash end lands at most ~1.6 units
from where the Lottie's continuous ease puts it, on a 600-unit canvas, against
a stroke 12 units wide.

**A track matte becomes a clip.** The disc is a `<clipPath>` circle. Each row's
wipe is a `<clipPath>` rectangle translated along x, sampled at the same nine
points `turn()` uses — linear between them there is not an approximation, it is
what the Lottie does.

**Layer order reverses.** `layers[0]` is topmost in Lottie; in SVG the last
element painted wins.

**A wake ghost is one animation with a negative delay.** Every ghost is the
same schedule shifted in time, so its keyframes are written once and each
element phase-shifts into them. The delay is negative — `-(loop - ip)` rather
than `+ip` — so a delayed element is already in its cycle on the first pass
instead of sitting in its unanimated state until the second. It also means the
file *says* the wake is the stroke's own past, which a per-element expansion
would bury.

## Reduced motion is one rule, and it works by construction

`prefers-reduced-motion: reduce` turns every animation off and nothing else.
What is left is the state the elements carry as attributes: strokes are
`opacity="0"`, and an unanimated wipe rectangle sits at its untranslated
position, which covers the whole canvas. So the still frame is the assembled
mark — the thing the motion was arriving at. `guidelines/speed.md` requires the
still state to say what the motion said; a surface using this as the >100 ms
work cue owes the reader a textual cue as well, because a static mark says the
brand and not that anything is happening.

That fallback holds one step further out: strip the `<style>` block entirely
and the attributes still resolve to the same assembled mark.
"""
import json

from curves import ease_at, length
from mark import MARK, MARK_D

#: Everything an element can be told to do, and nothing else. A shape group
#: whose item types are not one of these is refused by `shape_group`.
STROKE_TRIM = ("sh", "tm", "st", "tr")
FILLED = ("sh", "fl", "tr")


def num(x, places=3):
    """A number for an SVG attribute: fixed places, then trimmed."""
    s = f"{x:.{places}f}".rstrip("0").rstrip(".")
    return "0" if s in ("", "-", "-0") else s


def hex_of(colour):
    """A Lottie `[r, g, b, a]` as `#rrggbb`, opaque only.

    Alpha is refused rather than dropped: every colour in this animation is
    opaque by design — the geometry arrives from nothing and leaves to nothing
    on its own, and a translucent one would mean that stopped being true.
    """
    r, g, b, a = colour
    if abs(a - 1.0) > 1e-6:
        raise ValueError(f"translucent colour {colour} — the emitter draws opaque ink")
    return "#" + "".join(f"{round(c * 255):02x}" for c in (r, g, b))


def value_at(track, frame):
    """A Lottie animated scalar or vector, sampled at `frame`.

    Handles the two shapes `build_form.py` emits: a static value, and a
    keyframe list interpolated by the pair of bezier handles on the earlier
    keyframe of each segment (`LIN_I`/`LIN_O` making that segment linear).
    """
    if not track.get("a"):
        k = track["k"]
        return k if isinstance(k, list) else [k]
    kfs = track["k"]
    if frame <= kfs[0]["t"]:
        return kfs[0]["s"]
    if frame >= kfs[-1]["t"]:
        return kfs[-1]["s"]
    for a in range(len(kfs) - 1):
        k0, k1 = kfs[a], kfs[a + 1]
        if k0["t"] <= frame <= k1["t"]:
            u = (frame - k0["t"]) / (k1["t"] - k0["t"])
            w = ease_at(u, k0["o"], k0["i"])
            return [p + (q - p) * w for p, q in zip(k0["s"], k1["s"])]
    raise ValueError(f"frame {frame} fell outside its own keyframe list")


def path_d(path):
    """A Lottie bezier `{v, o, i, c}` as SVG path data.

    Lottie stores tangents as offsets from their own vertex; SVG wants absolute
    control points. That difference is the whole conversion.
    """
    v, o, i, closed = path["v"], path["o"], path["i"], path["c"]
    n = len(v)
    out = [f"M{num(v[0][0])},{num(v[0][1])}"]
    for a in range(n if closed else n - 1):
        b = (a + 1) % n
        c1 = (v[a][0] + o[a][0], v[a][1] + o[a][1])
        c2 = (v[b][0] + i[b][0], v[b][1] + i[b][1])
        out.append(f"C{num(c1[0])},{num(c1[1])} {num(c2[0])},{num(c2[1])} "
                   f"{num(v[b][0])},{num(v[b][1])}")
    if closed:
        out.append("Z")
    return "".join(out)


def only(items, ty):
    """The single item of type `ty` in a shape group, or a clear failure."""
    found = [it for it in items if it["ty"] == ty]
    if len(found) != 1:
        raise ValueError(f"expected exactly one {ty!r} in a shape group, got {len(found)}")
    return found[0]


def static(track, what):
    """Assert a transform channel is not animated, and return it."""
    if track.get("a"):
        raise ValueError(f"{what} is animated; the emitter does not translate that")
    return track["k"]


class Emitter:
    """One SVG document, built from one Lottie document.

    Keyframe bodies are deduplicated as they are produced: two elements whose
    schedules differ only by a shift write the same body once the percentages
    are taken relative to each element's own `ip`, so they share a `@keyframes`
    block and differ by `animation-delay` alone. That is not a size trick — it
    is the wake's definition, kept legible in the output.
    """

    def __init__(self, doc, verbatim=None):
        self.doc = doc
        #: (parsed path, its tracked `d` text) for every fillable shape this
        #: document may draw. Defaults to the mark alone, which is what `form`
        #: needs; `build_lockup.py` passes the mark and the wordmark together.
        self.verbatim = list(zip(MARK, MARK_D)) if verbatim is None else verbatim
        self.fps = doc["fr"]
        self.loop = doc["op"]
        # AN INLINED SVG IS NOT SANDBOXED. Its ids join the host page's
        # namespace and its `<style>` applies to the whole document, so an
        # artefact meant for `<img>` still has to behave when someone pastes it
        # into a page — which, given `brand/README.md`'s history, someone will.
        # Three consequences, all handled by this one name:
        #
        #   * `clipPath` ids are prefixed, so light and dark coexist
        #   * every rule is scoped under the root's id, so `.s` cannot reach a
        #     host element that happens to share the class
        #   * `@keyframes` names are prefixed, because those stay global however
        #     the selectors are scoped
        #
        # Two copies of the SAME file inlined still collide on ids. That is
        # inherent to inlining and is why `<img>` is what the README recommends.
        self.slug = "meridian-" + doc["nm"].replace("_", "-")
        if doc["ip"] != 0:
            raise ValueError("the composition is assumed to start at frame 0")
        self.assets = {a["id"]: a for a in doc.get("assets", [])}
        self.blocks = {}        # keyframe body -> its name
        self.rules = {}         # class body -> its name
        self.defs = []
        self.width = int(doc["w"])
        self.height = int(doc["h"])
        self.teeth_seen = set()
        self.max_sampling_error = 0.0

    # ---------------------------------------------------------------- naming

    def keyframes(self, stops, lead=None):
        """Register a `@keyframes` block, returning the name to animate with.

        `stops` is `[(percent, declarations)]`. **Both ends are closed here,
        and that is not tidiness.** A block with no `100%` keyframe does not
        hold its last value: CSS synthesises the missing keyframe from the
        element's *underlying* value and interpolates toward it. Left open,
        every stroke's dash ran back toward `none` — an undashed path, so a
        solid stroke — and every wipe drifted back toward untranslated, which
        covers the whole canvas, so all three rows of the mark stood on screen
        through the orbit. Both artefacts had the same schedule and only one
        of them was showing it.

        `lead` is the value to hold *before* the first stop, for a block whose
        window opens partway through the loop. Without it the synthesised `0%`
        is the underlying value again, which is right only by accident.

        Blocks are deduplicated by body: two elements whose schedules differ
        only by a shift produce the same text once the percentages are taken
        from their own starts, so the wake shares its leader's keyframes and
        differs by `animation-delay` alone.
        """
        if lead is not None and stops[0][0] > 1e-9:
            stops = [(0.0, lead)] + stops
        if stops[-1][0] < 100.0 - 1e-9:
            stops = stops + [(100.0, stops[-1][1])]
        body = "".join(f"{num(pct, 4)}%{{{decl}}}" for pct, decl in stops)
        if body not in self.blocks:
            self.blocks[body] = f"{self.slug}-k{len(self.blocks)}"
        return self.blocks[body]

    def rule(self, declarations):
        """Register a class body, returning its selector name."""
        if declarations not in self.rules:
            self.rules[declarations] = f"a{len(self.rules)}"
        return self.rules[declarations]

    def pct(self, frame, anchor):
        """A keyframe percentage, measured from an element's own start."""
        p = 100.0 * (frame - anchor) / self.loop
        if p < -1e-9 or p > 100.0 + 1e-9:
            raise ValueError(f"frame {frame} lies outside the loop from {anchor}")
        return p

    def delay(self, anchor):
        """The negative delay that puts an element `anchor` frames behind.

        `-(loop - anchor)` rather than `+anchor`: a positive delay would leave
        the element in its unanimated state for its first `anchor` frames,
        which for a ghost means the whole wake missing from the first pass.
        """
        return f"{num(-(self.loop - anchor) / self.fps, 4)}s"

    def animate(self, names, timings, anchor, also=""):
        """The `class` and `style` an animated element carries.

        A single `animation-delay` covers every animation in the shorthand
        list: CSS repeats a short comma list to match a longer one, and the
        two animations on a stroke — its dash and its visibility — are the same
        schedule read two ways, so they want the same shift.
        """
        dur = num(self.loop / self.fps, 4)
        body = "animation:" + ",".join(f"{n} {dur}s {t} infinite"
                                       for n, t in zip(names, timings))
        cls = " ".join(filter(None, (also, self.rule(body))))
        return f'class="{cls}" style="animation-delay:{self.delay(anchor)}"'

    # ----------------------------------------------------------------- clips

    def clip(self, matte):
        """A `<clipPath>` for a track-matte layer, returning its id."""
        if matte.get("tt"):
            raise ValueError("a matte layer that is itself matted")
        items = matte["shapes"][0]["it"]
        only(items, "fl")                       # a matte is a filled shape
        cid = f"{self.slug}-c{len(self.defs)}"
        kinds = {it["ty"] for it in items}

        if "el" in kinds:
            el = only(items, "el")
            if static(matte["ks"]["p"], "an ellipse matte's position")[:2] != [0, 0]:
                raise ValueError("an ellipse matte is expected to be placed by its own p")
            cx, cy = static(el["p"], "an ellipse matte's centre")
            w, h = static(el["s"], "an ellipse matte's size")
            if abs(w - h) > 1e-9:
                raise ValueError("an elliptical matte; this emitter draws circles")
            shape = f'<circle cx="{num(cx)}" cy="{num(cy)}" r="{num(w / 2)}"/>'

        elif "rc" in kinds:
            rc = only(items, "rc")
            if static(rc["r"], "a rectangle matte's corner radius") != 0:
                raise ValueError("a rounded matte; this emitter draws square corners")
            px, py = static(rc["p"], "a rectangle matte's centre")[:2]
            w, h = static(rc["s"], "a rectangle matte's size")
            shape = self.wipe_rect(matte, px, py, w, h)

        else:
            raise ValueError(f"a matte made of {sorted(kinds)}, which is not a rect or an ellipse")

        self.defs.append(f'<clipPath id="{cid}">{shape}</clipPath>')
        return cid

    def wipe_rect(self, matte, px, py, w, h):
        """The travelling rectangle that reveals one row of the mark.

        Its y never moves, so that half of the layer's position is folded into
        the rectangle and only x is animated. The sample points are the
        Lottie's own keyframes rather than a re-sampling of them: `turn()`
        writes a half-cosine as nine linear segments, so reproducing those
        segments *is* reproducing the motion, not approximating it.
        """
        kfs = matte["ks"]["p"]["k"]
        if not matte["ks"]["p"].get("a"):
            raise ValueError("a wipe matte that does not move")
        ys = {round(kf["s"][1], 6) for kf in kfs}
        if len(ys) != 1:
            raise ValueError("a wipe matte that moves in y; the emitter folds y in")
        for kf in kfs[:-1]:
            if kf["i"] != {"x": [1], "y": [1]} or kf["o"] != {"x": [0], "y": [0]}:
                raise ValueError("a wipe keyframe that is not linear")
        y0 = kfs[0]["s"][1]

        anchor = matte["ip"]
        name = self.keyframes([
            (self.pct(kf["t"], anchor),
             f'transform:translate({num(kf["s"][0])}px,0)') for kf in kfs])
        attrs = self.animate([name], ["linear"], anchor)
        return (f'<rect x="{num(px - w / 2)}" y="{num(py + y0 - h / 2)}" '
                f'width="{num(w)}" height="{num(h)}" {attrs}/>')

    # ---------------------------------------------------------------- layers

    def layers(self, layers):
        """A layer list, folded and reversed into SVG paint order."""
        out, matte = [], None
        for lay in layers:
            if lay.get("td"):
                if matte is not None:
                    raise ValueError("two matte layers in a row")
                matte = lay
                continue
            body = self.layer(lay)
            if lay.get("tt"):
                if matte is None:
                    raise ValueError("a matted layer with no matte above it")
                if lay["tt"] != 1:
                    raise ValueError(f"track matte mode {lay['tt']}; only alpha (1) is translated")
                body = f'<g clip-path="url(#{self.clip(matte)})">{body}</g>'
                matte = None
            out.append(body)
        if matte is not None:
            raise ValueError("a matte layer with nothing beneath it to matte")
        return "".join(reversed(out))

    def layer(self, lay):
        if lay["ty"] == 0:
            return self.precomp(lay)
        if lay["ty"] != 4:
            raise ValueError(f"layer type {lay['ty']}; only shape and precomp layers are translated")
        ks = lay["ks"]
        if static(ks["o"], "a layer's opacity") != 100:
            raise ValueError("a layer with baked-in opacity")
        for chan, want in (("r", 0), ("p", [0, 0, 0]), ("a", [0, 0, 0]), ("s", [100, 100, 100])):
            if static(ks[chan], f"a shape layer's {chan}") != want:
                raise ValueError(f"a shape layer with a non-identity {chan}")
        if len(lay["shapes"]) != 1 or lay["shapes"][0]["ty"] != "gr":
            raise ValueError("a shape layer that is not one group")
        return self.shape_group(lay, lay["shapes"][0]["it"])

    def precomp(self, lay):
        """A precomp reference, inlined.

        Lottie centres a precomp by setting `p` and `a` to the same point, so
        the two cancel; anything else is a placement this emitter would have to
        express as a transform, and it refuses instead.
        """
        ks = lay["ks"]
        p, a = static(ks["p"], "a precomp's position"), static(ks["a"], "a precomp's anchor")
        if p != a:
            raise ValueError("a precomp placed somewhere other than its own anchor")
        if static(ks["s"], "a precomp's scale") != [100, 100, 100]:
            raise ValueError("a scaled precomp")
        if lay["w"] != self.width or lay["h"] != self.height:
            raise ValueError("a precomp on a different canvas from the composition")
        asset = self.assets.get(lay["refId"])
        if asset is None:
            raise ValueError(f"missing asset {lay['refId']!r}")
        return self.layers(asset["layers"])

    def shape_group(self, lay, items):
        kinds = tuple(dict.fromkeys(it["ty"] for it in items))
        if kinds == STROKE_TRIM:
            return self.stroked(lay, items)
        if kinds == FILLED:
            return self.filled(items)
        raise ValueError(f"a shape group of {kinds}, which the emitter does not translate")

    def stroked(self, lay, items):
        """One trimmed stroke: an S, or one lap of the orbit."""
        sh, tm, st = only(items, "sh"), only(items, "tm"), only(items, "st")
        if static(tm["o"], "a trim's offset") != 0 or tm["m"] != 1:
            raise ValueError("a trim with an offset or a per-shape mode")
        if st["lc"] != 2 or st["lj"] != 2:
            raise ValueError("a stroke that is not round-capped and round-joined")
        if static(st["o"], "a stroke's opacity") != 100:
            raise ValueError("a translucent stroke")
        width = static(st["w"], "a stroke's width")
        colour = hex_of(static(st["c"], "a stroke's colour"))

        anchor, ip, op = lay["ip"], int(lay["ip"]), int(lay["op"])
        path = static(sh["ks"], "a stroke path")
        dash, seen = [], []
        for f in range(ip, op + 1):
            tail = value_at(tm["s"], f)[0] / 100.0
            head = value_at(tm["e"], f)[0] / 100.0
            if head < tail - 1e-9:
                raise ValueError("a trim whose head falls behind its tail")
            dash.append((f, head - tail, -tail))
            seen.append(head - tail > 1e-6)
        self.note_sampling_error(tm, ip, op, path)

        # A zero-length dash under a round cap is a dot of the full stroke
        # width — that is how dotted lines are drawn in SVG — so the visible
        # window is the frames the dash actually has length, not the layer's
        # own bounds. Same defect the Lottie hit at `op`, same fix, one format
        # further on.
        if not any(seen):
            raise ValueError("a stroke that is never visible")
        first, last = seen.index(True), len(seen) - 1 - seen[::-1].index(True)

        trim = self.keyframes([
            (self.pct(f, anchor), f"stroke-dasharray:{num(span, 5)} 1;"
                                  f"stroke-dashoffset:{num(offset, 5)}")
            for f, span, offset in dash])
        show = self.keyframes([(self.pct(ip + first, anchor), "opacity:1"),
                               (self.pct(ip + last + 1, anchor), "opacity:0")],
                              lead="opacity:0")
        attrs = self.animate([trim, show], ["linear", "step-end"], anchor, also="s")
        return (f'<path {attrs} d="{path_d(path)}" pathLength="1" '
                f'stroke="{colour}" stroke-width="{num(width)}" opacity="0"/>')

    def filled(self, items):
        """One beat's artwork: its paths, in one ink, under a wipe.

        The paths come out of the tracked SVG's own `d` text rather than being
        re-serialised from the Lottie's vertices, so the animated file carries
        the artwork to the character. Which path each shape is comes from
        matching its geometry against the parsed source, not from its layer
        name: a name can be edited without the drawing changing.

        `verbatim` is what makes this emitter serve more than one asset. For
        `form` it is the mark's six teeth from `mark.py`; for the lockup it is
        the mark AND the eight letters, from `lockup.py`. A shape whose
        geometry is in neither raises, because a path this emitter cannot name
        is one it would otherwise have to re-serialise — and a re-serialised
        artefact is no longer tied to the file it came from.
        """
        fl = only(items, "fl")
        if static(fl["o"], "a fill's opacity") != 100 or fl["r"] != 1:
            raise ValueError("a translucent or even-odd fill")
        colour = hex_of(static(fl["c"], "a fill's colour"))
        d = []
        for it in items:
            if it["ty"] != "sh":
                continue
            path = static(it["ks"], "a filled path")
            for known, text in self.verbatim:
                if path == known:
                    d.append(text)
                    break
            else:
                raise ValueError("a filled path that is not in the tracked "
                                 "artwork this emitter was given")
        if not d:
            raise ValueError("a fill with no paths")
        d = "".join(d)

        # Which copy of this beat is this? Lottie order runs leader first, so
        # the first time a set of paths appears it is the ink and every later
        # appearance is a ghost. Only the reduced-motion rule needs to know:
        # with the animation off, every copy is fully revealed and stacked on
        # identical geometry, and the leader covers the wake everywhere except
        # along its own antialiased edge — which left the still mark with a
        # measurable blue fringe, 616 pixels of it at 300px. The still frame is
        # the artwork, so the ghosts stand down.
        ghost = d in self.teeth_seen
        self.teeth_seen.add(d)
        return f'<path{" class=\"w\"" if ghost else ""} d="{d}" fill="{colour}"/>'

    # ------------------------------------------------------------ diagnostics

    def note_sampling_error(self, tm, ip, op, path):
        """How far one-stop-per-frame sampling sits off the continuous ease.

        Reported rather than assumed, and in canvas units rather than in
        fractions of a path: a dash end that lands 1% along a path nobody has
        measured is not a number anyone can judge. Against a stroke 12 units
        wide on a 600-unit canvas, it is.

        The compromise itself is unavoidable. A dash *length* is the difference
        of two eased ramps offset in time — the mechanism the whole animation
        is built on — and no CSS timing function expresses a difference. One
        stop per frame is where the sampling stops because that is the
        composition's own time resolution, not because it was far enough.
        """
        span = length(path)
        for track in (tm["s"], tm["e"]):
            for f in range(ip, op):
                for u in (0.25, 0.5, 0.75):
                    exact = value_at(track, f + u)[0]
                    lerp = (value_at(track, f)[0] * (1 - u)
                            + value_at(track, f + 1)[0] * u)
                    self.max_sampling_error = max(self.max_sampling_error,
                                                  abs(exact - lerp) / 100.0 * span)

    # -------------------------------------------------------------- document

    def render(self):
        """The document, with every selector scoped to its own root.

        `#slug` on the front of each rule is what keeps an inlined copy from
        restyling the page around it — including the reduced-motion rule, which
        is a `*` selector and would otherwise stop every animation in the host
        document rather than the ones in this file.
        """
        body = self.layers(self.doc["layers"])
        at = f"#{self.slug}"
        css = [f"{at} .s{{fill:none;stroke-linecap:round;stroke-linejoin:round}}"]
        for declarations, name in self.rules.items():
            css.append(f"{at} .{name}{{{declarations}}}")
        for keyframe_body, name in self.blocks.items():
            css.append(f"@keyframes {name}{{{keyframe_body}}}")
        # The still frame falls out of the attributes the elements already
        # carry: strokes are opacity="0", and an untranslated wipe rectangle
        # covers the whole canvas. What is left is the mark, once the ghost
        # copies of each row stand down — see `filled`.
        css.append(f"@media(prefers-reduced-motion:reduce){{"
                   f"{at} *{{animation:none!important}}{at} .w{{display:none}}}}")
        return (
            f'<svg xmlns="http://www.w3.org/2000/svg" id="{self.slug}" '
            f'viewBox="0 0 {self.width} {self.height}" width="{self.width}" '
            f'height="{self.height}" role="img" aria-label="Meridian">'
            f"<title>Meridian</title>"
            f'<style>{"".join(css)}</style>'
            f"<defs>{''.join(self.defs)}</defs>"
            f"{body}</svg>\n")


def emit(doc, verbatim=None):
    """One Lottie document as one animated SVG, plus its sampling error.

    `verbatim` names the tracked artwork the document is allowed to draw; see
    `Emitter.filled`. Omitted, it is the mark.
    """
    e = Emitter(doc, verbatim)
    svg = e.render()
    return svg, e.max_sampling_error


if __name__ == "__main__":
    import pathlib
    import sys

    for arg in sys.argv[1:]:
        src = pathlib.Path(arg)
        out, err = emit(json.loads(src.read_text()))
        dst = src.with_suffix(".svg")
        dst.write_text(out)
        print(f"{dst.name:<24} {len(out) / 1024:>5.1f} KB  sampling {err * 100:.4f}%")
