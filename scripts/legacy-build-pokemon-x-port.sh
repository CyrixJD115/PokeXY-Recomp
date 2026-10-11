#!/usr/bin/env bash
# Superseded: the builder moved to xy-recomp/build.sh (repo-relative paths,
# reuses .thirdparty/ checkouts, writes ports to .Data/ports/). This shim
# keeps the old entry point working.
exec "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")/../xy-recomp/build.sh" "$@"
