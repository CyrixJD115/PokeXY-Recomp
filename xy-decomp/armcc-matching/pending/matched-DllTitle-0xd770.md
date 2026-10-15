Module: DllTitle
Offset: 0xd770
Size: 0x8
Match: 2/2
One-line: Trivial int field getter (this+44), no calls.
Note: First function matched in this module. First real module function
seen starts at 0x7a8 (a uhadd8/uqsub8-heavy blend/compare routine, not
trivial); the easy cluster is further in, around 0xd530-0xd8ac, a dense
run of small getters/setters/predicates.
