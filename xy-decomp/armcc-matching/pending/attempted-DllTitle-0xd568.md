Module: DllTitle
Offset: 0xd568 (and a related open item at 0xd54c)
Size: 0x28 (0xd568), 0x8 (0xd54c)
What: two stuck candidates in the same dense getter/setter/predicate
cluster (0xd530-0xd8ac) as this module's four matched functions.

0xd568 (size 0x28) is a 4-int equality predicate: loads this+16/+24/+28/+20
(in that exact order) into r3/r1/r2/r0, then XORs the +16/+24 pair and the
+20/+28 pair, ORs the two diffs, and returns 1 if zero else 0 (classic
`(a==b) && (c==d)` compiled branchless via xor/or rather than cmp/cmp/and).

Tried, in order:
  - `(p[4]==p[6]) && (p[5]==p[7])` -- wrong shape entirely, armcc kept it
    as short-circuit branches (12/10 words, totally different structure).
  - `(p[4]==p[6]) & (p[5]==p[7])` -- still branchy (cmp+moveq/movne per
    compare then AND), not the xor/or trick.
  - `((p[6]^p[4])|(p[5]^p[7]))==0` direct on a flat `int *p` -- gets the
    right *op* shape (xor/xor/orr/moveq/movne) but armcc evaluates the two
    xor subexpressions one at a time (load both operands of the first xor,
    compute it, then load both operands of the second), 5/10 match.
  - Same expression but with each of the 4 loads hoisted into its own
    named int temp (`int a=p[4]; int b=p[6]; int c=p[7]; int d=p[5];
    return ((b^a)|(d^c))==0;`) in that exact declaration order -- this DOES
    reproduce the target's load order byte-for-byte (16,24,28,20) and the
    overall instruction shape (10/10 words present, same opcodes in the
    same slots), 6/10 masked match. The remaining 4-word mismatch is pure
    register allocation: target puts the +16 value in r3 and the +28 value
    in r2; this candidate puts +16 in r2 and +28 in r3 -- same values, same
    operations, swapped physical registers. Tried permuting xor operand
    order (a^b vs b^a, c^d vs d^c) and factoring the two xors into their
    own named intermediate variables (x=b^a; y=d^c; return (x|y)==0) --
    neither changes nor fixes the allocation; the explicit-intermediate
    version is actually worse (4/10, different load order entirely).
  - Not yet tried: reordering the 4 declarations as b,a,d,c or a,b,d,c
    (same load-sequence constraint, different source variable identity per
    load) might flip which temp lands in r2 vs r3 -- next agent's best lead.

0xd54c (size 0x8) is `ldrd r0,[r0,#16]; bx lr` -- a straight 64-bit field
getter. Every C phrasing tried (`long long` return cast through `char*`,
a struct with an 8-byte-aligned `long long` field at the right offset, an
8-byte `{int,int}` struct-by-value return) compiles to either two separate
`ldr`s (alignment not provable through the cast) or an indirect-return
calling convention (`add/ldm/stm`, treating r0 as a hidden return-pointer
parameter instead of putting the pair directly in r0:r1) -- never the
single `ldrd` the retail binary uses for what should be the simplest
possible getter shape. This looks like the same family as the
already-documented vptr-scheduling/dtor residues: a toolchain-version
default the fingerprint harness's flag set doesn't trigger from any C
source tried so far. A sibling getter at 0xd560 (`ldrd r0,[r0,#24]; bx lr`,
same size/shape) will have the identical problem and shouldn't be
re-attempted until this one is solved.

Verify (to reproduce the current state):
  XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
    xy-decomp/armcc-matching/tools/fingerprint_fn.py \
    DllTitle 0xd568 0x28 <candidate.c>
  XY_ROMFS=.Data/Raw_Extracted/romfs python3 \
    xy-decomp/armcc-matching/tools/fingerprint_fn.py \
    DllTitle 0xd54c 0x8 <candidate.c>
