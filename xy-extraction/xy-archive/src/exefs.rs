//! ExeFS extraction — the executable filesystem section inside any NCCH
//! (CXI/CCI partition or CIA content): `.code`, `icon`, `banner`, `logo`.
//!
//! Layout references (3dbrew "ExeFS"):
//! * Header is a fixed 0x200 bytes: 10 × 16-byte file-header entries
//!   `{name[8] ASCII NUL-padded, offset u32, size u32}`, offset relative to
//!   the end of the header (absolute = 0x200 + offset); the remaining
//!   0x140 bytes hold 10 × 32-byte SHA-256 hashes (one per slot, unused
//!   here — this toolkit only sees decrypted content, same stance as
//!   `romfs.rs`). Unused slots are all-zero and are skipped.
//! * Locating ExeFS inside its container mirrors `romfs.rs`: CCI partition 0
//!   descriptor at 0x120 (NCSD) -> NCCH header -> ExeFS offset/size (u32,
//!   `0x200`-byte media units) at NCCH+0x1A0/0x1A4.
//! * A standalone CXI (e.g. sliced out of a CIA's content region) has its
//!   own 0x100-byte signature before the `NCCH` magic, same as a cart NCCH
//!   partition — `open()` detects that case the same way it detects an
//!   NCSD card image with the magic at 0x100 instead of 0x0.

use std::fs;
use std::path::Path;
use xy_core::error::{Error, Result};
use xy_core::util::u32_le;

const HEADER_SIZE: usize = 0x200;
const ENTRY_COUNT: usize = 10;
const ENTRY_SIZE: usize = 0x10;

#[derive(Debug, Clone)]
pub struct ExefsEntry {
    /// ExeFS section name (e.g. `.code`, `icon`, `banner`, `logo`).
    pub name: String,
    /// Absolute offset of the entry's data within the Exefs's own image.
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug)]
pub struct Exefs<'a> {
    image: &'a [u8],
    entries: Vec<ExefsEntry>,
}

impl<'a> Exefs<'a> {
    /// Open an ExeFS from whichever container wraps it: a CCI (`.3ds`/`.cci`)
    /// or a standalone CXI/NCCH (with its leading signature, or headerless).
    pub fn open(image: &'a [u8]) -> Result<Self> {
        // A standalone CXI carries its own 0x100-byte signature before the
        // magic, same as a cart NCCH partition — check that before the
        // headerless case. Real .3ds CCI images carry the same leading
        // signature before "NCSD" (see romfs.rs); "NCSD"/"NCCH" only sit at
        // 0x0 in headerless tooling dumps.
        if image.len() > 0x1a8 {
            match image.get(0x100..0x104) {
                Some(b"NCCH") => return Self::from_ncch(image, 0),
                Some(b"NCSD") => return Self::from_cci(image),
                _ => {}
            }
        }
        match image.get(0..4) {
            Some(b"NCSD") => Self::from_cci(image),
            Some(b"NCCH") => Self::from_ncch(image, 0),
            _ => Err(Error::magic(
                0,
                "'NCSD' (at 0x0 or 0x100) or 'NCCH' (at 0x0 or 0x100)",
                image.get(0..4).unwrap_or(&[]),
            )),
        }
    }

    fn from_cci(image: &'a [u8]) -> Result<Self> {
        // Partition 0 descriptor at 0x120: (start sector, end sector) × 512 B.
        let start = u32_le(image, 0x120)? as usize * 512;
        Self::from_ncch(image, start)
    }

    fn from_ncch(image: &'a [u8], base: usize) -> Result<Self> {
        let ncch = base + 0x100;
        if image.get(ncch..ncch + 4) != Some(b"NCCH") {
            return Err(Error::magic(
                ncch,
                "'NCCH'",
                image.get(ncch..ncch + 4).unwrap_or(&[]),
            ));
        }
        let off = u32_le(image, base + 0x1A0)? as usize * 512;
        let size = u32_le(image, base + 0x1A4)? as usize * 512;
        if off == 0 || size == 0 {
            return Err(Error::invalid(
                "ncch",
                "no ExeFS section (offset/size is zero)",
            ));
        }
        let abs = base + off;
        if abs.checked_add(size).is_none_or(|end| end > image.len()) {
            return Err(Error::truncated(abs, size, "exefs region"));
        }
        Self::parse(&image[abs..abs + size])
    }

