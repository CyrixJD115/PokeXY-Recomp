//! PkmnContainer — GameFreak's generic sub-file container, used for
//! Pokémon model/texture bundles (magic is a 2-char tag: `PC`, `PF`, `PT`,
//! `PKi`, `PBJ`, `PKj`, `GR`, `MM`, `AD`, …).
//!
//! Layout (ported from Ohana3DS `Containers/PkmnContainer.cs`, validated
//! against `garc/a/0/0/7/0001.bin` — Bulbasaur — on 2026-10-05):
//!
//! ```text
//! 0x00  2-byte magic tag
//! 0x02  u16 entry count (N)
//! 0x04  N+1 × u32 end offsets (first entry starts at this header's size,
//!       typically 0x80); entry i = data[end[i] .. end[i+1]]
//! ```
//!
//! The first u32 of the table doubles as the header size (observed 0x80),
//! and the last entry ends at the file size.

use xy_core::error::{Error, Result};
use xy_core::util::{u16_le, u32_le};

pub struct PkmnContainer<'a> {
    pub magic: [u8; 2],
    pub entries: Vec<&'a [u8]>,
}

impl<'a> PkmnContainer<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < 4 {
            return Err(Error::truncated(0, 4, "pkmn container"));
        }
        let count = u16_le(data, 2)? as usize;
        if count == 0 || count > 4096 {
            return Err(Error::invalid(
                "pkmn container",
                format!("entry count {count}"),
            ));
        }
        // Entries: start = u32 @ 4 + 4*i, end = u32 @ 8 + 4*i (overlapping
        // pairs — end of entry i is the start of entry i+1).
        if 4 + count * 8 > data.len() {
            return Err(Error::truncated(4, count * 8, "pkmn offset table"));
        }
        let mut entries = Vec::with_capacity(count);
        for i in 0..count {
            let start = u32_le(data, 4 + i * 4)? as usize;
            let end = u32_le(data, 8 + i * 4)? as usize;
            if start > end || end > data.len() {
                return Err(Error::invalid(
                    "pkmn container",
                    format!("entry {i} range [{start:#x}, {end:#x}) out of bounds"),
                ));
            }
            entries.push(&data[start..end]);
        }
        Ok(Self {
            magic: [data[0], data[1]],
            entries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_synthetic_container() {
        // 2 entries: "AB" and "CDE", header 0x10.
        let mut v = vec![b'X', b'Y'];
        v.extend_from_slice(&2u16.to_le_bytes());
        v.extend_from_slice(&0x10u32.to_le_bytes()); // end of entry 0
        v.extend_from_slice(&0x13u32.to_le_bytes()); // end of entry 1
        v.extend_from_slice(&[0u8; 4]); // pad to 0x10? (table overruns header here for simplicity)
                                        // entries content
        v.extend_from_slice(b"AB");
        v.extend_from_slice(b"CDE");
        // fix: entry 0 = [0x10, 0x12) = "AB", entry 1 = [0x12, 0x15) = "CDE"
        v[4..8].copy_from_slice(&0x10u32.to_le_bytes());
        v[8..12].copy_from_slice(&0x12u32.to_le_bytes());
        v[12..16].copy_from_slice(&0x15u32.to_le_bytes());
        let c = PkmnContainer::parse(&v).expect("parse");
        assert_eq!(c.magic, *b"XY");
        assert_eq!(c.entries.len(), 2);
        assert_eq!(c.entries[0], b"AB");
        assert_eq!(c.entries[1], b"CDE");
    }

    #[test]
    fn rejects_out_of_bounds() {
        let mut v = vec![b'P', b'C'];
        v.extend_from_slice(&9u16.to_le_bytes());
        v.extend_from_slice(&[0u8; 32]);
        assert!(PkmnContainer::parse(&v).is_err());
    }
}
