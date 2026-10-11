//! Minimal RIFF/WAV writer (moved from the old shared `util` module; only
//! the audio crate needs it).

use std::io::Write;
use xy_core::error::Result;

/// Write a canonical 44-byte-header PCM WAV file.
pub fn write_wav(
    w: &mut impl Write,
    channels: u16,
    sample_rate: u32,
    bits: u16,
    pcm: &[u8],
) -> Result<()> {
    let byte_rate: u32 = sample_rate * channels as u32 * (bits / 8) as u32;
    let block_align: u16 = channels * (bits / 8);
    let data_len = pcm.len() as u32;

    w.write_all(b"RIFF")?;
    w.write_all(&(36 + data_len).to_le_bytes())?;
    w.write_all(b"WAVE")?;
    w.write_all(b"fmt ")?;
    w.write_all(&16u32.to_le_bytes())?; // fmt chunk size
    w.write_all(&1u16.to_le_bytes())?; // PCM format tag
    w.write_all(&channels.to_le_bytes())?;
    w.write_all(&sample_rate.to_le_bytes())?;
    w.write_all(&byte_rate.to_le_bytes())?;
    w.write_all(&block_align.to_le_bytes())?;
    w.write_all(&bits.to_le_bytes())?;
    w.write_all(b"data")?;
    w.write_all(&data_len.to_le_bytes())?;
    w.write_all(pcm)?;
    Ok(())
}
