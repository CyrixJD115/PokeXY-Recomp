/* DllPhotoViewer function @ CRO file offset 0x1918, size 0x8 — MATCHED (masked, 2/2). Trivial virtual-override stub: returns 1 (true), ignores `this`.
 *
 * One of a run of seven near-identical 1-2 word leaf stubs at 0x1918-0x193b
 * (two "return 1" pairs at 0x1918/0x1920, five bare `bx lr` at
 * 0x1928-0x1938), sitting right after a literal-pool word at 0x1914 (the
 * tail of a preceding function) and immediately before a real
 * push-{r4,lr}-prologue function at 0x193c. Same recurring trivial-stub
 * cluster shape already documented for every other Dll module mined so
 * far (default virtual-method overrides on the shared dialog-proc base
 * class — IsXxx() == true, no further evidence narrows the real name).
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllPhotoViewer 0x1918 0x8 xy-decomp/armcc-matching/matched/DllPhotoViewer/0x1918.c
 */

int stub_true(void *this)
{
    return 1;
}
