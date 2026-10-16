// DllField@0x92e54, size 0x34, 13/13 masked match.
// Gets obj=mgr->+12->+72->+240->+2740; if Q(obj+844) is false, clears obj byte +232 and returns 1 (predicated strb/mov), else returns 0.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: natural -- plain call, pointer-chain loads; no tricks.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x92e54 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x92e54.c

extern char *GetMgr(void);
extern int Q(char*);
int Fn(int self) {
    char *p=GetMgr();
    char *r=*(char**)(*(char**)(*(char**)(*(char**)(p+12)+72)+240)+2740);
    if(!Q(r+844)) {
        r[232]=0;
        return 1;
    }
    return 0;
}
