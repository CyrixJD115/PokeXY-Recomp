#!/usr/bin/env bash
# Clone the third-party tools this repo's recomp + decomp loops consume.
# Clones are gitignored (they are upstream code with their own histories);
# check them out again any time with this script.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
THIRD="$ROOT/thirdparty"

clone() { # <url> <dir>
  if [ -d "$THIRD/$2" ]; then
    echo "have $2"
  else
    git clone --depth 1 "$1" "$THIRD/$2"
  fi
}

clone https://github.com/fearkov/3dsrecomp                 3dsrecomp
clone https://github.com/fearkov/zakuro                    zakuro
clone https://github.com/AlgebraManiacABC/3DS-Decomp-Pipeline 3DS-Decomp-Pipeline
clone https://github.com/encounter/objdiff                 objdiff

echo "done — tool sources live under $THIRD (gitignored)"
