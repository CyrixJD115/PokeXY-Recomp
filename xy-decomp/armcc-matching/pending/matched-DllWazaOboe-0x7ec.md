Module: DllWazaOboe
Offset: 0x7ec
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, `return 1`, ignores `this`.
Note: First match in a new module. Sits right after the import-veneer table
(0x180-0x7eb) in a run of near-identical 1-2 word leaf stubs (0x7ec-0x837):
three `return 1` triplets at 0x7ec/0x7f4/0x7fc, bare `bx lr` weak-evidence
stubs at 0x804/0x818/0x81c/0x820, two more `return 1` at 0x808/0x810 and
0x824, then a field-setter variant at 0x82c (`mov r0,#1; strb r0,[r2,#12];
bx lr`, size 0xc) and a real function (push {r4-r8,lr}; vpush {d8}) at
0x838.
