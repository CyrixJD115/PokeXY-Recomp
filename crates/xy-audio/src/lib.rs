//! `xy-audio` — sound container parsing.
//!
//! * [`bcwav`] — banner/audio CWAV containers (PCM8/PCM16 + DSP-ADPCM → WAV)
//! * [`wav`]   — canonical 44-byte-header RIFF/WAV writer
//!
//! Scope note: XY's streamed BGM is **plain ADTS AAC** (`sound/*.aac`) —
//! that belongs to ffmpeg, not to this crate. The 3DS BCSTM stream format is
//! not used by XY and is out of scope. BCSAR (`sound/xy_sound.bcsar`, SEQ/
//! BANK/WAV collections) is a planned parser that will reuse [`bcwav`] for
//! its CWAV entries (PLAN.md Phase 3).
pub mod bcwav;
pub mod wav;
