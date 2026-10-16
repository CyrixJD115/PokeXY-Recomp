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
'*.c' -o -name '*.cpp' | wc -l; done | awk '$1>1' | wc -l` — **41 of 113**
as of this update: `DllDialogCommon`, `DllNumberInput`, `DllStrInput`,
`DllFatalErrorProc`, `DllIntro`, `DllSparringMenu`, `DllStatus`,
`DllSubScreenTutorial`, `DllStartMenu`, `DllStaffRoll`, `DllWazaOboe`,
`DllTrialDownload`, `DllTradelist`, `DllZukan`, `DllTitle`,
`DllTownmap`, `DllTradeDemo`, `DllTrainerCase`, `DllUSGymEsper`,
`DllTrialHouseResult`, `DllUSGymFighting`, `DllUSKinomi`,
`DllUSPassage`, `DllVSDemo`, `DllUSGymIce`, `DllWazaOmoidashi`,
`DllBattle`, `DllBattleMatch`, `DllSequence`, `DllField`, `DllGts`,
`DllBox`, `DllHologramMail`). Many of the
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

- `matched/DllBox/` — 6 functions, a **forty-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x9e8` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x9f0` | `0x8` | 2/2 | Same `return 1` stub, 2nd of the three identical siblings. |
  | `0x9f8` | `0x8` | 2/2 | Same `return 1` stub, 3rd of the three. |
  | `0xa00` | `0x4` | 1/1 | Bare `bx lr` no-op void stub (weak evidence). |
  | `0xa04` | `0x8` | 2/2 | Another `return 1` stub, interleaved in the bare-`bx lr` run right before the module's first real function. |
  | `0x1f46c` | `0x10` | 4/4 | Call-through-then-tail-call wrapper: two fixed no-arg calls, second compiles as a tail `b` since it's the function's last statement; `r4` push/pop is AAPCS stack-alignment padding. Found via `find_candidates.py`. |

  Sits right after the import-veneer table (`0x180`-`0x9e7`): all three
  identical `return 1` stubs at `0x9e8`/`0x9f0`/`0x9f8` are now claimed,
  plus `0xa00`/`0xa04`, before the module's first real function (a
  10-register-push dispatcher) at `0xa14` — unattempted, worth a future
  agent's look.

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

- `matched/DllBattleMatch/` — 6 functions, a **forty-seventh** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xd90` | `0x4` | 1/1 | Thin tail-call forwarder — entire body is a single `b <import veneer>` instruction, same idiom as `DllBattleVideoSave@0x454`/`DllDialogCommon@0x6e8`. |
  | `0xd8c` | `0x4` | 1/1 | Trivial no-op void stub, single `bx lr` (weak evidence — any void no-op compiles the same). Right before `0xd90`. |
  | `0xd94` | `0x4` | 1/1 | Trivial no-op void stub, single `bx lr` (weak evidence). Immediately follows `0xd90`. |
  | `0xde4` | `0x4` | 1/1 | Thin tail-call forwarder, same idiom as `0xd90`/`DllBattleVideoSave@0x454`/`DllDialogCommon@0x6e8`. |
  | `0xde8` | `0x4` | 1/1 | Trivial no-op void stub, single `bx lr` (weak evidence). Right after `0xde4`. |
  | `0xe38` | `0x10` | 4/4 | Two-call helper: `import_a(o)` then a tail call to an argument-less `import_b()`. Gotcha: the second call must take NO argument — passing `o` to both forces armcc to emit `mov r4,r0`/`mov r0,r4` save/restore pairs the retail binary doesn't have; once `import_b` is `void(void)`, `o` isn't live across the first call and `pop {r4,lr}; b <import_b>` falls out naturally. |

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

- `matched/DllHologramMail/` — 5 functions, a **sixty-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xf0c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0xcccc` | `0x24` | 9/9 | Field-copy-and-call: derefs `this+20` at `+136`, calls a local helper with literal 13, stores literal 5 into `this+28`. Found via `find_candidates.py`. |
  | `0xe970` | `0x24` | 9/9 | Calls an import veneer with `this+8` and 0, sets `this+17`/`this+16` to literal 2. Lever: wrapping the two stores in a tiny static `set_flags(T*,v)` helper pins the post-call constant into `r1` instead of the dead `r0` — a naive flat assignment only got 6/9. |
  | `0xd424` | `0x24` | 9/9 | Derefs `this+36` at `+4`, calls an import veneer with the function's 3rd argument plus that object, stores literal 11 into `this+40`. |
  | `0xd13c` | `0x24` | 9/9 | Byte-identical shape to `0xd424` (masked callee resolves elsewhere) — same candidate reused unmodified. |

  Right after the import-veneer table (`0x180`-`0xf07`) and a bare `bx
  lr` no-op stub at `0xf08`. The module's first real function at
  `0xf14` is a destructor-shaped helper with two conditional vtable
  calls, untried. Attempted: `0x10da8` (same idiom as `0xd424`/`0xd13c`
  but with a `this -= 4` adjustment first) — stuck at 2/9, see
  `attempted.md`; likely blocks siblings `0x10dcc` and the first half of
  the `0x10cf0` compound candidate too.

- `matched/DllGts/` — 6 functions, a **sixty-second** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1b7e4` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)`, confirmed via the module's own named export (`_ZN9xy_system2bg6System13SetIsCallDrawEb`). |
  | `0x5be0` | `0x3c` | 15/15 | Call+return helper: forwards two fields to an import veneer, zeroes two members, calls a second veneer, returns constant 56. Scheduling trap: armcc evaluates the two memory-backed call args in left-to-right source order while deferring the two constant args to the end in reverse order — writing the call inline or assigning locals in the wrong order flips three words. |
  | `0x5ba8` | `0x1c` | 7/7 | Double-pointer-deref call+return: `fn4f4(**field10, field30)`, returns `0x1b`. Load-latency-hiding trap: retail does a pre-indexed load (`ldr r2,[r0,#16]!`) advancing the base while fetching field10, then a plain offset load for field30 — one expression or a manually-advanced cursor produces plain non-writeback loads instead (3/7 off); fix is separate local-variable statements in that exact order. |
  | `0x5bc4` | `0x1c` | 7/7 | Identical sibling of `0x5ba8`, returns `0x26` instead of `0x1b`. |
  | `0x53e4` | `0x1c` | 7/7 | Third sibling of `0x5ba8`/`0x5bc4`, returns `0x13`. **`find_candidates.py` size-hint correction**: the tool (pre-trim-fix) flagged `0x24`; real/Ghidra boundary is `0x1c` — the extra 8 bytes are dead padding before the next function's prologue, not part of this one. |
  | `0x8850` | `0x28` | 10/10 | Double-indirect call+return: `fn284(this->field20->field5c, 1,1,1,1)`, returns 8. Register-CSE-master trap: armcc's constant-CSE for the four identical `1` args defaults to materializing into `r1` and copying outward, but retail materializes into `r2` — naming only the arg3 occurrence as its own local nudges the CSE to pick `r2` as master, matching retail exactly. Sibling at `0x12afc` (field `+0x24`, returns 7) uses the identical template — unclaimed, flagged as a quick pickup. |

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

- `matched/DllSequence/` — 6 functions, an **eighty-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1135c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0xee90` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. First of the module's previously-flagged untried leads. |
  | `0xef98` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. Second of the previously-flagged untried leads. |
  | `0x10a80` | `0x10` | 4/4 | Chased-pointer "touch then always return true" accessor (`this->inner->field0xcc`, result discarded, returns 1). The third flagged lead was mis-recorded as `0x10a88`/`0x8` (trailing stub words only) — real function starts at `0x10a80` and is `0x10` bytes. **Residue-class finding:** armcc 5.04u5 at `-O2 -Otime` fully eliminates two chained, unused, non-volatile pointer loads (0/4 without intervention); marking the leaf field `volatile` retains both loads and reproduces the target exactly. Worth re-checking other modules' "untried return-1 lead" notes in case some are this same pattern rather than a bare 2-word stub. |
  | `0xdc8` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — same shared setter as `DllTrainingAppManager@0xd68`/`DllPssConfig@0x1c20`/`DllCaptureRegister@0x27b0`/`DllColosseum@0x46fc`/`DllFieldDemo@0x1edc`/`DllIntro@0x9eb0`. |
  | `0x1140` | `0x18` | 6/6 | Sets a single byte flag deep in a buffer (`this->buf[0x181d] = 1`) via two sequential pointer-cursor adds. Offset exceeds `strb`'s 12-bit immediate range; armcc's natural single-expression split (`add #0x1000; strb [r0,#0x81d]`) doesn't match retail's (`add #0x1800; add #29; strb [r0,#0]`) — only a literal sequential-cursor idiom (`p=buf; p+=0x1800; p+=29; *p=1;`) reproduces it. Same cursor-store pattern as `DllNumberInput@0x18fc`, now seen in a plain field-set rather than a constructor. |

  Inside the recurring tail-of-import-veneer-table leaf-stub cluster
  (bare `bx lr` stubs on both sides). Module has an unusually large
  import-veneer table; still far from exhausted (untried candidates
  remain around `0xeeb0`/`0xdb4`-area indexed-predicate shapes and other
  stub clusters).

- `matched/DllField/` — 16 functions, an **eighty-fourth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x58c8` | `0xc` | 3/3 | Trivial hardcoded-true byte-field setter: unconditionally sets `this+0x1c` to 1, ignores any 2nd argument, void return. |
  | `0x10f10` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. Right after a destructor-stage function (ending `0x10f08`), right before the sibling setter at `0x10f18`. |
  | `0x10f18` | `0xc` | 3/3 | Byte-field setter: stores the *third* argument (`r2`, not `r1`) into `this+141`, then unconditionally returns 1. Watch register roles on siblings — not every setter in this module takes its value in `r1`. |
  | `0x228b8` | `0x8` | 2/2 | Pointer-field setter, stores 2nd argument at `this+224`, void return. Mid-module, not near the import-veneer table. |
  | `0x288c0` | `0x8` | 2/2 | Pointer-field setter, stores 2nd argument at `this+0`, void return. First of a sibling pair with `0x288c8`. |
  | `0x288c8` | `0xc` | 3/3 | Sibling of `0x288c0`, same field — hardcoded-zero variant: discards the 2nd argument and stores a freshly-materialized 0 instead. A bare `bx lr` weak-evidence stub at `0x288d4` right after is unclaimed. |
  | `0xa6ff0` | `0x14` | 5/5 | Setter clearing `this+0xacd` and copying `this+0xac4` into `this+0xab8` (load-then-store order matters). Sibling of `0xa714c`. Found via `find_candidates.py`. |
  | `0xa714c` | `0x14` | 5/5 | Sibling of `0xa6ff0`: sets `this+0xacd` to 1, copies `this+0xab8` into `this+0xac4` (opposite direction/flag value). Found via `find_candidates.py`. |
  | `0x5445c` | `0x38` | 14/14 | Call+return helper: two import-veneer calls `(this, byte@10, 0, 2, 8)` then `(field0, 33)`, returns 1. `find_candidates.py`'s hint size (`0x40`) overran into two trailing one-word stubs past the real end — true size `0x38`. |
  | `0x54d84` | `0x38` | 14/14 | Sibling of `0x5445c`: calls `(field0, 196)` then `(this, byte@10, 1, 1, 12)`, returns 1. Residue: the two identical-value args (`r2=r3=1`) only matched written as the bare literal `1, 1` — a named local reused for both compiled with armcc materializing into `r2` first instead of `r3` (retail's order). |
  | `0x54dc4` | `0x18` | 6/6 | Call+return helper: veneer call with `(field0, 16)`, returns hardcoded 3. Found scanning right after `0x54d84`. Easy unclaimed neighbor at `0x54de4` (same shape, `(field0, 0, 8)`, also returns 3). |
  | `0x54e08` | `0x38` | 14/14 | Two-call helper: `CallA(this, this->f10, 0, 2, 16)` then `CallB(this->f0, 31)`, always returns 1. Same `find_candidates.py` size-rounding correction as `0x5445c` (hinted `0x40`, real `0x38` — the extra bytes are two separate unclaimed one-word stubs). |
  | `0x55384` | `0x38` | 14/14 | Sibling of `0x54e08`, same shape, different constants `(0,4,8` / `33)`. Same size correction (hinted `0x40`, real `0x38`). |
  | `0x34238` | `0x10` | 4/4 | Two fixed no-arg calls, void return (`FunA(); FunB();`, second tail-called via `b`). Real size `0x10` (4 words) — `find_candidates.py`'s hint (`0x34`) overran into the next function's field setters; the `push/pop {r4,lr}` is AAPCS stack-alignment padding, not a real save. |
  | `0x36d04` | `0x10` | 4/4 | Byte-identical shape to `0x34238`. Same size correction. |
  | `0x39a88` | `0x10` | 4/4 | Byte-identical shape to `0x34238`/`0x36d04`. Same size correction. |

  Right after DllField's large import-veneer table (`0x180`-`0x5767`)
  and a run of small veneer-calling wrappers/float helpers
  (`0x5740`-`0x58c4`); plenty of unmined ground remains in this module
  (9,277 functions — one of the largest CRO modules, now with two
  agents mining it in parallel off a shared `find_candidates.py` scan
  instead of each re-disassembling it — see the tool's own section
  below). An attempted
  candidate at `0xae58` (zeroes two adjacent int fields) got stuck at
  1/4 — see `attempted.md`; siblings at `0xe380`/`0xe3fc` have the same
  shape and residue, don't re-attempt without a new idea.

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

- `matched/DllTradelist/` — 5 functions, a **ninety-second** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xb40` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0xc04` | `0x38` | 14/14 | Destructor — same proven idiom as `DllFatalErrorProc@0x284` etc. |
  | `0xb5f0` | `0x8` | 2/2 | Another trivial `return 1` stub. |
  | `0x2dac` | `0xc` | 3/3 | Prologue-less tail-call forwarder with two literal args (`r1=22, r2=1`), passes original `r0` through, tail-branches to a local function — a fixed-args forwarder variant. |
  | `0xc70` | `0x4` | 1/1 | Single `bx lr` no-op void stub, weak evidence. |

  Attempted and logged to `attempted.md`: the neighboring `0xb6d0`/
  `0xb6e4` struct-copy pair (same residue class as `DllConfig@0x31b8`).

- `matched/DllZukan/` — 5 functions, a **ninety-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xf9c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0xfac` | `0x8` | 2/2 | Twin `return 1` stub, same leaf-stub cluster. |
  | `0x1be54` | `0x8` | 2/2 | Field getter, returns `long` at `this+0x28`. |
  | `0x1be5c` | `0x8` | 2/2 | Field getter, returns `long` at `this+0xc`. |
  | `0x1be64` | `0x8` | 2/2 | Field getter, returns `long` at `this+0x8` — 3rd of a sibling-getter run. |

  Abandoned (not re-attempted, no formal write-up — it's the already-
  documented vtable-pool-load-scheduling residue class): `0x1194`,
  vtable-install + two import calls, stuck at 10/11 (armcc hoists the
  VT literal load before the prologue and keeps `this` in `r0`; retail
  moves `this` to `r4` first, loads VT after, re-materializes `r0=r4`
  before each call). Unclaimed easy leads: two more `return 1` twins at
  `0xfa4`/`0xfbc` plus bare `bx lr` stubs in the same `0xf98`-`0xfd7`
  cluster; an array-indexed getter at `0x1be6c`
  (`add r0,r0,r1,lsl#2; ldr r0,[r0,#56]; bx lr`); a stub cluster
  starting at `0x1be78`.

- `matched/DllTitle/` — 4 functions, a **ninety-fourth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xd770` | `0x8` | 2/2 | Trivial int field getter (`this+44`), no calls. |
  | `0xd7a8` | `0x8` | 2/2 | Trivial int field getter (`this+12`), same shape. |
  | `0xd778` | `0xc` | 3/3 | Int field setter (`this+44`), returns 1 unconditionally — pairs with `0xd770`. |
  | `0xd534` | `0x18` | 6/6 | Field-copy helper: byte → `this+4`, signed halfword → `this+0`. A direct `(int)*(short*)(src+2)` cast fuses to `ldrsh`+`str` and does **not** match; the retail binary does separate `ldrh`+`sxth`+`str`, which only reproduces if the unsigned halfword load is its own statement (a temp variable) before the signed cast-and-store. |

  See `attempted.md` for two stuck candidates in the same dense
  getter/setter/predicate cluster (`0xd530`-`0xd8ac`): `0xd568` (a
  4-int equality predicate, 6/10 — pure register-allocation swap) and
  `0xd54c` (a 64-bit `ldrd` getter, stuck on an indirect-return-vs-
  single-`ldrd` toolchain residue).

- `matched/DllTownmap/` — 5 functions, a **ninety-fifth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x46c` | `0x1c` | 7/7 | Conditional accessor, same idiom as `DllLangSelect@0x5b0`/`DllDendouDemo@0x798`/`DllBoxSearch@0x5b8`/etc.: veneer call gates between a member load (`+0x10`) and a re-materialized constant `3`. |
  | `0x438` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x488` | `0x10` | 4/4 | Calls a local module function (`0x1f94`) with `this` unchanged, discards the result, returns 1; the `push {r4,lr}` is AAPCS alignment padding. |
  | `0x24dc` | `0x8` | 2/2 | Trivial int field setter: stores the 2nd arg into the object at `+0xec`, void return. |
  | `0x3240` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`, ignores `this` — mid-module, not near the import-veneer table. |

  Unclaimed leads: `0x3248` (`ldr r0,[r0,#4]; ldr r0,[r0]; bx lr`,
  double-indirection getter, size `0xc`), a bare `bx lr` weak-evidence
  stub at `0x1494`, and a double-field-store helper around `0x3628`
  (`mov r2,r1; stm r0,{r1,r2}; bx lr`). Several larger dispatcher/
  destructor-shaped functions (`0x498`, `0x4fc`, `0x5ac`, etc.) were
  seen but not attempted.

- `matched/DllTradeDemo/` — 4 functions, a **ninety-sixth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x550` | `0x1c` | 7/7 | Conditional accessor, same idiom as `DllLangSelect`/`DllDendouDemo`/`DllBoxSearch`/`DllDendouPC`/`DllEggDemo`/`DllPCMenu`: veneer call gates between a member load (`+20`) and a re-materialized constant `3`. |
  | `0x644` | `0x24` | 9/9 | Prologue-less leaf predicate: double-indirects through a `+24` pointer field to a leaf object's `+56` byte flag, conditionally stores 5 into `+16` and returns 2, else returns 4. |
  | `0x1a68` | `0x8` | 2/2 | `xy_system::bg::System::SetIsCallDraw(bool)` — same shared setter as several other modules. |
  | `0x1a70` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |

  Stopped at 4/5 — the module's remaining functions are all harder
  shapes: a dtor-style helper at `0x668` (conditional frees via
  local+import calls, not the standard vptr-install idiom), a larger
  dtor-like function at `0x740`/`0x97c`, 5-case jump-table dispatchers
  at `0x1aa4` (the known unresolved `ldrcc pc,[pc,r1,lsl#2]` residue
  class), and a two-64-bit-globals init leaf at `0x1a78` — the exact
  same dead-end residue already logged for `DllFieldDebug@0x180`
  (value-computed-first/`strd`-pair ordering), recognized on sight and
  skipped rather than re-attempted.

- `matched/DllTrainerCase/` — 10 functions, a **ninety-seventh** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x37e4` | `0x38` | 14/14 | Destructor — same proven idiom as `DllFatalErrorProc@0x284` etc. Sibling at `0x381c` has the known-bad member-load-first ordering and was skipped. |
  | `0x37a4` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`. |
  | `0x37ac` | `0x8` | 2/2 | Identical twin stub, `return 1`. |
  | `0x3850` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0x3cc0` | `0x8` | 2/2 | Sub-object accessor, `add r0,r0,#16; bx lr` — returns `this+16` as a pointer, pure arithmetic, no load. |
  | `0x37b4` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1` (twin of `0x37a4`/`0x37ac`). |
  | `0x37bc` | `0x8` | 2/2 | Same `return 1` stub. |
  | `0x37cc` | `0x8` | 2/2 | Same `return 1` stub — last of this twin-stub run. |
  | `0x3cc8` | `0x8` | 2/2 | Sibling sub-object accessor, `add r0,r0,#4; bx lr` — returns `this+4`. |
  | `0x3cd0` | `0x8` | 2/2 | Sibling sub-object accessor, `add r0,r0,#28; bx lr` — returns `this+28`. |

  Leftover easy leads: a third accessor sibling at `0x3cd8` (`+8`), an
  untried thin tail-call forwarder at `0x3cb8`, and a bare `bx lr`
  stub at `0x3cbc`.

- `matched/DllUSGymEsper/` — 5 functions, a **ninety-eighth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xad4` | `0x18` | 6/6 | Array-indexed virtual dispatch: looks up an owned object in an inline pointer array at `this+0x24`, tail-calls virtual slot 7 on it (new `this`, 3rd arg passed through). Identical idiom to `DllUSBigFourSteel@0x2f4`. |
  | `0x328` | `0x14` | 5/5 | Boolean getter through one pointer indirection: `return this->p20->b == 1;`. Same idiom family as `DllStrInput@0x6674`, but compares to literal 1 (armcc reuses the already-loaded byte as the true-case result). |
  | `0x2fc` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0x480` | `0x18` | 6/6 | Boolean setter through the same `this+20` indirection (sibling of `0x328`). Source-phrasing gotcha: `v ? 1 : 0` compiled to 5 words (a register-reuse shortcut); an explicit `int flag = 0; if (v) flag = 1;` local reproduced the target's unconditional-zero/conditional-one 6-word shape exactly. |
  | `0x304` | `0x8` | 2/2 | Trivial `return 0` stub, byte-identical twin of `0x2fc`. |

  Leftover easy leads: four lone `bx lr` weak-evidence stubs
  (`0x2f8`/`0x30c`/`0x320`/`0x324`), a conditional tail-branch function
  at `0x310`, and the module's first large dispatcher at `0x33c`.

- `matched/DllTrialHouseResult/` — 5 functions, a **ninety-ninth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x4c8` | `0x1c` | 7/7 | Guarded field accessor: calls an import veneer on `this`, returns `*(this+28)` if non-zero else the constant 2. |
  | `0x2644` | `0x4` | 1/1 | Trivial no-op void stub (lone `bx lr`), weak evidence. |
  | `0x26b4` | `0x8` | 2/2 | Trivial `return 0` stub. |
  | `0x2648` | `0x38` | 14/14 | Destructor: installs vtable ptr, frees `+0x14` via masked veneer call if non-null, zeroes it, tail-calls another veneer with `this` (armcc folds `return Fn(this);` into `pop{r4,lr}; b <veneer>`). Needed the vtable literal and member value pulled into two separate named locals up front to get the exact literal-load/member-load/store/compare order — two plain sequential statements made armcc reuse a register and serialize them wrong (capped at 11/14 until fixed). |
  | `0x2680` | `0x34` | 13/13 | Sibling destructor, returns `this` directly instead of tail-calling — same shape as the already-matched `DllDialogCommon@0xcdc`. |

  Only the first ~`0x700` bytes and the tail ~`0x200` bytes of a
  `0x2618`-byte `.text` section were scanned; plenty of unmined
  territory remains for a follow-up agent.

- `matched/DllUSGymFighting/` — 4 functions, a **hundredth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x278` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0x3f4` | `0x18` | 6/6 | Conditional accessor: if `this->f16==3`, tail-calls a veneer on `this->f4` and returns its result, else returns `this` unchanged. Real size is `0x18` not the naive `0x14` — the trailing `bx lr` is part of the body. |
  | `0x11c0` | `0x1c` | 7/7 | Struct zero-initializer (fields at `+0`/`+8`/`+4`, then byte flags `+12`/`+13`). Same trailing-`bx lr` size gotcha (`0x1c` not `0x18`). |
  | `0xdb4` | `0x28` | 10/10 | Array-indexed virtual-call predicate: looks up `this->arr[idx]` (array at `+0x18`), calls vtbl slot 55 (offset `0xdc`) with arg 0, returns bool of the nonzero result. |

  See `attempted.md` for two stuck candidates in the same module:
  `0x11dc` (a globals-init leaf, same `strd`-vs-`stm` register-role
  residue as `DllFieldDebug@0x180`, 3/11) and `0xd64` (a short-circuit
  AND of two virtual-call predicates sibling to `0xdb4`, stuck at
  9/20 on an extra register-preservation `mov` the retail binary does
  before its prologue).

- `matched/DllUSKinomi/` — 5 functions, a **hundred-and-first** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0xf40` | `0x4` | 1/1 | Thin tail-call forwarder (single `b <import veneer>`), same idiom as `DllBattleVideoSave@0x454`/`DllBattleMatch@0xd90`/`DllBattleVideoPlayer@0xb14`/`DllIntro@0x868`. |
  | `0xf44` | `0x4` | 1/1 | Lone `bx lr` no-op stub, weak evidence. |
  | `0xf4c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0xf54` | `0x8` | 2/2 | Same `return 0` stub, identical bytes. |
  | `0xf5c` | `0x8` | 2/2 | Same `return 0` stub, identical bytes. |

  All five sit in one small stub cluster right after the module's
  first (large) real function, between file offsets `0xf40`-`0xf67`
  (`0xf64`, one more bare `bx lr`, is untried).

- `matched/DllUSPassage/` — 5 functions, a **hundred-and-second**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x334` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0x750` | `0x8` | 2/2 | Trivial field getter, `return *(int*)(this+0xb4)`. |
  | `0x1620` | `0x8` | 2/2 | Trivial field getter, `return *(signed char*)(this+0x5c)`. |
  | `0x1078` | `0x4` | 1/1 | Thin tail-call forwarder (single `b <veneer>`), same idiom as `DllBattleVideoSave@0x454` etc. |
  | `0x1610` | `0x4` | 1/1 | Same tail-call-forwarder idiom — this module has 4 total copies of this shape (`0x1078`/`0x1610` claimed, `0x1c80`/`0x20c4` untried). |

  Module's first real function (`0x350`) and its sibling cleanup loop
  (`0x64c`) are large/complex, not easy wins. An untried candidate at
  `0x20cc` (a 9-word global-reset function, two globals via
  literal-pool pointers, `strd`+`stm` writes) risks the same
  store-ordering residue class documented for `DllFieldDebug@0x180` —
  flagged but not attempted, no formal write-up (no real effort spent).

- `matched/DllVSDemo/` — 5 functions, a **hundred-and-third** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x3e0` | `0x8` | 2/2 | Trivial `return 1` virtual-override stub, first real function after the import-veneer table. |
  | `0x1a60` | `0x8` | 2/2 | Trivial `return 1` stub, after a destructor-stage vtbl-install helper. |
  | `0x1b00` | `0x8` | 2/2 | Trivial `return 1` stub, sandwiched between two conditional-free destructor helpers. |
  | `0x2b28` | `0x8` | 2/2 | Trivial `return 0` stub, after another destructor-stage helper. |
  | `0x408` | `0x4` | 1/1 | Bare `bx lr` no-op void stub, weak evidence. |

  See `attempted.md` for `0x2984`: the same prologue-less tail-virtual-
  call idiom as `DllDialogCommon@0x6e8`, stuck at 5/8 on a register
  choice (retail picks `r1`, armcc always picks `r3`) — flagged as a
  shallower single-hop variant of that same residue class.

- `matched/DllUSGymIce/` — 3 functions, a **hundred-and-fourth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x25c` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`, ignores `this`. Twin at `0x264` left unclaimed. |
  | `0x278` | `0x10` | 4/4 | Two-level-indirection sign-extended byte getter (`this->inner->field` narrowed to `signed char`). **Gotcha**: a plain (non-`volatile`) `int` field at that offset makes armcc 5.04u5 `-O2` optimize the load straight down to `ldrb` once it sees only the low byte survives the cast — doesn't match. Marking the field `volatile` blocks that optimization and reproduces the target's `ldr`+`sxtb` pair exactly. |
  | `0x11f4` | `0x4` | 1/1 | Trivial no-op void stub (lone `bx lr`), weak evidence. |

  A second lone `bx lr` at `0x146c` initially looked like another
  standalone stub and technically word-matched, but is actually the
  tail instruction of the function starting at `0x1430` (a coherent
  VFP wraparound leaf with no push/pop) — correctly discarded before
  committing. See `attempted.md` for `0x1974` (a globals-init pair,
  9/11 — closer than usual on the same `strd`-vs-`stm` residue class).

- `matched/DllWazaOmoidashi/` — 5 functions, a **hundred-and-fifth**
  module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x924` | `0x14` | 5/5 | Field setter: stores arg2 (halfword) into a sub-object at `this+0x3c` offset `+14`, then sets a byte flag at `this+0x49` to 1. Needed the constant `1` bound to its own local variable (declared right after the sub-pointer load, before either store) to get armcc's "load both values before either store" scheduling — otherwise it interleaves a load with its own store. |
  | `0x5f0` | `0x4` | 1/1 | Trivial no-op void stub (single `bx lr`), weak evidence, right after the import-veneer table. |
  | `0x18b4` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. |
  | `0x23d4` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. Unclaimed neighbors: a bare `bx lr` at `0x23dc` and a twin `return 1` stub at `0x23e0`. |
  | `0x3514` | `0x8` | 2/2 | Trivial virtual-override stub, `return 1`, ignores `this`. Four unclaimed bare `bx lr` stubs follow at `0x351c`/`0x3520`/`0x3524`/`0x3528`. |

  Module still has plenty unmined: a notification-dispatcher pair at
  `0x938`/`0x9e0` (same shape as the already-matched
  `DllNumberInput@0x1890`), and several larger dispatchers with
  jump-table-shaped switches (`0xb30` area).

- `matched/DllBattle/` — 5 functions, a **hundred-and-sixth** module:

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x1fac` | `0x10` | 4/4 | Boolean predicate `return x == 0;` on the 2nd arg, ignoring `this`. Straightforward `cmp`/`moveq`/`movne` compile, no scheduling trap. |
  | `0x1fbc` | `0x8` | 2/2 | Trivial virtual-override stub, `return 0`. |
  | `0x1fc4` | `0x8` | 2/2 | Pointer-offset accessor: `return (char*)this + 2912;`. |
  | `0x1fd8` | `0xc` | 3/3 | Field-getter-plus-offset: loads a pointer member at `this+2960`, returns it `+48`. |
  | `0x1f8c` | `0x20` | 8/8 | Zero-init helper: zeroes two `u32` fields then four `u16` fields of a 16-byte struct, returns `this` unchanged. Non-obvious: source **statement order** (not declared struct layout) drives armcc's instruction scheduling here — the two int stores had to be assigned before the four short-field stores to match, even though both groups sit at consistent byte offsets either way. |

  All five sit clustered together (`0x1f8c`-`0x1fe3`) right after
  `DllBattle`'s large import-veneer table (895 entries,
  `0x180`-`0x1d8c`) — this module uses an unusual `ldr pc, [pc, #-4]`
  absolute-literal veneer idiom rather than the `b <target>` forwarder
  seen in most other modules, worth noting for future agents mining
  it. Untried leads: a twin `return 0` stub at `0x1fd0`, a 1-word
  tail-call forwarder at `0x1fcc`, and a large VFP vector/matrix
  function at `0x1d8c`-`0x1f74`.

**2026-10-15: first deepening round — targeting single-match modules by
size.** With almost every CRO now touched at least once, the orchestrator
started picking the next round's targets differently: instead of an
unmined module, it ranked modules that already have exactly one matched
function by their total Ghidra function count (from the heatmap's
`heatmap_data.json`) and spawned agents on the three largest —
`DllField` (9,277 functions), `DllBattleMatch` (1,572), `DllSequence`
(1,247) — on the theory that a module with thousands of functions and
only one proven match almost certainly still has easy siblings sitting
unmined, same as the "scan near your last match" advice already in the
executor skill, just applied at orchestrator-assignment time instead of
within a single agent's run. All three paid off (each went from 1 to 6
matched functions in one round) — see their updated entries above. This
is now a reasonable default for future rounds once genuinely-unmined
modules run out: `for d in matched/*/; do n=$(find "$d" -name '*.c' -o
-name '*.cpp' | wc -l); [ "$n" = 1 ] && echo "$(basename "$d")"; done`
cross-referenced against `heatmap_data.json`'s per-module `n` field.

### Tool addition: `find_candidates.py`, a shared candidate scan (2026-10-15)

New deterministic, no-LLM tool at `tools/find_candidates.py <Module>`:
disassembles nothing itself, instead regex-matching the already-generated
Ghidra decompiled-C corpus (`decomp/cro_c/<Module>/`) against the small
set of shapes this file's "What reliably works" list documents (trivial
stubs, simple setters, short call+return helpers), excluding anything
already in `matched/`/`attempted.md`. Built so that when a module has
room for more than one agent at once (a big one like `DllField` has
thousands of untouched functions), they can split one shared scan's
output instead of each re-disassembling and re-scanning the same module
from scratch — exactly what happened in the round right below: two
agents split `DllField`'s candidate list with zero overlap and zero
duplicated disassembly cost.

