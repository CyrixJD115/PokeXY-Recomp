# Pending notes — agent hand-off, not a destination

This directory exists so parallel agents never edit the same shared doc
line at the same time. With several agents matching functions
concurrently on one branch, every agent trying to bump the same running
total in `README.md`/`docs/README.md`/`docs/decomp/README.md`, or append
to the same `attempted.md`, is a guaranteed git conflict waiting to
happen — not a maybe.

**Agents: do not edit `README.md`, `docs/README.md`, `docs/decomp/README.md`,
or `attempted.md` directly.** Instead:

- On a match: save your `.c` file to `matched/<Module>/<offset>.c` as
  normal, AND write a tiny note to `pending/matched-<Module>-<offset>.md`
  (a brand-new, uniquely-named file — no collision possible) containing
  just enough for someone else to write the README row: module, offset,
  size, match ratio, and a one-line description. Commit and push both
  files together. Do not touch the README/docs files yourself.
- On hitting the 30-minute cap without a match: write
  `pending/attempted-<Module>-<offset>.md` (same naming scheme) with
  what you tried and where it got stuck. Commit and push just that file.
  Do not touch `attempted.md` yourself.

The session orchestrating these agents periodically sweeps this
directory, folds each note into the real README/docs/`attempted.md`,
commits that consolidation as one change, and deletes the consumed
note files. If you see old `.md` files here that look already folded
in (the function they describe already has a proper table row
elsewhere), they're stale — leave them, the orchestrator will clean up.

Note format (keep it terse, this is scratch data, not prose):

```
Module: <Name>
Offset: 0x...
Size: 0x...
Match: N/N
One-line: <what it does>
Note: <anything non-obvious worth folding into the README row, optional>
```
