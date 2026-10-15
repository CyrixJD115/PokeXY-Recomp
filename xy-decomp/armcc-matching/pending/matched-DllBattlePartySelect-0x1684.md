Module: DllBattlePartySelect
Offset: 0x1684
Size: 0x10
Match: 4/4
One-line: Prologue-less leaf predicate, `return c == 0` on the third argument (ignores `this` and the second arg).
Note: A 43rd module. Source phrasing matters for instruction order, not just value: `return c == 0;` (direct or ternary) compiles to the right two conditional movs but in the wrong order (moveq before movne); an explicit `if (c != 0) r = 0; else r = 1;` reproduces the retail binary's movne-before-moveq order exactly. Module layout: import-veneer table `0x180`-`0x3cf`, then a 5-case jump-table dispatcher at `0x3d0` (same `--mask-ranges` shape as `DllDialogCommon@0xdd8`, not attempted) and a long constructor at `0x50c` — this candidate is the module's last function, right after a `pop {r4,r5,r6,pc}` epilogue and a one-word zero literal pool at `0x1680`.
