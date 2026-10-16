// DllField@0x5e534, size 0x38, masked match (all words).
// Script handler: u16 b[1] -> virtual call (vtable slot 7) on the object at (*(manager +12 -> +28)) + 0x190c4.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: trick(volatile) -- b read through a volatile pointer only so armcc emits ldr+narrow (uxth/and) instead of ldrh/ldrb.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5e534 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x5e534.c

extern char *GetMgr(void);
extern int F(int, int);
extern int G(int, int, int);
typedef int (*FP)(int *, int);
int Fn(int a, volatile int *b) {
    unsigned short x = b[1];
    int *o = (int *)(*(int *)*(char **)(*(char **)(GetMgr() + 12) + 28) + 0x190c4);
    return ((FP)(*(int **)o)[7])(o, x);
}
