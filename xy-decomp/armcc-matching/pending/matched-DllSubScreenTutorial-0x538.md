Module: DllSubScreenTutorial
Offset: 0x538
Size: 0x10
Match: 4/4
One-line: Tail virtual call through a direct member pointer (o = this->p38; return o->vtbl[9](o)), prologue-less leaf.
Note: First match in this module (72nd module overall, per the running README count). Sits right between two dispatcher-shaped functions (0x358-0x4c3 jump-table dispatcher, 0x4cc-0x538 three-case switch) and a large push{r4-sl,lr} dispatcher at 0x548 -- same shape as DllDialogCommon@0x6e8 but no null check and a direct (not nested) member offset.
