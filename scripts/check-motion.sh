#!/usr/bin/env bash
#
# The brand-motion artefacts still match the generator that produced them.
#
# This is the byte-for-byte half of ADR 0012's pin, and it is a script rather
# than a Rust test for one reason: the generator is Python. The crate's own
# emitters get this check for free — `cargo run --example dump_css` against a
# committed snapshot — because emitter and snapshot are both Rust. Here the
# emitter is `motion/build_form.py`, so the equivalent has to run Python, and
# `cargo test` must not depend on it. `meridian-design/tests/motion.rs` carries
# the other half: what the artefacts have to *be*, which a byte comparison
# cannot say.
#
# It catches the direction a committed artefact actually drifts. A snapshot
# notices someone editing the file; this notices someone editing the
# CHOREOGRAPHY and not regenerating — a schedule change that lands in the
# generator, ships nowhere, and leaves the tree looking clean.
#
# Toolchain-free and takes about a second, so it sits beside the hygiene gate
# rather than behind the crate build. Needs Python 3 and nothing else: the
# generator has no dependencies, by the same rule that keeps the crate free of
# them.
set -euo pipefail

cd "$(dirname "$0")/.."

if ! command -v python3 >/dev/null 2>&1; then
  echo "check-motion: python3 not found — the artefacts cannot be verified" >&2
  exit 1
fi

cd motion
exec python3 build_form.py --check