**Explicitly a hint source, not proof** — Ghidra's decompiled C can be
misleading for anything non-trivial, so every candidate still needs its
own `fingerprint_fn.py` masked match before it counts (stated in the
tool's own docstring and printed with every run).

**Known rough edge, found by both agents in the very first real use**:
reported sizes ("gap to the next Ghidra-tracked function start") are
consistently a bit too large — in every case this round, the real
function ends cleanly a few words early (`pop {...,pc}` or a tail `b`),
with the extra hinted bytes belonging to one or more separate, untracked
trailing stub(s) that Ghidra didn't carve its own file for (the same root
cause as the size-sanity bug fixed earlier that day, just inside the
plausibility bound that time instead of wildly over it). Agents correctly
treated the hint as a starting offset only and found the real end
themselves — this doesn't block using the tool, but a future improvement
would have the script itself locate the first return instruction after
each candidate's start (one extra disassembly pass, still done once
centrally) instead of trusting the next tracked function's start.

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

## Match style: natural vs. trick

A masked match proves the *bytes*; it doesn't say whether the C is how the
original authors wrote it. Following `pokebw2`'s "natural C only" rule, each
matched file may carry a `Style:` line in its header comment:

- **no tag (or `Style: natural`)** — plain C a human would write.
- **`Style: trick(<kind>) -- <why>`** — compiles to the retail bytes, but the C
  contains a construct that exists only to appease armcc. Fine as a
  placeholder; rewrite (or re-derive the real source construct) before this
  becomes readable decomp or feeds the native port.

Kinds (find them all with
`grep -rl 'Style: trick' xy-decomp/armcc-matching/matched`):

| kind | what it is | files tagged |
|---|---|---|
| `volatile-strd` | `volatile` on a 64-bit global only to get `strd` | 10 (globals-init family, incl. DllSalon@0xdd0) |
| `volatile-load` | `volatile` field only to keep armcc from narrowing/dropping a load | DllUSGymIce@0x278, DllSequence@0x10a80 |
| `redundant-local` | named literal/temp local only to steer armcc's constant-register choice | DllKisekae@0xb6f0, DllBattle@0x39c0, DllGts@0x8850, DllUSGymEsper@0x480, DllPoke3Select@0x63c, DllWazaOmoidashi@0x924, DllField@0x10c34, 0x10c54, DllBattle@0x2b9dc, 0x75050, 0x74ff8, 0x750a4 |
| `split-expression` | one expression split across statements only to stop armcc fusing a load (`ldrsh`) | DllTitle@0xd534 |
| `extra-arg` | unused pass-through int arg(s) to shift register allocation (1 arg -> vtable in r2, 2 -> r3) | DllTownmap@0xb10, 0x3554, 0x35f4, 0xd68, DllBattle@0x7a780 |
| `static-helper` | one-use static helper to pin an argument register / reproduce r0 reuse | DllEshop@0x1b10, DllHologramMail@0xe970 |
| `void-ptr-passthrough` | callee declared `void*`, result offset `(char*)fn(p) - N`, to avoid a spare `mov r4,r0` | DllPuzzle@0x1e90, 0x4ba0, DllSalon@0x790, DllHeading@0x868, 0x16f4, 0x8ac, 0x1738 |
| `pointer-walk` | sequential pointer-cursor moves only to reproduce retail's immediate splits / pre-indexed stores | DllSequence@0x1140, DllNumberInput@0x18fc |
| `live-r1-passthrough` | return type widened (`long long` built from `this` and the incoming 2nd arg) only to keep r1 live so a shared zero lands in r2 — least natural form; true signature unknown | DllField@0xae58, 0xe3fc, 0xe380 |
| `volatile-store` | the vptr store goes through a `volatile` pointer purely to pin a register copy (`mov r4,r0`) ahead of the store — used with the C++ inline-member `Reset()` form (needs a `.cpp` file; the verifier switches to C++ mode from the suffix) | DllField@0xb1bb8 and its 23 byte-identical sites |
| `wider-callee-return` | callee declared `long long` and the caller returns `(int)` of it, only to stop armcc tail-calling (r0 is the low word, the cast costs no code) | DllField@0xe1c4c, 0xe1fe8 |
| `cpp-inline-member` | pointee-side logic written as an inline C++ member called on the loaded pointer (needs a `.cpp` file) | DllField@0x2948c |
| `ordering-local` | locals (`vt = VT`, hoisted member loads) only to fix load / pool-load order | the `vt`-local destructor family (15: DllBattleHouseConnect@0x2fc, DllCommendation@0xc94, DllEshop@0x554, DllFatalErrorProc@0x284, DllFriendEntry@0x4c4, DllGameClearSave@0xd6c, DllGameSync@0x5e0, DllLiveCup@0x764, DllRandomTrade@0x8b4, DllStaffRoll@0x2c44, DllSubScreenTutorial@0x12cc, DllTrainerCase@0x37e4, DllTrialDownload@0x728, DllTrialHouseResult@0x2648, DllTradelist@0xc04) + DllGts@0x5be0, 0x53e4, 0x5ba8, 0x5bc4, 0x5ad4, DllSalon@0xd5c, DllBag@0x13848, DllPuzzle@0xbf80, DllTownmap@0x31d4, DllPhotoViewer@0x193c, DllStatus@0xa304, DllKisekae@0xb674, 0xb6ac |

**Audit (2026-10-15):** all 377 matched files were screened — a script flagged
`volatile` / static functions / armcc extensions / literal-assigned locals /
`(void)` casts / 3+ args, a comment scan flagged "non-obvious / phrasing /
named local / helper / swap / fold ..." language, and every flagged body was
read. Result: **46 files tagged**; the remaining 331 are natural C (plain
setters/getters/stubs, `(void)this;` unused-parameter silencing, plain
dtors with a member-load local, `if (r) return 0; return 1;` predicates,
literal call arguments). The audit is heuristic: a trick that leaves no
textual trace (e.g. a field type or struct layout chosen only to steer
codegen) would not be caught, so `natural` is "no trick found", not a proof.
Judgement call: the `vt`-local destructor family is tagged `ordering-local`
because the extra local exists only to order the pool load; the plain
`void *p = o->buf; o->vptr = VT; if (p) ...` form is natural and untagged.
Tiny stubs (`bx lr`, return-const, one-field setters) are natural but weak
evidence — any function of that shape compiles identically. New matches must
set the tag (see `pending/README.md`).

**Relaxed matches are not matches.** A function that only passes with
`--relax-nops` (nop words dropped from both sides, pool offsets masked) is
filed under `relaxed/<Module>/<offset>.c` and excluded from the totals; it is
the analogue of a row in `pokebw2`'s `docs/nonmatching-functions.md`. Current
list: `relaxed/DllBattle/0x30a4.c` (26/26 relaxed, destructor); DllTownmap@0xaa4 and 0xa34 (same shape; source not saved, see `attempted.md`); DllStatus@0x2fd8 and 0x2e90 (search loop, nop between `cmp` and `beq`; source in `attempted.md`).

