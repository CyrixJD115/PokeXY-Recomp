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
