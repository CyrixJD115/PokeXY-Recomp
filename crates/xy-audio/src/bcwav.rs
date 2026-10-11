//! BCWAV (CWAV) — audio container used by banners and CTR sound archives.
//!
//! Two layout variants exist in the wild; [`Bcwav::parse`] auto-detects them.
//!
//! # Variant A — reference-table header (standard CWAV)
//!
//! ```text
//! 0x00  "CWAV", u16 BOM, u16 version, u32 file size, u16 block count, u16 pad
//! 0x10  reference table: u32 info_off, u32 info_size, u32 data_off, u32 data_size
//!         (offsets relative to 0x10)
//! INFO block @ 0x10 + info_off:
//!   +0x08 u8 channels, +0x09 u8 codec, +0x0A u8 looped
//!   +0x0C u32 sample rate, +0x10 u32 loop start, +0x14 u32 loop end
//!   +0x18 u32 sample count
//! DATA block @ 0x10 + data_off:
//!   +0x08 u32 offset (relative to DATA start) of first channel's samples
//! ```
//!
//! # Variant B — chunked header (bannertool / GYTB banner files, e.g.
//! `banner.cbmd`-extracted `banner.bcwav`)
//!
//! ```text
//! 0x00  "CWAV", u16 BOM, u16 struct_len (0x40), u32 unknown, u32 file size,
//!       u32 chunk count (2)
//! 0x14  { u32 flags (0x7000), u32 info_offset, u32 info_len }   (absolute)
//! 0x20  { u32 flags (0x7000/0x7001), u32 data_offset, u32 data_len }
//! 0x2C  0x14 reserved bytes
//! INFO chunk @ info_offset:
//!   +0x08 u32 codec (2 = DSP-ADPCM), +0x0C u32 sample rate (32728 for DSP:
//!   the 3DS DSP native rate), +0x10 u32 loop start, +0x14 u32 sample count,
//!   +0x18 u32 loop end, +0x1C u32 channel count
//!   +0x20 per channel: { u32 0x7100, u32 ptr }
//! ```
//!
//! The per-channel `ptr` locates a 0x14-byte block at `info_off + 0x1C + ptr`
//! (all channel blocks first): `{ u32 0x1F00, u32 data_offset, u32 0x0300, .. }`
//! where `data_offset` is this channel's sample start, relative to the DATA
//! sample base (`data_offset + 8`). Channels are stored **sequentially**, not
//! interleaved. After all channel blocks, channel *i* has a 0x30-byte DSP
//! block at `info_off + 0x20 + ch*8 + ch*0x14 + i*0x30`:
//! 16 × i16 ADPCM coefficients, then 2 × i16 context, 2 × i16 loop context.
//!
//! # DSP-ADPCM decoding (codec 2)
//!
//! 8-byte frames → 14 samples. Per frame: `scale = 1 << (hdr & 0xF)`,
//! `coef_index = (hdr >> 4) & 0xF` (clamped to 7), then 14 signed nibbles
//! (high nibble first) with
//! `s = ((nib*scale) << 11 + 1024 + c1*h1 + c2*h2) >> 11`, clamped to i16.
//! Port of vgmstream `src/coding/ngc_dsp_decoder.c` — byte-exact against the
//! reference decoder on the XY banner jingle (see `tests/`).

use crate::wav::write_wav;
use std::io::Write;
use xy_core::error::{Error, Result};
use xy_core::util::{u16_le, u32_le, u8_at};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    Pcm8,
    Pcm16,
    DspAdpcm,
}

impl Codec {
    pub fn from_id(id: u8) -> Result<Self> {
        Ok(match id {
            0 => Codec::Pcm8,
            1 => Codec::Pcm16,
            2 => Codec::DspAdpcm,
            other => return Err(Error::invalid("bcwav codec", format!("id {other}"))),
        })
    }

    pub fn bits(self) -> u16 {
        match self {
            Codec::Pcm8 => 8,
            Codec::Pcm16 => 16,
            Codec::DspAdpcm => 4,
        }
    }
}

/// One channel's raw sample stream (sequential, not interleaved) plus its
/// DSP coefficients when present.
#[derive(Debug)]
struct ChanStream<'a> {
    samples: &'a [u8],
    /// Only populated for the chunked variant (DSP-ADPCM).
    coefs: Option<[i16; 16]>,
}

#[derive(Debug)]
pub struct Bcwav<'a> {
    pub channels: u8,
    pub codec: Codec,
    pub looped: bool,
    pub sample_rate: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub sample_count: u32,
    data: &'a [u8],
    /// Standard variant: interleaved PCM starts here.
    pcm_start: usize,
    /// Chunked variant: per-channel sequential streams (with DSP coefs).
    chan: Option<Vec<ChanStream<'a>>>,
}

