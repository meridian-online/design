# Brand assets — all rights reserved

**The MIT licence covering this repository does not extend to this directory.**

Everything in `meridian-design/brand/` — the Meridian prime mark, the wordmark,
their Affinity sources, and every artefact generated from them — is the
trademark and copyright of Meridian. All rights reserved.

This mirrors how the vendored typefaces are handled one directory over: the code
grant is MIT, and each non-code artefact carries the terms that actually govern
it. `fonts/` is Open Font License; `brand/` is reserved.

## What you may do

- Read, build and run this repository, including the code that references these
  files.
- Reproduce the mark **unmodified** when referring to Meridian — writing about
  it, linking to it, listing it among tools you use.

## What you may not do

- Use the mark, the wordmark or any derivative to identify your own product,
  service, organisation or site.
- Modify, recolour, redraw, re-letter or re-proportion them.
- Imply endorsement, affiliation or origin that does not exist.
- Take these files under the MIT terms of the surrounding crate. The MIT grant
  covers the source code in this repository. It does not cover this directory,
  and a trademark could not be granted by it in any case.

## Why this directory sits inside an MIT crate

Because the desktop consumer takes `meridian-design` as a Cargo dependency and
needs these bytes at compile time. A directory at the repository root would not
reach it. The carve-out is the price of that, and it is stated here, in
`../../README.md`, and in `../../LICENSE`.

Questions: <https://github.com/meridian-online/design>.
