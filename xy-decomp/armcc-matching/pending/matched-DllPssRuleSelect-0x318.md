Module: DllPssRuleSelect
Offset: 0x318
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: A seventy-fourth matched module. Sits right after the import-veneer
table (0x180-0x317, 51 entries): an identical twin stub follows
immediately at 0x320, then two bare `bx lr` single-word stubs at
0x328/0x32c (weak-evidence class, unclaimed), then the module's first
real function (a conditional field-reset leaf: `if (*(this->field_40)
== 4) { *that byte = 0; this->field_40[2] = 0; }`, size 0x20 at 0x330,
untried) and its near-twin set-to-4 variant at 0x354 (size 0x1c,
untried) — both look tractable for a future agent.
