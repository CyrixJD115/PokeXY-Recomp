//! PICA200 texture decoding — port of Ohana3DS-Rebirth `TextureCodec.cs`.
//!
//! Decodes raw texture payloads (as referenced by CGFX `TXOB` headers and
//! BCH texture entries) into RGBA8 byte buffers. All 14 PICA formats are
//! tiled 8×8 with a Morton `tile_order`; ETC1/ETC1A4 additionally use the
//! 3DS block-scramble order (the two 32-bit halves of each 64-bit ETC1
//! block are swapped versus standard ETC1 — see `etc1_decode`).
//!
//! Port notes (parity): the C# writes texels into a `Format32bppArgb`
//! buffer, i.e. **BGRA** byte order; every case here reproduces those
//! exact writes and the final pass swaps slots 0/2 to produce RGBA.
//! Channel naming inside each case follows the C# source (the variables
//! named `r`/`g`/`b` sometimes carry B/G/R bits — the slot assignments,
//! not the names, define the format).

use xy_core::error::{Error, Result};

/// PICA200 texture pixel formats (3dbrew "GPU texture formats"; the
/// `OTextureFormat` enum values in Ohana3DS RenderBase.cs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    Rgba8,
    Rgb8,
    Rgba5551,
    Rgb565,
    Rgba4,
    La8,
    Hilo8,
    L8,
    A8,
    La4,
    L4,
    A4,
    Etc1,
    Etc1a4,
}

