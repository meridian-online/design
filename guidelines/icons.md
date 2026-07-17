# Icons

**Tabler everywhere; framework internals exempt** (ADR 0009 + update).

## Rules

- **Tabler is the icon language** — outline style primary, filled variants
  for selected/toggle states. 24px grid, 2px stroke, `currentColor`.
- **Framework internals keep their bundled icons.** Vendored shadcn
  primitives and fumadocs UI import `lucide-react` internally (chevrons,
  checks, ✕ — visually identical to Tabler). `lucide-react` therefore stays
  a dependency and must never be "cleaned up"; it is not a second icon set,
  it is component plumbing.
- **One binding module per surface.** Icon names resolve to components in
  exactly one place — on the web, `lib/icon-map.tsx` binds the finetype
  type-icon vocabulary (kebab names → Tabler components, null → the
  domain-coloured dot). Never bind names ad hoc at call sites. If the
  desktop ever shows type icons, the name→Tabler mapping lifts into this
  repo as shared data.
- **Rust ingest**: strip Tabler's invisible bounding-rect path, flatten to
  kurbo paths; stroke-width stays a live attribute.
- **Web build**: keep `optimizePackageImports` for `@tabler/icons-react`
  (6k-module package).
- **Reuse the name precedents.** Non-1:1 Lucide→Tabler choices were settled
  in web PR #42 (BarChart3→IconChartBar, Sigma→IconSum, Info→IconInfoCircle,
  BookOpen→IconBook, FileJson2→IconJson, …) — follow them, don't re-derive.
- **Missing a glyph?** Check Tabler first (5,000+); if genuinely absent,
  Carbon's icon taxonomy is the *naming and coverage checklist* for drawing
  a new glyph in Tabler style — never mix Carbon's rendered icons in.

## Evidence

ADR 0009 (+ 2026-07-17 update). Shipped: web PR #42 (full migration,
measured set comparison in the ADR's research trail).
