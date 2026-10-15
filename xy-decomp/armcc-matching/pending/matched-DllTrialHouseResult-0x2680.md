Module: DllTrialHouseResult
Offset: 0x2680
Size: 0x34
Match: 13/13
One-line: destructor -- frees member +0x14 via masked veneer call if non-null, zeroes it, installs vptr, returns `this` directly (not a tail call, unlike the sibling @0x2648).
Note: retail order is member-load, vtable-literal-load, compare, store -- reproduced by fetching the member into a plain local before the vtable assignment/if (same shape as DllDialogCommon@0xcdc), no need for the "separate named locals for both values" trick that @0x2648's tail-call variant needed.
