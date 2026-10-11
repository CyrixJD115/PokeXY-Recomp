//! GARC (Game Archive) — `CRAG` containers holding most of Pokémon X/Y's
//! real content under `romfs/a/` (models, scripts, text, encounter data).
//!
//! Ported from pk3DS `pk3DS.Core/CTR/GARC.cs` (`UnpackGARC`), with every
//! offset cross-checked against real XY assets on 2026-10-05:
//! `romfs/a/0/0/1` (3 files, CGFX models) and `romfs/btl_ai.garc` (6 files).
//!
//! Layout (v4 = Gen 6, all little-endian, verified offsets in brackets for
//! `a/0/0/1`):
//!
//! ```text
//! header [0x00, 0x1C):
//!   0x00  "CRAG"  ("GARC" reversed)
//!   0x04  u32 header size (0x1C for v4)
//!   0x08  u16 BOM (0xFFFE)
//!   0x0A  u16 version (0x0400 = v4 Gen 6; 0x0600 = v6, unsupported here)
//!   0x0C  u32 chunk count (4)
//!   0x10  u32 data offset — absolute start of file payloads (FIMB + 0xC)
//!   0x14  u32 file size — the whole container; validated against the image
//!   0x18  u32 largest file (unpadded)          [v4 only field]
//!
//! FATO "OTAF" [0x1C, ...):
//!   +0x00  u32 header size = 0xC + 4 × entry count
//!   +0x04  u16 entry count
//!   +0x06  u16 padding (0xFFFF)
//!   +0x0C  entry count × u32 offsets, relative to (FATB + 0xC)
//!          [a/0/0/1: FATO @0x1C, size 0x18 → FATB @0x34]
//!
//! FATB "BTAF":
//!   +0x00  u32 header size = 0xC + Σ(4 + 12 × subentries per entry)
//!   +0x04  u32 file count (total subentries; validated)
//!   +0x0C  per FATO entry: u32 vector, then for each SET bit i (LSB first):
//!            u32 start, u32 end, u32 len — offsets relative to the FIMB
//!            payload start (= header data offset). `end` includes 0xFF
//!            padding to ContentPadToNearest (4); `len` is the real length.
//!          popcount(vector) > 1 ⇒ pk3DS renders the entry as a folder with
//!          subfiles named by bit index ("00", "01", …).
//!          [a/0/0/1: FATB @0x34, size 0x3C → FIMB @0x70, data @0x7C]
//!
//! FIMB "BMIF":
//!   +0x00  u32 header size (0xC)
//!   +0x04  u32 data size (payloads + padding)
//!   +0x0C  file payloads, each padded with 0xFF to a 4-byte boundary
//! ```
//!
//! Compression: many subentries across `romfs/a/` are LZ-0x11 streams
//! (first byte `0x11`). Those use the **dsdecmp indicator mapping** (pk3DS
//! `LZSS.cs`), which differs from our `xy-core` LZ11 on the extended-token
//! nibbles — see [`lzss`]. Detection is heuristic (first byte `0x11`, then
//! strict decode; on failure the raw bytes are kept), exactly mirroring
//! pk3DS's try/catch — e.g. `btl_ai.garc` file 3 legitimately starts `0x11`
//! while being raw data.

use xy_core::error::{Error, Result};
use xy_core::util::{u16_le, u32_le};

pub const MAGIC: &[u8; 4] = b"CRAG";
pub const VERSION_V4: u16 = 0x0400;

/// One sub-file of an entry: `(bit index, offset, padded end, length)`.
#[derive(Debug, Clone)]
pub struct SubEntry {
    /// Vector bit index (LSB first) — pk3DS uses this as the sub-file name.
    pub id: usize,
    /// Offset relative to the FIMB payload start.
    pub start: u32,
    /// End (start + padded length). `len` is the real length.
    pub end: u32,
    pub len: u32,
}

/// One FATO/FATB entry: a bit vector plus the sub-files it flags.
#[derive(Debug, Clone)]
pub struct Entry {
    /// Bit i (LSB first) set ⇒ sub-file i exists.
    pub vector: u32,
    pub subs: Vec<SubEntry>,
}

impl Entry {
    /// pk3DS naming semantics: >1 sub-file ⇒ render as a folder.
    pub fn is_folder(&self) -> bool {
        self.subs.len() > 1
    }
}

