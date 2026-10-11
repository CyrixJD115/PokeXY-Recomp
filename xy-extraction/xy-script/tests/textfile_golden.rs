//! Golden test: Gen 6 text-bank decoder against real gametext banks.
//! `.Data/src_extracted/garc/a/0/7/` language variants, mapped empirically
//! 2026-10-05 by decoding bank 080 of each: 2 & 3 = Japanese, 4 = English,
//! 5 = French, 6 = Italian, 7 = German, 8 = Spanish, 9 = Korean.
//! Skips cleanly when assets are absent.

use std::path::PathBuf;

use xy_script::textfile::TextFile;

fn data_root() -> PathBuf {
    std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"))
}

fn bank(garc: &str, file: &str) -> Option<PathBuf> {
    let p = data_root()
        .join("src_extracted/garc")
        .join(garc)
        .join(format!("{file}.bin"));
    p.exists().then_some(p)
}

#[test]
fn english_species_names_decode() {
    // TextReference.GameText_XY: index 080 = SpeciesNames; 0 = egg placeholder.
    let Some(path) = bank("a/0/7/4", "080") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let tf = TextFile::parse(&bytes).expect("parse");
    let lines = tf.lines().expect("lines");
    assert!(
        tf.line_count() >= 721,
        "Gen 6 national dex names: {}",
        tf.line_count()
    );
    assert_eq!(lines[0], "Egg");
    assert_eq!(lines[1], "Bulbasaur");
    assert_eq!(lines[2], "Ivysaur");
    assert_eq!(lines[3], "Venusaur");
    // National dex #658 = Greninja.
    assert_eq!(lines[658], "Greninja");
}

#[test]
fn english_item_names_decode() {
    // TextReference.GameText_XY: index 096 = ItemNames.
    let Some(path) = bank("a/0/7/4", "096") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let tf = TextFile::parse(&bytes).expect("parse");
    let lines = tf.lines().expect("lines");
    // Item 0 is the in-game "???" placeholder; 1 = Master Ball.
    assert_eq!(lines[0], "???");
    assert_eq!(lines[1], "Master Ball");
    assert_eq!(lines[2], "Ultra Ball");
    assert_eq!(lines[3], "Great Ball");
    assert_eq!(lines[4], "Poké Ball");
}

#[test]
fn japanese_gametext_is_language_variant_zero() {
    // romfs a/0/7/2 = Japanese (bank 3 is a second Japanese set).
    let Some(path) = bank("a/0/7/2", "080") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bytes = std::fs::read(&path).unwrap();
    let tf = TextFile::parse(&bytes).expect("parse");
    let lines = tf.lines().expect("lines");
    assert_eq!(lines[0], "タマゴ", "Japanese Egg");
    assert_eq!(lines[1], "フシギダネ", "Japanese Bulbasaur");
}
