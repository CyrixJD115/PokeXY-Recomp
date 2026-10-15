//! Tests for the collector port: synthetic Source_Extracted tree — never
//! touches the real `.Data`.

use super::*;
use std::fs;
use std::path::Path;

fn write(root: impl AsRef<Path>, rel: &str, contents: &str) {
    let path = root.as_ref().join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents).unwrap();
}

/// Minimal Source_Extracted tree exercising every source and frame rule.
fn build_tree(root: &Path) {
    write(
        root,
        "recomp/entries.c",
        "// generated\nstatic const ModuleEntry g_modules[] = {\n\
         {\"DllBag\", &m1_base, 0x00001000u, 1, NULL},\n\
         {\"DllBox\", &m2_base, 0x00002000u, 2, NULL},\n};\n",
    );
    write(
        root,
        "recomp/functions.h",
        "void f_00100000(Context *ctx);\nvoid f_00100010(Context *ctx);\n",
    );
    write(
        root,
        "recomp/m1_functions.h",
        "void m1_f_00000180(Context *ctx);\n",
    );
    write(
        root,
        "pipeline/symbols/code.bin.csv",
        "\"Location\",\"Name\",\"Namespace\",\"Mode\",\"Size\",\"Segment\"\n\
         \"0x00100000\",\"FUN_00100000\",\"Global\",\"T\",\"0x10\",\".text\"\n\
         \"0x00100010\",\"SetPitch\",\"ui::audio\",\"T\",\"0x20\",\".text\"\n",
    );
    write(
        root,
        "pipeline/symbols/DllBag.cro.csv",
        "\"Location\",\"Name\",\"Namespace\",\"Mode\",\"Size\",\"Segment\"\n\
         \"0x00000180\",\"FUN_00000180\",\"Global\",\"T\",\"0x8\",\".text\"\n",
    );
    write(
        root,
        "cro_extracted/static/symbols.tsv",
        "name\tload_addr\nOddExport\t00100021\n",
    );
    write(
        root,
        "cro_extracted/DllBag/symbols.tsv",
        "name\toffset\nBag_Open\t00000190\n_ZN3bag4ViewD1Ev\t00000190\n",
    );
    write(
        root,
        "cro_extracted/DllBag/imports.tsv",
        "module\tindexed\tanonymous\n|static|\t1\t0\nDllBox\t2\t1\n",
    );
    write(
        root,
        "decomp/code_c/00100000_main.c",
        "void f_00100000(void) {}\n",
    );
    write(
        root,
        "decomp/code_c/00100030_helper.c",
        "void f_00100030(void) {}\n",
    );
    // cro_c filenames carry ABSOLUTE Ghidra addresses (0x100000 base)
    write(
        root,
        "decomp/cro_c/DllBag/00100180_merge.c",
        "void f(void) {}\n",
    );
    write(
        root,
        "decomp/cro_c/DllBag/001001A0_thing.c",
        "void f(void) {}\n",
    );
    write(root, "pipeline/orig/code.bin", "MAIN-ORIG");
    write(root, "pipeline/out/code.bin", "MAIN-ORIG"); // identical -> ok
    write(root, "pipeline/orig/DllBag.cro", "BAG-ORIG");
    write(root, "pipeline/out/DllBag.cro", "BAG-RELINKED"); // differs -> mismatch
}

