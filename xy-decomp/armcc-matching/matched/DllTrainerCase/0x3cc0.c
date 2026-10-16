/* DllTrainerCase function @ CRO file offset 0x3cc0, size 0x8 — MATCHED
 * (masked, 2/2). Fifth match in this module.
 * Trivial sub-object accessor: returns a pointer to a member embedded
 * at `this+16` (`add r0,r0,#16; bx lr`) — no load, just pointer
 * arithmetic, the classic "get the embedded sub-object" idiom.
 * Three near-identical siblings follow in the same run (`0x3cc8` +4,
 * `0x3cd0` +28, `0x3cd8` +8 — untried), right after a thin tail-call
 * forwarder (`0x3cb8`, `b <import>`) and a bare `bx lr` stub (`0x3cbc`),
 * both also untried.
 * Style: natural
 *
 * Verify: XY_ROMFS=.Data/Raw_Extracted/romfs python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllTrainerCase 0x3cc0 0x8 xy-decomp/armcc-matching/matched/DllTrainerCase/0x3cc0.c
 */

typedef struct X {
    unsigned char pad[0x10];
    unsigned char member[4]; /* +0x10, embedded sub-object */
} X;

void *get_member(X *this)
{
    return &this->member;
}
