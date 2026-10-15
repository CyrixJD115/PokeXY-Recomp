/*
 * DllFieldEventSkatePipe @ 0x1b28, size 0x8, match 2/2.
 *
 * Trivial virtual-override stub: ignores `this`, returns 1 (`mov r0,#1;
 * bx lr`). Same recurring shape as every other mined module's tail-of-
 * import-veneer-table stub cluster (see armcc-matching/README.md). Sits
 * right after a field-copy-and-call helper ending at 0x1b24, and right
 * before a lone `bx lr` weak-evidence stub at 0x1b30 and a genuine field
 * accessor at 0x1b34 (`ldr r1,[r1,#28]; add r1,r1,#288; str r1,[r0,#36];
 * bx lr`) — both unclaimed, flagged here for a future agent.
 *
 * Verify:
 *   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py \
 *       DllFieldEventSkatePipe 0x1b28 0x8 \
 *       xy-decomp/armcc-matching/matched/DllFieldEventSkatePipe/0x1b28.c
 */
typedef struct Obj Obj;
struct Obj {
    int pad[1];
};

int stub_return_1(Obj *this) {
    (void)this;
    return 1;
}