## 2026-10-15 swarm batch (9 functions, independently re-verified)

All re-run with `fingerprint_fn.py` -> `*** MASKED MATCH ***`.

| Module | Offset | Size | Match | Note |
|---|---|---|---|---|
| `DllKisekae` | `0xb6e8` | `0x8` | 2/2 | virtual-override stub, `return 1` |
| `DllKisekae` | `0xb6f0` | `0xc` | 3/3 | byte-field setter |
| `DllPssCharaMenu` | `0x5e8` | `0x8` | stub | trivial stub |
| `DllPssMainMenu` | `0x430` | `0x8` | stub | trivial stub |
| `DllSalon` | `0x380` | `0x20` | 8/8 | predicate: returns 1 if in-module helper `0xaa0` (on `this->f48+4`) yields 0 |
| `DllSparring` | `0x10180` | `0x8` | stub | trivial stub |
| `DllSparring` | `0x10188` | `0x8` | stub | trivial stub |
| `DllTestPGL` | `0x180` | `0x8` | 2/2 | `SetIsCallDraw` byte-field setter (new module) |
| `DllUSPokecen` | `0x254` | `0x8` | stub | trivial stub |

## 2026-10-15 swarm batch 2

- `matched/DllTestSuzuki/` — 1 function, a new module (re-verified `*** MASKED MATCH ***`):

  | Offset | Size | Match | Note |
  |---|---|---|---|
  | `0x180` | `0x2c` | 11/11 | Globals-init: stores -1 into a 64-bit global and 0 into another. **New idiom:** the first global must be `volatile long long` to get retail `strd r0,[r2]`; plain / `__align(8)` / `unsigned long long` compile to `mvn/mov/stm`. Second global is plain `long long` (`stm {r1,r2}`). |

  The same init shape (`strd` of -1, `stm` of zeros) is the residue logged
  for DllUSGymFighting@0x11dc, DllUSGymIce@0x1974 and DllTest{Ichiraku,Kawada,
  Nagihashi}@0x180 (Kosaka differs by one word) — try the `volatile` trick there.
  The rest of DllTestSuzuki (0x1ac, `cmp r1,#5` jump table) is untried.

## 2026-10-15 swarm batch 3

All re-run: `*** MASKED MATCH ***`.

- **Globals-init family (`volatile long long` idiom, 0x2c bytes, 11/11):**
  `DllTestIchiraku@0x180`, `DllTestKawada@0x180`, `DllTestNagihashi@0x180`,
  `DllTestKosaka@0x180` (written `extern volatile long long g_a[2]; g_a[1] = -1;`,
  `strd` to `[r2,#8]`), `DllFieldDebug@0x180`, `DllUSGymFighting@0x11dc`,
  `DllUSGymIce@0x1974`. Same shape as DllTestSuzuki above; other CRO modules
  with a `strd r0,[r2]` globals-init may be worth a scan.
- **`DllZukan`, 5 trivial functions:** `0x1a1a4` (2/2, long getter this+0x20),
  `0x1a3f0` (2/2, byte setter this+0x71), `0x1bad8` (5/5, bool getter
  byte@+0xc == 3), `0x1be6c` (3/3, indexed long getter arr[i] at +0x38),
  `0x1d698` (4/4, clears u32@0 + u16@4). Easy siblings left: `0x155b4`/`0x156c4`/
  `0x1576c` (`strb` setters via `[r0,#4]`), `0x1baec`, `0x1bad0`,
  `0x1dcc0`-`0x1dce8` (`add r0,r0,#N` getters), `0x1be18`/`0x1be20`/`0x1be28`/`0x1bdb8`.

## 2026-10-15 swarm batch 4

- `DllPokeList@0x8e4`, `0x8ec`, `0x8f8` (each `0x8`, 2/2, re-verified): return-true virtual-override stubs (`mov r0,#1; bx lr`), byte-identical to the already-matched `0x8dc`. Remaining easy candidates there are only bare `bx lr` stubs (`0x8f4`, `0x900`-`0x90c`, weak evidence); first real function is a constructor-shaped one at `0x910`.

## 2026-10-15 swarm batch 5 (24 functions, all re-verified)

Mostly tiny leaf stubs/accessors, so individually weak evidence (any function of the same shape compiles to the same words) — bare `bx lr` ones are flagged.

| Module | Offsets (size, match) | What |
|---|---|---|
| `DllZukan` | `0x155b4`/`0x156c4`/`0x1576c` (0xc, 3/3) | setters `this->p(+4)->byte[0xf2/0xe8/0xf3]=v` |
| | `0x1baec` (0x8, 2/2) | word getter at +0x74 |
| | `0x1dcc0` (0x8, 2/2) | returns `&this->member` at +0x10 (`add r0,r0,#16`); siblings `0x1dcc8/d0/d8/e0/e8` open |
| `DllGts` | `0x4108` (0xc), `0x43dc` (0xc) | `strb 1` setters at +0x44 / +0xe0 (3/3) |
| | `0x43f4`, `0x4e9c` (0x10, 4/4) | clear +0x5c return 0x34; store 8 at +0x1c return 6 |
| | `0x3eb4` (0x8, 2/2) | word setter +0x50 |
| `DllBag` | `0xc88`, `0xc90`, `0xc9c`, `0x138bc`, `0x138b4` (0x8, 2/2) | return-1 x3, return-0, byte setter `this[217]` |
| `DllBox` | `0xa0c`, `0xa10` (0x4, 1/1) | bare `bx lr` no-ops (weak) |
| | `0xc80` (0x1c, 7/7) | conditional accessor via veneer: `+68` if non-zero else 2 (same idiom as DllEggDemo@0x568) |
| | `0x1dd58` (0x8, 2/2) | `return 1` |
| `DllStatus` | `0xbd0`, `0xbd8`, `0xbe4`, `0xbf8` (0x8, 2/2) | return-1 virtual stubs |
| | `0xbe0` (0x4, 1/1) | bare `bx lr` (weak) |

Open leads: DllGts `0x4174`, `0x43e8`, `0x453c`, `0x5320`, `0x7bd8`, `0x9874`, `0x119a8`, `0x125cc`; DllZukan `0x1bad0`, `0x1be18/20/28`, `0x1bdb8`; DllStatus `0x4590` (strb setter), void stubs `0xbec/f0/f4`; DllBag `0x13664`. DllGts `0x2a08` (zeroes +8/+0xc) failed: armcc zeroes from `r1`, retail from `r2`.

## 2026-10-15 swarm batch 6

- `DllGts@0x4174` (0xc, 3/3, `strb 1` setter +0x44), `0x43e8` (0xc, 3/3, `strb 1` setter +0xe0), `0x453c` (0x10, 4/4, clear +0x5c return 0x28), `0x5320` (0x10, 4/4, store 0x1d at +0x1c return 6) — all re-verified; weak-evidence stubs. Still open: `0x7bd8`, `0x9874`, `0x119a8`, `0x125cc`.

## 2026-10-15 swarm batch 7

- `DllBattle@0x2228` (0x24, 9/9): virtual call on slot 3 of the object at `this+0x24`, returns `result==0` — must be written `if (r) return 0; return 1;` (`!x` emits `movne/moveq` in the wrong order). `0x227c` (0x10, 4/4): tail-calls virtual slot 5 of the object at `this+0x24`. `0x2420` (0xc, 3/3): zeroes word at `this+8` (weak evidence). All re-verified.

## 2026-10-15 swarm batch 8

- `DllZukan@0x1dcc8` (+4), `0x1dcd0` (+0x1c), `0x1dce8` (+0xc): `add r0,r0,#N` getters, 0x8, 2/2 each, re-verified (weak evidence). Open siblings `0x1dcd8` (+8), `0x1dce0` (+4); return-const stubs `0x1be18`/`0x1be20`/`0x1be28`/`0x1bdb8`.

## 2026-10-15 swarm batch 9 (17 functions, all re-verified)

| Module | Offset (size, match) | What |
|---|---|---|
| `DllBattle` | `0x2c3c` (0x24, 9/9) | virtual call slot 3 via `this+0x24`, returns `result==0` (same if/return idiom as `0x2228`) |
| | `0x2c90` (0x10, 4/4) | tail-calls virtual slot 5 of `this+0x24` |
| | `0x2c60` (0x30, 12/12) | if `this+0x24` set: vtbl slot 1 (delete), free `this+0x20` via import, return 1 |
| | `0x2ef0` (0x44, 17/17) | if `this+0xc` set: vtbl slot 4, then slot 1 if still set, zero the field |
| | `0x2d88` (0x18, 6/6) | `p = import(); *p = vtable ptr` (pool word is a relocation) |
| `DllField` | `0x1a448` (0x14, 5/5) | stores -1 to three u16 fields +0/+2/+4 |
| | `0x30734` (0x10, 4/4) | stores -1 to two ints +0/+4 |
| | `0x1bd48` (0x14, 5/5) | `+0x18 = arg2`, `+0x20 = arg2->[+0x50]->[+0x30]` |
| | `0x297d4` (0x10, 4/4) | zero byte@0 + word@4 |
| | `0x3296c` (0x1c, 7/7) | zero-inits five fields; the `strb` at +0x14 must precede the `str` at +0x10 in source |
| `DllGts` | `0x7bd8`, `0x9874` (0xc, 3/3) | `strb 1` setter +0x3c |
| | `0x119a8` (0xc, 3/3) | `strb 1` setter +0x40 |
| | `0x125cc` (0x8, 2/2) | returns 0x1b |
| | `0x9898` (0x1c, 7/7) | if `this->obj@0x20`, tail-calls its vtable slot 9 |
| `DllStatus` | `0x4590` (0xc, 3/3) | byte setter `((u8*)arg3)[12]=1` |
| | `0x4654` (0x30, 12/12) | loads u32 at arg3+0x1dc; calls one import with `(this[0x58], v, 0)`, tail-calls another with `(this[0x58], v, 1)` |

Open: DllGts `0x119b4` (0x44) and `0x125d4`+ untried; DllField `0xe380`/`0xe3fc` (zero-init using r2 where armcc picks r1, 1/4) unlogged.

## 2026-10-15 swarm batch 10

- `DllPokeList@0x8f4`, `0x900`, `0x904`, `0x908`, `0x90c` (each 0x4, 1/1, re-verified): bare `bx lr` void no-ops — very weak evidence. `0x910` (~0x134, constructor with inlined sub-object ctors) judged too big, untried.

## 2026-10-15 swarm batch 11

- `DllTownmap@0xb10` (0x1c, 7/7): if `this->[0x88]` non-null, tail-calls that object's vtable slot 4 with arg1 passed through. `0x3554` (0x1c, 7/7): same shape, member at +0x10. **Idiom:** a one-arg `void(this)` signature loads the vtable into r1 and mismatches (retail uses r2); the C needs a pass-through `int a` argument, which keeps r1 occupied.
- `0x3248` (0xc, 3/3): double-indirection getter `return *this[1]`. `0x1530` (0x8, 2/2): returns 1. `0x1538` (0x1c, 7/7): void; if `this->[0x24]` and its `[0x14]` are non-null, tail-calls a function with `[0x14]`.
- Open: `0x1324` (bounds check then `base + idx*8`), `0x3610` (`strd` init function).

## 2026-10-15 swarm batch 12 (20 masked matches, all re-verified)

| Module | Offset (size, match) | What |
|---|---|---|
| `DllBattle` | `0x39c0` (0x28, 10/10) | `if (fa(this)) this->+4->+0x14 = 1; return 1` — needs `B *b = self->b; int one = 1; b->f = one;` (direct `b->f = 1` swaps r0/r1, 7/10) |
| | `0x39ac` (0x14, 5/5) | tail-call wrapper `fa(2,0,5,0)` |
| | `0x39f4` (0xc, 3/3) | setter `this->+8 = 0` |
| `DllPuzzle` | `0xf04` (0x2c, 11/11) | `mode==1 && fn(obj)` -> 1 else 0 |
| | `0x1438` (0x38, 13/13) | dtor stage: vtable, base cleanup, `fn(this->p4->f14)` with 17, returns this |
| | `0x1e90`, `0x4ba0` (0x14, 5/5) | `*o=v; return fn(o+N)-N` (N=0x18 / 4). **Idiom:** `return (char*)fn(p) - N;` with `fn` returning `void*`; keeping `o` in a local adds a spurious `mov r4,r0`. Real size 0x14 (trailing word is padding) |
| | `0x9310` (0x2c, 11/11) | dtor stage: vtable, `fn(o->p12, o->p4->f34)`, returns this |
| `DllBag` | `0x10c8` (0x1c, 7/7) | `veneer(this) ? this->f3c : 2` |
| | `0xc558` (0x10, 4/4) | veneer on `this`, returns 42 |
| | `0xe484` (0x24, 9/9) | `fnA(this); this->sub.f30 = fnB(this); return 0` — model `sub` as struct at +8 |
| | `0xfee4` (0x24, 9/9) | `fnA(this->c->p); fnB(this); return 40` |
| | `0x10334` (0x2c, 11/11) | `fnA(this); fnB(ctx->d->f8,2,3,1); return 71` (`this->ctx` +0x4c load hoisted above first call) |
| | `0x1366c` (0x14, 5/5) | calls fn on `this->f24`, returns 1 |

Open: DllPuzzle `0xb3fc`/`0xbf80` (0x38, free `this+0x14` + tail-call base), `0x92e4`/`0x1404` (tail-call dtor-stage variants).

### Relaxed (not masked) match — kept out of the totals

`relaxed/DllBattle/0x30a4.c` (0x68, 26/26 **only with `--relax-nops`**): destructor — installs vtable, releases owned object at +0xc via vcall slot 4 then slot 1 and zeroes it, frees +0x38, calls base sub-object dtor at +0x18, returns result-0x18. Pool load comes before the member load (the shape that works). Byte-match pending nop re-insertion; promote to `matched/` if relaxed matches are accepted.

## 2026-10-15 swarm batch 13

- `DllTownmap@0x35f4` (0x1c, 7/7): virtual-call wrapper — if member at +0x10 is non-null, tail-calls vtable slot 3, else returns. Needs **two** pass-through int args (`int a, int b`) so the vtable load lands in r3 (one arg -> r2).
- `DllTownmap@0x3610` (0x2c, 11/11): globals-init, `volatile long long g_a = -1` (`strd`) + plain `long long g_b = 0` (`stm`). Verify at size **0x2c** (includes the two pool words); at 0x24 the candidate is longer than the target and fails.

## 2026-10-15 swarm batch 14

- `DllPuzzle@0x1404` (0x30, masked): dtor-stage variant — stores the vtable passed as 2nd arg (`void *vt`), `fn71b0(o)`, `fn260(fn1c8(o->p4->f14), 17)`, then tail-calls `fn268(o)` (returning `void*`). `0x92e4` (0x28, masked): same variant as `0x9310` — stores `vt`, `fn8d0(o->p12, o->p4->f34)`, tail-calls `fn268(o)` (size 0x28: pool word at 0x930c belongs to the next function).

## 2026-10-15 swarm batch 15

- `DllEshop@0x1b10` (0x2c, 11/11): calls an import with `(2,0,8,0)`, sets byte `this+0x58 = 1`, returns 22. **Idiom:** a `static void set(X*, int v)` helper reproduces retail's reuse of r0 for the constant (inline store emits `mov r1,#1`). `0x25c4` (0x10, 4/4): `tail188(inner(o))` with `inner` returning `void*` so r0 passes straight through (no `mov r4,r0`). DllEshop is mostly large state-machine functions; small candidates are exhausted.

## 2026-10-15 swarm batch 16

- `DllBag@0x7330` (0x50, 20/20): state handler — reads `q->b` at +1, tests `b == 0`; if zero stores `fnA(this)` at +0x38 and returns 0, else sets halfword +0x42 of the sub-object to 1 and returns 75; always calls `fnB(this)` first. The `b == 0` test form matters (`b != 0` / swapped branches: 7/20, 11/20). `0xc508` (0x50, 20/20): same, reads `q->b` at +0x12, tests `b == 1`.
- `0xd03c` (0x3c, 15/15): `fnT(this->c->p->f8, 1, 97)`; zero -> return 14, else tail-call `fnU(this)`. `0x102f8` (return 18 on zero) and `0x10dbc` (return 36 on zero): same shape, 15/15.
- Open: `0x10844` (sibling of `0x7330`/`0xc508`), `0x113c8`/`0x13880`/`0x137f0` (destructor/stateful, untried).

## 2026-10-15 swarm batch 17

- `DllGts@0x119b4` (0x44, 17/17): three calls on `this->f24` (args `0x81,1,f50` / `6` / `1`), clears byte 0x41, returns 0x1a. `0x119f8` (0x18, 6/6): `fn(this->f24,1)`, returns 3. `0x11a10` (0x14, 5/5): stores 1 through `this->f10`, sets byte 0x40 = 1. `0x11a24` (0x70, 28/28): tick counter at 0x44; at 0x50 sets `f50=0x58`, calls, deletes `ctx->f9c` via vtable slot 1 after a null check; returns 0x18 or 0x19.
- Open: `0x11a94` (~0x54), `0x11b48` (~0x170), `0x11cb8` — large branchy state machines, untried.

## 2026-10-15 swarm batch 18

- `DllTownmap@0x2dc0` (0x14, 5/5): forwarder tail-calling `fn(this->[0x5c], 0, 0x104, x)`. `0x2ec8` (0x24, 9/9): `fn(this->[0x5c], this->[0x58], 0)`, sets byte `[0xc1] = 1`, returns 1 (the stored 1 doubles as the return value).

## 2026-10-15 swarm batch 19

- `DllPhotoViewer@0x19a8` (0x8, 2/2): byte setter `this[0xd9] = arg`. `0x19b0`, `0x19b8`, `0x1a08` (0x8, 2/2): virtual stubs returning 1. Tiny stubs remain at `0x1920`, `0x1928`-`0x1938`, `0x19c0`, `0x19c8` (weak evidence, unclaimed). `0x193c`/`0x1974` are dtor-stage helpers with the known-bad member-load-before-vtable ordering.

## 2026-10-15 swarm batch 20

- `DllBox@0x1dde8` (0x48, 18/18): init helper — stores `arg->f` into the owned sub-object, two veneer calls, stores `(*(f->[28])) + 0x13650`, returns 1. `arg->f` must be loaded once into a `void*` local and reused or the register schedule is wrong. `0x1dd60` (0x88, 34/34): teardown — walks `this->in->p`; if non-null and `q->c != 1`, virtual call (slot 9) plus four helper calls, then tail-calls a helper on `p->x` if non-null (matched first try).
- Open: DllBox small `ldr r0,[r0,#N]` getters near `0x12840`-`0x128a0`; switch tails around `0x1c9f4`-`0x1ce78`.

## 2026-10-15 swarm batch 21 (all `Style: natural`)

- `DllZukan@0x344c` (0x20): calls an import with `this->p` (+0x3c), returns `this->byte40 != 0`. `0x20b8` (0x28): vtable slot 0x20 on `this->p` (+0x44), returns `this->byte48 != 0`. `0xab20` (0x28): same as `0x20b8` with `+0x3c`/`byte40`.
- `0x8a70` (0x24): if `this->p` (+0xd0) non-null, vtable slot 1 with `(p, a)`, returns 0. `0x8a94` (0x24): same, slot 0. Retail loads the vtable pointer into r2, so the C takes the forwarded `int a` (it is passed on to the callee, so kept as natural).
- Open: `0x8338` (masked-matches the `0x344c` source, unwritten), `0x8ab8` (same as `0x8a70`, slot offset 12, untested).

## 2026-10-15 swarm batch 22 (all `Style: natural`)

