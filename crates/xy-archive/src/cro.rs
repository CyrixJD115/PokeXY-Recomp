//! CRO0 — 3DS relocatable code modules ("DLLs"): the `Dll*.cro` files at the
//! RomFS root and `static.crs` (the static module's symbol info).
//!
//! **Plaintext** — a correction to earlier notes: these files were never
//! encrypted; the 0x00–0x7F region is four SHA-256 hashes (random-looking),
//! which is why cursory first-byte checks misidentified them. Magic `CRO0`
//! sits at **0x80**, and files end with `0xCC` padding to 0x1000. Verified
//! across all 134 files on 2026-10-05.
//!
//! Spec from 3dbrew "CRO0" (fetched 2026-10-05); offsets are file offsets:
//!
//! ```text
//! 0x00  4 × SHA-256 (over code/data/name regions; RO verifies vs CRR)
//! 0x80  "CRO0" | 0x84 name offset | 0x88/0x8C RO runtime links (0)
//! 0x90  file size | 0x94 .bss size | 0x98 fixed size | 0x9C unknown
//! 0xA0  nnroControlObject seg-off (0xFFFFFFFF in CRS) | 0xA4 OnLoad
//! 0xA8  OnExit | 0xAC OnUnresolved (seg-offs; 0xFFFFFFFF = absent)
//! 0xB0  code offset | 0xB4 code size | 0xB8 data offset | 0xBC data size
//! 0xC0  module-name offset | 0xC4 size (NUL-terminated string)
//! 0xC8  segment table offset | 0xCC num (12-byte entries)
//! 0xD0  named exports off | 0xD4 num (8B) | 0xD8 indexed exports off
//! 0xDC  num (4B) | 0xE0 export strings off | 0xE4 size
//! 0xE8  export trie off | 0xEC num (8B)
//! 0xF0  import modules off | 0xF4 num (20B entries)
//! 0xF8  import relocations off | 0xFC num (12B)
//! 0x100 named imports off | 0x104 num (8B) | 0x108 indexed imports off
//! 0x10C num (8B) | 0x110 anonymous imports off | 0x114 num (8B)
//! 0x118 import strings off | 0x11C size
//! 0x120 unknown-reloc base off | 0x124 num (8B)
//! 0x128 internal relocations off | 0x12C num (12B)
//! 0x130 unknown relocations off | 0x134 num (12B)
//! ```
//!
//! Segment-offset encoding: bits 0-3 = segment index, bits 4-31 = offset
//! into that segment. Segment table entry: `{ u32 offset, u32 size,
//! u32 type }` with type 0 = .text, 1 = .rodata, 2 = .data, 3 = .bss.
//! Named exports/imports reference NUL-terminated strings in the string
//! tables — these are the module's real symbol names.

use xy_core::error::{Error, Result};
use xy_core::util::u32_le;

pub const MAGIC_OFFSET: usize = 0x80;
pub const MAGIC: &[u8; 4] = b"CRO0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentType {
    Text,
    Rodata,
    Data,
    Bss,
    Unknown(u32),
}

