//! Golden-file tests against real XY ROM assets.
//!
//! Skipped automatically when the assets are absent, so `cargo test` works on
//! machines without the extracted ROM data.

use std::path::PathBuf;

use xy_audio::bcwav::{Bcwav, Codec};

/// Data root for golden assets: `XY_DATA_ROOT` override, else the repo's
/// `.Data/` (asset layout: `.Data/Raw_Extracted/` raw dumps,
/// `.Data/Source_Extracted/` converted outputs).
fn data_root() -> PathBuf {
    if let Ok(root) = std::env::var("XY_DATA_ROOT") {
        return PathBuf::from(root);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data")
}

fn asset(rel: &str) -> Option<PathBuf> {
    let p = data_root().join(rel);
    p.exists().then_some(p)
}

/// Pokémon X banner jingle: DSP-ADPCM, converted previously by the validated
/// Python reference decoder (vgmstream-equivalent output).
#[test]
fn banner_jingle_decodes() {
    let Some(path) = asset("Source_Extracted/banner_jingle.bcwav") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let wav = Bcwav::parse(&bytes).expect("parse should succeed");

    assert_eq!(wav.codec, Codec::DspAdpcm);
    assert_eq!(wav.sample_rate, 32728); // 3DS DSP native rate
    assert_eq!(wav.channels, 2);
    assert_eq!(wav.sample_count, 95372);
    assert!(!wav.looped);

    let chans = wav.decode().expect("decode should succeed");
    assert_eq!(chans.len(), 2);
    assert_eq!(chans[0].len(), 95372);

    // First samples of frame 0, hand-verified against the C reference math.
    assert_eq!(&chans[0][..8], &[0, 0, 0, 1, 6, 13, 17, 16]);
    assert_eq!(&chans[1][..8], &[0, 0, 0, -1, -1, 0, 1, 1]);

    // Not silence: real audio has substantial peak amplitude.
    let peak = chans
        .iter()
        .flat_map(|c| c.iter())
        .map(|&s| s.unsigned_abs())
        .max()
        .unwrap();
    assert!(
        peak > 10_000,
        "decoded peak {peak} looks like silence/noise"
    );
}

/// Full byte-parity against the reference WAV conversion (Python port of the
/// same vgmstream algorithm) — guards the entire pipeline end to end.
#[test]
fn banner_jingle_matches_reference_wav() {
    let Some(src) = asset("Source_Extracted/banner_jingle.bcwav") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let Some(reference) = asset("Source_Extracted/audio/banner_jingle.wav") else {
        eprintln!("skipping: reference WAV not found");
        return;
    };

    let bytes = std::fs::read(&src).unwrap();
    let wav = Bcwav::parse(&bytes).unwrap();
    let pcm = wav.decode_pcm16().unwrap();

    let reference = std::fs::read(&reference).unwrap();
    // Canonical 44-byte RIFF header; the rest is interleaved PCM16 data.
    assert_eq!(&reference[..4], b"RIFF");
    assert_eq!(
        pcm.len(),
        reference.len() - 44,
        "decoded PCM length should match reference data chunk"
    );
    assert_eq!(
        pcm[..],
        reference[44..],
        "PCM must be byte-identical to the reference"
    );
}

/// The other banner CWAV (extracted from ExeFS banner.bin alongside the CBMD).
#[test]
fn banner_bcwav_from_exefs_parses() {
    let Some(path) = asset("Source_Extracted/banner/banner.bcwav") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let wav = Bcwav::parse(&bytes).expect("parse should succeed");
    assert_eq!(wav.codec, Codec::DspAdpcm);
    assert_eq!(wav.sample_rate, 32728);
    assert_eq!(wav.channels, 2);
    assert_eq!(wav.sample_count, 95372);
    // Same jingle as banner_jingle.bcwav — decode must also succeed.
    let chans = wav.decode().unwrap();
    assert_eq!(chans.len(), 2);
}
