//! `xy-core` — shared primitives for the XY decompilation toolkit.
//!
//! * [`error`] — the typed error enum used by every parser
//! * [`util`]  — bounds-checked little-endian reads + UTF-16LE decoding
//! * [`lz10`]  — Nintendo LZ10/LZ11 bitstream decompression
//! * [`sniff`] — magic-byte identification shared by the CLI
//!
//! This crate stays logic-only (no format parsers, no `png`, no I/O beyond
//! what the error type needs). Parsers elsewhere borrow their input, return
//! [`Result`], and never panic on malformed input.

pub mod error;
pub mod lz10;
pub mod util;

pub use error::{Error, Result};

/// Quick magic-based identification of a file image.
pub fn sniff(data: &[u8]) -> Option<&'static str> {
    // Real .3ds files begin with a 0x100-byte RSA signature; "NCSD" sits at
    // 0x100 (mirrors the logic in `xy_archive::romfs::Romfs::open`).
    if data.len() > 0x120 && &data[0x100..0x104] == b"NCSD" {
        return Some("CCI card image (.3ds/.cci)");
    }
    // CRO/CRS code modules carry four SHA-256 hashes before the magic.
    if data.len() > 0x84 && &data[0x80..0x84] == b"CRO0" {
        return Some("CRO code module (.cro/.crs, magic at 0x80)");
    }
    let m4 = data.get(0..4)?;
    Some(match m4 {
        b"NCSD" => "CCI card image (.3ds/.cci)",
        b"NCCH" => "NCCH container",
        b"IVFC" => "RomFS (IVFC-wrapped)",
        b"darc" => "DARC-family archive (XY e-manual variant)",
        b"CRAG" => "GARC game archive",
        b"CWAV" => "BCWAV audio",
        b"SMDH" => "SMDH icon/title metadata",
        b"CBMD" => "CBMD home-menu banner",
        b"CLIM" => "BCLIM texture",
        b"RIFF" => "RIFF (WAV)",
        _ if !data.is_empty() && (data[0] == 0x10 || data[0] == 0x11) => {
            "Nintendo LZ10/LZ11 (probable)"
        }
        _ => return None,
    })
}
