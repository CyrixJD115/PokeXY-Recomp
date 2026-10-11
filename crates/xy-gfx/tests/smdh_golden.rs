//! Golden test: SMDH icon decode against the real ExeFS icon.
//!
//! The icon is a dithered dark-navy image and *looks* like noise at 48×48 —
//! a 2026-10-05 session almost shipped a "fix" for it. These guards pin the
//! actual content: pixel (0,0) is white (the 0xFFFF cluster found in the
//! raw RGB565), and adjacent-pixel correlation must be well below random
//! (i.e., a real image, not scrambled bytes). Skips when assets are absent.

use std::path::PathBuf;

use xy_gfx::smdh::Smdh;

fn data_root() -> PathBuf {
    std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Data"))
}

fn lum(rgb: &[u8], i: usize) -> i32 {
    let (r, g, b) = (
        rgb[i * 3] as i32,
        rgb[i * 3 + 1] as i32,
        rgb[i * 3 + 2] as i32,
    );
    (r * 299 + g * 587 + b * 114) / 1000
}

#[test]
fn icon_decode_is_structured_not_noise() {
    let path = data_root().join("3ds_extracted/exefs/icon.bin");
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let smdh = Smdh::parse(&bytes).expect("parse SMDH");

    // Title sanity: the dump's own SMDH identifies it.
    let titles = smdh.titles();
    assert!(titles.iter().any(|t| t.long_name.contains("Pokémon X")));

    let rgb = smdh.icon_rgb();
    assert_eq!(rgb.len(), 48 * 48 * 3);

    // Pixel (0,0) is white — the 0xFFFF RGB565 cluster starts at index 0.
    // (Shift-based 5/6/5→8-bit expansion maxes out at 248/252/248.)
    assert_eq!((rgb[0], rgb[1], rgb[2]), (248, 252, 248), "corner pixel");

    // Not-noise guard: adjacent-pixel differences must be much smaller than
    // random-pair differences (a real image correlates spatially).
    let (w, h) = (48usize, 48usize);
    let mut adjacent = Vec::new();
    for y in 0..h {
        for x in 0..w - 1 {
            adjacent.push((lum(&rgb, y * w + x) - lum(&rgb, y * w + x + 1)).abs());
        }
    }
    let mut rng_samples: Vec<i32> = Vec::new();
    let mut seed = 0x12345678u64;
    for _ in 0..1000 {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let a = (seed >> 33) as usize % (w * h);
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let b = (seed >> 33) as usize % (w * h);
        rng_samples.push((lum(&rgb, a) - lum(&rgb, b)).abs());
    }
    let mean = |v: &[i32]| v.iter().sum::<i32>() as f64 / v.len() as f64;
    let (adj_mean, rnd_mean) = (mean(&adjacent), mean(&rng_samples));
    assert!(
        adj_mean < rnd_mean * 0.8,
        "adjacent diff {adj_mean:.1} should be well under random {rnd_mean:.1}"
    );

    // Upscale sanity: 8× keeps corner white and dimensions.
    let scaled = smdh.icon_rgb_scaled(8);
    assert_eq!(scaled.len(), (48 * 8) * (48 * 8) * 3);
    assert_eq!((scaled[0], scaled[1], scaled[2]), (248, 252, 248));
}
