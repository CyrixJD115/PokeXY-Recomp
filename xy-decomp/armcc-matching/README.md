# ARMCC matching — verified byte-match decompilation

This is the real **compiler-parity breakthrough** `docs/general/findings.md`
flags as the project's outstanding blocker: compiled-vs-original matching
was stuck at 0% because nobody had the *retail* toolchain. It exists —
Pokémon X/Y's ARM11 code was built with the CTR SDK's **ARM Compiler 5
(armcc)**, not GCC, and that toolchain is available locally.

This was discovered and proven independently in a sibling project
(`~/Documents/decomp/pokemon x`, the maintainer's own from-scratch
decompilation attempt) and is migrated here on 2026-10-14 so the finding
and its proof live in this repo too. Full writeup: `docs/decomp/armcc-toolchain.md`.

**Reading the module count honestly (2026-10-15 swarm note, re-derived
2026-10-15 after the per-module split below drifted out of sync with
the filesystem across several hand-tracked updates — see the dated note
further down for what happened and the fix):** the "N functions across M
modules" headline is breadth, not depth. The swarm's executor skill
deliberately mines the *easiest* win per module first (trivial 1-2
instruction virtual-override stubs and field setters have a near-100%
hit rate — see "What reliably works" below), so most matched modules
still have their full remaining function count (typically hundreds)
completely unaddressed regardless of how many functions are proven in
them. A module having **more than one** matched function is unambiguous
evidence of that (recompute with `for d in matched/*/; do find "$d" -name
'*.c' -o -name '*.cpp' | wc -l; done | awk '$1>1' | wc -l` — **12 of 91**
as of this update: `DllDialogCommon`, `DllNumberInput`, `DllStrInput`,
`DllFatalErrorProc`, `DllIntro`, `DllSparringMenu`, `DllStatus`,
`DllSubScreenTutorial`, `DllStartMenu`, `DllStaffRoll`, `DllWazaOboe`,
`DllTrialDownload`). Many of the
remaining single-match modules are also genuinely meaningful (a
destructor/constructor/dispatcher/conditional-accessor, not a 1-2 word
stub) — see each module's own entry above for which; that split isn't
repeated as a second hand-maintained number here anymore; see the dated
note below for why. Treat "M modules touched" as a toolchain-portability
proof (the technique works on unrelated modules, not just the first one
found), not as module completion.

**2026-10-15 — dropped the hand-maintained trivial-vs-meaningful split
after it silently drifted.** This paragraph used to carry two numbers
("N of M modules have exactly one trivial stub", plus a named list of
the rest) that every consolidation pass was supposed to increment by
hand alongside the real, filesystem-derived function/module totals. They
desynced anyway — by the time this was caught, the named list had 22
entries but the true module count was 89, an undetected 10-module gap
that must have accumulated over several updates where an increment was
missed or miscounted. The real totals (`find matched -name '*.c' -o
-name '*.cpp' | wc -l` and `ls matched | wc -l`) never drifted, because
every consolidation re-derives them from the filesystem instead of
incrementing a remembered number — exactly the rule this skill already
enforces for the orchestrator (see `.claude/skills/armcc-decomp-
orchestrator/SKILL.md`, "Never trust a running total without
recomputing it"). This paragraph just hadn't been following its own
rule. The >1-match count above is similarly cheap to recompute in full
every time, so it won't drift the same way; the finer trivial/meaningful
split per single-match module is left to each module's own entry instead
of a second summary number.

## What's here

- `tools/fingerprint_fn.py` — the verifier. Compiles a candidate `.c` file
  with armcc (via Wine) and masked-word-compares it against the real
  function bytes read straight out of a plaintext `.cro` (see
  `xy-archive/src/cro.rs` — CRO modules need no decryption, just
  `xy archive exefs`/`extract-tree`/romfs to get them onto disk at
  `.Data/Raw_Extracted/romfs/`).
- `matched/DllDialogCommon/` — 14 functions proven to **masked-match** the
  retail binary word-for-word (branch immediates excepted, since those are
  link-layout facts no decompiled source can know ahead of linking):

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x8d0` | `0x2c` | 11/11 | First matched function (2026-10-13). Two calls through an unresolved import veneer. |
  | `0xcdc` | `0x34` | 13/13 | Destructor: vptr install, frees member +0x14, returns `this` (ARM C++ ABI). |
  | `0x688` | `0x3c` | 15/15 | Calls local fn @0x300 on a member, virtual call through slot 8. |
  | `0x6c4` | `0x24` | 9/9 | Virtual call through slot 2; the `r4` push is AAPCS stack-alignment padding, not a real save. |
  | `0x6e8` | `0x20` | 8/8 | Prologue-less leaf (missed by the original function catalog): a tail virtual call (`bx r1`). |
  | `0x2d8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1` (2026-10-15). |
  | `0x2e0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x2e8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x2f0` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence, see note below. |
  | `0x2f4` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0x2f8` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0x2fc` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0xd10` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0xd18` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |

  The six 2026-10-15 stubs at `0x2d8`-`0x2ff` and the pair at `0xd10`-`0xd1f`
  are a run of near-identical 1-2 word leaf functions — likely default
  virtual-method overrides on the dialog-proc base class (`IsXxx() ==
  true/false`, no-op `OnXxx()`). The four single-word `bx lr` stubs are
  **weak-evidence matches**: any void function with no observable side
  effect compiles to that same one word, so the match alone doesn't prove
  the real body was empty — only that it's at most this short.

- `matched/DllNumberInput/` — 6 functions, proving the technique
  generalizes past `DllDialogCommon` to a second module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1888` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x1a40` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`) — weak evidence. |
  | `0x1ab0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0x18dc` | `0x20` | 8/8 | Copies a field into a sub-object, calls a veneer with it, stores the result. |
  | `0x1890` | `0x4c` | 19/19 | Notification dispatcher, two early-outs sharing one epilogue — see note below. |
  | `0x18fc` | `0x78` | 30/30 | Constructor; node-init tail is a moving-pointer-cursor chain, not fixed struct offsets — see note below. |

  Three more functions in this module (`0x1974`, `0x1a44`, `0x1a7c` —
  destructor-stage helpers that install a dtor vtable and conditionally
  free one member) were attempted and reconstructed correctly in
  semantics, but hit the **same vptr-install scheduling residue class**
  already documented for `DllDialogCommon`'s `0xc34`/`0xc68`/`0xca4`:
  armcc 5.04u5 always schedules the vtable pool-load register-first,
  while the retail binary schedules the member load first. Confirms the
  residue class is a toolchain-version property, not a per-function
  fluke — see `docs/decomp/armcc-toolchain.md`.

  `0x1890` is the most instructive specimen so far: its two early-return
  paths share a single `pop {r4, pc}` epilogue with **no** `mov r0, #N`
  before it, because armcc just returns whatever is already live in r0
  at the branch site rather than re-materializing the "obvious" return
  constant. Writing both early returns as literal `return 1;` cost an
  extra instruction; matching required tracing which value was actually
  sitting in the register at each branch (0 at the null-handle check, 1
  at the already-notified check), not which value reads naturally as
  true/false. See the file's header comment for the full trace.

  `0x18fc` is a constructor whose node-initialization tail is written in
  the retail binary as a **sequence of moving-pointer-cursor stores**
  (`str r6,[r0,#92]!` / `add r0,r0,#4` / `str r6,[r0,#-84]!` / `str r5,
  [r0,#-12]!` — each a pre-indexed store that writes and advances the
  pointer in one instruction) rather than fixed-offset struct-member
  access off one stable base. Translating it the same way in C
  (`p += 0x5c; *p = 0; p += 4; p = fn(p); p -= 0x54; *p = 0; p -= 0xc;
  *p = parent;`) matched on the first try. This is the sibling pattern
  to `DllDialogCommon@0x710` (same overall constructor shape) — but that
  specimen's node-init tail uses one `sub` followed by fixed offsets
  instead of a continued cursor chain, and still doesn't match; the two
  specimens turned out to use genuinely different source idioms rather
  than one rule explaining both. See `docs/decomp/armcc-toolchain.md`.

