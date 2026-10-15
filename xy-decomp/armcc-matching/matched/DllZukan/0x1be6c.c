/* DllZukan function @ CRO file offset 0x1be6c, size 0xc — MATCHED (masked, 3/3).
 * Indexed getter: this->arr[i] with long array at +0x38.
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllZukan 0x1be6c 0xc xy-decomp/armcc-matching/matched/DllZukan/0x1be6c.c
 */
typedef struct X { unsigned char pad[0x38]; long arr[4]; } X;
long get_arr(X *o, int i) { return o->arr[i]; }
