"""Colour tokens, read from the crate's emitted CSS rather than retyped.

`meridian-design/tests/snapshots/tokens.css` is the artefact `emit::tokens_css`
produces, and it is pinned byte-for-byte by a conformance test. That makes it
the one file in this repository where every token value is both current and
guaranteed not to have drifted — which is exactly what a generator wants, and
why this reads it instead of carrying a copy.

The rule it keeps is the repository's own: token values are defined in the
crate and nowhere else. A generator with its own idea of what Maritime is will
be wrong the first time the palette moves, and nothing will say so.

Two themes, because the CSS carries two: `:root` is light and `.dark` is dark.
Values come back as Lottie colours — `[r, g, b, a]` floats in 0..1.
"""
import pathlib
import re

CSS = (pathlib.Path(__file__).resolve().parent.parent / "meridian-design"
       / "tests" / "snapshots" / "tokens.css")


def _rgba(hex_text):
    """`#rrggbb` or `#rrggbbaa` -> Lottie's [r, g, b, a] in 0..1."""
    h = hex_text.lstrip("#")
    if len(h) not in (6, 8):
        raise ValueError(f"not a hex colour: {hex_text!r}")
    parts = [int(h[i:i + 2], 16) / 255 for i in range(0, len(h), 2)]
    return parts if len(parts) == 4 else parts + [1.0]


def _load():
    """Both themes, with dark INHERITING from light.

    This is the CSS cascade, not a convenience. `.dark` redefines a subset of
    `:root` — the chrome and the 12-step scales — and deliberately leaves the
    rest alone: the sequential and diverging chart ramps have one definition
    each, because a data ramp is the same ramp whichever theme surrounds it.
    Reading the two blocks as independent dictionaries makes those ramps look
    absent in dark, which they are not.
    """
    text = CSS.read_text()

    def block(selector):
        found = re.search(selector + r"\s*\{(.*?)\n\}", text, re.S)
        if not found:
            raise ValueError(f"{CSS.name} has no {selector} block")
        return dict(re.findall(r"--([\w-]+):\s*([^;]+);", found.group(1)))

    light = block(":root")
    return {"light": light, "dark": {**light, **block(r"\.dark")}}


_TOKENS = _load()


def colour(name, theme="light"):
    """A token as a Lottie colour. `name` is without the `--` prefix.

    Raises rather than falling back, because a silent default here would be a
    generator quietly inventing a brand colour.
    """
    try:
        raw = _TOKENS[theme][name]
    except KeyError:
        raise KeyError(f"{CSS.name} ({theme}) has no --{name}") from None
    if not raw.startswith("#"):
        raise ValueError(f"--{name} is {raw!r}, which is not a colour")
    return _rgba(raw)


def ramp(prefix, steps, theme="light"):
    """A list of tokens by step number — `ramp('m-maritime', [9, 7, 5, 3])`."""
    return [colour(f"{prefix}-{n}", theme) for n in steps]


def hex_of(name, theme="light"):
    """The raw hex, for printing what a build actually used."""
    return _TOKENS[theme][name]
