# Style audit, slice 2 (DllBattle 0x9a9b0..0xbc3a8)

## (a) Summary
- Files audited: 158 (all DllBattle, all tiny: 8-17 lines of body)
- Tag unchanged (kind and tag verdict): 158
- Retagged natural -> trick: 0
- Retagged trick -> natural: 0
- Trick kind corrected: 0
- Unnecessary tricks removed: 0 (both trick families re-tested without the trick; both still needed)
- Flagged not-GameFreak-like: 0 (no asm, intrinsics, pragma, register, goto, volatile, __attribute in the slice)
- Files edited: 19 (header `-- why` text only; code untouched). All 19 re-verified: MASKED MATCH.
- Tagged trick in slice: 19 (18 redundant-local, 1 static-helper); remaining 139 are natural.

## (b) Files whose header changed
| path | old Style | new Style | change |
|---|---|---|---|
| xy-decomp/armcc-matching/matched/DllBattle/0xa6120.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6188.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa62f0.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa638c.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6454.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6474.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6494.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6610.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6630.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6794.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa68e4.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6904.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa697c.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6abc.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6bec.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6c0c.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6cc8.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xa6df0.c | trick(redundant-local) | trick(redundant-local) | why text extended: plain literal 5th arg tested, emits `mov ip,#Y` first -> mismatch |
| xy-decomp/armcc-matching/matched/DllBattle/0xb8b84.c | trick(static-helper) | trick(static-helper) | why text extended: inline `&&` form tested, `and r0` instead of `and r1,r0,#255` -> mismatch |

## (c) FLAGGED - cannot be made natural
- 0xa6120 family (18 files: 0xa6120 0xa6188 0xa62f0 0xa638c 0xa6454 0xa6474 0xa6494 0xa6610 0xa6630 0xa6794 0xa68e4 0xa6904 0xa697c 0xa6abc 0xa6bec 0xa6c0c 0xa6cc8 0xa6df0): `int e = N; fnX(a,b,c,d,e,M);` Tested once for the family on 0xa6120 with `fnX(a,b,c,d,3,0)`: armcc emits `mov ip,#0` before `mov r4,#3`, retail has r4 first. Tag stays redundant-local.
- 0xb8b84: one-use `static int chk(c, v)`. Tested inlined `(unsigned char)f6b91c(4) != 31 && f4c5a8(c)`: the u8 mask lands in r0 (`and r0,r0,#255; cmp r0,#31`) vs retail r1. Tag stays static-helper.

## (d) Patterns / lessons
- The other 139 files are plain forwarders, singleton stubs (`*p = N; return &G;`), and getter-compare-tailcall event predicates. All natural. Narrowing casts `(unsigned char)g(..)` / `(unsigned short)g(..)` feeding an int callee were treated as natural (they mirror u8/u16 query semantics, no volatile).
- Several unused `a, b` params in the predicate family are the real incoming context args (r0/r1 untouched, `c` is r2), not an extra-arg steer.
- No new kind needed. The singleton stubs and the 5-arg forwarders are weak evidence (any same-shape stub compiles identically), as the README already notes.
- Note: worktrees have an empty `.Data/`; verification needs `XY_ROMFS` pointed at the main checkout's `.Data/Raw_Extracted/romfs` (absolute path).