- `matched/DllStrInput/` — 3 functions from a **third** module, confirming
  the trivial-stub/simple-accessor cluster recurs near the tail of every
  Dll module built on the shared dialog-proc base class:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x6148` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x666c` | `0x8` | 2/2 | Trivial byte-field setter, void return. |
  | `0x6674` | `0x14` | 5/5 | Boolean getter through one pointer indirection. |

  This module also has three more `0x1974`-style destructor-stage
  functions (around `0x6298`, `0x6600`, `0x6638`) that were recognized
  as the same already-documented vptr-install-scheduling residue class
  on sight and skipped rather than re-attempted.

- `matched/DllFatalErrorProc/` — 2 functions, a **fourth** module, this
  one unrelated to the `Dll*Input` family (fatal-error dialog proc):

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x2bc` | `0x34` | 13/13 | Destructor: vptr install, free member @+0x14, returns `this`. |
  | `0x284` | `0x38` | 14/14 | Same dtor idiom, but tail-calls a second import (`pop {r4,lr}; b <import>`) instead of returning via `pop {r4,pc}` (2026-10-15). |

  Structurally byte-identical in shape to `DllDialogCommon@0xcdc` (same
  member offset `0x14`, same literal-pool vtable load, same `bl` to a
  masked free import) — confirms this dtor idiom is a shared-base-class
  pattern reused verbatim across unrelated Dll modules, not a coincidence
  tied to the dialog-input family. The module's neighboring `0x284`
  function (the "plausible next easy win" flagged from the first pass)
  has the same body but tail-calls a second import instead of returning
  `this` directly — matching it required a size correction (the real
  function is `0x38` bytes, not `0x34`: its own literal pool sits inside
  that range, right before the next function at `0x2bc` starts) and
  getting the load/store scheduling order right. The retail binary loads
  the vtable literal into `r1` **before** loading `o->buf` into `r0`,
  deferring the `str` until after both loads — the opposite load order
  from `0x2bc` (member first). Writing `vt = VT;` as its own local
  *before* the member read, then `o->vptr = vt;` right after, reproduced
  this exact scheduling; inlining `o->vptr = VT;` directly made armcc
  store eagerly with one reused register instead, which didn't match.

- `matched/DllLangSelect/` — 1 function, a **fifth** module (language
  select menu, unrelated to the `Dll*Input` family):

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x5b0` | `0x1c` | 7/7 | Conditional accessor: veneer call gates between a member load and a re-materialized constant. |

  Real functions in this module start at `0x438` (the import-veneer
  table runs `0x180`-`0x437`, 0x1b4c bytes total for the `.text`
  segment at file offset `0x180`, per the CRO segment table). `0x5b0`
  is notable for *not* reusing the veneer's own boolean result on the
  false branch: `cmp r0,#0; ldrne r0,[r4,#20]; moveq r0,#2` throws the
  call's return value away entirely once it's known nonzero/zero and
  re-materializes the literal `2` — matching pattern #6 from this
  toolchain's residue notes (don't assume the "obvious" value is what's
  actually live in the register).

- `matched/DllConfig/` — 1 function, a **sixth** module, mined from the
  tail stub cluster:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x3a74` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same shape as the other modules' tail-cluster stubs, isolated between
  two destructor-stage helpers that were recognized on sight as the
  vptr-install-scheduling residue class and skipped. `DllConfig@0x31b8`
  (a 5-field struct-copy with no branches/calls) was also attempted: got
  to 5/8 masked-match (the 3-word `ldm` read side matches exactly) but
  the store side never reproduced the retail binary's `str`+`strd` split
  — every C idiom tried (flat field assignment, a local struct-copy
  temp, a nested 3-word sub-struct) either emitted zero merged stores or
  over-merged into one 3-word `stm`/`stmia!`, never the target's
  "store the far field alone, then `strd` the adjacent pair" split.
  Logged as an open item in `attempted.md` rather than re-attempted.

- `matched/DllHeading/` — 1 function, a **seventh** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x28bc` | `0xc` | 3/3 | Trivial field setter: zeroes a byte flag at `this+0xb4`, void return. |

  Sits in the same lone-`bx lr`-stub cluster shape already documented
  for `DllDialogCommon`/`DllStrInput`/`DllLangSelect`: three
  single-instruction no-op stubs immediately precede it, then the next
  function's full prologue starts right after — the trivial-stub/
  simple-setter cluster near a dialog-proc base class's vtable tail
  recurs in a seventh independent module.

- `matched/DllCommWait/` — 1 function, an **eighth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xcd8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same recurring trivial-stub shape as DllConfig/DllNumberInput/
  DllDialogCommon, right after a destructor-stage vptr-install pair
  (`0xc74`/`0xca8`, recognized on sight as the already-documented
  vptr-scheduling residue class and skipped) and right before a 5-case
  property get/set switch at `0xce0` (same jump-table shape as
  `DllDialogCommon@0xdd8`/`DllNumberInput@0x1af8` — not attempted, same
  `push {r4,lr}`/nop-relaxation issue likely applies). Three more lone
  `bx lr` single-word stubs at `0x2a0`/`0x2a4`/`0x2a8` (weak-evidence
  class) not yet claimed.

- `matched/DllSalon/` — 1 function, a **ninth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xdc8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same trivial-stub-cluster pattern seen tail-of-vtable in every other
  Dll module mined so far, right after a destructor-stage vptr-install/
  conditional-free helper (same idiom as DllDialogCommon/DllNumberInput/
  DllFatalErrorProc) ending at `0xdc0` plus its own literal-pool word at
  `0xdc4`; the stub occupies exactly `0xdc8`-`0xdcf`, and the next
  function (`0xdd0`) starts immediately after.

- `matched/DllCommendation/` — 1 function, a **tenth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xc94` | `0x38` | 14/14 | Destructor: vptr install, free member @+0x14 if non-null, tail-calls a second import instead of returning `this`. |

  Identical idiom to `DllFatalErrorProc@0x284`, including the "good"
  load ordering from the toolchain's residue-class rule #4: the vtable
  literal is loaded into `r1` **before** the member load into `r0`, with
  the store to `o->vptr` deferred until after both loads. The module's
  neighboring function at `0xccc` has the same shape but with the known-
  bad member-load-first ordering and was skipped on sight.

- `matched/DllBattleHouseConnect/` — 1 function, an **eleventh** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x2fc` | `0x38` | 14/14 | Same dtor idiom: vptr install, free member @+0x14, tail-call a second import. |

  Same gotcha already documented for `DllFatalErrorProc@0x284`: the
  naive `0x34` size guess omits the function's own literal-pool word
  (at file offset `0x330`, right before the next function's
  `push {r4,lr}` at `0x334`) — real size is `0x38`.

