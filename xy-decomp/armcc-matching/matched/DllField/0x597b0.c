// DllField@0x597b0, size 0x24, 9/9 masked match.
// Gets the manager object from the shared accessor (0x16c0), walks the pointer chain +12 -> +28 -> +0 -> +13328, adds a constant field offset to the loaded base pointer and tail-calls a helper with the resulting address (`pop {r4,lr}; b`).
// Shared-accessor thunk family (callee 0x16c0); siblings listed in pending/.
// Style: natural -- plain call, pointer-chain loads, helper call; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x597b0 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x597b0.c

extern char *GetMgr(void);
extern int Tail(char *);
extern void Sink(char *);
int Fn(void) {
    char *p = GetMgr();
    return Tail(*(char **)(*(char **)(*(char **)(p + 12) + 28) + 0) + 13328);
}
