#!/usr/bin/env python3
"""Print the real jump-table targets of a function from the CRO's internal relocations.

The jump-table data words are zero in the unlinked CRO, but each is covered by an
internal relocation (CRO header 0x128, 12-byte entries {u32 patched seg-tag, u8 type,
u8 last, u8 src-seg, u8 pad, u32 addend}); target = segment[src].offset + addend.
This recovers which slot points at which body -- i.e. the case-value-to-body mapping that
`fingerprint_fn.py --mask-ranges` cannot prove.

  python3 jumptable_relocs.py DllBattle 0x60e14 0x60e2c    # file-offset range of the table
"""
import struct, sys, os
from pathlib import Path

mod, lo, hi = sys.argv[1], int(sys.argv[2], 16), int(sys.argv[3], 16)
root = Path(os.environ.get("XY_ROMFS", Path(__file__).resolve().parents[3] / ".Data/Raw_Extracted/romfs"))
d = (root / f"{mod}.cro").read_bytes()
u = lambda o: struct.unpack_from("<I", d, o)[0]
so, sn = u(0xC8), u(0xCC)
segs = [u(so + 12 * i) for i in range(sn)]
res = lambda tag: segs[tag & 15] + (tag >> 4)
ro, rn = u(0x128), u(0x12C)
slot = 0
for i in range(rn):
    t, ty, last, seg, pad, add = struct.unpack_from("<IBBBBI", d, ro + 12 * i)
    fo = res(t)
    if lo <= fo < hi:
        print(f"slot {(fo - lo) // 4}  @{fo:#x} -> {segs[seg] + add:#x}")
