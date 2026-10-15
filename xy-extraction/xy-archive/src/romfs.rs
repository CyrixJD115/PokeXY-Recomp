//! RomFS extraction — directly from a CCI (`.3ds`/`.cci`), a bare NCCH, or a
//! raw IVFC-wrapped RomFS image.
//!
//! Layout references (3dbrew):
//! * CCI: "NCSD" header, partition descriptors at 0x20 (start/end in 512-byte
//!   sectors); partition 0 holds the game NCCH.
//! * NCCH: RomFS offset/size (u32, relative to NCCH start) at 0x1DC/0x1E0.
//! * IVFC header: u32 total size at 0x58; the RomFS proper follows it.
//! * RomFS header (0x28 bytes): directory/file hash+metadata table offsets
//!   (relative to RomFS start), then the file data offset.
//! * Directory entry: parent, sibling, first child dir, first child file,
//!   hash sibling (u32 × 5), u32 name length, UTF-16 name.
//!   File entry: parent, sibling (u32 × 2), data offset, data size (u64 × 2),
//!   hash sibling, name length (u32), UTF-16 name.
//! * Chains end with 0xFFFFFFFF; root is the first directory entry.
//!
//! Only decrypted images are supported (Pokémon X/Y dumps with
//! `Crypto Key: None`); encrypted partitions are detected and rejected.

use std::fs;
use std::io::Write;
use std::path::Path;
use xy_core::error::{Error, Result};
use xy_core::util::{u32_le, u64_le};

const END: u32 = 0xFFFF_FFFF;

#[derive(Debug, Clone)]
pub struct Entry {
    /// Path relative to the RomFS root, `/`-separated, no leading slash.
    pub path: String,
    /// Absolute offset of the file contents in the source image.
    pub offset: usize,
    pub size: u64,
}

#[derive(Debug)]
pub struct Romfs<'a> {
    image: &'a [u8],
    /// RomFS start (after the IVFC header).
    base: usize,
    dir_table: (u64, u64),
    file_table: (u64, u64),
    data_offset: u64,
}