impl TextureFormat {
    pub fn from_u32(v: u32) -> Result<Self> {
        Ok(match v {
            0 => Self::Rgba8,
            1 => Self::Rgb8,
            2 => Self::Rgba5551,
            3 => Self::Rgb565,
            4 => Self::Rgba4,
            5 => Self::La8,
            6 => Self::Hilo8,
            7 => Self::L8,
            8 => Self::A8,
            9 => Self::La4,
            0xa => Self::L4,
            0xb => Self::A4,
            0xc => Self::Etc1,
            0xd => Self::Etc1a4,
            other => return Err(Error::invalid("texture format", format!("{other:#x}"))),
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Rgba8 => "rgba8",
            Self::Rgb8 => "rgb8",
            Self::Rgba5551 => "rgba5551",
            Self::Rgb565 => "rgb565",
            Self::Rgba4 => "rgba4",
            Self::La8 => "la8",
            Self::Hilo8 => "hilo8",
            Self::L8 => "l8",
            Self::A8 => "a8",
            Self::La4 => "la4",
            Self::L4 => "l4",
            Self::A4 => "a4",
            Self::Etc1 => "etc1",
            Self::Etc1a4 => "etc1a4",
        }
    }

    pub fn bits_per_pixel(self) -> u32 {
        match self {
            Self::Rgba8 => 32,
            Self::Rgb8 => 24,
            Self::Rgba5551 | Self::Rgb565 | Self::Rgba4 | Self::La8 | Self::Hilo8 => 16,
            Self::L8 | Self::A8 | Self::La4 => 8,
            Self::L4 | Self::A4 => 4,
            Self::Etc1 => 4,
            Self::Etc1a4 => 8,
        }
    }

    /// Whether the format stores an alpha channel (drives export blend modes).
    pub fn has_alpha(self) -> bool {
        matches!(
            self,
            Self::Rgba8 | Self::Rgba5551 | Self::Rgba4 | Self::A8 | Self::A4 | Self::Etc1a4
        )
    }
}

/// Morton order of pixels inside each 8×8 tile (TextureCodec.tileOrder).
const TILE_ORDER: [usize; 64] = [
    0, 1, 8, 9, 2, 3, 10, 11, 16, 17, 24, 25, 18, 19, 26, 27, 4, 5, 12, 13, 6, 7, 14, 15, 20, 21,
    28, 29, 22, 23, 30, 31, 32, 33, 40, 41, 34, 35, 42, 43, 48, 49, 56, 57, 50, 51, 58, 59, 36, 37,
    44, 45, 38, 39, 46, 47, 52, 53, 60, 61, 54, 55, 62, 63,
];

/// ETC1 intensity modifier tables (TextureCodec.etc1LUT).
const ETC1_LUT: [[i32; 4]; 8] = [
    [2, 8, -2, -8],
    [5, 17, -5, -17],
    [9, 29, -9, -29],
    [13, 42, -13, -42],
    [18, 60, -18, -60],
    [24, 80, -24, -80],
    [33, 106, -33, -106],
    [47, 183, -47, -183],
];

fn saturate_byte(v: i32) -> u8 {
    v.clamp(0, 0xff) as u8
}

/// Decode a raw PICA200 texture payload into an RGBA8 buffer
/// (`width * height * 4` bytes, row-major).
pub fn decode_rgba(
    data: &[u8],
    width: usize,
    height: usize,
    format: TextureFormat,
) -> Result<Vec<u8>> {
    // Port note: the reference loops skip the tail when width/height are not
    // multiples of 8 (or 4 for ETC1) — such textures would stay zero-filled;
    // real assets are always tiled-sized, so reject earlier and clearly.
    let unit = if matches!(format, TextureFormat::Etc1 | TextureFormat::Etc1a4) {
        4
    } else {
        8
    };
    if width == 0 || height == 0 || !width.is_multiple_of(unit) || !height.is_multiple_of(unit) {
        return Err(Error::invalid(
            "texture size",
            format!("{width}x{height} not {unit}-aligned for {}", format.name()),
        ));
    }
    let expected = width * height * format.bits_per_pixel() as usize / 8;
    if data.len() < expected {
        return Err(Error::truncated(0, expected, "texture payload"));
    }

    let mut bgra = vec![0u8; width * height * 4];
    let mut off = 0usize; // read cursor into `data`
    let mut toggle = false; // nibble state for l4/a4 (shared, as in the C#)

    match format {
        TextureFormat::Rgba8 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        // file: A,B,G,R → BGRA slots
                        bgra[o] = data[off + 1];
                        bgra[o + 1] = data[off + 2];
                        bgra[o + 2] = data[off + 3];
                        bgra[o + 3] = data[off];
                        off += 4;
                    }
                }
            }
        }
        TextureFormat::Rgb8 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        bgra[o] = data[off];
                        bgra[o + 1] = data[off + 1];
                        bgra[o + 2] = data[off + 2];
                        bgra[o + 3] = 0xff;
                        off += 3;
                    }
                }
            }
        }
        TextureFormat::Rgba5551 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        let pd = u16::from_le_bytes([data[off], data[off + 1]]);
                        let r = ((pd >> 1) & 0x1f) << 3;
                        let g = ((pd >> 6) & 0x1f) << 3;
                        let b = ((pd >> 11) & 0x1f) << 3;
                        let a = (pd & 1) * 0xff;
                        bgra[o] = (r | (r >> 5)) as u8;
                        bgra[o + 1] = (g | (g >> 5)) as u8;
                        bgra[o + 2] = (b | (b >> 5)) as u8;
                        bgra[o + 3] = a as u8;
                        off += 2;
                    }
                }
            }
        }
        TextureFormat::Rgb565 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        let pd = u16::from_le_bytes([data[off], data[off + 1]]);
                        let r = (pd & 0x1f) << 3;
                        let g = ((pd >> 5) & 0x3f) << 2;
                        let b = ((pd >> 11) & 0x1f) << 3;
                        bgra[o] = (r | (r >> 5)) as u8;
                        bgra[o + 1] = (g | (g >> 6)) as u8;
                        bgra[o + 2] = (b | (b >> 5)) as u8;
                        bgra[o + 3] = 0xff;
                        off += 2;
                    }
                }
            }
        }
        TextureFormat::Rgba4 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        let pd = u16::from_le_bytes([data[off], data[off + 1]]);
                        let r = (pd >> 4) & 0xf;
                        let g = (pd >> 8) & 0xf;
                        let b = (pd >> 12) & 0xf;
                        let a = pd & 0xf;
                        bgra[o] = (r | (r << 4)) as u8;
                        bgra[o + 1] = (g | (g << 4)) as u8;
                        bgra[o + 2] = (b | (b << 4)) as u8;
                        bgra[o + 3] = (a | (a << 4)) as u8;
                        off += 2;
                    }
                }
            }
        }
        TextureFormat::La8 | TextureFormat::Hilo8 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        bgra[o] = data[off];
                        bgra[o + 1] = data[off];
                        bgra[o + 2] = data[off];
                        bgra[o + 3] = data[off + 1];
                        off += 2;
                    }
                }
            }
        }
        TextureFormat::L8 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        bgra[o] = data[off];
                        bgra[o + 1] = data[off];
                        bgra[o + 2] = data[off];
                        bgra[o + 3] = 0xff;
                        off += 1;
                    }
                }
            }
        }
        TextureFormat::A8 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        bgra[o] = 0xff;
                        bgra[o + 1] = 0xff;
                        bgra[o + 2] = 0xff;
                        bgra[o + 3] = data[off];
                        off += 1;
                    }
                }
            }
        }
        TextureFormat::La4 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        bgra[o] = data[off] >> 4;
                        bgra[o + 1] = data[off] >> 4;
                        bgra[o + 2] = data[off] >> 4;
                        bgra[o + 3] = data[off] & 0xf;
                        off += 1;
                    }
                }
            }
        }
        TextureFormat::L4 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        let c = if toggle {
                            let c = (data[off] & 0xf0) >> 4;
                            off += 1;
                            c
                        } else {
                            data[off] & 0xf
                        };
                        toggle = !toggle;
                        let c = (c << 4) | c;
                        bgra[o] = c;
                        bgra[o + 1] = c;
                        bgra[o + 2] = c;
                        bgra[o + 3] = 0xff;
                    }
                }
            }
        }
        TextureFormat::A4 => {
            for t_y in 0..height / 8 {
                for t_x in 0..width / 8 {
                    for &t in TILE_ORDER.iter() {
                        let x = t % 8;
                        let y = (t - x) / 8;
                        let o = ((t_x * 8 + x) + (t_y * 8 + y) * width) * 4;
                        bgra[o] = 0xff;
                        bgra[o + 1] = 0xff;
                        bgra[o + 2] = 0xff;
                        let a = if toggle {
                            let a = (data[off] & 0xf0) >> 4;
                            off += 1;
                            a
                        } else {
                            data[off] & 0xf
                        };
                        toggle = !toggle;
                        bgra[o + 3] = (a << 4) | a;
                    }
                }
            }
        }
        TextureFormat::Etc1 | TextureFormat::Etc1a4 => {
            let alpha = format == TextureFormat::Etc1a4;
            let decoded = etc1_decode(data, width, height, alpha)?;
            let order = etc1_scramble(width, height);
            let mut i = 0usize;
            for t_y in 0..height / 4 {
                for t_x in 0..width / 4 {
                    let tx = order[i] % (width / 4);
                    let ty = (order[i] - tx) / (width / 4);
                    for y in 0..4 {
                        for x in 0..4 {
                            let src = ((tx * 4 + x) + (ty * 4 + y) * width) * 4;
                            let dst = ((t_x * 4 + x) + (t_y * 4 + y) * width) * 4;
                            bgra[dst..dst + 4].copy_from_slice(&decoded[src..src + 4]);
                        }
                    }
                    i += 1;
                }
            }
        }
    }

    // BGRA → RGBA (single final swap; every case above wrote BGRA).
    for px in bgra.as_chunks_mut::<4>().0 {
        px.swap(0, 2);
    }
    Ok(bgra)
}

