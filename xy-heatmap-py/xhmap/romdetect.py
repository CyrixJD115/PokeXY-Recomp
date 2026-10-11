"""Identify 3DS ROM images from their title ID.

Parses only headers, mirroring the offsets xy-archive's romfs.rs uses
(NCSD magic at 0x100 in real dumps / 0x0 in headerless tooling dumps;
partition-0 descriptor at NCSD+0x20, sector-count × 512 B; title ID at
NCCH+0x118). Accepts CCI card images (.3ds/.cci) and standalone update
CXIs (.cxi/.app).
"""
import glob
import os
import struct

# title id -> (game, kind); 0004000E… = update partitions (v1.5 here)
KNOWN_TITLES = {
    "0004000000055D00": ("Pokémon X", "base"),
    "0004000000055E00": ("Pokémon Y", "base"),
    "0004000E00055D00": ("Pokémon X", "update (v1.5)"),
    "0004000E00055E00": ("Pokémon Y", "update (v1.5)"),
}


def identify(path):
    """Return {path, title_id, game, kind, recognized} or raise ValueError."""
    with open(path, "rb") as f:
        head = f.read(0x400)
    if len(head) < 0x200:
        raise ValueError(f"{path}: too small to be a ROM image")

    ncch = None
    if head[0x100:0x104] == b"NCSD":          # real .3ds/.cci dumps
        ncch = struct.unpack_from("<I", head, 0x120)[0] * 512
    elif head[0:4] == b"NCSD":                # headerless tooling dump
        ncch = struct.unpack_from("<I", head, 0x20)[0] * 512
    elif head[0x100:0x104] == b"NCCH":        # signed CXI (sig then magic)
        ncch = 0
    elif head[0:4] == b"NCCH":                # signature-stripped CXI
        ncch = -0x100                         # fields shift down by the sig
    else:
        raise ValueError(
            f"{path}: not a CCI (.3ds) or CXI (.cxi) image "
            f"(no NCSD/NCCH magic)")

    # Title ID normally sits at NCCH-structure+0x118 (+0x108 mirror in some
    # dumps). Validate against the 0004000x title-category prefix so
    # shifted layouts fall through to the next candidate.
    with open(path, "rb") as f:
        cands = [ncch + 0x118, ncch + 0x108]
        if ncch != 0:                     # stripped CXI may keep field spacing
            cands += [0x118, 0x108]
        for off in cands:
            f.seek(max(off, 0))
            tid = f.read(8)[::-1].hex().upper()
            if tid.startswith("0004000"):
                break
        else:
            raise ValueError(f"{path}: no 0004000x title ID found")
    game, kind = KNOWN_TITLES.get(tid, (None, None))
    return {
        "path": os.path.abspath(path),
        "title_id": tid,
        "game": game,
        "kind": kind,
        "recognized": game is not None,
    }


def scan_default_roms(repo_root):
    """List identifiable ROMs under .Data/Rom (X/Y × Base/Upd)."""
    roms = []
    pattern = os.path.join(repo_root, ".Data", "Rom", "*", "*", "*")
    for p in sorted(glob.glob(pattern)):
        if not os.path.isfile(p) or p.endswith((".bin", ".mem", ".keys")):
            continue
        try:
            roms.append(identify(p))
        except (ValueError, OSError):
            continue
    return roms


def default_game_rom(repo_root, prefer_game="Pokémon X"):
    """Pick the labeling ROM: --rom absent -> prefer the X base dump."""
    for r in scan_default_roms(repo_root):
        if r["game"] == prefer_game and r["kind"] == "base":
            return r
    for r in scan_default_roms(repo_root):    # any recognized base dump
        if r["recognized"] and r["kind"] == "base":
            return r
    return None
