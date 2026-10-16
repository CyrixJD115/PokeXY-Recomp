// DllField@0x5d4dc, size 0x28, masked match (all words).
// Getter: manager +12 -> +72 -> +240 object's byte at +16 == 0.
// Shared-accessor family (callee 0x16c0 = GetMgr).
// Style: natural
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5d4dc 0x28 \
//       xy-decomp/armcc-matching/matched/DllField/0x5d4dc.c

extern char *GetMgr(void);
int Fn(void) {
    return *(*(unsigned char **)(*(char **)(*(char **)(GetMgr() + 12) + 72) + 240) + 16) == 0;
}
