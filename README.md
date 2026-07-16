# Meridian Design System

The single source of truth for how Meridian looks, reads, and feels — across the
web ([meridian-online/web](https://github.com/meridian-online/web)), the
Brightfield desktop app
([meridian-online/brightfield](https://github.com/meridian-online/brightfield)),
and the chart ink both of them render.

## What lives here

| Path | Contents |
|---|---|
| `meridian-design/` | The token crate — MIT, dependency-free, framework-neutral Rust. Colours, type ramp, spacing, chart palettes. The **only** place token values are defined. |
| `decisions/` | Architecture decision records. The 16 scoping decisions that shaped the system are ADRs 0001–0010. |
| `guidelines/` | Written principles: identity, density, speed budgets, colour method, typography, icons. |
| `validation/` | Palette gates — colour maths runs in CI, never by eye. |

## How it is consumed

- **Web** takes the emitted `tokens.css` (CSS custom properties, light + dark),
  pinned by a conformance test.
- **Brightfield** takes `meridian-design` as a cargo dependency. The render
  crate (gpui-free) reads token values directly; the app shell applies the
  emitted gpui-component `ThemeConfig` JSON pair.
- **Future Linebender/Masonry chrome** consumes the same crate — tokens are
  plain `Copy` structs with framework-neutral sRGB colours and logical-pixel
  dimensions precisely so no re-translation is needed at migration time
  (see ADR 0003).

## Status

Phase 0 (scaffold + decisions) — see [ROADMAP.md](ROADMAP.md). Token values
land in Phase 1; until then the crate compiles but carries only the brand
signature constant and the type shapes.
