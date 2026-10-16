# Style audit, slice 8 (2026-10-16)

Slice: `audit_slice_8.txt` (157 files; DllStatus .. DllZukan matched/) plus all of `poolpad/` (10) and `relaxed/` (1) = **168 files**. Every file was re-verified after editing at the flags in its header (all 168 PASS: masked / pool-pad / relaxed as applicable).

## (a) Summary

| metric | count |
|---|---|
| files audited | 168 |
| Style tag unchanged (explicit tag already present, still correct) | 38 |
| no Style line before -> explicit `Style: natural` | 124 (incl. relaxed/0x30a4) |
| retagged natural -> trick | 6 (all `trick(redundant-local)`, poolpad/DllBattle) |
| retagged trick -> natural | 0 |
| trick kind corrected | 0 (DllField poolpad why-text extended, kind kept) |
| unnecessary tricks removed (code rewritten plain) | 0 |
| flagged not-GameFreak-like (asm / intrinsics / pragma / register / goto) | 0 |
| header-only fixes (stale text / wrong Verify) | 4 (GymIce@0x1974 Verify named 0x180; relaxed/0x30a4 Verify path; DllStatus@0xa4e0 "attempt"; Kosaka description) |
| code changed | none (headers only) |

Tricks tested by compiling a plain variant (all FAIL without the trick, so all kept):
- `ordering-local` dtors with tail call: DllStatus@0xa304 / DllTownmap@0x31d4 (identical bodies), DllSubScreenTutorial@0x12cc, DllTrainerCase@0x37e4, DllTrialDownload@0x728, DllTradelist@0xc04 (const-typedef VT form), DllTrialHouseResult@0x2648: plain `p = o->buf; o->vptr = VT;` fails in every one (the non-tail-call dtor 0xa33c/0x2680 form is natural and matches plain).
- `extra-arg`: DllTownmap@0xb10, 0xd68 (and by sibling 0x3554, 0x35f4): dropping the forwarded int arg(s) fails.
- `split-expression` DllTitle@0xd534: fused `*(int*)this = *(short*)(src+2)` fails.
- `redundant-local` DllUSGymEsper@0x480 (`(v != 0)` and if/else forms both fail), DllWazaOmoidashi@0x924 (literal `1` store fails).
- `volatile-load` DllUSGymIce@0x278: plain `int` and plain `signed char` field both fail.
- `volatile-strd` (DllTestSuzuki@0x180 representative; 8 more + DllTownmap@0x3610 same body): plain `long long` fails (stm instead of strd).
- poolpad `k` locals and DllField declaration-order/locals: see (b).

## (b) Every file whose tag or text changed

| path (under xy-decomp/armcc-matching/) | old Style | new Style | what changed / why |
|---|---|---|---|
| matched/DllStatus/0x6f6c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllStatus/0x871c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllStatus/0xa370.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllStatus/0xa4e0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code (+ "masked match attempt" -> "masked match": it verifies) |
| matched/DllStrInput/0x6148.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllStrInput/0x666c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllStrInput/0x6674.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllSubScreenTutorial/0x12a0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllSubScreenTutorial/0x1338.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllSubScreenTutorial/0x538.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllSubScreenTutorial/0x954.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTestKosaka/0x180.c | trick(volatile-strd) | trick(volatile-strd) | description line clarified only |
| matched/DllTestPGL/0x180.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTheEnd/0x750.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTitle/0xd770.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTitle/0xd778.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTitle/0xd7a8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x1530.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x1538.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x24dc.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x2dc0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x2ec8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x3240.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x3248.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x438.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x46c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTownmap/0x488.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTradeDemo/0x1a68.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTradeDemo/0x1a70.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTradeDemo/0x550.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTradeDemo/0x644.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTradelist/0x2dac.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTradelist/0xb40.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTradelist/0xb5f0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTradelist/0xc70.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x37a4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x37ac.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x37b4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x37bc.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x37cc.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x3850.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x3cc0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x3cc8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainerCase/0x3cd0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrainingAppManager/0xd68.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrialDownload/0x6fc8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrialDownload/0x6fd0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrialDownload/0x6fd8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrialDownload/0x700.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrialHouseResult/0x2644.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrialHouseResult/0x2680.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrialHouseResult/0x26b4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllTrialHouseResult/0x4c8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUS3DDun/0x24c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSBadgeCheckGate/0x228.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSBigFourDragon/0x2cc.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSBigFourFire/0x31c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSBigFourSteel/0x2f4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSBigFourWater/0x314.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSC02/0x708.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSDynamo/0x1b4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymElec/0x268.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymEsper/0x2fc.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymEsper/0x304.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymEsper/0x328.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymEsper/0xad4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymFairy/0x224.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymFighting/0x11c0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymFighting/0x278.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymFighting/0x3f4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymFighting/0xdb4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymGrass/0x22c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymIce/0x11f4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymIce/0x1974.c | trick(volatile-strd) | trick(volatile-strd) | tag unchanged; header Verify line named 0x180 (copy-paste), corrected to 0x1974 |
| matched/DllUSGymIce/0x25c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymInsect/0x344.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSGymRock/0x1d4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSKinomi/0xf40.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSKinomi/0xf44.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSKinomi/0xf4c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSKinomi/0xf54.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSKinomi/0xf5c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSMirror/0x1cc.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSPassage/0x1078.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSPassage/0x1610.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSPassage/0x1620.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSPassage/0x334.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSPassage/0x750.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSPokecen/0x24c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSPokecen/0x254.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSR13/0x24c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllUSR15/0x24c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllVSDemo/0x1a60.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllVSDemo/0x1b00.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllVSDemo/0x2b28.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllVSDemo/0x3e0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllVSDemo/0x408.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOboe/0x1020.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOboe/0x1024.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOboe/0x7ec.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOboe/0x804.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOboe/0x82c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOmoidashi/0x18b4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOmoidashi/0x23d4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOmoidashi/0x3514.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllWazaOmoidashi/0x5f0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x155b4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x156c4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1576c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1a1a4.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1a3f0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1bad8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1baec.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1be54.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1be5c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1be64.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1be6c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1d698.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1dcc0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1dcc8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1dcd0.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0x1dce8.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0xf9c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| matched/DllZukan/0xfac.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| poolpad/DllBattle/0x3b534.c | natural | trick(redundant-local) | named local `k` for the stored constant is required: literal-twice form fails `--pool-pad` (tested); code unchanged |
| poolpad/DllBattle/0x3bdd8.c | natural | natural (note added) | `k` is also passed to the callee; literal-inlined form fails, noted |
| poolpad/DllBattle/0x3e7a4.c | natural | trick(redundant-local) | named local `k` for the stored constant is required: literal-twice form fails `--pool-pad` (tested); code unchanged |
| poolpad/DllBattle/0x43aec.c | natural | trick(redundant-local) | named local `k` for the stored constant is required: literal-twice form fails `--pool-pad` (tested); code unchanged |
| poolpad/DllBattle/0x43fa8.c | natural | trick(redundant-local) | named local `k` for the stored constant is required: literal-twice form fails `--pool-pad` (tested); code unchanged |
| poolpad/DllBattle/0x4400c.c | natural | trick(redundant-local) | named local `k` for the stored constant is required: literal-twice form fails `--pool-pad` (tested); code unchanged |
| poolpad/DllBattle/0x71208.c | natural | trick(redundant-local) | named local `k` for the stored constant is required: literal-twice form fails `--pool-pad` (tested); code unchanged |
| poolpad/DllBattle/0x71a2c.c | none | natural | no Style line before; plain stub/getter/setter/forwarder/struct-offset code |
| poolpad/DllField/0xb6844.cpp | trick(declaration-order) | trick(declaration-order), why text extended | no-locals and a-before-b forms both fail, so locals and order both needed |
| poolpad/DllField/0xb76c8.cpp | trick(declaration-order) | trick(declaration-order), why text extended | no-locals and a-before-b forms both fail, so locals and order both needed |
| relaxed/DllBattle/0x30a4.c | none | natural | added Style; Verify path pointed at matched/ (stale), now relaxed/ |

