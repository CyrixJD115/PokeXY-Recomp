//! SMDH — home-menu icon and title metadata (ExeFS `icon.bin`).
//!
//! ```text
//! 0x0000  "SMDH", u16 version
//! 0x0008  16 × 0x200 application-title entries (per region):
//!           +0x00 long name        (UTF-16LE, NUL-terminated)
//!           +0x80 long publisher
//!           +0x100 short name
//!           +0x140 short publisher
//! 0x2040  24×24 small icon, RGB565, Morton-tiled (0x480 bytes)
//! 0x24C0  48×48 large icon, RGB565, Morton-tiled (0x1200 bytes)
//! ```
//!
//! Icons are stored as 8×8 Morton tiles: Z-order inside each tile, tiles in
//! row-major order — the standard CTR texture layout. Decode is a port of
//! Azahar's `SMDH::GetIcon` + `GetMortonOffset`
//! (`.thirdparty/reference/1-emulation/azahar/src/core/loader/smdh.cpp`),
//! cross-checked pixel-for-pixel against an independent Python port of the
//! same code. (The first version of this parser read the *large* icon
//! linearly out of the *small*-icon slot at 0x2040 and produced noise that
//! a same-mistake reference image "validated" — fixed 2026-10-11.)

use crate::error::Result;
use std::io::Write;
use xy_core::util::{u16_le, utf16le_string};

pub const FILE_SIZE: usize = 0x36C0;
pub const SMALL_ICON_OFFSET: usize = 0x2040;
pub const SMALL_ICON_SIZE: usize = 0x480;
pub const SMALL_DIM: u32 = 24;
pub const LARGE_ICON_OFFSET: usize = 0x24C0;
pub const LARGE_ICON_SIZE: usize = 0x1200;
pub const WIDTH: u32 = 48;
pub const HEIGHT: u32 = 48;

/// Z-order (Morton) LUTs within one 8×8 tile — Azahar `MortonInterleave`.
const XLUT: [u32; 8] = [0x00, 0x01, 0x04, 0x05, 0x10, 0x11, 0x14, 0x15];
const YLUT: [u32; 8] = [0x00, 0x02, 0x08, 0x0a, 0x20, 0x22, 0x28, 0x2a];

/// Byte offset of pixel (x, y) inside a `size`×`size` Morton-tiled icon
/// (Azahar `GetMortonOffset(x, y, 2) + coarse_y * size * 2`).
fn morton_offset(x: u32, y: u32, size: u32) -> usize {
    let in_tile = XLUT[(x % 8) as usize] + YLUT[(y % 8) as usize];
    ((in_tile + (x & !7) * 8) * 2 + (y & !7) * size * 2) as usize
}

pub const REGIONS: [&str; 16] = [
    "JA", "EN", "FR", "DE", "IT", "ES", "ZH", "NL", "PT", "RU", "KO", "TW", "-", "-", "-", "-",
];

#[derive(Debug, Clone)]
pub struct Title {
    pub region: &'static str,
    pub long_name: String,
    pub long_publisher: String,
    pub short_name: String,
    pub short_publisher: String,
}

#[derive(Debug)]
pub struct Smdh<'a> {
    data: &'a [u8],
}

impl<'a> Smdh<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < FILE_SIZE {
            return Err(xy_core::Error::truncated(0, FILE_SIZE, "smdh").into());
        }
        if &data[0..4] != b"SMDH" {
            return Err(xy_core::Error::magic(0, "'SMDH'", &data[0..4]).into());
        }
        Ok(Self { data })
    }

    /// All non-empty regional title entries.
    pub fn titles(&self) -> Vec<Title> {
        (0..16)
            .filter_map(|i| {
                let base = 8 + i * 0x200;
                let long_name = utf16le_string(&self.data[base..base + 0x80]);
                if long_name.is_empty() {
                    return None;
                }
                Some(Title {
                    region: REGIONS[i],
                    long_name,
                    long_publisher: utf16le_string(&self.data[base + 0x80..base + 0x100]),
                    short_name: utf16le_string(&self.data[base + 0x100..base + 0x140]),
                    short_publisher: utf16le_string(&self.data[base + 0x140..base + 0x180]),
                })
            })
            .collect()
    }

    fn icon_rgb_at(&self, base: usize, size: u32) -> Vec<u8> {
        let mut rgb = Vec::with_capacity((size * size * 3) as usize);
        for y in 0..size {
            for x in 0..size {
                let v = u16_le(self.data, base + morton_offset(x, y, size)).unwrap_or(0);
                rgb.push((((v >> 11) & 0x1F) << 3) as u8);
                rgb.push((((v >> 5) & 0x3F) << 2) as u8);
                rgb.push(((v & 0x1F) << 3) as u8);
            }
        }
        rgb
    }

    /// Large icon (48×48) as tightly packed RGB888, Morton-untiled.
    pub fn icon_rgb(&self) -> Vec<u8> {
        self.icon_rgb_at(LARGE_ICON_OFFSET, WIDTH)
    }

    /// Small icon (24×24) as tightly packed RGB888, Morton-untiled.
    pub fn small_icon_rgb(&self) -> Vec<u8> {
        self.icon_rgb_at(SMALL_ICON_OFFSET, SMALL_DIM)
    }

    /// Icon nearest-neighbor upscaled by `scale` (48×48 source).
    pub fn icon_rgb_scaled(&self, scale: u32) -> Vec<u8> {
        let src = self.icon_rgb();
        let w = WIDTH as usize * scale as usize;
        let h = HEIGHT as usize * scale as usize;
        let mut rgb = Vec::with_capacity(w * h * 3);
        for y in 0..h {
            let sy = (y / scale as usize) * WIDTH as usize * 3;
            for x in 0..w {
                let sx = (x / scale as usize) * 3;
                rgb.extend_from_slice(&src[sy + sx..sy + sx + 3]);
            }
        }
        rgb
    }

    /// Write the large icon as a PNG, optionally nearest-neighbor upscaled.
    pub fn write_png_scaled(&self, w: &mut impl Write, scale: u32) -> Result<()> {
        let rgb = self.icon_rgb_scaled(scale);
        let mut encoder = png::Encoder::new(w, WIDTH * scale, HEIGHT * scale);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&rgb)?;
        Ok(())
    }

    /// Write the large icon as a PNG.
    pub fn write_png(&self, w: &mut impl Write) -> Result<()> {
        self.write_png_scaled(w, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_smdh() {
        assert!(Smdh::parse(&[0u8; FILE_SIZE]).is_err());
    }

    #[test]
    fn morton_offset_matches_azahar_reference() {
        // Reference offsets computed with an independent Python port of
        // Azahar GetMortonOffset(x, y, 2) + coarse_y * size * 2.
        assert_eq!(morton_offset(0, 0, 48), 0);
        assert_eq!(morton_offset(1, 0, 48), 2);
        assert_eq!(morton_offset(0, 1, 48), 4);
        assert_eq!(morton_offset(7, 7, 48), (0x2a + 0x15) * 2);
        assert_eq!(morton_offset(8, 0, 48), 8 * 8 * 2);
        assert_eq!(morton_offset(0, 8, 48), 48 * 2 * 8);
        assert_eq!(morton_offset(23, 23, 24), (16 * 8 + 0x15 + 0x2a) * 2 + 16 * 24 * 2);
    }
}
