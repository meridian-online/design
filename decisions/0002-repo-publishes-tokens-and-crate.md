---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-16
---
# 0002. This repo holds tokens + guidelines and publishes both artefacts

## Context and Problem Statement

A design system needs a home and a shape. At two-consumer scale (web,
Brightfield), a Carbon-style full system with a component gallery is more
maintenance than value; a pure token dump loses the "why" and drifts through
interpretation. Separately: who owns the Rust token crate — this repo or
Brightfield?

## Considered Options

- **Tokens + guidelines here; repo publishes `tokens.css` AND the
  `meridian-design` crate** — consumers take dependencies on this repo
- **Tokens only** — pure data repo
- **Full system with component gallery** — Carbon-shaped
- **Crate lives in Brightfield** — this repo advisory only

## Decision Outcome

Chosen option: tokens + guidelines, publishing both artefacts. Components stay
in their host repos (shadcn on web, gpui-component/Masonry on desktop). The
crate living here — not in Brightfield — is what makes this repo *the* source
of truth rather than documentation about one.

### Consequences

- Good, because one definition feeds every consumer; conformance tests pin the
  emitted CSS and ThemeConfig against the crate.
- Good, because guidelines carry the rules tokens can't (density posture,
  speed budgets, colour method).
- Neutral, because Brightfield takes a cross-repo dependency (git/path); a
  version bump ritual is needed at each token release.
