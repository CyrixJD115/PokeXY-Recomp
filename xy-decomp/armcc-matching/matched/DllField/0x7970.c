/* DllField function @ CRO file offset 0x7970, size 0xc -- MATCHED (masked, 3/3).
 * sets byte field this[28]=1
 * Style: natural
 *
 * Verify: python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *     DllField 0x7970 0xc xy-decomp/armcc-matching/matched/DllField/0x7970.c
 */

void Fn(void *this) { *((unsigned char *)this + 28) = 1; }
