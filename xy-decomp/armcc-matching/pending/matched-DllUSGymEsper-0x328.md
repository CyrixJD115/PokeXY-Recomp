Module: DllUSGymEsper
Offset: 0x328
Size: 0x14
Match: 5/5
One-line: Boolean getter through one pointer indirection -- `return this->p20->b == 1;`.
Note: Same idiom as DllStrInput@0x6674, but comparing against literal 1 (not non-zero): armcc reuses the already-loaded byte as the true-case result and only conditionally clears on the false path, so the compiled form has a lone `movne` with no `moveq`.
