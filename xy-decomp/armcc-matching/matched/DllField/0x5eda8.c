// DllField@0x5eda8, size 0x24, masked match (all words).
// Script handler: forwards word b[1] with manager +12 -> +28 -> +36 object to a helper (tail call).
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5eda8 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x5eda8.c

extern char *GetMgr(void);
extern int F(int, int);
int Fn(int a, int *b) {
    int x = b[1];
    return F(*(int *)(*(char **)(*(char **)(GetMgr() + 12) + 28) + 36), x);
}