/// Encode an RGBA8 buffer as a PNG file (via the `png` crate).
pub fn write_png(
    rgba: &[u8],
    width: u32,
    height: u32,
    w: &mut dyn std::io::Write,
) -> std::result::Result<(), png::EncodingError> {
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(rgba)
}

// ---- ETC1 (3DS variant: the 64-bit block's two halves are byte-swapped
// versus standard ETC1 — the reference reverses all 8 bytes first) ----

fn etc1_decode(input: &[u8], width: usize, height: usize, alpha: bool) -> Result<Vec<u8>> {
    let mut output = vec![0u8; width * height * 4];
    let mut offset = 0usize;

    for y in 0..height / 4 {
        for x in 0..width / 4 {
            let mut color_block = [0u8; 8];
            let mut alpha_block = [0u8; 8];
            if alpha {
                for i in 0..8 {
                    color_block[7 - i] = input[offset + 8 + i];
                    alpha_block[i] = input[offset + i];
                }
                offset += 16;
            } else {
                for i in 0..8 {
                    color_block[7 - i] = input[offset + i];
                    alpha_block[i] = 0xff;
                }
                offset += 8;
            }

            let color_block = etc1_decode_block(&color_block);

            let mut toggle = false;
            let mut alpha_offset = 0usize;
            for t_x in 0..4 {
                for t_y in 0..4 {
                    let out = (x * 4 + t_x + (y * 4 + t_y) * width) * 4;
                    let blk = (t_x + t_y * 4) * 4;
                    color_block[blk..blk + 3]
                        .iter()
                        .enumerate()
                        .for_each(|(i, &b)| {
                            output[out + i] = b;
                        });
                    let a = if toggle {
                        let a = (alpha_block[alpha_offset] & 0xf0) >> 4;
                        alpha_offset += 1;
                        a
                    } else {
                        alpha_block[alpha_offset] & 0xf
                    };
                    output[out + 3] = (a << 4) | a;
                    toggle = !toggle;
                }
            }
        }
    }
    Ok(output)
}

