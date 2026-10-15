//! Golden test: SMDH icon decode against the real ExeFS icon.
//!
//! History worth keeping: the first version of this test pinned a *wrong*
//! decode — the parser read the large icon linearly out of the small-icon
//! slot at 0x2040, producing noise, and a 2026-10-05 session "validated"
//! that noise against a same-mistake reference image (a later session even
//! almost shipped the fix and was talked out of it by these guards). The
//! real layout (two Morton-tiled RGB565 icons at 0x2040/0x24C0) was settled
//! 2026-10-11 by porting Azahar's `SMDH::GetIcon`; the pins below come from
//! that independent Python port. Skips when assets are absent.

use std::path::PathBuf;

use xy_gfx::smdh::Smdh;

fn data_root() -> PathBuf {
    std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"))
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
fn icon_decode_matches_azahar_reference() {
    let path = data_root().join("Raw_Extracted/exefs/icon.bin");
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

    // Pins from the independent Python port of Azahar's GetIcon: the corners
    // sit on the white rainbow-border ring, the interior hits the dark X art.
    let px = |x: usize, y: usize| {
        (
            rgb[y * 48 * 3 + x * 3],
            rgb[y * 48 * 3 + x * 3 + 1],
            rgb[y * 48 * 3 + x * 3 + 2],
        )
    };
    assert_eq!(px(0, 0), (248, 252, 248), "top-left corner (border)");
    assert_eq!(px(0, 47), (248, 252, 248), "bottom-left corner (border)");
    assert_eq!(px(24, 24), (40, 16, 16), "center (dark X art)");
    assert_eq!(px(10, 30), (16, 60, 96), "lower-left (navy art)");
    assert_eq!(px(40, 8), (0, 56, 56), "upper-right (navy art)");

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

    // Small icon decodes too (24×24, white border corner).
    let small = smdh.small_icon_rgb();
    assert_eq!(small.len(), 24 * 24 * 3);
    assert_eq!((small[0], small[1], small[2]), (248, 252, 248));

    // Upscale sanity: 8× keeps corner white and dimensions.
    let scaled = smdh.icon_rgb_scaled(8);
    assert_eq!(scaled.len(), (48 * 8) * (48 * 8) * 3);
    assert_eq!((scaled[0], scaled[1], scaled[2]), (248, 252, 248));
}