- `DllStatus@0x2f80` (0x14, 5/5): tail-call wrapper `fn(this->f16, b, a)` with the two int args swapped. `0x2f94` (0x24, 9/9): next index with wrap — `n = fnCount(this->f8); return (i+1 < n) ? i+1 : 0`. `0x2fb8` (0x20, 8/8): previous index with wrap — `return i ? i-1 : n-1`.
- `0xa378` (0x88, 34/34): `obj = this->f4`; for k in {0,1,11,13} calls obj vtable slot {0,1,2,3}(obj) and returns 1, else 0 — a plain `switch` matches. `0x8724` (0x88, 34/34): same dispatch with the object at `this->f136`.

## 2026-10-15 swarm batch 23

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBox` | `0x17514` (0x30, 12/12) | natural | state-step: 5-arg import `(ctx->h, 25, &o->b, ctx->f, 0)` via `o->ctx`, returns 38. Using `o->ctx->x` directly gives retail's store-0-first order; a local `ctx` or `int z=0` loads ctx first (9/12) |
| | `0xeedc` (0x48, 18/18) | natural | check-then-act: `Chk(ctx->h->f4, 2, ctx->s24) == 0` -> 1, else `Do(ctx->h, ctx->b, s24, s26)` -> 0 (source recovered from the agent's scratch file) |
| | `0x11098` (0x54, 21/21) | natural | pairwise table search comparing `t[i]`/`t[i+1]` for i=start,+2 while i<12, returns `i+105`/`i+106`; miss -> error import, return 105. Must be a `do-while` (a `for` adds an entry precheck). Trailing zero pad word excluded from size |
| `DllField` | `0x5be0` (0x10, 4/4) | natural | `if (c) return 0; return 1;` (`arg3 == 0`) |
| | `0x76ec` (0xc, 3/3) | natural | stores arg1/arg2 into the int pair at +28/+32 (`add r0,#28; stm`) |
| | `0x5e34` (0x1c, 7/7) | natural | zeroes four floats at +4..+16 via `vldr` from the pool word + 4x `vstr` (size includes the pool word) |
| | `0xe3f4` (0x8, 2/2) | natural | stub returning -1 |
| | `0x10c34` (0x20, 8/8) | **trick(redundant-local)** | `sub[84]=0; sub[85]=1; this[73]=1`; locals `z=0, o=1` stop armcc hoisting the sub-pointer into r2 and mis-allocating constants |

Open DllField: `0x76f8`/`0x9ae0`/`0x9ae8` (`mov r0,#1` stubs), `0x7970`/`0x58b8` (`strb #1,[r0,#28]` setters), `0x630c`/`0x8604` (`ldr r0,[r0,#164]; mov r1,#0; b` tail calls), `0x10c54` (mirror of `0x10c34`), `0x112c8`/`0x112dc`/`0x112f0` (vtable-call wrappers). DllBox: `0x12868`-`0x12888` and `0x1c9f4`-`0x1ce78` are switch case bodies, not standalone functions; `0x12840` itself is a jump-table switch (untried).

## 2026-10-15 swarm batch 24

- `DllTownmap@0x5ac` (0x54, 21/21, natural): release helper — virtual-deletes members `[0x14]` (vtable slot 6) and `[0x18]` (slot 1) if set, zeroing each, then `fn(this+0x1c)`, returns `this->[0xc]`. `0xd68` (0x1c, 7/7, **trick(extra-arg)**): virtual wrapper — if `this->[0x88]`, tail-calls vtable slot 3 forwarding two int args (two args put the vtable in r3, one arg in r2, as in `0xb10`). `0xd84` (0x10, 4/4, natural): tail-call thunk `fn(this->[0x5c], this->[0x58], 0)`.
- **Relaxed-only (not counted):** `DllTownmap@0xaa4` (0x6c) — dtor stage, 26/26 with `--relax-nops`, 21/27 masked; sibling `0xa34` same body with a tail `b`. Same signature as `relaxed/DllBattle/0x30a4.c`: a destructor whose retail has one nop before a `bl`. Two independent cases now — the nop is the only difference — suggesting a recurring post-link relaxation window after the virtual-delete sequence.

## 2026-10-15 swarm batch 25

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllField` | `0x76f8`, `0x9ae0` (0x8, 2/2) | natural | return-true stubs |
| | `0x7970` (0xc, 3/3) | natural | sets byte at this+28 to 1 |
| | `0x630c` (0xc, 3/3) | natural | tail-calls `fn(this->p /*+164*/, 0)` |
| | `0x10c54` (0x20, 8/8) | **trick(redundant-local)** | mirror of `0x10c34`: `sub[84]=1; sub[85]=0; this[73]=1`; locals `z=0, o=1` required |
| `DllBag` | `0x11398` (0x2c, 11/11) | natural | dtor stage storing a passed-in vtable, `g1(o->sub->m)`, tail-call `g3((char*)g2(o+0x50)-0x50)` |
| | `0x113c8` (0x30, 12/12) | natural | same body, vtable from the pool word (size includes it); returns `(char*)g2(o+0x50)-0x50` |

Open DllField: `0x8604` (looks identical to `0x630c`), `0x9ae8` (return-1 stub), `0x112c8`/`0x112dc`/`0x112f0` (vtable-call wrappers returning 0, slots 0/4/8). DllBag `0x13880`/`0x137f0` (destructor/stateful) untried.

## 2026-10-15 swarm batch 26 (all `Style: natural`)

- `DllBattle@0x2c9bc` (0x30, 12/12): `if (this->vt[0x98/4](this)) tail-call fn(this->+0x12c);` void. `0x2c9f4` (0x34, 13/13): same, `return fn(...)` else `return 1`. `0x2c8c0` (0x38, 14/14): same, else `return 0.0f` (float; size includes the trailing pool word). `0x2d5a0` (0x38, 14/14): same source as `0x2c8c0`, different tail-call target. C: struct with vtable pointer, `pad[0x128]`, int at 0x12c; test is `o->vt[38](o)`.
- Their tail-call callees (`0xa94`/`0xaa4`/`0xaac`/`0xb04`) look like a related accessor family; ~120 small `blx`-using functions remain past `0x3a00` in DllBattle.

## 2026-10-15 swarm batch 27

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllStatus` | `0x2e54` (0x20, 8/8) | natural | `f470(c, f468(this->f8))` with the 3rd arg kept in r4 across the call, tail-calling `f470` |
| | `0x2e74` (0x1c, 7/7) | natural | tail-calls `f470(f478(this->f8), c)` |
| `DllGts` | `0x5ad4` (0x1c, 7/7) | **trick(ordering-local)** | `f4f4(**(this+16), *(int*)(this+48))`, returns 50; only matches with two locals (`q`, `v`) |
| | `0x36d4` (0x18, 6/6) | natural | `f314(this+32)`, returns int at `this+20` |
| | `0x486c` (0x18, 6/6) | natural | tail-calls `f4ec(f4cc(this), c)` |
| `DllSalon` | `0xd5c` (0x38, 14/14) | **trick(ordering-local)** | dtor stage: installs the vtable, frees member +0x14, tail-calls a second import (size includes its pool word at 0xd90) |
| | `0xd94` (0x34, 13/13) | natural | dtor stage: loads member +0x14 first, installs the vtable, frees if non-null, returns `this` — plain C matches here |
| | `0xdd0` (0x2c, 11/11) | **trick(volatile-strd)** | globals init: -1 via `strd` to `g_a[1]` (array base +8), 0 to a second 64-bit global |
| | `0x790` (0x20, 8/8) | **trick(void-ptr-passthrough)** | secondary-base thunk: import, post-indexed vtable store at +0x24, second import on the advanced pointer, returns result-0x24 |

**Relaxed-only (not counted):** `DllStatus@0x2fd8` (0x64) and `0x2e90` — search loop, match only with `--relax-nops`; retail has a nop between `cmp r0,#0` and `beq` (a third nop-window variant besides the DllBattle@0x30a4 / DllTownmap@0xaa4 dtors).

Open DllSalon: `0xdfc` (5-case property switch, needs `--mask-ranges`), `0x4ac` (call chain ending in `ldrd` + tail call), large functions `0x3a0`/`0x500`/`0x83c`/`0xaa0`. DllStatus `0x4184` (`f6280(); tail f310()`) matched but wasn't saved — a quick pickup.

## 2026-10-15 swarm batch 28 (all `Style: natural`)

- `DllField@0x8604` (0xc, 3/3): tail-calls an import veneer with `(this->field@0xa4, 0)`. `0x9ae8` (0x8, 2/2): returns 1. `0x142cc` (0x1c, 7/7): sets byte `this+0x25` = 1, `(this+0x20)->+0x1c` = 0, tail-calls a veneer with `(this->+4, 0)`. `0x142e8` (0x20, 8/8): same, sub-object byte set to 2.
- `0x112c8`/`0x112dc`/`0x112f0` are **case bodies** of one switch function (`cmp r1,#0x21/#0x27/#0x3c` at `0x112a8`-`0x112bc`, shared `pop {r4,pc}` epilogue), not standalone functions; the enclosing switch is the real candidate.

## 2026-10-15 swarm batch 29 (all `Style: natural`)

- `DllBattle@0xc9220` (0x2c, 11/11): `fa(o->x /*+0xc*/, o->vt[23](o))` — virtual call whose result is tail-called with a field. `0xc9298` (0x2c, 11/11): same with `vt[24]`. `0xc92c4` (0x2c, 11/11): same shape as `0xc9298`, different callee. `0xc8eb0` (0x1c, 7/7): `return o->vt[18](o) != 0`. `0xc8f94` (0x20, 8/8): `if (o->vt[24](o) != 4) return 1; return 0;`.
- Open: `0xc9018` (0x2c) masks 11/11 as `void fn(O *o, int a) { fa(o->x /*+0x14*/, o->vt[68](o, a)); }` but needs an unused `int a` (would be `trick(extra-arg)`); `0x2cbfc` (`if (vt[0x98](o)) { o->b346 = a; tail fn(o); }`) and `0x2cdc4` (`if (vt[0x98](o)) fn(o->x, a)`) — gated tail-call siblings of the `0x2c9bc` family; compare-and-return wrappers `0x42c0c`/`0x42c30` (`vt[0x80]`, `r == 160 || r == 161` and `r == 103 || r == 104`); `0x39884`.

## 2026-10-15 swarm batch 30

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllStatus` | `0x4184` (0x10, 4/4) | natural | `f6280(); tail f310()` |
| `DllBag` | `0x137f0` (0x3c, 15/15) | natural | clears field +0x88, calls two helpers, sets the field to 1 if the 2nd call is nonzero and the field is still 0; written `void` (retail leaves r0 live at the pop) |
| | `0x1382c` (0x10, 4/4) | natural | `fc78(); tail f300()` |
| | `0x13848` (0x38, 14/14) | **trick(ordering-local)** | dtor stage: vtable, free `buf` at +0x14 if non-null + zero, tail-call `f300(this)`; `void *vt = VT;` local forces the pool load ahead of the member load (natural form: 10/14) |
| | `0x13880` (0x34, 13/13) | natural | same dtor without the tail call, returns `this` |

**Docs findings (DllBag dtor agent):** declare the vtable as an array / `extern char VT[];`, not a scalar pointer — `void *VT` adds a dereference of the pool word and breaks the match; and the size passed to `fingerprint_fn.py` must include the trailing literal-pool word for dtors. Worth retrying the known-bad destructors (see `attempted.md`) with these in mind.

## 2026-10-15 swarm batch 31

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllZukan` | `0x8338` (0x20, 8/8) | natural | `fn(this->p@0x3c)`, returns `byte@0x40 != 0` (same source as `0x344c`) |
| | `0x8ab8` (0x24, 9/9) | natural | sibling of `0x8a70`/`0x8a94`: if `p@0xd0`, vtable slot 3 (+12) with `(p, a)`, returns 0; `a` must be forwarded (6/9 without) |
| `DllGts` | `0x7e50`, `0x80f0` (0x3c, 15/15) | natural | `fn(c@0x20, this+0x24, 3 / 4, 0, p->byte@0x226 / 0x227, 0)` — two stack args, returns 5 |
| | `0x93ac` (0x34, 13/13) | natural | if `fn(this)` non-zero return `r@0x1c`; else if `byte@4` store 6 to `r@0x14` and return 1; else return 2 |
| `DllBattle` | `0xc9018` (0x2c, 11/11) | natural | `void fn(O *o, int a) { fa(o->x /*+0x14*/, o->vt[68](o, a)); }` (the arg is forwarded to the call, so natural) |
| | `0x42c0c` (0x24, 9/9) | natural | `r = o->vt[32](o, a); return r == 160 \|\| r == 161;` |
| | `0x42c30` (0x24, 9/9) | natural | same, `r == 103 \|\| r == 104` |
| `DllHeading` | `0x55d8` (0x48, 18/18) | natural | looks up an 8-byte table entry by index; two imports on the sub-object at +4 with the entry's byte (2nd takes stack arg 1) |
| | `0x868` (0x40), `0x16f4` (0x40) | **trick(void-ptr-passthrough)** | passed-in vtable stored, two fields forwarded to `o->q->h`, tail-call with base adjusted by -4 (`(char*)fn(p) - 4`) |
| | `0x8ac` (0x44), `0x1738` (0x44) | **trick(void-ptr-passthrough)** | same with the vtable from the literal pool; returns the adjusted pointer |

Open DllHeading: the same two shapes also masked-match at `0x1ca8`/`0x3df0`/`0x8c58` (tail form, size 0x40) and `0x1cec`/`0x3e34`/`0x8c9c` (literal-pool form, size 0x44) — unsaved; plus dtors `0x87e4` (0x38) / `0x881c` (0x34) at 12/14 / 11/13 (retail zeroes the member via r1). Open DllBattle: `0x39884` (long chain of virtual calls and compares), `0x2cbfc`/`0x2cdc4` (gated tail-call siblings). DllGts `0x7c74` (`fn(this->c@0x20, 1); return 4`, masks 6/6, unsaved).

## 2026-10-15 swarm batch 32 (all `Style: natural`)

- `DllBattle@0x2ca28` (0x44, 17/17): `if (vt[0x98](o)) fa(o->x /*+0x12c*/, a)` with a `float` arg. `0x2d5d8` (0x44, 17/17): same shape. `0x393d8` (0x20, 8/8): `return vt[0x3e](o)->b5 != 0`. `0x2cdfc` (0x8, 2/2): tail call `fa(o->x /*+0x12c*/)`. `0x2cc34` (0xc, 3/3): `return vt[0x98](o)` (virtual tail call).
- **Int-arg gated tail-calls don't match yet:** `0x2cbfc`, `0x2cdc4`, `0x2ce44` (and likely `0x2c630`, 0x50) stop at 12/14 — retail does `mov r5,r1; ldr r1,[r0,#0x98]; blx r1`, armcc emits `ldr r2 ... blx r2` whenever an int arg is live across the virtual call (float-arg siblings match since the arg is in s0). See `attempted.md`.
- Scan caveat: the agent only scanned functions starting with `push {..., lr}` in `0x2c000`-`0x2e000` / `0x39000`-`0x3a000`; push-less leaf functions weren't scanned.

## 2026-10-15 swarm batch 33 — the destructor recipe

Retrying the "known-bad destructors" with two findings (array-declared vtable; a `void *vt = VT;` local forcing the pool load ahead of the member load) matched **6 of 12**. Recipe: `extern char VT[];` (never a scalar pointer), `void *vt = VT;` first, then the member load, `o->vptr = vt;`, free-if-non-null + zero, then tail-call or return `this`; verify at the size including the trailing literal-pool word.

| Module | Offset (size, match) | Style |
|---|---|---|
| `DllPuzzle` | `0xbf80` (0x38, 14/14) | **trick(ordering-local)** — tail-call dtor |
| `DllTownmap` | `0x31d4` (0x38, 14/14) | **trick(ordering-local)** |
| `DllPhotoViewer` | `0x193c` (0x38, 14/14) | **trick(ordering-local)** |
| | `0x1974` (0x34, 13/13) | natural — returns `this` via `pop {r4,pc}` |
| `DllStatus` | `0xa304` (0x38, 14/14) | **trick(ordering-local)** |
| | `0xa33c` (0x34, 13/13) | natural — returns `this` |

Still failing (`attempted.md`): DllPuzzle@0xb3fc (12/14), DllBox@0x1dfac (23/25) and 0x1df3c (20/28) — all blocked by the zero-in-r1 diff (retail `mov r1,#0; str r1,[...]`, ours r0); DllTownmap@0x31a0 ctor wrapper (8/12); DllBattle@0x3038 and DllPokeList@0xa50 are relaxed-only at best (post-link nop words).

## 2026-10-15 swarm batch 34

- `DllBattle@0x42bf4` (0x18, 6/6, natural): `vt[0x3e](this)` then returns the byte at +0x27 of the result.
- `DllBattle@0x7a780` (0x30, 12/12, **trick(extra-arg)**): takes `this` as the second argument; if `vt[0x42](this)` returns 0, tail-calls a function with `this->+0x190`. The unused first argument only puts `this` in r1.
- `DllBattle@0x39884` (0x150, 84/84, natural): state classifier via a chain of virtual calls, returns 0-3 — 0 if `vt[0x42](this)` is set or `vt[0x10](this)` isn't 1-3; 3 for specific `vt[0x20](this,1)` results (4 and 39-45); 2 or 1 from the `vt[0x3a]`/`vt[0x39]`/`vt[0x35]` probes. The vtable must be typed `int (**vt)()` (unprototyped) because the slots take different argument counts. **Largest real-logic match so far.**
- Open: `0x224c` (0x30, same shape as matched `0x2c60`). Scans of `0x40000`-`0x44000` and `0xc0000`-`0xd0000` found only these; the other hits were mid-function fragments.

## 2026-10-15 swarm batch 35

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllHeading` | `0x1ca8`, `0x3df0`, `0x8c58` (0x40, 16/16) | **trick(void-ptr-passthrough)** | tail form: store the vtable passed in r1 at `this[0]`, push the two config ints (`this+16`, `this+20`) into the handle at `this->q->h` (`q` at +12, handle at `q+0x18`), tail-call with the sub-object (`this+4`) base adjusted by -4 |
| | `0x1cec`, `0x3e34`, `0x8c9c` (0x44, 17/17) | **trick(void-ptr-passthrough)** | literal-pool form: own vtable from the pool (`extern void *VT[];`), same pushes, returns the sub-object base minus 4 |
| `DllGts` | `0x16aec` (0x14, 5/5) | natural | sets the byte at `*(this+0xc)` to 1 and `this->byte@0x3c` to 1 |
| | `0xb054` (0x14, 5/5) | natural | `this->ctx@0x24->byte@0x624 = 1`, returns 5 |
| | `0xb0f4` (0x18, 6/6) | natural | `this->byte@0x40 = 1`, clears `this->ctx@0x24->byte@0x624` |
| `DllPuzzle` | `0x99dc` (0x24, 9/9) | natural | copies four ints from a source struct into `this+0x3c/+0x40/+0xb0/+0xb4` |
| `DllEshop` | `0x9a8` (0x10, 4/4) | natural | `this->byte@0xe4 = 1`, returns 1 |

Push-less leaf scan (`armcc-decomp-55`): few leaves in DllBox (1), DllBag (1), DllGts (22), DllPuzzle (6), DllEshop (2); **DllBattle has 794 and DllField 293 — those lists weren't examined**. Not matched: DllGts `0x2a08` (2/4, zero through r2 vs r1 — the zero-register class), `0x7c8c` (1 word: retail hoists `mov r1,#1`), DllPuzzle `0x100c` (3 words: retail avoids conditional execution where armcc emits `moveq`/`strbeq`).

## 2026-10-15 swarm batch 36 (all `Style: natural`)

- `DllGts@0x7c74` (0x18, 6/6): import call `(this->child@0x20, 1)`, returns 4. `0x824c` (0x3c, 15/15): 6-arg import call — 5th/6th spill to the stack and armcc packs them with `strd` — args `(this->child@0x20, &this->f24, 0, &state+0x250, state->int@0x218, 0)` with `state` = object at `this+0xc`; returns 5. `0x8354` (0x3c, 15/15): same shape, arg 3 = 1 and the int at `state+0x21c`; returns 13.
- Remaining DllGts near `0x7cb0`-`0x8100` are large state-machine handlers.

