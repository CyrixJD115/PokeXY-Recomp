/* DllGts function @ CRO file offset 0x93ac, size 0x34 -- MATCHED (masked) 13/13.
 * If import fn(this) is nonzero returns this->r@0x1c; else if this->byte4 set,
 * stores 6 to this->r@0x14 and returns 1; else returns 2.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllGts 0x93ac 0x34 xy-decomp/armcc-matching/matched/DllGts/0x93ac.c
 */
typedef struct X { unsigned char pad[4]; unsigned char f4; unsigned char pad1[0xf]; int r14; int padx; int r1c; } X;
extern int fn(X *);
int g(X *o) {
  if (fn(o)) return o->r1c;
  if (o->f4) { o->r14 = 6; return 1; }
  return 2;
}
