//! Nintendo LZ10 / LZ11 decompression (stream type bytes `0x10` / `0x11`).
//!
//! Layout: 4-byte header (`type`, u24 LE uncompressed size), then a bitstream
//! of control bytes; each set bit introduces a back-reference token, cleared
//! bits mean a literal byte.

use crate::error::{Error, Result};

pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    let hdr = input
        .get(..4)
        .ok_or_else(|| Error::truncated(0, 4, "LZ header"))?;
    let ty = hdr[0];
    if ty != 0x10 && ty != 0x11 {
        return Err(Error::invalid(
            "LZ header",
            format!("type byte {ty:#04x} is neither 0x10 (LZ10) nor 0x11 (LZ11)"),
        ));
    }
    let size = hdr[1] as usize | (hdr[2] as usize) << 8 | (hdr[3] as usize) << 16;

    let byte = |i: usize| -> Result<u8> {
        input
            .get(i)
            .copied()
            .ok_or_else(|| Error::truncated(i, 1, "LZ stream"))
    };

    let mut out = Vec::with_capacity(size);
    let mut i = 4;
    while out.len() < size {
        let ctrl = byte(i)?;
        i += 1;
        for bit in (0..8).rev() {
            if out.len() >= size {
                break;
            }
            if ctrl & (1 << bit) == 0 {
                out.push(byte(i)?);
                i += 1;
                continue;
            }

            let b0 = byte(i)?;
            let (len, disp, advance) = if ty == 0x10 {
                // LZ10: fixed 2-byte token, length 3..=18
                (
                    (b0 >> 4) as usize + 3,
                    (((b0 & 0xF) as usize) << 8 | byte(i + 1)? as usize) + 1,
                    2,
                )
            } else {
                match b0 >> 4 {
                    // LZ11 short: 2-byte token, length 1..=16
                    n @ 1..=15 => (
                        n as usize + 1,
                        (((b0 & 0xF) as usize) << 8 | byte(i + 1)? as usize) + 1,
                        2,
                    ),
                    // LZ11 medium: 3-byte token, length 17..=272
                    0 => {
                        let b1 = byte(i + 1)?;
                        let b2 = byte(i + 2)?;
                        if b1 >> 4 != 0xF {
                            (
                                (((b0 & 0xF) as usize) << 4 | (b1 >> 4) as usize) + 0x11,
                                (((b1 & 0xF) as usize) << 8 | b2 as usize) + 1,
                                3,
                            )
                        } else {
                            // LZ11 long: 4-byte token (b1 low nibble = flag), length 273+
                            let b3 = byte(i + 3)?;
                            let b4 = byte(i + 4)?;
                            (
                                (((b0 & 0xF) as usize) << 12
                                    | ((b1 & 0xF) as usize) << 8
                                    | (b2 as usize) << 4
                                    | (b3 >> 4) as usize)
                                    + 0x111,
                                (((b3 & 0xF) as usize) << 8 | b4 as usize) + 1,
                                5,
                            )
                        }
                    }
                    _ => unreachable!("nibble match"),
                }
            };
            i += advance;
            back_copy(&mut out, disp, len)?;
        }
    }
    Ok(out)
}

/// Copy `len` bytes from `disp` back; overlapping (run-length) copies allowed.
fn back_copy(out: &mut Vec<u8>, disp: usize, len: usize) -> Result<()> {
    if disp == 0 || disp > out.len() {
        return Err(Error::invalid(
            "LZ stream",
            format!(
                "bad back-reference (disp={disp}, len={len}) with only {} bytes decoded",
                out.len()
            ),
        ));
    }
    let pos = out.len() - disp;
    for k in 0..len {
        let b = out[pos + k];
        out.push(b);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals_only() {
        // LZ10, 3 bytes, one control byte with all bits clear, 3 literals.
        let stream = [0x10, 0x03, 0x00, 0x00, 0x00, 0x61, 0x62, 0x63];
        assert_eq!(decompress(&stream).unwrap(), b"abc");
    }

    #[test]
    fn run_length_backreference() {
        // LZ10, output "aaaaa": control 0b01000000 = literal then back-reference.
        // token b0 = (len-3)<<4 | hi(disp-1) = (4-3)<<4 | 0 = 0x10, b1 = 0.
        let stream = [0x10, 0x05, 0x00, 0x00, 0x40, b'a', 0x10, 0x00];
        assert_eq!(decompress(&stream).unwrap(), b"aaaaa");
    }

    #[test]
    fn rejects_bad_type() {
        assert!(decompress(&[0x00, 0x00, 0x00, 0x00]).is_err());
        assert!(decompress(&[0x10, 0x01, 0x00, 0x00]).is_err()); // truncated stream
    }
}
