//! Golden test: BLZ (ExeFS `.code` backward-LZSS) decompression vs the
//! known-good v1.5 update `code.bin` size on record (5,169,152 bytes —
//! see `docs/3ds/exefs-cia.md`, `docs/general/findings.md`).
//!
//! Skips cleanly when the extracted `.code` section is absent.

use std::path::PathBuf;
use xy_archive::blz;

fn data_root() -> PathBuf {
    std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"))
}

fn check(code_path: PathBuf) {
    if !code_path.exists() {
        eprintln!("skipping: {} not found", code_path.display());
        return;
    }
    let compressed = std::fs::read(&code_path).expect("read .code");
    let decompressed = blz::decompress(&compressed).expect("decompress");

    assert_eq!(decompressed.len(), 5_169_152, "v1.5 code.bin size pin");

    // The scheme's leading uncompressed region is copied through verbatim.
    let prefix_len = 72;
    assert_eq!(
        &decompressed[..prefix_len],
        &compressed[..prefix_len],
        "uncompressed lead-in must match the source exactly"
    );

    // Thumb `BX LR` (0x4770) is a common function epilogue in compiled ARM
    // code. Random bytes would place it roughly once per 65536 bytes (~79
    // times over 5.1 MB); real code clears that baseline by a wide margin
    // (244 observed on both X and Y).
    let bx_lr_count = decompressed.windows(2).filter(|w| w == b"\x70\x47").count();
    assert!(
        bx_lr_count > 150,
        "expected Thumb BX LR bytes well above the random baseline (~79) \
         in real code, found {bx_lr_count}"
    );
}

#[test]
fn blz_decompresses_update_x_code() {
    check(data_root().join("Rom/X/Upd/exefs/.code"));
}

#[test]
fn blz_decompresses_update_y_code() {
    check(data_root().join("Rom/Y/Upd/exefs/.code"));
}
