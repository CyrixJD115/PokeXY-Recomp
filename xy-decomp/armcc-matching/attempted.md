# Attempted — not yet matched

Functions a decompiling agent spent real effort on (up to the 30-minute
per-agent time cap — see README.md's "Working on this" section) without
reaching a masked match. **Check this list before picking a candidate.**
If a function is already here, skip it unless you have a genuinely new
idea beyond what's listed — don't just retry the same C shape someone
else already tried.

This is a working log, not a polished doc: newest entries at the top,
terse is fine. Once something here actually gets matched, move its
entry out (delete it) and add it to `README.md`'s table instead — this
file is only for things still unsolved.

Format per entry:

```
## DllBattle 0xbd2dc (round b, event-trigger sweep 0xb8000-0xca000)

Module: DllBattle
Offset: 0xbd2dc
Size: 0x60
Match: 15/24

`if (g(3) == c) { v = (ushort)g(18); if (h(v, 30) != 0 || v == 26 || v == 136) post(50, K); }` -- retail does `cmp r4,#26; beq T; cmp r4,#136; bne end` with T the shared tail-call block; armcc fuses to `cmpne r4,#136; bne` and orders the tail call differently. Tried negated early return, if/else with a duplicated post call, `||` forms. The retail layout suggests the post call is reachable from two separate branches.

## DllBattle 0xc4f60 (round b, event-trigger sweep 0xb8000-0xca000)

Module: DllBattle
Offset: 0xc4f60
Size: 0x44
Match: 15/17

`if (g(3) == c && (ushort)g(18) == 505) post(54, 3072)`: retail compares via `sub r1,r0,#256; subs r1,r1,#249; bne` (two-step subtract), armcc emits `mvn r1,#504; cmn r0,r1`. Tried `- 256 - 249 == 0`, `- 256 == 249`, `g(18) - 505 == 0`. The retail split suggests the constant is built as 256 + 249 (macro/enum arithmetic); source form unknown.

## DllBattle event-trigger predicates that save BOTH b (r1) and c (r2) -- ~213 functions in 0xb8000-0xca000 (e.g. 0xb8d3c, 0xb8008, 0xb8048, 0xb9c70, 0xbfc9c, 0xc06c8, 0xbca9c) (round b, event-trigger sweep 0xb8000-0xca000)

Module: DllBattle
Offset: 0xb8d3c (representative; full list = every unmatched 0x6b91c caller in range whose prologue has `mov rN, r1`)
Size: 0x40
Match: 14/16

Shape (0xb8d3c): `if (g(3) == c && h1(b) == 1) post(44, 50)` -- retail prologue is `push {r4,r5,r6,lr}; mov r5,r1; mov r4,r2` (b first), armcc here emits `mov r4,r2; mov r5,r1`. Body otherwise identical (14/16, only the two prologue words differ). Same residue already logged in attempted.md for 0xa7cd0/0xa7188/0xc1244; this confirms it covers ~213 more functions here (the largest unmatched cluster in the range).

Tried (all the same 14/16): plain `&&` form, nested ifs, `(void)b;` first, `int bb = b; int cc = c;` locals in both orders, swapping the parameter order/names, calling h1(b) first. No natural form found.

Observation: the move order is NOT simply allocation order. Retail (b,c) emits r1-first; (d,c) = 0xc489c emits r3-first (matched, natural); (c,d) = 0xbc89c/0xbe5b8/0xbef24/0xc282c/0xc3cd0 emit r2-first while armcc here emits r3-first. So the (c,d) and (b,c) shapes both need the opposite order from what we produce; a single missing source-level fact (a 4th arg, an intermediate struct/vtable local) might explain both.

## DllBattle event predicates saving c (r2) then d (r3) in that order -- 0xbc89c 0xbe5b8 0xbef24 0xc282c 0xc18d8 0xc3cd0 0xc44ac 0xc1ff8 (round b, event-trigger sweep 0xb8000-0xca000)

Module: DllBattle
Offset: 0xc282c (representative)
Size: 0x24
Match: 7/9

`if (g(2) == c) *d = 0;` -- body correct, retail prologue is `mov r4,r2; mov r5,r3`, armcc emits `mov r5,r3; mov r4,r2`. Other members: 0xbc89c `if (g(2)==c && g(83)==0) *d = post(83,1)` (14/16), 0xc3cd0 `if (g(2)==c && d[3]) post(54,2048)` (13/15). Tried d as short*/char*, early-return form, local alias of d. Same family as the b+c residue (attempted-DllBattle-bc-saved-family.md); armcc's d-first order is what the natural source gives for the matched 0xc489c / 0xc1e40 / 0xbf0b0 shapes.

## DllBattle nop-relaxed (round b, event-trigger sweep 0xb8000-0xca000)

Module: DllBattle
Offset: 0xc2a58 (siblings 0xc2afc 0xc2b88 0xc4b58 0xc556c; also 0xc23f4, 0xbce68/0xc1cb8/0xc4618 two-block variants)
Size: 0x54
Match: relaxed-only

`if (g(4)==c && (ushort)g(30) != K) post(66,1) else *d = 0`-style predicates whose retail bytes carry the post-link branch-relaxation `nop nop` pair before the final `b`. Only matchable with --relax-nops, which does not count as a masked match, so none were saved.

## DllField@0x90c00 (size 0x54) -- volatile-load retry, 2026-10-16

- What it is: handler `(self, args)`: `o = self->+68; GetMgr(); s = (s16)args[2]; id = (u16)args[1]; tab = o->+144; x = mgr->+12->+72; r = (s == -1) ? tab->+16 : Q(G(x), s); if (r) T(r, id & 255); return 0`.
- Lever applied: `volatile int *b` read (`int a1 = b[1]; int a2 = b[2];` first) gives the retail `ldr r2,[r1,#4]; ldr r1,[r1,#8]; uxth/sxth` and the whole body shape (cmn / ldreq / blne). Best 19/21.
- Stuck on: callee-saved assignment only -- retail `o -> r5, id -> r6`, ours `o -> r6, id -> r5` (with `tab` volatile / hoisted `x = *(int*)(m+72)` before the `if` the instruction order matches exactly; without them the `ldr r1,[tab]` / `ldr r0,[r0,#72]` order differs). Declaration-order variants of o/s/id/a1/a2 (5 tried) all keep the swap; the variant that gets the right registers (volatile tab read inside the `if`) has the wrong load order.
- Lead: same r5/r6 swap class as 0x9e550 / 0x9d970.

## DllField@0x92424 (size 0x58) -- volatile-load retry 2026-10-16

- What it is: `i = (u16)args[1]; m = mgr->+12; r = Q(G(m->+72), i); if (r) { o = m->+72->+344->+12; T(o, F2(F1(r))); } return 0`.
- Lever applied (`volatile int *b`, `unsigned short i = b[1]`): arg handling and first half match (retail `ldr r4,[r0,#12]` = m, `movs r5,r0`).
- Stuck on 11/22: retail does `movs r5,r0` (result kept in callee-saved r5) and runs the `+72 +344 +12` chain through r0, then `mov r0,r5; bl F1; bl F2; mov r1,r0; mov r0,r4; bl T`. Ours keeps `r` in r0 (plain `cmp r0,#0`), runs the chain through r1 and never copies. Hoisting `o` before the call, `F2(F1(r))` split into locals not yet tried beyond one variant.
- Lead: r is probably used after the chain in a way that forces the copy (e.g. `r` also passed to `F1` after another call, or chain computed via a call); try a local used after F1.

## DllField@0x9aa14 / 0x9ad24 / 0xa0290 / 0x923f0 / 0x5e098 / 0x5e030 / 0x5aa34 -- ldm/ldrd/writeback variants, volatile-load retry 2026-10-16

All have the arg-load-width pattern PLUS a multi-word load instruction retail uses to fetch the args:
- 0x9aa14 (0x3c): retail `add r1,r1,#4; push; ldm r1,{r4,r5,r6}; bl; ...; sxtb r3,r6; sxtb r2,r5; sxtb r1,r4` then `H(*(mgr+12->+28)->* + 0x14cd0, ...)`. A plain `struct {int x,y,z;}` copy of `b+1` gives the exact body but each field is read as `ldrb` (armcc folds the narrowing into the load; 0/15 only because of the prologue shift). `volatile struct` copy / `volatile int*` give three separate `ldr`s, never `ldm`. Mixed plain-struct-read + volatile narrowing read not found (the narrowing cannot be separated from the load without a store to a volatile).
- 0x9ad24 (0x5c, `ldm {r4-r8}` of 5 args): same.
- 0x923f0 (0x38, `ldrd r0,[r1,#4]; and r5,r0,#255; and r4,r1,#255`): plain `long long v = *(long long*)(b+4); (int)v&255, (int)(v>>32)&255` gets two `ldrb` (folds); `volatile long long` gets `add r0,r1,#4; ldrd r2,[r0]` hoisted before the push (the ldrd imm form is not used for volatile) -- 0/14.
- 0xa0290 (0x2c): retail `ldr r3,[r4,#4]!` (pre-index writeback) then `ldr r0,[r0,#20]`, then `ldr r1,[r4,#4]`. `volatile int *p = b; x = *++p; y = p[1];` gives 8/11: no writeback, and `ldr r1,[r4,#8]` precedes the `ldr r0,[r0,#20]`; non-volatile first read folds to `ldrh`.
- 0x5e098 / 0x5e030 (0x38): `ldm r4,{r1,r2}` + `and r1,r1,#255` (callee gets args[1]&255, args[2]); not retried beyond the earlier note (volatile struct -> two ldr).
- 0x5aa34 (0x24): `ldm r1,{r0,r5}; and r4,r0,#255`, same.
- Lead: needs a construct that makes armcc form `ldm`/`ldrd` for a pair of adjacent loads while keeping the narrowing at the use (e.g. a struct-by-value parameter / inline-asm barrier); none found.

## DllBattle event-trigger predicates -- the prologue-move-order residue (round b; ~45 functions across 0xa5000-0xb8000, plus 0xa7cd0 / 0xa7188 / 0xc1244)

- Scan: 226 non-matched `bl 0x6b91c` callers in 0xad000-0xb8000 alone; the c-only / (d,c) shapes matched (see the batch tables). Everything that must save TWO args across the 0x6b91c call (b+c or c+d; `push {r4,r5,r6,lr}`) fails on the SAME two words and nothing else: retail moves the LOWER arg register first (`mov r5,r1; mov r4,r2` / `mov r4,r2; mov r5,r3`), armcc always emits the higher one first. The register assignment itself is already right.
- 0xad000-0xb8000: c AND d saved — 0xb67b0 13/15, 0xb6e80 17/19, 0xb47dc 14/16, 0xb6ecc 14/16, 0xb57f8 18/20, 0xb73c8 19/21; b AND c saved — 0xb76c8 14/16 (`if (g(3)==c && h(b)) post(83,1)`), same family 0xb0e94, 0xb0b4c, 0xae96c, 0xb1314 and ~20 more.
- 0xa5000-0xad000: ~25 functions — 0xa60c8 / 0xa67b4 / 0xa6924 / 0xa6a64 / 0xa6d98 / 0xa7130 / 0xa7788 / 0xa8530 (store-struct-then-0x67b48 family, 0x69f7c alloc), 0xa64bc / 0xa629c / 0xa6b90 / 0xa80a8 / 0xaa608 / 0xab074 (0x544a0 lookup + u16@12 compare), 0xa6504, 0xa79d4, 0xa71f4 (r4=c vs r0=b ordering), 0xa85cc / 0xabb48 / 0xacbc8 / 0xacc94 (c,d), 0xa7e18 (a,c). 0xaa608 also compiles `v==380||v==381` to `mvnne r1,#380` where retail has `subne #256; subsne #125` (a different range-test source shape).
- Tried on 0xb67b0: local copies of c/d in either order, a struct pointer, nested ifs, unsigned params, an int return, hoisting the d load before the g call (changes the code). None change the move order.
- 0xa5d60 (and siblings 0xa6c74, 0xa6ce8, 0xa7344, 0xa7aa0 at 0x54; 0xa7e18 at 0x70): `if (query(3)==c && query(58)==K) post(37, h(*(u16*)(a+56), 3))` stays 8/21 — retail saves `mov r4,r0; mov r5,r2` (a first, then c) and later reuses r5 for the literal 3 (`mov r5,#3; ldrh r0,[r0,#56]; mov r1,r5`); armcc puts c in r4 and a in r5. Tried a static helper, a local `k = 3`, a struct, a helper returning the u16.
- Lead for next time: the retail source probably had a different arg-forwarding shape (an inlined helper taking (b,c) / (c,d) in order), or the odd `mov r5,#3` implies a source-level variable holding 3 allocated into c's dead register — not found.
- Not attempted (bigger bodies): 0xadce8 (ldrh+cmp `?:` literal-pool pick), 0xb5574, 0xb79c0, 0xb67ec / 0xb68c4 style d-writers, the loop at 0xb6438, the VFP float-scaling pair 0xa7230 / 0xa7598.

## DllBattle pool-pad candidates that did NOT match (round b)

Scan method: a word-level scan of DllBattle .text for `zero word right after a terminator, followed by a
pc-loaded pool word` gave 111 start candidates; the short ones with the trace prefix
(`k=..; if (c->v != k) c->v = k;`) were tried. 8 landed in `poolpad/` (see poolpad-1..7 and 0x71208 notes).
Useful finding: the same `int k` prefix gets k->r2/old->r0 in some bodies and the retail k->r0/old->r2 in
others; no source-level spelling was found that flips it (int/unsigned/old-local/`k != c->v` all tried).

- 0x3981c (0x58): `if (f1(o,0)) { f2(o,0,0); f3(o->p190+0x280); return 1;} return 0` with the trace on arg2 (c=r1).
  Everything matches except the head allocation (retail `ldr r0,=k; ldr r2,[r1,#108]; cmp r2,r0`, ours k=r2/old=r0).
  Same register flip blocks 0x3cb48 and 0x724bc (same body with an `if (o->b6b8 == 255) return 1` front).
- 0x3badc (0x40) / 0x3f274 (0x40): `f1(p->q65c, 3); f2(p->q65c, 2, 1)` tail call; armcc emits `mov r2,#1; mov r1,#2`,
  retail `mov r1,#2; mov r2,#1` (tried int/uchar arg types, C++ member call, local copies).
- 0x3cbb4 (0x78): body matches (vcall slot 4, f1(0)/f2 pairs) but r5/r6 swapped (retail zero=r6, p+0x230=r5);
  naming the zero as a local `z` flips it but then reorders the head (`mov r3,#0` / ip use).
- 0x3ea10, 0x3f34c, 0x410f4: use `ip` as the trace pointer (in_ip); not attempted.
- 0x73dec / 0x74c08: inlined dead-code accessor chain (`cmp r0,#0` after `mov r0,#1`); not attempted.
- DllField: the same pad-word scan only yields mid-function pools (0x9ac4, 0x10a84, ...) or big functions
  (>= 0xb0 bytes); no new short candidates besides the three already in poolpad/.

## DllBattle 0x50-0xa0 tier -- unmatched (round b)

Candidate re-derivation: 777 push..pop-bounded functions of 0x50-0xa0 bytes not in matched/ or attempted.md (207 below file offset 0x30000). The large clusters are (a) ~330 "battle event trigger" predicates (`bl 0x6b91c` query(N) == c ...; tail-call `b 0x4ce84`/`0x658d4`) at 0xa5000-0xca000 and (b) ~60 script-command state handlers (`switch (*state)` 0/1/2/3, `ldr r1,[r2]; and r4,r1,#255; cmp #24` slot lookups).

- **Event triggers that save BOTH b (r1) and c (r2)** (0xa7cd0 0x64, 0xa7188 0x70, 0xc1244 0x68, 0xa6504 ...): body is 23/25 or 8/28 correct but retail emits the prologue moves `mov r5,r1 ; mov r4,r2` (b before c; in 0xa7cd0 `r5=r3, r6=r1, r4=r2`) while armcc here emits c before b (`r4=r2 ; r5=r1`). Allocation (b->r5/r6, c->r4) is identical, only the move order differs. Tried: operand order of the compare, early-return form, nested ifs, local copies of b/c/d, param type changes (unsigned/void*/const/extra arg). No natural form found. Siblings that save only c, or only (d,c), match fine (0xc489c, 0xc3a64, 0xc349c, 0xc32e4, 0xac16c, 0xb8b84 saved).
- **Two-case state handlers** 0xc020 (0x54), 0x86b0 (0x58), 0x8f84/0x7814-style: `cmp r1,#0; beq A; cmp r1,#1; bne end; b B` -- 20/21 with `switch (*p) { case 0: A; break; case 1: if (g()) return 1; break; } return 0;` but armcc emits `beq B` for the second dispatch (retail `bne end; b B`). if/else-if and default/return forms are worse. Unknown source form.
- **0x2d67c (0x60)** dtor: 18/24, retail loads `o->q@360` into r1 before `mov r4,r0` and puts the pool vtable in r0; every ordering form (plain, `q` local, `vt` local) swaps r0/r1.
- **0x2ad20 (0x74)** 13/29: right body (`if (n<=i) err; p=o+i*14; hp = hp>=amt ? hp-amt : 0; if (p[290]) { p[280]=hp; f(h,i) }`, no `and` with if/else form) but retail has idx in r6/amt in r5 and the slot pointer in r0; armcc swaps r5/r6 and uses r3.
- **0x2dcc4 / 0x2dd2c (0x68)** `{u8 buf[]; flag[]; u16 h[]; u8 a[]; n; max}` append (`if (n<max) {...; n++} else err(0,0,0)`): 16-17/26, register assignment (retail n in r4, zero in ip, array bases r5/r1 hoisted) differs.
- **0x53dc (0x5c)**, **0x9888 / 0x8ab4 / 0xfec8 / 0xe940 / 0x7814 / 0xcb44** script handlers: retail `ldr r1,[r2]; and r4,r1,#255` (word load then and); armcc folds to `ldrb` -- same already-documented residue (see the 0xf420/0xca9c entry); 0x9888's `ldm r2,{r1,r5}` pair load is the extra tell.
- **0x2bb58 (0x60)**: retail hoists `mov r5,#0; mvn r6,#0; ldr r7,=msg` before the call and pushes r4-r8 (inlined multi-assert shape); simple form is 21 words vs 24.
- Functions containing relaxation nops (0x8f84, 0xd3d0, 0x2baf0, 0x2b808, 0xbe48 ...) skipped as relaxed-only territory.

## DllField GetMgr (`0x16c0`) callers, file offsets 0x98000-0xa6000 -- residues (round b)

NEW LEVER (fixes the whole "arg-load width" residue class in `attempted.md`, e.g. 0x93d10 / 0x948f0 / 0x93b30 / 0x94d18 / 0x9aeb8):
declare the script-command args pointer `volatile int *a` (or read the field through `*(volatile int *)`). armcc then keeps a full
`ldr` and emits the separate `uxth` / `sxth` / `and #255` that retail has, instead of folding the narrowing into `ldrh`/`ldrb`.
Tagged `Style: trick(volatile-load)`. 0x9aeb8 (`& 15` ldrh) matched via `*(volatile unsigned short *)(p + 4) & 15`. The other
handlers listed in that attempted.md entry (outside 0x98000-0xa6000) should be retried with it.

Not matched (left for later):
- 0x9e550 / 0x9e74c / 0x9e3bc (0x44): `H(base, a[2]&255, a[1]!=0)` etc. Body right (volatile-load); only the callee-saved r4/r5 assignment is swapped
  (retail first-computed flag -> r5, ours r4). Declaration/compute-order variants tried (4); same swap class as 0x9ddc8 and 0x9d970.
- 0x9d970 (0x64): virtual call slot 13 on `F590(8)`'s result; body right, r5/r6 swapped (obj/sub-object).
- 0x9ddc8 (0x50): retail pushes `{r3-r7}` (two spare callee-saved regs) -- ours `{r3-r5}`; size/prologue mismatch.
- 0x9de9c, 0x99810 family (0x99930, 0x99ea0, 0x9accc, 0x99ef8): retail has `mov r0,rN; ldr r0,[r0,#off]` redundant moves (inline-member-on-this shape); plain C folds them. Not tried in C++.
- 0xa0b14 (0x64): stack-struct init + 3 calls; right body, retail schedules `mov r2,#37` after the pointer-chain loads and keeps the chain temp in r0 (ours r1).
- 0x9aa14 / 0x9ad24 (`ldm r1,{r4,r5,r6}` of 3-5 args then `sxtb`): folds to per-arg `ldrb`; volatile struct gives plain `ldr`s, never `ldm`.
- 0xa0290 (`ldr r3,[r4,#4]!` writeback + two uxth): volatile gives `ldr`s without the pre-index writeback (5 diffs).
- 0x9e844 (0x38): `r = H(..); if (r<0 || r>3) r = 0; return r;` retail keeps two signed compares + branches; armcc folds to unsigned `cmp #3; movhi/movcs` for every phrasing.
- 0x98c6c / 0x993dc / 0x99868 / 0x9a7cc / 0x9ff88 / 0xa0c74 and others whose disassembly shows `nop {0}`: relaxed-only (branch-relaxation nops), not matchable under the strict verifier.
- ldrd-pair handlers (0x9e074, 0x9e518, 0x9e790, 0x9eefc, 0xa0030, 0xa1238, 0xa1334, 0xa137c, 0x9966c): `ldrd` of two adjacent args -- known ldrd residue.
- 0x9a618 (pool-register), 0x9d8f4 (zero-register), 0x98158/0x9844c/0x986c0/0x9870c (not looked at): untouched this round.

## DllBattle script-command handlers 0x7a444 / 0x84af0 / 0x7ade8 / 0x8711c / 0x88130 / 0x83888 / 0x85080 / 0x7b5c4 (round b, push 0x14-0x80 scan of 0x60000-0x90000)

Family: `fn(ctx, obj[, args])` handlers that read script arguments through the
three read veneers (0x504 int, 0x1ab4 vec3/word, 0x1acc float) with `rd(a, &v, 1)`.
15 of this family matched (see matched/DllBattle/0x89aac ... 0x853a0). Useful
recipes found: the pointer-table-by-script-index heads need a redundant `O *q = o;`
copy (and `int x = v;` before the member load) to get retail's r4/r5 assignment
and `[sp]`-before-member load order (trick(redundant-local)); a block-scoped
`float z = 0.0f;` makes armcc load the zero pool constant first.

Not matched:
- 0x7a444 (0x30) `rd; fnB(b, v != 0)` and 0x84af0 (0x38) `fnB(b, v != 0, v != 0)`: retail
  does `ldr r0,[sp]; cmp r0,#0; movne r1,#1; moveq r1,#0; mov r0,r4; bl` (flag computed in r0/r1
  BEFORE `mov r0,r4`, both movne/moveq); ours loads into r1, emits `mov r0,r4` early and only
  `movne r1,#1`. Tried `!!v`, `?:`, if/else, `(unsigned char)`, C++ bool param, local `f`: all 5/12.
- 0x7ade8 (0x48): `rd; rd; if (b->c1858) fnC(b->p296)`; retail has `mov r1,r4` before the `cmp`
  and `ldrne r0,[r1,#296]` (conditional ldr+bl, no branch). 1-arg form is 12/18 (missing the mov r1,r4);
  2-arg `fnC(p, b)` forces a branch (12/18 too).
- 0x8711c (0x78): vec3 initialised from pool {0.2f,-0.45f,-0.55f} via `vstmia r0,{s0-s2}` with r0=sp
  (retail `mov r5,r0` first). Brace init / member stores / inline setter all give an `ldm/stm` copy from
  the pool instead (0-1/30). Tail (mode==1 && flag -> negate z -> setter) is the matched 0x82cb4 shape.
- 0x88130 (0x70): same as matched 0x853a0 but each iteration re-checks `k < o->n[i]` and null-checks the
  object. Best 22/28: retail keeps row pointer in r6 / count in r5, ours r5/r6 swapped; tried q=o copy,
  row struct pointer, if/continue/ternary forms.
- 0x83888 / 0x85080 (0x84): two vec3 reads, then per object in the row copy them to +284/+296 (+260/+272).
  retail uses two separate dest pointers (`add r6,r2,#284; add r2,r2,#296; stm; ...; stm`) and the count in r1;
  ours uses `stmia r1!`. 20/33 best (explicit `&x->a` locals); struct copy and per-field forms worse.
- 0x7b5c4 (0x40): relaxation nop before a tail `bl` (known nop residue).

## DllField GetMgr (0x16c0) callers 0x90000-0x98000 -- residues — **mostly RESOLVED 2026-10-16 (round b)** by the volatile-read lever (22 of these matched; leftovers: `0x90c00`, `0x92424`, and the `ldm`/`ldrd`/writeback group — see the entries above) (round b)

Matched this round: see pending/matched-DllField-0x9*.md (29 functions).

Stuck (all one instruction off unless noted):
- ldrb/ldrh narrowing family (retail `ldr r,[a,#4]` + `and #255` / `uxth` / `sxtb` at use; armcc narrows the load to ldrb/ldrh whatever the cast form -- &255, (unsigned char), (signed char), &0xffff, int local first): 0x9515c, 0x923f0, 0x93d10, 0x948f0, 0x928f0, 0x93b30, 0x94d18, 0x97ee4, 0x916a8, 0x922bc, 0x947ec, 0x93cac, 0x92d34, 0x91330, 0x90b24, 0x92424, 0x97a74, 0x95df0, 0x928ac, 0x94894, 0x9484c, 0x90c00 (every other word matches).
- Large-offset byte access (0x1539c via `add #0x15000; add #0x39c; ldrb/strb [r0]` -- armcc folds 0x39c into the ldrb imm for every C form tried, incl. nested structs): 0x93cec, 0x93d10, 0x92e28/0x94cb8 (0x1539d, literal-reg form, 7/9 words only).
- Bitset set/clear (0x93468 `|=`, 0x955cc `&=~`): 14/17, only the shift-amount/constant registers swap (r2<->r3); tried 6 spellings (%8, &7 temp, unsigned char m, reordered |).
- 0x9056c: r4/r5 swapped (self and mgr roles); 0x94580: retail does `ldr r0,[..#344]; mov r6,r0` before the a[1] load (5/21 words).
- Struct-on-stack out-params 0x9713c / 0x971e4: 21/27, retail loads `[m+72]+252` into r2 first then `mov r0,sp; ldr r1,[r2,#124]`, ours folds into r0; the D() arg setup (`add r2`/`add r1` before `mov r0,sp`) is also reversed.
- 0x97250: load ordering (m=[p+12] first, then a=[p+28]) not reproduced (15/23).
- nop-window family (`bl 0x200` + `nop {0}`; relaxed-only): 0x925f0, 0x94ec0, 0x94f04, 0x94f48, 0x959bc, 0x968e4, 0x93d90, 0x94f8c, 0x925a0, 0x93058, 0x969fc, 0x94e68, 0x915f4.
- Not tried: 0x926b4, 0x927b0, 0x94c58, 0x920dc/0x95a00/0x95c88/0x905c8 (operator-new constructors), 0x95fa4, 0x97acc, 0x9461c.

## DllField GetMgr (0x16c0) callers 0x50000-0x70000 -- residues (round b)

Cracked this round: `uxth`/`and #255` narrowing of a loaded word is produced by reading the argument through a
`volatile int *` (`unsigned short x = b[1];` -> `ldr r0,[r1,#4]; uxth r4,r0`; plain access gives `ldrh`/`ldrb`).
Style: trick(volatile). See the matched-DllField-0x59270 family. `b[1] & 255` also needs the volatile read AND the load
placed after the GetMgr call when retail does `mov r4,r1; bl; ldr r1,[r4,#4]` (0x5e068). Plain `b[1], b[2]` gives the
retail `add r4,r4,#4; ldm r4,{r1,r2}` (0x5c580).

Not matched:
- `ldrd r0,[r1,#4]` + two uxth/and (0x589fc, 0x58af4, 0x5ab58, 0x5a9e8, 0x5e808, 0x5ef18 struct-by-stack): best 1/11. `unsigned long long v` loads
  give `ldrh,ldrh` (or `add r0,r1,#4; ldrd r2,[r0]` with a volatile long long: wrong pair and extra add); `__packed` long long /
  struct, `struct{unsigned a,b;}` copy (volatile or not) never emit `ldrd r0,[r1,#4]`.
- 0x5e098 / 0x5e030 (`ldm r4,{r1,r2}` then `and r1,r1,#255`): armcc folds the `&255` into `ldrb`; volatile struct copy gives two ldr.
  Best 5/14 (non-volatile) / 11/14 (volatile struct, ldr,ldr instead of ldm).
- 0x588b0: retail `uxth r1,r4` sits AFTER the 4-load chain; armcc hoists it before (8/12, also with the chain in a local).
- 0x5cfc8 (sxth, same scheduling: `mov r1,r4` after the chain; 8/11).
- 0x5aa34 (`ldm r1,{r0,r5}; and r4,r0,#255`; 5/9: struct copy gives ldrb+ldr).
- 0x5bb78: `r = (r != -1)` always compiles to `adds r0,r0,#1; movne r0,#1` instead of retail `cmn r0,#1; moveq r0,#0; movne r0,#1`
  (12/16; tried ?:, early return, if/else).
- Not tried: 0x5bdf4.. (18-28-word bodies), jump-table dispatchers, vldr getters, 0x5c938/0x5d204 (bounds check + error call 0x200).

## DllField 0x50-0xa0 tier sweep, round b -- shapes NOT matched

Scan recipe: bound functions at `push {..lr}` and in-module `bl` targets, size 0x50-0xa0, exclude matched/ and
the GetMgr callers / other agents' ranges, drop any body with a post-link `nop` word (relax-only class, e.g. the
0x11fa0 / 0x11ff0 / 0x12ccc / 0x12d1c destructors that contain `nop` before `bl`).

