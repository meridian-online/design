# Speed

A good experience is always fast and intuitive. This page makes "fast" a
budget you can cite in a review, not a vibe (ADR 0010). "This violates the
speed policy" is a complete review comment.

## Budgets

- **Feedback lands next frame.** Anything that acknowledges input — cursor
  change, hover emphasis, focus ring, keypress echo, brush handle tracking —
  renders on the next paint. No debounce on acknowledgement, ever.
- **100 ms is the honesty line.** Work that can exceed ~100 ms (a re-query, a
  reload, a profile scan) must show its truth: a progress cue, a log entry,
  or an explicit deferred contract — never a frozen frame and never a stale
  view pretending to be current.
- **Gestures may defer the expensive part, visibly.** The committed pattern:
  the affordance tracks live at frame rate, the query dispatches on release
  (today's cross-filter brush). Moving to live re-query (continuous drag)
  requires throttling + cancellation discipline first — never uncancelled
  query pile-up.
- **No decorative animation.** Motion exists only for spatial continuity —
  a panel opening, a brush moving, an overlay appearing — and stays under
  ~150 ms. Nothing eases in for delight; data appears at once, not staggered.
- **Honour `prefers-reduced-motion`** on the web; desktop motion is sparse
  enough that there is nothing to reduce.
- **GPU is the enabler, not the excuse.** Rendering headroom is spent on
  more data on screen, never on ornamental effects.

## Citing this page

A PR that introduces post-input latency, decorative motion, or silent >100 ms
work should be blocked with a pointer here. Budgets are only eyeball-testable
today; instrumenting them is future work — treat any measured regression as a
regression, not a debate.

## Evidence

ADR 0010. Shipped practice: brush re-dispatch on release (#60); reload/save
outcomes surfaced in the Log dock; the continuous-drag design explicitly gates
on throttle/cancellation.
