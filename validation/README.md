# Validation

Colour maths runs in CI, never by eye. Two gates land in Phase 1:

## Palette validator

Checks every shipped palette in BOTH modes against the Meridian surfaces:
lightness band, chroma floor, adjacent-pair CVD separation (OKLab ΔE, ≥ 8
target), the normal-vision floor (≥ 15, hard fail), and surface contrast.
Categorical slot *ordering* is derived by enumerating orderings for maximum
minimum adjacent CVD ΔE — the order is a safety mechanism, not cosmetics.
Validation output is checked in beside the palette; CI re-runs and diffs it.

Implementation note: written here as our own script over the standard,
well-published maths (OKLab ΔE, CVD simulation matrices, WCAG contrast), so
the repo owns its gate end-to-end.

## Scale generator

`generateRadixColors` — vendored from the MIT-licensed
[radix-ui/website](https://github.com/radix-ui/website) repo
(`components/generate-radix-colors.tsx`; deps: colorjs.io, bezier-easing,
@radix-ui/colors) — generates the 12-step neutral/accent/semantic scales from
Meridian seeds (Maritime accent, hue-70 warm cream gray). Run offline; outputs
are committed into `meridian-design`, never computed at build time (ADR 0007).
