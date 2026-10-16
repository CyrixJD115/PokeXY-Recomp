// DllField@0x980fc, size 0x30, 12/12 masked match.
// Script-command handler: reads args[1] (low byte), calls a helper on mgr->+12->+28->*->+0x15fa8 with it, returns the result as u16.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x980fc.md.
// Style: trick(volatile-load) -- args is volatile int* only so armcc keeps ldr then and #255 instead of folding to ldrb
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x980fc 0x30 \
//       xy-decomp/armcc-matching/matched/DllField/0x980fc.c

extern char *GetMgr(void);
extern int Q(char *, int);
unsigned short Fn(int self, volatile int *a) {
    int v = a[1];
    char *p = GetMgr();
    return Q(*(char **)(*(char **)(*(char **)(p + 12) + 28)) + 0x15fa8, v & 255);
}
