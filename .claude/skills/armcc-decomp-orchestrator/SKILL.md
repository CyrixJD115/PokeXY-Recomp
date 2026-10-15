---
name: armcc-decomp-orchestrator
description: Run and manage a swarm of parallel subagents decompiling PokeXYZ-RDComp functions against the armcc masked-fingerprint toolchain (xy-decomp/armcc-matching/). Spawns up to a concurrency cap, verifies each agent's claimed match independently, folds their pending/ hand-off notes into the shared docs, cleans up worktrees, and re-arms itself on a /loop tick. Use when asked to spawn/manage/continue a decompiling swarm in this repo, or to "keep decompiling functions" with multiple agents.
---

# armcc-decomp-orchestrator

You are the orchestrator for a swarm of subagents that each decompile ONE
function from a plaintext `.cro` module and prove it byte-matches the retail
binary via `xy-decomp/armcc-matching/tools/fingerprint_fn.py`. Each executor
subagent follows the companion skill `armcc-decomp-executor` — you don't need
to re-explain its process, just spawn it correctly and verify its output.

## Parameters (from `args`, all optional)

- `cap=N` — max concurrent agents (default **5**).
- `branch=<name>` — the shared integration branch (default: whatever branch
  this repo is currently checked out to when you start — ask the user if
  ambiguous and no default makes sense). As of 2026-10-15 this is **`dev`**,
  a long-lived branch the orchestrator merges into; `main` is reached only
  via a PR from `dev` (see "Releasing to main" below), never written to
  directly.
- `count=N` — how many agents to top up to THIS pass, overriding `cap` for a
  one-shot burst (rare; normally you just top up to `cap`).

## Branching model — agents never touch the shared branch

**Every agent works on its own worktree branch and pushes only that branch
under its own name (`git push -u origin HEAD`). No agent ever checks out or
pushes directly to `<branch>` — only you, the orchestrator, merge into it,
from the main repo checkout.** This is load-bearing, not a style
preference: on 2026-10-15 agents following an earlier version of the
executor skill that told them to work directly on `<branch>` ended up with
a second worktree checked out onto the exact same branch ref the main
checkout already holds — git's one-worktree-per-branch rule normally
prevents this, but it happened anyway and corrupted the main checkout's
index twice (files appeared staged-deleted even though they were safely
committed; recovered both times with `git restore --staged` + `git
checkout --`, never `git reset --hard`, since that's blocked here as
irreversible and isn't needed — nothing was actually lost). If you ever see
`git worktree list` show an agent's worktree on `<branch>` itself (not a
`worktree-agent-*`/similar name), that agent is violating this model —
don't trust the state of either checkout until you've independently
verified it (`git fetch`, then re-check `git status --short` for
unexpected staged deletions and restore from HEAD if so).

## One orchestrator pass (what you do each time you're invoked)

Do these in order. Every git operation below happens in the **main repo
checkout**, not inside any agent's worktree.

### 1. Merge in each finished agent's branch, then consolidate

For each agent that has handed back since your last pass (its final report
names its own branch; the task-notification's `<worktree>` block's
`worktreeBranch` is normally the same one unless the agent said otherwise):

```sh
git fetch origin --quiet
git merge --no-ff origin/<agent-branch> -m "armcc-matching: merge <agent-branch> (<Module>@<offset>)"
```

This is a real merge (not fast-forward) since the agent's branch diverged
from an earlier point of `<branch>` — expect a merge commit per agent, no
conflicts in the normal case (each agent only adds new files under a
module/offset-specific path). If something unexpected conflicts, resolve by
inspection — don't guess.

Independently re-verify the match (step 2 below) before trusting it, for
every agent you just merged in.

```sh
ls xy-decomp/armcc-matching/pending/*.md 2>/dev/null
```

For each `pending/matched-<Module>-<offset>.md` (its match already
independently re-verified above):
- Fold it into `xy-decomp/armcc-matching/README.md` (a new module section or
  a new table row) and bump the running totals in `docs/README.md` and
  `docs/decomp/README.md`.
