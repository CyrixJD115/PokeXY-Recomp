#!/usr/bin/env python3
"""Deterministic candidate-function finder for armcc-matching agents.

Problem this solves: every executor agent that starts on a module pays the
same cost before it can even pick a function — disassemble the module's
.text, then read through it hunting for the handful of instruction shapes
that reliably masked-match (see the armcc-decomp-executor skill's "What
reliably works" list). If more than one agent ever works the same module
(a big one like DllField has room for several), that scan is fully
duplicated. This script does it once, deterministically, with no LLM
involved, so any agent (or several in parallel) can consume its output
directly instead of re-deriving it.

Rather than re-disassembling from raw bytes, it reuses the Ghidra
decompiled-C corpus already sitting at
.Data/Source_Extracted/decomp/cro_c/<Module>/ (Ghidra already did the hard
work of lifting bytes to readable C — this just pattern-matches that
output). Function starts come from the decompiled-C filenames
(<loadaddr>_FUN_<loadaddr>.c); size is inferred as the gap to the next
function start, so the module's last function is never emitted (no safe
end-of-segment to size it against) and candidates are automatically
excluded if their size doesn't look plausible for their pattern.

Output offsets are in the same module-relative .text file-offset space as
fingerprint_fn.py's own <offset> argument and matched/<Module>/<offset>.c
filenames (see AGENTS.md's "offset gotcha" and collect.py's module
docstring) — no rebase needed when copying a candidate's offset into a
fingerprint_fn.py invocation.

CAVEAT -- this trusts Ghidra's decompiled C as a hint, not ground truth.
Ghidra's decompiler is known to produce misleading or garbled output for
anything beyond the simplest functions (unusual casts, merged variables,
invented constructs that don't reflect the real logic). That risk is low
for the trivial 1-3 statement shapes this script targets, which is why it
only looks for those -- but a candidate appearing here is still only a
place to look first, never proof of anything. Every candidate must still
be independently proven with fingerprint_fn.py before it's trusted; this
script changes where an agent starts looking, not what counts as a match.
If this list is ever extended to flag more complex shapes, that caveat
gets a lot more load-bearing and the confidence tagging below should be
treated with much more suspicion for anything tagged below "high".

Usage:
    python3 find_candidates.py <Module> [--limit N] [--pattern setter,stub,call]

Env:
    XY_SRC   override the default .Data/Source_Extracted root
"""
from __future__ import annotations

import argparse
import glob
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
DEFAULT_SRC = os.path.join(ROOT, ".Data", "Source_Extracted")

FN_FILE_RE = re.compile(r'^([0-9A-Fa-f]{8})_FUN_[0-9A-Fa-f]{8}\.c$')

# Markers that mean "Ghidra struggled here" -- never worth suggesting,
# these are exactly the documented dead-end residue class (jump tables,
# switch dispatchers, raw indirect calls through a vtable/jump slot).
BAD_MARKERS = (
    "WARNING", "jumptable", "switchD", "indirect jump", "CONCAT",
    "stack0x", "halt_baddata", "unaff_", "extraout_",
)


def resolve_ci(directory, name):
    """Case-insensitive dir lookup (matched/entries.c spell some modules
    differently -- see collect.py's resolve_ci, same gotcha applies here)."""
    target = os.path.join(directory, name)
    if os.path.isdir(target):
        return target
    try:
        low = name.lower()
        for entry in os.listdir(directory):
            if entry.lower() == low:
                return os.path.join(directory, entry)
    except OSError:
        pass
    return target


def module_rebase(module):
    """CRO modules load at 0x100000; the main binary (code.bin) doesn't."""
    return 0 if module.lower() in ("main", "code.bin", "code_c") else 0x100000


def load_functions(cro_c_dir, rebase):
    """[(offset, c_file_path), ...] sorted by offset, module-relative."""
    out = []
    for fn in glob.glob(os.path.join(cro_c_dir, "*.c")):
        m = FN_FILE_RE.match(os.path.basename(fn))
        if not m:
            continue
        load_addr = int(m.group(1), 16)
        out.append(((load_addr - rebase) & ~1, fn))
    out.sort()
    return out


