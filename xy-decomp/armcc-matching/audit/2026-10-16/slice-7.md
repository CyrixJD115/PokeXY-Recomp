# Style audit, slice 7 (2026-10-16)

Slice: 158 matched files (DllGts .. DllStatus, `audit_slice_7.txt`). Every file
re-verified from the saved source with `fingerprint_fn.py` at the size in its
header: **158/158 `*** MASKED MATCH ***`** (before and after the edits).

## (a) Summary counts

| metric | count |
|---|---|
| files audited | 158 |
| had no Style line (now explicit `Style: natural`) | 102 |
| tag unchanged (already correct/complete) | 53 |
| retagged natural -> trick | 0 |
| retagged trick -> natural | 3 (2 unnecessary tricks removed, 1 convention fix) |
| trick kind corrected | 0 (kinds were right; 7 vague "why" texts made specific) |
| unnecessary tricks removed (code rewritten) | 2 (DllKisekae@0xb6f0, DllPoke3Select@0x63c) |
| dead code removed (unused externs) | 5 (DllHeading 0x16f4, 0x1738, 0x1cec, 0x3e34, 0x8c9c) |
| flagged not-GameFreak-like beyond the known tricks | 0 |

Final tags in slice: 127 natural, 31 trick (12 ordering-local, 13
void-ptr-passthrough, 2 pointer-walk, 1 redundant-local, 1 static-helper,
1 volatile-load, 1 volatile-strd).

## (b) Files whose tag or code changed

The 102 files that only gained `Style: natural` (header line, no code change)
are not listed individually (every slice file that previously had no Style
line; `git diff` shows them). Each was read; they are plain setters/getters/stubs, conditional
accessors, forwarding calls, and `this->member` chains with `pad[]` placeholder
structs. `git diff --stat` lists them.

| path | old Style | new Style | change / why |
|---|---|---|---|
| DllKisekae/0xb6f0.c | trick(redundant-local) | natural | plain `*(unsigned char*)(this+0x40a) = 1;` masks 3/3; the `int v = 1;` local and its "retail materializes the 1" comment were wrong |
| DllPoke3Select/0x63c.c | trick(redundant-local) | natural | literal stores in order f28,f30,f34,vptr,f38 masks 10/10 without the `zero`/`one`/`vt` locals (the store order, not the locals, fixes the r1/r2 roles) |
| DllKisekae/0xb6ac.c | trick(ordering-local) | natural | same form as DllPhotoViewer@0x1974 / DllSalon@0xd94 / DllSparringMenu@0x5ad8 (`void *m = o->m; o->vptr = VT; if (m) ...`), which README already defines as natural. Note: the fully inlined `if (o->m)` form does NOT match (9/13), so the member-load local is load-bearing; it is kept for README consistency |
| DllGts/0x5ba8.c, 0x5bc4.c, 0x53e4.c | trick(ordering-local) generic text | trick(ordering-local), specific why | text now names the hoisted `t`/`f30` locals and the `ldr r2,[r0,#16]!` scheduling (single-expression form 4/7) |
| DllGts/0x5be0.c | trick(ordering-local) generic text | trick(ordering-local), specific why | text names `f28`/`f550` hoists (inline-call form 12/15) |
| DllLiveCup/0x764.c, DllRandomTrade/0x8b4.c, DllStaffRoll/0x2c44.c | trick(ordering-local) generic text | trick(ordering-local), specific why | text names the `vt = VT` local (plain forms 10-11/14) |
| DllHeading/0x16f4.c | trick(void-ptr-passthrough) | same | removed unused `extern void *VT[]` |
| DllHeading/0x1738.c, 0x1cec.c, 0x3e34.c, 0x8c9c.c | trick(void-ptr-passthrough) | same | removed unused `extern void *fn1f8` |

## (c) FLAGGED, cannot be made natural (tested without the trick; fails)

| file | construct | no-trick result |
|---|---|---|
| DllGts 0x5ba8/0x5bc4/0x53e4 | two hoisted member-load locals | one-expression call 4/7 |
| DllGts 0x5ad4 | `q`, `v` locals | one-liner 4/7 |
| DllGts 0x5be0 | `f28`/`f550` locals | inline call 12/15 |
| DllLiveCup 0x764, DllRandomTrade 0x8b4, DllStaffRoll 0x2c44, DllSalon 0xd5c, DllPhotoViewer 0x193c, DllPuzzle 0xbf80, DllKisekae 0xb674 | `vt = VT` local before member load | 10-11/14 for both plain and member-hoist forms |
| DllGts 0x8850 | `int v3 = 1;` for arg 3 | plain `fn(p->f, 1,1,1,1)` 4/10 |
| DllHologramMail 0xe970 | one-use static helper `set_flags` | flat stores 6/9 |
| DllSequence 0x1140 | `p += 0x1800; p += 29;` cursor | `buf[0x181d] = 1` 2/6 |
| DllNumberInput 0x18fc | pre-indexed cursor chain | fixed-offset variant 23/30 |
| DllSequence 0x10a80 | `volatile int` field | non-volatile 0/4 (loads dropped) |
| DllSalon 0xdd0 | `volatile long long g_a[2]` | non-volatile 4/11 (no strd) |
| DllHeading 0x868/0x8ac/0x16f4/0x1738/0x1ca8/0x1cec/0x3df0/0x3e34/0x8c58/0x8c9c, DllPuzzle 0x1e90/0x4ba0, DllSalon 0x790 | `(char*)fn(p) - N` on a `void*` callee | not retested; the result-minus-N is semantically required (a saved `o` local adds `mov r4,r0`, per file notes). Likely a C++ secondary-base this-adjusting thunk in the real source |

Rewritten to natural and verified: DllKisekae 0xb6f0, DllPoke3Select 0x63c.

## (d) Patterns and lessons

- **Two `redundant-local` tags were unnecessary.** Both came from the author
  guessing "named local steers register choice" when the real lever was store
  order (Poke3Select) or nothing at all (Kisekae). Lesson: always retry the
  plain literal form before tagging redundant-local. Remaining redundant-local
  (DllGts@0x8850) was re-tested and is genuinely needed.
- **ordering-local needs two sub-kinds in practice:** `vt = VT` (pool-load
  order) versus hoisted member loads (`t`/`f30`, `f28`/`f550`) for arg-eval
  order. Kept as one kind, but the why-texts now say which.
- **Untagged files with a plain-looking body are not all trivially natural:**
  the untagged dtor/ctor-shaped files in the slice (e.g. DllSalon@0xd94,
  DllPhotoViewer@0x1974, DllSparringMenu@0x5ad8) use the member-hoist form
  and match; plain `if (o->buf)` does not. README's rule covers this, but the
  rule is a convention, not an RE finding.
- `long` return type in DllStaffRoll stubs/forwarders (0x59c, 0xe90, 0x2304)
  is the same type as `int` on ARM32 armcc; not a widening trick.
- Many stubs use `(void)this;`; fine per the audit rules (no `volatile`,
  asm, intrinsics, `register`, `#pragma`, or `goto` anywhere in the slice).
- No kind was missing from the README table.