- `matched/DllGameClearSave/` — 1 function, a **twelfth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xd6c` | `0x38` | 14/14 | Same dtor idiom: vptr install, free member @+0x14, tail-call a second import. |

  Same shape and same size gotcha as `DllFatalErrorProc@0x284`/
  `DllBattleHouseConnect@0x2fc`/`DllCommendation@0xc94` (the function's
  own one-word literal pool sits at the 13/14-word boundary right
  before the next function at `0xda4`). Three bare `bx lr` stubs at
  `0xd60`/`0xd64`/`0xd68` precede it (untried), and a near-identical
  sibling dtor at `0xda4` has the known-bad member-load-before-vtable-
  load scheduling order — flagged as unlikely to match, not attempted.

- `matched/DllTheEnd/` — 1 function, a **thirteenth** module (the
  end-credits/ending-sequence Dll, unrelated to the `Dll*Input` family):

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x750` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |

  Same tail-of-vtable stub cluster shape already documented for every
  other mined module: a one-word literal pool at `0x74c` (the previous
  function's epilogue) immediately precedes it, and the next function
  starts right after at `0x758`.

- `matched/DllUS3DDun/` — 1 function, a **fourteenth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x24c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Part of a run of 9 tiny 1-2-word stub functions (`0x248`-`0x273`)
  right after the import-veneer table (`0x180`-`0x247`) and before the
  module's first real function at `0x274` — the recurring trivial-stub
  idiom, this time at the head of real code instead of the tail.

- `matched/DllTrainingAppManager/` — 1 function, a **fifteenth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xd68` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — byte-field setter, void return. |

  Identified directly from the module's own named export table
  (`_ZN9xy_system2bg6System13SetIsCallDrawEb`), which also confirms a
  prior guess (`DllStrInput@0x666c`, same `+0xd9` field shape) was in
  fact a `bool` setter.

- `matched/DllUSDynamo/` — 1 function, a **sixteenth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1b4` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Tiny module (`.text` only `0x2b0` bytes): import-veneer table runs
  `0x180`-`0x1af`, then the stub cluster starts immediately — a lone
  `bx lr` at `0x1b0`, three identical `return 0` stubs at
  `0x1b4`/`0x1bc`/`0x1c4` (only `0x1b4` claimed), two more lone `bx lr`
  at `0x1cc`/`0x1d0`, then a real constructor-shaped function with
  float/VFP math at `0x1d4` (unattempted).

- `matched/DllUSBadgeCheckGate/` — 1 function, a **seventeenth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x228` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same shape, sitting right after the import-veneer table (`0x180`-
  `0x218`) and four bare `bx lr` single-word stubs
  (`0x218`/`0x21c`/`0x220`/`0x224`), with two more identical `return 0`
  stubs immediately following at `0x230`/`0x238` (untried).

- `matched/DllUSPokecen/` — 1 function, an **eighteenth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x24c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same recurring stub cluster right after the import-veneer table
  (`0x180`-`0x23f`): three bare `bx lr` stubs at
  `0x240`/`0x244`/`0x248`, then three identical `return 0` stubs at
  `0x24c`/`0x254`/`0x25c` (only `0x24c` claimed), another bare `bx lr`
  at `0x264`, then the module's first real function (a constructor) at
  `0x268`.

- `matched/DllUSGymGrass/` — 1 function, a **nineteenth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x22c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same cluster shape right after the import-veneer table (`0x180`-
  `0x227`): a lone `bx lr` at `0x228`, three `return 0` stubs at
  `0x22c`/`0x234`/`0x23c` (only `0x22c` claimed), three more lone
  `bx lr` at `0x244`/`0x248`/`0x24c`, then real code at `0x250`.

- `matched/DllUSGymRock/` — 1 function, a **twentieth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1d4` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same cluster shape: a lone `bx lr` at `0x1d0`, three identical
  `return 0` stubs at `0x1d4`/`0x1dc`/`0x1e4` (only `0x1d4` claimed),
  two more lone `bx lr` at `0x1ec`/`0x1f0`, then a real constructor-
  shaped function with VFP/float math at `0x1f4` (unattempted).

- `matched/DllUSMirror/` — 1 function, a **twenty-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1cc` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  One of three identical `return 0` stubs (`0x1cc`/`0x1d4`/`0x1dc`) in
  the tail-of-import-veneer-table leaf-stub cluster (`0x1c8`-`0x1fc`)
  that recurs in every Dll module mined so far.

- `matched/DllUSGymFairy/` — 1 function, a **twenty-second** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x224` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  One of a run of three identical `return 0` stubs at
  `0x224`/`0x22c`/`0x234`, sitting right after a lone `bx lr` stub
  (`0x220`) and before two more `bx lr` stubs (`0x23c`) and the
  module's first real function (`0x240`).

- `matched/DllPssConfig/` — 1 function, a **twenty-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1c20` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — byte-field setter, void return. |

  The module's own named export
  (`_ZN9xy_system2bg6System13SetIsCallDrawEb`), identical shared base/
  mixin-class method already matched at `DllTrainingAppManager@0xd68`
  (same `+0xd9` field).

- `matched/DllPssPlaySelect/` — 1 function, a **twenty-fourth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x2c8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |

  Sits in the recurring tail-of-import-veneer-table leaf-stub cluster:
  24 import veneers (`0x180`-`0x2c4`), then two identical `return 1`
  stubs at `0x2c8`/`0x2d0` (only `0x2c8` claimed), three bare `bx lr`
  stubs at `0x2d8`/`0x2dc`/`0x2e0`, then the module's first real
  function (a notification dispatcher, same shape as
  `DllNumberInput@0x1890`) at `0x2e4`.

- `matched/DllUSR13/` — 1 function, a **twenty-fifth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x24c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same recurring stub cluster shape: a lone `bx lr` precedes it at
  `0x248`, two more identical `return 0` stubs follow at
  `0x254`/`0x25c` (unclaimed), then two bare `bx lr` at
  `0x264`/`0x268`, then a real constructor-shaped function at `0x26c`.

- `matched/DllPssMessageWindow/` — 1 function, a **twenty-sixth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x3c0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |

  One of a run of five tiny leaf stubs at the end of the module's
  import-veneer table (`0x180`-`0x3bf`): a twin `return 1` stub at
  `0x3c8` (unclaimed), three bare `bx lr` stubs at
  `0x3d0`/`0x3d4`/`0x3d8`, then the module's first real function (a
  VFP-using constructor) at `0x3dc`.

- `matched/DllPhotoViewer/` — 1 function, a **twenty-seventh** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1918` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |

  One of a run of seven near-identical 1-2 word leaf stubs
  (`0x1918`-`0x193b`: two `return 1` pairs at `0x1918`/`0x1920`, five
  bare `bx lr` at `0x1928`-`0x1938`), right after a literal-pool word
  at `0x1914` and before a real `push {r4,lr}`-prologue function at
  `0x193c`.

- `matched/DllUSBigFourFire/` — 1 function, a **twenty-eighth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x31c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same recurring stub cluster right after the import-veneer table
  (`0x180`-`0x317`): a lone `bx lr` at `0x318`, three identical
  `return 0` stubs at `0x31c`/`0x324`/`0x32c` (only `0x31c` claimed),
  another lone `bx lr` at `0x334`, then the module's first real
  function (a notification dispatcher) at `0x338`.

- `matched/DllUSBigFourDragon/` — 1 function, a **twenty-ninth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x2cc` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same cluster shape: a lone `bx lr` at `0x2c8`, two more identical
  `return 0` stubs at `0x2d4`/`0x2dc` (unclaimed), two more bare
  `bx lr` at `0x2e4`/`0x2e8`, a prologue-less tail-virtual-call leaf at
  `0x2ec`, another bare `bx lr` at `0x304`, then the module's first
  real function at `0x308`.

- `matched/DllUSBigFourWater/` — 1 function, a **thirtieth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x314` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same cluster shape: a lone `bx lr` at `0x310`, three identical
  `return 0` stubs at `0x314`/`0x31c`/`0x324` (only `0x314` claimed),
  one more lone `bx lr` at `0x32c`, then a real function at `0x330`.

- `matched/DllUSBigFourSteel/` — 1 function, a **thirty-first** module,
  a genuine non-stub win:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x2f4` | `0x18` | 6/6 | Array-indexed virtual dispatch: looks up an owned object in an inline pointer array at `this+0x24`, tail-calls virtual slot 7 on it. |

  The module's first real function, right after its own leaf-stub
  cluster (`0x2d0`-`0x2f0`). Passes the caller's original 3rd argument
  through (`mov r1, r2` before the tail `bx r3`) and the looked-up
  object itself as the new `this` — the first array/vtable-indexing
  specimen matched by this swarm, not another trivial stub.

- `matched/DllUSC02/` — 1 function, a **thirty-second** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x708` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same recurring shape, but this time sitting **mid-module** rather
  than right after the import-veneer table or at the tail: right after
  a constructor-shaped function ending at `0x6fc` plus its own 2-word
  literal pool, and right before a bare `bx lr` stub at `0x710` and a
  one-line accessor/tail-call at `0x714`.

- `matched/DllUSR15/` — 1 function, a **thirty-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x24c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same recurring stub cluster right after the import-veneer table
  (`0x180`-`0x247`): a lone `bx lr` at `0x248`, three identical
  `return 0` stubs at `0x24c`/`0x254`/`0x25c` (only `0x24c` claimed),
  three more lone `bx lr` at `0x260`/`0x264`/`0x268`, then a real
  (VFP-using) function at `0x26c`.

- `matched/DllBattleVideoSave/` — 1 function, a **thirty-fourth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x454` | `0x4` | 1/1 | Prologue-less single-instruction tail-call forwarder (`b <import veneer>`). |

  The entire function body is one `b` to the import-veneer table
  (target masked — a link-time fact), right after a dispatcher
  function (`0x3f0`-`0x448`) and its 2-word literal pool. A void→void
  tail call with no other statements needs no stack frame, so armcc
  5.04u5 at `-O2 -Otime` folds the call straight into a bare `b`
  instead of `bl`+`pop {..,pc}` — same "thin forwarder" idiom as
  `DllDialogCommon@0x6e8` (`bx r1`), but through a fixed branch instead
  of an indirect one.

- `matched/DllUSGymInsect/` — 1 function, a **thirty-fifth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x344` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |

  Same recurring stub cluster: a call-through wrapper at `0x318`
  (`push {lr}; sub sp,#12; bl veneer@0x180; pop {pc}`), a lone `bx lr`
  at `0x340`, this stub, two more identical `return 0` stubs at
  `0x34c`/`0x354` (unclaimed), two more lone `bx lr` at
  `0x35c`/`0x360`, then a real VFP-heavy dispatcher at `0x364`
  (unattempted).

- `matched/DllFriendEntry/` — 1 function, a **thirty-sixth** module,
  a genuine non-stub win:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x4c4` | `0x38` | 14/14 | Destructor: vptr install, free member @+0x14 if non-null, tail-call a second import. |

  Same idiom as `DllFatalErrorProc@0x284`/`DllCommendation@0xc94`/
  `DllBattleHouseConnect@0x2fc`/`DllGameClearSave@0xd6c` (good vtable-
  literal-before-member-load ordering; own literal pool at `0x4f8`
  pushes real size to `0x38`, not `0x34`). The sibling function right
  after this one (`0x4fc`, same shape but member-load-first — the
  known-bad scheduling order) was skipped on sight.

- `matched/DllPssFavorite/` — 1 function, a **thirty-seventh** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x430` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |

  Import-veneer table runs `0x180`-`0x42f` (84 entries, unusually
  large), then this stub and a twin at `0x438`, then three bare
  `bx lr` single-word stubs at `0x440`/`0x444`/`0x448`, then the
  module's first real function (a constructor-shaped dispatcher) at
  `0x44c`.

- `matched/DllCaptureRegister/` — 1 function, a **thirty-eighth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x27b0` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` byte-field setter (`strb r1,[r0,#217]; bx lr`) — identical bytes to `DllTrainingAppManager@0xd68`/`DllPssConfig@0x1c20`/`DllColosseum@0x46fc`, the same shared base/mixin-class method recurring across unrelated modules. |

  Sits in a stub cluster at the module's import-veneer-table tail
  (`0x2728`-`0x2844`): two small dtor-shaped helpers at `0x2744`/`0x277c`
  (skipped), this setter, then two `return 1` stubs (`0x27b8`/`0x27c0`)
  and a run of ~15 identical `return 0` stubs (`0x27c8`-`0x2844`),
  none of those claimed yet.

- `matched/DllBackGround/` — 1 function, a **thirty-ninth** module,
  a genuine non-stub win:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x4e4` | `0x24` | 9/9 | Boolean predicate on a byte field at `this+0x164` — returns 1 if it equals 22, 11, or 63, else 0. |

  Prologue-less leaf (no calls) right after the module's import-veneer
  table (`0x180`-`0x4d7`) and a tiny VFP one-liner at `0x4d8`. A genuine
  three-way-equality compile: `cmp`/`cmpne`/`beq` handles the first two
  equalities, falling through to `cmp`/`movne`/`bne` for the third, both
  paths converging on a shared `mov r0,#1 / bx lr` tail.

- `matched/DllColosseum/` — 1 function, a **fortieth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x46fc` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — same shared setter as `DllCaptureRegister@0x27b0`/`DllTrainingAppManager@0xd68`/`DllPssConfig@0x1c20`, confirmed directly from the module's own named-export table (`_ZN9xy_system2bg6System13SetIsCallDrawEb`, load addr `0x1046fc`). |

- `matched/DllBox/` — 1 function, a **forty-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x9e8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits right after the import-veneer table (`0x180`-`0x9e7`): three
  identical `return 1` stubs at `0x9e8`/`0x9f0`/`0x9f8` (only `0x9e8`
  claimed), a `return 1` stub at `0xa04`, and bare `bx lr`
  weak-evidence stubs at `0xa00`/`0xa08`/`0xa0c`/`0xa10`, before the
  module's first real function (a 10-register-push dispatcher) at
  `0xa14`.

- `matched/DllFieldEventSlopeSlip/` — 1 function, a **forty-second** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x268` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits right after the module's import-veneer table and a real init
  function. Two more promising leads flagged in this same module for a
  future agent: a call-through-then-tail-branch wrapper at
  `0x468`-`0x47f`, and a no-arg globals-init function at
  `0x47c`-`0x49c`.

- `matched/DllBag/` — 1 function, a **forty-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xc80` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Tail of the import-veneer table (`0x180`-`0xc7f`): a run of six
  near-identical 1-2 word leaf stubs follows (`0xc80`/`0xc88`/`0xc90` =
  `return 1`, `0xc98`/`0xca0`/`0xca4`/`0xca8` = bare `bx lr`
  weak-evidence stubs, only `0xc80` claimed), then the module's first
  real function (a 9-register-push dispatcher) at `0xcac`.

- `matched/DllDendouDemo/` — 1 function, a **forty-fourth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x798` | `0x1c` | 7/7 | Conditional accessor: calls an import veneer with `this` unchanged, returns the field at `+32` if the call result is nonzero, else the literal `2`. |

  First real function right after the module's import-veneer table
  (`0x180`-`0x797`); matched on the first attempt. The module's second
  function (`0x7b4`, a larger multi-case dispatcher with VFP float math)
  was not attempted.

- `matched/DllBoxSearch/` — 1 function, a **forty-fifth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x5b8` | `0x1c` | 7/7 | Conditional accessor, same idiom as `DllLangSelect@0x5b0`/`DllDendouDemo@0x798`: veneer call gates between a member load at `+32` and a re-materialized constant `2`. |

  Module's first real function, right after the import-veneer table
  (`0x180`-`0x5b7`).

- `matched/DllBattlePartySelect/` — 1 function, a **forty-sixth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1684` | `0x10` | 4/4 | Prologue-less leaf predicate, `return c == 0` on the third argument (ignores `this` and the second arg). |

  Source phrasing matters for instruction order here, not just value:
  `return c == 0;` (direct or ternary) compiles to the right two
  conditional movs but in the wrong order (`moveq` before `movne`); an
  explicit `if (c != 0) r = 0; else r = 1;` reproduces the retail
  binary's `movne`-before-`moveq` order exactly. Module layout: a
  5-case jump-table dispatcher at `0x3d0` (same `--mask-ranges` shape
  as `DllDialogCommon@0xdd8`, not attempted) and a long constructor at
  `0x50c`; this candidate is the module's last function.

- `matched/DllBattleMatch/` — 1 function, a **forty-seventh** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xd90` | `0x4` | 1/1 | Thin tail-call forwarder — entire body is a single `b <import veneer>` instruction, same idiom as `DllBattleVideoSave@0x454`/`DllDialogCommon@0x6e8`. |

  Sandwiched between two bare `bx lr` no-op stubs (`0xd8c`/`0xd94`) at
  the tail of the import-veneer table. A more interesting neighbor at
  `0xd98` (vptr-install destructor, 0x28 bytes) was attempted and got
  stuck at 5/10: the retail binary hoists the vtable-literal `ldr`
  *before* the `push {r4,lr}` prologue and installs the vptr via a
  post-indexed `str r1,[r0],#8` directly on the parameter register,
  while every C idiom tried made armcc push first and route through
  `r4` instead — a residue class distinct from the already-documented
  vptr-scheduling one (that one is member-load-vs-vtable-load order;
  this one is literal-load-vs-prologue-push order).

- `matched/DllDendouPC/` — 1 function, a **forty-eighth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x650` | `0x1c` | 7/7 | Conditional accessor, same idiom as `DllDendouDemo@0x798`/`DllBoxSearch@0x5b8`: veneer call gates between a member load (here `+36`) and a re-materialized constant `2`. |

  Module's first real function, right after the import-veneer table
  (`0x180`-`0x61f`) and a trivial-stub cluster (`0x620`-`0x64f`: three
  `return 1` stubs, five bare `bx lr` weak-evidence stubs, none claimed).

- `matched/DllBattleVideoPlayer/` — 1 function, a **forty-ninth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xb14` | `0x4` | 1/1 | Thin tail-call forwarder (`b <import veneer>`), same idiom as `DllBattleVideoSave@0x454`/`DllBattleMatch@0xd90`. |

  One of 8 near-identical one-word leaf functions (`0xb14`-`0xb33`:
  alternating `b 0x188` forwarders and bare `bx lr` no-op stubs) right
  after a veneer-calling wrapper ending at `0xb0c`. The other 7 stubs in
  this run (`0xb18`/`0xb1c`/`0xb20`/`0xb24`/`0xb28`/`0xb2c`/`0xb30`) are
  untried.

- `matched/DllEggDemo/` — 1 function, a **fiftieth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x568` | `0x1c` | 7/7 | Conditional accessor, same idiom as `DllDendouDemo@0x798`/`DllBoxSearch@0x5b8`/`DllDendouPC@0x650`: veneer call gates between a member load (here `+24`) and a re-materialized constant `3`. |

  Module's first real function, right after a ~125-entry import-veneer
  table (`0x180`-`0x567`).

- `matched/DllEshop/` — 1 function, a **fifty-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x554` | `0x38` | 14/14 | Destructor: installs a new vtable, frees owned member at `+0x14` if non-null, tail-calls a second import instead of returning `this` directly. |

  Identical idiom to `DllFatalErrorProc@0x284`/`DllCommendation@0xc94`/
  `DllBattleHouseConnect@0x2fc`/`DllGameClearSave@0xd6c`/
  `DllFriendEntry@0x4c4`: same "good" load ordering (vtable literal into
  `r1` before the member load into `r0`), same size gotcha (own literal
  pool word at `0x588` pushes real size to `0x38`, not the naive
  `0x34`). Mined from the stub cluster right after the import-veneer
  table (`0x180`-`0x52f`); two trivial `mov r0,#1; bx lr` stubs at
  `0x530`/`0x538` and six bare `bx lr` weak-evidence stubs at
  `0x53c`-`0x550` precede it, unclaimed. A sibling dtor-shaped function
  immediately follows at `0x58c` (same shape, load-order not yet
  checked).

- `matched/DllFieldEventEntranceIn/` — 1 function, a **fifty-second**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x628` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits mid-module rather than right after the import-veneer table
  (`0x180`-`0x2df`, 44 entries): a literal-pool word precedes it at
  `0x624`, two bare `bx lr` weak-evidence stubs follow at
  `0x630`/`0x634`, and a boolean getter starts right after at `0x638`.

- `matched/DllFieldEventPlayer/` — 1 function, a **fifty-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x624` | `0x8` | 2/2 | Trivial int field setter (`*(int*)((char*)this+32) = v;`). |

  A two-field setter at `0x494` (`str r1,[r0,#28]; strb r2,[r0,#32]; bx
  lr`, size `0xc`) looks equally tractable and wasn't attempted; two
  dispatcher functions at `0x4ac` and `0x63c` also look promising.

- `matched/DllFieldDemo/` — 1 function, a **fifty-fourth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1edc` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — same shared setter as `DllTrainingAppManager@0xd68`/`DllPssConfig@0x1c20`/`DllCaptureRegister@0x27b0`/`DllColosseum@0x46fc`. |

  An untried `return 1` stub sits right next door at `0x1ee4`, and a
  small struct-zero init function at `0x1eec`-`0x1f0c`.

- `matched/DllFieldEventPokeRideAction/` — 1 function, a **fifty-fifth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x520` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits mid-module: a field-copy-and-store helper at `0x514`
  (`add r0,r0,#0x24; stm r0,{r1,r2}; bx lr`, untried, looks tractable)
  precedes it, a real `push {r4,r5,r6,lr}` function follows at `0x528`.

- `matched/DllGameSync/` — 1 function, a **fifty-sixth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x5e0` | `0x38` | 14/14 | Destructor — same idiom already proven in `DllFatalErrorProc@0x284`/`DllCommendation@0xc94`/`DllBattleHouseConnect@0x2fc`/`DllGameClearSave@0xd6c`/`DllFriendEntry@0x4c4`/`DllEshop@0x554`: installs a new vtable, frees an owned member at `+0x14` if non-null, tail-calls a second import instead of returning `this` directly. |

  An unclaimed easy follow-up: a trivial `return 1` stub at `0x5c8` in
  the same module.

- `matched/DllFieldEventSkatePipe/` — 1 function, a **fifty-seventh**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1b28` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Two more promising, unclaimed candidates in the same module: a lone
  `bx lr` stub at `0x1b30` (weak evidence), and a genuine field
  accessor at `0x1b34` (`ldr r1,[r1,#28]; add r1,r1,#288; str
  r1,[r0,#36]; bx lr`, size `0x10`).

- `matched/DllFieldEventTakinobori/` — 1 function, a **fifty-eighth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x11d4` | `0x8` | 2/2 | Trivial int field setter: `str r1,[r0,#24]; bx lr` — stores the 2nd argument into the object at `+0x18`, void return. |

  A twin `return 1` stub sits right next door at `0x11dc`, untried.

- `matched/DllIntro/` — 2 functions, a **fifty-ninth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x9eb0` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — same shared setter as `DllTrainingAppManager@0xd68`/`DllPssConfig@0x1c20`/`DllColosseum@0x46fc`/`DllCaptureRegister@0x27b0`/`DllFieldDemo@0x1edc`. |
  | `0x868` | `0x4` | 1/1 | Thin tail-call forwarder (`b <import veneer>`), same idiom as `DllBattleVideoSave@0x454`/`DllBattleMatch@0xd90`/`DllBattleVideoPlayer@0xb14`. |

  Two untried leads in the `0x868` cluster: a call-through wrapper at
  `0x858` (`push {r4,lr}; bl <local 0x66cc>; pop {r4,lr}; b <import
  veneer>`) and a bare `bx lr` stub at `0x86c`.

- `matched/DllGift/` — 1 function, a **sixtieth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xae0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits right after DllGift's unusually large (~696-entry) import-veneer
  table (`0x180`-`0xadf`); six bare `bx lr` single-word stubs follow at
  `0xae8`-`0xaf8` (weak-evidence class, unclaimed), then the module's
  first real function at `0xafc` (a destructor-stage helper, already-
  documented vtable-install/free-member idiom), and a second similar
  dtor at `0xb34`.

- `matched/DllHologramMail/` — 1 function, a **sixty-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xf0c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Right after the import-veneer table (`0x180`-`0xf07`) and a bare `bx
  lr` no-op stub at `0xf08`. The module's first real function at
  `0xf14` is a destructor-shaped helper with two conditional vtable
  calls, untried.

- `matched/DllGts/` — 1 function, a **sixty-second** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1b7e4` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)`, confirmed via the module's own named export (`_ZN9xy_system2bg6System13SetIsCallDrawEb`). |

  Candidate compiled as `.cpp` (not `.c`) — the C++ `namespace`
  wrapper needed for the demangled symbol name requires armcc's C++
  mode, which it selects from the file extension.

- `matched/DllKisekae/` — 1 function, a **sixty-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xb6e0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits right after a destructor-stage vptr-install helper; an
  identical unclaimed twin stub follows at `0xb6e8` and an untried
  hardcoded-true byte setter at `0xb6f0`.

- `matched/DllPCMenu/` — 1 function, a **sixty-fourth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x6b8` | `0x1c` | 7/7 | Conditional accessor, same idiom as `DllLangSelect@0x5b0`/`DllDendouDemo@0x798`/`DllBoxSearch@0x5b8`/`DllDendouPC@0x650`/`DllEggDemo@0x568`. |

  Sits right after the module's first real function (`0x550`-`0x6b0`,
  a larger unattempted dispatcher/state-machine) and that function's
  own one-word literal pool at `0x6b4`.

