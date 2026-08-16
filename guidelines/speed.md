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
- **No decorative animation in the apps.** The test is what the motion depicts,
  not where it runs. Motion earns its place — spatial continuity as a panel
  opens, a brush moves, an overlay appears — and continuity motion stays under
  ~150 ms. Nothing eases in for delight; data appears at once, not staggered.
- **On a data surface, the test is whether the motion depicts a property of the
  data.** The worked example is flow direction: a dot travelling an edge of a
  graph encodes which way the graph flows — a property of the graph, not an
  ornament on it, and the same fact an arrowhead encodes. That is data ink, and
  it is admitted. Motion tied to no state is not: an ambient field behind a
  canvas depicts nothing about what is drawn on it, so it ships behind a
  preference a reader can leave off, if it ships at all. The ~150 ms ceiling
  above is a continuity budget — it bounds a transition, not a cue that runs
  while a surface sits idle. **"This is decorative" is still a complete review
  comment.** "This moves, and it is on a data surface" is not one by itself —
  say what it depicts, or say that it depicts nothing.
- **Brand surfaces are governed separately.** Marketing, install, OG images, the
  desktop front door and the README may carry brand motion, capped at three
  assets, under ADR 0012. The mark itself does not move in application chrome or
  on a data surface; the honest-work cue below is its one in-app appearance, and
  this page already required that cue.
- **The honest-work cue may wear the mark.** The >100 ms indicator this page
  requires is not decorative motion, and drawing it as the Meridian mark rather
  than a generic spinner changes how it looks, not whether it is allowed. The
  asset is `meridian-design/brand/motion/form.*` — Lottie for the desktop,
  animated SVG for the web. **Take it; do not draw another one.** Under reduced
  motion it settles to the assembled mark, which says the brand and not that
  anything is happening, so a surface using it as the work cue carries a textual
  cue beside it.
- **Reduced motion is honoured wherever motion ships, desktop included.** On the
  web that is the `prefers-reduced-motion` media query. The desktop obligation is
  the same one, and it falls on whichever surface introduces the motion: a reader
  who has asked for less motion gets a still frame that carries the same
  information — for flow direction, the arrowhead rather than a bare edge. Take
  the signal from the platform where the host exposes it and from an app
  preference where it does not; what is not negotiable is that the still state
  says what the motion said.
- **GPU is the enabler, not the excuse.** Rendering headroom is spent on
  more data on screen, never on ornamental effects.

## Citing this page

A PR that introduces post-input latency, decorative motion, or silent >100 ms
work should be blocked with a pointer here. Budgets are only eyeball-testable
today; instrumenting them is future work — treat any measured regression as a
regression, not a debate.

## Evidence

ADR 0010; ADR 0012 for the brand-surface carve-out and for the amendment that
makes what the motion depicts the test on a data surface. Shipped practice: brush
re-dispatch on release (#60); reload/save outcomes surfaced in the Log dock; the
continuous-drag design explicitly gates on throttle/cancellation.
