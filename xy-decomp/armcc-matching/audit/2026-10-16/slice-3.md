# Style audit, slice 3 (158 files: DllBattle@0xbc65c.. through DllField@0x5be0)

Run 2026-10-16. All 158 files re-verified after edits (`fingerprint_fn.py`, size/flags from the header): 158/158 MASKED MATCH.

## (a) Summary counts

| metric | count |
|---|---|
| files audited | 158 |
| tag unchanged (already `natural` or `trick(...)` and correct) | 112 (105 natural + 7 trick) |
| no Style line -> explicit `Style: natural` added | 46 |
| retagged natural -> trick | 0 |
| retagged trick -> natural | 0 |
| trick kind corrected | 0 |
| trick `why` text refined (placeholder `{K}` filled, audit result noted) | 7 |
| unnecessary tricks removed (code rewritten) | 1 (DllBattlePartySelect@0x1684, previously untagged) |
| flagged not-GameFreak-like | 0 beyond the 7 known tricks below |

No inline asm, intrinsics, pragmas, `register`, `goto`, `volatile` or `__attribute__` anywhere in the slice. Remaining oddities (`pad[]` structs, `(char*)p + N` casts, `extern char VT[]`, `(void)this`, `unsigned char v = g(..)` narrowing locals that carry real semantics, zero-init/byte-splat stores) are acceptable RE placeholders.

## (b) Files whose tag or code changed

| path | old Style | new Style | change |
|---|---|---|---|
| DllBattleMatch/0xd8c.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBattleMatch/0xd90.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBattleMatch/0xd94.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBattleMatch/0xde4.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBattleMatch/0xde8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBattleMatch/0xe38.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBattlePartySelect/0x1684.c | (none) | natural | added tag; removed unnecessary local `int r; if/else` -> `if (c != 0) return 0; return 1;` (plain predicate form matches); header note rewritten |
| DllBattleVideoPlayer/0xb14.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBattleVideoSave/0x454.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0x9e8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0x9f0.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0x9f8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0xa00.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0xa04.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0xa0c.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0xa10.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0xc80.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0x1dd58.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0x1dd60.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0x1dde8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBox/0x1f46c.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBoxSearch/0x5b8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllCaptureRegister/0x27b0.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllColosseum/0x46fc.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllCommWait/0xcd8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllConfig/0x3a74.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDendouDemo/0x798.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDendouPC/0x650.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x2d8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x2e0.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x2e8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x2f0.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x2f4.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x2f8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x2fc.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x688.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x6c4.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x6e8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0x8d0.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor); stale `test/fingerprint/...` Verify path fixed |
| DllDialogCommon/0xcdc.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0xd10.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllDialogCommon/0xd18.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllEggDemo/0x568.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllEshop/0x25c4.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllFatalErrorProc/0x2bc.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllField/0x58c8.c | (none) | natural | added explicit `Style: natural` (plain stub/getter/predicate/accessor) |
| DllBattle/0xbf0b0.c | trick(redundant-local) -- ...({K})... | trick(redundant-local) -- (arg 17, test result) | filled the unexpanded `{K}` template placeholders; tested literal inlined: FAILS, so trick stays |
| DllBattle/0xbf0fc.c | trick(redundant-local) -- ...({K})... | trick(redundant-local) -- (arg 14, test result) | same as 0xbf0b0 (tested inlined 14: FAILS) |
| DllBattleHouseConnect/0x2fc.c | trick(ordering-local) | trick(ordering-local) + audit note | plain `p = o->buf; o->vptr = VT;` form tested: FAILS (r1/r0 load order) |
| DllEshop/0x554.c | trick(ordering-local) | same + audit note | same test, FAILS |
| DllCommendation/0xc94.c | trick(ordering-local) | same + audit note | same test, FAILS |
| DllFatalErrorProc/0x284.c | trick(ordering-local) | same + audit note | same test, FAILS |
| DllEshop/0x1b10.c | trick(static-helper) | same + audit note | inline `o->f58 = 1;` tested: FAILS (constant lands in r1, retail reuses r0) |

## (c) FLAGGED -- cannot be made natural (known tricks, must stay tagged)

- `redundant-local` DllBattle@0xbf0b0, 0xbf0fc: `int k = 17/14;` constant must be held in a callee-saved reg hoisted above the first query; inlining the literal changes scheduling.
- `ordering-local` DllBattleHouseConnect@0x2fc, DllEshop@0x554, DllCommendation@0xc94, DllFatalErrorProc@0x284 (byte-identical tail-call dtor family): `void *vt = VT;` local before the member load; direct `o->vptr = VT` stores eagerly and fails.
- `static-helper` DllEshop@0x1b10: one-use `static void set(X*, int v)`; inline store picks r1 for the constant instead of reusing r0.

## (d) Patterns / lessons

- The 46 untagged files were all natural on reading (event-trigger predicates, guarded report helpers, zero-init/getter/setter stubs, vtable thunks, `return 1/0` stubs, bx-lr no-ops, veneer forwarders). The dominant remaining weakness is evidence strength, not style: most 0x4-0x10 byte stubs compile identically from any body.
- One unnecessary trick was hiding in an untagged file: DllBattlePartySelect@0x1684 used an `int r; if/else` local to force movne-before-moveq ordering; the plain `if (c != 0) return 0; return 1;` form (the README's accepted predicate shape) matches. Only `return c == 0;` / ternary fail. Worth remembering: the "comparison-result" predicate shape should be written `if (x) return 0; return 1;`.
- Some headers carried unexpanded template placeholders (`{K}`) from a generator (the DllBattle 0xbf0b0 family); other generated DllBattle headers may too -- a grep for `{K}`/`{N}` across matched/ is cheap.
- DllDialogCommon@0x8d0 had a stale Verify path (`test/fingerprint/...`); fixed. Other early files may have similar stale paths.
- No new kind needed for the README table; the audit used only existing kinds (`redundant-local`, `ordering-local`, `static-helper`).