- **Compute the true counts from the filesystem, never by incrementing the
  old number by one per note** — parallel agents drift these numbers
  (duplicate ordinal labels like two different modules both claiming to be
  "the fifth module", stale totals from a note that already got folded in
  by the agent itself before it got your process instructions, etc.).
  Ground truth:
  ```sh
  find xy-decomp/armcc-matching/matched \( -name '*.c' -o -name '*.cpp' \) | wc -l   # total functions
  # (an agent sometimes needs .cpp when the candidate requires a C++
  # `namespace` block for a demangled symbol -- armcc picks C vs C++ mode
  # from the extension, so don't "fix" this by renaming to .c; it'll just
  # fail to compile. Glob both extensions or this undercounts.)
  ls xy-decomp/armcc-matching/matched | wc -l                  # total modules
  ```

For each `pending/attempted-<Module>-<offset>.md`: fold it into
`xy-decomp/armcc-matching/attempted.md` using the format already documented
in that file, then delete the pending note.

Delete every pending note you've folded in. Commit the whole consolidation
as **one** commit, then:
```sh
git pull --rebase origin <branch> && git push
```
If a module/table edit conflicts with something a sibling orchestrator pass
or an agent's direct push already did, resolve by re-deriving the correct
state from the filesystem (per the ground-truth counts above), not by
picking one side arbitrarily.

### 2. Verify, then clean up finished agents

Watch for `<task-notification>` / subagent hand-back messages. Treat a
report as a claim, not a fact — the note it left in `pending/` is a
convenience, but the match itself must be independently re-run:

```sh
XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
    <Module> <offset> <size> xy-decomp/armcc-matching/matched/<Module>/<offset>.c
```
Confirm `*** MASKED MATCH ***` yourself before treating the function as done.
If it doesn't reproduce, say so to the user rather than reporting success.

Then reclaim the worktree and tidy up its now-merged remote branch:
```sh
git push origin --delete <agent-branch>   # safe: already merged into <branch> above
git worktree remove <path> --force
git branch -D <worktree-branch>
```
(Worktree path and branch are given in the task notification's `<worktree>`
block.)

### 3. Top up to the cap

```sh
# ListAgents to count currently-running decompiling subagents
```
If running count < cap, spawn `(cap - running)` new agents. Each spawn:
- `subagent_type: "general-purpose"`, `isolation: "worktree"`, a distinct
  `name` (e.g. `armcc-decomp-N` with an incrementing N you haven't used).
- A SHORT prompt — the detailed process lives in the executor skill, don't
  re-paste it. Something like: *"Repo: PokeXYZ-RDComp. Shared integration
  branch: `<branch>` — stay on your own worktree branch, never check that
  one out; rebase onto `origin/<branch>` for freshness and push only your
  own branch (`git push -u origin HEAD`). Invoke the `armcc-decomp-executor`
  skill now (via the Skill tool) and follow it to decompile and verify
  exactly one function, then hand back. If you can't find that skill
  listed, read `.claude/skills/armcc-decomp-executor/SKILL.md` directly
  instead — it's committed in this repo. Suggested starting module to
  reduce collision with sibling agents: `<pick one not yet in
  xy-decomp/armcc-matching/matched/, vary it per agent>`. If that module
  isn't tractable, pick any other unmined one."*
- Give each spawned agent a **different** suggested starting module where
  you can (check `ls xy-decomp/armcc-matching/matched/` and
  `ls .Data/Raw_Extracted/romfs/*.cro` to pick ones not yet touched) — this
  is a hint, not a hard assignment; collisions are low-risk but not free
  (git rebases cost agent time).

If already at cap, spawn nothing this pass — that's normal, not an error.

### 4. If invoked by `/loop`

Re-arm with `ScheduleWakeup` per the loop skill's own rules (don't duplicate
that logic here). A reasonable fallback heartbeat is 600–900s when a swarm
is actively running; widen it once things quiet down. Mention in your
visible update: how many agents are running, how many matched/attempted
since the last pass, and the current true totals.

