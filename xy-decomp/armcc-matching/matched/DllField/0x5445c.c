// DllField@0x5445c, size 0x38, 14/14 masked match.
// Call+return helper: calls an import veneer with (this, this->byte@10,
// 0, 2, 8) -- 5th arg passed on the stack -- then calls a second import
// veneer with (this->field0, 33), and returns 1 unconditionally.
// Argument evaluation order matches armcc's typical right-to-left scheme:
// writing the call as ImportA(this, byte, 0, 2, 8) reproduces retail's
// exact instruction order (stack arg first, this loaded last right
// before the call). find_candidates.py's reported size (0x40) rounds up
// to the next catalogued function and includes two trailing one-word
// stubs (a `b <veneer>` thin forwarder and a bare `bx lr` no-op) that
// are separate, already-tail-of-function candidates, not part of this
// body -- the real function ends at the `pop {r3,r4,r5,pc}` at 0x54490,
// size 0x38 (not 0x34 as a naive single-bl guess would suggest, and not
// 0x40 either).
//
// Verify:
//   python3 xy-decomp/armcc-matching/tools/fingerprint_fn.py DllField 0x5445c 0x38 \
//       xy-decomp/armcc-matching/matched/DllField/0x5445c.c

extern void ImportA(void *, int, int, int, int);
extern void ImportB(void *, int);

int Fn(void *this) {
    ImportA(this, *(unsigned char *)((char *)this + 10), 0, 2, 8);
    ImportB(*(void **)this, 33);
    return 1;
}
