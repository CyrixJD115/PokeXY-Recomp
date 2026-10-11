# .Data/ — game-derived material (machine-local, never share)

Everything in here derives from your own legally-dumped copies of Pokémon
X/Y and from processing them. **Nothing under .Data/ is ever committed or
distributed** — game dumps, extracted assets, generated C, symbol maps,
port trees and binaries all carry Nintendo's IP and/or console-unique
keys. Per repo convention, dot-prefixed top-level dirs are machine-local
"agentic reference" material — only this README and `update/README.md`
are tracked.

| Path | Contents |
|---|---|
| `0004000000055D00_v00.3ds` | Pokémon X base ROM v1.0 (kujira-1) — primary target |
| `0004000000055E00_v00.3ds` | Pokémon Y base ROM v1.0 (kujira-2) |
| `*.dec.code`, `boot9.bin`, `otp.mem`, `firm0.bin` | decrypted code + console material — highest sensitivity |
| `3ds_extracted/` | raw decrypted RomFS/ExeFS partitions (treat as read-only sources) |
| `src_extracted/` | all derived output — index in `src_extracted/MAP.md` |
| `src_extracted/garc/` | 272 GARCs unpacked (44,519 files) + `manifest.tsv` |
| `src_extracted/decomp/` | Ghidra project + 38,385 decompiled C functions |
| `src_extracted/cro/` | 133 plaintext CRO modules + `static.crs` |
| `src_extracted/pipeline/` | matching-decomp workspace: `orig/`, `symbols/` CSVs, generated `split/` (72,742 .o), `out/` (134/134 byte-identical recreates), `objdiff.json` |
| `src_extracted/recomp/` | 3dsrecomp analyze/build output: `.so`, codeNNN.c/.o, verify reports |
| `src_extracted/recomp/port/` | the standalone port project (user's current build) |
| `ports/` | future port trees from `xy-recomp/build.sh` |
| `update/` | **v1.5 update dump slot — see `update/README.md`** |

Console key material at this level (`boot9.bin`, `otp.mem`, `firm0.bin`,
any `*.dec.code`) is listed explicitly in the repo `.gitignore` as
belt-and-braces on top of the whole-tree ignore.
