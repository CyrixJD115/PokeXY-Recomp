#!/usr/bin/env python3
"""Self-test for fingerprint_fn.py --pool-pad (run: python3 tools/test_pool_pad.py).

Part 1 (pure, no armcc/ROM): unit tests of pool_pad_match on synthetic words.
Part 2 (needs armcc via Wine + the plaintext CROs; skipped cleanly if absent):
  * a known masked match still prints MASKED MATCH (exit 0), with and without --pool-pad;
  * DllBattle@0x71a2c prints POOL-PAD MATCH (exit 3) with --pool-pad, and without the
    flag stays a plain failure (exit 1) -- default behaviour unchanged;
  * the POOL-PAD verdict never contains the string "MASKED MATCH";
  * a wrong pool constant is rejected even with --pool-pad.
"""
import os
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import fingerprint_fn as fp  # noqa: E402

MATCHED = HERE.parent / "matched"
POOLPAD = HERE.parent / "poolpad"
fails = 0


def check(name, cond):
    global fails
    print(("ok   " if cond else "FAIL ") + name)
    fails += 0 if cond else 1


# ---- part 1: synthetic ------------------------------------------------------
PUSH, POP, CMP = 0xE92D4010, 0xE8BD8010, 0xE1530001
cand = [PUSH, 0xE59F1004, CMP, POP, 0x12C6]       # ldr at idx1 -> pool word idx4
tgt = [PUSH, 0xE59F1008, CMP, POP, 0, 0x12C6]     # pad at idx4, ldr disp +4 -> idx5
d, _ = fp.pool_pad_match(tgt, cand)
check("synthetic pad at code/pool boundary accepted", d == [4])
check("different pool constant rejected",
      fp.pool_pad_match(tgt[:5] + [0x12C7], cand)[0] is None)
check("ldr reading the pad word rejected",
      fp.pool_pad_match([PUSH, 0xE59F1004, CMP, POP, 0, 0x12C6], cand)[0] is None)
check("non-zero extra word rejected",
      fp.pool_pad_match([PUSH, 0xE59F1008, CMP, POP, 1, 0x12C6], cand)[0] is None)
check("pad inside code (not at boundary) rejected",
      fp.pool_pad_match([PUSH, 0, 0xE59F1008, CMP, POP, 0x12C6], cand)[0] is None)
check("equal lengths: nothing to pad, rejected", fp.pool_pad_match(cand, cand)[0] is None)

# ---- part 2: real armcc -----------------------------------------------------
romfs = Path(os.environ.get("XY_ROMFS", fp.REPO_ROOT / ".Data" / "Raw_Extracted" / "romfs"))
try:
    fp._armcc_env()
    have = (romfs / "DllBattle.cro").is_file() and (romfs / "DllDialogCommon.cro").is_file()
except FileNotFoundError:
    have = False
if not have:
    print("SKIP part 2: armcc or CRO modules not available (set XY_ROMFS / ARMCC_HOME)")
    sys.exit(1 if fails else 0)


def run(mod, off, size, src, *extra):
    r = subprocess.run([sys.executable, str(HERE / "fingerprint_fn.py"), mod, off, size,
                        str(src), *extra], capture_output=True, text=True,
                       env={**os.environ, "XY_ROMFS": str(romfs)})
    return r.returncode, r.stdout


known = MATCHED / "DllDialogCommon" / "0x8d0.c"
rc, out = run("DllDialogCommon", "0x8d0", "0x2c", known)
check("known masked match: MASKED MATCH, exit 0", rc == 0 and "*** MASKED MATCH ***" in out)
rc, out = run("DllDialogCommon", "0x8d0", "0x2c", known, "--pool-pad")
check("known masked match + --pool-pad: still MASKED MATCH, exit 0",
      rc == 0 and "*** MASKED MATCH ***" in out and "POOL-PAD" not in out)

src = POOLPAD / "DllBattle" / "0x71a2c.c"
rc, out = run("DllBattle", "0x71a2c", "0x2c", src)
check("0x71a2c without flag: unchanged plain failure (exit 1)", rc == 1 and "MATCH ***" not in out)
rc, out = run("DllBattle", "0x71a2c", "0x2c", src, "--pool-pad")
check("0x71a2c --pool-pad: POOL-PAD MATCH, exit 3", rc == 3 and "*** POOL-PAD MATCH ***" in out)
check("POOL-PAD verdict never says MASKED MATCH", "MASKED MATCH" not in out)

with tempfile.TemporaryDirectory() as td:
    wrong = Path(td) / "w.c"
    wrong.write_text(src.read_text().replace("0x12c6;", "0x12c7;"))
    rc, out = run("DllBattle", "0x71a2c", "0x2c", wrong, "--pool-pad")
    check("wrong pool constant rejected under --pool-pad", rc == 1 and "POOL-PAD MATCH" not in out)

print("FAILED" if fails else "all passed")
sys.exit(1 if fails else 0)
