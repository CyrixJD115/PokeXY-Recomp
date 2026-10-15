/* DllHeading@0x55d8 (size 0x48) -- masked match.
 * Style: natural
 * Indexed table lookup (8-byte entries, byte @+4): calls two imports on sub-object @+4 with the entry byte; the second takes a stack arg 1.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllHeading 0x55d8 0x48 xy-decomp/armcc-matching/matched/DllHeading/0x55d8.c
 */
typedef struct E { int a; unsigned char b; unsigned char pad[3]; } E;
typedef struct X { void *vptr; unsigned char sub[0xc]; int f; } X;
extern E TAB[];
extern void fn298(void *s, int f, int b, int z);
extern void fn1d8(void *s, int f, int b, int z, int one);
void Fn(X *o, int idx)
{
    E *e = &TAB[idx];
    fn298(o->sub, o->f, e->b, 0);
    fn1d8(o->sub, o->f, e->b, 0, 1);
}
