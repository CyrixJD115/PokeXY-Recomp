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
- `branch=<name>` — git branch all agents work on (default: whatever branch
  this repo is currently checked out to when you start — ask the user if
  ambiguous and no default makes sense).
- `count=N` — how many agents to top up to THIS pass, overriding `cap` for a
  one-shot burst (rare; normally you just top up to `cap`).

## One orchestrator pass (what you do each time you're invoked)

Do these in order. Every git operation below happens in the **main repo
checkout**, not inside any agent's worktree.

### 1. Consolidate `pending/` notes left by finished agents

```sh
git fetch origin --quiet && git merge --ff-only origin/<branch>
ls xy-decomp/armcc-matching/pending/*.md 2>/dev/null
```

For each `pending/matched-<Module>-<offset>.md`:
- Independently re-verify the match yourself (step 2 below) before trusting
  the note — a note describes what an agent *claimed*, not a proven fact.
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
  find xy-decomp/armcc-matching/matched -name '*.c' | wc -l   # total functions
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

Then reclaim the worktree:
```sh
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
  re-paste it. Something like: *"Repo: PokeXYZ-RDComp. Branch: `<branch>`.
  Invoke the `armcc-decomp-executor` skill now (via the Skill tool) and
  follow it to decompile and verify exactly one function, then hand back. If
  you can't find that skill listed, read
  `.claude/skills/armcc-decomp-executor/SKILL.md` directly instead — it's
  committed in this repo. Suggested starting module to reduce collision with
  sibling agents: `<pick one not yet in xy-decomp/armcc-matching/matched/,
  vary it per agent>`. If that module isn't tractable, pick any other
  unmined one."*
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
