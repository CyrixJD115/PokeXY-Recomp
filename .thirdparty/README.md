# .thirdparty/ — upstream tool checkouts (machine-local)

The four external tools the daily recomp/decomp loops call into. All are
upstream projects with their own licenses and histories — never commit
them; `./fetch.sh` recreates any missing checkout. Dot-prefixed top-level
dirs are machine-local "agentic reference" material by repo convention —
only this README and `fetch.sh` are tracked.

| Tool | Upstream | License | Role |
|---|---|---|---|
| `3dsrecomp` | fearkov/3dsrecomp | MIT | ARM11 → C static recompiler (`analyze`/`build`/`verify`/`port`) |
| `zakuro` | fearkov/zakuro | MIT | 3DS OS/GPU/audio HLE runtime the standalone port links against |
| `3DS-Decomp-Pipeline` | AlgebraManiacABC/3DS-Decomp-Pipeline | see repo | split → compile → byte-compare → relink matching-decomp loop |
| `objdiff` | encounter/objdiff | MIT | per-unit diff UI driven by this repo's `objdiff.json` |

The rest of the ecosystem (emulator source, format references, C# tools,
Ghidra script suites) lives in `reference/` — also gitignored, with its own
README index.
