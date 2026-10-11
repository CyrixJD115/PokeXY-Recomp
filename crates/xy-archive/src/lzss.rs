//! LZ-0x11 decompression for GARC sub-file payloads — **dsdecmp flavor**
//! (pk3DS `pk3DS.Core/CTR/LZSS.cs`, "heavily taken from dsdecmp").
//!
//! Stream layout:
//!
//! ```text
//! 0x00  u8  type byte (0x11)
//! 0x01  u24 decompressed size; if 0, a u32 size follows instead
//! then: flag bytes (MSB first), each governing 8 blocks:
//!   bit clear — literal byte
//!   bit set   — back-reference token, first nibble S = b0 >> 4 selects:
//!     S >= 2 : 2 bytes, LEN = S + 1,                  DISP = 12-bit + 1
//!     S == 0 : 3 bytes, LEN = ((b0&0xF)<<4|b1>>4) + 0x11,   DISP = 12-bit + 1
//!     S == 1 : 4 bytes, LEN = ((b0&0xF)<<12|b1<<4|b2>>4) + 0x111, DISP = 12-bit + 1
//! ```
//!
//! **Flavor note:** the *indicator mapping* above differs from textbook
//! Nintendo LZ11 (where S==1 selects the 3-byte token and S==0 the 4-byte
//! one). Game Freak's GARC payloads follow the dsdecmp mapping used by
//! pk3DS — validated against real `romfs/a/` sub-files (see
//! `tests/garc_golden.rs`). `xy-core::lz10` keeps the Nintendo flavor for
//! LZ10 (manual pages); its LZ11 branch has not been validated against real
//! data and should not be reused here.
//!
//! Detection is deliberately heuristic: a first byte of `0x11` is *not*
//! proof of compression (e.g. `btl_ai.garc` file 3 starts `0x11` while
//! being raw data). [`try_decompress`] mirrors pk3DS's try/catch: decode
//! strictly, keep raw bytes on any failure or implausible slack.

use xy_core::error::{Error, Result};

/// Decompress a complete LZ-0x11 stream (dsdecmp flavor).
pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    let byte = |i: usize| -> Result<u8> {
        input
            .get(i)
            .copied()
            .ok_or_else(|| Error::truncated(i, 1, "lz-0x11 stream"))
    };

    let ty = byte(0)?;
    if ty != 0x11 {
        return Err(Error::invalid(
            "lz-0x11 header",
            format!("type byte {ty:#04x} is not 0x11"),
        ));
    }
    let mut size = byte(1)? as usize | (byte(2)? as usize) << 8 | (byte(3)? as usize) << 16;
    let mut i = 4usize;
    if size == 0 {
        size = byte(4)? as usize
            | (byte(5)? as usize) << 8
            | (byte(6)? as usize) << 16
            | (byte(7)? as usize) << 24;
        i = 8;
    }

    let mut out = Vec::with_capacity(size.min(1 << 26));
    let mut flags = 0u8;
    let mut mask = 0u8; // 0 => fetch a new flag byte
    while out.len() < size {
        if mask == 0 {
            flags = byte(i)?;
            i += 1;
            mask = 0x80;
        }
        let bit_set = flags & mask != 0;
        mask >>= 1;

        if !bit_set {
            out.push(byte(i)?);
            i += 1;
            continue;
        }

        let b0 = byte(i)?;
        let (len, disp, advance) = match b0 >> 4 {
            0 => {
                let b1 = byte(i + 1)?;
                let b2 = byte(i + 2)?;
                (
                    (((b0 & 0xF) as usize) << 4 | (b1 >> 4) as usize) + 0x11,
                    (((b1 & 0xF) as usize) << 8 | b2 as usize) + 1,
                    3,
                )
            }
            1 => {
                let b1 = byte(i + 1)?;
                let b2 = byte(i + 2)?;
                let b3 = byte(i + 3)?;
                (
                    (((b0 & 0xF) as usize) << 12 | (b1 as usize) << 4 | (b2 >> 4) as usize) + 0x111,
                    (((b2 & 0xF) as usize) << 8 | b3 as usize) + 1,
                    4,
                )
            }
            s => (
                s as usize + 1,
                (((b0 & 0xF) as usize) << 8 | byte(i + 1)? as usize) + 1,
                2,
            ),
        };
        i += advance;
        if disp > out.len() {
            return Err(Error::invalid(
                "lz-0x11 stream",
                format!(
                    "back-reference disp {disp:#x} exceeds {:#x} decoded bytes",
                    out.len()
                ),
            ));
        }
        if len == 0 || out.len() + len > size + 0x1000 {
            // Wildly overrunning the declared size means this is not a valid
            // stream (or our token read is off); clamp hard rather than
            // allocate unbounded memory.
            return Err(Error::invalid(
                "lz-0x11 stream",
                format!("token length {len:#x} overruns declared size {size:#x}"),
            ));
        }
        let pos = out.len() - disp;
        for k in 0..len {
            let b = out[pos + k];
            out.push(b);
        }
    }

    // dsdecmp's TooMuchInput check: more than ~4 bytes of 0xFF alignment
    // slack left over means this was probably not a compressed stream.
    let consumed_slack = input.len().saturating_sub(i);
    if consumed_slack > 7 {
        return Err(Error::invalid(
            "lz-0x11 stream",
            format!("{consumed_slack:#x} unread trailing byte(s) — not a valid stream"),
        ));
    }
    Ok(out)
}

