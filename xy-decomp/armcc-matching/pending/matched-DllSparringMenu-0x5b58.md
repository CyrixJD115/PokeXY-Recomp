Module: DllSparringMenu
Offset: 0x5b58
Size: 0x34
Match: 13/13
One-line: This-adjusted conditional field setter: v==1/16/17 clamp to 2/3/4, stored at (this-0x84)+0x8c.
Note: Sibling at 0x5b1c (same cmp/moveq/beq cascade, plus an extra unconditional
zero-store to a second field) got stuck at 9/15 — not re-attempted, flagged for a
future agent. 0x5b58 is the simpler variant (single field, no extra store) and
matched with the direct C translation on the first try.
