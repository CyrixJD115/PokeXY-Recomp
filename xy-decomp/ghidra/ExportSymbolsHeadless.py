# Export per-binary symbol CSVs for 3DS-Decomp-Pipeline (headless).
# @category 3ds
#
# Python port of AlgebraManiacABC's ExportSymbols.java (dialog-free, takes
# script args). Exports every program in the project root to
# <out_dir>/<orig-binary>.csv with columns:
#   Location,Name,Namespace,Mode,Size,Segment
# Programs named <Mod>.bin (Ghidra CRO imports) are written as <Mod>.cro.csv;
# code.bin keeps its name. Addresses are the program's own (0x100000-based).
#
# Usage (script args): <out_dir> [unique]

import csv

from ghidra.program.model.listing import Instruction, Data

args = getScriptArgs()
if not args:
    raise Exception('usage: ExportSymbolsHeadless.py <out_dir> [unique]')
out_dir = args[0]
unique = len(args) > 1 and args[1].lower() == 'true'

listing = currentProgram.getListing()
fm = currentProgram.getFunctionManager()
rm = currentProgram.getReferenceManager()
tmode = currentProgram.getProgramContext().getRegister('TMode')
memory = currentProgram.getMemory()

rows = []  # (addr_offset, name, namespace, mode, size, segment)
counts = {}

for symbol in currentProgram.getSymbolTable().getAllSymbols(False):
    if not symbol.isPrimary() or symbol.isExternal():
        continue
    addr = symbol.getAddress()
    if any(r.isExternalReference() for r in rm.getReferencesFrom(addr)):
        continue
    name = symbol.getName(False)
    namespace = symbol.getParentNamespace().getName(True)
    cu = listing.getCodeUnitAt(addr)
    func = fm.getFunctionAt(addr)
    size = 0
    mode = None
    if isinstance(cu, Instruction):
        if func is None:
            continue
        if unique and func.isThunk() and addr.toString() not in func.getName():
            name += '_' + addr.toString()
        rv = currentProgram.getProgramContext().getRegisterValue(tmode, addr)
        if rv is not None:
            mode = '$t' if rv.getUnsignedValue().toString() == '1' else '$a'
        body = func.getBody()
        if body.getNumAddressRanges() == 1:
            size = body.getNumAddresses()
        else:
            for rng in sorted(body.getAddressRanges(), key=lambda r: r.getMinAddress().getOffset()):
                r_addr = rng.getMinAddress()
                r_name = name if r_addr.equals(addr) else name + '_' + r_addr.toString()
                seg = memory.getBlock(r_addr)
                seg_name = seg.getName() if seg is not None else '.text'
                if seg is None:
                    print('No block at {} - symbol {} set to .text by default!'.format(addr, symbol.getName()))
                counts.setdefault(r_name, []).append((r_addr, r_name, namespace, mode, rng.getLength(), seg_name))
            continue
    elif isinstance(cu, Data):
        d = listing.getDataAt(addr)
        if d is None:
            continue
        size = d.getLength()
        mode = '$d'
    if mode is None or size <= 0:
        continue
    seg = memory.getBlock(addr)
    seg_name = seg.getName() if seg is not None else '.text'
    if seg is None:
        print('No block at {} - symbol {} set to .text by default!'.format(addr, symbol.getName()))
    counts.setdefault(name, []).append((addr, name, namespace, mode, size, seg_name))

if unique:
    for lst in counts.values():
        if len(lst) > 1:
            for (a, n, ns, m, s, seg) in lst:
                rows.append((a, n + '_' + a.toString(), ns, m, s, seg))
        else:
            rows.extend(lst)
else:
    for lst in counts.values():
        rows.extend(lst)

rows.sort(key=lambda r: r[0].getOffset())

prog_name = currentProgram.getName()
csv_name = 'code.bin.csv' if prog_name == 'code.bin' else prog_name.replace('.bin', '.cro.csv')
out_path = out_dir.rstrip('/') + '/' + csv_name
with open(out_path, 'w', newline='') as fh:
    w = csv.writer(fh, quoting=csv.QUOTE_ALL, lineterminator='\n')
    w.writerow(['Location', 'Name', 'Namespace', 'Mode', 'Size', 'Segment'])
    for (a, n, ns, m, s, seg) in rows:
        size_hex = format(int(s), '08x')
        # Location quoted like the Java exporter; Size stays bare hex
        fh.write('"%s","%s","%s",%s,%s,"%s"\n' % (a.toString(), n, ns, m, size_hex, seg))
print('{}: {} symbols'.format(csv_name, len(rows)))
