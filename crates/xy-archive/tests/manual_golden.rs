//! Golden test: the full e-manual pipeline (BCMA → LZ10 → inner DARC)
//! vs the validated reference tree at `.Data/src_extracted/manual`
//! (3663 assets from 206 pages — byte-parity first verified by hand on
//! 2026-10-05). Compares relative path sets and FNV-1a content hashes.
//!
//! Skips cleanly when the assets are absent.

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;

use xy_archive::bcma::{unique_name, Bcma};
use xy_core::lz10;
use xy_core::util::fnv1a64;

fn data_root() -> PathBuf {
    std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"))
}

/// Same page-stem naming rule as the CLI.
fn page_stem(name: &str) -> String {
    name.rsplit_once('.')
        .map_or_else(|| name.to_string(), |(stem, _)| stem.to_string())
}

#[test]
fn manual_pipeline_matches_reference_tree() {
    let root = data_root();
    let bcma_path = root.join("3ds_extracted/part1_romfs/Manual.bcma");
    let reference = root.join("src_extracted/manual");
    if !bcma_path.exists() || !reference.is_dir() {
        eprintln!("skipping: golden assets not found");
        return;
    }

    let data = std::fs::read(&bcma_path).expect("read Manual.bcma");
    let archive = Bcma::parse(&data).expect("parse BCMA");
    let out_dir = std::env::temp_dir().join(format!("xy_manual_golden_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    let mut seen = HashSet::new();
    let mut pages = 0usize;
    let mut assets = 0usize;
    for entry in archive.files() {
        let (name, blob) = entry.expect("bcma entry");
        let page = unique_name(&page_stem(name), &mut seen);
        let inner = lz10::decompress(blob).expect("lz10 page");
        let darc = Bcma::parse(&inner).expect("inner darc");
        let page_dir = out_dir.join(&page);
        let mut page_seen = HashSet::new();
        for inner_entry in darc.files() {
            let (iname, iblob) = inner_entry.expect("inner entry");
            let iname = unique_name(iname, &mut page_seen);
            let dest = page_dir.join(&iname);
            std::fs::create_dir_all(dest.parent().expect("parent")).expect("mkdir");
            std::fs::write(&dest, iblob).expect("write asset");
            assets += 1;
        }
        pages += 1;
    }
    assert_eq!(pages, 206, "documented page count");
    assert_eq!(assets, 3663, "documented asset count");

    // Reference manifest.
    let mut reference_manifest: BTreeMap<String, u64> = BTreeMap::new();
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
                let bytes = std::fs::read(&path).expect("read reference");
                reference_manifest.insert(rel, fnv1a64(&bytes));
            }
        }
    }
    assert_eq!(reference_manifest.len(), 3663);

    let mut mismatches = 0usize;
    let mut produced: BTreeMap<String, u64> = BTreeMap::new();
    let mut stack = vec![out_dir.clone()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("read_dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path
                    .strip_prefix(&out_dir)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                let bytes = std::fs::read(&path).expect("read produced");
                produced.insert(rel, fnv1a64(&bytes));
            }
        }
    }
    for (rel, hash) in &produced {
        match reference_manifest.get(rel) {
            Some(want) if want == hash => {}
            Some(_) => {
                eprintln!("content mismatch: {rel}");
                mismatches += 1;
            }
            None => {
                eprintln!("not in reference: {rel}");
                mismatches += 1;
            }
        }
    }
    for rel in reference_manifest.keys() {
        if !produced.contains_key(rel) {
            eprintln!("missing from pipeline output: {rel}");
            mismatches += 1;
        }
    }
    let _ = std::fs::remove_dir_all(&out_dir);
    assert_eq!(mismatches, 0, "{mismatches} manual assets differ");
}