Useful new lever: the `ordering-local` form (`O *a = s->a; void *vt = VT; s->vt = vt;`) cracks the
"member load before vtable pool-load" dtor shape (DllField 0x3954c + 4 siblings matched). Switch with
`case 16: case 56:` reproduces a three-compare `cmp #16 / #22 / #56` chain (an `||` chain gives `cmpne`).
An `unsigned` loop counter fixes `bcc` vs `blt`.

- DllField@0x14fa0 (0x50), 0xe7168 (this-0x74 variant, 0x50), 0x12040 (0x5c, +0x80/+0x89): `if (chk(a, 0x60, 0) == 0) return 1; if (this->o && !this->b) o->vt[2](o); return 0;`
  Best 2/20: armcc emits `mov r3,r1` BEFORE the push and passes r3 as arg 0; retail does `mov r4,r0; mov r0,r1` after the push (a is forwarded straight). Tried int / void* arg, C and C++.
- DllField@0x394f4 / 0x396d0 / 0x3a644 / 0x3a908 / 0x3cd5c (0x58, tail-calling `b 0x1c8` dtor variants): retail stores the vtable via the ORIGINAL r0 (`str r2,[r0]` after `mov r4,r0`), loads the member into r1 and calls through r2 (`ldr r0,[r1]; ldr r2,[r0,#4]; mov r0,r1; blx r2`). The ordering-local form that solved the non-tail sibling gives r4-based stores (4/22).
- DllField@0x47fec (0x5c): `if (cur != v) { switch (h->mode) { 1: h->o1->vt[18](o1); 2: h->o2->vt[11](o2); } cur = v; }` (h = this+0x1f8, cur at +0x218). Callee-saved swap: retail keeps this in r5 / v in r4 (and loads the mode into r2 before computing h into r0); armcc picks r4/r5 the other way and loads mode into r0. 7/23.
- DllField@0x7ee64 (0x5c): `t = this->tbl; hi = t[idx].b; if (t[idx].a & 0xff) err(0,0,0); go(this->base + hi*64, d, 0, c)` (tail call). Only the callee-saved assignment (r6=c first, r7=d later) and the `ldrh [r0, r2]` with r2 = 2 + idx*4 addressing differ. 3/23.
- DllField@0x4f214 (0x54): `c = chk(b); set(p, c ? 32 : 33); go(this, b, 1, 1, (signed char)(c ? 11 : 8)); return 1`. Retail keeps an `sxtb` of the value (r5) before the stack-arg store; armcc folds the cast away because it sees the constants. Interleaving of the conditional moves is already identical (11/21).
- DllField@0xe1bf0 / 0xe1f8c (0x50, script-command handlers, constant 2 / 3): retail pushes r4-r8 and keeps the constant in r7 (`mov r7,#2` early), non-tail final call; plain code gives a 4-register push and a tail call. Same class as the arg-load-width script handlers in attempted.md.