- `matched/DllLiveCup/` — 1 function, a **sixty-fifth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x764` | `0x38` | 14/14 | Destructor — same idiom already proven in `DllFatalErrorProc@0x284`/`DllCommendation@0xc94`/`DllBattleHouseConnect@0x2fc`/`DllGameClearSave@0xd6c`/`DllFriendEntry@0x4c4`/`DllEshop@0x554`/`DllGameSync@0x5e0`: installs a new vtable, frees an owned member at `+0x14` if non-null, tail-calls a second import instead of returning `this` directly. |

  Unclaimed nearby candidates: trivial `return 1` stubs at
  `0x738`/`0x740`/`0x748`, bare `bx lr` stubs at `0x750`-`0x760`, and
  the known-bad member-load-first sibling dtor at `0x79c`.

- `matched/DllKawaigari/` — 1 function, a **sixty-sixth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x3fc6c` | `0x30` | 12/12 | Prologue-less zero-argument leaf: follows a nested global pointer chain (`g_outer.mid->inner`), sets that struct's `+0x24` int field to 1 if non-null, unconditionally sets a second global byte flag to 1, always returns 1. |

  `DllKawaigari`'s `.text` is large (`0x43bbc` bytes) with an unusually
  long import-veneer table (ends ~`0xd84`); plenty of unmined real
  functions remain, e.g. the thin-forwarder cluster right after the
  veneer table at `0xd88`-`0xdd8`.

