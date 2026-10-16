# Style audit, slice 4 (DllField, 158 files)

Scope: `matched/DllField/` files 0x5e34 .. 0x5c4bc (list in the audit slice file).
All 158 files re-verified from the saved sources at the size in their header
(`fingerprint_fn.py DllField <off> <size> <file>`): **158/158 MASKED MATCH**
after the edits. No `--mask-ranges` / `--pool-pad` files in this slice.

## (a) Summary counts

| metric | count |
|---|---|
| files audited | 158 |
| tag unchanged (and not edited) | 106 |
| files with no Style line -> explicit `Style: natural` | 18 |
| retagged natural -> trick | 3 (`assign-as-return`, new kind) |
| retagged trick -> natural | 5 (ordering-local dtor family, DllField) |
| trick kind corrected | 2 (`volatile-load` -> `volatile`), plus 1 why-text sharpened (0x588b0) |
| unnecessary tricks removed (code rewritten) | 5 (`vt` local) + 2 redundant locals in natural-tagged range predicates |
| other cleanups (dead externs, stale header) | 22 files lost unused `extern`s, 1 stale pasted header removed |
| flagged not-GameFreak-like (no natural form found) | 0 beyond the tricks listed in (c); every such construct is tagged |

(52 files changed on disk; the other 106 were only read and re-verified.)

## (b) Every file whose tag or code changed

Paths are `xy-decomp/armcc-matching/matched/DllField/<name>`.

### No Style line -> `Style: natural` (code untouched)

| file | old | new | note |
|---|---|---|---|
| 0x10f10.c | (none) | natural | `return 1` virtual stub |
| 0x10f18.c | (none) | natural | byte setter from 3rd arg + `return 1`; `(void)a1` placeholder |
| 0x1a448.c | (none) | natural | three u16 = -1 |
| 0x1bd48.c | (none) | natural | two-field setter with pad[] struct |
| 0x228b8.c | (none) | natural | pointer-field setter |
| 0x288c0.c | (none) | natural | pointer-field setter |
| 0x288c8.c | (none) | natural | zero setter, `(void)v` |
| 0x297d4.c | (none) | natural | zero init |
| 0x30734.c | (none) | natural | -1 pair init |
| 0x3296c.c | (none) | natural | 5-field zero init (store order is real source order) |
| 0x34238.c | (none) | natural | `FunA(); FunB();` |
| 0x36d04.c | (none) | natural | same |
| 0x39a88.c | (none) | natural | same |
| 0x5445c.c | (none) | natural | two-call helper, literal args |
| 0x54d84.c | (none) | natural | two-call helper; literal `1, 1` is the natural form (a named local was the failing variant) |
| 0x54dc4.c | (none) | natural | call + `return 3` |
| 0x54e08.c | (none) | natural | two-call helper (`long` == `int` on ARM) |
| 0x55384.c | (none) | natural | same family |

### Retagged

| file | old Style | new Style | what / why |
|---|---|---|---|
| 0x2aa40.c, 0x2aee0.c, 0x343d0.c | natural | trick(assign-as-return) | `char Fn(){ Imp(8); return this[32] = 0; }`: the char return exists only to share one `mov r0,#0` between the store and the return. Tested: `void` + `this[32]=0` and `int` + `this[32]=0; return 0;` both give a spare `mov r1,#0` (fail). New kind, see (d). |
| 0x3954c.c, 0x39728.c, 0x3a69c.c, 0x3a960.c, 0x3cdb4.c | trick(ordering-local) | natural | `void *vt = VT; s->vt = vt;` rewritten to `s->vt = VT;`, still MASKED MATCH. The remaining `O *a = s->a;` hoisted member read is the form the README already calls natural (plain `s->a` use fails, so it is kept). |
| 0x58ba0.c, 0x58bc0.c | trick(volatile-load) | trick(volatile) | same lever as the GetMgr uxth family (full-word `ldr` + separate narrowing). Plain `(short)b[1]` fails (retail loads b[1] first, then `sxth`). Kind unified with the other 14 arg-width volatile files. |
| 0x588b0.c | trick(volatile-load) | trick(volatile-load) (why text) | why now says the chain-load volatiles are also needed: dropping them (arg volatile only) fails. |

### Code rewritten (tag unchanged = natural)

| file | change |
|---|---|
| 0x4a408.c, 0x57d1c.c | redundant local `unsigned x = s->v - N; return x <= 3;` -> `return s->v - N <= 3;` (still matches) |
| 0x7f40.c | stale pasted copy of the 0x5e34 header (wrong offset) removed from the middle of the file |

