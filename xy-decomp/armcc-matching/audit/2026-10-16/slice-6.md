# Style audit, slice 6 (DllField 0x9d564..0xe7264 plus 10 small modules; 158 files)

Method: every file read; every trick tag and every suspicious "natural" file (locals, pre-loaded args, literal-holding
locals, volatile) was re-compiled WITHOUT the construct and re-run through `fingerprint_fn.py`. All 158 files re-verify
`MASKED MATCH` from the saved files after the edits (header Verify size/flags).

## (a) Summary counts

| metric | count |
|---|---|
| files audited | 158 |
| tag unchanged (already correct) | 139 |
| no Style line -> `natural` added (trivial stubs/setters) | 8 |
| no Style line -> `trick(ordering-local)` added | 2 |
| retagged natural -> trick | 7 (1 `redundant-local` q-local, 6 `redundant-local` literal-holder locals) |
| retagged trick -> natural | 0 |
| trick kind corrected / extended | 2 (0xe1c4c, 0xe1fe8) |
| unnecessary tricks removed (code rewritten plain) | 0 (every tested trick fails without the construct) |
| unused `extern` declarations deleted (code, no semantic change) | 6 files |
| flagged not-GameFreak-like (cannot be made natural) | 0 files outright; known tricks listed in (c) |

## (b) Files whose tag or code changed

| path (under matched/) | old Style | new Style | change / why |
|---|---|---|---|
| DllFieldEventPlayer/0x624.c | none | natural | explicit line added; trivial int field setter |
| DllFieldEventPokeRideAction/0x520.c | none | natural | explicit line; return-1 stub |
| DllFieldEventSkatePipe/0x1b28.c | none | natural | explicit line; return-1 stub (`(void)this;` is an acceptable placeholder) |
| DllFieldEventEntranceIn/0x628.c | none | natural | explicit line; return-1 stub |
| DllGift/0xae0.c | none | natural | explicit line; return-1 stub |
| DllFieldEventSlopeSlip/0x268.c | none | natural | explicit line; return-1 stub |
| DllFieldEventTakinobori/0x11d4.c | none | natural | explicit line; int field setter |
| DllFieldDemo/0x1edc.c | none | natural | explicit line; byte field setter |
| DllField/0xa6ff0.c | none | trick(ordering-local) | copy word loaded into a local first so the order is ldr, strb, str. Plain `flag=0; dst=src;` and `dst=src; flag=0;` both fail (code unchanged) |
| DllField/0xa714c.c | none | trick(ordering-local) | same (both plain orders tested, fail) |
| DllField/0x9e018.c | natural | trick(redundant-local) | `q = t + 420` local pins retail's early `add r6,r5,#420`; inline `t + 420` computes late (8/23); the float locals alone don't help |
| DllField/0xce874.c | natural | trick(redundant-local) | `int n = 7` local holds the literal in a callee-saved reg across A/B; literal args inline fail (length mismatch) |
| DllField/0xcf44c.c | natural | trick(redundant-local) | same, n = 11 |
| DllField/0xcf880.c | natural | trick(redundant-local) | same, n = 1 |
| DllField/0xcfcd0.c | natural | trick(redundant-local) | same, n = 6 |
| DllField/0xcfde4.c | natural | trick(redundant-local) | same, n = 0 |
| DllField/0xcfee0.c | natural | trick(redundant-local) | same, n = 8 |
| DllField/0xe1c4c.c | trick(wider-callee-return) | trick(wider-callee-return+redundant-local) | `struct Sub *p = &a->s` local is also needed (early `add r5,r1,#16`); with `long long` callee but inline `a->s.y` the compile fails |
| DllField/0xe1fe8.c | trick(wider-callee-return) | trick(wider-callee-return+redundant-local) | same (twin) |
| DllField/0x9dd04.c, 0x9dd2c.c, 0x9de18.c, 0x9e87c.c, 0x9ea30.c, 0x9f1e0.c | natural | natural | deleted the unused `extern void Sink(char *);` line (dead declaration); still masked-match |

## (c) FLAGGED -- known tricks that must stay tagged (re-tested, plain form fails)

- `volatile-load` (10 files: 0x9e400, 0x9e7e4, 0x9ea94, 0x9f1fc, 0x9ff64, 0xa02bc, 0xa0c34, 0xa0cf4, 0xa14c0, 0xa02e0):
  `volatile int *a` so armcc keeps `ldr` + `uxth`. Tested plain `int *a` with the `unsigned short` local, with an inline
  cast, and with the callee prototype taking `unsigned short` (hoping armcc narrows at the call site): all fail
  (armcc does not narrow at the call site in C mode). 0xa02e0 additionally fails without volatile either way.
- `volatile-store` (all 25 `.cpp` sites 0xb1a90..0xc2618): `*(void *volatile *)o = vt`. Plain `o->vt = vt` fails for both
  the void/delete shape (0xb1a90) and the returning shape (0xb1bb8); the inline-member `Reset()` is also required (a plain
  `if (o->m) fr(o, &o->m)` in a `.cpp` fails with a length mismatch). The `Reset()` member is documented in the 0xb1bb8 header;
  arguably a second kind (`cpp-inline-member`) layered on the volatile store.
- `ordering-local` (0xc8f88; DllFriendEntry@0x4c4, DllGameClearSave@0xd6c, DllGameSync@0x5e0): removing the local fails
  (0xc8f88 needs the `r = byte; state = 13; return r;` form; the dtors need `vt = VT` before the member read).
- `redundant-local` 0xcff28: `p = t + 0x100` confirmed needed.
- `volatile-strd` DllFieldDebug@0x180: plain `long long g_a` gives stm, confirmed.
- `wider-callee-return` 0xe1c4c / 0xe1fe8: see (b).

Borderline, left `natural` (judgement call, listed for the reviewer): 14 script-command handlers read `int v = a[1];` into a
local BEFORE the `GetMgr()` call (0x9d564, 0x9dd04, 0x9de18, 0x9eab8, 0x9f0e0, 0xa0008, 0xa0cd8, 0xa13c4, ...). Passing `a[1]`
inline (0x9eab8, 0x9d564 tested) fails because armcc then reads it after the call, so the local does steer order; I judged
"read the event arg first, then fetch the manager" as the plausible original statement order, not a codegen-only local. If the
reviewer disagrees these are `ordering-local`. Also 0xca1a8: the `int v` parameter must be full-width (`char`/`unsigned char`
fail; `bool` = int in C passes), so keep `int` -- natural.

## (d) Patterns / lessons

- The 2026-10-15 audit missed locals whose only effect is hoisting a literal or pointer-add into a callee-saved register
  (state-handler family `int n = <step id>`, `q = t + 420`). The reliable test is mechanical: inline the local and re-run the
  verifier; "no trick found by reading" is not enough. Other slices' natural files with a single `int x = <literal>` local
  deserve the same test.
- No kind missing from the README table; `redundant-local` covers the new cases. I used a combined kind
  `wider-callee-return+redundant-local` for 0xe1c4c/0xe1fe8 (a `+` join of two table kinds, since the tag grammar has one slot).
- Every tag in the slice that existed before was needed; none of the volatile / ordering tricks is removable by an alternative
  plain form I could think of (inline arg, u16 prototype, struct field).
- Dead `extern` declarations (copy-paste leftovers like `Sink`) show up in the shared-accessor family; a script that counts
  uses of each extern in the non-comment lines found all 6.
