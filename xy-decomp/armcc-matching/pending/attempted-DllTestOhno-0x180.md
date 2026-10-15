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
