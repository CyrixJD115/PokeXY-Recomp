Module: DllTownmap
Offset: 0x24dc
Size: 0x8
Match: 2/2
One-line: Trivial int field setter: `str r1,[r0,#236]; bx lr` — stores the 2nd argument into the object at +0xec, void return.
Note: Same idiom as DllFieldEventPlayer@0x624/DllFieldEventTakinobori@0x11d4. A 3-word literal pool at 0x24d0-0x24d8 precedes it; the next function (push {r3,r4,r5,lr}) starts right after at 0x24e4.