/// pk3DS-style heuristic: `Some(decoded)` if `data` looks like and decodes
/// as an LZ-0x11 stream, `None` to keep the raw bytes.
pub fn try_decompress(data: &[u8]) -> Option<Vec<u8>> {
    if data.first() != Some(&0x11) || data.len() < 4 {
        return None;
    }
    match decompress(data) {
        Ok(out) if !out.is_empty() => Some(out),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals_only() {
        // size 3, flag byte 0x00, then "abc"
        let s = [0x11, 0x03, 0x00, 0x00, 0x00, b'a', b'b', b'c'];
        assert_eq!(decompress(&s).unwrap(), b"abc");
    }

    #[test]
    fn two_byte_token() {
        // 'a' literal, then back-ref len 6 disp 1 => 7 bytes total.
        // flag 0x40: block0 literal, block1 compressed.
        // token: S = len-1 = 5 => b0 = 0x50, disp-1 = 0 => b1 = 0x00
        let s = [0x11, 0x07, 0x00, 0x00, 0x40, b'a', 0x50, 0x00];
        assert_eq!(decompress(&s).unwrap(), b"aaaaaaa");
    }

    #[test]
    fn three_byte_token_s0() {
        // 'x' literal, then S=0 3-byte token: LEN = 0+0x11 = 17, DISP = 1.
        // flag 0x40: block0 literal, block1 compressed. Total = 18 = 0x12.
        let s = [0x11, 0x12, 0x00, 0x00, 0x40, b'x', 0x00, 0x00, 0x00];
        let out = decompress(&s).unwrap();
        assert_eq!(out.len(), 0x12);
        assert!(out.iter().all(|&b| b == b'x'));
    }

    #[test]
    fn rejects_bad_type_and_disp() {
        assert!(decompress(&[0x10, 0x01, 0x00, 0x00]).is_err());
        // back-reference with nothing decoded yet: flag 0x80, token disp 1
        let s = [0x11, 0x04, 0x00, 0x00, 0x80, 0x30, 0x00];
        assert!(decompress(&s).is_err());
    }

    #[test]
    fn try_decompress_heuristic() {
        // Not 0x11-prefixed.
        assert!(try_decompress(b"raw").is_none());
        // btl_ai-style false positive: starts 0x11 but decodes nothing sane.
        let fp = [0x11, 0x01, 0x00, 0x00, 0xE0, 0xF1, 0x0A, 0x0A];
        assert!(try_decompress(&fp).is_none());
        // Real compressed stream decodes.
        let s = [0x11, 0x03, 0x00, 0x00, 0x00, b'a', b'b', b'c'];
        assert_eq!(try_decompress(&s).unwrap(), b"abc");
    }
}
