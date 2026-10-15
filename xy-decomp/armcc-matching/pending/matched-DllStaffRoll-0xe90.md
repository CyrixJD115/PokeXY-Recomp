Module: DllStaffRoll
Offset: 0xe90
Size: 0x10
Match: 4/4
One-line: Calls a function on `this` (result discarded), unconditionally returns 1.
Note: `push {r4,lr}`/`pop {r4,pc}` is a real stack frame here (the body makes a genuine `bl` call), unlike the padding-only variant at 0x2304. Sits right before a larger property-setter function (0xea0) that was not attempted.
