# xy-recomp/ — static recompilation track

Everything here drives [3dsrecomp](https://github.com/fearkov/3dsrecomp) and
[Zakuro](https://github.com/fearkov/zakuro) (both MIT, vendored under
`.thirdparty/`) to turn a decrypted Pokémon X/Y dump into a native Linux
binary.

```
./xy-recomp/build.sh --rom .Data/Rom/X/Base/0004000000055D00_v00.3ds --run
```

- Generated port projects land in `.Data/Curated/ports/<name>/` (game-derived — never
  commit or share). The C translation is ~679 MiB; the first link takes a
  while even with ccache.
- `overrides/` holds our C override shims — hand-written replacements that
  3dsrecomp links in place of recompiled functions (via an `--overrides`
  file). Start from upstream `docs/overrides.md` in the 3dsrecomp checkout.

## Validated state (X v1.0, 2026-10-10)

- `3dsrecomp analyze/build/verify`: 71,163 / 74,340 functions reached
  (95.7%), **0 verify mismatches**; the remaining ~4% run through Zakuro's
  interpreter fallback.
- The resulting binary runs the game natively on Linux (user-confirmed:
  smooth, near-zero lag). Y works the same way from upstream (`99.8%`
  reached, 2,886 verify mismatches upstream — not yet re-verified here).
- Pins: **v1.0 code.bin = 5,156,864 bytes**. Update v1.5 code.bin is
  5,169,152 bytes — updates landed at `.Data/Rom/{X,Y}/Upd/` before pointing any of this
  at an update dump.

See `docs/general/findings.md` for the full metrics, gotchas (CRO address
shift, thunk dedup) and the matching-decomp round-trip results.
