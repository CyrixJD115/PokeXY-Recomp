---
name: armcc-decomp-executor
description: Decompile and verify up to a few functions (ceiling of 5, or ~60-75 min total, whichever first — stopping earlier, e.g. around 3, is expected and fine once easy candidates run dry) from a PokeXYZ-RDComp .cro module against the retail ARM Compiler 5 binary, using the masked-fingerprint verifier at xy-decomp/armcc-matching/tools/fingerprint_fn.py, then hand off cleanly via pending/ notes. Use when spawned as a subagent to find a small batch of function matches — do not use this to decompile a whole module's worth of functions in one run.
---

# armcc-decomp-executor

Your job: pick not-yet-attempted functions from a plaintext `.cro` module
in this repo, reconstruct each in C, prove it byte-matches the retail
binary, commit+push your results, and hand back. **Ceiling: 5 functions,
or ~60-75 minutes total, whichever comes first** — but that's a cap, not
a target. Stopping earlier (3, 2, even 1) is completely normal and
expected once the current module runs low on easy candidates; don't go
hunting through a second or third module just to hit a round number.
Sibling agents are very likely running in parallel on other modules, and
this isn't a full-module sweep either way. Prefer finding your next
function in the **same module** as your last one before moving to a new
one: you've already paid the cost of disassembling it and learning its
layout, and hand-off notes routinely flag an easy sibling stub right next
to a just-matched function — that's free reuse of work you already did.
Move to a different module only once the current one is genuinely out of
easy candidates within your remaining budget — not just because you
haven't reached 5 yet.

## 0. Setup

- **Stay on your own worktree's local branch — never check out the shared
  integration branch named in your spawning prompt.** That branch is
  already checked out in the main repo checkout; a second worktree
  checking it out too corrupts both checkouts' index/working-tree state
  (git's one-worktree-per-branch rule exists exactly to prevent this — two
  agents hit this during the 2026-10-15 swarm run before it was fixed).
  Check `git branch --show-current` — whatever it reports (typically
  `worktree-agent-<id>`) is your branch; just work there. To pick up
  sibling agents' latest matches first: `git fetch origin && git rebase
  origin/<branch>` (rebases your own branch onto the shared one's tip —
  does not touch or check out `<branch>` itself).
- Fix the worktree's missing `.Data`: it's mostly gitignored (machine-local
  ROM-derived data), but `.Data/README.md` IS git-tracked, so don't clobber
  it — symlink just the subtree you need:
  ```sh
  ln -s /home/yvonne/Documents/decomp/PokeXYZ-RDComp/.Data/Raw_Extracted .Data/Raw_Extracted
  ```
  Treat everything through that link as **strictly read-only** — it's the
  same files the main checkout and every sibling agent use, not a private
  copy.
- **Never write scratch/temp files (e.g. a draft commit message) to the
  shared session-wide scratchpad folder** — sibling agents run concurrently
  and will race on any path there, which can leak one agent's text into
  another's temp file. Put any such temp file inside your own worktree
  instead (it's private to you) and clean it up before you finish.
- Read, in this order, before touching any code:
  1. `xy-decomp/armcc-matching/README.md` — the match table (what's already
     proven, so you don't repeat it) and the "Working on this" rules
     (summarized below, but that's the source of truth if it's changed).
  2. `xy-decomp/armcc-matching/attempted.md` — functions already tried and
     stuck. **Skip anything listed there** unless you have a genuinely new
     idea beyond what's already written down.
  3. `docs/decomp/armcc-toolchain.md` — background on why armcc 5.04u5 (not
     GCC) is the right toolchain, the fingerprinted flags, and narrative
     detail on the open puzzles.

## 1. The toolchain

`xy-decomp/armcc-matching/tools/fingerprint_fn.py <Module> <hex offset>
<hex size> <your .c file>` compiles your candidate with armcc 5.04u5 via
Wine and compares it word-for-word against the real bytes read straight out
of the plaintext `.cro` (no decryption needed — CRO magic `CRO0` is at file
offset `0x80`). It masks `b`/`bl` branch immediates automatically (those are
link-time facts no isolated compile can know). Extra flags exist for harder
cases — run `python3 fingerprint_fn.py --help` or read its module docstring
for `--relax-nops` (post-link branch-relaxation nop words) and
`--mask-ranges` (jump-table data words — **read its caveat carefully before
using it**: it proves case-body correctness and ordering, never which case
*value* selects which body).

Setup is already done on this machine: `ARMCC_HOME` defaults to
`~/Documents/decomp/tools/armcc/ARMCC_Nintendo_5`, Wine and
`arm-none-eabi-objcopy`/`objdump` are installed. If the tool can't find your
CRO, pass `XY_ROMFS=.Data/Raw_Extracted/romfs` explicitly. Proven default
flags: `--cpu=MPCore --arm --apcs=/interwork -O2 -Otime` (already the
script's default — you don't need to pass `--flags` unless deviating).

## 2. Finding a candidate

Pick a `.cro` under `.Data/Raw_Extracted/romfs/` not yet in
`xy-decomp/armcc-matching/matched/` (or one already started, if you spot an
obviously-untried simple function there). Extract its `.text` section (file
offset from the CRO header's segment table, right after the `CRO0` magic)
and disassemble: `arm-none-eabi-objdump -D -b binary -m arm
--adjust-vma=<text_file_offset> <extracted.bin>`.

**Offset gotcha:** `xy cro info`'s reported segment offsets (e.g. "seg 0:
.text @0x180") are already **absolute file offsets** into the `.cro` — do
not add the `CRO0` header's `0x80` on top when hand-disassembling.
`fingerprint_fn.py`'s own `<hex offset>` argument is likewise a raw
absolute file offset (cross-checked against `DllDialogCommon@0x8d0`'s
known-good bytes). Getting this wrong disassembles the wrong bytes
entirely, not just a shifted view of the right ones.

**What reliably works** (near-100% hit rate across every module mined so
far — try these before anything fancier):

1. **Trivial virtual-override stubs**: isolated 1-2 word leaf functions —
   `mov r0,#1; bx lr` (return true), `mov r0,#0; bx lr` (return false), or a
   lone `bx lr` (no-op void, weak evidence since any side-effect-free void
   function compiles to the same word). These cluster near the tail of a
   module's `.text`, around the import-veneer table and destructor-stage
   functions.
