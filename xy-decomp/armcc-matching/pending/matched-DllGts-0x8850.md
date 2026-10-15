Module: DllGts
Offset: 0x8850
Size: 0x28
Match: 10/10
One-line: double-indirect call+return helper — `fn284(this->field20->field5c, 1,1,1,1)`, returns 8.
Note: register-CSE-master trap. armcc's constant-CSE for 4 identical `1` args defaults to materializing into r1 (arg2's register) and copying outward, but retail materializes into r2 (arg3's register) instead — 6/10 words off otherwise. Fix: name ONLY the arg3 occurrence as its own local (`int v3 = 1;`), leave the others as literal `1`; that alone nudges armcc's CSE to pick r2 as master, matching retail exactly.
Follow-up queued (not committed, to respect this run's 5-function ceiling): the identical shape recurs at DllGts@0x12afc (field at offset 0x24 instead of 0x20, returns 7 instead of 8) — already verified to masked-match with the exact same C template (just swap the offset and the return constant). Should take under a minute for the next agent to save + commit.
