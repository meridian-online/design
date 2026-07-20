---
status: accepted
date-created: 2026-07-16
date-modified: 2026-07-20
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

### Update (2026-07-20 — amended for desktop only; see ADR 0011)

Two clauses above narrow, **for the desktop half only**:

- "Components stay in their host repos (shadcn on web, gpui-component/Masonry
  on desktop)". The **web** half stands unchanged and permanently — shadcn on
  Base UI, owned by the web repo, no npm package and no TypeScript build path
  here. The **desktop** half no longer holds: Brightfield is migrating off GPUI
  onto egui, and egui ships primitive widgets and no system, so there is no
  host library left to defer to. Desktop component code moves into this repo as
  a second crate, `meridian-egui` (ADR 0011).
- The rejection of a **component gallery** stands as written. What ADR 0011
  authorises is a capped set of ~15 primitives, each earned by three
  independent call sites — the opposite of a gallery, and capped precisely
  because the maintenance fear recorded above was correct.

This is an amendment, not a supersession. The premise changed — the desktop
host library went away — rather than the reasoning being wrong. Everything else
here is unchanged and load-bearing: one definition feeding every consumer, the
crate living here rather than in Brightfield, and guidelines carrying the rules
tokens cannot.

### Consequences

- Good, because one definition feeds every consumer; conformance tests pin the
  emitted CSS and ThemeConfig against the crate.
- Good, because guidelines carry the rules tokens can't (density posture,
  speed budgets, colour method).
- Neutral, because Brightfield takes a cross-repo dependency (git/path); a
  version bump ritual is needed at each token release.
