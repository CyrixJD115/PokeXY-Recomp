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
        "decomp/code_c/00100030_helper.c",
        "void f_00100030(void) {}\n",
    );
    // cro_c filenames carry ABSOLUTE Ghidra addresses (0x100000 base)
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
