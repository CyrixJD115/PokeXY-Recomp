Module: DllBattle
Offset: 0x1fc4
Size: 0x8
Match: 2/2
One-line: Pointer-offset accessor, `return (char*)this + 2912;` (no dereference).
Note: Right between the two 0x1fbc/0x1fd0 `return 0` stubs. Right after it at 0x1fcc is a 1-word tail-call forwarder (`b` to the module's import-veneer table at 0x19c) -- same thin-forwarder idiom documented elsewhere in this project, untried here.