impl SegmentType {
    fn from_id(id: u32) -> Self {
        match id {
            0 => SegmentType::Text,
            1 => SegmentType::Rodata,
            2 => SegmentType::Data,
            3 => SegmentType::Bss,
            other => SegmentType::Unknown(other),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            SegmentType::Text => ".text",
            SegmentType::Rodata => ".rodata",
            SegmentType::Data => ".data",
            SegmentType::Bss => ".bss",
            SegmentType::Unknown(_) => "?",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub offset: u32,
    pub size: u32,
    pub ty: SegmentType,
}

#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    /// Decoded file offset of the exported symbol.
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct ImportModule {
    pub name: String,
    pub indexed_count: u32,
    pub anonymous_count: u32,
}

#[derive(Debug)]
pub struct Cro<'a> {
    data: &'a [u8],
    pub file_size: u32,
    pub bss_size: u32,
    pub code_offset: u32,
    pub code_size: u32,
    pub data_offset: u32,
    pub data_size: u32,
    name_offset: u32,
    pub segments: Vec<Segment>,
    pub named_exports: Vec<Export>,
    pub indexed_export_count: usize,
    pub import_modules: Vec<ImportModule>,
    pub named_import_count: usize,
    pub internal_relocation_count: usize,
}

impl<'a> Cro<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < 0x138 {
            return Err(Error::truncated(0, 0x138, "cro header"));
        }
        if &data[MAGIC_OFFSET..MAGIC_OFFSET + 4] != MAGIC {
            return Err(Error::magic(
                MAGIC_OFFSET,
                "'CRO0' (at 0x80)",
                &data[MAGIC_OFFSET..MAGIC_OFFSET + 4],
            ));
        }
        let file_size = u32_le(data, 0x90)?;
        if file_size as usize != data.len() {
            return Err(Error::invalid(
                "cro header",
                format!("file size {file_size:#x} != actual {:#x}", data.len()),
            ));
        }

        let name_offset = u32_le(data, 0x84)?;
        let code_offset = u32_le(data, 0xB0)?;
        let code_size = u32_le(data, 0xB4)?;
        let data_offset = u32_le(data, 0xB8)?;
        let data_size = u32_le(data, 0xBC)?;

        let seg_off = u32_le(data, 0xC8)? as usize;
        let seg_num = u32_le(data, 0xCC)? as usize;
        if seg_num > 16 || seg_off + seg_num * 12 > data.len() {
            return Err(Error::invalid("cro segment table", "out of bounds"));
        }
        let segments = {
            let mut segs = Vec::with_capacity(seg_num);
            for i in 0..seg_num {
                let at = seg_off + i * 12;
                segs.push(Segment {
                    offset: u32_le(data, at)?,
                    size: u32_le(data, at + 4)?,
                    ty: SegmentType::from_id(u32_le(data, at + 8)?),
                });
            }
            segs
        };

        // Named exports: { u32 name file-offset, u32 segment-offset }.
        let exp_off = u32_le(data, 0xD0)? as usize;
        let exp_num = u32_le(data, 0xD4)? as usize;
        if exp_num > 100_000 || exp_off + exp_num * 8 > data.len() {
            return Err(Error::invalid("cro export table", "out of bounds"));
        }
        let mut named_exports = Vec::with_capacity(exp_num);
        for i in 0..exp_num {
            let at = exp_off + i * 8;
            let name_at = u32_le(data, at)? as usize;
            let seg_off_value = u32_le(data, at + 4)?;
            let Some(offset) = Self::decode_segment_offset(&segments, seg_off_value) else {
                return Err(Error::invalid(
                    "cro export",
                    format!("entry {i}: bad segment offset {seg_off_value:#x}"),
                ));
            };
            named_exports.push(Export {
                name: Self::string_at(data, name_at)?,
                offset,
            });
        }

        let indexed_export_count = u32_le(data, 0xDC)? as usize / 4;
        let named_import_count = u32_le(data, 0x104)? as usize;
        let internal_relocation_count = u32_le(data, 0x12C)? as usize;

        // Import modules: { name off, idx head, idx num, anon head, anon num }.
        let imp_off = u32_le(data, 0xF0)? as usize;
        let imp_num = u32_le(data, 0xF4)? as usize;
        let mut import_modules = Vec::new();
        if imp_num > 0 {
            if imp_num > 10_000 || imp_off + imp_num * 20 > data.len() {
                return Err(Error::invalid("cro import modules", "out of bounds"));
            }
            for i in 0..imp_num {
                let at = imp_off + i * 20;
                import_modules.push(ImportModule {
                    name: Self::string_at(data, u32_le(data, at)? as usize)?,
                    indexed_count: u32_le(data, at + 8)?,
                    anonymous_count: u32_le(data, at + 0x10)?,
                });
            }
        }

        Ok(Self {
            data,
            file_size,
            bss_size: u32_le(data, 0x94)?,
            code_offset,
            code_size,
            data_offset,
            data_size,
            name_offset,
            segments,
            named_exports,
            indexed_export_count,
            import_modules,
            named_import_count,
            internal_relocation_count,
        })
    }

    /// Module name (e.g. `DllBattle`, `|static|`).
    pub fn name(&self) -> Result<&str> {
        let at = self.name_offset as usize;
        let end = self
            .data
            .get(at..)
            .and_then(|rest| rest.iter().position(|&b| b == 0).map(|p| at + p))
            .ok_or_else(|| Error::truncated(at, 1, "cro module name"))?;
        std::str::from_utf8(&self.data[at..end])
            .map_err(|_| Error::invalid("cro module name", "not UTF-8"))
    }

    fn string_at(data: &[u8], at: usize) -> Result<String> {
        if at >= data.len() {
            return Err(Error::truncated(at, 1, "cro string"));
        }
        let end = data[at..]
            .iter()
            .position(|&b| b == 0)
            .map(|p| at + p)
            .unwrap_or(data.len());
        Ok(String::from_utf8_lossy(&data[at..end]).into_owned())
    }

    /// Decode a segment-offset value (bits 0-3 segment index, 4-31 offset).
    fn decode_segment_offset(segments: &[Segment], value: u32) -> Option<usize> {
        let seg = segments.get((value & 0xF) as usize)?;
        Some(seg.offset as usize + (value >> 4) as usize)
    }

    /// Borrow a segment's bytes (.bss excluded — zero-initialized).
    pub fn segment_data(&self, index: usize) -> Result<&'a [u8]> {
        let seg = self
            .segments
            .get(index)
            .ok_or_else(|| Error::invalid("cro segment", format!("no segment {index}")))?;
        if seg.ty == SegmentType::Bss {
            return Err(Error::invalid("cro segment", ".bss has no file data"));
        }
        let start = seg.offset as usize;
        let end = start.saturating_add(seg.size as usize);
        self.data
            .get(start..end)
            .ok_or_else(|| Error::truncated(start, seg.size as usize, "cro segment data"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        // name at 0x200, code seg at 0x100, one named export.
        let mut v = vec![0xCCu8; 0x210];
        // Header fields default to zero (counts!), padding stays 0xCC.
        v[0x80..0x138].fill(0);
        v[MAGIC_OFFSET..MAGIC_OFFSET + 4].copy_from_slice(MAGIC);
        v[0x84..0x88].copy_from_slice(&0x200u32.to_le_bytes()); // name offset
        let size = v.len() as u32;
        v[0x90..0x94].copy_from_slice(&size.to_le_bytes()); // file size
        v[0xB0..0xB4].copy_from_slice(&0x100u32.to_le_bytes()); // code offset
        v[0xB4..0xB8].copy_from_slice(&0x10u32.to_le_bytes()); // code size
        v[0xC0..0xC4].copy_from_slice(&0x200u32.to_le_bytes()); // module name
                                                                // segment table at 0x150: one .text {offset 0x100, size 0x10, type 0}
        v[0xC8..0xCC].copy_from_slice(&0x150u32.to_le_bytes());
        v[0xCC..0xD0].copy_from_slice(&1u32.to_le_bytes());
        v[0x150..0x154].copy_from_slice(&0x100u32.to_le_bytes());
        v[0x154..0x158].copy_from_slice(&0x10u32.to_le_bytes());
        v[0x158..0x15C].copy_from_slice(&0u32.to_le_bytes());
        // export table at 0x160: {name @0x180, segoff = segment 0, offset 4}
        v[0xD0..0xD4].copy_from_slice(&0x160u32.to_le_bytes());
        v[0xD4..0xD8].copy_from_slice(&1u32.to_le_bytes());
        v[0x160..0x164].copy_from_slice(&0x180u32.to_le_bytes());
        v[0x164..0x168].copy_from_slice(&0x40u32.to_le_bytes()); // (4 << 4) | 0
        v[0x180..0x186].copy_from_slice(b"funcA\0");
        v[0x200..0x208].copy_from_slice(b"DllTest\0");
        v
    }

    #[test]
    fn parses_synthetic_cro() {
        let v = fixture();
        let cro = Cro::parse(&v).expect("parse");
        assert_eq!(cro.name().unwrap(), "DllTest");
        assert_eq!(cro.segments.len(), 1);
        assert_eq!(cro.segments[0].ty, SegmentType::Text);
        assert_eq!(cro.named_exports.len(), 1);
        assert_eq!(cro.named_exports[0].name, "funcA");
        assert_eq!(cro.named_exports[0].offset, 0x104);
        assert_eq!(cro.segment_data(0).unwrap().len(), 0x10);
    }

    #[test]
    fn rejects_bad_magic_and_size() {
        let mut v = fixture();
        v[0x80] = b'X';
        assert!(Cro::parse(&v).is_err());
        let mut v = fixture();
        v[0x91] += 1; // size mismatch
        assert!(Cro::parse(&v).is_err());
        assert!(Cro::parse(&[0u8; 0x80]).is_err());
    }
}
