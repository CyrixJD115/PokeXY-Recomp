//! Golden-file tests for the GARC parser and LZ-0x11 (dsdecmp flavor)
//! decoder against real Pokémon X ROM assets.
//!
//! Skipped automatically when the assets are absent, so `cargo test` works
//! on machines without the extracted ROM data.
//!
//! Parity note: Kuriimu2 1.2.1's GARC4 plugin refuses to load XY's v4 GARCs
//! ("No plugin could identify the file"), so the reference is pk3DS
//! `GARC.cs`/`LZSS.cs` semantics plus the container's self-describing
//! cross-checks exercised below (fileSize, FATO/FATB consistency, FIMB
//! bounds, and payload-embedded size fields).

use std::path::PathBuf;

use xy_archive::garc::Garc;
use xy_archive::lzss;

fn romfs(rel: &str) -> Option<PathBuf> {
    let root = std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"));
    let p = root.join("Raw_Extracted/romfs").join(rel);
    p.exists().then_some(p)
}

/// Small model GARC: 3 flat CGFX files, all offsets/sizes verified by hand
/// from a hexdump on 2026-10-05.
#[test]
fn a_0_0_1_model_garc() {
    let Some(path) = romfs("a/0/0/1") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let garc = Garc::parse(&bytes).expect("parse should succeed");

    assert_eq!(garc.entries().len(), 3);
    assert_eq!(garc.file_count(), 3);
    let lens: Vec<u32> = garc.entries()[0..3].iter().map(|e| e.subs[0].len).collect();
    assert_eq!(lens, vec![0x725, 0x44E, 0x444]);

    for (i, &len) in lens.iter().enumerate() {
        let blob = garc.subfile(i, 0).unwrap();
        assert_eq!(&blob[..4], b"CGFX", "entry {i} should be a CGFX model");
        // The CGFX container embeds its own total size at +0x0C; it must
        // agree with the FATB sub-entry length (padding excluded).
        let self_size = u32::from_le_bytes(blob[0x0C..0x10].try_into().unwrap());
        assert_eq!(self_size, len, "CGFX self-size vs FATB length");
    }
}

/// `btl_ai.garc`: 6 flat files; file 3 starts with `0x11` while being raw
/// data — the pk3DS-style heuristic must keep it raw.
#[test]
fn btl_ai_raw_false_positive() {
    let Some(path) = romfs("btl_ai.garc") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let garc = Garc::parse(&bytes).expect("parse should succeed");

    assert_eq!(garc.entries().len(), 6);
    assert_eq!(garc.file_count(), 6);
    let third = garc.subfile(3, 0).unwrap();
    assert_eq!(&third[..4], &[0x11, 0x01, 0x00, 0x00]);
    assert!(lzss::try_decompress(third).is_none(), "must stay raw");
    for (i, _) in garc.entries().iter().enumerate() {
        assert!(lzss::try_decompress(garc.subfile(i, 0).unwrap()).is_none());
    }
}

/// `a/0/1/0`: one compressed sub-file decoding to a named BCLAN text bank —
/// pins the dsdecmp indicator mapping against real data.
#[test]
fn a_0_1_0_compressed_bclan() {
    let Some(path) = romfs("a/0/1/0") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let garc = Garc::parse(&bytes).expect("parse should succeed");

    assert_eq!(garc.file_count(), 1);
    let raw = garc.subfile(0, 0).unwrap();
    assert_eq!(raw[0], 0x11, "payload should be LZ-0x11 compressed");
    let dec = lzss::try_decompress(raw).expect("should decode");
    // Structure (from the decoded bytes): u32 0x0C, then a NUL-terminated
    // bank name at +4, zero padding to 0x20-byte alignment.
    assert_eq!(&dec[0..4], &[0x0C, 0, 0, 0]);
    assert_eq!(&dec[4..0x1A], b"menu_app_active.bclan\0");
}
