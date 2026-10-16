# Style audit, slice 1 (158 files: DllBackGround@0x4e4, DllBag x23, DllBattle x133)

All 158 files re-verified after editing (`fingerprint_fn.py`, flags from each Verify line): 158/158 MASKED MATCH.
Extra check run over the whole slice: every intra-function `b`/`bcond` immediate (not just the masked word) compared
unmasked between retail and the fresh compile. The only differences are tail-call `b` to imports (link-time facts).
So no masked match in this slice hides a different branch shape. (Throwaway script, not committed.)

## (a) Summary counts

| metric | count |
|---|---|
| files audited | 158 |
| tag unchanged (explicit tag already present, kept) | 124 (of which 16 are `trick(...)`, 108 natural) |
| no Style line, now explicit `Style: natural` (natural-by-default made explicit) | 33 |
| retagged natural -> trick | 0 |
| retagged trick -> natural | 1 (DllBattle@0x7a780) |
| trick kind corrected | 0 |
| unnecessary tricks / dead code removed (code changed) | 3 code rewrites + 1 dead struct + 3 stale headers (below) |
| flagged not-GameFreak-like | 0 (16 files remain tagged trick, all needed; see (c)) |

Remaining `trick` files (16): DllBag@0x13848; DllBattle@0x39c0, 0x12fcc, 0x2b9dc, 0x4d184, 0x704ac, 0x797b0, 0x74ff8,
0x75050, 0x750a4, 0x7bae0, 0x7efb4, 0x80820, 0x80858, 0x82a7c, 0x853a0.

## (b) Files whose tag or code changed

The 33 "added natural" files only gained a header line `Style: natural`, no code change (listed compactly after the table).

| path (matched/...) | old Style | new Style | change / why |
|---|---|---|---|
| DllBattle/0x82324.c -> 0x82324.cpp | natural (none-trick) | natural | `^ 1` (bit trick emulating `eor #1`) replaced with C++ `bool` + `!`: `extern bool fn799d8(...); bool fn(...) { return !fn799d8(...); }` compiles to the same `eor r0,r0,#1`. Renamed to `.cpp` (bool semantics, not a steer). Verify line path updated. |
| DllBattle/0x4d184.cpp -> 0x4d184.c | trick(ordering-local) | trick(ordering-local) (why text expanded) | `.cpp` was unnecessary: it was only there for a declaration after a statement. Rewritten as plain C (`unsigned char lo;` at top, assigned after the assert call), still matches. The `lo` local itself stays a trick (plain `hi - lo` loads hi first; retail loads lo first). Verify path updated. |
| DllBattle/0x7a780.c | trick(extra-arg) | natural | Retail receives `this` in r1, so the leading unused parameter is a real parameter, not a steer; it is the same `(script ctx, obj)` shape as every script-command handler in 0x7af3c/0x7bae0/0x80820/... Tag + note rewritten. |
| DllBattle/0x1fd8.c | none | natural | one-use local `p` inlined (`return *(char **)((char *)o + 2960) + 48;`), re-verified. |
| DllBattle/0x4c7d0.c | none | natural | removed dead unused `struct O { char pad[17920+122]; };`. |
| DllBattle/0x4344c.c | natural | natural | header description was wrong ("returns r==103\|\|r==104"); the code (and retail, branch immediates checked) returns 0 for 103 and `r==104` otherwise. Text corrected. |
| DllBag/0xc88.c, 0xc90.c, 0xc9c.c | none | natural | removed a pasted duplicate of DllBag@0xc80's header (stale/contradictory: wrong offset, wrong Verify line). |

Files that only gained `Style: natural` (header line, no code change): DllBackGround/0x4e4; DllBag/0xc80, 0xc88, 0xc90,
0xc9c, 0x10c8, 0x7330, 0xc508, 0xc558, 0xd03c, 0xe484, 0xfee4, 0x102f8, 0x10334, 0x10dbc, 0x1366c, 0x138b4, 0x138bc;
DllBattle/0x1f8c, 0x1fac, 0x1fbc, 0x1fc4, 0x1fd8, 0x2228, 0x227c, 0x2420, 0x2c3c, 0x2c60, 0x2c90, 0x2d88, 0x2ef0,
0x39ac, 0x39f4.

