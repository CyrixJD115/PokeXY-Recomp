/*
 * DllBag @ 0x1382c (size 0x10) -- masked match 4/4.
 * calls fc78() then tail-calls f300().
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllBag 0x1382c 0x10 xy-decomp/armcc-matching/matched/DllBag/0x1382c.c
 */
extern void fc78(void);
extern void f300(void);
void fn(void){ fc78(); f300(); }
