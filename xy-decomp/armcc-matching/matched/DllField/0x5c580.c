// DllField@0x5c580, size 0x24, masked match (all words).
// Script handler: forwards b[1], b[2] with manager +12 -> +28 to a helper (tail call; retail loads the pair with ldm).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5c580 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x5c580.c

extern char *GetMgr(void);
extern int X(int, int, int);
int Fn(int a, int *b) {
    return X(*(int *)(*(char **)(GetMgr() + 12) + 28), b[1], b[2]);
}