fn etc1_decode_block(data: &[u8; 8]) -> [u8; 64] {
    let block_top = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let block_bottom = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);

    let flip = block_top & 0x100_0000 > 0;
    let table1 = ((block_top >> 29) & 7) as usize;
    let table2 = ((block_top >> 26) & 7) as usize;

    // (r1, g1, b1, r2, g2, b2), expanded to 8 bits per channel.
    let rgb = if block_top & 0x200_0000 > 0 {
        // Difference mode: base1 5-bit, base2 = base1>>3 + signed 3-bit delta.
        // The C# sign-extends via (sbyte)(x << 5) >> 5.
        let sx = |v: u32| ((v << 5) as u8 as i8 as i32) >> 5;
        let r1 = block_top & 0xf8;
        let g1 = (block_top & 0xf800) >> 8;
        let b1 = (block_top & 0x00f8_0000) >> 16;
        let exp5 = |v: u32| v | (v >> 5);
        let exp8 = |v: u32| (v << 3) | (v >> 2);
        let r2 = exp8((r1 >> 3).wrapping_add(sx(block_top & 7) as u32));
        let g2 = exp8((g1 >> 3).wrapping_add(sx((block_top & 0x700) >> 3) as u32));
        let b2 = exp8((b1 >> 3).wrapping_add(sx((block_top & 0x7_0000) >> 11) as u32));
        (exp5(r1), exp5(g1), exp5(b1), r2, g2, b2)
    } else {
        // Individual mode: two 4-bit colors, expanded with nibble mirror.
        let r1 = block_top & 0xf0;
        let g1 = (block_top & 0xf000) >> 8;
        let b1 = (block_top & 0xf0_0000) >> 16;
        let r2 = (block_top & 0xf) << 4;
        let g2 = (block_top & 0xf00) >> 4;
        let b2 = (block_top & 0xf_0000) >> 12;
        let exp = |v: u32| v | (v >> 4);
        (exp(r1), exp(g1), exp(b1), exp(r2), exp(g2), exp(b2))
    };

    etc1_emit(rgb, flip, block_bottom, table1, table2)
}

fn etc1_emit(
    rgb: (u32, u32, u32, u32, u32, u32),
    flip: bool,
    block: u32,
    table1: usize,
    table2: usize,
) -> [u8; 64] {
    let (r1, g1, b1, r2, g2, b2) = rgb;
    let mut output = [0u8; 64];
    let put = |output: &mut [u8; 64],
               slot: usize,
               (r, g, b): (u32, u32, u32),
               x: usize,
               y: usize,
               block: u32,
               table: usize| {
        // TextureCodec writes Color B,G,R into slots 0..2 (BGRA buffer).
        let (r, g, b) = etc1_pixel(r, g, b, x, y, block, table);
        output[slot] = b as u8;
        output[slot + 1] = g as u8;
        output[slot + 2] = r as u8;
    };
    if !flip {
        for y in 0..4usize {
            for x in 0..2usize {
                put(
                    &mut output,
                    (y * 4 + x) * 4,
                    (r1, g1, b1),
                    x,
                    y,
                    block,
                    table1,
                );
                put(
                    &mut output,
                    (y * 4 + x + 2) * 4,
                    (r2, g2, b2),
                    x + 2,
                    y,
                    block,
                    table2,
                );
            }
        }
    } else {
        for y in 0..2usize {
            for x in 0..4usize {
                put(
                    &mut output,
                    (y * 4 + x) * 4,
                    (r1, g1, b1),
                    x,
                    y,
                    block,
                    table1,
                );
                put(
                    &mut output,
                    ((y + 2) * 4 + x) * 4,
                    (r2, g2, b2),
                    x,
                    y + 2,
                    block,
                    table2,
                );
            }
        }
    }
    output
}

fn etc1_pixel(
    r: u32,
    g: u32,
    b: u32,
    x: usize,
    y: usize,
    block: u32,
    table: usize,
) -> (u32, u32, u32) {
    let index = x * 4 + y;
    let msb = block << 1;

    let pixel = if index < 8 {
        ETC1_LUT[table][(((block >> (index + 24)) & 1) + ((msb >> (index + 8)) & 2)) as usize]
    } else {
        ETC1_LUT[table][(((block >> (index + 8)) & 1) + ((msb >> (index - 8)) & 2)) as usize]
    };

    (
        saturate_byte(r as i32 + pixel) as u32,
        saturate_byte(g as i32 + pixel) as u32,
        saturate_byte(b as i32 + pixel) as u32,
    )
}