#[derive(Debug)]
pub struct Garc<'a> {
    image: &'a [u8],
    /// Absolute offset of the FIMB payload area (header `data offset`).
    data_offset: usize,
    fatb_entries_off: usize,
    entries: Vec<Entry>,
}

impl<'a> Garc<'a> {
    /// Parse a full GARC container image.
    pub fn parse(image: &'a [u8]) -> Result<Self> {
        if image.len() < 0x1C {
            return Err(Error::truncated(0, 0x1C, "garc header"));
        }
        if &image[0..4] != MAGIC {
            return Err(Error::magic(0, "'CRAG' (GARC)", &image[0..4]));
        }
        let header_size = u32_le(image, 0x04)? as usize;
        let bom = u16_le(image, 0x08)?;
        let version = u16_le(image, 0x0A)?;
        let chunk_count = u32_le(image, 0x0C)?;
        let data_offset = u32_le(image, 0x10)? as usize;
        let file_size = u32_le(image, 0x14)? as usize;

        if bom != 0xFEFF {
            return Err(Error::invalid("garc header", format!("BOM {bom:#06x}")));
        }
        if version != VERSION_V4 {
            return Err(Error::unsupported(
                "garc version",
                format!(
                    "{version:#06x} — only v4 (0x0400, Gen 6) is implemented; \
                     v6 (0x0600) is a later-title layout"
                ),
            ));
        }
        if chunk_count != 4 {
            return Err(Error::invalid(
                "garc header",
                format!("chunk count {chunk_count}"),
            ));
        }
        if header_size != 0x1C {
            return Err(Error::invalid(
                "garc header",
                format!("header size {header_size:#x}, expected 0x1C for v4"),
            ));
        }
        if file_size != image.len() {
            return Err(Error::invalid(
                "garc file size",
                format!("header says {file_size:#x}, image is {:#x}", image.len()),
            ));
        }

        // ---- FATO ----
        let fato_off = 0x1C;
        if image.get(fato_off..fato_off + 4) != Some(b"OTAF") {
            return Err(Error::magic(
                fato_off,
                "'OTAF' (FATO)",
                image.get(fato_off..fato_off + 4).unwrap_or(&[]),
            ));
        }
        let fato_hdr_size = u32_le(image, fato_off + 0x04)? as usize;
        let entry_count = u16_le(image, fato_off + 0x08)? as usize;
        if fato_hdr_size != 0xC + entry_count * 4 {
            return Err(Error::invalid(
                "garc FATO",
                format!("header size {fato_hdr_size:#x} inconsistent with {entry_count} entries"),
            ));
        }
        let fato_end = fato_off + fato_hdr_size;
        if entry_count == 0 || fato_end > image.len() {
            return Err(Error::invalid(
                "garc FATO",
                "no entries or table past image end",
            ));
        }
        let mut fato_entries = Vec::with_capacity(entry_count);
        for i in 0..entry_count {
            fato_entries.push(u32_le(image, fato_off + 0xC + i * 4)? as usize);
        }

        // ---- FATB ----
        let fatb_off = fato_end;
        if image.get(fatb_off..fatb_off + 4) != Some(b"BTAF") {
            return Err(Error::magic(
                fatb_off,
                "'BTAF' (FATB)",
                image.get(fatb_off..fatb_off + 4).unwrap_or(&[]),
            ));
        }
        let fatb_hdr_size = u32_le(image, fatb_off + 0x04)? as usize;
        let file_count = u32_le(image, fatb_off + 0x08)? as usize;
        // Entry data starts right after the FATB header; FATO offsets index it.
        let entries_off = fatb_off + 0xC;
        let fatb_end = fatb_off + fatb_hdr_size;

        let mut entries = Vec::with_capacity(entry_count);
        let mut cursor = 0usize; // running offset from `entries_off`, for validation
        let mut total_subs = 0usize;
        for (i, &fato) in fato_entries.iter().enumerate() {
            if fato != cursor {
                return Err(Error::invalid(
                    "garc FATO/FATB",
                    format!("FATO entry {i} offset {fato:#x} != expected {cursor:#x}"),
                ));
            }
            if entries_off + fato + 4 > fatb_end {
                return Err(Error::truncated(entries_off + fato, 4, "garc FATB vector"));
            }
            let vector = u32_le(image, entries_off + fato)?;
            let mut subs = Vec::new();
            let mut rel = fato + 4;
            for bit in 0..32 {
                if vector & (1 << bit) == 0 {
                    continue;
                }
                if entries_off + rel + 12 > fatb_end {
                    return Err(Error::truncated(
                        entries_off + rel,
                        12,
                        "garc FATB subentry",
                    ));
                }
                let at = entries_off + rel;
                subs.push(SubEntry {
                    id: bit,
                    start: u32_le(image, at)?,
                    end: u32_le(image, at + 4)?,
                    len: u32_le(image, at + 8)?,
                });
                rel += 12;
            }
            cursor = rel;
            total_subs += subs.len();
            entries.push(Entry { vector, subs });
        }
        if cursor + 0xC != fatb_hdr_size {
            return Err(Error::invalid(
                "garc FATB",
                format!(
                    "entries end at {:#x} but header size is {fatb_hdr_size:#x}",
                    cursor + 0xC
                ),
            ));
        }
        if total_subs != file_count {
            return Err(Error::invalid(
                "garc FATB",
                format!("vector popcount sum {total_subs} != file count {file_count}"),
            ));
        }

        // ---- FIMB ----
        let fimb_off = fatb_end;
        if image.get(fimb_off..fimb_off + 4) != Some(b"BMIF") {
            return Err(Error::magic(
                fimb_off,
                "'BMIF' (FIMB)",
                image.get(fimb_off..fimb_off + 4).unwrap_or(&[]),
            ));
        }
        let fimb_hdr_size = u32_le(image, fimb_off + 0x04)? as usize;
        let fimb_data_size = u32_le(image, fimb_off + 0x08)? as usize;
        if fimb_hdr_size != 0xC {
            return Err(Error::invalid(
                "garc FIMB",
                format!("header size {fimb_hdr_size:#x}, expected 0xC"),
            ));
        }
        if fimb_off + 0xC != data_offset {
            return Err(Error::invalid(
                "garc data offset",
                format!(
                    "header says {data_offset:#x}, FIMB implies {:#x}",
                    fimb_off + 0xC
                ),
            ));
        }
        if data_offset + fimb_data_size != file_size {
            return Err(Error::invalid(
                "garc FIMB",
                format!(
                    "data [{data_offset:#x}, {:#x}) does not end at file size {file_size:#x}",
                    data_offset + fimb_data_size
                ),
            ));
        }

        // Per-subentry bounds.
        for e in &entries {
            for s in &e.subs {
                let start = data_offset + s.start as usize;
                let end = data_offset + s.end as usize;
                if s.len > s.end.saturating_sub(s.start) || end > file_size || start > end {
                    return Err(Error::invalid(
                        "garc subentry",
                        format!(
                            "entry [{:#x}, {:#x}) len {} outside payload bounds",
                            s.start, s.end, s.len
                        ),
                    ));
                }
            }
        }

        Ok(Self {
            image,
            data_offset,
            fatb_entries_off: entries_off,
            entries,
        })
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// Total sub-file count (the FATB `file count`).
    pub fn file_count(&self) -> usize {
        self.entries.iter().map(|e| e.subs.len()).sum()
    }

    /// Borrow one sub-file's bytes by vector bit id (`SubEntry::id`).
    pub fn subfile(&self, entry: usize, id: usize) -> Result<&'a [u8]> {
        let e = self
            .entries
            .get(entry)
            .ok_or_else(|| Error::invalid("garc entry", format!("no entry {entry}")))?;
        let s = e.subs.iter().find(|s| s.id == id).ok_or_else(|| {
            Error::invalid(
                "garc subentry",
                format!("entry {entry} has no sub-file with vector bit {id}"),
            )
        })?;
        let start = self.data_offset + s.start as usize;
        let end = start + s.len as usize;
        self.image
            .get(start..end)
            .ok_or_else(|| Error::truncated(start, s.len as usize, "garc subfile data"))
    }

