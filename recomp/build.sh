#!/usr/bin/env bash
# PokeXYZ-RDComp recomp builder: produce a standalone native binary for
# Pokémon X or Y from YOUR decrypted dump, reusing this repo's thirdparty
# clones (3dsrecomp + Zakuro) so repeat builds never re-clone.
#
#   ./recomp/build.sh [--rom /path/to/PokemonX.3ds] [--run]
#
# Nothing game-derived leaves Data/: the generated port projects land in
# Data/ports/<name> and the binaries are gitignored. Compilation of the
# port is intentionally manual (first link pulls in hundreds of MiB).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
THIRD="$ROOT/thirdparty"
PORTS="$ROOT/Data/ports"
RUN=0
ROM=""

step() { printf '\n\033[1;36m==>\033[0m %s\n' "$*"; }

while [ $# -gt 0 ]; do
  case "$1" in
    --rom) ROM="${2:-}"; shift 2 ;;
    --run) RUN=1; shift ;;
    *) echo "usage: $0 [--rom <decrypted.3ds>] [--run]" >&2; exit 1 ;;
  esac
done

step "checking tools"
command -v cargo >/dev/null || { echo "Rust is required: https://rustup.rs" >&2; exit 1; }
command -v gcc >/dev/null || command -v clang >/dev/null || { echo "a C compiler (gcc/clang) is required" >&2; exit 1; }
command -v ar >/dev/null || { echo "binutils (ar) is required" >&2; exit 1; }
if [ -x /usr/lib/ccache/gcc ]; then
  CC_BIN="/usr/lib/ccache/gcc"
  step "ccache shim found — repeat builds will be fast"
elif command -v ccache >/dev/null; then
  CC_BIN="$(command -v ccache) gcc"
  step "ccache found (wrapper mode)"
else
  CC_BIN="gcc"
  step "ccache not installed — expect a long C compile (pacman -S ccache / apt install ccache)"
fi

step "checking thirdparty checkouts (see thirdparty/fetch.sh)"
[ -d "$THIRD/3dsrecomp" ] || { echo "missing $THIRD/3dsrecomp — run thirdparty/fetch.sh" >&2; exit 1; }
[ -d "$THIRD/zakuro" ] || { echo "missing $THIRD/zakuro — run thirdparty/fetch.sh" >&2; exit 1; }
if [ ! -x "$THIRD/3dsrecomp/target/release/3dsrecomp" ]; then
  step "building 3dsrecomp (first time only)"
  (cd "$THIRD/3dsrecomp" && cargo build --release)
fi
RECOMP="$THIRD/3dsrecomp/target/release/3dsrecomp"

if [ -z "$ROM" ]; then
  for cand in "$ROOT/Data/0004000000055D00_v00.3ds" "$ROOT/Data/0004000000055E00_v00.3ds"; do
    [ -f "$cand" ] && ROM="$cand" && break
  done
fi
[ -n "$ROM" ] || { echo "ERROR: pass your decrypted dump: $0 --rom /path/to/PokemonX.3ds [--run]" >&2; exit 1; }
[ -f "$ROM" ] || { echo "ERROR: ROM not found: $ROM" >&2; exit 1; }
ROM="$(readlink -f "$ROM")"

case "$(basename "$ROM")" in
  *0055D00*|*[Xx]*) NAME="pokemon-x" ;;
  *0055E00*|*[Yy]*) NAME="pokemon-y" ;;
  *) NAME="recomp-game" ;;
esac
PORT="$PORTS/$NAME"

step "generating recompiled project for $NAME (writes + compiles ~679 MiB of C)"
mkdir -p "$PORTS"
rm -rf "$PORT"
CC="$CC_BIN" "$RECOMP" port "$ROM" "$PORT" --name "$NAME" --zakuro "$THIRD/zakuro"

step "linking standalone binary"
(cd "$PORT" && cargo build --release)

BIN="$PORT/target/release/$NAME"
step "done: $BIN"
ls -lh "$BIN"

if [ "$RUN" = 1 ]; then
  step "launching $NAME"
  exec "$BIN" "$ROM"
fi
echo
echo "run it with:"
echo "  \"$BIN\" \"$ROM\""
