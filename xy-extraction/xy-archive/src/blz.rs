//! `.code` compression used by 3DS ExeFS code sections — Nintendo's
//! backward-LZSS variant (community tools call it "BLZ"). Distinct from
//! both `xy-core::lz10` (LZ10/LZ11) and `xy-archive::lzss` (the GARC
//! LZ-0x11 payload codec) — do not reuse either for this.
//!
//! Ported from LumaTeam/Luma3DS's `lzss_decompress`
//! (`sysmodules/loader/source/loader.c`), the routine that actually
//! decompresses `.code` on real hardware at boot — about as authoritative
//! a reference as exists for this format.
//!
//! Format: the compressed buffer, decoded backward from its own end, plus
//! an 8-byte footer at the very end:
//! * `u32` at `len-8`: top byte = `header_size` (bytes before `len` where
//!   the backward control-byte stream starts), low 24 bits = `stop_offset`
//!   (bytes before `len` where it stops).
//! * `u32` at `len-4`: `additional_size`, so `decompressed_size = len +
//!   additional_size`.
//!
//! Decompression runs in a single buffer sized to `decompressed_size`,
//! with the compressed bytes copied into its head; a read cursor walks
//! backward from `len - header_size` down to `len - stop_offset`, a write
//! cursor walks backward from `decompressed_size`. Each control byte's
//! bits (MSB first) select, per bit: `0` = copy one literal byte from the
//! read cursor; `1` = a back-reference — two bytes (read backward) pack a
//! 12-bit displacement (`((near & 0xF) << 8) | far, + 2`) and a length
//! nibble (`(near >> 4) + 3`, so 3..18), copied byte-by-byte from
//! `write_cursor + displacement` (self-overlapping, like any LZSS window
//! copy — each byte is fetched fresh at write time).
//!
//! Validated against the real Pokémon X/Y v1.5 update `.code`
//! (`.Data/Rom/{X,Y}/Upd/exefs/.code`, 2,864,776 bytes, `additional_size =
//! 2,304,376`): decompresses to exactly the `5,169,152`-byte `code.bin`
//! already on record (`docs/3ds/exefs-cia.md`).

use xy_core::error::{Error, Result};
use xy_core::util::u32_le;

/// Decompress a 3DS `.code`-style backward-LZSS buffer.
pub fn decompress(compressed: &[u8]) -> Result<Vec<u8>> {
    let len = compressed.len();
    if len < 8 {
        return Err(Error::truncated(0, 8, "blz footer"));
    }

    let footer = u32_le(compressed, len - 8)?;
    let additional_size = u32_le(compressed, len - 4)? as usize;
    let header_size = (footer >> 24) as usize;
    let stop_offset = (footer & 0x00FF_FFFF) as usize;

    if header_size > len || stop_offset > len {
        return Err(Error::invalid(
            "blz",
            format!(
                "footer offsets exceed buffer (header_size={header_size}, \
                 stop_offset={stop_offset}, len={len})"
            ),
        ));
    }

    let decompressed_size = len
        .checked_add(additional_size)
        .ok_or_else(|| Error::invalid("blz", "decompressed size overflow"))?;

    let mut out = vec![0u8; decompressed_size];
    out[..len].copy_from_slice(compressed);

    // `read_idx`/`write_idx` mirror the original's `v3`/`v2`: both are
    // decremented *before* every access (the C reads/writes as
    // `*(ptr-- - 1)`), except the back-reference lookup, which indexes
    // the *current* `write_idx` before it moves (`v2[v15]`).
    let mut read_idx = len - header_size;
    let stop_idx = len - stop_offset;
    let mut write_idx = decompressed_size;

    while read_idx > stop_idx {
        read_idx -= 1;
        let mut control = out[read_idx];
        for _ in 0..8 {
            if control & 0x80 != 0 {
                if read_idx < 2 {
                    return Err(Error::invalid("blz", "back-reference pair underruns input"));
                }
                let near = out[read_idx - 1] as usize;
                let far = out[read_idx - 2] as usize;
                read_idx -= 2;

                let displacement = (((near & 0xF) << 8) | far) + 2;
                let count = (near >> 4) + 3;
                for _ in 0..count {
                    // Read uses the *current* write_idx (unmoved), matching
                    // the original's `v2[v15]` before the pointer decrements.
                    let src = write_idx
                        .checked_add(displacement)
                        .ok_or_else(|| Error::invalid("blz", "back-reference overflow"))?;
                    if src >= decompressed_size {
                        return Err(Error::invalid("blz", "back-reference out of range"));
                    }
                    let byte = out[src];
                    if write_idx == 0 {
                        return Err(Error::invalid("blz", "back-reference underruns output"));
                    }
                    write_idx -= 1;
                    out[write_idx] = byte;
                }
            } else {
                if read_idx == 0 || write_idx == 0 {
                    return Err(Error::invalid("blz", "literal copy underruns buffer"));
                }
                read_idx -= 1;
                write_idx -= 1;
                out[write_idx] = out[read_idx];
            }
            control <<= 1;
            if read_idx <= stop_idx {
                break;
            }
        }
    }

    Ok(out)
}