- `matched/DllNuts/` — 1 function, a **sixty-seventh** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x3384` | `0xc` | 3/3 | Trivial field setter: unconditionally zeroes a byte flag at `this+156` (`0x9c`), void return. |

  Three bare `bx lr` single-word stubs (`0x3378`/`0x337c`/`0x3380`,
  weak-evidence class) sit right before it, unclaimed. See
  `attempted.md` for a new VFP `vldm`/`vstm` residue class found (not
  solved) on this module's other two candidates.

- `matched/DllKawaigariArrange/` — 1 function, a **sixty-eighth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x773c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Found as a clean 2-word stub between two real functions in a module
  whose import-veneer table is unusually large (`0x180`-`0x647`) and
  whose other functions are mostly larger/non-trivial.

- `matched/DllPokeList/` — 1 function, a **sixty-ninth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x8dc` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits right after the import-veneer table (`0x180`-`0x8db`) in a
  cluster of near-identical leaf stubs (more `return 1` stubs at
  `0x8e4`/`0x8ec`/`0x8f8`, bare `bx lr` weak-evidence stubs at
  `0x8f4`/`0x900`/`0x904`/`0x908`/`0x90c`, all unclaimed), followed by
  the module's first real function (a constructor-shaped dispatcher)
  at `0x910`.

