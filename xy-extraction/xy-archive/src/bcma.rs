//! The Pokémon X/Y e-manual archive (`Manual.bcma`) — a DARC-family container
//! with a layout that differs from the standard CTR DARC.
//!
//! Reverse-engineered layout (validated against title `0004000000055D00`):
//!
//! ```text
//! header (0x1C):
//!   0x00  "darc"
//!   0x04  u16 BOM (0xFFFE stored; fields below are little-endian)
//!   0x06  u16 header size (0x1C)
//!   0x08  u32 version
//!   0x0C  u32 total file size
//!   0x10  u32 table offset (0x1C)
//!   0x14  u32 table size
//!   0x18  u32 data offset (end of table; data offsets are absolute anyway)
//!
//! node table at 0x1C — `node_count` entries × 12 bytes:
//!   +0x00 u32 bit 24 = directory flag, low 24 bits = name id
//!   +0x04 u32 file: absolute data offset   | dir: (unused here)
//!   +0x08 u32 file: data size              | dir: total node count (root)
//!
//! string table directly after the node area:
//!   name id = BYTE offset from the end of the node area
//!   names are UTF-16LE, NUL-terminated
//! ```
//!
//! Quirks discovered in the wild:
//! * names may be **suffix-shared**: a node's name id can point into the
//!   middle of a longer string, so the terminator scan must stay 2-byte
//!   aligned and duplicate file names are possible
//! * the root directory's name id is 0 (empty name)
//! * this is *not* the standard 24-byte-node DARC; don't mix the two

use xy_core::error::{Error, Result};
use xy_core::util::u32_le;

#[derive(Debug, Clone)]
pub struct Node {
    pub is_dir: bool,
    pub name: String,
    /// File: absolute offset in the container. Dir: unused.
    pub offset: u32,
    /// File: data size. Dir (root): total node count.
    pub size: u32,
}

#[derive(Debug)]
pub struct Bcma<'a> {
    pub image: &'a [u8],
    pub nodes: Vec<Node>,
}

impl<'a> Bcma<'a> {
    /// Parse a full container image (including the 0x1C header).
    pub fn parse(image: &'a [u8]) -> Result<Self> {
        if image.len() < 0x1C {
            return Err(Error::truncated(0, 0x1C, "bcma header"));
        }
        if &image[0..4] != b"darc" {
            return Err(Error::magic(0, "'darc'", &image[0..4]));
        }
        let table_off = u32_le(image, 0x10)? as usize;
        let data_off = u32_le(image, 0x18)? as usize;
        if table_off >= image.len() || table_off > data_off || data_off > image.len() {
            return Err(Error::invalid(
                "bcma header",
                format!("table [{table_off:#x}, {data_off:#x}) out of bounds"),
            ));
        }
        let table = &image[table_off..data_off];
        if table.len() < 12 {
            return Err(Error::truncated(table_off, 12, "bcma node table"));
        }

        // Root node's `b` field sizes the node table.
        let node_count = u32_le(table, 8)? as usize;
        if node_count == 0 || table.len() < node_count * 12 {
            return Err(Error::invalid(
                "bcma node table",
                format!(
                    "node count {node_count} exceeds table size {:#x}",
                    table.len()
                ),
            ));
        }
        let strbase = node_count * 12;

        let name = |nid: u32| decode_name(table, strbase + nid as usize);

        let mut nodes = Vec::with_capacity(node_count);
        for i in 0..node_count {
            let (fl, a, b) = (
                u32_le(table, i * 12)?,
                u32_le(table, i * 12 + 4)?,
                u32_le(table, i * 12 + 8)?,
            );
            nodes.push(Node {
                is_dir: fl & 0x0100_0000 != 0,
                name: name(fl & 0x00FF_FFFF),
                offset: a,
                size: b,
            });
        }
        Ok(Self { image, nodes })
    }

    /// Iterate `(name, data)` for every file node, in table order.
    /// Names may repeat (suffix sharing) — callers should deduplicate.
    pub fn files(&self) -> impl Iterator<Item = Result<(&str, &'a [u8])>> + '_ {
        self.nodes.iter().filter_map(move |n| {
            if n.is_dir || n.name.is_empty() {
                return None;
            }
            let start = n.offset as usize;
            let end = start.saturating_add(n.size as usize);
            let slice = self
                .image
                .get(start..end)
                .ok_or_else(|| {
                    Error::invalid(
                        "bcma file entry",
                        format!(
                            "{:?} covers {start:#x}..{end:#x}, past end of image",
                            n.name
                        ),
                    )
                })
                .map(|data| (n.name.as_str(), data));
            Some(slice)
        })
    }
}

/// Deduplicate names (suffix-shared strings in BCMA can collide): appends
/// `001_`, `002_`, … before the first repeat. Shared by the CLI and the
/// golden tests so naming stays pinned.
pub fn unique_name(name: &str, seen: &mut std::collections::HashSet<String>) -> String {
    if seen.insert(name.to_string()) {
        return name.to_string();
    }
    let mut i = 1;
    loop {
        let candidate = format!("{:03}_{name}", i);
        if seen.insert(candidate.clone()) {
            return candidate;
        }
        i += 1;
    }
}

/// Decode a NUL-terminated UTF-16LE name at byte offset `p`, scanning for the
/// terminator on 2-byte boundaries only (a byte-wise scan would stop early on
/// the high byte of the last real character).
fn decode_name(table: &[u8], p: usize) -> String {
    if p + 1 >= table.len() {
        return String::new();
    }
    let mut end = p;
    while end + 1 < table.len() && table[end..end + 2] != *b"\x00\x00" {
        end += 2;
    }
    let units: Vec<u16> = table[p..end]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|c| u16::from_le_bytes(*c))
        .collect();
    String::from_utf16_lossy(&units)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminator_scan_is_pair_aligned() {
        // 'c' = 63 00 followed by 00 00: a byte-wise scan would see the
        // high byte of 'c' + first terminator byte as "found".
        let table = [0x63, 0x00, 0x00, 0x00];
        assert_eq!(decode_name(&table, 0), "c");
    }
}