## 2026-10-15 swarm batch 37

- `DllKisekae@0xb674` (0x38, 14/14, **trick(ordering-local)**): installs the vtable, frees the owned member at +0x14 if non-null and zeroes it, tail-calls the base dtor; needs `void *vt = VT; void *m = o->m;` so the pool load precedes the member load. `0xb6ac` (0x34, 13/13, **trick(ordering-local)**): frees the member and zeroes it, installs the vtable, returns `this`; here the member load comes first, so the hoisted `m = o->m` (no `vt` local) is what the ordering needs; size includes the trailing pool word (0x30 omits it and fails). The two look alike but need opposite load orders; a plain `o->vptr = VT; if (o->m) ...` fails on both.

## 2026-10-15 swarm batch 38 — DllBattle push-less leaves (all `Style: natural`)

- `0x567e0` (0x18, 6/6): fills 8 bytes by replicating the low byte of the arg into a u32 and storing it twice. `0x6fd18` (0x18, 6/6): same fill over 6 bytes (u32 + u16). `0xc9e84` (0x1c, 7/7): over 10 bytes (u32, u32, u16).
- `0x5daf8` (0x14, 5/5): counted array getter `i < p[24] ? ((u32*)p)[i] : 0` (the `ldrhi/movls` conditional-execution form matches). `0x76360` (0x24, 9/9): zero-inits a struct with six u32 stores and a zero byte at +24.
- **Scan:** functions of 0xc-0x60 bytes with no `push` and no `b`/`bl` ending in `bx lr` (bounds from the Ghidra `cro_c` filenames), minus anything in `matched/` — 59 non-trivial candidates remained; offsets are valid for `fingerprint_fn.py`.
- **Families for the next round:**
  - counted u32 array getter: `0x76384` (identical bytes to `0x5daf8`, same C should match); byte-array variant `0xc87a0` (`ldrbhi/movls #6`, count at +25)
  - zero-init structs: `0xc5c18` (0x28, strb at +24/+25 plus six u32 stores), `0x71840` (u32+u16), `0xc9e58` (u32+u8), `0xc9e30` (0x28, two small functions)
  - field copies: `0x5fa8c` (u32+u16), `0x5faa0` (u32+u8), `0xc9e68` (u32+u16+u8), `0x747fc` (2×u32)
  - bit-test getters: `0x4aa9c`, `0x51c28` (`ldrb [27]; and #64/#32; lsr`); bitset-test `0x4c818`, `0x4ca4c`
  - `add r0,r0,#N; ldrh` getters: `0x1cc04`, `0xc86c8`, `0xc8754`, `0x4c278`, `0x4c7fc`
  - bit-pack builders: `0x704ac`, `0x76e20`, `0x797b0`

## 2026-10-15 swarm batch 39 — DllField push-less leaves (all `Style: natural`)

- `0x412d0` (0xc, 3/3): pair setter, stores arg1/arg2 at `this+28/+32` (`add r0,#28; stm`). `0x36d18` (0x10, 4/4): `this+24 = owner; this+32 = owner->+408`. `0x74dc8` (0x10, 4/4): sets `this->kind` (+134) to 1, returns `&arg1->sub` (+256). `0x7f60c` (0x10, 4/4): table getter — u16 at +10 of the 12-byte entry `[i]` with the table pointer in r3 (4th arg). `0x9ff58` (0xc, 3/3): getter `this->+68->+148`.
- **Sibling families for the next round** (same C, only constants change):
  - pair setter (`add r0,#28; stm`): `0x1959c`, `0x41f24`, `0x42f7c`, `0x444f0`, `0x4344c`, `0x9888` (store at +28); `0x417f8` (+44)
  - owner-init (`str r1,[r0,#24]; ldr r1,[r1,#408]; str r1,[r0,#N]`): `0x2aec4`, `0x343b4` (N=28); `0x3424c` (N=36); `0x387c0`, `0x3e5c8` (N=32)
  - kind setter + sub-record address: `0x74dd8` (K=2, +276), `0x74de8` (K=3, +296), `0x74df8` (K=3, +308), `0x74e08` (K=3, +320), `0x74e18` (K=4, +332)
  - table getters: `0x7f628` (+22), `0x7f65c` (+8), `0x7f678` (+20); chain getters: `0xa0bdc` (+152), `0xa0024` (`ldrb`, +160)
  - other shapes worth a try: `ldr r1,[pc,#4]; str r1,[r0]; bx lr` vptr setters (`0x28b24`, `0x28dc0`, `0x2905c`, `0x2907c`, `0x2941c`, `0x29abc`, `0x29e1c`); range predicates (`0x4a408`, `0x57d1c`, `0x57d54`); `vldr`/`vstr` zero-float inits (`0x7f40`, `0x7fb78`); `ldmia`/`vstmia` struct copies (`0x72534`, `0x72554`, `0x7dbc4`, `0x70f58`)
- **Scan caveat:** Ghidra's function list omits push-less leaves (why they were unexamined); the scan cuts chunks at push/pop/b/bl/bx and leading literal-pool words show up as junk instructions that need stripping — the recipe is in the `0x412d0` header.

## 2026-10-15 swarm batch 40 — zero-in-r1 leaf class cracked

- `DllField@0xae58` (this+20/+24), `0xe3fc` (this+20/+24) and `0xe380` (this+24/+28) (each 0x10, 4/4, **trick(live-r1-passthrough)**): zero two adjacent ints, retail zeroes through r2. Mechanism: armcc puts the shared zero in the lowest free scratch register after r0 (`this`); retail had r1 live (an incoming arg handed back). An unused extra parameter does NOT keep r1 live; declaring `long long f(T *t, int x)` and returning `((long long)x << 32) | (unsigned)t` does, with zero extra code. **The real signature is unknown — this is the least natural trick form in the tree**; rewrite when the true signature is found.
- The dtor variant of this class (`DllPuzzle@0xb3fc`, `DllHeading@0x87e4`/`0x881c`, `DllBox@0x1dfac`/`0x1df3c`, `DllGts@0x9824`) is still open, but its instruction ordering is solved (`void **vt = VT; void *m = o->f14; o->vptr = vt; if (m) {...}`); only the zero register differs. See `attempted.md`.

## 2026-10-16 round a — batch 1 (DllBattle getters, all natural)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x1cc04` (0xc, 3/3) | natural | returns the address of the embedded sub-object at this+0xca4 (add 0xc00; add 0xa4) |
| `DllBattle` | `0x4c278` (0xc, 3/3) | natural | getter: returns the u16 field at this+0x162 |
| `DllBattle` | `0x4c7fc` (0xc, 3/3) | natural | getter: returns the u16 field at this+0x160 |
| `DllBattle` | `0xc86c8` (0xc, 3/3) | natural | getter: returns the u16 field at this+0x15c |
| `DllBattle` | `0xc8754` (0xc, 3/3) | natural | getter: returns the u16 field at this+0x150 |