#[test]
fn collects_and_merges_all_sources() {
    let dir = tempfile();
    let root = dir.path().join("Source_Extracted");
    build_tree(&root);
    let ds = ProjectSource::new(&root).unwrap().dataset().unwrap();

    assert_eq!(ds.title, "Pokémon X/Y recomp + decomp progress");
    // Main first, then entries.c order.
    assert_eq!(
        ds.modules
            .iter()
            .map(|m| m.name.as_str())
            .collect::<Vec<_>>(),
        vec!["Main", "DllBag", "DllBox"]
    );

    let main = &ds.modules[0];
    let at = |a: u64| {
        main.cells
            .iter()
            .find(|c| c.addr == a)
            .unwrap_or_else(|| panic!("no cell at 0x{a:X}"))
    };
    let c0 = at(0x100000);
    assert!(c0.covered());
    // the decomp-corpus C file earns the tier: under the current rule an
    // anonymous FUN_ CSV row alone only proves existence (stays raw)
    assert_eq!(c0.tier, TIER_DECOMPILED);
    assert_eq!(c0.size, 0x10);
    let c1 = at(0x100010);
    assert_eq!(c1.name, "SetPitch");
    assert_eq!(c1.namespace, "ui::audio");
    assert_eq!(c1.tier, TIER_NAMED);
    // Thumb bit stripped: 0x100021 export lands at 0x100020
    assert_eq!(at(0x100020).name, "OddExport");
    assert!(main.relink_ok == Some(true));

    let bag = ds.module("DllBag").unwrap();
    let at = |a: u64| {
        bag.cells
            .iter()
            .find(|c| c.addr == a)
            .unwrap_or_else(|| panic!("no cell at 0x{a:X}"))
    };
    // 0x180: covered + CSV FUN_ row + a cro_c C file -> decompiled
    assert!(at(0x180).covered() && at(0x180).tier == TIER_DECOMPILED);
    // multiple export names at one addr -> lexicographically smallest
    assert_eq!(at(0x190).name, "Bag_Open");
    // namespace demangles the *chosen* name only; "Bag_Open" isn't mangled
    assert_eq!(at(0x190).namespace, "");
    // cro_c absolute filename address shifted into the module frame
    assert_eq!(at(0x1a0).tier, TIER_DECOMPILED);
    assert_eq!(bag.relink_ok, Some(false));
    assert_eq!(bag.imports, vec!["DllBox", "Main"]);
    // demangling only applies when the mangled name is the one chosen
    assert_eq!(demangle_namespace("_ZN3bag4ViewD1Ev"), "bag");

    // every source is present for Main + DllBag; DllBox has none on disk
    // (fixture provisions it only in entries.c) and must warn, not crash
    assert_eq!(
        ds.warnings,
        vec![
            "DllBox: no symbol CSV (DllBox.cro.csv)",
            "DllBox: no export symbols (symbols.tsv)",
            "DllBox: no decompiled C corpus",
        ]
    );
}

#[test]
fn optional_sources_warn_not_crash() {
    let dir = tempfile();
    let root = dir.path().join("Source_Extracted");
    write(
        &root,
        "recomp/entries.c",
        "{\"DllBag\", &m1_base, 0x1000u, 1,\n",
    );
    write(
        &root,
        "recomp/functions.h",
        "void f_00100000(Context *ctx);\n",
    );
    let ds = ProjectSource::new(&root).unwrap().dataset().unwrap();
    let main = &ds.modules[0];
    assert_eq!(main.cells.len(), 1);
    assert!(main.cells[0].covered());
    assert_eq!(
        ds.warnings.len(),
        6,
        "missing CSV + TSV + decomp for both modules: {:?}",
        ds.warnings
    );
}

#[test]
fn rejects_missing_entries_c() {
    let dir = tempfile();
    let err = ProjectSource::new(dir.path().join("nope")).unwrap_err();
    assert!(err.message.contains("recomp/entries.c"));
}

#[test]
fn discovers_data_root_by_walking_up() {
    let dir = tempfile();
    let root = dir
        .path()
        .join("proj")
        .join(".Data")
        .join("Source_Extracted");
    write(
        &root,
        "recomp/entries.c",
        "{\"DllBag\", &m1_base, 0x1000u, 1,\n",
    );
    let deep = dir.path().join("proj").join("a").join("b").join("c");
    fs::create_dir_all(&deep).unwrap();
    assert_eq!(discover_data_root(Some(&deep)).unwrap(), root);
    assert!(discover_data_root(Some(&dir.path().join("elsewhere"))).is_err());
}

#[test]
fn fingerprint_is_stable_and_content_sensitive() {
    let dir = tempfile();
    let root = dir.path().join("Source_Extracted");
    build_tree(&root);
    let src = ProjectSource::new(&root).unwrap();
    let fp1 = src.fingerprint().unwrap();
    let fp2 = src.fingerprint().unwrap();
    assert_eq!(fp1, fp2);
    assert_eq!(fp1.len(), 16);

    // append to a tracked file -> size changes -> fingerprint changes
    let mut c = fs::OpenOptions::new()
        .append(true)
        .open(root.join("decomp/code_c/00100030_helper.c"))
        .unwrap();
    use std::io::Write;
    writeln!(c, "// more").unwrap();
    let fp3 = src.fingerprint().unwrap();
    assert_ne!(fp1, fp3);
}

#[test]
fn demangler_handles_nested_names() {
    assert_eq!(demangle_namespace("_ZN3bag4ViewD1Ev"), "bag");
    assert_eq!(demangle_namespace("_ZN2ui5audio9SetPitchE"), "ui::audio");
    assert_eq!(demangle_namespace("_ZNSt6vectorIiEC1E"), ""); // template args = bail
    assert_eq!(demangle_namespace("PlainName"), "");
    assert_eq!(demangle_namespace("_Z3fooi"), ""); // not a nested name
}

