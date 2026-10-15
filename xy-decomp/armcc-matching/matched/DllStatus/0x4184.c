/*
 * DllStatus @ 0x4184 (size 0x10) -- masked match 4/4.
 * calls f6280() then tail-calls f310()
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllStatus 0x4184 0x10 xy-decomp/armcc-matching/matched/DllStatus/0x4184.c
 */
extern void f6280(void);
extern void f310(void);
void fn(void){ f6280(); f310(); }
