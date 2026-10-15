//! Scene -> PNG (replaces `png.py`'s Pillow rasterizer). The scene is
//! serialized to the same deterministic SVG the exporter produces, then
//! rasterized by resvg at an integral scale. DPI metadata is injected as a
//! pHYs chunk (Pillow parity) — the pixel bytes themselves stay
//! deterministic.

use resvg::tiny_skia;
use resvg::usvg;

use crate::error::{HeatmapError, Result};
use crate::scene::Scene;
use crate::svg::render_svg;

pub struct PngOut {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub dpi: u32,
}

/// Rasterize a scene to PNG. `dpi` defaults to 96 × scale.
pub fn png_bytes(
    scene: &Scene,
    scale: i64,
    dpi: Option<i64>,
    font_path: Option<&str>,
) -> Result<PngOut> {
    let scale = scale.max(1) as f32;
    let dpi = dpi.unwrap_or(96 * scale as i64).max(1) as u32;

    let svg = render_svg(scene);

    let mut fontdb = usvg::fontdb::Database::new();
    if let Some(path) = font_path {
        fontdb
            .load_font_file(path)
            .map_err(|e| HeatmapError::new(format!("cannot load font {path}: {e}")))?;
    } else {
        fontdb.load_system_fonts();
    }
    let opt = usvg::Options {
        fontdb: std::sync::Arc::new(fontdb),
        ..Default::default()
    };
    let tree = usvg::Tree::from_data(&svg, &opt)
        .map_err(|e| HeatmapError::new(format!("SVG parse failed: {e}")))?;

    let size = tree.size();
    let width = (size.width() * scale).round() as u32;
    let height = (size.height() * scale).round() as u32;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| HeatmapError::new("invalid PNG dimensions"))?;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    let plain = pixmap
        .encode_png()
        .map_err(|e| HeatmapError::new(format!("PNG encode failed: {e}")))?;
    let data = set_dpi(&plain, dpi)?;
    Ok(PngOut {
        data,
        width,
        height,
        dpi,
    })
}

/// Insert a pHYs chunk (pixels-per-metre, unit=1) after IHDR. PNG chunks
/// are `[len u32][type][data][crc32]`; inserting before IDAT keeps the
/// image data untouched, so the pixels stay byte-identical.
fn set_dpi(png: &[u8], dpi: u32) -> Result<Vec<u8>> {
    const SIG: &[u8] = b"\x89PNG\r\n\x1a\n";
    if png.len() < SIG.len() + 8 || &png[..SIG.len()] != SIG {
        return Err(HeatmapError::new("not a PNG stream"));
    }
    let ppm = (dpi as f64 * 10000.0 / 25.4).round() as u32;
    let mut chunk = Vec::with_capacity(21);
    chunk.extend_from_slice(&9u32.to_be_bytes());
    chunk.extend_from_slice(b"pHYs");
    chunk.extend_from_slice(&ppm.to_be_bytes());
    chunk.extend_from_slice(&ppm.to_be_bytes());
    chunk.push(1); // unit: metre
    let crc = crc32(&chunk[4..17]);
    chunk.extend_from_slice(&crc.to_be_bytes());

    // IHDR ends at 8 (sig) + 4 (len) + 4 (type) + 13 (data) + 4 (crc) = 33
    let insert_at = SIG.len() + 25;
    let mut out = Vec::with_capacity(png.len() + chunk.len());
    out.extend_from_slice(&png[..insert_at]);
    out.extend_from_slice(&chunk);
    out.extend_from_slice(&png[insert_at..]);
    Ok(out)
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::dataset::{Cell, Dataset, Module};
    use crate::views::build_scene;

    fn sample() -> Dataset {
        let mut m = Module {
            name: "M".into(),
            ..Default::default()
        };
        for i in 0..12 {
            m.cells.push(Cell {
                addr: 0x180 + (i * 4) as u64,
                size: 4,
                tier: i % 4,
                ..Default::default()
            });
        }
        Dataset {
            title: "t".into(),
            modules: vec![m],
            ..Default::default()
        }
    }

    #[test]
    fn png_is_deterministic_and_has_dpi_metadata() {
        let ds = sample();
        let cfg = Config::default();
        let scene = build_scene(&ds, "overview", None, &cfg, 0, None, false, None).unwrap();
        let a = png_bytes(&scene, 2, None, None).unwrap();
        let b = png_bytes(&scene, 2, None, None).unwrap();
        assert_eq!(a.data, b.data, "PNG bytes must be deterministic");
        assert_eq!(a.width, (scene.width * 2.0) as u32);
        assert_eq!(a.height, (scene.height * 2.0) as u32);
        assert_eq!(a.dpi, 192); // 96 × 2
                                // pHYs present: 3780 ppm == 96 dpi
        assert!(a.data.windows(4).any(|w| w == b"pHYs"));
        // not blank: real image data is sizeable
        assert!(a.data.len() > 1000);
        // explicit dpi is honored
        let c = png_bytes(&scene, 2, Some(300), None).unwrap();
        assert_eq!(c.dpi, 300);
    }
}
