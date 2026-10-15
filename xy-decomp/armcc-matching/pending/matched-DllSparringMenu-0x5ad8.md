Module: DllSparringMenu
Offset: 0x5ad8
Size: 0x34
Match: 13/13
One-line: Destructor: vptr install, free member @+0x14, returns `this` (ARM C++ ABI).
Note: Same idiom already proven in DllDialogCommon@0xcdc and many other modules
(member-load-before-vtable-load scheduling order — the "good" ordering).
