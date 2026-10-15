#!/usr/bin/env python3
"""Compare a candidate C/C++ function against original CRO bytes, per flag set.

Compiles one candidate source with the retail-matching ARM Compiler 5
(armcc, via Wine), extracts its `.text`, and compares word-by-word against
the original function's bytes read straight out of the plaintext CRO.

Match criterion: every 32-bit word must be identical, except that b/bl
instructions compare only their opcode bits, not the imm24 branch offset.
Branch targets are link-layout facts the compiler cannot know; a candidate
that matches under this mask will fully byte-match once placed at the
original address with the original call targets.

Example (the project's first matched function, ported from the sibling
`pokemon x` repo — see `../matched/DllDialogCommon/README.md`):

  python3 fingerprint_fn.py DllDialogCommon 0x8d0 0x2c \\
      ../matched/DllDialogCommon/0x8d0.c

Exit status: 0 on a full masked match, 1 otherwise.

CAVEAT on --mask-ranges (jump-table data): masking a jump table's data
words only proves the function has the right number of case-body blocks,
in the right order, each byte-correct in isolation. It does NOT prove
which case VALUE (0, 1, 2, ...) drives which block -- the table's real
content (which case selects which address) is exactly what's masked out,
since it's a link-time fact zeroed in the unlinked CRO. Any case-to-block
assignment in a candidate using --mask-ranges is inferred from physical
body order (assuming armcc emits case bodies in ascending case-value
order) and is a hypothesis, not a verified fact. A matched file built
this way must say so in its header comment, not claim "case 0 does X".

Setup (see docs/decomp/armcc-toolchain.md for the full writeup):
  ARMCC_HOME            directory containing bin/armcc.exe
                        (default: ~/Documents/decomp/tools/armcc/ARMCC_Nintendo_5
                        — a shared, proprietary install outside any repo;
                        never commit it)
  ARMLMD_LICENSE_FILE   FlexLM license; if unset, derived from
                        <ARMCC_HOME>/../ARMCC_License/5.x/nintendo-anyhost.lic
  ARMCC_FLAGS           overrides the default flag set below (space-separated)
  XY_ROMFS              directory holding the plaintext .cro modules
                        (default: <repo>/.Data/Raw_Extracted/romfs)
"""

from __future__ import annotations

import argparse
import os
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]

NOP_WORD = 0xE320F000  # ARMv6K+ nop hint, armcc's pad in relaxed call windows

# Fingerprinted 2026-10-13 (in the sibling `pokemon x` repo) against
# DllDialogCommon @0x8d0: ARM11 MPCore, ARM mode, interworking, -O2 -Otime
# reproduces the retail bytes exactly (modulo b/bl immediates). -O3 was
# identical on the specimens so far but is not used by default: it enables
# cross-function inlining that could hurt per-function matching once whole
# modules are compiled. See docs/decomp/armcc-toolchain.md.
ARMCC_COMPILE_FLAGS: list[str] = [
    "-c",
    "--cpu=MPCore",
    "--arm",
    "--apcs=/interwork",
    "-O2",
    "-Otime",
]


def _wine_path(p: Path) -> str:
    """Translate an absolute unix path to Wine's Z:-drive Windows form."""
    return "Z:" + str(p).replace("/", "\\")


def _armcc_env() -> tuple[Path, dict[str, str]]:
    """Locate armcc.exe and build the subprocess environment (license etc.)."""
    home = Path(os.environ.get(
        "ARMCC_HOME",
        Path.home() / "Documents" / "decomp" / "tools" / "armcc" / "ARMCC_Nintendo_5",
    ))
    exe = home / "bin" / "armcc.exe"
    if not exe.is_file():
        raise FileNotFoundError(
            f"armcc.exe not found at {exe}; set ARMCC_HOME to the ARM Compiler 5 "
            "install directory (the one containing bin/armcc.exe)"
        )
    env = dict(os.environ)
    if "ARMLMD_LICENSE_FILE" not in env:
        lic = home.parent / "ARMCC_License" / "5.x" / "nintendo-anyhost.lic"
        if not lic.is_file():
            raise FileNotFoundError(
                f"no license file at {lic}; set ARMLMD_LICENSE_FILE"
            )
        env["ARMLMD_LICENSE_FILE"] = _wine_path(lic)
    elif "/" in env["ARMLMD_LICENSE_FILE"]:
        # A unix path was supplied; armcc runs inside Wine and needs the
        # Windows form.
        env["ARMLMD_LICENSE_FILE"] = _wine_path(Path(env["ARMLMD_LICENSE_FILE"]))
    return exe, env


def _romfs_dir() -> Path:
    return Path(os.environ.get("XY_ROMFS", REPO_ROOT / ".Data" / "Raw_Extracted" / "romfs"))


def masked_words(data: bytes, relax: bool = False) -> list[int]:
    words = []
    for i in range(0, len(data) - len(data) % 4, 4):
        w = struct.unpack_from("<I", data, i)[0]
        # ARM b/bl: bits 27..25 == 0b101 (0xA/0xB in bits 27..24 with L bit).
        if (w >> 25) & 0x7 == 0x5:
            w &= 0xFF000000  # keep cond + opcode + L, mask imm24
        elif relax and (w & 0x0F7F0000) == 0x051F0000:
            # pc-relative ldr/str (literal pool access): eliding relaxation
            # nops shifts every pool offset, so mask the 12-bit immediate.
            w &= 0xFFFFF000
        words.append(w)
    if relax:
        words = [w for w in words if w != NOP_WORD]
    return words


