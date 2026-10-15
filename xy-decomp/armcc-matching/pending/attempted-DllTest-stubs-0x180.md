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