## 2026-10-16 round a — batch 2 (DllBattle + DllField families)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x2984` (0x40, 16/16) | natural | lazy-member creator: n = new(12, this->@0x20); if (n) { n->@4 = n->@8 = 0; n->vt = <vtable> } this->@0x14 = n; n->@4 = this (back-pointer). Pool word is a relocation. Masked match 16/16. |
| `DllBattle` | `0xc074` (0x30, 12/12) | natural | virtual-override: ctx = *this; sets ctx bytes @0x7d8/@0x7d9 = 1 and ctx->@0x7e0 = ctx->@0x10->@0x40, calls import(10), returns 1. Masked match 12/12. |
| `DllBattle` | `0xea24` (0x18, 6/6) | natural | virtual-override wrapper: calls import(this->@0x40, 0) and returns 1 (true). Masked match 6/6. |
| `DllBattle` | `0x16aa0` (0x14, 5/5) | natural | virtual-override wrapper: calls import(this, this->@0x10) and returns 1 (true). Masked match 5/5. |
| `DllBattle` | `0x16bf0` (0x34, 13/13) | natural | virtual-override: calls import(this+0x88, this, 0) then import(this+0x428, this, 1) (two embedded sub-objects bound to this), returns 1. Masked match 13/13. |
| `DllBattle` | `0x44d4c` (0x18, 6/6) | natural | calls a helper and returns its logical negation (1 if it returned 0). |
| `DllBattle` | `0x4d348` (0x1c, 7/7) | natural | looks up an object via this->[2], returns its byte field at +25, or 31 if none. |
| `DllBattle` | `0x5f848` (0x18, 6/6) | natural | calls a helper and returns true if it returned 0. |
| `DllBattle` | `0x71840` (0x10, 4/4) | natural | zero-inits a 6-byte struct: u32 at +0, u16 at +4 |
| `DllBattle` | `0x742b0` (0x10, 4/4) | natural | calls a helper (result discarded) and returns 0. |
| `DllBattle` | `0x76384` (0x14, 5/5) | natural | counted u32 array getter, i < p[24] ? ((u32*)p)[i] : 0 (twin of 0x5daf8) |
| `DllBattle` | `0xc5c18` (0x28, 10/10) | natural | clears a 26-byte struct: zero byte at +24 and +25, then six zero u32 stores (+0..+20) |
| `DllBattle` | `0xc87a0` (0x14, 5/5) | natural | counted byte array getter, i < p[25] ? p[i] : 6 |
| `DllBattle` | `0xc9e30` (0x14, 5/5) | natural | zero-inits a 10-byte struct: u32 at +0, u32 at +4, u16 at +8 |
| `DllBattle` | `0xc9e44` (0x14, 5/5) | natural | zero-inits a 12-byte struct: three zero u32 stores (+0,+4,+8) |
| `DllBattle` | `0xc9e58` (0x10, 4/4) | natural | zero-inits a 5-byte struct: u32 at +0, u8 at +4 |
| `DllField` | `0x7f40` (0x1c, 7/7) | natural | zeroes four floats at this+4..+16 (vldr s0 from the pool word, 4x vstr); size includes the literal-pool word. |
| `DllField` | `0x9888` (0xc, full (masked)) | natural | setter: stores arg1/arg2 into the adjacent int pair at this+28/this+32 (add r0,#28; stm). |
| `DllField` | `0x132b8` (0x1c, 7/7) | natural | Predicate: passes this->field@0x24 to a callee and returns its logical negation (1 if result is 0). |
| `DllField` | `0x14c60` (0x1c, 7/7) | natural | Predicate: passes this->field@0x24 to a callee and returns its logical negation (1 if result is 0). |
| `DllField` | `0x1959c` (0xc, full (masked)) | natural | setter: stores arg1/arg2 into the adjacent int pair at this+28/this+32 (add r0,#28; stm). |
| `DllField` | `0x1bd34` (0x14, 5/5) | natural | Call+return helper: calls an import veneer with this, returns this. |
| `DllField` | `0x28b28` (0x10, 4/4) | natural | vptr setter: stores a class vtable address (pool word, relocated at link) into this[0]; size includes the pool word. |
| `DllField` | `0x28dc4` (0x10, 4/4) | natural | vptr setter: stores a class vtable address (pool word, relocated at link) into this[0]; size includes the pool word. |
| `DllField` | `0x29060` (0x10, 4/4) | natural | vptr setter: stores a class vtable address (pool word, relocated at link) into this[0]; size includes the pool word. |
| `DllField` | `0x29080` (0x10, 4/4) | natural | vptr setter: stores a class vtable address (pool word, relocated at link) into this[0]; size includes the pool word. |
| `DllField` | `0x29420` (0x10, 4/4) | natural | vptr setter: stores a class vtable address (pool word, relocated at link) into this[0]; size includes the pool word. |
| `DllField` | `0x29ac0` (0x10, 4/4) | natural | vptr setter: stores a class vtable address (pool word, relocated at link) into this[0]; size includes the pool word. |
| `DllField` | `0x29e20` (0x10, 4/4) | natural | vptr setter: stores a class vtable address (pool word, relocated at link) into this[0]; size includes the pool word. |
| `DllField` | `0x2aec4` (0x10, full (masked)) | natural | init: stores the owner pointer arg at this+24 and caches owner->field(+408) at this+28. |
| `DllField` | `0x334d8` (0x20, 8/8) | natural | Destructor stage: calls an import veneer (this, 1), then another on the sub-object at this+108, returns this. |
| `DllField` | `0x3424c` (0x10, full (masked)) | natural | init: stores the owner pointer arg at this+24 and caches owner->field(+408) at this+36. |
| `DllField` | `0x343b4` (0x10, full (masked)) | natural | init: stores the owner pointer arg at this+24 and caches owner->field(+408) at this+28. |
| `DllField` | `0x387c0` (0x10, full (masked)) | natural | init: stores the owner pointer arg at this+24 and caches owner->field(+408) at this+32. |
| `DllField` | `0x3e5c8` (0x10, full (masked)) | natural | init: stores the owner pointer arg at this+24 and caches owner->field(+408) at this+32. |
| `DllField` | `0x417f8` (0xc, full (masked)) | natural | setter: stores arg1/arg2 into the adjacent int pair at this+44/this+48 (add r0,#44; stm). |
| `DllField` | `0x41f24` (0xc, full (masked)) | natural | setter: stores arg1/arg2 into the adjacent int pair at this+28/this+32 (add r0,#28; stm). |
| `DllField` | `0x42f7c` (0xc, full (masked)) | natural | setter: stores arg1/arg2 into the adjacent int pair at this+28/this+32 (add r0,#28; stm). |
| `DllField` | `0x4344c` (0xc, full (masked)) | natural | setter: stores arg1/arg2 into the adjacent int pair at this+28/this+32 (add r0,#28; stm). |
| `DllField` | `0x444f0` (0xc, full (masked)) | natural | setter: stores arg1/arg2 into the adjacent int pair at this+28/this+32 (add r0,#28; stm). |
| `DllField` | `0x4a408` (0x18, 6/6) | natural | range predicate: returns (this->field4 - 6) <= 3 unsigned, i.e. field4 in 6..9. |
| `DllField` | `0x57d1c` (0x18, 6/6) | natural | range predicate: returns (this->field4 - 4) <= 3 unsigned, i.e. field4 in 4..7. |
| `DllField` | `0x57d54` (0x14, 5/5) | natural | predicate: returns this->field4 == 7. |
| `DllField` | `0x5ee58` (0x14, 5/5) | natural | Virtual-override stub: ignores this, calls an import veneer with arg2->f4 (the second argument's field at offset 4) and returns 0. |
| `DllField` | `0x74dd8` (0x10, 4/4) | natural | kind setter (=2), returns sub-record at arg1+276 |
| `DllField` | `0x74de8` (0x10, 4/4) | natural | kind setter (=3), returns sub-record at arg1+296 |
| `DllField` | `0x74df8` (0x10, 4/4) | natural | kind setter (=3), returns sub-record at arg1+308 |
| `DllField` | `0x74e08` (0x10, 4/4) | natural | kind setter (=3), returns sub-record at arg1+320 |
| `DllField` | `0x74e18` (0x10, 4/4) | natural | kind setter (=4), returns sub-record at arg1+332 |
| `DllField` | `0x7f628` (0x10, 4/4) | natural | table getter: u16 at +22 from 12-byte entry [i] of r3 table |
| `DllField` | `0x7f65c` (0x10, 4/4) | natural | table getter: u16 at +8 from 12-byte entry [i] |
| `DllField` | `0x7f678` (0x10, 4/4) | natural | table getter: u16 at +20 from 12-byte entry [i] |
| `DllField` | `0x7fb78` (0x5c, 23/23) | natural | initializer: zeroes two u16, 11 floats at +4..+0x2c, int +0x30, float +0x34, sets int +0x38 = 15, zeroes +0x3c/+0x40; size includes the pool word. |
| `DllField` | `0x84e54` (0x18, 6/6) | natural | Bool getter: returns (Import(this->f12) != 0), i.e. loads the pointer at offset 12, passes it to an import veneer and normalises the result to 0/1 (cmp/movne). |
| `DllField` | `0x948d8` (0x18, 6/6) | natural | Field-chain getter: loads this->f68 (0x44), calls a no-arg import veneer for its side effect (result unused), then returns the byte at ((f68)->f144)+330. |
| `DllField` | `0x98bf0` (0x10, 4/4) | natural | Negated predicate: calls an import veneer with this unchanged and returns its result XOR 1 (logical not of a boolean getter). |
| `DllField` | `0xa0024` (0xc, 3/3) | natural | getter this->inner(+68)->u8(+160) |
| `DllField` | `0xa0bdc` (0xc, 3/3) | natural | getter this->inner(+68)->int(+152) |
| `DllField` | `0xe4878` (0x10, 4/4) | natural | Call+return helper: forwards this to an import veneer (void result discarded) and returns the constant 12. |

Notes:
- **DllField vptr setters** (`extern char VT[]; *p = VT;`, 0x10 incl. the pool word): the offsets in the earlier list were the *preceding pool word*; the functions start 4 bytes later — `0x28b28`, `0x28dc4`, `0x29060`, `0x29080`, `0x29420`, `0x29ac0`, `0x29e20`.
- **DllField table getters at +20/+22** reach past the 12-byte stride into the next entry, so the `t[i].k` form of `0x7f60c` can't express them: use `struct E { char pad[12]; }` and `*(unsigned short *)((char *)&t[i] + N)`.
- Open DllField: tail-call thunks to `0x16c0`/`0x6d64c` (e.g. `0x9d9d4`, `0x92318`, `0x9e904`, `0x9ea4c`, `0x9ea64`, `0x9ea7c`, `0x9ecd8`, `0x9ecf0`); the `bl; mov r0,#0/#1` family (`0x945d4`, `0x97ce0`, `0x9e594`, `0x95454`, `0xa50c4`); `push {lr}` leaf stubs `0x4a564`, `0x4ebd8`. ~25 push-with-lr+call candidates below `0x40000` were outside `matched/`.
- Open DllBattle: `0x2f34` (same shape as `0x2984`, `this->@0x1c`/`@0x10`), `0x2da0`/`0x2dd8` (dtor-style, free member +0x14), `0x5faa0` (u32+u8 copy; same residue expected), `0x4ca4c` (bitset test).

## 2026-10-16 round a — batch 3 (DllField GetMgr chains, DllBattle forwarders/comparisons/leaves)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x224c` (0x30, 12/12) | natural | teardown step: if (this->@0x24) vcall slot 1; fn(this->@0x20); return 1 |
| `DllBattle` | `0x2f34` (0x40, 16/16) | natural | lazy-member creator: n = new(12, this->@0x1c); init n (owner/b/vt) if non-null; this->@0x10 = n; n->owner = this |
| `DllBattle` | `0x12fcc` (0x40, 16/16) | **trick(ordering-local)** | dtor stage: install vtable, vcall slot 1 on member @8 if non-null, free member @0xc if non-null (no zero), return this |
| `DllBattle` | `0x4b81c` (0x10, 4/4) | natural | getter: loads this->+4 -> +16 (object pointer chain) then returns the u8 at +16 (3-deep ldr/ldr/ldrb chain). Family: 0x4bf14 (u8 at +1), 0x544ec (u8 at +0), identical chain. |
| `DllBattle` | `0x4b8d4` (0xc, 3/3) | natural | getter: returns the u8 at +9 of the object pointed to by this->+28 (ldr; ldrb). Family: 0xc7580 (+16 -> +72). |
| `DllBattle` | `0x4bf14` (0x10, 4/4) | natural | getter: this->+4->+16 pointer chain, returns the u8 at +1 of the final object. Sibling of 0x4b81c (field +16) and 0x544ec (field +0). |
| `DllBattle` | `0x544ec` (0x10, 4/4) | natural | getter: this->+4->+16 pointer chain, returns the u8 at +0 of the final object. Sibling of 0x4b81c / 0x4bf14. |
| `DllBattle` | `0x704ac` (0x20, 8/8) | **trick(redundant-local)** | bit-pack builder + store: *p = (a<<4 & 0x70) \| 3 \| (b<<7 & 0x380) (two fields, constant tag 3). |
| `DllBattle` | `0x76e20` (0x10, 4/4) | natural | bit-pack builder: ((a << 3) & 0x1f8) \| 3 -- 6-bit field at bit 3 with a constant low tag 3. Identical bytes at 0x9e638 (also matched here). |
| `DllBattle` | `0x78668` (0x24, all (masked)) | natural | returns fn(0x100) > *p (unsigned compare of a helper result against a word read through p). |
| `DllBattle` | `0x78c70` (0x24, all (masked)) | natural | returns fn(0x100) == *p (helper result compared for equality with a word read through p). |
| `DllBattle` | `0x78c94` (0x24, all (masked)) | natural | returns fn(0x100) < *p (unsigned compare of a helper result against a word read through p). |
| `DllBattle` | `0x79624` (0x24, all (masked)) | natural | returns fn(0x100) != *p (helper result compared for inequality with a word read through p). |
| `DllBattle` | `0x797b0` (0x1c, 7/7) | **trick(redundant-local)** | bit-pack builder: (b<<3 & 0x1f8) \| 4 \| (a<<9 & 0x7e00) -- two fields plus constant tag 4. Retail hoists both mask constants before the shifts/ands; plain expression loads them interleaved. |
| `DllBattle` | `0x80124` (0x1c, all (masked)) | natural | returns whether fn(u16 at +198, 3) is non-zero (bool wrapper over a helper call). |
| `DllBattle` | `0x82324` (0x14, all (masked)) | natural | returns !fn(o, p->a, p->b): calls a predicate with a two-word pair unpacked from p, then flips the boolean (eor #1). |
| `DllBattle` | `0x9e638` (0x10, 4/4) | natural | bit-pack builder: ((a << 3) & 0x1f8) \| 3 -- byte-identical sibling of 0x76e20. |
| `DllBattle` | `0xa6a50` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (4) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa6d3c` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (1) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa7584` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (0) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa782c` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (2) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa7d34` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (3) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa7e94` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (4) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa82a4` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (1) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa8e48` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (0) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa91d8` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (2) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa958c` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (16) to shared worker 0x4c284 |
| `DllBattle` | `0xa95ac` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (3) to shared worker 0x4c5e0 |
| `DllBattle` | `0xa9684` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (13) to shared worker 0x4c284 |
| `DllBattle` | `0xa9698` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (7) to shared worker 0x4c284 |
| `DllBattle` | `0xa97fc` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (5) to shared worker 0x4c284 |
| `DllBattle` | `0xa9de8` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (3) to shared worker 0x4c284 |
| `DllBattle` | `0xaa048` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (11) to shared worker 0x4c284 |
| `DllBattle` | `0xaa120` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (10) to shared worker 0x4c284 |
| `DllBattle` | `0xaa134` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (6) to shared worker 0x4c284 |
| `DllBattle` | `0xaa148` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (0) to shared worker 0x4c284 |
| `DllBattle` | `0xaa1f4` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (17) to shared worker 0x4c284 |
| `DllBattle` | `0xaa46c` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (12) to shared worker 0x4c284 |
| `DllBattle` | `0xaa480` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (2) to shared worker 0x4c284 |
| `DllBattle` | `0xaa510` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (9) to shared worker 0x4c284 |
| `DllBattle` | `0xaa550` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (4) to shared worker 0x4c284 |
| `DllBattle` | `0xaa564` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (1) to shared worker 0x4c284 |
| `DllBattle` | `0xaa668` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (14) to shared worker 0x4c284 |
| `DllBattle` | `0xaa948` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (15) to shared worker 0x4c284 |
| `DllBattle` | `0xaa95c` (0x14, 5/5) | natural | void wrapper forwarding 4 int args plus a constant fifth stack arg (8) to shared worker 0x4c284 |
| `DllBattle` | `0xc7580` (0xc, 3/3) | natural | getter: returns the u8 at +72 of the object pointed to by this->+16 (ldr; ldrb). Sibling of 0x4b8d4. |
| `DllField` | `0x4a49c` (0x1c, masked match (all words, b/bl masked)) | natural | Calls an import veneer (this, 2, 34, this->byte@10) and returns 1. |
| `DllField` | `0x58b88` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x59c98` (0x1c, masked match (all words, b/bl masked)) | natural | Calls H(G(a), b[1]) -- second arg is loaded from the second param before the first call -- and returns 0. |
| `DllField` | `0x5aa14` (0x1c, masked match (all words, b/bl masked)) | natural | Getter: G(a)->+12->+28 chain, then reads the u16 at +292 (sub-struct at +288, field at +4). |
| `DllField` | `0x5b29c` (0x1c, masked match (all words, b/bl masked)) | natural | Getter: G(a)->+12->+28, then ptr@+384, returns signed byte ptr[1]. |
| `DllField` | `0x5c470` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +240, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x5c4bc` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +20, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x5c9dc` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +240, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x5d898` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +240, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x5d9cc` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +240, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x5eb24` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +36, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x5f504` (0x1c, masked match (all words, b/bl masked)) | natural | Loads t=a->@68 first, calls G(a), then H(t->@144) and returns 0. |
| `DllField` | `0x91ddc` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +44 -> +20, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x92318` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +64, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x945d4` (0x10, 4/4) | natural | Forwards this to an import veneer (result discarded) and returns 0. |
| `DllField` | `0x95454` (0x14, 5/5) | natural | Calls two no-arg import veneers in sequence and returns 0. |
| `DllField` | `0x9619c` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +316, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x97ce0` (0x10, 4/4) | natural | Forwards this to an import veneer (result discarded) and returns 1. |
| `DllField` | `0x98c4c` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +52, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x98cd0` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +56 -> +4 and returns the loaded word. |
| `DllField` | `0x9921c` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +52, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x995e4` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +52, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9a04c` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +52 -> +1596 and returns the loaded word. |
| `DllField` | `0x9ad74` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +56 -> +8 and returns the loaded word. |
| `DllField` | `0x9adc4` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +56 -> +0 and returns the loaded word. |
| `DllField` | `0x9bb0c` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +408, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9bb4c` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +408, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9bccc` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +408, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9be30` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +408, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9c088` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +408, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9c2ec` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +408, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9c30c` (0x20, 8/8) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +408, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9d66c` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +20, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9d9d4` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9e344` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9e594` (0x10, 4/4) | natural | Calls a no-arg import veneer (result discarded) and returns 0. |
| `DllField` | `0x9e708` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9e904` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9ea4c` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9ea64` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9ea7c` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100 -> +2712 and returns the loaded word. |
| `DllField` | `0x9ecd8` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9ecf0` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9efe0` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, passes the loaded value to a void helper and returns 0. |
| `DllField` | `0x9f170` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9f1b0` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0x9f1c8` (0x18, 6/6) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100, and tail-calls a helper with the loaded value (`pop {r4,lr}; b`). |
| `DllField` | `0xa50c4` (0x14, 5/5) | natural | Passes the pointer field at this+344 to an import veneer and returns 0. |

**Families and leads from this batch**
- **DllField `GetMgr()` accessor chains (38 matches, `armcc-r2-f6`):** `p = GetMgr();` (`bl 0x16c0`) followed by a chain of `*(int*)(...)` loads, one per `ldr` (`push {r4,lr}; bl 0x16c0; ldr r0,[r0,#12]; ...`), in three tail shapes — 14 tail-call thunks (`Tail(chain)`, `pop {r4,lr}; b`: chains `12,28` / `12,64` / `12,28,52` / `12,72,408` / `12,28,20` / `12,100`), 5 return-chain getters (`pop {r4,pc}`: `0x98cd0`, `0x9a04c`, `0x9ad74`, `0x9adc4`, `0x9ea7c`) and 19 void-call-then-`return 0` (`Sink(chain); return 0;`). Chains with `ldrb`/`ldrh`/other loads or extra args were not tried; many other `0x16c0` callers (e.g. `0x5de80`, `0x92d6c`, `0x94824`) have a different shape; none called `0x6d64c` with this shape.
- **DllBattle forwarding family (28 matches, `armcc-r2-b9`):** `push {r3,lr}; mov ip,#N; str ip,[sp]; bl worker; pop {r3,pc}` — `void fn(int a,int b,int c,int d){ fn_X(a,b,c,d,N); }`: worker `0x4c5e0` (`0xa6a50` N=4, `0xa6d3c` 1, `0xa7584` 0, `0xa782c` 2, `0xa7d34` 3, `0xa7e94` 4, `0xa82a4` 1, `0xa8e48` 0, `0xa91d8` 2, `0xa95ac` 3) and worker `0x4c284` (`0xa958c` 16, `0xa9684` 13, `0xa9698` 7, `0xa97fc` 5, `0xa9de8` 3, `0xaa048` 11, `0xaa120` 10, `0xaa134` 6, `0xaa148` 0, `0xaa1f4` 17, `0xaa46c` 12, `0xaa480` 2, `0xaa510` 9, `0xaa550` 4, `0xaa564` 1, `0xaa668` 14, `0xaa948` 15, `0xaa95c` 8). Not tried: `0xa9e08` (calls a local function), the this-adjusting thunks `0xc9c4c`/`0xc9c60`/`0xc9c74`/`0xc9c88`/`0xc9c9c`, `0xc8f80` (`uint16 f(o){ return g(o->x_0xc); }`), the ctor stages `0xc6d24`/`0xc6d38` (base ctor, then `this->[0x48] = this`).
- **DllBattle comparison wrappers (`armcc-r2-b8`):** `0x78c70` (`fn(0x100) == *p`), `0x78c94` (`<`, unsigned, `movcc`/`movcs`), `0x78668` (`>`, `movhi`/`movls`), `0x79624` (`!=`). ~128 candidates (push, 0x14-0x60 bytes) remain in `0x60000`-`0x90000`; small ones not yet tried: `0x71a2c`, `0x89aac`, `0x7b234`.
- **DllBattle push-less leaf scan (`armcc-r2-b10`):** 884 chunks, ~590 are `<pool junk>; mov r1,#N; str r1,[r0]; ldr r0,[pc]; bx lr` singleton stubs near `0xa4xxx`-`0xa8xxx` (not tried); untried families are listed in `attempted.md` (float vec3 setters, vec3/8-byte copies, `==N` boolean getters, conditional-store chains, indexed `(i&1)` getters, `ldrsb` getters, orr-shift setters, `vldr/vstmia` pair copies).
- **DllBattle `0x12fcc`** (0x40, 16/16, **trick(ordering-local)**): size must include the trailing pool word (0x3c fails by one word).
- **DllField `0x5aa14`** needs a sub-struct at +288 `{int; u16}` — a flat +292 offset makes armcc emit `add #256` plus `ldrh #36`. **Bit-pack builders** `0x797b0` / `0x704ac` need named mask locals `m1`, `m2` plus a separate `v |= tag` statement (retail loads both mask constants before the shifts); tagged `trick(redundant-local)`.
- **DllField open:** ~70 candidates at 0x1c-0x2c bytes in `0x40000`-`0x90000`; not yet tried: `0x4a478`, `0x543b4`, `0x547a8`, `0x552b0`, `0x552d0`, `0x552f0`, `0x55310`, `0x58ba0` (`0x4368c`/`0x44f6c` are `push {r4,lr}; bl; pop {r4,lr}; b` tail-call shapes, skipped).

## 2026-10-16 round a — batch 4 (DllBattle 0x30000 range, DllField 0x90000+)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x3f444` (0x28, 10/10) | natural | teardown stage: if member at +0x840 non-null, fnB(fnA(member)); then clear member |
| `DllBattle` | `0x4af00` (0x30, 12/12) | natural | range-check assert (x>=2 -> fnA(0,0,0)) then return x==0 |
| `DllBattle` | `0x4c948` (0x24, 9/9) | natural | tail-call fnB(this, fnA(this+0x437c)) |
| `DllBattle` | `0x4d2f4` (0x24, 9/9) | natural | r = fnB(fnA(this,14), p); returns r, or 1 if r==0 |
| `DllField` | `0xc2b64` (0x20, 8/8) | natural | this->[152] non-null && import() != 0 bool getter (null returns the 0 loaded) |
| `DllField` | `0xc74c4` (0x28, 10/10) | natural | if import check(this) true: clear words +164/+168, return byte +188; else 0 |
| `DllField` | `0xcac98` (0x28, 10/10) | natural | sibling of 0xc74c4 -- if import check(this) true: clear words +164/+168, return byte +188; else 0 |
| `DllField` | `0xe1208` (0x1c, 7/7) | natural | calls a void import, then returns (import() == 3) |
| `DllField` | `0xe4588` (0x20, 8/8) | natural | returns (import(this->[8]->[20]) < 6) as an unsigned compare |

Notes: DllBattle `0x30000`-`0x60000` has ~55 more candidates (0x14-0x60 bytes); the six attempts below are logged in `attempted.md`. DllField `0x90000`+ skipped `0xca330`/`0xca494` (save r4-r8, body looks like the `0xc74c4` shape) and `0xe2160` (zero lands in r2 instead of r1). **Two high-value unmatched families:** `DllField@0xde620` (~11 byte-identical sites) and `DllField@0xb1bb8` (24 byte-identical sites) — see `attempted.md`.

## 2026-10-16 round a — batch 5 (DllField 0x0-0x40000 families)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllField` | `0x5740` (0x28, 10/10) | natural | wrapper Imp(this,0,0,0,1,0) with 2 stack args |
| `DllField` | `0x5784` (0x28, 10/10) | natural | wrapper Imp(this,0,0,0,1,0) with 2 stack args |
| `DllField` | `0x9acc` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x12b84` (0x34, 13/13) | natural | Imp(0x6002f,0,-1,0); this->o@32 byte89=1; this[37]=1 |
| `DllField` | `0x130e0` (0x2c, 11/11) | natural | if (this->p@0x7c && id==20) p->vt[0](p); return 1 |
| `DllField` | `0x1980c` (0x28, 10/10) | natural | constructor: base ctor call, vtable install, field clears, returns this |
| `DllField` | `0x1acd4` (0x28, 10/10) | natural | calls child(this+0x2c) virtual slot 16 with v, then stores byte v at this+200 |
| `DllField` | `0x243d4` (0x2c, 10/10) | natural | constructor: base ctor call, vtable install, field clears, returns this |
| `DllField` | `0x2aa40` (0x1c, 7/7) | natural | Imp(8); this[32]=0; returns that zero (char return of assignment) |
| `DllField` | `0x2aee0` (0x1c, 7/7) | natural | Imp(8); this[32]=0; returns that zero (char return of assignment) |
| `DllField` | `0x2b0d4` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x2b380` (0x2c, 11/11) | natural | if (this->p@32) { p->vt[1](p); this->p = 0; } |
| `DllField` | `0x2b8d0` (0x30, 12/12) | natural | if (Chk(h=this[24])) Act(h,1,1) tail call |
| `DllField` | `0x31b40` (0x2c, 11/11) | natural | if (this->p@48) { p->vt[1](p); this->p = 0; } |
| `DllField` | `0x320e8` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x33ef4` (0x2c, 11/11) | natural | if (this->p@48) { p->vt[1](p); this->p = 0; } |
| `DllField` | `0x343d0` (0x1c, 7/7) | natural | Imp(8); this[32]=0; returns that zero (char return of assignment) |
| `DllField` | `0x36bfc` (0x2c, 11/11) | natural | if (this->p@48) { p->vt[1](p); this->p = 0; } |
| `DllField` | `0x37054` (0x2c, 11/11) | **trick(redundant-local)** | constructor: base ctor call, vtable install, field clears, returns this |
| `DllField` | `0x37080` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x3728c` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x387d8` (0x2c, 11/11) | natural | if (this->p@40) { p->vt[1](p); this->p = 0; } |
| `DllField` | `0x38a80` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x397b0` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x39864` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x399e8` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x3c474` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x3c53c` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |
| `DllField` | `0x3f034` (0x2c, 11/11) | natural | if (this->p@40) { p->vt[6](p); this->p = 0; } |
| `DllField` | `0x3fba0` (0x10, 4/4) | natural | two-stage call chain B(A(this)) (A result forwarded as tail-call arg) |

**Families from this batch (`armcc-r2-f8`, DllField `0x00000`-`0x40000`):** two-stage chain `B(A(this))` at 0x10 bytes (12 sites: `0x9acc`, `0x2b0d4`, `0x320e8`, `0x37080`, `0x3728c`, `0x38a80`, `0x397b0`, `0x39864`, `0x399e8`, `0x3c474`, `0x3c53c`, `0x3fba0`); `Imp(8); return this[32] = 0;` with a `char` return (`0x2aa40`, `0x2aee0`, `0x343d0` — these were thought to be the zero-register residue but match with the `char` return); release helper `if (this->p) { p->vt[slot](p); this->p = 0; }` (`0x2b380`, `0x31b40`, `0x33ef4`, `0x36bfc`, `0x387d8` slot 1; `0x3f034` slot 6); base constructors `0x1980c` (0x28), `0x243d4` (0x2c) and `0x37054` (0x2c, **trick(redundant-local)**: a shared `char z = 0` makes armcc load the vtable into r2 and the zero into r1); wrappers `0x5740`/`0x5784` (`Imp(this,0,0,0,1,0)`, two stack args), `0x1acd4` (virtual call through slot 16 of the child at +0x2c, then a byte store), `0x130e0` (event handler, `vt[0]` when id == 20, returns 1), `0x12b84` (state step; the pool word is the constant `0x6002f`, not an address), `0x2b8d0` (`if (Chk(h)) Act(h,1,1)`). The scan script misreads the 0x10 chain stubs; use it only as a first pass.
- **Unmatched / not counted:** DllField `0x37714`, `0x37760` (dtor shape; retail loads the vtable into r0 after `mov r4,r0`, every form hoists the `ldr` above the `push`, best 5/9), `0x32988` (8/11; retail loads the member into r0 before the compare), `0x2948c` (branch-vs-conditional-execution), `0x7490`/`0x964c` (contain a `nop` — relaxed-only at best).

## 2026-10-16 round a — batch 6 (DllBattle thunks / ctor stages, all natural)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0xa9e08` (0x14, all words (masked)) | natural | forwarder: calls local function 0xa9e1c with the 4 incoming args plus a constant 5th stack arg 1. |
| `DllBattle` | `0xc6d24` (0x14, all words (masked)) | natural | ctor stage: call base ctor, then this->[0x48] = this (self pointer). |
| `DllBattle` | `0xc6d38` (0x14, all words (masked)) | natural | ctor stage: call a different base ctor, then this->[0x48] = this. |
| `DllBattle` | `0xc8f80` (0x14, all words (masked)) | natural | return (uint16)g(this->+0xc). |
| `DllBattle` | `0xc9c48` (0x14, all words (masked)) | natural | deleting-dtor thunk for a secondary base: this-=0xdc, call the base dtor, then tail-call delete on the returned pointer. |
| `DllBattle` | `0xc9c5c` (0x14, all words (masked)) | natural | deleting-dtor thunk for a secondary base: this-=0xdc, call the base dtor, then tail-call delete on the returned pointer. |
| `DllBattle` | `0xc9c70` (0x14, all words (masked)) | natural | deleting-dtor thunk for a secondary base: this-=0xdc, call the base dtor, then tail-call delete on the returned pointer. |
| `DllBattle` | `0xc9c84` (0x14, all words (masked)) | natural | deleting-dtor thunk for a secondary base: this-=0xe8, call the base dtor, then tail-call delete on the returned pointer. |
| `DllBattle` | `0xc9c98` (0x14, all words (masked)) | natural | deleting-dtor thunk for a secondary base: this-=0xe8, call the base dtor, then tail-call delete on the returned pointer. |
| `DllBattle` | `0xc9cac` (0x14, all words (masked)) | natural | deleting-dtor thunk for a secondary base: this-=0xe8, call the base dtor, then tail-call delete on the returned pointer. |

Note: the this-adjusting dtor thunks start at `0xc9c48`, `0xc9c5c`, `0xc9c70` (this-0xdc), `0xc9c84`, `0xc9c98`, `0xc9cac` (this-0xe8) — the `sub r0,#N` is the first instruction of each thunk (the earlier lead offsets were +4 each); each is `del(dtor(p - N))`. `0xa9e08` is a sibling of the 28 forwarders but passes a constant stack arg 1 to the local function `0xa9e1c`. `0xc8f80` is `uint16 f(o){ return g(o->x_0xc); }`; `0xc6d24`/`0xc6d38` are ctor stages (base ctor, then `this[0x48] = this`). The tail of the module past `0xc0000` was not scanned further.

## 2026-10-16 round a — batch 7 (DllBattle 0x8b2e8 family, DllField state handlers)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x8b2e8` (0x28, 10/10) | natural | if member at this+232 is non-null: fa(member) then tail-call fb(arg2); else return |
| `DllBattle` | `0x8bcb8` (0x28, 10/10) | natural | if member at this+236 is non-null: fa(member) then tail-call fb(arg2); else return |
| `DllBattle` | `0x8db3c` (0x28, 10/10) | natural | if member at this+216 is non-null: fa(member) then tail-call fb(arg2); else return |
| `DllBattle` | `0x8e858` (0x28, 10/10) | natural | if member at this+232 is non-null: fa(member) then tail-call fb(arg2); else return |
| `DllBattle` | `0x8f210` (0x28, 10/10) | natural | if member at this+224 is non-null: fa(member) then tail-call fb(arg2); else return |
| `DllBattle` | `0x8fa38` (0x28, 10/10) | natural | if member at this+224 is non-null: fa(member) then tail-call fb(arg2); else return |
| `DllField` | `0xc9060` (0x30, 12/12) | natural | Calls two imports (second returns a flag); if the flag is 0 and the pointer at this+68 is non-null, tail-calls an import on that pointer, else returns. Identical bytes to 0xca54c. |
| `DllField` | `0xca54c` (0x30, 12/12) | natural | Calls two imports (second returns a flag); if the flag is 0 and the pointer at this+68 is non-null, tail-calls an import on that pointer, else returns. Identical bytes to 0xc9060. |
| `DllField` | `0xcbc60` (0x3c, 15/15) | natural | State-handler step: calls an import on this->[68] (arg 1), then reads the state id of the object at this->[68]+184; if not 103 returns 36, else calls an import on this and returns 1. |
| `DllField` | `0xcc474` (0x58, 22/22) | natural | State-handler step: calls an import on this->[68] (arg 1); if byte +145 is set and the state id at this->[68]+184 is 104, sets bit 0x80000 in flags +168, clears +148 and returns 1; otherwise returns 38. |

Notes: **DllBattle `0x8b2e8` family** (0x28 incl. the trailing `pop {r4,pc}`; a 0x24 compile shows 9/9 and misses that last word): `if (this->member@N) { fa(member); } return fb(arg2)`-style member-gated tail call; N = +232 (`0x8b2e8`, `0x8e858`), +236 (`0x8bcb8`), +216 (`0x8db3c`), +224 (`0x8f210`, `0x8fa38`). **Same-bytes siblings not yet compiled** (member offset in brackets; same source with a different `pad[]` should match): `0x90590` [224], `0x910d4` [244], `0x92874` [228], `0x935cc` [228], `0x949f0` [228], `0x954a0` [244], `0x96744` [244], `0x97960` [216], `0x98a04` [232], `0x993b8` [232], `0x99ff4` [220], `0x9a9b0` [280], `0x9c5dc` [224]. **DllField `0xc9060`/`0xca54c`** (0x30): two imports, if the second returns 0 and `this->[68]` is non-null tail-calls an import on it. `0xcbc60` / `0xcc474`: state-handler steps (id 103 / 104). The "getter via 0x5dc68/0x707d4" cluster (DllBattle `0x7b20c`, `0x835f8`, `0x7b2e4`, `0x7bec4`, `0x81544`, `0x81568`, `0x81594`, tail forms `0x7a11c`, `0x7b320`, `0x78488`, `0x7957c`) failed — see `attempted.md`. DllField `0xcf000`-`0xd0000` and `0xe1c00`-`0xe2000` tiers look untried; ~374 candidates at 0x2c-0x70 bytes remain above `0x90000`.

## 2026-10-16 round a — batch 8 (DllField call veneers, DllBattle misc)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x3342c` (0x2c, 11/11) | natural | ctor stage: base ctor, install vtable, zero four ints at +56..+68, return this |
| `DllBattle` | `0x36d64` (0x10, 4/4) | natural | chained call `fnB(fnA(o))`, second call a tail call |
| `DllBattle` | `0x38678` (0x1c, 7/7) | natural | `b = a ? a->b : fnA(a); return b->v` with a = o->+12 |
| `DllBattle` | `0x4c124` (0x1c, 7/7) | natural | tail-call wrapper `return fnB(fnA(o, 14), p)` |
| `DllBattle` | `0x4c374` (0x3c, 15/15) | natural | range-checked (i<11) getter `o->p->v[i] != 0` with assert stub on out-of-range |
| `DllField` | `0x543b4` (0x18, 6/6) | natural | calls import veneer (*arg0, 16) and returns 3 |
| `DllField` | `0x543d4` (0x1c, 7/7) | natural | calls import veneer (*arg0, 8, 1) and returns 3 |
| `DllField` | `0x5449c` (0x1c, 7/7) | natural | calls import veneer (this, 2, 21, this->byte@10) and returns 1 |
| `DllField` | `0x544c0` (0x1c, 7/7) | natural | calls import veneer (this, 4, 19, this->byte@10) and returns 1 |
| `DllField` | `0x544e4` (0x1c, 7/7) | natural | calls import veneer (this, 8, 15, this->byte@10) and returns 1 |
| `DllField` | `0x547a8` (0x18, 6/6) | natural | calls import veneer (arg1->field@16, arg2) and returns 1 |
| `DllField` | `0x54de4` (0x1c, 7/7) | natural | calls import veneer (*arg0, 8, 0) and returns 3 |
| `DllField` | `0x54e48` (0x1c, 7/7) | natural | calls import veneer (this, 16, 10, this->byte@10) and returns 1 |
| `DllField` | `0x54e6c` (0x1c, 7/7) | natural | calls import veneer (this, 32, 5, this->byte@10) and returns 1 |
| `DllField` | `0x552b0` (0x18, 6/6) | natural | calls import veneer (this, this->byte@10, 12) and returns 1 |
| `DllField` | `0x552d0` (0x18, 6/6) | natural | calls import veneer (this, this->byte@10, 14) and returns 1 |
| `DllField` | `0x552f0` (0x18, 6/6) | natural | calls import veneer (this, this->byte@10, 16) and returns 1 |
| `DllField` | `0x55310` (0x18, 6/6) | natural | calls import veneer (this, this->byte@10, 32) and returns 1 |

Notes: **DllField call-veneer families (`armcc-r3-f1`):** `(this, this->byte@10, 12/14/16/32)` returns 1 (`0x552b0`/`0x552d0`/`0x552f0`/`0x55310`); `(this, a, b, this->byte@10)` returns 1 with literals (2,21), (4,19), (8,15), (16,10), (32,5) (`0x5449c`, `0x544c0`, `0x544e4`, `0x54e48`, `0x54e6c` — literal-only siblings of `0x4a49c`); `(*arg0, 16)`/`(*arg0, 8, 1)`/`(*arg0, 8, 0)` returning 3 (`0x543b4`/`0x543d4`/`0x54de4`); `0x547a8` (`arg0` unused). **DllBattle `0x4c124`** (`return fnB(fnA(o, 14), p)`), **`0x36d64`** (0x10; the trailing `b` belongs to the next function), **`0x4c374`** (range-checked getter, write the in-range case first — the `if (i >= 11)` form flips the branch and block order), **`0x3342c`** (0x2c incl. the vtable pool word: base ctor, vtable, zero four ints at +56..+68), **`0x38678`**. DllBattle `0x30f1c` and `0x4b82c` are switch prologues; most remaining `0x30000`-`0x60000` candidates are switch / jump-table functions. DllField dtor-style siblings `0x4a458`, `0x530ec`, `0x4009c` look residue-prone.

## 2026-10-16 round a — batch 9 (DllField GetMgr caller shapes x46)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllField` | `0x58d2c` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +13328, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x591a0` (0x10, 4/4) | natural | Calls the shared manager accessor (0x16c0), discards the result, returns 0. |
| `DllField` | `0x593f0` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +76200, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x597b0` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +13328, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x59db8` (0x30, 12/12) | natural | Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0, adds a field offset and calls a helper with (address, a[1]), returning 0. |
| `DllField` | `0x59de8` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +13328, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x59e0c` (0x30, 12/12) | natural | Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0, adds a field offset and calls a helper with (address, a[1]), returning 0. |
| `DllField` | `0x59e3c` (0x30, 12/12) | natural | Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0, adds a field offset and calls a helper with (address, a[1]), returning 0. |
| `DllField` | `0x5a650` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +3816, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x5a674` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +3816, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x5b5bc` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +83012, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x5c2c8` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +101552, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x5e650` (0x28, 10/10) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +240 -> +2740 -> +420, adds a field offset to the loaded pointer and passes the address to a void helper, returning 0. |
| `DllField` | `0x5eef4` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +404; if the loaded pointer is non-null calls a helper with it (conditional `blne`), returns 0. |
| `DllField` | `0x9170c` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +404; if the loaded pointer is non-null calls a helper with it (conditional `blne`), returns 0. |
| `DllField` | `0x9257c` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +404; if the loaded pointer is non-null calls a helper with it (conditional `blne`), returns 0. |
| `DllField` | `0x92d6c` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), loads its +12 sub-object and calls a helper with a constant bool flag argument, returning 0. |
| `DllField` | `0x93d3c` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +404; if the loaded pointer is non-null calls a helper with it (conditional `blne`), returns 0. |
| `DllField` | `0x97a4c` (0x28, 10/10) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +85200, adds a constant field offset and passes the resulting address to a void helper, returning 0. |
| `DllField` | `0x97ec0` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +90024, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x9876c` (0x28, 10/10) | natural | Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +36 and calls a helper with (loaded object, a[1]), returning 0. |
| `DllField` | `0x98d14` (0x28, 10/10) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +85200, adds a constant field offset and passes the resulting address to a void helper, returning 0. |
| `DllField` | `0x98df8` (0x30, 12/12) | natural | Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0, adds a field offset and calls a helper with (address, a[1]), returning 0. |
| `DllField` | `0x98e28` (0x28, 10/10) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +85200, adds a constant field offset and passes the resulting address to a void helper, returning 0. |
| `DllField` | `0x98ef8` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), loads its +12 sub-object and calls a helper with a constant bool flag argument, returning 0. |
| `DllField` | `0x98f14` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), loads its +12 sub-object and calls a helper with a constant bool flag argument, returning 0. |
| `DllField` | `0x99238` (0x28, 10/10) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +4184, adds a constant field offset and passes the resulting address to a void helper, returning 0. |
| `DllField` | `0x99260` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +4184, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x99284` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +4184, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x993a4` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), loads its +12 sub-object and calls a helper with a constant bool flag argument, returning 0. |
| `DllField` | `0x993c0` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), loads its +12 sub-object and calls a helper with a constant bool flag argument, returning 0. |
| `DllField` | `0x99440` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +76200, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x99464` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +76200, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x9a7a8` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +4184, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x9a9f0` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +85200, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x9b98c` (0x28, 10/10) | natural | Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +408 and calls a helper with (loaded object, a[1]), returning 0. |
| `DllField` | `0x9dcdc` (0x28, 10/10) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +101552, adds a constant field offset and passes the resulting address to a void helper, returning 0. |
| `DllField` | `0x9dd04` (0x28, 10/10) | natural | Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +248 and calls a helper with (loaded object, a[1]), returning 0. |
| `DllField` | `0x9dd2c` (0x24, 9/9) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +101552, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`). |
| `DllField` | `0x9de18` (0x28, 10/10) | natural | Reads the event argument word a[1] first, gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +248 and calls a helper with (loaded object, a[1]), returning 0. |
| `DllField` | `0x9e724` (0x28, 10/10) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +101048, adds a constant field offset and passes the resulting address to a void helper, returning 0. |
| `DllField` | `0x9e87c` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100 -> +2761, adds a field offset and returns the signed byte (`ldrsb`) found there. |
| `DllField` | `0x9ea30` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100 -> +2916, adds a field offset and returns the signed byte (`ldrsb`) found there. |
| `DllField` | `0x9f1e0` (0x1c, 7/7) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +100 -> +2084, adds a field offset and returns the signed byte (`ldrsb`) found there. |
| `DllField` | `0x9feb4` (0x28, 10/10) | natural | Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +72 -> +240 -> +2740 -> +724, adds a field offset to the loaded pointer and passes the address to a void helper, returning 0. |
| `DllField` | `0xbbb54` (0x10, 4/4) | natural | Deleting-destructor thunk: calls 0x6d64c (dtor body, returns this) then tail-calls free helper 0x1c8. |

Notes — **DllField GetMgr (`0x16c0`) caller scan (`armcc-r3-f2`):** 626 `bl 0x16c0` callers were outside `matched/` at scan time; only callers up to 0x38 bytes were shaped, so ~470 larger callers are untouched. Shapes matched: tail-call with constant add (`Tail(**(chain) + C)`, 16 fns, 0x24), void-call with constant add + `return 0` (7 fns), call with a flag arg `mov r1,#0/#1` (5 fns, 0x1c), null-check `blne` (4 fns, 0x24), `ldrsb` signed byte getters (3 fns), event arg `a[1]` loaded before `GetMgr` (8 fns, 0x28/0x30), bare `GetMgr()` + `return 0` (`0x591a0`), and `0xbbb54` (`Free(Dtor(t))`, `Dtor` = `0x6d64c`, the only `bl 0x6d64c` caller; `Free` = `0x1c8`). Some saved files carry an unused `extern` (e.g. `Sink`) — harmless.
Not matched (see `attempted.md`): nested `0x865cc` call + conditional tail (`0x59270` and five siblings), `ldrd`/`uxth` arg pairs, jump-table dispatchers on `ldrb [..,#460]` (`0x94d48`, `0x957fc`, `0x930ac`; need `--mask-ranges`), float getters `0x92330`/`0x92370`, struct-by-stack arg `0x5ef18`, `self`-in-r4 multi-call functions (`0x945e4`, `0x95190`, `0x9752c`, `0x91330`, `0x91364`).

## 2026-10-16 round a — batch 10 (DllBattle push-less families x21)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x12704` (0x14, 5/5) | natural | vec3 setter: this->+4 = (x,y,z) float triple by value; this->+0x4c (u8) = 1 (dirty flag). |
| `DllBattle` | `0x12a30` (0x14, 5/5) | natural | vec3 setter: this->+52 = (x,y,z) float triple by value; this->+0x4c (u8) = 1 (dirty flag). |
| `DllBattle` | `0x12af8` (0x14, 5/5) | natural | vec3 setter: this->+16 = (x,y,z) float triple by value; this->+0x4c (u8) = 1 (dirty flag). |
| `DllBattle` | `0x12b24` (0x14, 5/5) | natural | vec3 setter: this->+28 = (x,y,z) float triple by value; this->+0x4c (u8) = 1 (dirty flag). |
| `DllBattle` | `0x29abc` (0x14, 5/5) | natural | splat setter: this->+0x40/+0x44/+0x48 = v (float); this->+0x4c (u8) = flag (3rd arg). |
| `DllBattle` | `0x2b9dc` (0x18, 6/6) | **trick(redundant-local)** | writes constant pair (-94.0f, -54.0f) to arg2->{x,y}; arg1 unused (default position/offset reset). |
| `DllBattle` | `0x3357c` (0x14, 5/5) | natural | splat setter: this->+0x40/+0x44/+0x48 = v (float); this->+0x4c (u8) = flag (3rd arg). |
| `DllBattle` | `0x3c10c` (0x18, 6/6) | natural | copy 8 bytes from arg3->+164 (two ints) into this->+0x730; this->+0x72c (u8) = arg4. |
| `DllBattle` | `0x3c124` (0x18, 6/6) | natural | copy 8 bytes from arg3->+172 (two ints) into this->+0x730; this->+0x72c (u8) = arg4. |
| `DllBattle` | `0x3c13c` (0x18, 6/6) | natural | copy 8 bytes from arg3->+180 (two ints) into this->+0x730; this->+0x72c (u8) = arg4. |
| `DllBattle` | `0x3c154` (0x18, 6/6) | natural | copy 8 bytes from arg3->+188 (two ints) into this->+0x730; this->+0x72c (u8) = arg4. |
| `DllBattle` | `0x3c16c` (0x18, 6/6) | natural | copy 8 bytes from arg3->+196 (two ints) into this->+0x730; this->+0x72c (u8) = arg4. |
| `DllBattle` | `0x3c184` (0x18, 6/6) | natural | copy 8 bytes from arg3->+204 (two ints) into this->+0x730; this->+0x72c (u8) = arg4. |
| `DllBattle` | `0x3d21c` (0x14, 5/5) | natural | boolean getter: returns this->+7 (u8) != 1. |
| `DllBattle` | `0x762f4` (0x14, 5/5) | natural | boolean getter: returns this->+0x694 (u8) == 3. |
| `DllBattle` | `0x81638` (0x14, 5/5) | natural | boolean getter: returns this->+0x10 (u8) == 6. |
| `DllBattle` | `0xc5dd8` (0x14, 5/5) | natural | splat setter: this->+0x40/+0x44/+0x48 = v (float); this->+0x4c (u8) = flag (3rd arg). |
| `DllBattle` | `0xc7304` (0x10, 4/4) | natural | indexed getter: returns this->+0xb18[arg & 1] (int array). |
| `DllBattle` | `0xc73f4` (0x10, 4/4) | natural | indexed getter: returns this->+0xb1c[arg & 1] (int array). |
| `DllBattle` | `0xc8fec` (0xc, 3/3) | natural | signed byte getter: returns (signed char) this->+0x874. |
| `DllBattle` | `0xc900c` (0xc, 3/3) | natural | signed byte getter: returns (signed char) this->+0x875. |

Notes — **DllBattle push-less leaf families (`armcc-r3-b4`):** float splat setters `x=y=z=v`, `+0x4c (u8) = flag` (`0x29abc`, `0x3357c`, `0xc5dd8`); vec3-by-value setters with `+0x4c = 1` (`0x12704` +4, `0x12af8` +16, `0x12b24` +28, `0x12a30` +52); 8-byte struct copy from `arg3->+N` into `this->+0x730`, `+0x72c (u8) = arg4` (`0x3c10c`..`0x3c184`, N = 164..204 step 8); `==N` boolean getters (`0x762f4` `+0x694 == 3`, `0x81638` `+0x10 == 6`, `0x3d21c` `+7 != 1` — matches only as `if (v == 1) return 0; return 1;`, a spelling choice); `o->v[i & 1]` indexed getters (`0xc7304`, `0xc73f4`); `ldrsb` getters (`0xc8fec`, `0xc900c`); `0x2b9dc` writes (-94.0f, -54.0f) to `arg2->{x,y}` (**trick(redundant-local)**: named locals `a`, `b` get armcc to emit one `vstmia` instead of two `vstr`).
Not matched: conditional-store chain (`0x41f94`, `0x434ec`, `0x439d4`, `0x3c960`, `0x43958`; best 5/8), ldm/strd vec3 copies (`0x126ec`, `0x12864`, `0x128d4`, `0x12a00`, `0x12ae0`, `0x12b0c`; 2/6, field-copy residue). `0x4b9cc`, `0x76ce8`, `0x56a10`, `0x56a38`, `0x569fc`, `0x56a48`, `0x56a58` are tail blocks / case arms of larger functions, not standalone — the enclosing functions are the real candidates.

## 2026-10-16 round a — batch 11 (DllBattle 0x60000 range, 3 tricks)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x74ff8` (0x58, 22/22) | **trick(redundant-local)** | if f18e4(h,0,1) then f18ec(h,0,1,0); f34c(h,0,0,0); return 1 else 0, h = this+0xd38 handle. |
| `DllBattle` | `0x75050` (0x54, 21/21) | **trick(redundant-local)** | three calls on the handle at this+0xd38 (18ec(h,0,0,0); 34c(h,0,0,1); 18f4(h,0,1,1,1)), handle reloaded each call. |
| `DllBattle` | `0x750a4` (0x84, 33/33) | **trick(redundant-local)** | handle (this+0xd38) call pair, then converts an 8-byte {int,int} table entry [i] to a float vec (3-float array) and passes it to a third call. |

Notes — **DllBattle `0x60000`-`0x78000` (`armcc-r3-b1`):** all three are **trick(redundant-local)**: a local `void **pp = &o->fld` reproduces retail's cached field address (using `o->fld` directly gave 2/21 on `0x75050`); `0x750a4` (0x84 incl. the trailing pool word) converts entry [i] of an 8-byte `{int,int}` global table to floats in a `float v[3]` (z unused, lands at sp+4). **Verifier limitation found:** `DllBattle@0x71a2c` matches all 8 code words, but retail has a 0 pad word before the pool constant (function starts at 4 mod 8, pool 8-aligned) which a standalone compile can't reproduce — it would match if `fingerprint_fn.py` tolerated pool-alignment padding (a tool improvement worth making).

## 2026-10-16 round a — batch 12 (DllBattle 0x8b2e8 family siblings x14, all natural)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x90590` (0x28, 10/10) | natural | if member at this+224 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x910d4` (0x28, 10/10) | natural | if member at this+244 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x92874` (0x28, 10/10) | natural | if member at this+228 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x935cc` (0x28, 10/10) | natural | if member at this+228 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x949f0` (0x28, 10/10) | natural | if member at this+228 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x954a0` (0x28, 10/10) | natural | if member at this+244 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x96744` (0x28, 10/10) | natural | if member at this+244 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x97960` (0x28, 10/10) | natural | if member at this+216 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x98a04` (0x28, 10/10) | natural | if member at this+232 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x993b8` (0x28, 10/10) | natural | if member at this+232 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x99ff4` (0x28, 10/10) | natural | if member at this+220 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x9a9b0` (0x28, 10/10) | natural | if member at this+280 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |
| `DllBattle` | `0x9b87c` (0x24, 9/9) | natural | if member at this+224 is non-null, fa(member) then tail-call fb(0); else no-op (void) |
| `DllBattle` | `0x9c5dc` (0x28, 10/10) | natural | if member at this+224 is non-null, fa(member) then tail-call fb(a) with the 2nd arg; else no-op (void) |

Notes — the 13 listed siblings of the `0x8b2e8` family all match (`pad[]` / member offset 216-280, 10/10 at 0x28); `0x9b87c` (0x24, 9/9) is the same shape with no second argument (`if (o->m) { fa(o->m); fb(0); }`, member +224). A scan of the whole DllBattle `.text` for the exact masked pattern found 19 sites, all now matched.

## 2026-10-16 round a — batch 13 (DllBattle comparison-wrapper family x32, all natural)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0xa5db4` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportA(54, 2048) (tail call, no return value) |
| `DllBattle` | `0xa7fd4` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportA(54, 6144) (tail call, no return value) |
| `DllBattle` | `0xa8000` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportB(17, 0) (tail call, no return value) |
| `DllBattle` | `0xa802c` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportA(54, 2048) (tail call, no return value) |
| `DllBattle` | `0xaa524` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xab0d4` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportA(54, 2048) (tail call, no return value) |
| `DllBattle` | `0xab168` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportA(54, 2048) (tail call, no return value) |
| `DllBattle` | `0xab408` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportB(66, 1) (tail call, no return value) |
| `DllBattle` | `0xaf540` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xb2e8c` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(56, 20) (tail call, no return value) |
| `DllBattle` | `0xb4cdc` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(23, 2) (tail call, no return value) |
| `DllBattle` | `0xb5414` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(56, 40) (tail call, no return value) |
| `DllBattle` | `0xb7850` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportA(50, 6144) (tail call, no return value) |
| `DllBattle` | `0xb8340` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(84, 1) (tail call, no return value) |
| `DllBattle` | `0xb8b58` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xb9b44` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xb9cb0` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(80, 0) (tail call, no return value) |
| `DllBattle` | `0xb9d64` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(67, 0) (tail call, no return value) |
| `DllBattle` | `0xb9d90` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xb9de8` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xba544` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(76, 1) (tail call, no return value) |
| `DllBattle` | `0xba7f8` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(67, 1) (tail call, no return value) |
| `DllBattle` | `0xba824` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportB(76, 1) (tail call, no return value) |
| `DllBattle` | `0xbb87c` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xbbccc` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(66, 1) (tail call, no return value) |
| `DllBattle` | `0xbbdf4` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xbd6b4` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportB(17, 0) (tail call, no return value) |
| `DllBattle` | `0xbe4ec` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(66, 1) (tail call, no return value) |
| `DllBattle` | `0xbf8cc` (0x2c, 11/11) | natural | guarded report helper: if (getter(3) == c) reportB(83, 1) (tail call, no return value) |
| `DllBattle` | `0xbfde4` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportA(54, 8192) (tail call, no return value) |
| `DllBattle` | `0xbfed8` (0x2c, 11/11) | natural | guarded report helper: if (getter(2) == c) reportA(54, 2048) (tail call, no return value) |
| `DllBattle` | `0xbff04` (0x2c, 11/11) | natural | guarded report helper: if (getter(4) == c) reportB(83, 0) (tail call, no return value) |

Notes — **DllBattle comparison-wrapper family (32 sites, all size 0x2c, 11/11):** `void fn(int a,int b,int c){ if (g(N) == c) fnX(A, B); }` where `g` is the `0x6b91c` getter (argument N = 2, 3 or 4) and `fnX` is the `0x4ce84` veneer when `B` is `0x800`/`0x1800`/`0x2000`, else `0x658d4`; only `N`, `A`, `B` change. Getter 2: `0xa5db4`, `0xa7fd4`, `0xa8000`, `0xa802c`, `0xaa524`, `0xab0d4`, `0xab168`, `0xab408`, `0xba824`, `0xbd6b4`, `0xbfde4`, `0xbfed8`. Getter 3: `0xaf540`, `0xb2e8c`, `0xb4cdc`, `0xb5414`, `0xb7850`, `0xb8340`, `0xb8b58`, `0xb9b44`, `0xb9cb0`, `0xb9d64`, `0xb9d90`, `0xb9de8`, `0xba544`, `0xba7f8`, `0xbb87c`, `0xbbccc`, `0xbbdf4`, `0xbe4ec`, `0xbf8cc`. Getter 4: `0xbff04`. Of 480 candidates, only this shape was a mechanical family. Leads not attempted: `mov r4,r0; ldr r0,[r0,#off]; bl 0x19f4; mov r0,r4; pop` (`0x902c8`, `0x97688`, `0x99d44`, `0x9b5e8`, `0x9c2f4`; dtor-like), the same getter shape with `cmp r0,#K` against a constant and `mov r0,#22` as the getter argument in `0xa5de0`..`0xa92b0` (`push {r4-r6,lr}`).

## 2026-10-16 round a — batch 14 (DllField 0xd0000-0xe4000 tier x5, all natural)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllField` | `0xd50b8` (0x34, 13/13) | natural | float getter: 0.0f if Lookup(this->[12]) null else GetValue(Lookup(this->[12])) |
| `DllField` | `0xd7964` (0x2c, 11/11) | natural | if this->[184] non-null, ImportB(arg, ImportA(this->[184])) (tail call) |
| `DllField` | `0xd90cc` (0x30, 12/12) | natural | 7-arg wrapper reordering to Import(a0,a6,a5,a4,a3,a1); a2 unused |
| `DllField` | `0xe248c` (0x24, 9/9) | natural | o = arg->[4]; return o->vtable[55](o, 0) != 0 |
| `DllField` | `0xe31f4` (0x30, 12/12) | natural | returns 1 if o=arg->[0] has byte[0x87f] set and Check(o,0x800000) true, else 2 |

Notes — **DllField `0xd000`-`0xe400` tier (`armcc-r4-f3`):** `0xe248c` is 0x24 (a size enumeration said 0x2c): the trailing `mov r0,#12; bx lr` at `0xe24b0` is a separate return-12 stub nobody has claimed. `0xd50b8` size 0x34 includes the 0.0f literal word.

## 2026-10-16 round a — batch 15 (residues cracked: DllField 0xb1bb8 x24 via C++ Reset(); DllField 0x90000-0xd0000 tiers)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x4344c` (0x2c, 11/11) | natural | vt[0x80](this,a) result compared, returns r==103\|\|r==104 (early-return spelling gives retail beq/moveq/beq shape) |
| `DllField` | `0x91364` (0x30, 12/12) | natural | Calls H1(mgr->+12->+72), H2(), H3(mgr->+28, 2); returns 0. |
| `DllField` | `0x916dc` (0x30, 12/12) | natural | If mgr->+28 has a non-null pointer at +148, calls H() and clears the word at (mgr->+28)+124+24; returns 0. |
| `DllField` | `0x92688` (0x2c, 11/11) | natural | Reads args[1], fetches the manager and calls Helper(mgr->+12->+28->+36, args[1], 10); returns 0. |
| `DllField` | `0x9af9c` (0x30, 12/12) | natural | Calls H(q) on the sub-object at mgr->+12->+28->[0] + 0x1aeec, then returns bits 4+ of its u16 field at +4. |
| `DllField` | `0xb0378` (0x3c, 15/15) | natural | boolean query: import(this) nonzero, or this->[0x5a4] non-null and import on it nonzero -> 1, else 0 |
| `DllField` | `0xb1a10` (0x50, 20/20) | natural | dtor stage: install vtable, virtual-delete member +0x430 and free handle +4 (each if set, zeroed), return this |
| `DllField` | `0xb1bb8` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb1ccc` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb231c` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb253c` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb2864` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb29ac` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb2e3c` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb3320` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb3704` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb5a00` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb5ab4` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb7bbc` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb7ec8` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb8180` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb86ec` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb8778` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xb8e28` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xba914` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xbc390` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xbc5dc` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xc0060` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xc0ebc` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xc1f28` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xc2618` (0x2c, 11/11) | **trick(volatile-store)** | secondary-base dtor stage -- store vtable arg, `if (this[32]) Free(this, this+32)`, return Base(this+8)-8 (one of 24 identical sites) |
| `DllField` | `0xc2a7c` (0x44, 17/17) | natural | release helper: this[168] -> fb(fa(x)) + zero; this[156] -> virtual delete (vt slot 1) + zero |
| `DllField` | `0xc2ac0` (0x40, 16/16) | natural | argument-marshalling thunk forwarding seven values (derefs of 4 args, 0, plus a stack arg) to an import |
| `DllField` | `0xc8f88` (0x3c, 15/15) | **trick(ordering-local)** | state-handler step: import(this[100],this[180]) == 0 -> 50; else second import, state[180]=13, returns byte this[189] |
| `DllPuzzle` | `0x1014` (0x1c, 7/7) | natural | if (p->f4->f2c->f60 == 1) p->f8 = 2 leaf (moveq/strbeq) |

**Mechanism cracked — C++ inline-member `Reset()` (DllField `0xb1bb8` family, 24 identical sites, each 11/11, `.cpp` files, `trick(volatile-store)`):** armcc if-converts a call arm (`addne r1,r0,#32; blne`) only when the arm is ≤ 2 instructions. In plain C the null-test load lands in r0 (r0 looks dead once `this` is copied to r4), so the arm needs `mov r0,r4` and armcc emits a `beq` branch instead (4/11). Written as an inline C++ member (`void Reset() { if (m) fr(this, &m); }`) the test load goes to r1 and r0 stays `this`, giving retail's form (8/11); the vptr store through a `volatile` pointer then pins the `mov r4,r0` ahead of it (11/11). Sites: `0xb1bb8`, `0xb1ccc`, `0xb231c`, `0xb253c`, `0xb2864`, `0xb29ac`, `0xb2e3c`, `0xb3320`, `0xb3704`, `0xb5a00`, `0xb5ab4`, `0xb7bbc`, `0xb7ec8`, `0xb8180`, `0xb86ec`, `0xb8778`, `0xb8e28`, `0xba914`, `0xbc390`, `0xbc5dc`, `0xc0060`, `0xc0ebc`, `0xc1f28`, `0xc2618` (a byte-pattern scan of every `.cro` found the pattern only in DllField). **This C++ trick may unlock other branch-vs-conditional-execution cases** (DllField `0xcae50` / `0xcb41c`, DllBattle `0x7198c` / `0x73524`, the `0xb6844` / `0xb76c8` secondary-base dtor pair).
Notes: `DllBattle@0x4344c` matched (natural; `if (r == 103) return 0; return r == 104;`). `DllPuzzle@0x1014` matched (plain C: `if (p->f4->f2c->f60 == 1) p->f8 = 2`; the old `DllPuzzle@0x100c` entry was a mis-offset — 0x100c..0x1013 are two zero words). `DllField@0xde620` (11 sites) is still not cracked (best 7/11; the Reset() trick doesn't carry over). Other DllField batch notes: `0xc8f88` needs the byte read in a local before the state store (else the `ldrb` lands after the `str`, 12/15); `0xb1a10` (0x50) includes the trailing vtable pool word (0x4c is 19/19); `0xcff28`, `0xcaf30`, `0xc2b00`, `0xcc4d8` and ~44 candidates in `0xb0000`-`0xd0000` untried (many are 0x64-byte 4-call functions). `armcc-r4-f1` stuck handlers: `0x93d10`, `0x948f0`, `0x93b30`, `0x94d18`, `0x9aeb8` (arg-load width) and `0x98bc0`, `0x9d8f4`, `0x9a618` (see `attempted.md`); `0x95130` (11/11, sibling of `0x92688`) was dropped at the ceiling — a quick pickup.

## 2026-10-16 round a — batch 16 (small jump-table dispatchers x4)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllBattle` | `0x60e00` (0x74, 29/29 (with --mask-ranges 0x14:0x18)) | natural | switch(k) 6-slot jump table: selectors 1,3,4,5 return 1; selector 2 returns 2\|(((calc(global.f54,3)+2)<<3)&0x1f8); selector 0 / >=6 call err(0,0,0) and return 0. Pool word = address of a global OBJECT (not pointer): needs `extern S g;` (g.f) not `S *g` or armcc emits an extra ldr. |
| `DllField` | `0x11298` (0x6c, 27/27) | natural | event dispatch -- if this->f7c non-null and k in {0x21,0x27,0x3c} call that object's vtable slot 0/1/2 and return 0, else return 1 (compare-chain switch, no mask-ranges). NOTE: task named 0x112a8, which is the mid-function switch; real function start is 0x11298 (push {r4,lr}; ldr r0,[r0,#0x7c]). |
| `DllField` | `0x79dc4` (0x60, 24/24 (with --mask-ranges 0x10:0x40; mapping recovered from relocations via tools/jumptable_relocs.py)) | natural | leaf bool predicate: true for ids 4-7 and 12-19, false for 8-11 / out of range (sub #4; cmp #16 jump table). |
| `DllField` | `0x94d48` (0x6c, 27/27 (with --mask-ranges 0x20:0x24; mapping recovered from relocations via tools/jumptable_relocs.py)) | natural | classifies mode byte inst()->+12->+28->+460: 0 for {0,2,6}, 1 for 8, 2 for {1,3}, err(0,0,0) then 2 for the rest. Note: cases that skip the error call and reach the shared `return 2` are written as `break` with `return 2` after the switch. |

**New tool — `tools/jumptable_relocs.py <Module> <tbl_lo> <tbl_hi>`:** prints the real jump-table targets from the CRO's internal relocations (header `0x128`, 12-byte entries), i.e. the case-value→body mapping that `--mask-ranges` cannot prove. Matches made with `--mask-ranges` carry the exact Verify flags in their header and are labelled *relocation-derived, not compiler-proven* (details in `docs/decomp/armcc-toolchain.md`). `DllField@0x11298` (0x6c, 27/27, **no mask**) is the real start of the function whose mid-function switch is at `0x112a8` — if `this+0x7c` is non-null and k is 0x21/0x27/0x3c it calls that object's vtable slot 0/1/2 and returns 0, else 1 (the earlier `0x112c8`/`0x112dc`/`0x112f0` "functions" are its case bodies). `DllBattle@0x60e00` needs the global declared `extern S g;` (an object, not a pointer). Dead ends (`attempted.md`): `0x4b82c`, `0x6de1c`, `0x71250` / `0x743a8`; `0x30f1c` is too large (~0x180); `DllField@0x12840` is a destructor/constructor pair, not a switch; `bl; nop; nop; b` shapes (`DllBattle@0x4f158`, `0x4f5b0`, `0x52db0`) have the same relaxation-nop problem.

## 2026-10-16 round a — batch 17 (non-tail-call blocker cracked: DllField 0xe1c4c twins)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllField` | `0xe1c4c` (0x2c, 11/11) | **trick(wider-callee-return)** | f11(a->first); return f79(a, a->sub.y /*+17*/, 0, 5) as a NON-tail call returning the callee r0 (twin of 0xe1fe8) |
| `DllField` | `0xe1fe8` (0x2c, 11/11) | **trick(wider-callee-return)** | f11(a->first); return f79(a, a->sub.y /*+17*/, 0, 5) as a NON-tail call returning the callee r0 (twin of 0xe1c4c) |

**`DllField@0xe1c4c` / `0xe1fe8` (0x2c, 11/11, `trick(wider-callee-return)`)** — `f11(a->first); return f79(a, a->sub.y /*+17*/, 0, 5)` as a NON-tail call returning the callee's r0. Mechanism: declare the callee as returning `long long` and have the caller return `(int)` of it — r0 is the low word, so the cast costs no code, but armcc stops tail-calling (matching int/int types give `pop {..,lr}; b f79`, 6/11). A struct, `unsigned char`, pointer, float or varargs return, or a static wrapper, did not block the tail call. A scan of every `.cro` for this prologue found only these two sites. The 20+ C++ forms tried on the zero-register destructor class did not help (see `attempted.md`).

## 2026-10-16 round a — batch 18 (C++ inline-member sweep: DllField 0x2948c)

| Module | Offset (size, match) | Style | What |
|---|---|---|---|
| `DllField` | `0x2948c` (0x44, 17/17) | **trick(cpp-inline-member)** | guarded release: `t = this->t; if (t) { if (t->x) { fa(arg, t->x); t->st = -1; } else fb(0,0,0) }` (else arm is a tail call) |

**`DllField@0x2948c` (0x44, 17/17, `trick(cpp-inline-member)`, `.cpp`)** — `t = this->t; if (t) { if (t->x) { fa(arg, t->x); t->st = -1; } else fb(0,0,0) /* tail call */ }`: the pointee-side logic written as an inline member `U::Go()` called on the loaded pointer (`if (s->t) ((U*)s->t)->Go(a);`); the plain-C form is 5/17. The mechanism generalises ("write the pointee-side logic as an inline member called on the loaded pointer") but only flipped this one of ~12 branch-vs-conditional-execution / scheduling failures swept (`attempted.md`). No byte-identical siblings exist in the corpus.
**Verifier limitation, now confirmed twice:** `DllBattle@0x71a2c` and `DllField@0xb6844` / `0xb76c8` start at 4 mod 8, so retail has a zero pad word before its 8-aligned literal pool (ldr displacement 40 vs our 36), which a standalone compile starting at offset 0 can never emit — they need a `fingerprint_fn.py` option that tolerates pool-alignment padding. **Proposed tool improvement.**

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