### 5. Releasing to `main`

`main` is never written to directly — not by agents, not by you. Release
roughly **every 25 commits accumulated on `dev` since the last release**
(check with `git log --oneline <last-release-sha>..dev | wc -l`) — treat
that as a trigger to check in with the user, not a hard mechanical rule;
a count that lands mid-round (an agent not yet consolidated) can wait for
the round to finish. Also release whenever the user asks, or whenever the
swarm is being paused — don't let two releases' worth of work stack up
in one open PR (see the dated note at the end of this section for why
that defeats the whole point of `dev`).

When it's time to release:

**First, regenerate the heatmap** (cheap — ~0.7s once `xy-viz` is already
built; only rebuilds `xy-viz` itself the very first time, which takes
~55s):
```sh
python3 xy-heatmap/heatmap.py readme
git status --short docs/assets/
```
This derives from the Ghidra/recomp corpus (`decomp/`, `recomp/`), not
from `xy-decomp/armcc-matching/`, so a PR that's pure armcc-matching work
will usually only bump the embedded `generated` timestamp (not the
counts) — commit it anyway, on `dev`, before opening the PR. Only treat a
counts change as notable; a timestamp-only diff is expected and fine to
commit quietly.

Then open the PR:
```sh
gh pr create --base main --head dev --title "..." --body "..."
```

A direct push/merge to `main` (e.g. `git push origin dev:main`) is blocked
by this environment's own guardrails ("Merge Without Review") — don't try
to work around that; a PR is the correct path anyway, not a fallback. If a
PR from `dev` is already open, new consolidation commits on `dev` land in
it automatically (no need to open a second one) — just tell the user it
updated. Once the user merges the PR, keep working on `dev` as normal;
`dev` stays ahead of `main` by whatever hasn't been released yet.

**2026-10-15 note — don't let an open PR go stale while `dev` keeps
growing.** The first PR (#1, from the pre-`dev` branch
`armcc-matching-dllnumberinput`) sat open while `dev` was created from
its tip and the swarm kept adding commits on top. Since `dev` already
contained everything in that PR, every commit added to `dev` afterward
was *also* owed to that same unmerged PR — so the next `dev`→`main` PR
would have re-included all of it plus the new work, right back to one
giant diff, defeating the reason `dev` exists. If you ever notice an
older PR targeting `main` is still open when you're about to open a new
one, or when a release checkpoint comes up, flag it to the user instead
of opening a second PR on top of it — ask them to merge or close the
stale one first (you can't merge PRs yourself; it hits the same
guardrail as a direct push).

## Things that will bite you if you skip them

- **Never trust a running total without recomputing it from the filesystem.**
  This is the single most common failure mode with parallel agents — see
  step 1.
- **Never let agents edit `README.md`/`docs/*.md`/`attempted.md` directly.**
  That's exactly the collision the `pending/` convention exists to avoid;
  if you see an agent's prompt missing that instruction, it's stale, fix it.
- **A worktree that was never cleaned up accumulates.** Run `git worktree
  list` periodically and reclaim anything whose agent has already handed
  back.
- **Don't force-push, ever**, even to resolve a consolidation conflict.
  Re-derive and recommit instead.
- **Never use `git reset --hard` to fix a weird main-checkout state** (it's
  blocked here as irreversible, and you don't need it). An unexpected
  staged deletion for a file that's clearly in `HEAD` is an index glitch,
  not real lost work — fix it with `git restore --staged <path> && git
  checkout -- <path>`, then re-run the `git fetch`/merge step.
- **If an agent's worktree shows up checked out on `<branch>` itself**
  (`git worktree list`), that's the exact corruption class described under
  "Branching model" above — treat both checkouts' state as suspect until
  verified, and make sure that agent's instructions (and the executor
  skill, if it's stale) tell it to push its own branch instead.
