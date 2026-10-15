Module: DllShinkaDemo
Offset: 0xfdc
Size: 0x8
Match: 2/2
One-line: Trivial signed-byte field getter, `return *(signed char*)(obj+0x82);` -- no calls, no branches.
Note: Seventy-first-ish matched module (not yet renumbered in README). Sits right
  after a dispatcher function ending at 0xfd4 and its one-word literal pool at
  0xfd8. An identical twin getter for the adjacent byte (+0x83) sits right next
  door at 0xfe4, unclaimed -- an easy follow-up for another agent. The module's
  import-veneer table runs 0x180-0x807 (unusually long for a demo module);
  first real function starts at 0x808.