- `matched/DllPoke3Select/` — 1 function, a **seventieth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x63c` | `0x28` | 10/10 | Trivial constructor: installs a vtable pointer, zeroes fields `+0x28`/`+0x34`/`+0x38`, sets a byte flag at `+0x30` to 1 — no calls or branches. |

  Register-role trap: armcc only lands the retail binary's exact
  `r1`(=0, reused 3×)/`r2`(=1, then reused for the vtable literal)
  split when the two constants are declared as separate named locals
  assigned before any field write; inline immediates or a single named
  local gave the right shape/word-count but swapped `r1`/`r2` (2/10,
  not 10/10).

- `matched/DllPssAssistPower/` — 1 function, a **seventy-first**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x618` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits right after the import-veneer table (`0x180`-`0x617`) in a
  cluster of near-identical leaf stubs.

- `matched/DllPssCharaMenu/` — 1 function, a **seventy-second** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x5e0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Sits right after the import-veneer table (`0x180`-`0x5df`), with an
  unclaimed identical twin at `0x5e8` and two bare `bx lr`
  weak-evidence stubs at `0x5f0`/`0x5f4` before the module's first
  real function at `0x5f8`.

- `matched/DllPromotion/` — 1 function, a **seventy-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xc78` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Found at the tail of the import-veneer table right after a small
  constructor and a float-field-init leaf, followed by an unclaimed
  run of six bare `bx lr` stubs and then the module's first real
  function at `0xc94`.

- `matched/DllPssIconSelect/` — 1 function, a **seventy-fourth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x540` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Right after the module's 120-entry import-veneer table
  (`0x180`-`0x53f`). A twin `return 1` stub at `0x548` and three bare
  `bx lr` stubs (`0x550`/`0x554`/`0x558`) remain unclaimed, followed by
  the module's first real function at `0x55c`.

- `matched/DllUSGymElec/` — 1 function, a **seventy-fifth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x268` | `0x8` | 2/2 | Trivial virtual-override stub, `mov r0,#0; bx lr` — returns 0 (false), ignores `this`. |

  Same tail-of-import-veneer-table stub cluster shape already seen in
  `DllUSGymRock`/`DllUSGymGrass`/`DllUSGymFairy`/`DllUSGymInsect` (28
  veneers `0x180`-`0x25f`, then a run of `bx lr`/`return 0` stubs; this
  is the first of three identical `return 0` stubs at
  `0x268`/`0x270`/`0x278`, only `0x268` claimed).

- `matched/DllPssMainMenu/` — 1 function, a **seventy-sixth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x428` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Right after the import-veneer table (`0x180`-`0x427`); an identical
  unclaimed twin sits at `0x430`, two bare `bx lr` weak-evidence stubs
  at `0x438`/`0x43c`, then the module's first real function (a
  multi-veneer dispatcher) at `0x440`.

- `matched/DllPssRuleSelect/` — 1 function, a **seventy-seventh**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x318` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Right after the import-veneer table (`0x180`-`0x317`, 51 entries);
  an identical twin stub immediately follows at `0x320`, two bare
  `bx lr` weak-evidence stubs at `0x328`/`0x32c`, then the module's
  first real function at `0x330` — a conditional field-reset leaf
  (checks a byte field for value 4, zeroes two bytes if so), with a
  near-twin set-to-4 variant at `0x354`, both untried.

