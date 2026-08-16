"""The Meridian prime mark, parsed from the tracked SVG into Lottie paths.

One copy of this, shared by every generator here. A second copy of the mark is
the exact defect `meridian-design/brand/README.md` documents: the web app
inlined the mark's path as source and its copy drifted from the tracked file by
a byte before anyone noticed. A generator with a private mark drifts the same
way, and nothing says so.

It is also where a licence boundary sits. The mark is a trademark reserved
outside this repository's MIT grant, so the geometry stays in `brand/` under
the terms that govern it, and this borrows it at build time.
"""
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
MARK_SVG = HERE.parent / "meridian-design" / "brand" / "meridian_black.svg"

#: The mark's design canvas and outer radius, in its own units.
VIEWBOX = 600.0
RADIUS = 300.0


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


def mark_subpaths(svg_text, parsed):
    """The same six teeth, kept as the file's own path text where it can be.

    The SVG emitter wants the mark verbatim rather than re-serialised: a round
    trip through floats would carry geometry that is *equivalent* to the
    tracked file, which is not a property any test can tie back to it. Text
    that came out of the file is.

    One thing has to change, and only one. The six subpaths are a chain — the
    first opens `M300,100` and the other five open with a relative `m`, taken
    from where `Z` left the pen, which is the previous subpath's start. That
    chain is broken the moment a row draws teeth 5 and 4 as its own element,
    so each subpath's opening move is rewritten as the absolute point the
    parser already computed for it. **Everything after that first move is the
    file's own characters**, because every later command in the mark is
    relative and so cannot depend on where the subpath began.

    That is the same seam `brand.rs` compares the padded mark across — text
    from the first `l` onwards — and `tests/motion.rs` compares this one the
    same way.
    """
    found = re.search(r' d="([^"]*)"', svg_text)
    if not found:
        raise ValueError(f"{MARK_SVG.name} has no path data")
    pieces = [piece for piece in found.group(1).split("Z") if piece.strip()]
    if len(pieces) != len(parsed):
        raise ValueError(f"{MARK_SVG.name}: {len(parsed)} parsed teeth against "
                         f"{len(pieces)} subpaths — the two readings disagree")

    out = []
    for piece, path in zip(pieces, parsed):
        at = piece.find("l")
        if at < 0:
            raise ValueError("a subpath with no relative line; the mark has one each")
        x, y = path["v"][0]
        out.append(f"M{_num(x)},{_num(y)}{piece[at:]}Z")
    return out


def _num(x):
    s = f"{x:.4f}".rstrip("0").rstrip(".")
    return "0" if s in ("", "-", "-0") else s


_SVG_TEXT = MARK_SVG.read_text()

#: The six teeth as Lottie paths, for the animation's geometry.
MARK = mark_paths(_SVG_TEXT)
if len(MARK) != 6:
    raise SystemExit(f"expected 6 teeth in {MARK_SVG.name}, parsed {len(MARK)}")

#: The same six as SVG path data, each standing on its own.
MARK_D = mark_subpaths(_SVG_TEXT, MARK)