### Unused `extern` declarations deleted (Style unchanged)

0x58cfc (G), 0x58d2c (Sink), 0x58d50 (K3, G), 0x590fc (G), 0x5912c (G),
0x593f0 (Sink), 0x597b0 (Sink), 0x59848 (F, K3, G), 0x59cb4 (G),
0x59d7c (F, K3, G), 0x59db8 (Sink), 0x59de8 (Sink), 0x59e0c (Sink),
0x59e3c (Sink), 0x5a380 (G), 0x5a650 (Sink), 0x5a674 (Sink), 0x5b5bc (Sink),
0x5c1ac (G), 0x5c2c8 (Sink). The file names/old Style are unchanged
(volatile files stay `trick(volatile)`, the rest `natural`).

## (c) FLAGGED -- cannot be made natural (tricks that stay tagged)

| files | kind | construct / what was tried |
|---|---|---|
| 0x588b0, plus 0x58ba0, 0x58bc0, 0x58cfc, 0x590fc, 0x5912c, 0x59270, 0x592a8, 0x592e0, 0x593cc, 0x59848, 0x59b04, 0x59cb4, 0x59d7c, 0x5a380, 0x5a6b4, 0x5b758, 0x5c490 | volatile (588b0: volatile-load) | arg word read through `volatile int *` so armcc does a full-word `ldr` *before* the GetMgr call and narrows later (`uxth`/`sxth`/`and`). Tried on 0x593cc: `unsigned short x = b[1]`, inline `(unsigned short)b[1]`, `int x=b[1]` + cast, `unsigned *b`, `b[1] & 0xffff`, a one-use static accessor, callee prototyped with `unsigned short` param, `unsigned short *b` -- all fail (armcc folds to `ldrh`/`ldrsh` or loads after the call). |
| 0x10c34, 0x10c54 | redundant-local | `unsigned char z = 0, o = 1;` plain literals give a different r2/r3 choice (3/8 words). |
| 0x37054 | redundant-local | `char z = 0` shared by every store; literal zeros schedule the pool `ldr` into r1 (8/11 words). |
| 0xae58, 0xe380, 0xe3fc | live-r1-passthrough | widened `long long` return built from `this` and unused 2nd arg to keep r1 live; documented plain form gives `mov r1,#0`. True signature unknown. |
| 0x2948c.cpp | cpp-inline-member | tried plain C++ free function (nested ifs and early-return forms): both fail. |
| 0x2aa40, 0x2aee0, 0x343d0 | assign-as-return | see (b). |

## (d) Patterns / lessons

- **New kind `assign-as-return`**: `return p->field = 0;` with a `char`/int return that retail's caller never needs. It is the cousin of `live-r1-passthrough` (widened return solely to reuse a register); proposing a README table row. Files: DllField 0x2aa40, 0x2aee0, 0x343d0.
- **Kind inconsistency in the README table**: the task brief tags the arg-width volatile lever `volatile-load`, but the README table defines `volatile` for exactly that (GetMgr uxth family) and `volatile-load` for "keep armcc from narrowing/dropping a load". I followed the README: 17 DllField files stay `volatile`; the 2 that said `volatile-load` for the same lever were moved to `volatile`; 0x588b0 stays `volatile-load` because it additionally needs volatile chain loads. Decide whether to merge the two kinds.
- **`ordering-local` dtor family was half-unnecessary**: the `void *vt = VT;` local never mattered; only the hoisted member read does, and that is the README's own natural form. Other modules' `vt`-local destructor files (the 15-file list in the README) likely need the same one-line rewrite plus retag to natural.
- **Borderline but kept natural** (named locals that DO matter, so removing them fails, but they are ordinary "read the argument first" idioms): `int v = b[1];` before a call (0x59c98, 0x59db8/e0c/e3c), `int base = *(int*)m;` (0x5bd84), `struct T *t = &p->t;` (0x5aa14; `p->t.b` folds the +288 into `ldrh #36`). If the project wants these classed as `ordering-local`, they are those 6 files.
- The Verify header lines of `.c` files use the repo-relative `XY_ROMFS` default; in a fresh worktree `.Data/` is absent, so verification needs `XY_ROMFS=<main checkout>/.Data/Raw_Extracted/romfs`.
- ~50 of the 158 files are byte-identical siblings (setters, release helpers, call chains, `vptr` setters, GetMgr thunks); the family tests were run once per shape and apply to all siblings, and all siblings were then re-verified individually.
