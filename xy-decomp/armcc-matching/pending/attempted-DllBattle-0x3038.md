## DllBattle@0x3038 (size 0x6c)

- What it is: same body as matched 0x30a4 but retail loads the vtable pool word into r1 BEFORE `push {r4,lr}`, and ends with tail `b`.
- Tried: 0x30a4 shape plus tail call to extern fc(r).
- Stuck on: 21/26; only the first 5 words differ (ldr r1,pool hoisted above push, str r1,[r0] before mov r4,r0).
- Lead for next time: none yet.
