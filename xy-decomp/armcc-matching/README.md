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

**Reading the module count honestly (2026-10-15 swarm note):** the
"N functions across M modules" headline is breadth, not depth. The swarm's
executor skill deliberately mines the *easiest* win per module first
(trivial 1-2 instruction virtual-override stubs and field setters have a
near-100% hit rate — see "What reliably works" below), so as of this note
**21 of 31 matched modules have exactly one trivial `return 0`/`return 1`/
field-setter stub proven and nothing else** — the module itself still has
its full remaining function count (typically hundreds) completely
unaddressed. The other 10 modules (`DllDialogCommon`, `DllNumberInput`,
`DllStrInput`, `DllFatalErrorProc`, `DllLangSelect`, `DllHeading`,
`DllCommendation`, `DllBattleHouseConnect`, `DllGameClearSave`,
`DllUSBigFourSteel`) have either multiple matched functions or at least
one structurally meaningful match (a destructor/constructor/dispatcher,
not just a 1-2 word stub). Treat "M modules touched" as a
toolchain-portability proof (the technique works on unrelated modules, not
just the first one found), not as module completion.

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