## DllBattle 0x30000-0x60000 `push` sweep, round b -- unmatched leftovers

Round matches: 0x520e4, 0x4c7d0, 0x4c0f8, 0x52774, 0x527b8, 0x4d184, 0x544a0 (see matched-*.md). Candidate list was re-derived by bounding push..pop/bx lr over the .text window (0x14-0x80 bytes), minus matched/attempted.

- 0x3e920 (0x40): trace-style prefix `k=0x97d; if (G.v != k) G.v = k;` (global struct +108) then `fnC(o->p400, b, 0); o->p400->w1096 = c`. Code words match modulo registers: retail loads &G into r2 AFTER `mov r5,r2` and keeps r3 for the old value; armcc hoists `ldr r3,[pc]` above the push-saved moves and uses r3 as the address (10/16 best, `struct T *t = &G; int k=..` form). Pool order also differs (retail: &G first, K second).
- 0x3bdd8 / 0x3e7a4 / 0x3981c / 0x3b534 (all start at 4 mod 8): same trace prefix on an ARG pointer (`c->v` at +108 = arg3 or arg2). Code words match 9/12 but ONLY the literal pool differs: retail has a 0 pad word before the constant (function at 4 mod 8, pool 8-aligned); standalone compile has none. Same verifier limitation as the 0x71a2c note in attempted.md -- would match if the verifier tolerated pool-alignment padding.
- 0x551d4 (0x38): `if (fnA(*o,2)) return 0; return fnB(o,13) != 0;` 12/14; only diff is `mov r0,r4; mov r1,#13` (retail) vs `mov r1,#13; mov r0,r4` (armcc). Tried if/ternary/r-var forms and a C++ member call.
- 0x397b8 (0x44): vcall slot 25 (this,0) then slot 32 (this,r); return r==175?1:r==102?2:0. Retail keeps this in r0 and uses r1 as the vtable-load scratch (`ldr r1,[r0]; mov r4,r0; ldr r2,[r1,#100]; mov r1,#0; blx r2`); armcc always loads the vtable into r0 and needs `mov r0,r4` (2/17; prototyped casts, unprototyped `int (**vt)()`, C++ virtuals all give the same shape).
- 0x4bfd0 (0x30): `if (i>=7) fnA(0,0,0); return o->v[i]` (+28) 10/12; callee-saved swap r4/r5 (`mov r5,r0; mov r4,r1` retail vs reverse) -- the known 0x4cf8c class.
- 0x4b394 (0x54): 10-slot 3-byte record table at +0x120c; assert i<10, assert rec.a!=0, return rec.c. 15/21 with `p = o+0x120c` local; only r4/r5 swapped (retail: index r4, base r5).
- 0x4b3f8 (0x60): `r = fnX(o+0x120c, c); l = o->p3520; if (l->n < 10) { l->a[n]=a; l->c[n]=c(+21); l->r[n]=r(+11); l->n++ }` 6/24; retail keeps the call result in r0 and loads `l` into r3, armcc copies r0 to ip and uses r0 for l; also r4/r5/r6 assignment differs.
- 0x4e808 (0x4c): big-endian u16 reader {size,pos,data@8}: assert pos < size-1; v = *(u16*)(d+pos); pos += 2; return rev16(v). NEW: armcc has no `__rev16` intrinsic (emits a call) but `((v & 0xff) << 8) | (v >> 8)` on an `unsigned short v` compiles to a real `rev16 r0,r0`; `__rev(x)` is the word rev. Best 13/19 (pointer-local form): retail loads size first (`ldr r1,[r0]` before `mov r4,r0`) and uses `add r2,r0,#2 ; ldrh r0,[r1] ; str r2` ordering; armcc does `add r0,r0,#2 ; str ; ldrh`.
- 0x4c438 (nop-relaxation `bl; b; nop`), 0x52db0 (jump table + relaxation nops), 0x5c9b8/0x4cbe4/0x5c95c (VFP float math) not attempted.

## DllField@0xc2824 / 0xcd2dc (and deleting variants 0xc27ec / 0xcd2a4) -- dtor member-load-order residue

