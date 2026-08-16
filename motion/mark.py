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


MARK = mark_paths(MARK_SVG.read_text())
if len(MARK) != 6:
    raise SystemExit(f"expected 6 teeth in {MARK_SVG.name}, parsed {len(MARK)}")