impl<'a> Bcwav<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < 0x20 {
            return Err(Error::truncated(0, 0x20, "bcwav header"));
        }
        if &data[0..4] != b"CWAV" {
            return Err(Error::magic(0, "'CWAV'", &data[0..4]));
        }
        // Variant A: INFO at 0x10 + u32@0x10 (offset relative to 0x10).
        let std_info = 0x10 + u32_le(data, 0x10)? as usize;
        if data.get(std_info..std_info + 4) == Some(b"INFO") {
            return Self::parse_standard(data, std_info);
        }
        // Variant B: chunk table at 0x14 with absolute offsets.
        let info_off = u32_le(data, 0x18)? as usize;
        if data.get(info_off..info_off + 4) == Some(b"INFO") {
            return Self::parse_chunked(data, info_off);
        }
        Err(Error::invalid(
            "bcwav layout",
            "neither reference-table nor chunked INFO offset resolves to an INFO chunk",
        ))
    }

    /// Variant A: standard CWAV with a reference table at 0x10.
    fn parse_standard(data: &'a [u8], info_off: usize) -> Result<Self> {
        let data_off = 0x10 + u32_le(data, 0x18)? as usize;
        check_sig(data, data_off, b"DATA")?;

        let channels = u8_at(data, info_off + 8)?;
        let codec = Codec::from_id(u8_at(data, info_off + 9)?)?;
        let looped = u8_at(data, info_off + 10)? != 0;
        let sample_rate = u32_le(data, info_off + 12)?;
        let loop_start = u32_le(data, info_off + 16)?;
        let loop_end = u32_le(data, info_off + 20)?;
        let sample_count = u32_le(data, info_off + 24)?;

        let chan_rel = u32_le(data, data_off + 8)? as usize;
        let pcm_start = data_off + chan_rel;

        Ok(Self {
            channels,
            codec,
            looped,
            sample_rate,
            loop_start,
            loop_end,
            sample_count,
            data,
            pcm_start,
            chan: None,
        })
    }

    /// Variant B: chunked bannertool/GYTB banner layout.
    fn parse_chunked(data: &'a [u8], info_off: usize) -> Result<Self> {
        // Sanity: header file-size field must match the real size.
        let file_size = u32_le(data, 0x0C)?;
        if file_size as usize != data.len() {
            return Err(Error::invalid(
                "bcwav file size",
                format!("header says {file_size:#x}, file is {:#x}", data.len()),
            ));
        }
        let data_off = u32_le(data, 0x24)? as usize;
        let data_len = u32_le(data, 0x28)? as usize;
        check_sig(data, data_off, b"DATA")?;

        let codec = Codec::from_id(u8_at(data, info_off + 8)?)?;
        let sample_rate = u32_le(data, info_off + 0x0C)?;
        let loop_start = u32_le(data, info_off + 0x10)?;
        let sample_count = u32_le(data, info_off + 0x14)?;
        let loop_end = u32_le(data, info_off + 0x18)?;
        let channels = u32_le(data, info_off + 0x1C)?;
        if !(1..=16).contains(&channels) {
            return Err(Error::invalid("bcwav channels", format!("{channels}")));
        }
        let ch = channels as usize;

        // Pointer entries at INFO+0x20; channel blocks at INFO+0x1C+ptr;
        // DSP blocks (coefs + context) after all channel blocks, 0x30 each.
        let ptr_base = info_off + 0x20;
        let coef_base = ptr_base + ch * 8 + ch * 0x14;
        let samples_base = data_off + 8;
        let mut chan = Vec::with_capacity(ch);
        let mut starts = Vec::with_capacity(ch + 1);
        for i in 0..ch {
            let ptr = u32_le(data, ptr_base + i * 8 + 4)? as usize;
            let block = info_off + 0x1C + ptr;
            let stream_off = u32_le(data, block + 4)? as usize;
            starts.push(samples_base + stream_off);
        }
        starts.push(data_off + data_len);
        for i in 0..ch {
            let cb = coef_base + i * 0x30;
            let mut coefs = [0i16; 16];
            for (j, c) in coefs.iter_mut().enumerate() {
                *c = u16_le(data, cb + j * 2)? as i16;
            }
            let stream = data.get(starts[i]..starts[i + 1]).ok_or_else(|| {
                Error::truncated(
                    starts[i],
                    starts[i + 1] - starts[i],
                    "bcwav channel samples",
                )
            })?;
            chan.push(ChanStream {
                samples: stream,
                coefs: Some(coefs),
            });
        }

        Ok(Self {
            channels: channels as u8,
            codec,
            looped: loop_end > 0,
            sample_rate,
            loop_start,
            loop_end,
            sample_count,
            data,
            pcm_start: samples_base,
            chan: Some(chan),
        })
    }

    /// Raw sample bytes (interleaved for the standard variant; first-channel
    /// start through last-channel end for the chunked variant).
    pub fn pcm(&self) -> Result<&'a [u8]> {
        match &self.chan {
            None => {
                let frame = self.channels as usize * (self.codec.bits() / 8) as usize;
                let len = self.sample_count as usize * frame;
                self.data
                    .get(self.pcm_start..self.pcm_start.saturating_add(len))
                    .ok_or_else(|| Error::truncated(self.pcm_start, len, "bcwav samples"))
            }
            Some(chan) => {
                let base = self.data.as_ptr() as usize;
                let first = chan
                    .first()
                    .ok_or_else(|| Error::invalid("bcwav", "no channels"))?;
                let last = chan.last().expect("first() succeeded implies non-empty");
                let start = first.samples.as_ptr() as usize - base;
                let end = last.samples.as_ptr() as usize + last.samples.len() - base;
                self.data
                    .get(start..end)
                    .ok_or_else(|| Error::truncated(start, end - start, "bcwav samples"))
            }
        }
    }

    /// Decode to per-channel PCM16, trimmed to the declared sample count.
    pub fn decode(&self) -> Result<Vec<Vec<i16>>> {
        let mut out: Vec<Vec<i16>> = match &self.chan {
            // Standard variant: interleaved PCM in `pcm()`.
            None => {
                let pcm = self.pcm()?;
                match self.codec {
                    Codec::Pcm8 => {
                        let mut chans = vec![
                            Vec::with_capacity(pcm.len() / self.channels as usize);
                            self.channels as usize
                        ];
                        for (i, &b) in pcm.iter().enumerate() {
                            chans[i % self.channels as usize].push(((b as i16) - 0x80) << 8);
                        }
                        chans
                    }
                    Codec::Pcm16 => {
                        let smp: Vec<i16> = pcm
                            .chunks_exact(2)
                            .map(|c| i16::from_le_bytes([c[0], c[1]]))
                            .collect();
                        let mut chans = vec![
                            Vec::with_capacity(smp.len() / self.channels as usize);
                            self.channels as usize
                        ];
                        for (i, &s) in smp.iter().enumerate() {
                            chans[i % self.channels as usize].push(s);
                        }
                        chans
                    }
                    Codec::DspAdpcm => return Err(Error::unsupported(
                        "bcwav DSP-ADPCM",
                        "coefficient layout for this (reference-table) variant is not mapped yet",
                    )),
                }
            }
            // Chunked variant: per-channel sequential streams.
            Some(chan) => chan
                .iter()
                .map(|c| match self.codec {
                    Codec::DspAdpcm => Ok(decode_dsp(
                        c.samples,
                        c.coefs
                            .as_ref()
                            .expect("chunked DSP channels carry coefficients"),
                    )),
                    Codec::Pcm8 => Ok(c
                        .samples
                        .iter()
                        .map(|&b| ((b as i16) - 0x80) << 8)
                        .collect()),
                    Codec::Pcm16 => Ok(c
                        .samples
                        .chunks_exact(2)
                        .map(|p| i16::from_le_bytes([p[0], p[1]]))
                        .collect()),
                })
                .collect::<Result<_>>()?,
        };
        let n = (self.sample_count as usize).min(out.iter().map(|c| c.len()).min().unwrap_or(0));
        for c in &mut out {
            c.truncate(n);
        }
        Ok(out)
    }

    /// Decode to interleaved PCM16 little-endian bytes (WAV-ready).
    pub fn decode_pcm16(&self) -> Result<Vec<u8>> {
        let chans = self.decode()?;
        let mut buf = Vec::with_capacity(chans.first().map_or(0, |c| c.len()) * chans.len() * 2);
        for i in 0..chans.first().map_or(0, |c| c.len()) {
            for ch in &chans {
                buf.extend_from_slice(&ch[i].to_le_bytes());
            }
        }
        Ok(buf)
    }

    pub fn write_wav(&self, w: &mut impl Write) -> Result<()> {
        match self.chan {
            // Chunked variant (or any codec): decode to PCM16 and write that.
            Some(_) => {
                let pcm = self.decode_pcm16()?;
                write_wav(w, self.channels as u16, self.sample_rate, 16, &pcm)
            }
            // Standard variant, PCM codecs: write raw bytes as before.
            None => match self.codec {
                Codec::DspAdpcm => Err(Error::unsupported(
                    "bcwav DSP-ADPCM",
                    "coefficient layout for this (reference-table) variant is not mapped yet",
                )),
                c => write_wav(
                    w,
                    self.channels as u16,
                    self.sample_rate,
                    c.bits(),
                    self.pcm()?,
                ),
            },
        }
    }
}