// ------------------------------------------------------ armcc-matching

/// armcc-matching evidence tree as a SIBLING of the `.Data` root's parent
/// (the layout `find_armcc_dir` walks up to find): proven masked matches,
/// a relaxed match, and the attempted log.
fn build_armcc_tree(repo: &Path) {
    // existing named cell (Bag_Open @0x190): escalates to verified, size
    // comes from the header (the export TSV carries no sizes)
    write(
        repo,
        "xy-decomp/armcc-matching/matched/DllBag/0x190.c",
        "/* DllBag @ 0x190 size 0x20 MATCHED (masked): bool setter.\n\
         * Verify: fingerprint_fn.py DllBag 0x190 0x20 ...\n */\n\
         void Bag_Open(void *o, int v) { *(int*)((char*)o + 4) = v; }\n",
    );
    // an offset NO other source knows (prologue-less leaf missed by the
    // Ghidra catalog) — must be INSERTED as its own cell
    write(
        repo,
        "xy-decomp/armcc-matching/matched/DllBag/0x1b0.c",
        "/* DllBag function @ CRO file offset 0x1b0, size 0x8.\n\
         * Match: 2/2 words (*** MASKED MATCH ***).\n */\n\
         void f(void) {}\n",
    );
    // module dir that matches no entries.c name -> warning, not a crash
    write(
        repo,
        "xy-decomp/armcc-matching/matched/OldName/0x180.c",
        "/* OldName @ 0x180 size 0x4 MATCHED. */\nvoid f(void) {}\n",
    );
    // relaxed match: counts as an overlay, never as verified
    write(
        repo,
        "xy-decomp/armcc-matching/relaxed/DllBag/0x1c0.c",
        "/* DllBag @ 0x1c0 size 0x4 RELAXED (--relax-nops only). */\nvoid f(void) {}\n",
    );
    // attempted log: one unproven function, a two-address continuation
    // row for another module, a module cited WITHOUT its Dll prefix
    // (resolved by the dll-prefix fallback), and a prose citation of an
    // ALREADY-MATCHED function (must NOT be flagged attempted)
    write(
        repo,
        "xy-decomp/armcc-matching/attempted.md",
        "# Attempted — not yet matched\n\n\
         ## DllBag@0x1d0 (size ?)\n\n\
         - Stuck on: armcc if-conversion vs retail branch.\n\n\
         ## Bag@0x1d8 destructor variant\n\n\
         - Same residue class as the DllBox@0x180/0x184 destructor class.\n\n\
         - The leaf variant is cracked (see matched/ DllBag@0x190).\n",
    );
}

#[test]
fn armcc_matching_evidence_sets_verified_and_overlays() {
    let dir = tempfile();
    let root = dir.path().join(".Data").join("Source_Extracted");
    build_tree(&root);
    build_armcc_tree(dir.path());
    let ds = ProjectSource::new(&root).unwrap().dataset().unwrap();

    let bag = ds.module("DllBag").unwrap();
    let at = |a: u64| {
        bag.cells
            .iter()
            .find(|c| c.addr == a)
            .unwrap_or_else(|| panic!("no DllBag cell at 0x{a:X}"))
    };
    // masked match on an existing named cell: verified wins over named,
    // and the proven size from the matched file's header fills in
    assert_eq!(at(0x190).tier, TIER_VERIFIED);
    assert_eq!(at(0x190).size, 0x20);
    // attempted.md prose cites the already-matched 0x190: no attempted flag
    assert!(!at(0x190).attempted());
    // matched-only offset: new cell (Ghidra-missed prologue-less leaf)
    assert_eq!(at(0x1b0).tier, TIER_VERIFIED);
    assert_eq!(at(0x1b0).size, 0x8);
    // relaxed match: overlay flag, tier untouched, NOT verified
    let relaxed = at(0x1c0);
    assert!(relaxed.hint());
    assert_ne!(relaxed.tier, TIER_VERIFIED);
    // attempted: overlay flag on a raw cell
    assert!(at(0x1d0).attempted());
    assert_eq!(at(0x1d0).tier, 0);
    // unprefixed module citation (Bag@0x1d8 -> DllBag): still attributed
    assert!(at(0x1d8).attempted());

    // dataset-level counts
    assert_eq!(ds.tier_counts()[3], 2);
    assert_eq!(ds.hint_count(), 1);
    // DllBag@0x1d0/0x1d8 + the DllBox@0x180/0x184 continuation pair
    assert_eq!(ds.attempted_count(), 4);

    // DllBox has no other sources; its attempted.md continuation
    // addresses (/0x184 shares DllBox@0x180's module) still become cells
    let boxm = ds.module("DllBox").unwrap();
    assert!(boxm.cells.iter().any(|c| c.addr == 0x180 && c.attempted()));
    assert!(boxm.cells.iter().any(|c| c.addr == 0x184 && c.attempted()));

    // the unknown module dir surfaces as a warning after the per-module
    // ones (DllBox has no CSV/TSV/decomp on disk)
    assert_eq!(
        ds.warnings.len(),
        4,
        "unexpected warnings: {:?}",
        ds.warnings
    );
    assert!(ds.warnings[3]
        .contains("armcc-matching: matched/relaxed module \"oldname\" matches no known module"));
}

