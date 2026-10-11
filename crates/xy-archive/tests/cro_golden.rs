//! Golden test: CRO0 parser against the real plaintext modules.
//! The 2026-10-05 correction: these files were never encrypted — the
//! 0x00-0x7F SHA-256 hash block merely looks random; `CRO0` sits at 0x80.
//! Skips cleanly when assets are absent.

use std::path::PathBuf;

use xy_archive::cro::Cro;

fn data_root() -> PathBuf {
    std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Data"))
}

#[test]
fn dll_battle_parses_with_symbols() {
    let path = data_root().join("3ds_extracted/romfs/DllBattle.cro");
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let cro = Cro::parse(&bytes).expect("parse");
    assert_eq!(cro.name().unwrap(), "DllBattle");
    assert_eq!(cro.file_size as usize, bytes.len());
    assert_eq!(cro.code_offset, 0x180); // header + tables end at 0x180
    assert!(cro.code_size > 0x8000, "battle module code size");
    assert!(
        cro.segments
            .iter()
            .any(|s| s.ty == xy_archive::cro::SegmentType::Text),
        "needs a .text segment"
    );
    // Real named symbols must be present and land inside the file.
    // (GameFreak modules export mostly anonymously; named exports are few.)
    assert!(
        !cro.named_exports.is_empty(),
        "battle module named exports: {}",
        cro.named_exports.len()
    );
    for e in cro.named_exports.iter().take(16) {
        assert!(e.offset < bytes.len(), "export {} out of bounds", e.name);
        assert!(!e.name.is_empty());
    }
    // Cross-module imports reference other Dlls and/or the static module.
    assert!(
        !cro.import_modules.is_empty(),
        "battle module should import symbols"
    );
}

#[test]
fn static_crs_is_the_static_module() {
    let path = data_root().join("3ds_extracted/romfs/static.crs");
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let cro = Cro::parse(&bytes).expect("parse");
    assert_eq!(cro.name().unwrap(), "|static|");
    // The static module exports the main executable's symbols (461 named)
    // and imports from all 78 game subsystem modules.
    assert_eq!(cro.named_exports.len(), 461);
    assert_eq!(cro.import_modules.len(), 78);
}
