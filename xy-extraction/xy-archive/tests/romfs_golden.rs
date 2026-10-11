//! Golden test: RomFS extraction vs the `ctrtool --romfsdir` reference tree
//! (`.Data/Raw_Extracted/romfs`, 653 files — parity first verified by hand on
//! 2026-10-05). Compares path sets, sizes, and FNV-1a content hashes.
//!
//! Skips cleanly when the dump or reference tree is absent.

use std::collections::BTreeMap;
use std::path::PathBuf;

use xy_archive::romfs::Romfs;
use xy_core::util::fnv1a64;

fn data_root() -> PathBuf {
    std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"))
}

#[test]
fn romfs_matches_ctrtool_reference() {
    let root = data_root();
    let cci = root.join("Rom/X/Base/0004000000055D00_v00.3ds");
    let reference = root.join("Raw_Extracted/romfs");
    if !cci.exists() || !reference.is_dir() {
        eprintln!("skipping: golden assets not found");
        return;
    }

    let bytes = std::fs::read(&cci).expect("read CCI");
    let romfs = Romfs::open(&bytes).expect("open RomFS");
    let entries = romfs.walk().expect("walk");
    let image = romfs.image(); // entry offsets are relative to this slice
    assert_eq!(entries.len(), 653, "documented ctrtool parity file count");

    // Reference manifest: relpath -> (size, fnv).
    let mut reference_manifest: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    let mut stack = vec![reference.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("read_dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path
                    .strip_prefix(&reference)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                let data = std::fs::read(&path).expect("read reference file");
                reference_manifest.insert(rel, (data.len() as u64, fnv1a64(&data)));
            }
        }
    }
    assert_eq!(
        reference_manifest.len(),
        653,
        "reference tree should hold exactly the ctrtool output"
    );

    let mut mismatches = 0usize;
    for e in &entries {
        let Some(&(size, hash)) = reference_manifest.get(&e.path) else {
            eprintln!("missing from reference: {}", e.path);
            mismatches += 1;
            continue;
        };
        let blob = &image[e.offset..e.offset + e.size as usize];
        let got = (blob.len() as u64, fnv1a64(blob));
        if got != (size, hash) {
            eprintln!(
                "content mismatch: {} {:?} vs ({size}, {hash:#x})",
                e.path, got
            );
            mismatches += 1;
        }
    }
    assert_eq!(
        mismatches, 0,
        "{mismatches} files differ from ctrtool output"
    );
}