- `matched/DllPuzzle/` — 1 function, a **seventy-eighth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xae8` | `0x1c` | 7/7 | Prologue-less leaf, conditional field setter: chases `this->obj1` (`+4`) `->obj2` (`+0x2c`), reads a byte at `+0x60`, and if it equals 1, writes the literal 2 into a byte field at `this+8`. Void return. |

  First real function in the module, right after the import-veneer
  table (ends at `0xae4`).

- `matched/DllRandomTrade/` — 1 function, a **seventy-ninth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x8b4` | `0x38` | 14/14 | Destructor — same idiom proven in 8 other modules: installs a new vtable, frees an owned member at `+0x14` if non-null, tail-calls a second import instead of returning `this` directly. |

  Six unclaimed trivial stubs immediately precede it (`0x890`/`0x898`
  = `return 1`, `0x8a0`-`0x8b0` = four bare `bx lr`), and a
  near-identical sibling dtor sits right after at `0x8ec`
  (member-load-vs-vtable-load order not yet checked).

- `matched/DllSparring/` — 1 function, an **eightieth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x10178` | `0x8` | 2/2 | Trivial virtual-override stub, `return 8`, ignores `this`. |

  One of three near-identical 2-word constant-getter stubs right after
  the module's second real function (a dispatcher ending at `0x10174`):
  `return 8` (claimed), `return 16` at `0x10180` and `return 4` at
  `0x10188` (both unclaimed). Module layout: a 244-entry import-veneer
  table (`0x180`-`0x91f`), then a large unattempted constructor at
  `0x920` (~`0x8c0` bytes).

- `matched/DllShinkaDemo/` — 1 function, an **eighty-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xfdc` | `0x8` | 2/2 | Trivial signed-byte field getter: `return *(signed char*)(obj+0x82);`. |

  Right after a dispatcher function (ends `0xfd4`) and its literal pool
  (`0xfd8`). An identical twin getter for the adjacent byte (`+0x83`)
  sits at `0xfe4`, unclaimed.

- `matched/DllReport/` — 1 function, an **eighty-second** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x728` | `0x1c` | 7/7 | Conditional accessor, same idiom as `DllLangSelect@0x5b0`/`DllDendouDemo@0x798`/`DllBoxSearch@0x5b8`/`DllDendouPC@0x650`/`DllEggDemo@0x568`/`DllPCMenu@0x6b8`: veneer call gates between a member load (here `+84`) and a re-materialized constant `2`. |

  Module's first real function, right after its import-veneer table
  (`0x180`-`0x727`).

- `matched/DllSequence/` — 1 function, an **eighty-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1135c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Inside the recurring tail-of-import-veneer-table leaf-stub cluster
  (bare `bx lr` stubs on both sides). Module has an unusually large
  import-veneer table; three more untried `return 1`-shaped leads at
  `0xee90`, `0xef98`, `0x10a88`.

- `matched/DllField/` — 1 function, an **eighty-fourth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x58c8` | `0xc` | 3/3 | Trivial hardcoded-true byte-field setter: unconditionally sets `this+0x1c` to 1, ignores any 2nd argument, void return. |

  Right after DllField's large import-veneer table (`0x180`-`0x5767`)
  and a run of small veneer-calling wrappers/float helpers
  (`0x5740`-`0x58c4`); plenty of unmined ground remains in this module
  (~9,227 functions — one of the largest CRO modules).

**2026-10-15: first batch under the multi-function-per-agent model** (up
to 5 functions per spawn instead of exactly 1) — 5 agents landed 24
functions across 5 new modules in one round, each reusing its own
module-disassembly cost across several matches instead of one.

- `matched/DllSparringMenu/` — 4 functions, an **eighty-fifth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x5b0c` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — same shared setter as 7 other modules. |
  | `0x5b14` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x5ad8` | `0x34` | 13/13 | Destructor — same idiom as `DllDialogCommon@0xcdc` and 8+ other modules: vptr install, free member `+0x14` if non-null, returns `this`. |
  | `0x5b58` | `0x34` | 13/13 | This-pointer-adjusted conditional field setter: receiver is `(char*)this - 0x84`; clamps `v==1/16/17` to `2/3/4` stored at `+0x8c`, else leaves it untouched. |

  Sibling at `0x5b1c` (same `cmp`/`moveq`/`beq` cascade as `0x5b58`, plus
  an extra unconditional zero-store to a second field at `+0x88`/`+0x8c`)
  got stuck at 9/15: armcc folds that store's address to a direct
  `this+8` offset instead of routing it through the same adjusted base
  register used for the rest of the function — every straightforward C
  translation produces the folded form instead. Possibly a this-adjusting
  -thunk codegen artifact; not logged to `attempted.md` (wasn't a full
  investigation), just flagged here for whoever picks up this module next.

- `matched/DllStatus/` — 5 functions, an **eighty-sixth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xbc8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x6f6c` | `0x4` | 1/1 | Thin tail-call forwarder (`b <import veneer>`), same idiom as `DllBattleVideoSave@0x454`/`DllBattleMatch@0xd90`. |
  | `0x871c` | `0x4` | 1/1 | A second, independent thin-forwarder specimen in the same module. |
  | `0xa370` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`, ignores `this`. |
  | `0xa4e0` | `0xc` | 3/3 | Byte-field setter with an unusual arg convention: the object pointer arrives in the **3rd** register slot (`r2`), not `r0`/`this` — reproduced in C with an `(int, int, Obj*)` signature. |

  Tail-of-import-veneer-table cluster at `0xbc8`-`0xbff` has 4 more
  identical `return 1` stubs and 4 bare `bx lr` weak-evidence stubs left
  unclaimed.

- `matched/DllSubScreenTutorial/` — 5 functions, an **eighty-seventh**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x538` | `0x10` | 4/4 | Prologue-less tail virtual call (`o = this->p38; return o->vtbl[9](o)`), same idiom as `DllDialogCommon@0x6e8` but a direct member offset, no null check. |
  | `0x12a0` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x12cc` | `0x38` | 14/14 | Destructor — same proven idiom as `DllFatalErrorProc@0x284` etc. |
  | `0x1338` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`, ignores `this`. |
  | `0x954` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`, ignores `this`. |

  Plenty unmined: a few dispatcher/constructor-shaped functions flagged
  but not attempted (`0x358` jump-table dispatcher, `0x4cc` 3-case
  switch, `0x548` larger dispatcher, a `0x6e0`/`0x734` matched
  destructor-pair idiom with an extra two-base-class dtor call). The
  known-bad member-load-first destructor twin at `0x1304` was correctly
  skipped on sight per the documented residue class.

