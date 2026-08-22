# Throwaway — delete this directory with the branch

Two rendered frames and nothing else. They exist so a choice about the status pill's horizontal geometry can be made by looking, and they are expected to be thrown away once it is made. Nothing in the repo reads them, the published evidence site does not carry them, and no test compares against them — they are pictures, not baselines.

## The question they are for

`meridian-egui`'s `status_pill` spends `space[2]` (4.0) on each outer edge of its capsule and `ICON_LABEL_GAP` = `SPACE_3` (6.0) between the icon and the label, so the group is looser in its middle than it is inset from its container. `key_chip` reaches for `space[2]` the same way, so both chip primitives share whatever answer wins.

| row | outer inset | icon–label gap | pill | chip |
|---|---|---|---|---|
| today | 4.0 | 6.0 | — | — |
| arm 1 | 6.0 | 6.0 | +4.0pt each | +4.0pt each |
| arm 2 | 4.0 | 4.0 | −2.0pt each | unchanged |

Both frames carry the same six specimens: the four pills the consuming app's gallery draws, then two keycaps. Three of the four pill labels have a descender and `ok` has none, which is deliberate — the pill's *other* open question is that it centres its icon on the icon's own symmetric box and its label galley on the full line box including descent, so a descender sits low and `ok` does not. That is reproduced here, not repaired.

Light and dark are two files rather than two halves of one, because a single frame would have to pick one surface to sit both halves on, and one of the two would then be judged against the wrong background.

## Regenerating them

    cargo test -p meridian-egui --test pill_geometry_montage -- --ignored --nocapture

`#[ignore]`d because it renders through wgpu and needs a GPU adapter, which this repo's CI runner has not got. The measurement test beside it needs no GPU and runs normally: it draws the shipping primitives beside the parameterised copies the montage uses and fails if the two ever allocate different widths, so the "today" row cannot quietly stop being what ships.

Rendered at 2.0 pixels per point, which is the scale the consuming app photographs its pills at.
