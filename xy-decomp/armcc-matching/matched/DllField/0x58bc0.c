// DllField@0x58bc0, size 0x24, 9/9 masked match.
// Script handler: Imp(a, (s16)args[1], 2, 9, 0).
// Shared-accessor (GetMgr, callee 0x16c0) script-command handler family.
// Style: trick(volatile-load) -- args pointer read through volatile so armcc keeps the full-word ldr + separate uxth/sxtb/and (retail) instead of folding into ldrh/ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x58bc0 0x24 \
//       xy-decomp/armcc-matching/matched/DllField/0x58bc0.c

extern void Imp(int, int, int, int, int);
void Fn(int a, volatile int *b) {
    int t = b[1];
    Imp(a, (short)t, 2, 9, 0);
}
