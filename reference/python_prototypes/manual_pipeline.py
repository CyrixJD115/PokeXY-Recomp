#!/usr/bin/env python3
"""Full e-manual pipeline: BCMA-DARC -> LZ10 -> inner DARC -> assets.

Format notes (reverse-engineered):
  Outer container: DARC variant with 12-byte nodes [fl|nameid, a, b];
    dir flag = bit 24; file = [nameid, abs_offset, size]; dir = [nameid, ?, total_nodes];
    nameid = BYTE offset into UTF-16LE string table that starts right after
    the node area (node count from root dir's b field).
  Page .arc payloads: Nintendo LZ10 (0x10 magic) wrapping an inner DARC.
"""
import struct, os
from collections import Counter

SRC = "extracted/part1_romfs/Manual.bcma"
WORK = "extracted_assets/manual"
os.makedirs(WORK, exist_ok=True)

data = open(SRC, "rb").read()
tableoff = struct.unpack_from("<I", data, 0x10)[0]
data_base = struct.unpack_from("<I", data, 0x18)[0]
table = data[tableoff:data_base]

def parse_darc12(table, data):
    """Parse the 12-byte-node DARC variant. Returns node list."""
    root_fl, _, total_nodes = struct.unpack_from("<III", table, 0)
    nnodes = total_nodes
    strbase = nnodes * 12
    if strbase > len(table):  # insane header — fall back to all-node table
        strbase, nnodes = (len(table) // 12) * 12, len(table) // 12
    def name(nid):
        p = strbase + nid
        if p >= len(table) - 1:
            return ""
        end = p
        while end + 1 < len(table) and table[end:end+2] != b"\x00\x00":
            end += 2                      # char-aligned scan: never split a pair
        return table[p:end].decode("utf-16-le", "replace")
    nodes = []
    for i in range(nnodes):
        fl, a, b = struct.unpack_from("<III", table, i * 12)
        nodes.append(dict(isdir=bool(fl & 0x01000000), name=name(fl & 0xFFFFFF),
                          a=a, b=b))
    return nodes

# ---------------- outer container ----------------
outer = parse_darc12(table, data)
print(f"outer: {len(outer)} nodes")
files = []
seen = set()
for i, n in enumerate(outer):
    if n["isdir"]:
        continue
    nm = n["name"] or f"file_{i}"
    if nm in seen:
        nm = f"{i:03d}_{nm}"
    seen.add(nm)
    files.append((nm, data[n["a"]:n["a"] + n["b"]]))
print(f"  {len(files)} page archives, e.g. {files[0][0]}, {files[1][0]}, {files[2][0]}")

# ---------------- LZ10 + inner DARC ----------------
def lz10(buf):
    size = buf[1] | buf[2] << 8 | buf[3] << 16
    out, i = bytearray(), 4
    while len(out) < size and i < len(buf):
        ctrl = buf[i]; i += 1
        for bit in range(8):
            if len(out) >= size or i >= len(buf): break
            if not (ctrl >> (7 - bit)) & 1:
                out.append(buf[i]); i += 1
            else:
                b0, b1 = buf[i], buf[i+1]; i += 2
                ln, disp = (b0 >> 4) + 3, ((b0 & 0xF) << 8 | b1) + 1
                pos = len(out) - disp
                for _ in range(ln):
                    out.append(out[pos]); pos += 1
    return bytes(out)

def extract_inner(darc, prefix):
    dsize = struct.unpack_from("<I", darc, 0x18)[0]
    t = darc[0x1C:dsize]          # node/string table WITHOUT the 0x1C header
    d = darc
    nodes = parse_darc12(t, d)
    for i, n in enumerate(nodes):
        if n["isdir"] or not n["name"]:
            continue
        blob = d[n["a"]:n["a"] + n["b"]]
        outp = os.path.join(WORK, prefix, n["name"])
        os.makedirs(os.path.dirname(outp), exist_ok=True)
        open(outp, "wb").write(blob)
    return [n["name"] for n in nodes if not n["isdir"]]

exts = Counter()
total = 0
for nm, blob in files:
    if blob[:1] not in (b"\x10", b"\x11"):
        print(f"  not LZ: {nm}"); continue
    inner = lz10(blob)
    assert inner[:4] == b"darc", nm
    got = extract_inner(inner, os.path.splitext(nm)[0])
    for g in got:
        exts[os.path.splitext(g)[1].lower() or "(none)"] += 1
    total += len(got)
print(f"inner extraction: {total} assets from {len(files)} pages")
for e, c in exts.most_common():
    print(f"  {e:<8} x{c}")