def already_matched(matched_dir):
    """{offset} already proven for this module -- skip these outright."""
    out = set()
    if not os.path.isdir(matched_dir):
        return out
    for fn in os.listdir(matched_dir):
        base = os.path.splitext(fn)[0]
        try:
            out.add(int(base, 16) & ~1)
        except ValueError:
            pass
    return out


def already_attempted(attempted_md, module):
    """{offset} already documented as stuck for this module, or the whole
    module flagged dead -- in which case return None as a sentinel."""
    if not os.path.isfile(attempted_md):
        return set()
    text = open(attempted_md).read()
    dead_re = re.compile(
        rf'^##\s+{re.escape(module)}\s+—.*whole module is a dead end',
        re.IGNORECASE | re.MULTILINE)
    if dead_re.search(text):
        return None
    out = set()
    for mod, off in re.findall(r'(\w+)@0x([0-9A-Fa-f]+)', text):
        if mod.lower() == module.lower():
            out.add(int(off, 16) & ~1)
    return out


def strip_body(c_text):
    """Return (signature_line, body_lines) with Ghidra's header comment and
    blank lines dropped; body_lines excludes the enclosing braces."""
    lines = [ln for ln in c_text.splitlines() if ln.strip()]
    lines = [ln for ln in lines if not ln.strip().startswith("//")]
    if not lines:
        return "", []
    sig = lines[0]
    body = []
    in_body = False
    for ln in lines[1:]:
        s = ln.strip()
        if s == "{":
            in_body = True
            continue
        if s == "}":
            break
        if in_body:
            body.append(s)
    return sig, body


ASSIGN_RE = re.compile(
    r'^\*\([A-Za-z_][\w ]*\*\)\(param_\d+ \+ 0x[0-9a-f]+\) = '
    r'(0x[0-9a-f]+|\d+|\*\([A-Za-z_][\w ]*\*\)\(param_\d+( \+ 0x[0-9a-f]+)?\));?$',
    re.IGNORECASE)
RETURN_CONST_RE = re.compile(r'^return(\s+(param_\d+|0x[0-9a-f]+|\d+|true|false))?;?$',
                              re.IGNORECASE)
CALL_RE = re.compile(r'^(FUN_[0-9a-f]+|thunk_FUN_[0-9a-f]+)\(.*\);?$', re.IGNORECASE)


def plausible_size(pattern, n_assigns, n_calls):
    """Reject a candidate whose "gap to next tracked function" size is
    bigger than this pattern could plausibly compile to. Needed because
    Ghidra's own function-boundary detection has real gaps -- confirmed
    during validation: DllField has 4-5 distinct tiny leaf functions
    (each ending its own `bx lr` well before the gap's end) packed into a
    single untracked 0x60-byte span between two files it DID emit, with
    no decompiled-C file of their own. Without this bound, the "next
    file's start" for the one tracked function in that span silently
    swallows its untracked neighbors and reports an inflated size against
    the wrong (first) function's trivial body. This bound is deliberately
    tight -- better to miss a real candidate than hand an agent a
    plausible-looking offset/size pair that isn't actually one function."""
    if pattern == "stub":
        return 16                       # const/void return: 1-4 instructions
    if pattern == "setter":
        return 16 + 20 * n_assigns      # ~1-2 instructions/assign + return
    if pattern == "call+return":
        return 16 + 24 * n_calls        # calls can need arg setup too
    return 16