    /// Parse a raw ExeFS region (header at offset 0, no outer container).
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < HEADER_SIZE {
            return Err(Error::truncated(0, HEADER_SIZE, "exefs header"));
        }
        let mut entries = Vec::new();
        for i in 0..ENTRY_COUNT {
            let base = i * ENTRY_SIZE;
            let name = ascii_name(&data[base..base + 8]);
            let rel_off = u32_le(data, base + 8)? as usize;
            let size = u32_le(data, base + 0xC)? as usize;
            if size == 0 {
                continue;
            }
            let abs = HEADER_SIZE
                .checked_add(rel_off)
                .ok_or_else(|| Error::invalid("exefs entry", format!("{name}: offset overflow")))?;
            let end = abs
                .checked_add(size)
                .ok_or_else(|| Error::invalid("exefs entry", format!("{name}: size overflow")))?;
            if end > data.len() {
                return Err(Error::truncated(abs, size, "exefs entry data"));
            }
            entries.push(ExefsEntry {
                name,
                offset: abs,
                size,
            });
        }
        // Slot order in the header isn't guaranteed to match data layout.
        entries.sort_by_key(|e| e.offset);
        Ok(Self {
            image: data,
            entries,
        })
    }

    pub fn entries(&self) -> &[ExefsEntry] {
        &self.entries
    }

    pub fn data(&self, entry: &ExefsEntry) -> &'a [u8] {
        &self.image[entry.offset..entry.offset + entry.size]
    }

    /// Look up an entry by exact name (e.g. `.code`, `icon`, `banner`).
    pub fn find(&self, name: &str) -> Option<&ExefsEntry> {
        self.entries.iter().find(|e| e.name == name)
    }

    /// Extract every entry under `out_dir`, one file per section (named
    /// after the ExeFS entry name). Returns the number of files written.
    pub fn extract(&self, out_dir: &Path) -> Result<usize> {
        fs::create_dir_all(out_dir)?;
        let mut count = 0;
        for e in &self.entries {
            fs::write(out_dir.join(&e.name), self.data(e))?;
            count += 1;
        }
        Ok(count)
    }
}

fn ascii_name(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_exefs(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut data_section = Vec::new();
        let mut header = vec![0u8; HEADER_SIZE];
        for (i, (name, bytes)) in entries.iter().enumerate() {
            let offset = data_section.len() as u32;
            let base = i * ENTRY_SIZE;
            let name_bytes = name.as_bytes();
            header[base..base + name_bytes.len()].copy_from_slice(name_bytes);
            header[base + 8..base + 12].copy_from_slice(&offset.to_le_bytes());
            header[base + 12..base + 16].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
            data_section.extend_from_slice(bytes);
        }
        header.extend_from_slice(&data_section);
        header
    }

    #[test]
    fn parses_bare_exefs_entries() {
        let image = build_exefs(&[(".code", b"CODE-BYTES"), ("icon", b"ICON-BYTES")]);
        let exefs = Exefs::parse(&image).unwrap();
        assert_eq!(exefs.entries().len(), 2);
        let code = exefs.find(".code").unwrap();
        assert_eq!(exefs.data(code), b"CODE-BYTES");
        let icon = exefs.find("icon").unwrap();
        assert_eq!(exefs.data(icon), b"ICON-BYTES");
    }

    #[test]
    fn skips_empty_slots() {
        let image = build_exefs(&[(".code", b"X")]);
        let exefs = Exefs::parse(&image).unwrap();
        assert_eq!(exefs.entries().len(), 1);
    }

    #[test]
    fn rejects_truncated_header() {
        assert!(Exefs::parse(&[0u8; 0x10]).is_err());
    }

    #[test]
    fn extract_writes_one_file_per_entry() {
        let image = build_exefs(&[(".code", b"CODE"), ("banner", b"BANNER")]);
        let exefs = Exefs::parse(&image).unwrap();
        let dir = std::env::temp_dir().join(format!("xy-exefs-test-{}", std::process::id()));
        let n = exefs.extract(&dir).unwrap();
        assert_eq!(n, 2);
        assert_eq!(fs::read(dir.join(".code")).unwrap(), b"CODE");
        assert_eq!(fs::read(dir.join("banner")).unwrap(), b"BANNER");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn open_rejects_unknown_magic() {
        assert!(Exefs::open(b"????").is_err());
    }
}