fn check_sig(data: &[u8], off: usize, expected: &[u8; 4]) -> Result<()> {
    match data.get(off..off + 4) {
        Some(sig) if sig == expected => Ok(()),
        Some(found) => Err(Error::magic(off, "'DATA'", found)),
        None => Err(Error::truncated(off, 4, "bcwav chunk signature")),
    }
}

// ---------------------------------------------------------------------------
// DSP-ADPCM decoder — port of vgmstream `ngc_dsp_decoder.c` (decode_ngc_dsp).
// i64 intermediates: extreme coefficients would overflow i32 (the C reference
// only avoids it because real-world coefficients are small); results are
// identical for all in-range values.
// ---------------------------------------------------------------------------

fn high_nibble_signed(b: u8) -> i64 {
    let n = ((b >> 4) & 0xF) as i64;
    if n >= 8 {
        n - 16
    } else {
        n
    }
}

fn low_nibble_signed(b: u8) -> i64 {
    let n = (b & 0xF) as i64;
    if n >= 8 {
        n - 16
    } else {
        n
    }
}

pub(crate) fn decode_dsp(data: &[u8], coefs: &[i16; 16]) -> Vec<i16> {
    let mut out = Vec::with_capacity(data.len() / 8 * 14);
    let (mut hist1, mut hist2) = (0i64, 0i64);
    for frame in data.chunks_exact(8) {
        let header = frame[0];
        let scale = 1i64 << (header & 0xF);
        let mut coef_index = ((header >> 4) & 0xF) as usize;
        if coef_index > 7 {
            // Possibly not clamped in the original decoder, but harmless.
            coef_index = 7;
        }
        let (c1, c2) = (
            coefs[coef_index * 2] as i64,
            coefs[coef_index * 2 + 1] as i64,
        );
        for i in 0..14usize {
            let nibbles = frame[1 + i / 2];
            let nib = if i & 1 == 1 {
                low_nibble_signed(nibbles)
            } else {
                high_nibble_signed(nibbles)
            };
            let s = ((nib * scale) << 11) + 1024 + c1 * hist1 + c2 * hist2;
            let s = (s >> 11).clamp(i16::MIN as i64, i16::MAX as i64);
            out.push(s as i16);
            hist2 = hist1;
            hist1 = s;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nibbles_are_signed() {
        assert_eq!(high_nibble_signed(0x1F), 1);
        assert_eq!(low_nibble_signed(0x1F), -1);
        assert_eq!(high_nibble_signed(0x8F), -8);
        assert_eq!(low_nibble_signed(0x07), 7);
    }

    #[test]
    fn zero_input_decodes_to_silence() {
        let coefs = [0i16; 16];
        let frame = [0u8; 8]; // scale = 1<<0, coef 0, all zero nibbles
        let out = decode_dsp(&frame, &coefs);
        assert_eq!(out.len(), 14);
        assert!(out.iter().all(|&s| s == 0));
    }

    #[test]
    fn coef_index_is_clamped_to_seven() {
        // header 0xF0 => coef index 15 (clamped to 7), scale 1<<0.
        let mut coefs = [0i16; 16];
        coefs[14] = 100; // coef7 c1
        let frame = [0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert!(decode_dsp(&frame, &coefs).iter().all(|&s| s == 0));
    }

    /// Golden frame hand-computed from the C reference algorithm
    /// (frame 0 of the XY banner jingle, left channel).
    #[test]
    fn matches_reference_sample_math() {
        // header 0x10: scale = 1<<0, coef index 1 => c1 = coefs[2], c2 = coefs[3].
        let mut coefs = [0i16; 16];
        coefs[2] = 2859; // 0x0B2B
        coefs[3] = -1374; // 0xFAA2
        let frame = [0x10, 0x00, 0x01, 0x55, 0x31, 0x0D, 0xAF, 0xFE];
        let out = decode_dsp(&frame, &coefs);
        assert_eq!(&out[..8], &[0, 0, 0, 1, 6, 13, 17, 16]);
    }

    #[test]
    fn output_clamps_to_i16_range() {
        let mut coefs = [0i16; 16];
        coefs[0] = 32767;
        coefs[1] = 32767;
        let frame = [0x0F, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88]; // scale 32768, nib -8
        assert!(decode_dsp(&frame, &coefs).iter().all(|&s| s == i16::MIN));
    }
}
