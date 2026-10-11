//! SMDH — home-menu icon and title metadata (ExeFS `icon.bin`).
//!
//! ```text
//! 0x0000  "SMDH", u16 version
//! 0x0008  16 × 0x200 application-title entries (per region):
//!            +0x00 long name        (UTF-16LE, NUL-terminated)
//!            +0x80 long publisher
//!            +0x100 short name
//!            +0x140 short publisher
//! 0x2040  48×48 icon, RGB565 little-endian (0x1200 bytes)
//! ```

use crate::error::Result;
use std::io::Write;
use xy_core::util::{u16_le, utf16le_string};

pub const ICON_SIZE: usize = 0x1200;
pub const ICON_OFFSET: usize = 0x2040;
pub const WIDTH: u32 = 48;
pub const HEIGHT: u32 = 48;

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
        if data.len() < ICON_OFFSET + ICON_SIZE {
            return Err(xy_core::Error::truncated(0, ICON_OFFSET + ICON_SIZE, "smdh").into());
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

    /// Icon as tightly packed RGB888 (48×48×3 bytes).
    pub fn icon_rgb(&self) -> Vec<u8> {
        let mut rgb = Vec::with_capacity((WIDTH * HEIGHT * 3) as usize);
        for i in 0..(WIDTH * HEIGHT) as usize {
            let v = u16_le(self.data, ICON_OFFSET + i * 2).unwrap_or(0);
            rgb.push((((v >> 11) & 0x1F) << 3) as u8);
            rgb.push((((v >> 5) & 0x3F) << 2) as u8);
            rgb.push(((v & 0x1F) << 3) as u8);
        }
        rgb
    }

    /// Icon nearest-neighbor upscaled by `scale` — the art is dithered on a
    /// dark background and reads as noise at 48×48 without zooming.
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

    /// Write the icon as a PNG, optionally nearest-neighbor upscaled.
    pub fn write_png_scaled(&self, w: &mut impl Write, scale: u32) -> Result<()> {
        let rgb = self.icon_rgb_scaled(scale);
        let mut encoder = png::Encoder::new(w, WIDTH * scale, HEIGHT * scale);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&rgb)?;
        Ok(())
    }

    /// Write the icon as a PNG.
    pub fn write_png(&self, w: &mut impl Write) -> Result<()> {
        self.write_png_scaled(w, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_smdh() {
        assert!(Smdh::parse(&[0u8; 0x36C0]).is_err());
    }
}
