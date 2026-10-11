//! Golden test: BCRES inspection against a real extracted model
//! (`Source_Extracted/garc/a/0/0/1/0.bin` — validated against `strings`
//! output on 2026-10-05). Skips when the asset is absent.

use std::path::PathBuf;

use xy_gfx::bcres::Bcres;

fn asset(rel: &str) -> Option<PathBuf> {
    let p = std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"))
        .join(rel);
    p.exists().then_some(p)
}

#[test]
fn model_container_inspects() {
    let Some(path) = asset("Source_Extracted/garc/a/0/0/1/0.bin") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let bcres = Bcres::parse(&bytes).expect("parse");
    assert_eq!(bcres.version_stamp, 0x0500_0000);
    assert_eq!(bcres.content_count, 1);

    let tags: Vec<String> = bcres
        .chunk_tags()
        .into_iter()
        .map(|(_, t)| String::from_utf8_lossy(&t).into_owned())
        .collect();
    for expected in ["DICT", "CFLT", "CALT", "CENV"] {
        assert!(
            tags.iter().any(|t| t == expected),
            "expected chunk {expected} in {tags:?}"
        );
    }

    let names = bcres.names();
    for expected in ["Light1", "AmbientLight1", "SceneEnvironment1"] {
        assert!(
            names.iter().any(|n| n == expected),
            "expected name {expected} in {names:?}"
        );
    }
}