## (c) FLAGGED -- cannot be made natural (known tricks that stay tagged)

All verified 2026-10-16 to fail in the plain form:

- `volatile long long` global for `strd`: DllTestIchiraku/Kawada/Kosaka/Nagihashi/Suzuki@0x180, DllUSGymFighting@0x11dc, DllUSGymIce@0x1974, DllTownmap@0x3610.
- `volatile int field` read as `signed char`: DllUSGymIce@0x278 (`volatile-load`).
- `vt = VT` local before the member load (`ordering-local`): DllStatus@0xa304, DllTownmap@0x31d4, DllSubScreenTutorial@0x12cc, DllTradelist@0xc04, DllTrainerCase@0x37e4, DllTrialDownload@0x728, DllTrialHouseResult@0x2648.
- forwarded unused int args (`extra-arg`): DllTownmap@0xb10, 0x3554, 0xd68, 0x35f4.
- split load (`split-expression`): DllTitle@0xd534.
- steering locals (`redundant-local`): DllUSGymEsper@0x480, DllWazaOmoidashi@0x924, and the six poolpad/DllBattle files retagged here.
- `declaration-order` + named locals: poolpad/DllField@0xb6844, 0xb76c8.

Nothing in the slice uses inline asm, intrinsics, pragmas, `register`, attributes or `goto`.

## (d) Patterns / lessons

- **124 of 168 files had no Style line** (they were "natural by default"). All of those are leaf stubs, getters/setters, add-offset accessors, thunks and a few small switch/vcall bodies; after reading each, none contains a steering construct. Caveat as in the README: tiny stubs (`return 0`, `bx lr`, `(void)this;`) compile identically from any source, so natural there is weak evidence.
- **`declaration-order` is a kind used in the files but missing from the README kinds table** (DllField poolpad x2). Suggest adding it (or folding it into `ordering-local`).
- **The poolpad "trace-prefix" idiom** (`int k = C; if (c->v != k) c->v = k;`, a store-if-different of a constant into `c+0x6c`, c = 3rd arg) appears in 8 poolpad files. In six the local `k` exists only for that store and the literal-twice form fails, so they are now tagged `redundant-local`. If the real source is a trace macro with a temp, a single macro definition would make this natural across all of them; worth identifying in the callee/caller context. In 0x3bdd8 and 0x71a2c `k` is also passed on to the callee, so the local is ordinary there (still required for the match).
- `int unused` placeholders before `c` in poolpad files are arg-slot RE placeholders (c is the 3rd arg), accepted as natural.
- The `extra-arg` tag may be over-strict: the retail wrappers do forward r1 unchanged, so a real forwarded argument is plausible; the one-/two-arg shape is needed only because armcc's register choice depends on the declared arg count. Kept as trick since true signature is unknown.
- Copy-paste hazard in headers: DllUSGymIce@0x1974 carried the Verify line of 0x180 (the file still verified when run with the right offset). The other Test*/globals-init siblings were correct.
