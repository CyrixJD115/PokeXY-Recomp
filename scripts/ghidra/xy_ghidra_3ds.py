#!/usr/bin/env python3
"""Run the zaksabeast 3ds-Ghidra-Scripts suite over every program in the
PokemonX Ghidra project, via PyGhidra's ghidra_script() runner.

(Ghidra 12 removed Jython, so the stock analyzeHeadless CLI cannot run .py
scripts; pyghidra.ghidra_script() runs any GhidraScript with stdout capture.)

Usage:
    python3 xy_ghidra_3ds.py <program-name>...    # e.g. DllBag.bin code.bin
    python3 xy_ghidra_3ds.py --all                # every *.bin program

Order per README: Label3dsSVCs -> Label3dsHandles -> Label3dsCTRFns ->
Set3dsTlsType. Each program is saved once after the full chain.
"""
import os
import sys
import time
from pathlib import Path

import __main__ as _main

import pyghidra

RD_ROOT = Path(__file__).resolve().parents[2]
GHIDRA_DIR = os.environ.get("GHIDRA_INSTALL_DIR", "/opt/ghidra")
PROJECT_LOC = os.environ.get(
    "XY_PROJECT_LOC",
    str(RD_ROOT / "Data/src_extracted/decomp/ghidra_project_3ds"),
)
PROJECT = os.environ.get("XY_PROJECT", "PokemonX")
SCRIPT_DIR = os.environ.get("XY_SCRIPT_DIR", str(RD_ROOT / "scripts/ghidra"))
SCRIPTS = ["Label3dsSVCs.py", "Label3dsHandles.py", "Label3dsCTRFns.py", "Set3dsTlsType.py"]

pyghidra.start(install_dir=GHIDRA_DIR)
from pyghidra import ghidra_script, open_project, walk_programs  # noqa: E402
from ghidra.util.task import ConsoleTaskMonitor  # noqa: E402

monitor = ConsoleTaskMonitor()


def process(file, program):
    name = file.getName()
    if not name.endswith(".bin"):
        return
    # The upstream scripts (and their ghidra_utils/ctr_services helpers) pull
    # currentProgram/monitor via `from __main__ import ...` — a Jython-era
    # idiom. Under pyghidra, __main__ is this driver, so publish the names
    # here; the imports then resolve for both the script and its imports.
    _main.currentProgram = program
    _main.monitor = monitor
    t0 = time.time()
    print(f"[{time.strftime('%H:%M:%S')}] === {name} ===", flush=True)
    for s in SCRIPTS:
        st = time.time()
        try:
            out, err = ghidra_script(f"{SCRIPT_DIR}/{s}", project, program)
        except Exception as e:
            print(f"    {s}: EXCEPTION {e}", flush=True)
            continue
        tail = " | ".join(out.strip().splitlines()[-2:]) if out.strip() else "(no output)"
        status = f"{s}: {time.time()-st:.1f}s :: {tail[:220]}"
        if err.strip():
            status += f"  [stderr: {err.strip().splitlines()[-1][:200]}]"
        print("    " + status, flush=True)
    program.save(f"3ds-Ghidra-Scripts suite", monitor)
    print(f"[{time.strftime('%H:%M:%S')}] --- {name} saved ({time.time()-t0:.1f}s total)", flush=True)


project = open_project(PROJECT_LOC, PROJECT)

targets = sys.argv[1:]
if not targets:
    sys.exit("usage: xy_ghidra_3ds.py <program>... | --all")

if targets == ["--all"]:
    walk_programs(project, process)
else:
    wanted = set(targets)
    seen = []
    def cb(file, program):
        if file.getName() in wanted:
            seen.append(file.getName())
            process(file, program)
    walk_programs(project, cb)
    missing = wanted - set(seen)
    if missing:
        print(f"NOT FOUND: {sorted(missing)}", flush=True)

project.close()
print("=== driver done ===", flush=True)