Module: DllField
Offset: 0xc2824 (siblings 0xcd2dc plain; 0xc27ec, 0xcd2a4 deleting variants)
Size: 0x34 / 0x38
Match: 8/13 best
One-line: dtor: vptr = pool addr; free member [+20] via import if non-null, zero it, return this.
Style: n/a
Note: retail order is ldr r1,[pool]; ldr r0,[r0,#20]; str r1,[r4]; cmp; ... zero via r1. Tried extern char VT[] (8/13: vptr store before member load), member-read-first local (7/13: pool ldr after member ldr, str after cmp). Known member-load-order destructor residue; also zeroing via r1 not r0.

## C++ inline-member sweep (2026-10-16, `armcc-r5-cpp1`) -- the DllField@0xb1bb8 mechanism applied to the other branch-vs-conditional-execution / scheduling failures

2026-10-16 C++ inline-member sweep (the DllField@0xb1bb8 mechanism). One NEW match came out of it:
DllField@0x2948c (matched-DllField-0x2948c.md; trick(cpp-inline-member)). The mechanism generalises as
"write the pointee-side logic as an inline member called on the loaded pointer", but only that one flipped.
Everything below stayed stuck; what was tried, so nobody repeats it:

- DllField 0xb6844 / 0xb76c8 (secondary-base dtor pair) — **RESOLVED 2026-10-16 as POOL-PAD** (`poolpad/`, see README 'Tool addition: --pool-pad'): with a volatile-stored first vptr (or a plain store)
  plus `Reset()` member the tail is exact; best 19/22 (plain `o->vt = v`, stores [56],[0],[80]) -- only the head
  `add r2,r1,#48` (retail keeps v+48 in r2 and v+32 in r0; ours puts v+48 in r0) and the pool `ldr` displacement
  differ. NEW FINDING: the pool displacement is NOT a code diff we can fix -- retail's function starts at 4 mod 8
  so the literal pool word is 8-aligned after a 0 pad word (ldr disp 40 vs our 36); the standalone verifier
  compile (text starts at 0) can never emit that pad. Same class as DllBattle@0x71a2c. Even a perfect head
  cannot reach a masked 22/22 for these two; needs a verifier option to tolerate pool-alignment padding.
  Tried head forms: locals q/t in both orders, typed `void **v` arithmetic (v+8, v+12), early `v+48`.
- DllField 0xbe3e8 / 0xb4e84 (`movs r1,r0`): inline member Go(a) {r=fa(m,a,1); if(r) fb(this,r);} in 8 phrasings
  (if-init, bool temp, extra copy, helper member taking r, volatile r, int/unsigned/char* typing): all 6/11 (cmp
  r0,#0 then mov r1,r0). 0xb0334 as a member: 8/17 and it never pushes {r4-r6}; retail keeps the result in
  r5 (callee-saved) across only loads, so some extra live value (r6 = 8-byte push alignment pad?) exists in the
  original that none of these reproduce.
- DllField 0xcae50 / 0xcb41c: member Go() on the object, plain/goto/shared-store/ret-variable/destructor-in-class
  forms: 19/27 plain, 25/27 goto-shared-store (unchanged). Retail `movne r0,#0; bne <final pop>` needs the
  return-0 path to share the final epilogue after `mov r0,#1`; no form produced it.
- DllField 0xde620 (record fill): inline member Init()/ctor/placement-new, volatile kind member (best 7/11,
  same as before), integer-address round-trip, identity-helper, reference alias: none make armcc compute
  `add r4,r1,#12` before the stores and reuse r1 for the constant.
- DllBattle 0x7198c / 0x73524: whole predicate chain as an inline member of the pointee, && / if-return /
  nested-if forms, volatile loads: volatile fixes `ldr` vs `ldrb` (9/16) but armcc still if-converts the second
  load (`ldreq/tsteq`); in 0x73524 the volatile load is if-converted too (4/16). No form blocks the conversion.
- DllBattle 0x78a24 / 0x77338 family: member Go() with m loaded in member scope, nested helper member, volatile m:
  all 4/9 (`ldr r2,[r4,#8]` hoisted before `mov r1,r0`; even volatile m is hoisted).

## DllBattle @ 0x4b82c (size 0xa8) -- not matchable as-is: relaxed-call nops
- What it is: 8-slot jump table (cmp r1,#8); slots select `this+0x104..0x10a` (7 bodies; computed as add #0x100 then add #N, then shared `ldrsb r5,[r0]; r0 = 12 - r5`); default calls err(0,0,0) with r5=0 -> returns 12. push {r4,r5,r6,lr} with r4=12/r5=0 initialised before the dispatch.
- Tried: plain switch with `p = this+N` per case and shared `v=*p`: armcc folds +0x100+N into one add, uses push {r4,lr} and r4 for the load; retail keeps add #256 and add #N separate and holds 12 and 0 in r4/r5. Also the default has `bl; nop; nop; b` (relaxation nops in the retail bytes) which is relaxed-only territory. Not a match candidate.
- Relocation-derived mapping: slot 0 -> default/err block, slots 1..7 -> this+0x104,0x105,...,0x10a in ascending order.

## DllBattle @ 0x6de1c (size 0x80) -- 30/32 with --mask-ranges 0x18:0x14, stuck on arg-copy order
- What it is: fn(k, flags, n): 5-slot jump table (relocs: slot0 n==0 ok, slot1 n<2 ok, slot2 n<3 ok, slot3 n==0 ok, slot4 unconditional ok; failed check or k>=5 -> err(0,0,0)); ok tail returns (u8)(((flags&1)?0:1) + (n<<1)).
- Best source: gotos `if (cond) goto ok; break;` per case, `default: break;` then `err(0,0,0); ok: if(!(flags&1)) return (unsigned char)(1+(n<<1)); return (unsigned char)(n<<1);`. Everything matches except the two prologue moves: retail `mov r5,r1; mov r4,r2` (flags first), armcc emits `mov r4,r2; mov r5,r1` (same registers, swapped order). Tried: local copies of flags/n in either order, static helper for the tail (stays inlined), `?:` vs if forms (those change the tail; the if(!(flags&1)) form is the one that fixes the eq/ne order).
- Lead: something that makes flags used/copied before n in the first basic block (not found).

## DllBattle @ 0x71250 (and its getter sibling 0x743a8, 7 slots) -- setter via kind()-switch, 14/29
- What it is: `mov r3,r0; push {r4,lr}; mov r4,r2; bl 0x620d4(this,..)` then 5-slot table (sub r0,#8; cmp #5) with bodies `strh r4,[r3,#0xf6..0xfe]; pop {r4,pc}` and default tail call `b 0x394` with zeros. The sibling at 0x743a8 is the ldrh getter (cmp #7, 6 bodies, one `mov r0,#6`).
- Stuck: retail keeps `this` in r3 (caller-saved!) across the `bl 0x620d4`, i.e. armcc believed the callee does not clobber r3 (intra-TU register-usage knowledge?). Our compiles always put `this` in r5 (push {r4,r5,r6,lr}). Tried extern callee, static noinline callee defined before/after: either inlines or ignores. The 0x620d4 callee is a normal function (mov r2,r0 ... uses r2 only).
- Table mapping from relocations (tools/jumptable_relocs.py): see the matched DllBattle 0x60e00 note.

## DllField@0xde620 (size 0x2c incl. pool word) -- record fill at out+12 (about 11 identical sites)

- What it is: `f(a, out)`: `r = out+12; r->kind@4 = 2; r->vt = VT (pool); r->id@6 = imp(22)`.
  Retail: `push{r4,lr}; ldr r0,=VT; add r4,r1,#12; mov r1,#2; strb r1,[r4,#4]; str r0,[r4]; mov r0,#22; bl imp; strh r0,[r4,#6]`.
- Best: 7/11 (volatile qualification of the kind store or of the whole record pointer, e.g. `((volatile R*)r)->kind = 2`, or reassigning the param `r = (R*)((char*)r+12)` + `*(volatile unsigned char*)&r->kind = 2`). Those fix the "push first, pool load later" ordering, but armcc still addresses the pre-call stores as `[r1,#12/#16]` and keeps `add r4,r1,#12` only for the post-call strh; retail stores everything via r4 and reuses r1 for the constant 2 (so `out` is dead right after the add).
- Tried (all 5-7/11): plain / pointer local / `R *r = out+1` / `&out[1]`; param reassignment; static inline init helpers (void and pointer-returning); inline C++ member Fill()/Fill2() incl. `volatile` member and volatile this-casts; placement-new ctor (own inline operator new); `char *p` cursor; kind-first vs vt-first store order; `R * volatile` pointer variable (0/11).
- Lead: need a form where the record pointer pseudo is materialised BEFORE the stores and not folded into base+offset. The inline-member trick that cracked DllField@0xb1bb8 (see matched-DllField-0xb1bb8.md) did NOT carry over here.

## DllField 0xbe3e8 (0x2c), 0xb4e84 (0x2c), 0xb0334 (0x44), 0xb6844 / 0xb76c8 (0x58) -- stuck

- `0xbe3e8` / `0xb4e84` (identical bytes bar branch targets): `push {r4,lr}; mov r4,r0; r0=this[0x24]; r2=1; bl fa (r1 = incoming 2nd arg); movs r1,r0; beq ret; mov r0,r4; pop {r4,lr}; b fb; ret: pop {r4,pc}`.
  Best C `void Fn(char*s,int a){int r=fa(*(void**)(s+36),a,1); if(r) fb(s,r);}` gives 6/11: armcc emits `cmp r0,#0` + later `mov r1,r0` instead of the retail `movs r1,r0` before the branch. Tried: int/pointer result, reusing the param (`a = fa(..)`), `if ((r = fa()))`, non-void `return fb(s,r)` / `return r`. None change it. Lead: some construct that makes armcc hoist the `mov r1,r0` above the test (the movs form) -- maybe the result is stored to a second live variable, or the true signature has fb taking more args.
- `0xb0334` (0x44): same `movs r5,r0` shape (pushes {r4-r6}, so something extra is live): `r=fa(this[0x560]); if(r){o=this[0x5a4]->[0x24]; if(o) tail vt[1](o, r, 0);}`. My C compiles to `push {r4,lr}` + `movs r1,r0` (8/17).
- `0xb6844` / `0xb76c8` (0x58 incl. pool word) — **RESOLVED 2026-10-16 as POOL-PAD** (`poolpad/`, see README 'Tool addition: --pool-pad') (head fixed by declaring `char *b = v + 0x30;` before `char *a = v + 0x20;`): secondary-base dtor: `this[56]=v+32; this[0]=v; this[80]=v+48; fa(this[92], this+80); this[0]=VT; if(this[32]) fb(this, this+32); return fc(this+8)-8`. The tail (17/22 after the `q`/`t` locals making the `addne r1,r0,#32; blne` form) matches, but the head allocates `v+48` into r0 where retail uses r2 (and keeps r0 = v+32 live), and the pool offset differs by one word. Stores must appear [56],[0],[80]; locals for v+32/v+48 swap the order wrongly.

## DllField script-command handlers whose only miss is the arg-load width — **mostly RESOLVED 2026-10-16 (round b)** by the volatile-read lever (22 of these matched; leftovers: `0x90c00`, `0x92424`, and the `ldm`/`ldrd`/writeback group — see the entries above) (0x93d10, 0x948f0, 0x93b30, 0x94d18, 0x9aeb8) -- 2026-10-16

Handlers `int Fn(int self, int *args)` reading `args[1]` and narrowing it. Retail does a full `ldr rX,[r1,#4]` then `uxth`/`sxtb`/`strb`;
armcc always folds the narrowing into the load (`ldrh`/`ldrb`), for every form tried (`unsigned short i = a[1]`, `int v=a[1]; short i=v`,
`a[1] & 0xffff`). Same residue class as ldrd/ldrh narrowing. Each is otherwise 11/12 or 12/12 with the right body:
- 0x93d10 (0x2c): `*(char*)(*(char**)(*(char**)(*(char**)(mgr+12)+28)) + 0x1539c) = args[1]; return 0` (local `char *q = b + 0x1539c` reproduces the split `add`s); miss ldr vs ldrb.
- 0x948f0 (0x2c): `return ((int*)(tbl396 + (u16)args[1]*28))[5]`; miss ldr vs ldrh.
- 0x93b30 (0x30): `H1(mgr[11], (u16)args[1]); H2(mgr); return 0`; miss ldr vs ldrh.
- 0x94d18 (0x30): `H(b + 0x14cd0, (signed char)args[1])`; miss ldr vs ldrb.
- 0x9aeb8 (0x30) sibling of matched 0x9af9c: `& 15` form emits `ldrb` instead of retail `ldrh; and #15`.
Also stuck: 0x98bc0 (0x2c, uxth scheduled before `mov r3,#0`), 0x9d8f4 (0x2c, zero-register: retail materialises `mov r1,#0` after the adds), 0x9a618 (0x2c, pool-reg allocation differs).

## DllBattle@0x76398 (0x3c), 0x71a2c (0x2c), 0x7198c (0x40), 0x73524 (0x40), 0x77338 / 0x77428 family

- 0x76398: bounded array push (`if (s->n >= 6) f394(0,0,0); s->a[s->n] = v; s->n++`, struct {u32 a[6]; u8 n}). Best 13/15 with `i = s->n; s->n = i+1; s->a[i] = v;`; only diff: retail `add r1,r0,#1; strb r1`, ours `add r2,r0,#1; strb r2` (register-allocation class).
- 0x71a2c — **RESOLVED 2026-10-16 as POOL-PAD** (`poolpad/`, see README 'Tool addition: --pool-pad'): `k = 0x12c6; if (c->v != k) c->v = k; return f394b8(a, k, c) != 0;` (c = arg3, +0x6c) matches all 8 code words; the only diff is the literal pool — retail has a 0 pad word then the constant (function starts at 4 mod 8, pool 8-aligned), our standalone compile has no pad (10 vs 11 words). Would be a match if the verifier tolerated pool-alignment padding.
- 0x7198c / 0x73524: `&&` predicate chains (`p && f74420(p,13) && !(p->a&7) && !(p->b&7)`); armcc emits conditional execution (`ldrbeq/tsteq/moveq`) and narrows `&7` loads to `ldrb`; retail uses plain `ldr` + `bne/beq` branches (branch-vs-conditional-execution class; best 8/16, 4/16).
- 0x77338 / 0x77360 / 0x77398 / 0x773ec / 0x77428 family (`f5dc68(this,*k)` then `f707d4(this->+8, r)` ...): retail does `mov r1,r0` then `ldr r0,[r4,#8]`; armcc emits `ldr r2,[r4,#8]` before the mov (scheduling class; the same cluster as 0x78a24). Best 4/10; 0x77428 also starts with `mov r2,r1`.

## DllBattle conditional-store chain (0x41f94, 0x434ec, 0x439d4, 0x3c960, 0x43958)

- What it is: `if (g.v != K) g.v = K;` with `g` a pool-address global, `v` at +0x6c.
- Tried: plain, local `k`, local pointer, `old` temp, `0x5d3 != g.v`, an extra unused int arg, `return 0` / `return 1` variants.
- Stuck on: best 5/8 — retail loads the ADDRESS into r1 first and the constant into r0 (pool order addr, K); armcc always puts the constant in r0 and the address in r1, or swaps the pool order (3-4/8).
- Lead for next time: the real function probably has a different arg / return shape or an inlined context.

## DllBattle vec3 copies via ldm/strd (0x126ec, 0x12864, 0x128d4, 0x12a00, 0x12ae0, 0x12b0c)

- What it is: `ldm r1,{r2,r3,ip}; str ip; strd r2,r3`.
- Stuck on: 2/6 — armcc emits `ldm r1,{r1-r3}; add ip; stm` for a struct copy and per-field ldr/str for field copies (the known field-copy residue).

## DllField GetMgr (`0x16c0`) family -- shapes NOT yet matched (2026-10-16)

- Scan found 626 callers of `0x16c0` outside `matched/`; 46 matched this batch. Only callers up to 0x38 bytes were shaped; ~470 larger callers are untouched.
- Seen but not tried (same chain idiom; offsets are examples): `uxth`-arg + helper call (`0x865cc`) + `cmp` + conditional tail `b` — `0x59270`, `0x592a8`, `0x592e0`, `0x5a6b4`, `0x5d698`, `0x92d34` (two nested calls, `beq` around a tail `b`); `ldrd`/`uxth` pair args with `push {r4,r5,r6,lr}` — `0x589fc`, `0x5ab58`, `0x58af4`, `0x5a9e8`, `0x95130`, `0x95f78`, `0x5e808`, `0x923f0`; jump-table dispatchers on `ldrb [..,#460]` (`cmp #9; ldrcc pc,[pc,r0,lsl #2]`) — `0x94d48`, `0x957fc`, `0x930ac` (need `--mask-ranges`); `vldr`/`vcvt` float getters — `0x92330`, `0x92370`; struct-by-sp arg (`strb r0,[sp]; strb r1,[sp,#1]`) — `0x5ef18`; `mov r4,r0` (self kept) + 3-4 call sequences — `0x945e4`, `0x95190`, `0x9752c`, `0x91330`, `0x91364`.
- `0x6d64c` itself (a ctor/dtor body storing vtable pointers at +0 and +56) is not attempted.

## DllBattle@0x4bed4 (size 0x20)

- What it is: `return fnA(o->m@0x5c, k, 16) == 5;` (`k` passes through in r1).
- Tried: plain `void *` member, typed struct, int member, unprototyped `fnA()`, local `n = 16` / local `m`, if/else return form.
- Stuck on: 6/8 — retail emits `mov r2,#16` BEFORE `ldr r0,[r0,#92]`; armcc always loads r0 first.
- Lead for next time: the retail order suggests the 16 is materialised before the member load (maybe an inlined accessor); untested: the constant via a static / enum, or `k` reassigned.

## DllField@0x4a478 (size 0x20; siblings 0x4a458, 0x530ec) -- dtor stage: install vptr, call import, return this

- Retail: `push {r4,lr}; mov r4,r0; ldr r0,[pool vptr]; str r0,[r4]; mov r0,r4; bl; mov r0,r4; pop`.
- Tried: `a->vp = vt; Imp(a); return a;` and a local-vt variant -> 0/8; armcc loads the pool vptr into r1 BEFORE the push where retail loads it after `mov r4,r0`, reusing r0 (vptr scheduling residue).
- Lead for next time: none yet.

## DllField@0x58ba0 (size 0x20; sibling 0x58bc0)

- What it is: `Imp(a, (short)b[1], 0, 9, 0)` — retail `ldr r1,[r1,#4]; mov r2,#0; mov r3,#9; str r2,[sp]; sxth r1,r1; bl`.
- Tried: int / short-param prototypes.
- Stuck on: 4/8 — armcc narrows the int load + `sxth` to `ldrh` (the ldrd/ldrb-style narrowing residue).
- Lead for next time: none yet.

## DllField@0xcae50 (size 0x6c; byte-sibling 0xcb41c: constants 75/77/78, bits 0x100000/0x200000)

- What it is: state handler — `fa(this->[68], 1); if (fb(p+184) && (r = fc(p+184)) != 83) { if (r == 85 || r == 86) flags(+164) |= 0x1000000, return 1; else return 0 } else flags |= 0x2000000, return 1`.
- Tried: goto-shared-store variant (`int v; ... v = f|0x1000000; goto st; ... st: *(int*)(s+164)=v; return 1;`) — best 25/27; the plain nested-if version (store duplicated per arm) is 19/27; an `int ret` variable pushes to r5 (worse).
- Stuck on: retail's `return 0` path does `movne r0,#0; bne <final pop>` (shares the epilogue pop at the very end); mine emits `movne; beq st; pop {r4,pc}` (inline pop for the 0 path) — two words differ (branch-vs-conditional-execution class).
- Lead for next time: none yet.

## DllBattle "getter via 0x5dc68 / 0x707d4" cluster (0x7b20c `ldrh` +0x160, 0x835f8 `ldrh` +0x158, 0x7b2e4, 0x7bec4, 0x81544, 0x81568, 0x81594; tail forms 0x7a11c, 0x7b320, 0x78488, 0x7957c)

- What it is: `fn(o, *p)`, then `fb(o->+8, result)`, then read a field of the result (same shape as the logged 0x78a24).
- Stuck on: best 4/10 — retail loads `o->+8` after the first call straight into r0 and moves the call result into r1; every phrasing hoists `ldr r2,[r4,#8]` before the call and adds `mov r0,r2`.
- Lead for next time: the second call's first argument probably isn't a plain field load of `o` (accessor or different arg order).

## DllBattle small push-functions, 0x30000-0x60000 sweep (unmatched)

- 0x422e8 (0x24): `fnA(o->m@0xafc); o->y@0x876 = 0; o->x@0x874 = 1`. Retail `mov r1,#0; mov r0,#1; strb r1; strb r0` (zero in r1); ours reuses r0 — zero-register residue. Tried store orders / duplicate store.
- 0x4bb2c (0x30): `if (i >= 5) fnA(0,0,0); return o->v[i]` (u8 at +365+i). Only diff: prologue `mov r5,r0; mov r4,r1` vs ours `mov r4,r1; mov r5,r0` (10/12). Tried struct array and flat pointer index.
- 0x4cf8c (0x34): `if (i >= 7) fnA(0,0,0); o->v[i] = val` (int array at +28). Callee-saved assignment swapped (9/13) — val/o swap r5/r6.
- 0x4d318 (0x30): `return fnA(o,13) == fnA(o,14)`; o lands in r5 / first result r4 (ours) vs r4/r5 (retail), 8/12. Tried temporaries and an if-form.
- 0x48344 (0x38): `if (vt[38](this)) fnA(this->m@0xb8, v); this->b@0x4e = v;` 12/14 — retail loads the vfn into r1, ours r2; tried prototyped / unprototyped vt and a local fn ptr.
- 0x4344c (0x2c): **matched 2026-10-16** (11/11, natural) — written `if (r == 103) return 0; return r == 104;` (the `||` spelling gives `cmpne/movne`); forwarded int arg moves the vtable load into r2.

## DllBattle@0x2da0 (0x38) / 0x2dd8 (0x34) -- zero-register dtors

- What they are: dtor stages — install the vtable, free the member at +0x14 if non-null and zero it; 0x2da0 tail-calls a base dtor, 0x2dd8 returns `this`.
- Tried: the dtor recipe (`void *vt = VT; void *p = this->m; this->vt = vt; if (p) {free(p); this->m = 0;}`), which matches DllBag@0x13848.
- Stuck on: 12/14 and 11/13 — retail `mov r1,#0; str r1,[r4,#20]`, armcc `mov r0,#0; str r0,[r4,#20]`; a hoisted `z = 0` local is worse (5/13). Zero-register dtor residue (see the class entry).

## DllBattle@0x12f84 (size 0x48)

- What it is: dtor stage like matched 0x12fcc but with a tail `b` to the base dtor.
- Tried: the 0x12fcc shape plus a tail call; vt local dropped / vtable store hoisted (3 variants).
- Stuck on: 2/18 words (candidate 17 words vs retail 18). Retail keeps the member in r1 and the vtable in r2 (`ldr r2,pool; ldr r1,[r0,#8]; mov r4,r0; str r2,[r0]`), vcall as `ldr r0,[r1]; ldr r2,[r0,#4]; mov r0,r1; blx r2`; armcc puts the member in r0.
- Lead for next time: none yet.

## DllBattle@0x78a24 (size 0x24; sibling 0x7b2e4, `ldrb [r0,#365]`)

- What it is: `r = A(o, *p); return B(o->field8, r)->u16 at +12`.
- Tried: `int r = fn5dc68(o,*p); return *(u16*)(fn707d4(*(int*)(o+8), r)+12);`, the nested-call form, a typed-struct form.
- Stuck on: 4/9..8/9 — armcc hoists `ldr r2,[r4,#8]` right after the first `bl` and adds `mov r0,r2` (10 words vs 9); retail does `mov r1,r0` then `ldr r0,[r4,#8]` straight into r0 after the call.
- Lead for next time: the second call's first arg is probably not a plain field load (an accessor / vt thing, or a different arg order).

## DllField@0x5c3e8 (size 0x1c; sibling 0x5c404 differs only in callee)

- What it is: `push {r4,lr}; ldrd r0,[r1,#4]; and r0,#255; uxth r4,r1; bl F; mul r0,r0,r4; pop` — `F((u8)b[1]) * (u16)b[2]` with the pair loaded by one `ldrd`.
- Stuck on: 5/7 — plain u32 reads give `ldrb` + `mov r4,r1` + `ldrh` after the call; a u64 load narrows to `ldrb/ldrh` (no `ldrd`).
- Lead for next time: keep the full u64 live past the call (use it after) so armcc can't narrow.

## DllField@0x4a564 / 0x4ebd8 -- `push {lr}` leaf stubs

- Stuck on: 2/4 — retail `push {lr}` / `pop {pc}` where armcc emits `push {r4,lr}` / `pop {r4,pc}` (the known single-register push residue, also DllBattle 0x4b804 / 0x4bfa8).
- Lead for next time: none yet.

## DllBattle push-less leaf scan (2026-10-16) -- untried families

- Scan recipe: disassemble `.text`, cut chunks at push/pop/b/bl/bx-lr/.word, keep chunks ending in `bx lr`, 3-24 words, not in `matched/`; strip leading pool junk (recipe also in `matched/DllField/0x412d0.c`).
- Untried families (offsets are chunk starts; leading pool-junk words may shift the true start):
- vstr s0 x3 [r0,#64/68/72] + strb r2,[r0,#76]: 0x29abc 0x3357c 0xc5dd8 (float-vec3 zero/init setter, s0 is an incoming float arg)
- strb from vstmia (vec3 copy to +4 with flag +76): 0x12704 0x12a30 0x12af8 0x12b24; ldm/strd variants 0x126ec 0x12864 0x128d4 0x12a00 0x12ae0 0x12b0c
- 8-byte copy to +1840 with strb +1836: 0x3c10c 0x3c124 0x3c13c 0x3c154 0x3c16c 0x3c184
- ldr/ldr/ldr/cmp/strne (conditional store): 0x41f94 0x434ec 0x439d4 (+ mov variants 0x3c960 0x43958)
- ldrb cmp #N moveq/movne (==N getter): 0x3d21c(!=1 ->#7) 0x762f4(==3,+1684) 0x81638(==6,+16) -- eq/ne boolean getters, probably `return v==N`
- cmp r1,#0 moveq/movne (bool not): 0x366c 0x4a614(r0); 0x71434/0x9e5a0 conditional-movne leaves
- ands/movne mask-test getters (known residue class, `bit-test`): 0x78be4 0x795f8 (+32->+72 &7) 0xc79d0
- indexed ldr `(i&1)*4` +2840/2844: 0xc7304 0xc73f4 ; ldrsb at 2048+116/117: 0xc8fec 0xc900c
- orr-shift-or setters `r1 = r2 | r1<<N; *p = r1`: 0x4b9cc (<<9), 0x76ce8 (<<25)
- vldr/vstmia float pair copies: 0x56a10 0x56a48 0x569fc 0x56a38 0x56a58 0x2b9dc

## DllBattle@0x5fa8c / 0x5faa0 / 0xc9e68 / 0x747fc -- field-copy helpers (leaf, `bx lr`)

- What they are: 0x5fa8c (0x14) `ldr r2,[r1]; str r2,[r0]; ldrh r3,[r1,#4]; strh r3,[r0,#4]`; 0x5faa0 (0x14) the same with `ldrb/strb` at +4 (untested); 0xc9e68 (0x1c) u32 + u16 + u8; 0x747fc (0x14) `ldr r2,[r1]; ldr r3,[r1,#4]; str r2,[r0]; str r3,[r0,#4]`.
- Tried: direct casts, locals at the top, member-wise struct copies, struct assignment, loads-first locals.
- Stuck on: right opcodes but armcc reuses r1/r2 for the second temp where retail uses a fresh r3 (and ip for the third) — 3/5 and 3/7. 0x747fc: retail hoists both loads before the stores; plain `d[0]=s[0];d[1]=s[1]` gives ldr/str/ldr/str (2/5) and the locals-first / struct-assign forms collapse to `ldm r1,{r1,r2}; stm r0,{r1,r2}`.
- Lead for next time: retail probably inlined these from a larger function (temps never recycled); an isolated compile may not reproduce the allocation.

## DllField@0x72534 (size 0x20; same bytes at 0x72554, 0x7dbc4 tail, 0x70f58 variant)

- What it is: 44-byte struct copy from src+12 — `add r2,r0,#12; vldmia r2,{s0-s7}; vstmia r1,{s0-s7}; ldrd r2,[r0,#44]; ldr r0,[r0,#52]; str r0,[r1,#40]; strd r2,[r1,#32]; bx lr`.
- Tried (armcc -O2 -Otime): plain struct assignment (`float f[11]`; `float f[8]` + 3 floats; `long long` + int tail; V4 sub-structs) -> push/ldm/stm cores, 1/8; `float[8]` struct assign + 3 scalar copies -> ldm/stm + vldr/vstr, 0/8.
- Stuck on: retail uses VFP `vldmia` for the first 8 words and an LDRD/STRD pair for the next 2; no assignment form produced that. 0x72554/0x7dbc4/0x70f58 skipped (same class).
- Lead for next time: none yet.

## DllBattle@0xf420 (0x1c), 0xf3fc (0x24), 0xca9c (0x20) -- u8-masked arg wrappers

- What they are: virtual-override wrappers `fn(this, a, int *args) { import(this->@0x40, args[0] & 0xff [, args[1]...]); return 1; }` (f3fc: two masked args; ca9c: `ldm r2,{r1,r3}` then `and r1,r1,#255`, second arg unmasked).
- Tried: `& 0xff`, `(unsigned char)` casts, `unsigned char` callee params, local copy, struct copy `S s = *args`, by-value struct param.
- Stuck on: armcc narrows the load to `ldrb r1,[r2]` and drops the `and` (5/7 on f420); retail does a full-word `ldr` then `and r1,r1,#255` after the `ldr r0`. Only `volatile unsigned *args` reproduces f420 (7/7) — a `trick(volatile-load)` not saved under `matched/`.
- Lead for next time: the original probably copies the whole arg struct (ldm for ca9c) with a u8 narrowing at the call; a natural form that defeats `ldrb` narrowing is unknown.

## DllBattle@0x4aa9c / 0x51c28 (bit-test getters), 0x4c818 / 0x4ca4c (bitset tests)

- 0x4aa9c (0x10) `ldrb [27]; and #64; lsr #6` and 0x51c28 (`#32`, `lsr #5`): tried `(p[27]&64)>>6`, `/64`, `(p[27]>>6)&1`, uchar/uint/int return, volatile, bitfield struct (`:1` at bit 6/5), `?1:0`. armcc always folds to `lsl #25; lsr #31`, `mov #1; and r0,r1,r0,lsr#6`, or `ands/movne`, never the plain `and; lsr` pair.
- 0x4c818 (0x2c) / 0x4ca4c (+0x69 instead of +0x66): bitset test `base = this+0x166; (1<<(i&7)) & base[i>>3]`. Target keeps `lsr r0,r1,#3` as a separate index register (`ldrb r2,[r2,r0]`), `mov r3,#1; lsl r0,r3,r1; and #255; ands r0,r0,r2; movne #1`. Best 4/11 (local `a=p+0x166; k=i>>3; m=1<<(i&7); b=a[k]`); static-helper and fused forms 1-3/11.
- Lead for next time: a source that yields the unfolded and-then-shift.

## DllBattle@0x4b804 / 0x4bfa8 (0x18 each), 0x7960c (0x18)

- 0x4b804 / 0x4bfa8: `return g() != 18` / `!= 10`; retail uses `push {lr}` / `pop {pc}`, our compile gives `push {r4,lr}` / `pop {r4,pc}`. The cmp and movne/moveq bodies already match (order of the two conditional moves differs in the `if (g()==18) return 0; return 1;` form); three phrasings failed to get the single-register push.
- 0x7960c: `ldm r1,{r1,r2}; uxth r2,r2; bl; eor r0,r0,#1` — by-value struct / struct with a `short` / local copy / mask all fail (`ldm` without `uxth`, or an extra instruction).
- Lead for next time: none yet.

## "Release member, zero via r1" destructor class (DllPuzzle@0xb3fc, DllHeading@0x87e4/0x881c, DllBox@0x1dfac/0x1df3c, DllGts@0x9824)

- Status 2026-10-15: the leaf variant is cracked (DllField@0xae58/0xe3fc/0xe380, see `matched/` — `trick(live-r1-passthrough)`); the dtor shape is **not**.
- Solved: the instruction-ordering part. Declare in this order: `void **vt = VT; void *m = o->f14; o->vptr = vt; if (m) { fn(m); o->f14 = 0; } return fn268(o);` (swapping the `vt`/`m` lines or storing `vptr` first gives the old mismatch). With it: Puzzle@0xb3fc 12/14, Heading@0x87e4 12/14, Heading@0x881c 11/13 — ONLY the zero register differs (retail `mov r1,#0; str r1,[r4,#20]`, ours r0). DllBox 0x1dfac/0x1df3c and DllGts@0x9824 were not re-run with the new ordering (Box has an extra vtable-call prefix).
- How armcc allocates: the shared zero goes in the lowest free scratch register after r0 (`this`). In the leaf, retail had r1 live (an incoming arg passed through), so the zero went to r2; unused params don't keep r1 live, returning `x` does but adds `mov r0,r1`, and a `long long` return `((long long)x<<32)|(unsigned)t` keeps r1 live with zero extra code. In the dtor, after the call both r0 and r1 are free; armcc picks r0, retail r1.
- Tried on top of the fixed ordering (all still r0 or worse): reusing the `vt`/`m` locals as the zero, hoisted `z=0` (lands in callee-saved r5), const-folding forms (`vt-vt`, `vt&0`, `m==0`), int/`void*`/`long long` callee result ignored, inline `memset`, struct wrapper assigned from a static const zero (adds pool loads), `*(int*)&f14=0`, `do{}while(0)`, an empty else, a 64-bit zero cast, a `char` zero.
- Lead for next time: need r0 live across the store via a use that emits no code; a C++ context with an inlined smart-pointer `reset()` is the one unexplored idea.
- Update 2026-10-16 (C++ attempt, `armcc-r5-cpp2`): 20+ C++ forms on top of the solved ordering, all still r0 — inline `Reset()` (also one returning `this`, one doing the whole destructor body), smart-pointer sub-object `P{p; reset()}` (9/14), `rel(void *n)` called with 0, a reference-parameter zero helper, `delete` / `delete[]` (12/14; delete of a virtual-dtor class 6/14), int / long long / pointer / double callee results ignored or used in `r-r` / `r&0` zeroing, a `P()` temporary (adds an sp spill), a real C++ destructor with a base class (ARM ABI emits D1/D0/D2, 29 words, not comparable). The inline-member lever that cracked DllField@0xb1bb8 doesn't apply: there it moved the null-test load to r1; here retail's test load is already r0 and the zero sits in r1 — opposite polarity, so r0 would have to stay live across the store without emitting code. DllPuzzle@0xb3fc stays 12/14; DllGts@0x9824 (virtual-call form) 12/14 with the same diff. The `a->member = 0` leaves and DllField@0x37714/0x37760/0x2aa40 sites were not retried.

## DllGts@0x7c8c

- What it is: stores 8 and 3 through a pointer and sets `byte@0x3c`.
- Stuck on: 1 word — retail hoists `mov r1,#1` ahead of the second store; armcc emits it after (constant-hoisting class).
- Lead for next time: none yet.
- Update 2026-10-15: size 0x24 — setter chain `this->@0xc` bytes +0 = 8, +1 = 3, plus `this->@0x3c = 1`; retail hoists the three constant `mov`s before the stores; plain and redundant-locals versions both 2/9.

## DllBattle@0x2cdc4 / 0x2cbfc / 0x2ce44 (0x38 each), 0x2c630 (0x50, probably the same)

- What it is: gated tail-calls with an int arg — `if (vt[0x98](o)) fa(o->x /*+0x12c*/, a)`; `if (vt[0x98](o)) { o->b346 = a; tail fa(o); }`; the same with `b348` (`a` = incoming r1).
- Tried: plain / negated / early-return forms, arg types int/char/short/unsigned/void*, local copy of the arg, static helper, function-pointer local, C++ member `virtual int v38()`, reordered `x` load.
- Stuck on: all 12/14 — only the vtable-slot register differs. Retail does `mov r5,r1; ldr r1,[r0,#0x98]; blx r1`; armcc always emits `ldr r2 ... blx r2` when an int arg is live across the virtual call. Float-arg siblings (0x2ca28, 0x2d5d8) match since the arg is in s0.
- Lead for next time: an extra-arg-style trick (cf. DllBattle@0xc9018) or an unknown original arg shape.

## DllZukan@0x8adc (size 0xd4)

- What it is: table lookup — loop `i < 7` over a 28-byte-stride global table comparing field0 to the arg; on a hit, scan `short arr[o->n@0xd4]` at 0xd6 for `i`, store the index to `short@0xe4`, range-check `(u8)i < 7` else `err(0,0,0)`, store a byte global, `fn(o,i)`, `fn2(o->q@0xe8->@0xd4, 0)`, vtable slot 2 call with `(p@0xd0, i & 0xff)`; returns 0 else 1.
- Tried: for / while / do-while, `n` hoisted vs `o->n` each iteration, `short j` (adds `sxth`, worse).
- Stuck on: best 46/53; only diff is a register swap in the inner scan loop (retail: count in r1, `j` in r0, `cmp r1,r0; bgt`; ours: count r0, `j` r1, `blt`).
- Lead for next time: find the shape that makes armcc allocate `n` before `j` (maybe the scan is an inlined helper).

## DllGts@0x9824 (size 0x38)

- What it is: dtor-ish — `p = this->p@0x20; if (p) { p->vt[6](p); this->p = 0; } fn(this+0x24); return this->r@0x14`.
- Tried: int-returning virtual, local zero variable.
- Stuck on: best 12/14 — retail keeps the zero for `this->p = 0` in r1 (`mov r1,#0; str r1,[r4,#32]`); armcc uses r0 (same zero-register class as DllBox@0x1dfac, DllField@0xe380).
- Lead for next time: none yet.

## DllHeading@0x87e4 (size 0x38) / 0x881c (size 0x34)

- What it is: destructors (tail-call / returns `this`) with the usual `vt`-local dtor idiom.
- Tried: no `vt` local; explicit zero local (both worse).
- Stuck on: 12/14 and 11/13 — retail zeroes the member with `mov r1,#0; str r1,...`, we emit r0 (the same zero-register class).
- Lead for next time: none yet.

## DllStatus@0x2fd8 (size 0x64) and 0x2e90 -- relaxed-only, NOT a masked match

- What it is: forward search loop. `f480(buf, this->f4); start = n; for (;;) { n++; if ((unsigned)n >= 6) { n = start; break; } if (f4b0(this->f16, n, buf)) break; } f490(buf); return n;` (`buf[4]`). 0x2e90 has the same shape.
- Result: matches only with `--relax-nops`; retail has a nop between `cmp r0,#0` and `beq`.
- Lead: code is right; only the post-link nop differs.

## DllStatus@0x6a30 / 0x6a00 / 0xa630 / 0xa664 (sizes 0x34/0x2c/0x2c/0x28)

- What it is: secondary-base ctor-stage family — `this->vt = VT; (this+8)->vt = VT+20; f448(this+8); return f450(this+8) - 8` (0x6a00 takes the vtable as an arg and tail-calls f310; 0xa630/0xa664 are the this-8 adjusted-thunk versions: `f448(this)` r0 passthrough, `f450(o+8)`).
- Tried: `char*` cursors, struct members, hoisted `q = VT+20`, `o = this-8` forms, store reorderings.
- Stuck on: 0xa664 best 7/10 (only scheduling: retail does `sub r4,r0,#8` first, then `add r2,r1,#20`; ours sinks the `sub` after the stores); 0x6a30 retail hoists `add r1,r0,#20` before `str r0,[r4]` and `add r0,r4,#8` before the second store, best 9/13.
- Lead for next time: none yet (scheduler-order class).

## DllBag@0x10844 (size 0x58)

- What it is: sibling of 0x7330/0xc508 state handlers, but `fnB(this)` is called before the `b == 1` test; `b = this->q->b` (q at +0x44) is loaded into r6 before the call.
- Tried: the sibling `struct Sub *s = &this->sub` style (true branch via `s->p->h`, `s->v`) -> best 15/22; every variant writing the true branch via `this` flips register allocation (this->r4, s->r6/r5, b->r5): 8-9/22.
- Stuck on: retail's true branch uses `ldr r3,[r5,#8]` (this, not s), hoists `mov r1,#0 / r2,#75 / r0,#1`, then `strh / str [r5,#56]! / str [r5,#8]`.
- Lead for next time: a source form where `s` (= this+8) stays live in r4 but the true branch addresses via `this`.

## DllBag@0x1089c (size 0x54)

- What it is: `fnC(this->y->ctx, this+0x50, 5, 0, 0, 0)` (two stack zeros); `this->st = 55; this->y->flag(+0x2c) = 1; return 4`.
- Tried: `int r = 4;` local (spills to r5, 8/21).
- Stuck on: 14/21 — retail does `ldr r0,[r4,#76]; mov r1,#1; mov r2,#4; strb r1,[r0,#44]; mov r0,r2` (return value 4 hoisted into r2 before the store); mine emits `ldr r1 / mov r0,#1 / strb / mov r0,#4`.
- Lead for next time: none yet (constant-hoisting class).

## DllTownmap@0xaa4 (size 0x6c; sibling 0xa34) -- relaxed-only, NOT a masked match

- What it is: dtor stage — store VT, virtual-delete members `[0x14]` (slot 6) / `[0x18]` (slot 1) zeroing, `fn1b8(this+0x1c)`, `return (char*)fn228(this+0x1c) - 0x1c` (void* callee). 0xa34 is the same with `pop {..,lr}` + tail `b fn230`.
- Result: 26/26 with `--relax-nops` (one nop elided before the `bl fn228`); masked 21/27 because the retail nop can't be reproduced.
- Lead: the code is right; only the post-link nop differs (same as DllBattle@0x30a4).

## DllTownmap@0x928 / @0x94c (size 0x24 each)

- What it is: `o = this[1]; if (o) o[1] = K; this[0x34] = 1; tail fn(this[0x14], 0)` with K = 1 (0x928) / 0 (0x94c).
- Tried: a static helper taking K as a parameter.
- Stuck on: 0x94c 5/9, 0x928 mismatched. Retail loads K into r1 unconditionally before the `cmp` and reloads 1 for the second store; armcc emits a conditional `movne` and reuses r1.
- Lead for next time: none yet.

## DllStatus@0x3ed8 (size 0x48)

- What it is: function loading a global, with a stack argument fed from `#0`.
- Tried: several rewrites (details not logged by the agent).
- Stuck on: 11/18. Retail keeps `this` in r4 and the loaded global in r5 (armcc swaps them) and reuses r1 for the `#0` that feeds the stack arg.
- Lead for next time: none yet (register-allocation class, like DllTownmap@0x1324).

## DllBox@0x1dfac (size 0x6c; sibling 0x1df3c, size 0x70)

- What it is: dtor — `vptr=VT1; if (o->[36]) f258(f910(f910(sub+108)-24)-84); f260(o->[32]); vptr=VT2; if (o->[20]) { f620(o->[20]); o->[20]=0; } return o`. 0x1df3c is the same body plus a tail-call of a base dtor instead of `return o`.
- Tried: `f620` returning `void*`, int-cast zero, `p=0; o->buf14=p`, reading `p` before the VT2 store (22/27), vt2 held in a local (9/27).
- Stuck on: 0x1dfac 25/27 — only diff: retail zeroes via `mov r1,#0; str r1,[r4,#20]`, ours `mov r0,#0; str r0,[r4,#20]`. 0x1df3c 20/28: same zero-register diff plus a `mov r4,r0` ordering diff (retail loads VT and sub before `mov r4,r0`).
- Lead for next time: something keeps r0 live (or r1 free) across the zeroing store; try different inlined-helper shapes for the member release.
- Update 2026-10-15 (dtor retry): with `extern char VT1[]/VT2[]` the natural form is 23/25 — only the zero-in-r1 diff remains (same as DllPuzzle@0xb3fc). 0x1df3c with a vt1 local is 20/28: retail loads VT1 into r2 and the member into r1 and delays `mov r4,r0`; a vt2 local spills to r5/r6 (8/28). Fixing the zero-in-r1 diff would likely unlock both.

## DllPhotoViewer@0x19cc (size 0x3c)

- What it is: this-0x24 thunk — checks state byte at +0x59 for values 7, 8, 11, then sets byte +0xb8 to 2 under a condition.
- Tried: `char*` arithmetic, reassigned parameter, unsigned-int parameter, volatile state byte.
- Stuck on: 13/15; logic right, only the first two words swapped. Retail does `sub r0,r0,#36` before `ldrb r2,[r0,#89]`; armcc hoists the load to `ldrb r2,[r0,#53]` ahead of the sub.
- Lead for next time: none yet.

## DllTownmap@0x32f0 (size 0x60)

- What it is: table lookup `find(this, key, int *out)` — scans 64-byte entries (key +4, value +8) from a count at `[0]`, returns 0 or 1.
- Tried: for loop, do-while inside `if (n > 0)`, char-pointer arithmetic with offset expressions.
- Stuck on: best 4/24. Retail hoists constants 4 and 8 into registers (`mov r4,#4`, `mov r1,#8`) with a `push {r4,r5}` prologue; armcc folds the offsets into immediates every time.
- Lead for next time: none yet.

## DllTownmap@0x31a0 (size 0x30)

- What it is: constructor — stores `vt` at `[0]` and `vt+0x40` at `[0x74]`, calls `fa`, tail-calls `fb`.
- Tried: several store and local-variable orderings.
- Stuck on: 8/12. Retail does `add r1,r0,#64` before the first store with `mov r0,r4` between the two stores.
- Lead for next time: none yet.
- Update 2026-10-15: locals a=VT, b=a+0x40 in several orders hoist the pool load above `push` (5-6/12); plain form stays 8/12. Retail: `ldr r0,pool` after `mov r4,r0; add r1,r0,#64; str r0,[r4]; mov r0,r4; str r1,[r4,#116]`. Sibling dtor 0x31d4 now matches.

## DllPuzzle@0xb3fc (size 0x38; twin 0xbf80)

- What it is: dtor stage — store VT, `if (this->f14) { fn1f8(f14); f14 = 0; }`, tail-call base `fn268(this)`.
- Tried: plain C; local-variable `m` variant. Tail call (`void*` return) works.
- Stuck on: 9/14. Retail interleaves `ldr r1,pool; ldr r0,[r0,#20]; str r1,[r4]` and zeroes through r1; armcc loads pool into r0, stores, then loads the member, `mov r0,#0`. Known-bad dtor member-load ordering. 0xbf80 differs only by using r0 for the zero.
- Lead for next time: none yet.
- Update 2026-10-15: twin 0xbf80 now MATCHES (trick ordering-local, `extern char VT[]`). 0xb3fc is 12/14 — only the 2-word zero-register diff (retail `mov r1,#0; str r1,[r4,#20]`, ours r0); tried an int member, `f1f8` returning `void*` with `(r&0)`, hoisted `z=0` (6/14), reusing the vt local as the zero (12/14). The same diff blocks DllBox 0x1dfac/0x1df3c.

## DllTownmap@0x1324 (size 0x38)

- What it is: bounds-checked accessor — `if (this->[4]->[0] <= idx) fail(0,0,0); return this->[0xc] + idx*8`.
- Tried: struct-field C, hoisted count load, if/else forms, pointer-arith forms (~5 variants).
- Stuck on: 9/14. Instruction stream identical except register allocation: retail keeps `this` in r4 and `idx` in r5; armcc gives the reverse.
- Lead for next time: none yet (try swapping parameter order / an unused extra arg to shift allocation).

## DllBattle@0x3038 (size 0x6c)

- What it is: same body as matched 0x30a4 but retail loads the vtable pool word into r1 BEFORE `push {r4,lr}`, and ends with tail `b`.
- Tried: 0x30a4 shape plus tail call to extern fc(r).
- Stuck on: 21/26; only the first 5 words differ (ldr r1,pool hoisted above push, str r1,[r0] before mov r4,r0).
- Lead for next time: none yet.
- Update 2026-10-15: 16/27 with a vt local; retail has a post-link nop word at 0x308c so it can never be a pure masked match, and it loads the pool into r1 before `push`. Relaxed-only at best.

## DllBag@0x10e4 (size 0x6c)

- What it is: lazy-init + conditional setup: `if (!this->ctx(+0x4c)) fnInit(this); if (fnTest(ctx->p0->f8)) { fnA(this,1); fnB(ctx->p0,1,1,1,1[stack]); tail fnC(this, this->f38); }`.
- Tried: unsigned char param types, varargs, local `k = 1`, hoisting the first arg into a local.
- Stuck on: 15/27. Words up to the `fnA` call match. Retail: loads ctx first then `mov r1,#1; mov r3,r1; mov r2,r1; ldr r0,[r0]; str r1,[sp]` (constant lives in r1); ours: `mov r0,#1; str r0,[sp]; ldr ctx; mov r3,#1...`.
- Lead for next time: fnB signature (struct/bool mix) or the 1s coming from distinct sources.

## DllGts@0x9880 (size 0x18)

- What it is: stores constants then loads others (5/6 words matched).
- Tried: see note. Retail hoists `mov r2,#10` and `mov r1,#1` before the first store; armcc emits `mov r1,#1` after it (word 0x988c).
- Stuck on: 5/6, constant-hoisting order.
- Lead for next time: none yet.

## DllPokeList@0xaac (size 0x54; same shape at 0xa50 size 0x5c)

- What it is: multiple-inheritance destructor.
- Tried: `--relax-nops`, struct-typed fields, locals `vt = VT`, char*/void**/unsigned* vtable types, member load before/after stores.
- Stuck on: best 12/20. Retail loads the vtable pool word into r1 BEFORE `push {r4,lr}` and keeps it live with independent `add r0,r1,#16` / `add r2,r1,#32`; armcc loads it after `push` and chains the adds. Store order target `[12],[0],[32]`.
- Lead for next time: three separate vtable externs (VT0/VT1/VT2 at +0/+16/+32 of one object) might force independent address materialisation — untried.
- Update 2026-10-15 (batch 10): the three-separate-externs lead was **disproven** — VT0/VT1/VT2 scored 8/21 (worse; three independent pool loads, no shared base); single extern with `VT+16`/`VT+32` + `--relax-nops` stays at 12/20. Retail loads the pool into r1 before `push`; likely a different source construct (e.g. member-init in an inlined base dtor) — untried.
- Update 2026-10-15: 0xa50 has a nop word at 0xa94 (between the two `bl`) — relaxed-only at best; 0xaac not re-disassembled.

## DllStatus@0x459c (size 0xb8)

- What it is: byte-toggle region at `this+0xf5`.
- Tried: if/else, ternary, local variable, goto join.
- Stuck on: best 26/46. Retail never hoists constants #0/#1 into registers (re-materializes `mov r0,#1; mov r3,r0`), armcc does in every shape.
- Lead for next time: none yet.

## DllStatus@0x6f74 (size 0x1c)

- What it is: vldmia/vstmia copy of 32 bytes src+0 -> dst+32, then 8-byte copy src+32 -> dst+64.
- Tried: u32[8]/float[8] struct assignment, loops, unrolled float assigns, V4 structs.
- Stuck on: armcc gives ldm/stm or vldr/vstr, never vldmia.
- Lead for next time: none yet.

## DllStatus@0x25c8 (size 0x18)

- What it is: `(float)a/(float)b` from two u32 args.
- Tried: `float fn(unsigned a,unsigned b){return (float)a/(float)b;}`.
- Stuck on: 2/6. Target does both `vmov` first and divides s2/s1; armcc interleaves vmov/vcvt and swaps the divide operands.
- Lead for next time: none yet.

## DllZukan@0x1bdc0 (size 0x58)

- What it is: `if (this[0xe0]) { a = vA(x) != 0; b = vB(x) != 0 }` then `return (vC(this)|a|b) != 0`.
- Tried: plain int, ternary, temp variable, C++ `bool` (.cpp, 7/22).
- Stuck on: best 9/22. `mov r4,#0; mov r5,r4` init matches, but bool normalization differs: retail emits `cmp r0,#0; movne rN,#1; moveq rN,#0` (first call result in r4, second in r5); armcc emits `adds rN,r0,#0; movne` and swaps r4/r5.
- Lead for next time: none yet.

## DllBattle@0x23e8 (size 0x34), 0x23b0 (same + tail call)

- What it is: simple destructors — vtable store + free member at +0x14.
- Tried: plain C dtor; 0x23b0 not separately compiled.
- Stuck on: 8/13 words. Candidate hoists the member load ahead of the vtable store; retail interleaves (`ldr r1,pool; ldr r0,[r0,#20]; str r1,[r4]`). Known-bad dtor shape.
- Lead for next time: none yet.

## DllBattle@0x2390 (size 0x1c)

- What it is: ctor wrapper — calls a base function, installs vtable, stores an argument at +0x28.
- Tried: `char[]` and `int` forms of the extern vtable symbol.
- Stuck on: 5/7. armcc picks r2 for the vtable temporary where retail uses r1.
- Lead for next time: none yet.

## DllFieldDebug@0x1ac (size 0xac) -- same function in every tiny debug/test module
Also present identically at @0x180 (size 0xac) in DllDebugPokeMake, DllDebugProcLoop,
DllPokeViewer, DllTest{Ariizumi,Hosaka,Ikuta,Ito,Mori,Taya,NCL,...} (all have .text of
only 0xac bytes, 0 imports) and at @0x1ac in DllFieldDebug/DllTestKawada (0xdc text,
preceded by the 0x2c strd/stm function below). One match would cover ~20 modules.

- What it is: `int f(int *out, int op)` -- switch on op (cmp r1,#5 / ldrcc pc,[pc,r1,lsl#2]
  inline jump table, 5 cases, push {r4,lr}):
  0: *out = <reloc addr>; 1: g_ptr = out (str r0,[r2], r2 = reloc addr of a module global);
  2: *out = *g_ptr; 3: out[0]=<addr>, out[1]=<addr>; 4: same with 2 other addrs; all return 0.
  default: `ldr r0,=0x30000000; nop; ldr r0,=0x30000026; nop{0}; nop; ldr r0,=0xD8E12C21;
  pop {r4,pc}` -- three consecutive dead-looking r0 loads separated by nop windows
  (likely bl-to-import windows relaxed to nops), returns 0xD8E12C21.
- Tried: plain C switch with externs (`extern int A1..`, `*out = A1` derefs, `return 0xD8E12C21`
  default): 7/43 only. Findings: (a) the case values are ADDRESSES (ldr rN,=sym, no deref) -- use
  `extern char X[]; *out=(int)X`; (b) retail has push {r4,lr}/pop {r4,pc} (a call exists
  somewhere, in the default path's nop windows); mine is a leaf (bx lr).
- Stuck: the default block. No C construct tried yields dead r0 loads + nops without bl (those
  would be bl words, absent in target even with --relax-nops). Untried: `--relax-nops` plus
  calls to externs in default (target has no bl, so would need the nop-window hypothesis proven),
  or __asm blocks (would not be a genuine decomp).
- Also: 0x180 (size 0x2c, set two u64 globals) is the known strd/stm residue; retried the
  extern-helper idea (static helper `set_u64(v,&g)` => 5/11, non-static noinline => calls
  not inlined 0/11) and unsigned/-1 literal/local-copy/struct variants (3-5/11). Still stuck.

Module: DllDebugPokeMake (identical 0xac function in DllDebugProcLoop, DllPokeViewer, ~17 DllTest* modules; also at 0x1ac in the 0xdc-sized ones)
Offset: 0x180
Size: 0xac

What it is: 5-way switch on r1 (`cmp r1,#5; ldrcc pc,[pc,r1,lsl #2]`, inline jump table of relocs) with push {r4,lr}. Cases in body order: *p=&X1; g=p (r2=&g); *p=g; p[0]=&X2,p[1]=&X3; p[0]=&X4,p[1]=&X5. Default (op>=5) returns 0xd8e12c21 through a weird tail: ldr r0,=0x30000000; nop; ldr r0,=0x30000026; nop,nop(0xe320f000 in the middle); ldr r0,=0xd8e12c21; pop {r4,pc} (dead loads of the first two constants survive).

Tried: plain switch C with extern int objects and (int)&Xn stores, default return 0xd8e12c21. Got 0 full match: armcc emits no push {r4,lr} (bx lr leaf, no shared tail), puts the default as a separate leaf, and the table layout differs. Case-body shape (ldr const / str / b shared tail) matches in spirit.
Lead: the dead 0x30000000/0x30000026 loads + nops look like inlined Result-construct helpers (nn::Result module/description bits) emitted unoptimized, and the r4 push suggests a call-free non-leaf frame; likely needs the original inline helper structure. Not worth more until a second example of that Result idiom is matched elsewhere.

## DllTestOhno@0x180 (size 0xac) -- same function in ~25 stub modules

- What it is: the ONLY function in DllTestOhno (text +0xac). Identical shape
  (0xac; 0xdc variant in FieldDebug/TestIchiraku/TestKawada/TestKosaka/
  TestNagihashi/TestSuzuki) in every unmined stub module: DebugPokeMake,
  DebugProcLoop, PokeViewer, TestAriizumi/Hosaka/Ikeda/Ikuta/Ishiguro/Ito/
  Mori/Nakahiro/NCL/Obata/Ohno/Onoue/SeqDemoView/Sogabe/Tamada/Taya/Tomoya
  (all 25 unmined Dll*.cro). One solve would unlock them all. Same
  jump-table/NOP-hint residue class as DllDialogCommon@0xdd8.
- Shape: f(int *out, unsigned sel), 5-case switch (`ldrcc pc` table at +0x14,
  5 words; mask with --mask-ranges 0x14:0x14), returns 0; pool words are
  relocated (zero in the CRO). Bodies in physical order:
  *out = &symA; gv = out (str r0,[r2]); *out = gv (ldr r1,[r2]; str r1,[r0]);
  out[0]=&symB,out[1]=&symC; out[0]=&symD,out[1]=&symE (all plain
  `ldr r1,[pc]` of a pool word = address constant, NOT a deref).
  Default: ldr r0,=0x30000000; nop; ldr r0,=0x30000026; nop{0}; nop; ldr
  r0,=0xd8e12c21; pop {r4,pc} -> returns 0xd8e12c21 (nn result code).
- Got: with `extern char symX; *out=(int)&symX` and a global `gv`, the first
  case bodies' words match (up to case 2's load); the rest differs in the
  shared-tail layout and the default case. Original frame is
  `push {r4,lr}`; mine only gets one if a call exists. `__nop()` x3 in
  default gives e320f000 words (original interleaves mov r0,r0 / ldr /
  nop-hint); `dbg(0x3000...)` calls give bl (original has nops) and
  `mov r0,#0x30000000` instead of the pool `ldr`.
  So the default's dead loads of 0x30000000/0x30000026 come from some
  inline/intrinsic (likely an SDK assert/log macro) that leaves dead loads +
  nops and no call, which also would explain the frame. Unresolved.
- Lead: find what yields `ldr r0,=const; nop` pairs without a call. Not
  solved in ~20 min; skip all these stub modules until the DialogCommon@0xdd8
  class is cracked.

## Unmined-module survey 2026-10-15 (DllPokeViewer + all remaining Dll{Test*,Debug*} modules)

- Every module without a matched/ dir is an 8192-byte stub. Surveyed first
  words at 0x180 for all 26: 21 (incl. DllPokeViewer, DllDebugPokeMake,
  DllDebugProcLoop, DllTestAriizumi/Hosaka/Ikeda/Ikuta/Ishiguro/Ito/Mori/NCL/
  Nakahiro/Obata/Ohno/Onoue/SeqDemoView/Sogabe/Tamada/Taya/Tomoya) contain
  ONLY the nnroControlObject_ 5-case jump-table dispatcher (known dead end).
- DllTestIchiraku, Kawada, Kosaka, Nagihashi, Suzuki additionally have a
  leading globals-init function @0x180 (size 0x2c: `ldr r2,[pc]; mvn r0,#0;
  mov r1,r0; strd r0,[r2]; ldr r0,[pc]; mov r1,#0; mov r2,r1; stm r0,{r1,r2};
  bx lr`) -- same residue as DllUSGymFighting@0x11dc / DllUSGymIce@0x1974.
  (Kosaka differs in one word: 0xe1c200f8, strd of r0:r1 to [r2,#8].)
- Tried on DllTestIchiraku@0x180 (--mask-ranges 0x24:4,0x28:4): two named zero
  locals assigned to g_b[0]/g_b[1] -> 4/11 (address-first, stm for the -1);
  struct copy from a zeroed local -> 3/11. No improvement over prior notes.
- Conclusion: no easy candidates remain in unmined modules; no matches produced.
  Next batch should target partially-mined modules instead.

## <Module>@<offset> (size <hex>)

- What it is: <one-line guess at what the function does>
- Tried: <approaches / C shapes attempted>
- Stuck on: <match ratio reached, and exactly which words/behavior differ>
- Lead for next time: <anything that might unblock it, or "none yet">
```

---

## DllConfig@0x31b8 (size 0x20)

- What it is: no-branch, no-call field-copy helper. `r1` points to a
  3-word source struct (f0,f4,f8); copies those into `r0`'s fields at
  +0x38,+0x3c,+0x40, and *also* re-copies f0/f4 into +0x44/+0x48 (so
  f0/f4 are each written twice, to two different destinations).
- Tried: (1) flat field-by-field assignment, every statement order —
  compiles to 5 separate ldr/str pairs, no merging (11 words vs
  target's 8). (2) `d->sub = *s;` via a 3-word nested sub-struct at
  +0x38 plus the two extra re-reads — reproduces the target's `ldm
  r1,{r2,r3,ip}` read side exactly, but the write side merges into one
  `stmia r0!,{r2,r3,ip}` (full 3-word post-increment store), not the
  target's split. (3) same via an explicit local `S tmp = *s;` with
  three separate per-field stores in the target's own store order —
  identical over-merge, byte-for-byte.
- Stuck on: 5/8 masked words. The `ldm` line and the two +0x44/+0x48
  re-read/re-store lines already match. The 3 words in between never
  reproduce: armcc 5.04u5 at -O2 -Otime either doesn't merge adjacent
  stores at all, or merges all three into one block op — never the
  target's partial merge (one lone `str` + one `strd` pair, skipping
  the 3-word `stm`). Source statement order didn't change the outcome
  in either the flat or struct-copy case.
- Lead for next time: maybe the real destination isn't one flat struct
  — e.g. the far field (+0x40) could belong to a different member or a
  union arm that breaks armcc's "is this one aggregate" merge check
  while the +0x38/+0x3c pair stays recognized as adjacent. Not tried:
  `-O1`/`-O3`/`-Ospace` flag variants, or reaching +0x40 through a
  genuinely unrelated pointer variable rather than the same `d` base.

## DllDialogCommon@0xd20 (size 0x8c)

- What it is: message dispatcher — loads a handler from `this+8`, and
  for `msg` in `{4,10}`/`{5,13}`/`{16}` calls `handler->vtbl[0](handler,
  0|1|2)`, else returns 1.
- Tried: plain `switch`, equivalent `goto`-chain matching the exact
  observed branch order.
- Stuck on: 11/35 masked words. Control flow is fully correct; armcc's
  block-layout for the comparison tree places blocks in a different
  physical order than the retail binary no matter how the switch/goto
  is phrased.
- Lead for next time: none yet — this is armcc's own layout heuristic,
  not something source-order seems to control. Full detail:
  `docs/decomp/armcc-toolchain.md`.

## DllDialogCommon@0x710 (size 0x94)

- What it is: constructor — fetches an object via an import veneer,
  installs a vtable, builds a buffer via two more veneer calls,
  placement-allocates a second small object and zero-initializes
  several of its fields using a pointer recovered by subtracting a
  fixed offset (8) from a veneer's return value.
- Tried: typed struct member access, raw byte-offset casts, splitting
  the subtraction into its own statement, forcing early use of the
  adjusted pointer, `-O1`/`-O3`/`-Ospace` flag variants.
- Stuck on: 30/37 masked words — every value and the final state is
  correct, but armcc keeps the *unadjusted* pointer live and folds the
  `-8` into each field's immediate offset, only materializing the
  adjusted pointer at the very end, while the retail binary computes it
  immediately after the veneer call and uses small positive offsets
  throughout.
- Lead for next time: the sibling `DllNumberInput@0x18fc` (already
  matched) has the same overall shape but its node-init tail genuinely
  *is* a sequential pointer-cursor chain in the retail binary (confirmed
  by its own pre-indexed `str rX,[rY,#N]!`), so that idiom doesn't
  transfer here — `0x710`'s tail really is one `sub` + fixed offsets.
  Full detail: `docs/decomp/armcc-toolchain.md`.

## DllDialogCommon@0xdd8 (size 0xb0)

- What it is: a 5-case property get/set switch on `this`, no function
  calls anywhere in the reconstructed body.
- Tried: `switch` over the 5 case values with `--mask-ranges` to exclude
  the jump table's data words.
- Stuck on: never reached a full match — the candidate didn't need a
  stack frame (no calls), but the retail binary has `push {r4,lr}` /
  `pop {r4,pc}` anyway, which shifts every subsequent word. The default
  case (selector >= 5) also loads three pool constants in a row with
  `e320f000` NOP-hint words interleaved — this project's own marker for
  post-link branch-relaxation nops — suggesting the real default case
  made calls that got relaxed away entirely, not just argument-setup
  with nothing to call.
- Lead for next time: find what the default case actually calls; that
  would likely also explain the frame. `DllNumberInput@0x1af8` has the
  same overall shape and hasn't been attempted — expect the same issue.
  Full detail: `docs/decomp/armcc-toolchain.md`.
## DllFieldDebug@0x180 (size 0x2c)

- What it is: module's very first function (no import-veneer table at all
  in this tiny module — 0 imports, `.text` is only 0xdc bytes total). A
  no-call, no-branch leaf: sets one 64-bit module-static global (call it
  `g_handle`) to -1 (invalid-handle sentinel) and a second, independent
  64-bit global (`g_state`) to 0. Confirmed via the CRO's own internal
  relocation table: two entries patch this function's two literal-pool
  words (file offsets 0x1a4/0x1a8) to `.data_segment_base + 0` and
  `.data_segment_base + 8` respectively — two distinct 8-byte objects,
  8 bytes apart, inside the module's 0x14-byte `.data` segment.
- Tried: (1) two plain `extern long long` globals, `g_handle = -1;
  g_state = 0;` (both statement orders) — word count matches (11) but
  only the *second* store matches byte-for-byte (confirms `g_state = 0;`
  is the right idiom for that half). (2) same but `static` instead of
  `extern` — armcc notices the two statics are provably adjacent within
  the TU and fuses both stores into one `ldr`+`stmia r0!,...`+`stm`
  sequence (9 words, not 11) — wrong shape entirely, extern is required
  to block that fusion. (3) explicit `Pair{unsigned int lo,hi;}` struct
  with individual `.lo`/`.hi` field assignments for both globals — loses
  the `stm` merge entirely (falls back to separate `str`/`str` pairs,
  worse). (4) local `Pair` temp copy-assigned into `g_handle` — store2
  (`g_state`) matches exactly this way too (7/11 overall) but store1 is
  unaffected. (5) an inlined `static void set_u64(long long value,
  long long *dest) { *dest = value; }` helper called as
  `set_u64(-1, &g_handle);`, hoping the AAPCS value-then-pointer argument
  order would survive inlining into the right register roles — armcc
  fully erases the call and allocates identically to the plain-assignment
  case (no change).
- Stuck on: 7/11 masked words (best result, variant 4's second half
  matches; store1 never reaches more than partial credit). The *only*
  piece that never reproduces is store1's register role assignment:
  retail computes the **value** first (`mvn r0,#0; mov r1,r0`, landing in
  the r0:r1 even-aligned pair) and the **destination address** *second*
  (`ldr r2,[pc,...]`), then uses `strd r0,[r2]` (one store-pair
  instruction). Every C idiom tried instead makes armcc compute the
  **address first** (always lands in r0, the lowest free register) and
  the value second (`r1,r2` — odd-start, so `strd` isn't even available,
  forcing a `stm` instead). Store2 (`g_state = 0;`) *does* naturally
  reproduce retail's address-first/`stm` pattern, so that half is solved
  and not worth re-attempting.
- Lead for next time: the value-first/address-second/`strd` ordering
  looks like a *calling-convention echo* — i.e. the real source likely
  passes the 64-bit sentinel value through an inlined helper whose
  parameter order is `(value, dest_ptr)` (AAPCS would naturally put a
  64-bit first arg in r0:r1 and a pointer second arg in r2), the
  opposite of the `(dest_ptr, value)`/`memset`-shaped order that *does*
  match for `g_state`. Attempt (5) tried exactly this shape but the
  inliner discarded the register-role hint — worth trying with the
  helper left **un-inlined** (give it external linkage / `__attribute__
  ((noinline))` equivalent so armcc must honor the AAPCS call, then see
  whether a *second* invocation of the function, compiled as a true call
  with masked `bl`, reproduces the body) — not yet tried due to the time
  cap. Also untried: `-O1`/`-O3`/`-Ospace` flag variants, and whether
  declaring `g_handle` with an explicit `__align(8)` changes anything.
  The module's only other function (`0x1ac`, size ~0xa0, the
  `nnroControlObject_` export) is a `cmp r1,#5; ldrcc pc,[pc,r1,lsl#2]`
  5-case jump-table dispatcher — same hard shape already logged for
  `DllDialogCommon@0xdd8`/`DllNumberInput@0x1af8`/`DllCommWait@0xce0`,
  not attempted here.


## DllNuts@0x708 (size 0x20) and DllNuts@0x728 (size 0x28)

- What it is: the module's first two real functions, right after the
  import-veneer table. Both copy an 8-single-float block, plus a
  trailing int/pair field, from one struct to another.
- Tried: struct assignment, a nested float-only sub-struct, `memcpy(32)`,
  an element-wise loop, a manually unrolled per-element float
  assignment, and `--fpu=vfpv2`/`--fpu=softvfp+vfpv2` flag overrides.
- Stuck on: every variant lowers either to GPR `ldm`/`stm` (which needs
  `push`/`pop r4-r8`; the target doesn't push those) or to individual
  per-element `vldr`/`vstr` pairs (no `vldm`/`vstm` formation) — never
  the target's single merged 8-register VFP multiple-transfer
  (`vldmia`/`vstmia`).
- Lead for next time: this looks like a new residue class, distinct
  from the already-documented vptr-scheduling one — armcc 5.04u5
  doesn't appear to peephole-merge consecutive single-float scalar
  loads/stores into one `vldm`/`vstm`, and its generic aggregate-copy
  lowering (struct assignment / inlined `memcpy`) always picks GPRs
  regardless of the aggregate's element type. Worth a fresh angle (a
  `union`, an explicit vector intrinsic, or a differently-shaped source
  idiom entirely) — not logged as fully dead, just not solved in this
  pass.

## DllPokeViewer — whole module is a dead end (not a single-function attempt)

- What it is: this module's entire real `.text` is exactly one function,
  at `0x180` — the `nnroControlObject_` module-control dispatcher (a
  5-case jump-table `switch` with a `push {r4,lr}`/`pop {r4,pc}` frame
  and a default-case tail of pool-constant loads interleaved with
  `e320f000`/`mov r0,r0` NOP-hint words).
- Stuck on: this is the exact unresolved jump-table/NOP-hint residue
  class already logged above for `DllDialogCommon@0xdd8` and
  `DllCommWait@0xce0`, and separately for `DllFieldDebug`'s own
  `nnroControlObject_` (same shape). No prior agent has cracked this
  class.
- Lead for next time: the module offers no other candidate function —
  it's the only real code in the file — so there is nothing else to try
  here until the jump-table/NOP-hint class itself is solved. Skip this
  module entirely (don't re-investigate) until that happens.

## DllTradelist@0xb6d0 (size 0x14) and DllTradelist@0xb6e4 (size 0x14)

- What it is: two adjacent struct-copy helper functions, no
  branches/calls. `0xb6d0`: `ldr r2,[r1]; str r2,[r0]; ldrh r3,[r1,#4];
  strh r3,[r0,#4]; bx lr` (copies a 4-byte field then a 2-byte field).
  `0xb6e4`: `ldr r2,[r1]; ldr r3,[r1,#4]; str r2,[r0]; str r3,[r0,#4]; bx
  lr` (loads both fields, then stores both — two 4-byte fields).
- Tried: field-by-field assignment, reversed field order, `void*`
  params with local casts, unsigned vs signed short, whole-struct
  assignment (`*dst = *src;` — over-merges into `ldm`/`stm`), and a
  volatile-source variant that got the load-both-then-store-both shape
  right for `0xb6e4` but still picked the wrong registers/order.
- Stuck on: armcc 5.04u5 always reuses `r1` (the `src` pointer register,
  dead after its last load) for the second temp instead of allocating
  `r3`, and/or orders the second store before the first. Every C idiom
  tried either reproduces the retail register choice or the retail
  store order, never both at once. Best ratio reached: 3/5 (`0xb6d0`),
  2/5 (`0xb6e4`).
- Lead for next time: this is the same general residue class already
  documented for `DllConfig@0x31b8` (struct-copy store-merge/
  register-allocation quirks) — a case of "two adjacent struct-copy
  helpers, same symptom" rather than a new independent lead. Try
  passing the struct by value (not pointer) to see if parameter-register
  pressure changes allocation, or an explicit `register` hint, or
  splitting into a helper that copies through an intermediate local
  array.

## DllTitle@0xd568 (size 0x28) and DllTitle@0xd54c (size 0x8)

Two stuck candidates in the same dense getter/setter/predicate cluster
(`0xd530`-`0xd8ac`) as this module's four matched functions.

**`0xd568` (size `0x28`) is a 4-int equality predicate**: loads
`this+16`/`+24`/`+28`/`+20` (in that exact order) into `r3`/`r1`/`r2`/`r0`,
then XORs the `+16`/`+24` pair and the `+20`/`+28` pair, ORs the two
diffs, and returns 1 if zero else 0 — a classic `(a==b) && (c==d)`
compiled branchless via xor/or rather than cmp/cmp/and.

- Tried, in order: `(p[4]==p[6]) && (p[5]==p[7])` — wrong shape entirely,
  armcc kept it as short-circuit branches (12/10 words). `(p[4]==p[6]) &
  (p[5]==p[7])` — still branchy, not the xor/or trick.
  `((p[6]^p[4])|(p[5]^p[7]))==0` direct on a flat `int *p` — gets the
  right op shape but armcc evaluates the two xor subexpressions one at a
  time (5/10). Same expression with each of the 4 loads hoisted into its
  own named int temp, in the exact declaration order matching the target
  — reproduces the load order and instruction shape byte-for-byte
  (10/10 words present, same opcodes in the same slots), **6/10 masked
  match**. The remaining 4-word mismatch is pure register allocation:
  target puts `+16` in `r3` and `+28` in `r2`; this candidate swaps them
  (same values, same operations, swapped physical registers). Permuting
  xor operand order and factoring the two xors into named intermediates
  neither fixes nor changes the allocation (the intermediate version is
  actually worse, 4/10, different load order entirely).
- Lead for next time: not yet tried — reordering the 4 declarations as
  b,a,d,c or a,b,d,c (same load-sequence constraint, different source
  variable identity per load) might flip which temp lands in `r2` vs
  `r3`.

**`0xd54c` (size `0x8`) is `ldrd r0,[r0,#16]; bx lr`** — a straight
64-bit field getter.

- Tried: `long long` return cast through `char*`, a struct with an
  8-byte-aligned `long long` field at the right offset, an 8-byte
  `{int,int}` struct-by-value return. Every phrasing compiles to either
  two separate `ldr`s (alignment not provable through the cast) or an
  indirect-return calling convention (`add`/`ldm`/`stm`, treating `r0`
  as a hidden return-pointer parameter instead of putting the pair
  directly in `r0:r1`) — never the single `ldrd` the retail binary uses
  for what should be the simplest possible getter shape.
- Stuck on: this looks like the same family as the already-documented
  vptr-scheduling/dtor residues — a toolchain-version default the
  fingerprint harness's flag set doesn't trigger from any C source tried
  so far. A sibling getter at `0xd560` (`ldrd r0,[r0,#24]; bx lr`, same
  size/shape) will have the identical problem and shouldn't be
  re-attempted until this one is solved.


## DllUSGymFighting@0xd64 (size 0x50)

- What it is: short-circuit AND of two virtual-call predicates, sibling
  of the matched `0xdb4` array-indexed predicate, same vtbl-slot-55
  (offset `0xdc`) idiom but non-indexed: `return this->f30->vtbl[55]
  (this->f30, 0) && this->f2c->vtbl[55](this->f2c, 0);`.
- Tried: four variants (locals in `a,b` order; `b,a` order; an explicit
  `self = this;` alias; direct inline field access with no
  intermediate locals). All four reproduce the retail instruction
  content for the calls and loads word-for-word, but every variant
  either uses the wrong register to hold the first operand across the
  call, or skips an extra `mov r1, r0` the retail binary performs right
  at function entry, before the `push`.
- Stuck on: best result 9/20 masked words — the middle (first virtual
  call) and tail (second virtual call, bool normalization, `pop`)
  match exactly; only the first ~4 words differ. Retail's shape:
  `mov r1, r0` (save `this` into `r1`) **before** `push {r4, lr}`, then
  after the push, `ldr r0,[r0,#48]` (load the first field still via the
  original `r0`) and `ldr r4,[r1,#44]` (load the second field via the
  *preserved* `r1` copy of `this`) — i.e. retail computes the two
  fields through two different base registers (one live `this`, one
  saved copy), rather than reading both off the same live pointer
  before it's clobbered (which every candidate did instead — one
  instruction shorter and arguably more optimal).
- Lead for next time: smells like a statement-count/optimizer-barrier
  effect rather than a load-order effect — maybe the real source has
  an intervening statement between the two field reads that this
  reconstruction is missing entirely, making `this` appear "live
  across a barrier" to armcc. Not tried: `register` hints, `volatile`
  on `this`, or wrapping the two field reads in a helper that takes
  `this` by value (forcing an AAPCS-style parameter copy). A fresh
  angle is needed, not more order permutations of the same three
  statements.

## DllVSDemo@0x2984 (size 0x20)

- What it is: prologue-less leaf, same idiom as the already-matched
  `DllDialogCommon@0x6e8` — loads an owned object pointer from
  `this+0x7c`; if null, returns 1; else tail-calls virtual slot 0 on it
  (`bx rX`) with the object itself as the new `this`.
- Tried: a nested struct with an `int (**vtbl)(O*)` field (same shape as
  the matched `0x6e8` reference), accessed directly or through an
  explicit intermediate `vt` local holding the vtable pointer before
  indexing; a fully untyped `void*` double-deref through casts with the
  function pointer assigned to its own local before calling — three
  variants across `-O1`/`-O2`/`-O3`/`-Ospace` (12 compiles total).
- Stuck on: 5/8 masked words best (variants 1/2, `-O2`/`-O3` tie). The
  null-check and final `bx lr`/early-return path match exactly; the
  two-instruction vtable-dereference-then-tail-call sequence never
  reproduces. The retail binary picks **r1** as its scratch register for
  both the vtable-pointer load and the function-pointer load
  (`ldr r1,[r0]; ldr r1,[r1]; bx r1`); every C idiom tried instead makes
  armcc 5.04u5 pick **r3** for the same two loads + tail branch —
  otherwise byte-identical structure, just the wrong register number
  throughout. `-O1` destabilizes further (moves the early-return
  `mov r0,#1` after the branch, dropping to 3/8); `-Ospace` drops the
  `beq` for a conditional `bxeq lr` (3/8, 7 words not 8).
- Lead for next time: unlike `0x6e8` (reached via a two-hop nested
  struct `t->p24->p4`), this specimen's object pointer is a single
  direct field (`t->obj` at `+0x7c`) — the shallower pointer chain may
  be exactly what changes armcc's register-pressure heuristic and
  picks `r3` over `r1`. Not tried: deliberately keeping the struct two
  levels deep (an unnecessary wrapper struct) purely to match `0x6e8`'s
  register-pressure profile, or adding an unused dead local variable
  before the vtable dereference to perturb the allocator's
  free-register ordering.


## DllField@0xae58 (size 0x10)

- What it is: no-branch, no-call leaf — zeroes two adjacent 4-byte int
  fields at `this+20` and `this+24`. Retail: `mov r2,#0; str r2,[r0,#20];
  str r2,[r0,#24]; bx lr`.
- Tried: (1) plain two-statement zero-assignment, no extra params — armcc
  allocates the zero constant into `r1` (lowest free scratch register),
  not `r2`. (2) same with an added-but-unused second parameter (`int
  unused`) hoping to push the scratch register up to `r2` — no change,
  still `r1`.
- Stuck on: 1/4 masked words (only the final `bx lr` matches). Retail
  picks `r2` for the zero constant, every candidate picks `r1`.
- Lead for next time: this looks like the same general register-role
  residue class already documented for `DllFieldDebug@0x180`/
  `DllUSGymFighting@0x11dc`/`DllUSGymIce@0x1974` (armcc picks the lowest
  free register for a materialized constant unless something external
  forces otherwise), but for a 32-bit value instead of 64-bit, and without
  an address-register competing for the pairing. Not tried: a genuinely
  used (not just unused) second parameter that itself occupies `r1`
  before the zero-store, or writing the two stores through an
  un-inlined helper taking `(dest_struct*, int)` so AAPCS parameter
  passing forces `r1` to already hold something else. A twin function at
  `0xe380`/`0xe3fc` elsewhere in the same module (identical shape, `+24`/
  `+28` and `+20`/`+24` field pairs) will have the same problem — don't
  re-attempt until this is solved.

## DllHologramMail@0x10da8 (size 0x24)

- What it is: same idiom as the matched `0xd424`/`0xd13c` (deref a pointer
  field at `this+36`+4, call an import with an argument + that object,
  store literal 11 into `this+40`), but `this` is **adjusted by -4** first
  (`sub r4, r0, #4`), and the called-with argument is the function's 2nd
  parameter (r1) instead of the 3rd. Retail bytes:
  ```
  push {r4, lr}
  sub  r4, r0, #4
  mov  r0, r1
  ldr  r1, [r4, #36]
  ldr  r1, [r1, #4]
  bl   <import 0xd10>
  mov  r0, #11
  strb r0, [r4, #40]
  pop  {r4, pc}
  ```
- Tried: `T *t = (T*)((char*)this_raw - 4); void *obj = *(void**)((char*)t->f36+4); import_d10(arg1, obj); t->f40=11;` (two statement orderings: obj-computed-as-its-own-statement-first, and obj-computed-inline-inside-the-call-expression). Both compile identically.
- Stuck on: 2/9 masked words. armcc 5.04u5 always **folds the -4 adjustment into the field offset** and reads the pointer field directly off the original (unmodified) r0 (`ldr r0,[r0,#32]` — offset 36-4), rather than going through the already-computed adjusted pointer in r4 (`ldr r1,[r4,#36]`) the way the retail binary does. It also swaps which register holds the outer argument during the computation (r2 instead of r0). This happens regardless of C statement order — it's the optimizer's own value-numbering choice (both expressions compute the same runtime address, so it picks whichever is cheaper for its internal scheduling), not something textual reordering influences.
- Lead for next time: this looks like the same general family as the
  already-documented vptr-scheduling/register-role residues (see
  `DllVSDemo@0x2984`, `DllUSGymFighting@0xd64` elsewhere in this file) but
  specifically about "this minus a small constant" never surviving as a
  materialized register when the compiler can re-derive the same address
  by adjusting an immediate offset instead. Not yet tried: making the `-4`
  adjustment opaque to value-numbering (e.g. route it through a
  `volatile`-qualified intermediate, or an un-inlined real function call
  that returns the adjusted pointer so the compiler can't algebraically
  unify it with the original `r0`). This same residue almost certainly
  also blocks two siblings in this module: `0x10dcc` (identical shape,
  different masked callee `0xd78` and uses the 3rd argument instead of the
  2nd — not independently attempted, assumed same problem on sight) and
  the first half of `0x10cf0` (size 0x34, a two-call compound function
  whose first half is this exact idiom at field offset +24 instead of
  +36 — also not independently attempted for the same reason).

## DllCommendation@0x7dc (size 0x2c) — stuck at 2/11 (2026-10-16 round c)

Like the matched `0x808`, but the import call takes (r0=arg2, r1=0x30, r2=0). Retail does `mov r0,r1` first then the constants; armcc emits `mov r3,r1` + an extra `mov r0,r3` in every C shape tried (void*/int/uchar arg types, local consts, reordered/extra args). Lead: unknown; maybe the arg is a re-loaded member or a wrapper inline.
