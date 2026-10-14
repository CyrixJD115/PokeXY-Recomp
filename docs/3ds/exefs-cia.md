# ExeFS + CIA update-title containers

## 2026-10-14 — ExeFS parser landed; CIA still hand-parsed; `.code` turns out compressed

**Goal:** get `code.bin` out of the v1.5 update partitions
(`.Data/Rom/X/Upd/update_x.cia`, `.Data/Rom/Y/Upd/update_y.cia`, title
`0004000E…`) using the toolkit, per the `.Data/Rom/X/Upd/` landmark in
AGENTS.md.

### ExeFS parser (`xy-archive/src/exefs.rs`, `xy archive exefs`)

Added the ExeFS container parser — full offset table is in the module
docs; summary: fixed 0x200-byte header, 10 × 16-byte entries
`{name[8], offset u32 (relative to end of header), size u32}`, remaining
0x140 bytes are unused SHA-256 hashes (not verified — this toolkit only
ever sees decrypted content, same stance as `romfs.rs`). `Exefs::open()`
accepts a CCI, a standalone CXI (signature + magic at +0x100, same as a
cart NCCH partition), or a headerless NCCH; `Exefs::parse()` takes a bare
ExeFS region directly. Unit-tested (round-trip, truncation, empty slots);
no golden test yet — see below for why.

Validated against the real update CXIs (see below): both extract cleanly
to 2 entries, `.code` and `icon`, matching the known ExeFS section-name
convention.

### CIA container — no parser yet, offsets hand-derived

`xy` has **no CIA parser**. CIA (`.cia`) is the installable-title container
(cert chain + ticket + TMD + content + meta), distinct from CCI (the
cartridge image, NCSD + partitions) — converting one to the other isn't
meaningful; a CIA's content is already a single NCCH/CXI, you just need to
slice it out.

CIA header (offset 0): `u32 hdr_size, u16 type, u16 version, u32 cert_sz,
u32 tik_sz, u32 tmd_sz, u32 meta_sz, u64 content_sz`, then each section
(cert chain, ticket, TMD, content, meta) back-to-back, each **64-byte
aligned**. So: `content_off = align64(hdr_size) + align64(cert_sz) +
align64(tik_sz) + align64(tmd_sz)`.

For both `update_x.cia` and `update_y.cia`: `hdr_size=0x2020, cert_sz=2560,
tik_sz=848, tmd_sz=2868, meta_sz=15040, content_sz=31715328`, giving
`content_off=0x3900`. The NCCH magic sits at `content_off + 0x100 =
0x3a00` — i.e. the content *is* a standalone CXI, signature and all, same
layout `Exefs::open()`/`Romfs::open()` already expect for a bare NCCH.

**Crypto check:** NCCH header flags byte (`content_off + 0x18F`) = `0x04`
for both — `NoCrypto` bit set, same as the base ROM dumps. Confirms the
update partitions are plaintext, no AES-CTR needed.

No CIA-slicing command exists yet in `xy`; the content was sliced by hand
(Python, offsets above) into `update_x.cxi` / `update_y.cxi` next to the
source `.cia`. Worth a proper `xy archive cia` subcommand later if the
update track needs this more than once — small, same shape as the GARC
family (header parse → section table → slice).

### 2026-10-14 — resolved: `blz.rs` decodes `.code`, matches the recorded size exactly

Added `xy-archive/src/blz.rs` (`xy archive blz`), a faithful port of
**LumaTeam/Luma3DS**'s `lzss_decompress`
(`sysmodules/loader/source/loader.c`) — the routine that actually
decompresses `.code` on real hardware at boot, found via `gh search code
LZSS_Decompress` after yuzu/Citra mirrors turned out to be only partially
fetchable. Footer layout: `u32` at `len-8` packs `header_size` (top byte)
and `stop_offset` (low 24 bits); `u32` at `len-4` is `additional_size`,
giving `decompressed_size = len + additional_size`. Full algorithm
(control-byte bit meaning, the 12-bit-displacement/length-nibble pair
encoding, the backward read/write cursor convention) is in the module
doc comment.

Validated against the real update partitions before trusting it:
- Footer math alone predicts `decompressed_size = 2,864,776 +
  2,304,376 = 5,169,152` — the exact size already on record (see
  `docs/general/findings.md`), on the very first attempt.
- Ran it for real: `xy archive blz .Data/Rom/X/Upd/exefs/.code -o
  code.bin` → 5,169,152 bytes, same for Y.
- The scheme's leading uncompressed region (72 bytes) is byte-identical
  between compressed input and decompressed output on both X and Y —
  confirms the read/write cursor boundary math is exactly right, not
  just the total size.
- Decompressed bytes are recognizable Thumb/ARM code (e.g. `7047` = `BX
  LR` function epilogues throughout, not noise).
- X vs Y v1.5 `code.bin` differ in only 28 of 5,169,152 bytes — exactly
  what two SKUs sharing one engine should look like.
- Cross-checked the *compressed* `.code` independently: the user pulled
  `.code` off a modded 3DS via GodMode9 and it came out byte-identical
  (SHA-256) to `xy archive exefs`'s own extraction — confirms `exefs.rs`
  too, from a second, independent source.

Golden test: `xy-archive/tests/blz_golden.rs` (skips cleanly if the
`.code` assets aren't present).

**Dead end avoided (documented in the earlier entry below, now resolved):
`xy-core::lz10`'s LZ11 branch and `xy-archive::lzss`'s GARC LZ-0x11 are
both the wrong algorithm for this** — BLZ reads backward from the file's
own end and uses a different control-byte/back-reference encoding
entirely. Do not reach for either on `.code`.

### Finding (resolved above): the `.code` entry is BLZ-compressed, not plain `code.bin`

`xy archive exefs update_x.cxi out/` extracts `.code` (2,864,776 bytes)
and `icon` (14,016 bytes) — structurally valid, but `.code`'s size does
**not** match the `5,169,152`-byte decrypted `code.bin` already on record
for the v1.5 update in AGENTS.md's landmark table. The NCCH ExHeader has a
per-title "compress ExeFS code" flag (3dbrew `SCI` flags, bit 0); this
update evidently has it set.

3DS `.code` compression is the Nintendo "BLZ" (backward LZ) scheme used
across GBA/NDS/3DS homebrew tooling (CUE's `blz` tool) — **not** the
`LZ10`/`LZ11` in `xy-core::lz10`, and **not** the GARC-payload `LZ-0x11` in
`xy-archive::lzss`. A 12-byte footer at the end of the compressed buffer
(`increase_size`, `compressed_size`, `header_size` fields, read backward)
drives decompression; not yet decoded or verified against this file —
marked open, not implemented. Until a BLZ decoder lands, `.code` as
extracted here is the **compressed** blob, not `code.bin` itself.

**Dead end avoided:** do not reach for `xy-core::lz10`'s LZ11 branch for
this — that branch is already flagged unvalidated for GARC payloads
(see `docs/formats/`) and is a different algorithm from BLZ regardless.

