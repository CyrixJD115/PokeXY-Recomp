Module: DllPoke3Select
Offset: 0x63c
Size: 0x28
Match: 10/10
One-line: Constructor — installs a vtable pointer, zeroes fields +0x28/+0x34/+0x38, sets byte flag +0x30 to 1; no calls/branches.
Note: Sixty-sixth module touched. Register-role trap: armcc keeps constant 0 live in r1 across all three zero-stores and constant 1 live in r2 for one store before reusing r2 (now dead) for the vtable literal load. Reaching this exact r1/r2 split required declaring both constants as separate named locals (`zero`, `one`) assigned *before* any field write — using inline immediates or omitting the named constants kept the same instruction shape/count but swapped which physical register held which value (2/10, not 10/10). A weak-evidence lone `bx lr` stub immediately follows at 0x664, and a call-through wrapper (push/bl/pop/b-forwarder) follows that at 0x668 — both unclaimed, easy next leads for this module.
