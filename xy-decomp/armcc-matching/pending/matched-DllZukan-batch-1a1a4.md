Module: DllZukan
Five trivial functions, all MASKED MATCH (see matched/DllZukan/<offset>.c):
- 0x1a1a4 size 0x8  2/2  getter long at this+0x20
- 0x1a3f0 size 0x8  2/2  byte setter this+0x71
- 0x1bad8 size 0x14 5/5  bool getter byte this+0x0c == 3
- 0x1be6c size 0xc  3/3  indexed getter long arr[i] at this+0x38
- 0x1d698 size 0x10 4/4  clears {u32 @0, u16 @4}
Note: more easy siblings remain in DllZukan: 0x155b4/0x156c4/0x1576c (ldr r0,[r0,#4]; strb r1,[r0,#242/232/243]), 0x1baec (ldr #116), 0x1bad0 (ret 0), 0x1dcc0..0x1dce8 (add r0,r0,#N getters), 0x1be18/0x1be20/0x1be28/0x1bdb8 (mov r0,#N).
