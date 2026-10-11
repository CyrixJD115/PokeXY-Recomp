//! Small shared helpers: bounds-checked integer reads and UTF-16 string
//! decoding. (The RIFF/WAV writer now lives in `xy-audio`.)

use crate::error::{Error, Result};

pub fn u16_le(b: &[u8], off: usize) -> Result<u16> {
    let s = b
        .get(off..off.saturating_add(2))
        .ok_or_else(|| Error::truncated(off, 2, "u16 read"))?;
    Ok(u16::from_le_bytes([s[0], s[1]]))
}

pub fn u32_le(b: &[u8], off: usize) -> Result<u32> {
    let s = b
        .get(off..off.saturating_add(4))
        .ok_or_else(|| Error::truncated(off, 4, "u32 read"))?;
    Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

pub fn u64_le(b: &[u8], off: usize) -> Result<u64> {
    let s = b
        .get(off..off.saturating_add(8))
        .ok_or_else(|| Error::truncated(off, 8, "u64 read"))?;
    Ok(u64::from_le_bytes(s.try_into().unwrap()))
}

pub fn u8_at(b: &[u8], off: usize) -> Result<u8> {
    b.get(off)
        .copied()
        .ok_or_else(|| Error::truncated(off, 1, "u8 read"))
}

pub fn f32_le(b: &[u8], off: usize) -> Result<f32> {
    let s = b
        .get(off..off.saturating_add(4))
        .ok_or_else(|| Error::truncated(off, 4, "f32 read"))?;
    Ok(f32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

/// Decode a NUL-terminated UTF-16LE string from raw bytes.
/// Trailing padding after the first NUL unit is ignored.
pub fn utf16le_string(bytes: &[u8]) -> String {
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    let end = units.iter().position(|&u| u == 0).unwrap_or(units.len());
    String::from_utf16_lossy(&units[..end])
}

/// FNV-1a 64-bit hash — used by golden tests for content manifests.
pub fn fnv1a64(data: &[u8]) -> u64 {
    data.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| {
        (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3)
    })
}

pub fn i32_le(b: &[u8], off: usize) -> Result<i32> {
    Ok(u32_le(b, off)? as i32)
}

/// Minimal JSON string escaping (no serde dependency in the core crates).
pub fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_little_endian() {
        let b = [0x78, 0x56, 0x34, 0x12, 0xFF, 0x00];
        assert_eq!(u16_le(&b, 0).unwrap(), 0x5678);
        assert_eq!(u32_le(&b, 0).unwrap(), 0x12345678);
        assert!(u32_le(&b, 3).is_err());
    }

    #[test]
    fn stops_utf16_at_nul() {
        let b = [0x61, 0x00, 0x62, 0x00, 0x00, 0x00, 0xFF, 0xFF];
        assert_eq!(utf16le_string(&b), "ab");
    }

    #[test]
    fn fnv_matches_reference_vectors() {
        // Standard FNV-1a 64 test vectors.
        assert_eq!(fnv1a64(b""), 0xcbf29ce484222325);
        assert_eq!(fnv1a64(b"a"), 0xaf63dc4c8601ec8c);
        assert_eq!(fnv1a64(b"foobar"), 0x85944171f73967e8);
    }
}
