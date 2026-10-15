Module: DllSubScreenTutorial
Offset: 0x1338
Size: 0x8
Match: 2/2
One-line: Trivial virtual-override stub, return 0, ignores this.
Note: Sits right after the known-bad member-load-first sibling destructor at 0x1304 (same idiom as DllFatalErrorProc@0x2bc but wrong scheduling order -- skipped on sight) and before a real function at 0x1340.