- `matched/DllStartMenu/` — 5 functions, an **eighty-eighth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x480` | `0x40` | 16/16 | Init helper: calls an import veneer on `this->f24` with arg 0, sets `this->f34=1` and `this->f4->f2=1`, then tail-calls a second veneer with constants `(0x60065, 0, -1, 0)`, returning its result directly. |
  | `0x4c0` | `0x38` | 14/14 | Guard check: `if (fnC(this)==0) return 2; else if (this->f16==3) veneer(this->f24, 1); return this->f16;`. |
  | `0x1b74` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x1b7c` | `0x4` | 1/1 | Trivial no-op void stub (`bx lr`), weak evidence — one of a run of five identical 1-word stubs at `0x1b7c`-`0x1b8c`. |
  | `0x1bfc` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`, ignores `this`. |

  Easy follow-ups left unclaimed: the `0x1b7c` cluster's siblings at
  `0x1b80`/`0x1b84`/`0x1b88`/`0x1b8c`. A large (~`0x214`-byte)
  multi-branch function at `0x4f8` and a small constructor-shaped
  function around `0x1ae0` were spotted but not attempted.

- `matched/DllStaffRoll/` — 5 functions, an **eighty-ninth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x59c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x2c44` | `0x38` | 14/14 | Destructor — same proven idiom as `DllFatalErrorProc@0x284` etc. |
  | `0x2304` | `0x10` | 4/4 | Chained thin forwarder: `return veneer(local(this));` (`bl` into a local function, tail-call `b` into another with the result). |
  | `0xe90` | `0x10` | 4/4 | Calls a function on `this` (result discarded), unconditionally returns 1. |
  | `0x2cb0` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — same shared setter as 6 sibling modules. |

  Unmined lead: a more complex double-vtable-install destructor around
  `0x28c8`/`0x289c` (two local destructor calls before the member-free,
  distinct from the simple single-vtable idiom), plus an untried
  `return 0` stub at `0x2cb8` right next to the last match.

- `matched/DllWazaOboe/` — 5 functions, a **ninetieth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x7ec` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x804` | `0x4` | 1/1 | Bare `bx lr` no-op void stub, weak evidence. |
  | `0x82c` | `0xc` | 3/3 | Field setter + return 1: sets a byte flag at `+12` of the object in `r2` (3rd arg), returns 1. |
  | `0x1020` | `0x4` | 1/1 | Thin tail-call forwarder (`b <import veneer>`), same idiom as `DllBattleVideoSave`/`DllBattleMatch`. |
  | `0x1024` | `0x4` | 1/1 | Bare `bx lr` no-op void stub right after the forwarder, weak evidence. |

  All from the recurring tail-of-import-veneer-table leaf-stub cluster
  (`0x7ec`-`0x837`) plus a second small stub run at `0x1020`-`0x1027`
  further into the module. The module's real functions (constructor-
  shaped, `push {r4-r8,lr}; vpush {d8}`, starting at `0x838` and
  `0x1028`) were not attempted.

- `matched/DllTrialDownload/` — 5 functions, a **ninety-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x728` | `0x38` | 14/14 | Destructor — same proven idiom as `DllFatalErrorProc@0x284` etc. |
  | `0x700` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x6fc8` | `0x8` | 2/2 | Thin tail-call forwarder with a constant arg (`r0=6`) into an import veneer. |
  | `0x6fd0` | `0x8` | 2/2 | Twin of the above — same constant arg, different import. |
  | `0x6fd8` | `0x14` | 5/5 | Prologue-less leaf: zeroes two int fields + one short field of the arg struct, void return. |

  All at the tail of the module's `.text` (import-veneer table runs
  `0x180`-`0x6ff`; real code is sparse there — mostly one large
  state-machine dispatcher). Untried leads: bare `bx lr` weak-evidence
  stubs at `0x708`/`0x70c`/`0x714`/`0x718`/`0x71c`/`0x720`/`0x724`, and
  a sibling destructor at `0x760` with the known-bad member-load-first
  ordering (skipped on sight).

### Tool addition: `--mask-ranges` for jump-table data (2026-10-15)

Some larger dispatcher functions (`DllDialogCommon@0x300`/`0xdd8`,
`DllNumberInput@0x1af8`, ...) compile a `switch` into a jump table: a
run of raw address words immediately after an `ldrcc pc,[pc,rX,lsl#2]`
instruction. In the unlinked CRO those words are zero placeholders (the
real addresses are a link-time fact), so they can never byte-match an
isolated compile. `fingerprint_fn.py` now takes `--mask-ranges
offset:length[,offset:length...]` (byte ranges relative to the
function's own start) to exclude exactly those words from comparison.

**Important limitation, not yet resolved:** masking the table proves the
function has the right *number* of case-body blocks, each byte-correct
in the right physical order — it does **not** prove which case *value*
(0, 1, 2, ...) selects which block, because that mapping is exactly what
the masked table encodes. Any case-number assignment in a candidate that
uses this flag is inferred from physical body order (on the assumption
armcc emits bodies in ascending case-value order) and must be documented
as a hypothesis, never asserted as fact. The tool prints this caveat at
runtime whenever `--mask-ranges` is used.

First attempt with it, `DllDialogCommon@0xdd8` (a 5-case property
setter/getter switch with no function calls at all), did not reach a
full match: the retail binary has a `push {r4, lr}` / `pop {r4, pc}`
frame the candidate didn't need (no calls in my reconstruction to
justify saving `lr`), and the default case's three sequential pool
loads are interleaved with `e320f000` NOP-hint words in the retail
binary — the project's own marker for post-link branch-relaxation nops
— suggesting the real default case made calls that got relaxed away
entirely, not just argument-setup loads. Left as an open item; no
case-to-block mapping was ever asserted for it since it never matched.

## Working on this (rules for agents picking up a function)

- **30-minute cap per agent.** If you haven't reached a masked match
  within about 30 minutes of starting, stop regardless of where you are
  — don't keep grinding on one hard function. Write up what you tried
  and where it got stuck as a `pending/attempted-<Module>-<offset>.md`
  note (see below — NOT a direct edit to `attempted.md`), commit and
  push just that note, and end your report saying you hit the time cap
  without a match. This frees the slot for a fresh agent to work on
  something new next tick instead of duplicating your effort.
- **Check `attempted.md` before picking a candidate.** If a function is
  already logged there, skip it unless you have a genuinely new idea
  beyond what's already listed.
- **Do not edit `README.md`, `docs/README.md`, `docs/decomp/README.md`,
  or `attempted.md` directly — several agents run concurrently on this
  branch, and those files all have a single running-total line or
  append point every agent would try to touch at once, which is a
  guaranteed git conflict, not a maybe.** Instead, on a match, write a
  tiny uniquely-named note to `pending/matched-<Module>-<offset>.md`
  (format in `pending/README.md`) alongside your `matched/<Module>/
  <offset>.c` file, and commit+push both together. The orchestrating
  session folds pending notes into the real docs on its own schedule —
  that's not your job. A function that DOES match in under 30 minutes:
  just save the `.c` file + the pending note, commit, push, done.

## Reproducing a match in this repo

```sh
# One-time: get the plaintext CRO onto disk (the game needs no AES key —
# CROs are plaintext, see docs/3ds/ + xy-archive/src/cro.rs).
./target/release/xy archive romfs .Data/Rom/X/Base/baserom_x.cci .Data/Raw_Extracted/romfs

# armcc 5.04u5 "for Nintendo" is a proprietary, shared install outside any
# repo — set ARMCC_HOME if it isn't at the tool's default
# (~/Documents/decomp/tools/armcc/ARMCC_Nintendo_5). Needs Wine.
python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
    DllDialogCommon 0x8d0 0x2c \
    xy-decomp/armcc-matching/matched/DllDialogCommon/0x8d0.c
# -> target 11 words, candidate 11 words, 11/11 match (b/bl targets masked)
# -> *** MASKED MATCH ***
```

All 5 specimens above were re-verified against this repo's own freshly
extracted `DllDialogCommon.cro` on migration day and reproduce their
masked match exactly — the toolchain and verifier are proven portable,
not a one-off result tied to the sibling repo's layout.

## What this does *not* do yet

This is per-function fingerprinting (prove a candidate's instruction
stream matches, modulo branch targets), not whole-module linking. Getting
from "5 matched functions" to "DllDialogCommon fully decompiled" needs the
remaining ~13,549 cataloged functions (project-wide: 38,385) recovered and
fingerprinted the same way, then an actual CRO relink to confirm full byte
identity including resolved branch targets. See the sibling repo's
`docs/progress.md` for the live residue-class/puzzle list (some functions
hint the *true* original compiler may be an even earlier armcc than
5.04u5 — open item, not yet closed).