2. **Simple field accessors/setters**: `strb r1,[r0,#N]; bx lr` (setter), or
   a short `ldr`/`cmp`/`movne`/`bx lr` chain (boolean getter through one
   indirection). Model structs with `unsigned char pad[N]` arrays to hit
   exact byte offsets.
3. **Simple field-copy-and-call helpers**: copy a field from one argument
   into a sub-object, call an import veneer (`extern void *fnXXX(...);` —
   name doesn't matter, only the call site's register setup, since `bl`
   targets are masked), store the result.
4. **Simple destructors**: install a vtable pointer, free one owned member
   via a masked call if non-null, zero it, return `this` — but only when the
   **member-load happens AFTER the vtable-pool-load** in the disassembly.
   The opposite ordering (member-load first) is a known-bad shape — check
   `attempted.md` for the current state of that residue class before
   spending time on it.
5. **Constructors with cursor-style stores**: if the retail binary writes
   init fields as a chain of ARM pre/post-indexed stores (`str r6,
   [r0,#92]!` — stores AND advances the pointer in one instruction), write
   literal C cursor-increment code (`p += 0x5c; *p = 0; p += 4; ...`), not
   struct-member offsets off a fixed base. The two idioms produce different
   armcc output even for logically-identical final state — see
   `attempted.md`/the toolchain doc for the specific trap if you hit it.
6. **Shared epilogues with no return-value `mov`**: when two branches jump
   to the same `pop {..., pc}` with no `mov r0,#N` right before it, the
   retail binary is returning whatever's *already* live in that register at
   each branch site, not the semantically "obvious" constant. Trace the
   actual register value at each branch before guessing the return value.

Iterate on your C until `fingerprint_fn.py` prints `*** MASKED MATCH ***`.
**If a function genuinely won't match after real effort and doesn't look
like a recognized trap, abandon it and pick a different one** rather than
burning your whole budget on one puzzle — there are always easy wins
elsewhere. (ARM calling convention reminder: r0-r3 = args, r0 = return.)

Once you land a match, before moving on, take a moment to scan the same
module's disassembly near it for another easy candidate (the patterns in
"What reliably works" above) — that's the cheapest function you'll find
all session, since you already have the module's layout loaded.

## 3. The time/count budget

