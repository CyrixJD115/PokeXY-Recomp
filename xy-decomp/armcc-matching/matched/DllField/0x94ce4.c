// DllField@0x94ce4, size 0x34, 13/13 masked match.
// Fn(self): fetches the manager; if self->+72 is non-null calls V(self72->+404); then W2(mgr->+28, 1); returns 1.
// Script-command handler / manager-accessor family (callee 0x16c0 = shared manager accessor).
// Style: trick(redundant-local) -- extra local q = o ? o->+404 : o steers armcc to keep self in r4 and mgr in r5 (retail order); without it regalloc swaps r4/r5.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x94ce4 0x34 \
//       xy-decomp/armcc-matching/matched/DllField/0x94ce4.c

extern char *GetMgr(void);
extern void V(int);
extern void W2(int,int);
int Fn(int self) {
    char *p=GetMgr();
    int o=*(int*)(self+72);
    int q=o?*(int*)(o+404):o;
    if(o) V(q);
    W2(*(int*)(p+28),1);
    return 1;
}
