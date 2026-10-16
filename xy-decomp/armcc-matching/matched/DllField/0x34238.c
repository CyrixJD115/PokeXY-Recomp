/*
 * DllField @ 0x34238 (size 0x10) -- masked match 4/4.
 *
 * Two fixed no-arg calls, void return: FunA(); FunB(); -- the second
 * call is tail-called (push {r4,lr} / bl FunA / pop {r4,lr} / b FunB,
 * no explicit epilogue after the tail branch). The push/pop of r4 is
 * AAPCS stack-alignment padding, not a real register save (same
 * pattern already documented for DllDialogCommon@0x6c4).
 * Style: natural
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField \
 *     0x34238 0x10 xy-decomp/armcc-matching/matched/DllField/0x34238.c
 */

extern void FunA(void);
extern void FunB(void);

void Func(void) {
    FunA();
    FunB();
}
