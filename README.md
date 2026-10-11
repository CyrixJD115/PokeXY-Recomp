# PokeXYZ-RDComp

One project space for the Pokémon X/Y **R**ecomp + **D**ecomp effort — a
Rust-first toolkit where [3dsrecomp](https://github.com/fearkov/3dsrecomp)
turns a decrypted dump into a native Linux binary, the `xy` CLI extracts and
converts every game format, and a matching-decompilation loop
(3DS-Decomp-Pipeline + objdiff) rebuilds the original code byte-for-byte.
Python shows up only as helper scripts under `scripts/` (Ghidra automation,
heatmap, analysis glue).

## Layout

| Path | What it is |
|---|---|
| `crates/` + `src/` | the `xy` toolkit — 8 domain crates (`xy-core/archive/audio/gfx/script/battle/curate/convert`) behind one unified CLI |
| `recomp/` | static-recomp track: `build.sh` port builder, C override shims |
| `thirdparty/` | upstream checkouts (3dsrecomp, zakuro, 3DS-Decomp-Pipeline, objdiff) + `reference/` clones — gitignored; `fetch.sh` recreates the four |
| `scripts/` | python/shell helpers: patched 3DS Ghidra suite + PyGhidra driver, RDComp heatmap, rebuild glue |
| `docs/` | `findings.md` (recomp metrics, gotchas, round-trip results), `ghidra-scripts.md` |
| `Data/` | **your** game-derived material — dumps, extracts, generated code, ports. Gitignored by construction; never commit or share |

## Quick start

```bash
# toolkit
cargo build --release
./target/release/xy gfx icon Data/3ds_extracted/exefs/icon.bin -o icon.png

# native standalone port (needs Rust, gcc/clang, ar; your own decrypted dump)
./thirdparty/fetch.sh          # once — clones 3dsrecomp + zakuro
./recomp/build.sh --run        # picks up Data/0004000000055D00_v00.3ds by default
```

## Status

- **X v1.0 recomp**: 71,163/74,340 functions reached (95.7%), 0 verify
  mismatches; standalone binary runs natively on Linux (X and Y).
- **Matching decomp**: 134/134 modules recreated **byte-identical** from
  generated splits (parity gate for future hand-written C).
- **Toolkit**: 65/65 golden tests byte/pixel-identical against upstream
  references; clippy `-D warnings` clean.
- **v1.5 update**: slot prepared at `Data/update/` — see its README before
  landing the update dumps.

Details and the full roadmap: [`AGENTS.md`](AGENTS.md),
[`PLAN.md`](PLAN.md), [`docs/findings.md`](docs/findings.md).

## Legal

Requires your own legally-obtained, **decrypted** game dumps. Everything
game-derived (dumps, extracted assets, generated C, symbol maps, port
projects, binaries) and all console key material is excluded from version
control by `.gitignore` and must never be shared or published. Upstream
tools are vendored unpackaged via `thirdparty/fetch.sh` and keep their own
licenses (MIT for 3dsrecomp/zakuro/objdiff).