def disasm_word(w: int) -> str:
    """Best-effort one-word disassembly via objdump, for diff readability."""
    try:
        with tempfile.NamedTemporaryFile(suffix=".bin", delete=False) as fh:
            fh.write(struct.pack("<I", w))
            name = fh.name
        out = subprocess.run(
            ["arm-none-eabi-objdump", "-D", "-b", "binary", "-m", "arm", name],
            capture_output=True, text=True,
        ).stdout
        os.unlink(name)
        for line in out.splitlines():
            if line.lstrip().startswith("0:"):
                return line.split("\t", 2)[-1].strip()
    except Exception:
        pass
    return "?"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("module", help="module stem, e.g. DllDialogCommon")
    parser.add_argument("offset", type=lambda s: int(s, 0),
                        help="function's file offset in the CRO")
    parser.add_argument("size", type=lambda s: int(s, 0),
                        help="function size in bytes")
    parser.add_argument("candidate", type=Path, help="candidate .c/.cpp file")
    parser.add_argument("--flags", default=None,
                        help="override armcc flags (space-separated; default: "
                             "ARMCC_COMPILE_FLAGS above)")
    parser.add_argument("--relax-nops", action="store_true",
                        help="elide link-time relaxation nops (e320f000) from "
                             "both sides and mask pc-relative literal offsets, "
                             "which those elisions shift; for originals whose "
                             "call sites carry nop-padded bl windows")
    parser.add_argument("--mask-ranges", default=None,
                        help="comma-separated byte ranges 'offset:length' (hex ok), "
                             "relative to the function's own start, excluded from "
                             "comparison -- for inline jump-table data words (raw "
                             "branch-target addresses, not opcodes) whose values are "
                             "link-time facts an isolated compile can't reproduce; "
                             "word count at each range must still match. CAVEAT: "
                             "this proves the case bodies are byte-correct and in "
                             "the right order, NOT which case value selects which "
                             "body -- that mapping is exactly what's masked out, "
                             "so it's a hypothesis (see module docstring)")
    args = parser.parse_args()

    cro = _romfs_dir() / f"{args.module}.cro"
    if not cro.is_file():
        print(f"original module not found: {cro}", file=sys.stderr)
        return 2
    target = cro.read_bytes()[args.offset:args.offset + args.size]

    exe, env = _armcc_env()
    flags = args.flags.split() if args.flags else [
        f for f in ARMCC_COMPILE_FLAGS if f != "-c"
    ]
    if args.candidate.suffix in (".cpp", ".cc", ".cxx"):
        flags = ["--cpp"] + flags
    obj = Path(tempfile.mkstemp(suffix=".o")[1])
    try:
        cmd = (["wine", str(exe), "-c", *flags,
                "-o", _wine_path(obj), _wine_path(args.candidate.resolve())])
        res = subprocess.run(cmd, capture_output=True, text=True, env=env)
        if res.returncode != 0:
            noise = ("winediag", "systray", "explorer")
            for line in res.stderr.splitlines():
                if not any(n in line for n in noise):
                    print(line, file=sys.stderr)
            return 2
        text = Path(str(obj) + ".text")
        subprocess.run(
            ["arm-none-eabi-objcopy", "-O", "binary",
             "--only-section=.text", str(obj), str(text)],
            check=True, capture_output=True,
        )
        cand = text.read_bytes()
        text.unlink(missing_ok=True)
    finally:
        obj.unlink(missing_ok=True)

    tw = masked_words(target, relax=args.relax_nops)
    cw = masked_words(cand, relax=args.relax_nops)
    nops_elided = ""
    if args.relax_nops:
        n = len(masked_words(target)) - len(tw)
        nops_elided = f", {n} relaxation nops elided, pool offsets masked"

    ranges_note = ""
    if args.mask_ranges:
        word_idxs: set[int] = set()
        for part in args.mask_ranges.split(","):
            off_s, len_s = part.split(":")
            off, length = int(off_s, 0), int(len_s, 0)
            word_idxs.update(range(off // 4, (off + length) // 4))
        for i in word_idxs:
            if i < len(tw):
                tw[i] = 0
            if i < len(cw):
                cw[i] = 0
        ranges_note = f", {len(word_idxs)} jump-table words masked"

    same = sum(1 for a, b in zip(tw, cw) if a == b)
    print(f"target {len(tw)} words, candidate {len(cw)} words, "
          f"{same}/{len(tw)} match (b/bl targets masked{nops_elided}{ranges_note})")
    if args.mask_ranges:
        print("NOTE: --mask-ranges proves the case bodies are byte-correct and "
              "in the right order, NOT which case value selects which body -- "
              "that mapping is masked out along with the table, so any "
              "case-number assignment in the candidate is a hypothesis.")
    status = 0 if (same == len(tw) and len(tw) == len(cw)) else 1
    for i in range(max(len(tw), len(cw))):
        t = tw[i] if i < len(tw) else None
        c = cw[i] if i < len(cw) else None
        mark = "  " if t == c else "!!"
        toff = args.offset + i * 4
        tstr = f"{t:08x} {disasm_word(t):28s}" if t is not None else f"{'—':>8} {'':28s}"
        cstr = f"{c:08x} {disasm_word(c)}" if c is not None else "—"
        print(f"{mark} 0x{toff:05x}  {tstr} | {cstr}")
    if status == 0:
        kind = ("RELAXED MATCH — byte-match additionally pending nop "
                "re-insertion and pool layout" if args.relax_nops
                else "MASKED MATCH")
        print(f"\n*** {kind} *** full byte-match pending link-time branch targets.")
    return status


if __name__ == "__main__":
    raise SystemExit(main())