Unchanged-but-checked (variant compiled without the trick, still fails, tag and reason confirmed): DllBag@0x13848
(plain `vt`-less form fails), DllBattle@0x39c0 (`b->f = 1` swaps r0/r1), 0x12fcc, 0x2b9dc (`v->x=-94; v->y=-54` gives two
vstr, no vstmia), 0x704ac and 0x797b0 (single-expression form reorders the orr/and), 0x74ff8/0x75050/0x750a4
(direct `o->fld` fails), 0x7bae0/0x853a0 (no `q` copy fails), 0x7efb4 (`x == 0.0f`, `0.0f == x`, `x == 0`, ternary,
braced all fail), 0x80820/0x80858/0x82a7c (no `q`/`x` copies fail).

## (c) FLAGGED, cannot be made natural (stay tagged trick)

| file | construct | why needed |
|---|---|---|
| DllBag@0x13848 | `void *vt = VT;` local | forces vtable pool load before the member load |
| DllBattle@0x12fcc | `vt = VT` local + hoisted `a = this->a` | same pool-load/member-load order |
| DllBattle@0x39c0 | `int one = 1; b->f = one;` | direct store swaps r0/r1 (7/10) |
| DllBattle@0x2b9dc | named float locals `a`,`b` | armcc pairs them into one `vstmia`; direct stores give two `vstr` |
| DllBattle@0x4d184 | `unsigned char lo` local | load order (lo byte before hi byte) |
| DllBattle@0x704ac, 0x797b0 | `m1`/`m2` mask locals + split `v \|= tag` | retail hoists both constants and emits tag orr before the final or |
| DllBattle@0x74ff8, 0x75050, 0x750a4 | `void **pp = &o->fld` | retail forms `&o->fld` once in a callee-saved register |
| DllBattle@0x7bae0, 0x853a0 | `O *q = o;` copy | register assignment of the row pointer (20/23 without) |
| DllBattle@0x7efb4 | `float z = 0.0f;` | loads the 0.0f pool constant before the first stack float |
| DllBattle@0x80820, 0x80858, 0x82a7c | `q = o`, `x = v` copies | r4/r5 assignment, `[sp]`-before-member load order |

## (d) Patterns and lessons

* **`^ 1` was an instruction emulation, not source**: C++ `bool` + `!` yields `eor #1` natively. Worth grepping the other slices for `^ 1` / `^ 1u` after bool-returning calls.
* **`.cpp` hygiene**: 0x4d184 used `.cpp` only to allow a mid-block declaration; always try declare-at-top C first. `.cpp` is legitimate for `bool`.
* **The unused leading arg of script handlers is a real parameter** (retail has `this`/obj in r1, ctx in r0). The `extra-arg` kind should not be applied when the callee is a member of an `(int ctx, O *obj)` family; suggest README note for DllBattle@0x7a780. Not edited, per instructions.
* **Borderline natural files, kept natural by the README precedent that a member-load local is natural**: plain forms fail for DllBag@0xe484, 0x7330, 0xc508 (`Sub *s = &this->sub` sub-object pointer local; retail does `add r4,r0,#8` first), DllBag@0x10334 (`E *c = this->c` hoisted above the call), DllBattle@0x4c374 (`P *p = o->p` above the range check), 0xc074 (`t` loaded before the byte stores). They are top-of-function initialised locals, ordinary C89 style, but each is an ordering-sensitive local. If the lead wants a stricter reading, these six are the candidates for `ordering-local`.
* **No new kind needed**: all tags used come from the README table (`ordering-local`, `redundant-local`).
* **Header drift**: DllBattle@0x4344c's description contradicted its own code (`||` vs early-return). Masked matching plus a prose-only description let this survive; the unmasked intra-function branch check above is cheap and could be folded into `fingerprint_fn.py` as an optional strict mode (compare non-`bl` `b`/`bcond` immediates whose retail target falls inside the function).
* Natural-by-default held up: none of the 33 previously unlabeled files contained a trick.
