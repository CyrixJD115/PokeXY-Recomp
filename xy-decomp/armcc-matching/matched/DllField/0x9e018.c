// DllField@0x9e018, size 0x5c, 23/23 masked match.
// Gets mgr->+12 and the animation sub-object t at mgr->+12->+72->+240->+2740; reads two floats (A, B) from mgr->+12->+28->+40 and calls C(t+420, t, A, B); returns 0.
// Shared-accessor (GetMgr, callee 0x16c0) caller family; see pending/matched-DllField-0x9e018.md.
// Style: natural -- plain loads, two float-returning calls, call with float args
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x9e018 0x5c \
//       xy-decomp/armcc-matching/matched/DllField/0x9e018.c

extern char *GetMgr(void);
extern float A(char *);
extern float B(char *);
extern void C(char *, char *, float, float);
int Fn(void) {
    char *m = *(char **)(GetMgr() + 12);
    char *t = *(char **)(*(char **)(*(char **)(m + 72) + 240) + 2740);
    char *q = t + 420;
    float x = A(*(char **)(*(char **)(m + 28) + 40));
    C(q, t, x, B(*(char **)(*(char **)(m + 28) + 40)));
    return 0;
}
