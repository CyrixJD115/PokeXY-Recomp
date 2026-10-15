---
name: armcc-decomp-executor
description: Decompile and verify exactly ONE function from a PokeXYZ-RDComp .cro module against the retail ARM Compiler 5 binary, using the masked-fingerprint verifier at xy-decomp/armcc-matching/tools/fingerprint_fn.py, then hand off cleanly via a pending/ note. Use when spawned as a subagent to find one function match — do not use this to decompile many functions in one run.
---

# armcc-decomp-executor

Your job: pick ONE not-yet-attempted function from a plaintext `.cro`
module in this repo, reconstruct it in C, prove it byte-matches the retail
binary, commit+push your result, and hand back. **Do not attempt more than
one function** — sibling agents are very likely running in parallel on
different ones.

## 0. Setup

- Confirm you're on the right branch (your spawning prompt names it; if
  unclear, ask or check `git branch --show-current`). If stale: `git fetch
  origin && git reset --hard origin/<branch>` — sibling agents may have
  pushed since your worktree was created.
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

## 3. The 30-minute cap

Budget yourself ~30 minutes **total** for this task. If you haven't reached
a masked match by then, stop regardless of where you are — don't keep
grinding. Write up exactly what you tried and where it got stuck (see the
attempted-note format below) and hand back. This isn't a failure; it frees
the slot for a fresh agent to work on something new instead of two agents
burning time on the same wall.

## 4. Handing off — never edit the shared docs directly

**Do not edit `README.md`, `docs/README.md`, `docs/decomp/README.md`, or
`attempted.md` directly.** Several agents run concurrently on this branch;
those files each have a running-total line or single append point that
every agent would try to touch at once — a guaranteed conflict, not a
maybe. Instead:

**On a match:**
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
  Note: <anything non-obvious worth folding into the README row, optional>
  ```
- Commit both files together (include `Co-Authored-By: Claude Sonnet 5
  <noreply@anthropic.com>`), then `git pull --rebase origin <branch> && git
  push`. If rebase conflicts (another agent pushed first), resolve and
  retry — **never force-push**.

**On hitting the 30-minute cap without a match:**
- Write `xy-decomp/armcc-matching/pending/attempted-<Module>-<offset>.md`
  following the format already used in `attempted.md` (what it is, what you
  tried, exactly what match ratio/words you got stuck on, any lead for next
  time). Commit and push just that note the same way.

## 5. Your final report

Keep it under 200 words: which function (module + offset + match ratio, or
"hit the time cap, logged to pending/"), one sentence on what it does,
confirmation you committed and pushed. The orchestrator re-verifies your
match independently and folds your pending note into the real docs — that's
not your job.