#[test]
fn armcc_absent_is_silent() {
    // no xy-decomp/armcc-matching anywhere above the data root: the
    // optional source contributes neither cells nor warnings
    let dir = tempfile();
    let root = dir.path().join("Source_Extracted");
    build_tree(&root);
    let ds = ProjectSource::new(&root).unwrap().dataset().unwrap();
    assert!(ds.warnings.iter().all(|w| !w.contains("armcc-matching")));
    assert_eq!(ds.tier_counts()[3], 0);
    assert_eq!(ds.hint_count(), 0);
    assert_eq!(ds.attempted_count(), 0);
}

#[test]
fn fun_catalog_row_without_c_stays_raw() {
    // a FUN_ row in the Ghidra symbol CSV only proves the function EXISTS
    // (address + size): without a decomp-corpus C file or a real name it
    // stays raw — the "still in raw binary" state the heatmap exists to show
    let dir = tempfile();
    let root = dir.path().join("Source_Extracted");
    write(
        &root,
        "recomp/entries.c",
        "{\"DllBag\", &m1_base, 0x1000u, 1,\n",
    );
    write(
        &root,
        "pipeline/symbols/DllBag.cro.csv",
        "\"Location\",\"Name\",\"Namespace\",\"Mode\",\"Size\",\"Segment\"\n\
         \"0x00000180\",\"FUN_00000180\",\"Global\",\"T\",\"0x8\",\".text\"\n",
    );
    let ds = ProjectSource::new(&root).unwrap().dataset().unwrap();
    let bag = ds.module("DllBag").unwrap();
    assert_eq!(bag.cells.len(), 1);
    assert_eq!(bag.cells[0].tier, 0);
    assert_eq!(bag.cells[0].size, 0x8);
}

#[test]
fn main_matched_offsets_accept_both_frames() {
    // Main (code.bin) matched offsets: VAs (>= 0x100000) pass through,
    // file offsets (< 0x100000) shift up by the load base
    assert_eq!(ProjectSource::armcc_addr(true, 0x7_0000), 0x17_0000);
    assert_eq!(ProjectSource::armcc_addr(true, 0x10_0180), 0x10_0180);
    // CRO offsets are module-relative and never shifted
    assert_eq!(ProjectSource::armcc_addr(false, 0x180), 0x180);
}

#[test]
fn fingerprint_tracks_armcc_evidence() {
    let dir = tempfile();
    let root = dir.path().join(".Data").join("Source_Extracted");
    build_tree(&root);
    build_armcc_tree(dir.path());
    let src = ProjectSource::new(&root).unwrap();
    let fp1 = src.fingerprint().unwrap();

    // a NEW armcc match file must trigger a live reload
    write(
        dir.path(),
        "xy-decomp/armcc-matching/matched/DllBag/0x1e0.c",
        "/* DllBag @ 0x1e0 size 0x4 MATCHED. */\nvoid f(void) {}\n",
    );
    assert_ne!(
        fp1,
        src.fingerprint().unwrap(),
        "new matched file must change the fingerprint"
    );
}

fn tempfile() -> tempdir::TempDirGuard {
    tempdir::create()
}

mod tempdir {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(0);

    pub struct TempDirGuard(pub PathBuf);

    impl Drop for TempDirGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    impl TempDirGuard {
        pub fn path(&self) -> &std::path::Path {
            &self.0
        }
    }

    pub fn create() -> TempDirGuard {
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let path = std::env::temp_dir().join(format!("xy-heatmap-test-{pid}-{n}"));
        std::fs::create_dir_all(&path).unwrap();
        TempDirGuard(path)
    }
}
