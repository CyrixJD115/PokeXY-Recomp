//! BCRES (`.cgfx` — on-disk magic `CGFX`) structural inspection.
//!
//! BCRES is Nintendo's CTR graphics container (models, textures,
//! animations, environments). Pokémon X stores them as GARC sub-files
//! (`a/0/0/1`, `a/0/0/7`, `a/1/6/5`, …). Full mesh/material decoding is
//! Phase 2 (glTF export for Blender); this module provides validated
//! container inspection for inventorying the corpus.
//!
//! Header (0x14 bytes, verified against `a/0/0/1/0.bin` on 2026-10-05):
//!
//! ```text
//! 0x00  "CGFX"
//! 0x04  u16 byte-order marker (0xFEFF read LE, like the file fields)
//! 0x06  u16 header size (0x0014)
//! 0x08  u32 version stamp (0x05000000 in observed files)
//! 0x0C  u32 total size   (== file length; validated)
//! 0x10  u32 content count
//! 0x14  "DATA" chunk begins
//! ```
//!
//! Inside DATA: sub-chunks with 4CC tags (`DICT`, `CFLT`, `CALT`, `CENV`,
//! `MODL`, `TXPT`, `SKLA`, `ANIM`, …) and DICT name tables holding object
//! names as u16-length-prefixed strings. The exact DICT entry layout is
//! **not yet mapped** — names are harvested by scanning length-prefixed
//! ASCII (validated against `strings` output on real files).

use xy_core::error::{Error, Result};
use xy_core::util::{u16_le, u32_le};

pub const MAGIC: &[u8; 4] = b"CGFX";

#[derive(Debug)]
pub struct Bcres<'a> {
    data: &'a [u8],
    pub version_stamp: u32,
    pub content_count: u32,
}

impl<'a> Bcres<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < 0x14 {
            return Err(Error::truncated(0, 0x14, "bcres header"));
        }
        if &data[0..4] != MAGIC {
            return Err(Error::magic(0, "'CGFX' (BCRES)", &data[0..4]));
        }
        let bom = u16_le(data, 0x04)?;
        if bom != 0xFEFF {
            return Err(Error::invalid("bcres header", format!("BOM {bom:#06x}")));
        }
        let header_size = u16_le(data, 0x06)? as usize;
        if header_size != 0x14 {
            return Err(Error::invalid(
                "bcres header",
                format!("header size {header_size:#x}, expected 0x14"),
            ));
        }
        let total = u32_le(data, 0x0C)? as usize;
        if total != data.len() {
            return Err(Error::invalid(
                "bcres size",
                format!("header says {total:#x}, file is {:#x}", data.len()),
            ));
        }
        if &data[0x14..0x18] != b"DATA" {
            return Err(Error::magic(0x14, "'DATA'", &data[0x14..0x18]));
        }
        Ok(Self {
            data,
            version_stamp: u32_le(data, 0x08)?,
            content_count: u32_le(data, 0x10)?,
        })
    }

    /// Known sub-chunk tags we census (grows as formats are mapped).
    fn is_tag(tag: &[u8]) -> bool {
        tag.len() == 4
            && tag
                .iter()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || *b == b'@' || *b == b'_')
            && tag != b"DATA"
    }

    /// Census of 4CC sub-chunk tags at 4-byte alignment, in file order,
    /// deduplicated to consecutive repeats.
    pub fn chunk_tags(&self) -> Vec<(usize, [u8; 4])> {
        let mut out = Vec::new();
        let mut last: Option<[u8; 4]> = None;
        let mut off = 0x14;
        while off + 4 <= self.data.len() {
            let tag: [u8; 4] = [
                self.data[off],
                self.data[off + 1],
                self.data[off + 2],
                self.data[off + 3],
            ];
            if Self::is_tag(&tag) && last != Some(tag) {
                out.push((off, tag));
                last = Some(tag);
            }
            off += 4;
        }
        out
    }

    /// Harvest object names: NUL-terminated printable ASCII runs of
    /// 3..=64 bytes (the DICT string pools). Chunk-tag-like strings
    /// (all uppercase, ≤ 4 chars) are filtered out.
    pub fn names(&self) -> Vec<String> {
        let d = self.data;
        let mut out = Vec::new();
        let mut start: Option<usize> = None;
        for i in 0..=d.len() {
            let b = d.get(i).copied();
            match (start, b) {
                (Some(s), Some(0)) | (Some(s), None) => {
                    let run = &d[s..i.min(d.len())];
                    if (3..=64).contains(&run.len())
                        && run.iter().all(|c| c.is_ascii_graphic())
                        && run
                            .iter()
                            .any(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                    {
                        out.push(String::from_utf8_lossy(run).into_owned());
                    }
                    start = None;
                }
                (None, Some(b)) if b.is_ascii_graphic() => start = Some(i),
                (Some(_), Some(b)) if b.is_ascii_graphic() => {}
                _ => start = None,
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_bcres() {
        assert!(Bcres::parse(b"nope").is_err());
        let mut v = vec![0u8; 0x18];
        v[..4].copy_from_slice(MAGIC);
        v[4..6].copy_from_slice(&0xFEFFu16.to_le_bytes());
        v[6..8].copy_from_slice(&0x14u16.to_le_bytes());
        let size = v.len() as u32;
        v[0x0C..0x10].copy_from_slice(&size.to_le_bytes());
        v[0x14..0x18].copy_from_slice(b"DATA");
        assert!(Bcres::parse(&v).is_ok());
        v[0x0D] = 0xFF; // size mismatch
        assert!(Bcres::parse(&v).is_err());
    }
}