Ceiling: **~60-75 minutes total, and at most 5 functions**, whichever
limit you hit first — this is a batch cap across your whole run, not per
function, and not a target to chase. Within that budget, spend no more
than ~20-25 minutes stuck on any single function before abandoning it for
a different candidate (same module first, per above). **Stop as soon as
the current module stops offering easy candidates** — that might be
after 1 function, might be after 5; either is a normal, complete run, not
a shortfall. Don't go digging through a second or third module just to
pad the count up. If you do reach the time or count ceiling, stop where
you are — with however many matches you have, plus optionally one
`attempted-*.md` note for whatever you were stuck on when you stopped —
and hand back. Don't keep grinding past the budget either way; this isn't
a failure, it frees the slot for a fresh agent instead of one agent
sitting on the wall indefinitely.

## 4. Handing off — never edit the shared docs directly

**Do not edit `README.md`, `docs/README.md`, `docs/decomp/README.md`, or
`attempted.md` directly.** Several agents run concurrently on this branch;
those files each have a running-total line or single append point that
every agent would try to touch at once — a guaranteed conflict, not a
maybe. Instead:

**On each match** (do this per function, as you land it — don't batch all
of them up to the end in case you run out of budget mid-way):
- Save `xy-decomp/armcc-matching/matched/<Module>/<offset>.c` as normal,
  with a header comment: one-line summary + match ratio, any non-obvious
  note (what you had to figure out), a `Verify:` line with the exact
  command.
- Write `xy-decomp/armcc-matching/pending/matched-<Module>-<offset>.md`:
  ```
  Module: <Name>
  Offset: 0x...
  Size: 0x...
  Match: N/N
  One-line: <what it does>
  Style: natural | trick(<kind>) -- <why>
  Note: <anything non-obvious worth folding into the README row, optional>
  ```
  `Style:` is required, and the same line goes in the `.c` header comment.
  Use `trick(<kind>)` if the C contains anything that exists only to make
  armcc emit the retail bytes (a `volatile` the original wouldn't need, a
  redundant local, an unused extra arg, a one-use static helper, a `void*`
  return used only to avoid a spare move); prefer a natural form when two
  both match. Kinds are listed in `xy-decomp/armcc-matching/README.md`
  ("Match style: natural vs. trick"). Relaxed-only (`--relax-nops`) results
  are NOT matches — don't put them under `matched/`.
- Commit both files together (include `Co-Authored-By: Claude Sonnet 5
  <noreply@anthropic.com>`) **on your own worktree branch** — never on
  `<branch>` itself. One commit per matched function is fine even across
  your 2nd/3rd function in the same run — small, clearly-attributed
  commits are easier for the orchestrator to merge and verify than one
  commit bundling several unrelated matches.
- **Push after every single match, not just at the end of your run**:
  `git fetch origin && git rebase origin/<branch>` (picks up sibling
  agents' latest work), then push your own branch. Your first push each
  run is a plain `git push -u origin HEAD`; every push **after** you've
  rebased again (because `<branch>` moved since your last push) needs
  `git push --force-with-lease origin HEAD` instead — a plain push would
  be rejected as non-fast-forward, since the rebase rewrote commits your
  remote branch already has. This is safe and expected here: it's **your
  own exclusive branch**, nobody else pushes to it, so there's nothing to
  lose — this is a different thing from force-pushing the *shared*
  integration branch, which stays forbidden (only the orchestrator ever
  writes to `<branch>`, and never with `--force`). Do **not** push to
  `<branch>` yourself and do not use a refspec that writes to it.

**On hitting the time/count budget without a match for your current
candidate:**
- Write `xy-decomp/armcc-matching/pending/attempted-<Module>-<offset>.md`
  following the format already used in `attempted.md` (what it is, what you
  tried, exactly what match ratio/words you got stuck on, any lead for next
  time). Commit it the same way as a match (its own commit is fine), and
  make sure it's included in your final push.

## 5. Your final report

Keep it under 350 words: one line per function you handled (module +
offset + match ratio, or "hit the budget, logged to pending/" for an
attempted one), one short phrase each on what it does, **the exact name
of your own branch** (`git branch --show-current`) and that you pushed
it. The orchestrator re-verifies every match independently, merges your
branch into the shared integration branch itself, and folds your pending
notes into the real docs — that's not your job.
