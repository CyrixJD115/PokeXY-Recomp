Module: DllConfig
Offset: 0x31b8
Size: 0x20
Match: 5/8 (best reached)
One-line: No-branch, no-call field-copy helper. `r1` points to a 3-word source struct (f0,f4,f8); copies those into `r0`'s fields at +0x38,+0x3c,+0x40, and *also* re-copies f0/f4 into +0x44/+0x48 (so f0/f4 are each written twice, to two different destinations).

Tried:
1. Flat field-by-field assignment (`d->f38=s->f0;` etc, every statement order tried) — compiles to 5 separate ldr/str pairs, no merging at all (11 words vs target's 8).
2. `d->sub = *s;` with a 3-word nested sub-struct at +0x38, plus the two extra re-reads — triggers the target's `ldm r1,{r2,r3,ip}` on the read side exactly, but the write side comes out as `add r0,r0,#0x38` + `stmia r0!,{r2,r3,ip}` (one merged 3-word post-increment store), not the target's `str ip,[r0,#0x40]` followed by `strd r2,[r0,#0x38]`.
3. Same but via an explicit local `S tmp = *s;` then three separate per-field stores from `tmp` in the same statement order the target stores them (far field first, then the adjacent pair) — identical over-merge result as #2, byte-for-byte.

Stuck on: 5/8 masked words — the `ldm` line and the two re-read/re-store lines for +0x44/+0x48 already match exactly. The 3 words in between never reproduce: armcc 5.04u5 under -O2 -Otime either doesn't merge adjacent stores at all, or merges all three contiguous stores into one block op — never the target's partial merge (1 lone str + 1 strd pair, skipping the 3-word stm). Source statement order didn't change the outcome in either the flat or struct-copy case; the optimizer appears to pick the merge shape purely by cost, irrespective of how the C expresses it.

Lead for next time: maybe the real destination isn't one flat struct — e.g. the far field (+0x40) could belong to a different (possibly differently-typed) member or a union arm that breaks armcc's "is this one aggregate" merge check while the +0x38/+0x3c pair stays recognized as adjacent. Not tried: -O1/-O3/-Ospace flag variants, or forcing the pair through its own 2-word sub-struct while leaving +0x40 reached via a genuinely unrelated C expression (different pointer variable) rather than the same `d` base.