    /// Offset of the FATB entry array (diagnostics/tests).
    pub fn fatb_entries_offset(&self) -> usize {
        self.fatb_entries_off
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal v4 GARC: 2 flat files ("AB", padding) + one 2-sub folder entry.
    fn fixture() -> Vec<u8> {
        // entries: e0 flat {0:"AB"}, e1 folder {0:"C", 1:"DE"}
        let data: Vec<u8> = vec![
            0x41, 0x42, 0xFF, 0xFF, // 0: "AB" + pad
            0x43, 0xFF, 0xFF, 0xFF, // 4: "C" + pad
            0x44, 0x45, 0xFF, 0xFF, // 8: "DE" + pad
        ];
        let entry_count = 2u16;
        let fato_size = 0xC + entry_count as usize * 4;
        // e0: vector 1, 1 sub => 16 bytes; e1: vector 3, 2 subs => 28 bytes
        let fatb_entries = 4 + 12 + 4 + 24;
        let fatb_size = 0xC + fatb_entries;
        let fimb_off = 0x1C + fato_size + fatb_size;
        let data_off = fimb_off + 0xC;
        let file_size = data_off + data.len();

        let mut v = Vec::new();
        v.extend_from_slice(MAGIC);
        v.extend_from_slice(&0x1Cu32.to_le_bytes());
        v.extend_from_slice(&0xFEFFu16.to_le_bytes()); // BOM: bytes FF FE, like real files
        v.extend_from_slice(&VERSION_V4.to_le_bytes());
        v.extend_from_slice(&4u32.to_le_bytes());
        v.extend_from_slice(&(data_off as u32).to_le_bytes());
        v.extend_from_slice(&(file_size as u32).to_le_bytes());
        v.extend_from_slice(&4u32.to_le_bytes()); // largest unpadded
                                                  // FATO
        v.extend_from_slice(b"OTAF");
        v.extend_from_slice(&(fato_size as u32).to_le_bytes());
        v.extend_from_slice(&entry_count.to_le_bytes());
        v.extend_from_slice(&0xFFFFu16.to_le_bytes());
        v.extend_from_slice(&0u32.to_le_bytes()); // e0 @ +0
        v.extend_from_slice(&16u32.to_le_bytes()); // e1 @ +0x10
                                                   // FATB
        v.extend_from_slice(b"BTAF");
        v.extend_from_slice(&(fatb_size as u32).to_le_bytes());
        v.extend_from_slice(&3u32.to_le_bytes()); // file count
        v.extend_from_slice(&1u32.to_le_bytes()); // e0 vector: sub 0
        v.extend_from_slice(&0u32.to_le_bytes()); // start
        v.extend_from_slice(&4u32.to_le_bytes()); // end (padded)
        v.extend_from_slice(&2u32.to_le_bytes()); // len
        v.extend_from_slice(&3u32.to_le_bytes()); // e1 vector: subs 0,1
        v.extend_from_slice(&4u32.to_le_bytes());
        v.extend_from_slice(&8u32.to_le_bytes());
        v.extend_from_slice(&1u32.to_le_bytes());
        v.extend_from_slice(&8u32.to_le_bytes());
        v.extend_from_slice(&12u32.to_le_bytes());
        v.extend_from_slice(&2u32.to_le_bytes());
        // FIMB
        v.extend_from_slice(b"BMIF");
        v.extend_from_slice(&0xCu32.to_le_bytes());
        v.extend_from_slice(&(data.len() as u32).to_le_bytes());
        v.extend_from_slice(&data);
        v
    }

    #[test]
    fn parses_synthetic_v4() {
        let v = fixture();
        let g = Garc::parse(&v).expect("parse");
        assert_eq!(g.entries().len(), 2);
        assert_eq!(g.file_count(), 3);
        assert!(!g.entries()[0].is_folder());
        assert!(g.entries()[1].is_folder());
        assert_eq!(g.subfile(0, 0).unwrap(), b"AB");
        assert_eq!(g.subfile(1, 0).unwrap(), b"C");
        assert_eq!(g.subfile(1, 1).unwrap(), b"DE");
        assert_eq!(g.entries()[1].subs[1].id, 1);
    }

    #[test]
    fn rejects_bad_magic_and_version() {
        let mut v = fixture();
        v[0] = b'X';
        assert!(Garc::parse(&v).is_err());
        let mut v = fixture();
        v[0x0A] = 0x00;
        v[0x0B] = 0x06; // v6
        let err = Garc::parse(&v).unwrap_err().to_string();
        assert!(err.contains("v4"), "{err}");
    }

    #[test]
    fn rejects_size_mismatch_and_truncation() {
        let mut v = fixture();
        v[0x15] += 1; // file size low byte
        assert!(Garc::parse(&v).is_err());
        assert!(Garc::parse(&fixture()[..0x30]).is_err());
    }
}