impl<'a> Romfs<'a> {
    /// Open a RomFS from whichever container `image` is.
    pub fn open(image: &'a [u8]) -> Result<Self> {
        // Real .3ds files begin with the 0x100-byte NCSD RSA signature; the
        // "NCSD" magic only sits at 0x0 in headerless tooling dumps.
        if image.len() > 0x120 && &image[0x100..0x104] == b"NCSD" {
            return Self::from_cci(image);
        }
        match image.get(0..4) {
            Some(b"NCSD") => Self::from_cci(image),
            Some(b"NCCH") => Self::from_ncch(image, 0),
            Some(b"IVFC") => Self::parse(image),
            _ => Err(Error::magic(
                0,
                "'NCSD' (at 0x0 or 0x100), 'NCCH' or 'IVFC'",
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
        // RomFS offset/size are u32s in 0x200-byte media units (NCCH header:
        // plain @0x190, logo @0x198, exefs @0x1A0, romfs offset @0x1B0,
        // romfs size @0x1B4, romfs hash-region @0x1B8, superblock hashes @0x1C0/0x1E0).
        let off = u32_le(image, base + 0x1B0)? as usize * 512;
        let size = u32_le(image, base + 0x1B4)? as usize * 512;
        let abs = base + off;
        if abs + size > image.len() {
            return Err(Error::truncated(abs, size, "romfs region"));
        }
        if image.get(abs..abs + 4) != Some(b"IVFC") {
            return Err(Error::unsupported(
                "ncch",
                "RomFS does not start with IVFC magic — the partition is probably encrypted; \
                 only decrypted dumps (Crypto Key: None) are supported",
            ));
        }
        Self::parse(&image[abs..abs + size])
    }

    /// Parse a raw IVFC-wrapped RomFS image.
    pub fn parse(image: &'a [u8]) -> Result<Self> {
        if image.len() < 0x60 {
            return Err(Error::truncated(0, 0x60, "ivfc header"));
        }
        if &image[0..4] != b"IVFC" {
            return Err(Error::magic(0, "'IVFC'", &image[0..4]));
        }
        let ivfc_size = u32_le(image, 0x58)? as usize;

        // The IVFC header is followed by the master hash and alignment padding
        // before the RomFS header; locate it structurally: u32 0x28 magic size,
        // with the file table ending at/below the declared data offset.
        let mut base = None;
        let mut cand = (ivfc_size + 15) & !15;
        while cand + 0x28 <= image.len() && cand <= ivfc_size + 0x2000 {
            if let (Ok(sz), Ok(data_off), Ok(ft_off), Ok(ft_size)) = (
                u32_le(image, cand),
                u32_le(image, cand + 0x24),
                u32_le(image, cand + 0x1C),
                u32_le(image, cand + 0x20),
            ) {
                if sz == 0x28 && data_off >= ft_off + ft_size {
                    base = Some(cand);
                    break;
                }
            }
            cand += 4;
        }
        let base = base.ok_or_else(|| {
            Error::invalid("ivfc", "no valid RomFS header found after IVFC header")
        })?;

        // RomFS header: all u32s, offsets relative to `base`.
        //  0x00 header size, 0x04/08 dirhash off/size, 0x0C/10 dir table off/size,
        //  0x14/18 file hash off/size, 0x1C/20 file table off/size, 0x24 data offset
        let dir_table = (
            u32_le(image, base + 0x0C)? as u64,
            u32_le(image, base + 0x10)? as u64,
        );
        let file_table = (
            u32_le(image, base + 0x1C)? as u64,
            u32_le(image, base + 0x20)? as u64,
        );
        let data_offset = u32_le(image, base + 0x24)? as u64;
        Ok(Self {
            image,
            base,
            dir_table,
            file_table,
            data_offset,
        })
    }

    fn dir_entry(&self, off: usize) -> Result<DirEntry> {
        let at = self.base + self.dir_table.0 as usize + off;
        let d = &self.image;
        let name_len = u32_le(d, at + 20)? as usize;
        let name_at = at + 24;
        let raw = d
            .get(name_at..name_at.saturating_add(name_len))
            .ok_or_else(|| Error::truncated(name_at, name_len, "dir name"))?;
        Ok(DirEntry {
            parent: u32_le(d, at)?,
            sibling: u32_le(d, at + 4)?,
            child_dir: u32_le(d, at + 8)?,
            child_file: u32_le(d, at + 12)?,
            name: decode_name(raw),
        })
    }

    fn file_entry(&self, off: usize) -> Result<FileEntry> {
        let at = self.base + self.file_table.0 as usize + off;
        let d = &self.image;
        let name_len = u32_le(d, at + 28)? as usize;
        let name_at = at + 32;
        let raw = d
            .get(name_at..name_at.saturating_add(name_len))
            .ok_or_else(|| Error::truncated(name_at, name_len, "file name"))?;
        Ok(FileEntry {
            parent: u32_le(d, at)?,
            sibling: u32_le(d, at + 4)?,
            offset: u64_le(d, at + 8)?,
            size: u64_le(d, at + 16)?,
            name: decode_name(raw),
        })
    }

    /// Walk the whole filesystem and return every file as a flat entry list.
    pub fn walk(&self) -> Result<Vec<Entry>> {
        let mut out = Vec::new();
        let root = self.dir_entry(0)?;
        self.walk_dir(root.child_dir, root.child_file, String::new(), &mut out, 0)?;
        Ok(out)
    }

    fn walk_dir(
        &self,
        child_dir: u32,
        child_file: u32,
        prefix: String,
        out: &mut Vec<Entry>,
        depth: usize,
    ) -> Result<()> {
        if depth > 32 {
            return Err(Error::invalid(
                "romfs",
                "directory nesting exceeds 32 levels (cycle?)",
            ));
        }
        // Child directories.
        let mut off = child_dir;
        let mut guard = 0usize;
        while off != END {
            guard += 1;
            if guard > 1_000_000 {
                return Err(Error::invalid(
                    "romfs",
                    "directory sibling chain too long (cycle?)",
                ));
            }
            let e = self.dir_entry(off as usize)?;
            let path = if prefix.is_empty() {
                e.name.clone()
            } else {
                format!("{prefix}/{}", e.name)
            };
            self.walk_dir(e.child_dir, e.child_file, path, out, depth + 1)?;
            off = e.sibling;
        }
        // Child files.
        let mut off = child_file;
        let mut guard = 0usize;
        while off != END {
            guard += 1;
            if guard > 1_000_000 {
                return Err(Error::invalid(
                    "romfs",
                    "file sibling chain too long (cycle?)",
                ));
            }
            let e = self.file_entry(off as usize)?;
            let path = if prefix.is_empty() {
                e.name.clone()
            } else {
                format!("{prefix}/{}", e.name)
            };
            out.push(Entry {
                path,
                offset: self.base + self.data_offset as usize + e.offset as usize,
                size: e.size,
            });
            off = e.sibling;
        }
        Ok(())
    }

    /// The image slice offsets in [`Entry`](Self::walk) are relative to —
    /// the IVFC-wrapped RomFS region, not the outer container.
    pub fn image(&self) -> &'a [u8] {
        self.image
    }

    /// Extract every file under `out_dir`. Returns the number of files written.
    pub fn extract(&self, out_dir: &Path) -> Result<usize> {
        let entries = self.walk()?;
        for e in &entries {
            let dest = out_dir.join(&e.path);
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            let blob = self
                .image
                .get(e.offset..e.offset.saturating_add(e.size as usize))
                .ok_or_else(|| {
                    Error::invalid(
                        "romfs file",
                        format!(
                            "{} covers {:#x}..{:#x}, past end of image",
                            e.path,
                            e.offset,
                            e.offset as u64 + e.size
                        ),
                    )
                })?;
            let mut f = fs::File::create(&dest)?;
            f.write_all(blob)?;
        }
        Ok(entries.len())
    }
}

struct DirEntry {
    #[allow(dead_code)]
    parent: u32,
    sibling: u32,
    child_dir: u32,
    child_file: u32,
    name: String,
}

struct FileEntry {
    #[allow(dead_code)]
    parent: u32,
    sibling: u32,
    offset: u64,
    size: u64,
    name: String,
}

/// RomFS stores names as UTF-16; byte order is chosen per-name by scoring
/// which decoding looks like sane text (handles mixed tool output safely).
fn decode_name(raw: &[u8]) -> String {
    let units: Vec<u16> = raw
        .as_chunks::<2>()
        .0
        .iter()
        .map(|c| u16::from_le_bytes(*c))
        .collect();
    let end = units.iter().position(|&u| u == 0).unwrap_or(units.len());
    let units = &units[..end];

    let le = String::from_utf16_lossy(units);
    let be_units: Vec<u16> = units.iter().map(|u| u.swap_bytes()).collect();
    let be = String::from_utf16_lossy(&be_units);

    if score(&le) >= score(&be) {
        le
    } else {
        be
    }
}

fn score(s: &str) -> f32 {
    if s.is_empty() {
        return 0.0;
    }
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '/' | ' '))
        .count() as f32
        / s.chars().count() as f32
}