def classify(sig, body):
    """Return (pattern, confidence) or None if this isn't a recognized easy
    shape. Mirrors the executor skill's "What reliably works" list."""
    joined = "\n".join(body)
    if any(m in joined for m in BAD_MARKERS) or any(m in sig for m in BAD_MARKERS):
        return None
    if not body:
        return None

    assigns = [ln for ln in body if ASSIGN_RE.match(ln)]
    calls = [ln for ln in body if CALL_RE.match(ln)]
    returns = [ln for ln in body if RETURN_CONST_RE.match(ln)]
    other = [ln for ln in body
             if ln not in assigns and ln not in calls and ln not in returns]

    # 1. trivial stub: nothing but a constant/param/void return.
    if not assigns and not calls and not other and len(body) <= 1:
        return ("stub", "high")

    # 2. simple setter(s): only direct field writes, then a plain return.
    if assigns and not calls and not other and len(returns) <= 1:
        return ("setter", "high" if len(assigns) <= 2 else "medium")

    # 3. simple call-and-return helper: a couple of fixed calls, then a
    #    plain constant/void return, nothing else going on.
    if calls and not other and len(returns) <= 1 and len(calls) <= 2:
        return ("call+return", "medium")

    return None


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                  formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("module")
    ap.add_argument("--limit", type=int, default=20)
    ap.add_argument("--pattern", default=None,
                     help="comma-separated subset of stub,setter,call+return")
    args = ap.parse_args()

    src = os.environ.get("XY_SRC", DEFAULT_SRC)
    cro_c_dir = resolve_ci(os.path.join(src, "decomp", "cro_c"), args.module)
    if not os.path.isdir(cro_c_dir):
        print(f"no decompiled-C corpus for {args.module} at {cro_c_dir}",
              file=sys.stderr)
        return 2
    module = os.path.basename(cro_c_dir)  # canonical on-disk spelling

    rebase = module_rebase(module)
    funcs = load_functions(cro_c_dir, rebase)
    if not funcs:
        print(f"no functions found under {cro_c_dir}", file=sys.stderr)
        return 2

    matched = already_matched(os.path.join(
        ROOT, "xy-decomp", "armcc-matching", "matched", module))
    attempted = already_attempted(
        os.path.join(ROOT, "xy-decomp", "armcc-matching", "attempted.md"),
        module)
    if attempted is None:
        print(f"{module}: whole module flagged as a dead end in "
              "attempted.md -- skipping (see that file before retrying)",
              file=sys.stderr)
        return 1

    wanted = set((args.pattern or "stub,setter,call+return").split(","))
    results = []
    for i, (off, path) in enumerate(funcs[:-1]):   # last fn: no safe size
        if (off & ~1) in matched or (off & ~1) in attempted:
            continue
        size = funcs[i + 1][0] - off
        if size <= 0 or size % 4 != 0:
            continue
        sig, body = strip_body(open(path).read())
        cls = classify(sig, body)
        if cls is None or cls[0] not in wanted:
            continue
        n_assigns = sum(1 for ln in body if ASSIGN_RE.match(ln))
        n_calls = sum(1 for ln in body if CALL_RE.match(ln))
        if size > plausible_size(cls[0], n_assigns, n_calls):
            continue   # likely an untracked-neighbor gap, not this function
        snippet = " ".join(body) or sig
        results.append((off, size, cls[0], cls[1], snippet[:100]))

    conf_order = {"high": 0, "medium": 1, "low": 2}
    results.sort(key=lambda r: (conf_order[r[3]], -r[1]))

    print(f"# {module}: {len(results)} candidate(s) "
          f"({len(matched)} already matched, "
          f"{len(attempted)} already attempted, excluded)")
    print("# HINTS ONLY, from Ghidra's decompiled C -- not proof. Every "
          "offset below still needs its own fingerprint_fn.py MASKED MATCH "
          "before it counts as anything.")
    print(f"# offset   size   pattern      conf    snippet")
    for off, size, pat, conf, snippet in results[:args.limit]:
        print(f"0x{off:<8x} 0x{size:<4x} {pat:<12} {conf:<7} {snippet}")
    if not results:
        print("# (nothing matched the known easy shapes -- fall back to "
              "manual disassembly per the executor skill)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
