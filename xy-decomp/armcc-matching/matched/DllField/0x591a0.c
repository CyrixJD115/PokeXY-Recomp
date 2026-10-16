// DllField@0x591a0, size 0x10, 4/4 masked match.
// Calls the shared manager accessor (0x16c0), discards the result and returns 0
// (an event-handler stub whose only effect is the accessor call).
// Style: natural -- plain call, result unused, return 0.
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x591a0 0x10 \
//       xy-decomp/armcc-matching/matched/DllField/0x591a0.c

extern char *GetMgr(void);
int Fn(void) {
    GetMgr();
    return 0;
}