/// 3DS ETC1 block scramble order (TextureCodec.etc1Scramble, verbatim).
fn etc1_scramble(width: usize, height: usize) -> Vec<usize> {
    let tiles = (width / 4) * (height / 4);
    let mut tile_scramble = vec![0usize; tiles];
    let mut base_accumulator = 0i32;
    let mut row_accumulator = 0i32;
    let mut base_number = 0i32;
    let mut row_number = 0i32;

    for (tile, slot) in tile_scramble.iter_mut().enumerate() {
        if tile % (width / 4) == 0 && tile > 0 {
            if row_accumulator < 1 {
                row_accumulator += 1;
                row_number += 2;
                base_number = row_number;
            } else {
                row_accumulator = 0;
                base_number -= 2;
                row_number = base_number;
            }
        }

        *slot = if base_number < 0 {
            0
        } else {
            base_number as usize
        };

        if base_accumulator < 1 {
            base_accumulator += 1;
            base_number += 1;
        } else {
            base_accumulator = 0;
            base_number += 3;
        }
    }

    tile_scramble
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l8_decodes_morton_tiled() {
        // 8x8 l8 texture: pixel values 0..64. Under the Morton tile order,
        // file byte i lands at (x, y) of TILE_ORDER[i].
        let data: Vec<u8> = (0..64u8).collect();
        let rgba = decode_rgba(&data, 8, 8, TextureFormat::L8).unwrap();
        for (i, &t) in TILE_ORDER.iter().enumerate() {
            let x = t % 8;
            let y = t / 8;
            let o = (x + y * 8) * 4;
            assert_eq!(rgba[o], i as u8, "grayscale at ({x},{y})");
            assert_eq!(rgba[o + 3], 0xff, "alpha");
        }
    }

    #[test]
    fn rgba8_swaps_channels() {
        // 8x8 rgba8: file texel = A,B,G,R (verified against Ohana writes).
        let mut data = vec![0u8; 8 * 8 * 4];
        for i in 0..64 {
            data[i * 4..i * 4 + 4].copy_from_slice(&[0x11, 0x22, 0x33, 0x44]); // A B G R
        }
        let rgba = decode_rgba(&data, 8, 8, TextureFormat::Rgba8).unwrap();
        // every texel → R=0x44 G=0x33 B=0x22 A=0x11
        for o in (0..rgba.len()).step_by(4) {
            assert_eq!(&rgba[o..o + 4], &[0x44, 0x33, 0x22, 0x11]);
        }
    }

    #[test]
    fn etc1_block_flat_color() {
        // All-zero block: individual mode, base 0, table 0 → every pixel
        // gets modifier +2 (LUT row [2, 8, -2, -8], index 0).
        let block = [0u8; 8];
        let decoded = etc1_decode_block(&block);
        assert!(
            decoded
                .chunks_exact(4)
                .all(|px| px[0] == 2 && px[1] == 2 && px[2] == 2),
            "flat +2: {decoded:?}"
        );

        // Individual mode: data[0..3] high nibbles = sub-block-1 RGB
        // (white), low nibbles = sub-block-2 RGB (black); flip = false →
        // left half white, right half near-black. Slots are B,G,R (BGRA).
        let mut block = [0u8; 8];
        block[0] = 0xf0;
        block[1] = 0xf0;
        block[2] = 0xf0;
        let decoded = etc1_decode_block(&block);
        for y in 0..4 {
            for x in 0..4 {
                let o = (y * 4 + x) * 4;
                let expect = if x < 2 { 255u8 } else { 2u8 };
                assert_eq!(decoded[o], expect, "B at ({x},{y})");
                assert_eq!(decoded[o + 1], expect, "G at ({x},{y})");
                assert_eq!(decoded[o + 2], expect, "R at ({x},{y})");
            }
        }
    }

    #[test]
    fn rejects_unaligned_size() {
        assert!(decode_rgba(&[0; 16], 7, 8, TextureFormat::L8).is_err());
        assert!(decode_rgba(&[0; 16], 8, 8, TextureFormat::Rgba8).is_err()); // too short
    }

    #[test]
    fn etc1_scramble_starts_deterministically() {
        // First tiles follow the reference pattern: 0,1, (wrap) 2,3, 5,6, …
        let s = etc1_scramble(16, 16);
        assert_eq!(s[..4].to_vec(), vec![0, 1, 4, 5]);
        // covers all tiles exactly once
        let mut seen = vec![false; s.len()];
        for &v in &s {
            seen[v] = true;
        }
        assert!(seen.iter().all(|&b| b));
    }
}
