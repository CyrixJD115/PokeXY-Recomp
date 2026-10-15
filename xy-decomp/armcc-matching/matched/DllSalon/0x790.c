/* DllSalon function @ CRO file offset 0x790, size 0x20 — MATCHED (masked, 8/8).
 * Style: trick(void-ptr-passthrough) -- callee declared `void*`, result offset `(char*)fn(p) - N`, to avoid a spare `mov r4,r0`
 * Secondary-base thunk: calls an import with no args, installs a vtable at
 * the returned object via post-indexed store (+0x24), calls a second import
 * on the advanced pointer, returns that result minus 0x24. Own literal pool
 * word at 0x7ac => size 0x20.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllSalon 0x790 0x20 xy-decomp/armcc-matching/matched/DllSalon/0x790.c
 */
extern void *VT[];
extern void *getbase2a0(void);
extern void *fin2a8(void *p);

void *Thunk(void)
{
    void **p = (void **)getbase2a0();
    *p = VT;
    return (char *)fin2a8((char *)p + 0x24) - 0x24;
}
