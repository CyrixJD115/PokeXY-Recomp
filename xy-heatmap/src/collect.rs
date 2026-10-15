//! Read-only collectors for the project's recomp/decomp data sources
//! (port of `collect.py`). Everything here only READS; collection produces
//! an in-memory [`Dataset`] and a cheap source fingerprint. Nothing writes
//! to disk — the web UI and the exporters consume the dataset directly.
//!
//! Canonical frames: **Main** cells use absolute addresses (0x00100000
//! base); **CRO** cells use module-relative text offsets (the 0x180 frame).
//!
//! Status tiers (color = highest rung reached):
//! - **raw** — the function is catalogued (Ghidra symbol CSV / recomp
//!   coverage) but has no decompiled C and no real name: still raw binary.
//! - **decompiled** — a C file for this address exists in the decomp corpus
//!   (`decomp/code_c`, `decomp/cro_c/<Mod>/`).
//! - **named** — a real symbol (CRO export table or a named CSV row).
//! - **verified** — **armcc masked-match proven**: a
//!   `xy-decomp/armcc-matching/matched/<Mod>/<offset>.c` candidate compiled
//!   with the retail toolchain reproduces the binary word-for-word
//!   (branch immediates excepted).
//!
//! Flags (overlays, never color steps): `covered` = recomp port coverage;
//! hint = relaxed armcc match (`relaxed/`, `--relax-nops` only — excluded
//! from verified per the armcc-matching README rule); attempted = an
//! armcc-matching agent burned its time cap here without a match
//! (`attempted.md`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use sha1::{Digest, Sha1};
use sha2::Sha256;

use crate::dataset::{Cell, Dataset, Module, FLAG_ATTEMPTED, FLAG_COVERED, FLAG_HINT};
use crate::error::{HeatmapError, Result};

const MAIN: &str = "Main";
const GHIDRA_BASE: u64 = 0x100000;

const TIER_DECOMPILED: usize = 1;
const TIER_NAMED: usize = 2;
const TIER_VERIFIED: usize = 3;

/// Data-source abstraction shared by the CLI and the server: the project
/// corpus, or one generic JSON dataset file.
pub trait Source {
    fn describe(&self) -> String;
    fn fingerprint(&self) -> Result<String>;
    fn dataset(&self) -> Result<Dataset>;
}

/// Walk up from `start` (default: cwd) looking for .Data/Source_Extracted.
pub fn discover_data_root(start: Option<&Path>) -> Result<PathBuf> {
    let p = start
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let mut cur = Some(p.as_path());
    while let Some(dir) = cur {
        let root = dir.join(".Data").join("Source_Extracted");
        if root.join("recomp").join("entries.c").is_file() {
            return Ok(root);
        }
        cur = dir.parent();
    }
    Err(HeatmapError::new(format!(
        "no data root found (looked for .Data/Source_Extracted with recomp/entries.c \
         above {}); pass --data-root, set XY_HEATMAP_DATA_ROOT, or supply --dataset",
        p.display()
    )))
}

/// Best-effort Itanium nested-name prefix -> `a::b::Class` ("" if unsure).
pub fn demangle_namespace(name: &str) -> String {
    let rest = match name.strip_prefix("_ZN") {
        Some(r) => r,
        None => return String::new(),
    };
    let bytes = rest.as_bytes();
    let n = bytes.len();
    let mut parts: Vec<&str> = Vec::new();
    let mut i = 0usize;
    while i < n {
        if bytes[i] == b'E' {
            break;
        }
        if rest[i..].starts_with("St") {
            parts.push("std");
            i += 2;
            continue;
        }
        if !parts.is_empty()
            && (bytes[i] == b'C' || bytes[i] == b'D')
            && i + 1 < n
            && bytes[i + 1].is_ascii_digit()
        {
            break; // constructor/destructor abbreviation ends the nested name
        }
        let mut j = i;
        while j < n && bytes[j].is_ascii_digit() {
            j += 1;
        }
        if j == i {
            return String::new(); // substitution code or expression — not worth guessing
        }
        let ln: usize = match rest[i..j].parse() {
            Ok(v) => v,
            Err(_) => return String::new(),
        };
        if ln == 0 || j + ln > n {
            return String::new();
        }
        parts.push(&rest[j..j + ln]);
        i = j + ln;
    }
    if parts.len() >= 2 {
        parts[..parts.len() - 1].join("::")
    } else {
        String::new()
    }
}

/// Per-address accumulator while merging sources.
#[derive(Default, Clone)]
struct Acc {
    size: u64,
    name: String,
    namespace: String,
    tier: usize,
    flags: u8,
}

/// Python `int(s, 16)` equivalent: base-16 with an optional `0x` prefix
/// (the Ghidra symbol CSVs use bare hex, the cro_extracted TSVs use `0x`).
fn parse_hex16(s: &str) -> Option<u64> {
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    u64::from_str_radix(s, 16).ok()
}

// ------------------------------------------------------- armcc-matching

/// armcc-matching evidence (the masked-match decomp workflow in
/// `xy-decomp/armcc-matching/` — see its README; PRs #1–#3). Module keys
/// are lowercased dir names / references.
#[derive(Default)]
struct Armcc {
    /// module -> [(function offset in the module frame, parsed size)]
    matched: BTreeMap<String, Vec<(u64, Option<u64>)>>,
    /// module -> function offsets (only matches with `--relax-nops`:
    /// excluded from the verified total per the armcc-matching rule)
    relaxed: BTreeMap<String, Vec<u64>>,
    /// (module, function offset) references from attempted.md — functions
    /// agents burned their time cap on without reaching a match
    attempted: Vec<(String, u64)>,
}

/// Walk up from the data root looking for the repo's armcc-matching tree
/// (the heatmap is standalone; the matching tree is repo content, found as
/// `<ancestor>/xy-decomp/armcc-matching` with a populated `matched/`).
fn find_armcc_dir(start: &Path) -> Option<PathBuf> {
    let mut cur = Some(start);
    while let Some(dir) = cur {
        let candidate = dir.join("xy-decomp").join("armcc-matching");
        if candidate.join("matched").is_dir() {
            return Some(candidate);
        }
        cur = dir.parent();
    }
    None
}

/// `matched/<Mod>/*.c` entries -> (offset from the filename stem, size from
/// the header comment). Files whose stem is not `0x<HEX>` are skipped.
fn scan_match_dir(dir: &Path) -> Vec<(u64, Option<u64>)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for p in entries.flatten() {
        let path = p.path();
        if !path.is_file() {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let Some(off) = parse_hex16(stem) else {
            continue;
        };
        let size = match_header_size(&path);
        out.push((off, size));
    }
    out.sort();
    out
}

/// The proven function length from a matched file's header comment: the
/// first `size <hex|dec>` in the leading `/* Module @ 0xNNN size 0xMMM …`
/// block (two observed shapes: `@ 0x1404 size 0x30` and
/// `@ CRO file offset 0xae8, size 0x1c.`). Prose later in the header may
/// discuss wrong candidate sizes — first occurrence is the canonical one.
fn match_header_size(path: &Path) -> Option<u64> {
    static SIZE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = SIZE.get_or_init(|| regex::Regex::new(r"size\s+(0x[0-9A-Fa-f]+|\d+)").unwrap());
    let bytes = std::fs::read(path).ok()?;
    let head = String::from_utf8_lossy(&bytes[..bytes.len().min(1024)]);
    let cap = re.captures(&head)?;
    let s = cap[1].trim_start_matches("0x");
    if cap[1].starts_with("0x") {
        u64::from_str_radix(s, 16).ok()
    } else {
        s.parse().ok()
    }
}

/// attempted.md references: `DllPuzzle@0x100c`, plus `/0x881c`
/// continuations sharing the preceding module reference
/// (`DllHeading@0x87e4/0x881c`). Prose also cites already-matched
/// functions — the caller skips references that ended up verified.
fn parse_attempted(path: &Path) -> Vec<(String, u64)> {
    static REF: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = REF.get_or_init(|| {
        regex::Regex::new(r"([A-Za-z][A-Za-z0-9_]*)@0x([0-9A-Fa-f]+)((?:/0x[0-9A-Fa-f]+)*)")
            .unwrap()
    });
    let text = String::from_utf8_lossy(&std::fs::read(path).unwrap_or_default()).into_owned();
    let mut out = Vec::new();
    for cap in re.captures_iter(&text) {
        let module = cap[1].to_lowercase();
        let first = u64::from_str_radix(&cap[2], 16).unwrap_or(0);
        out.push((module.clone(), first));
        if let Some(rest) = cap.get(3) {
            for tok in rest.as_str().split('/').skip(1) {
                if let Some(off) = parse_hex16(tok) {
                    out.push((module.clone(), off));
                }
            }
        }
    }
    out
}

/// The project dataset: lazily collected, fingerprinted for live reload.
#[derive(Debug)]
pub struct ProjectSource {
    pub root: PathBuf,
    entries_c: PathBuf,
}

impl ProjectSource {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        let entries_c = root.join("recomp").join("entries.c");
        if !entries_c.is_file() {
            return Err(HeatmapError::new(format!(
                "data root {} is missing recomp/entries.c — this does not look like a \
                 Source_Extracted tree",
                root.display()
            )));
        }
        Ok(Self { root, entries_c })
    }

    fn read_lossy(&self, rel: &str) -> Result<String> {
        let bytes = std::fs::read(self.root.join(rel))?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    /// parent/name, falling back to a case-insensitive match of one level
    /// (entries.c says `DllEShop`, the extracted tree says `DllEshop`).
    fn ci_child(&self, parent: &Path, name: &str) -> Option<PathBuf> {
        let exact = parent.join(name);
        if exact.exists() {
            return Some(exact);
        }
        let low = name.to_lowercase();
        if let Ok(entries) = std::fs::read_dir(parent) {
            for entry in entries.flatten() {
                if entry.file_name().to_string_lossy().to_lowercase() == low {
                    return Some(entry.path());
                }
            }
        }
        None
    }

    fn fingerprint(&self) -> Result<String> {
        let mut h = Sha1::new();

        fn feed(h: &mut Sha1, rel: &str, mut entries: Vec<(String, u64)>) {
            entries.sort();
            for (name, size) in entries {
                h.update(format!("{rel}/{name}:{size}\n").as_bytes());
            }
        }
        fn name_and_size(p: &Path) -> Option<(String, u64)> {
            let size = p.metadata().ok()?.len();
            Some((p.file_name()?.to_string_lossy().into_owned(), size))
        }
        fn top_level(dir: &Path) -> Vec<(String, u64)> {
            let mut out = Vec::new();
            if let Ok(rd) = std::fs::read_dir(dir) {
                for p in rd.flatten() {
                    if let Some(e) = name_and_size(&p.path()) {
                        out.push(e);
                    }
                }
            }
            out
        }

        feed(
            &mut h,
            "recomp",
            vec![("entries.c".to_string(), self.entries_c.metadata()?.len())],
        );
        // recomp/functions.h and recomp/m*_functions.h
        let mut rec_entries = Vec::new();
        if let Ok(rd) = std::fs::read_dir(self.root.join("recomp")) {
            for p in rd.flatten() {
                let name = p.file_name().to_string_lossy().into_owned();
                let keep = name == "functions.h"
                    || (name.starts_with('m') && name.ends_with("_functions.h"));
                if keep {
                    if let Some(e) = name_and_size(&p.path()) {
                        rec_entries.push(e);
                    }
                }
            }
        }
        feed(&mut h, "recomp", rec_entries);

        // pipeline/symbols/*.csv
        let mut csv_entries = Vec::new();
        let sym = self.root.join("pipeline").join("symbols");
        if sym.is_dir() {
            if let Ok(rd) = std::fs::read_dir(&sym) {
                for p in rd.flatten() {
                    if p.file_name().to_string_lossy().ends_with(".csv") {
                        if let Some(e) = name_and_size(&p.path()) {
                            csv_entries.push(e);
                        }
                    }
                }
            }
        }
        feed(&mut h, "pipeline/symbols", csv_entries);

        // cro_extracted/*/symbols.tsv (one level deep, unlike the rest)
        let mut tsv_entries = Vec::new();
        let cro = self.root.join("cro_extracted");
        if cro.is_dir() {
            if let Ok(rd) = std::fs::read_dir(&cro) {
                for d in rd.flatten() {
                    let tsv = d.path().join("symbols.tsv");
                    if tsv.is_file() {
                        if let Some((_, size)) = name_and_size(&tsv) {
                            tsv_entries.push((
                                format!("{}/symbols.tsv", d.file_name().to_string_lossy()),
                                size,
                            ));
                        }
                    }
                }
            }
        }
        feed(&mut h, "cro_extracted", tsv_entries);

        // decomp/code_c/*.c
        let mut main_decomp = Vec::new();
        let code_c = self.root.join("decomp").join("code_c");
        if code_c.is_dir() {
            if let Ok(rd) = std::fs::read_dir(&code_c) {
                for p in rd.flatten() {
                    if p.file_name().to_string_lossy().ends_with(".c") {
                        if let Some(e) = name_and_size(&p.path()) {
                            main_decomp.push(e);
                        }
                    }
                }
            }
        }
        feed(&mut h, "decomp/code_c", main_decomp);

        // decomp/cro_c/<Mod>/*.c
        let mut cro_decomp = Vec::new();
        let cc = self.root.join("decomp").join("cro_c");
        if cc.is_dir() {
            if let Ok(rd) = std::fs::read_dir(&cc) {
                for d in rd.flatten() {
                    if !d.path().is_dir() {
                        continue;
                    }
                    if let Ok(files) = std::fs::read_dir(d.path()) {
                        for p in files.flatten() {
                            if p.file_name().to_string_lossy().ends_with(".c") {
                                if let Some((_, size)) = name_and_size(&p.path()) {
                                    cro_decomp.push((
                                        format!(
                                            "{}/{}",
                                            d.file_name().to_string_lossy(),
                                            p.file_name().to_string_lossy()
                                        ),
                                        size,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        feed(&mut h, "decomp/cro_c", cro_decomp);

        feed(
            &mut h,
            "pipeline/orig",
            top_level(&self.root.join("pipeline").join("orig")),
        );
        feed(
            &mut h,
            "pipeline/out",
            top_level(&self.root.join("pipeline").join("out")),
        );

        // armcc-matching evidence: per-function candidate files + the
        // attempted log — every new match must trigger a live reload
        if let Some(dir) = find_armcc_dir(&self.root) {
            for sub in ["matched", "relaxed"] {
                let mut mods: Vec<(String, PathBuf)> = Vec::new();
                if let Ok(entries) = std::fs::read_dir(dir.join(sub)) {
                    for d in entries.flatten() {
                        if d.path().is_dir() {
                            mods.push((d.file_name().to_string_lossy().into_owned(), d.path()));
                        }
                    }
                }
                mods.sort();
                for (name, d) in mods {
                    let mut files = Vec::new();
                    if let Ok(entries) = std::fs::read_dir(&d) {
                        for p in entries.flatten() {
                            if let Some(e) = name_and_size(&p.path()) {
                                files.push(e);
                            }
                        }
                    }
                    feed(&mut h, &format!("armcc/{sub}/{name}"), files);
                }
            }
            if let Some((_, size)) = name_and_size(&dir.join("attempted.md")) {
                feed(&mut h, "armcc", vec![("attempted.md".to_string(), size)]);
            }
        }

        let digest = h.finalize();
        Ok(hex(&digest)[..16].to_string())
    }

    /// [(name, m-number)] from the entries.c module table; Main implied.
    fn module_table(&self) -> Result<Vec<(String, u32)>> {
        static MODULE_ROW: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        let re = MODULE_ROW.get_or_init(|| {
            regex::Regex::new(r#"\{"([A-Za-z0-9_]+)", &m(\d+)_base, (0x[0-9A-Fa-f]+)u, (\d+),"#)
                .unwrap()
        });
        let text = self.read_lossy("recomp/entries.c")?;
        let mut rows = Vec::new();
        for cap in re.captures_iter(&text) {
            rows.push((
                cap[1].to_string(),
                cap[2].parse::<u32>().expect("regex guarantees digits"),
            ));
        }
        if rows.is_empty() {
            return Err(HeatmapError::new(format!(
                "{}: no module table rows found — the file layout does not match the \
                 expected 3dsrecomp output",
                self.entries_c.display()
            )));
        }
        Ok(rows)
    }

    /// m-number (0 = Main) -> set of covered start addresses.
    fn covered_starts(rec_dir: &Path) -> BTreeMap<u32, std::collections::BTreeSet<u64>> {
        static FUNC_DECL: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        let re = FUNC_DECL.get_or_init(|| {
            regex::Regex::new(r"^void (?:m(\d+)_)?f_([0-9A-Fa-f]+)\(Context").unwrap()
        });
        let mut covered: BTreeMap<u32, std::collections::BTreeSet<u64>> = BTreeMap::new();
        let read = |p: &Path| -> String {
            String::from_utf8_lossy(&std::fs::read(p).unwrap_or_default()).into_owned()
        };
        let main_h = rec_dir.join("functions.h");
        if main_h.is_file() {
            let starts = covered.entry(0).or_default();
            for line in read(&main_h).lines() {
                if let Some(cap) = re.captures(line.trim()) {
                    if cap.get(1).is_none() {
                        starts.insert(u64::from_str_radix(&cap[2], 16).unwrap_or(0));
                    }
                }
            }
        }
        let mut headers: Vec<PathBuf> = match std::fs::read_dir(rec_dir) {
            Ok(rd) => rd
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.file_name()
                        .and_then(|f| f.to_str())
                        .map(|f| f.starts_with('m') && f.ends_with("_functions.h"))
                        .unwrap_or(false)
                })
                .collect(),
            Err(_) => Vec::new(),
        };
        headers.sort();
        for p in headers {
            let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            let mnum: u32 = match stem
                .strip_prefix('m')
                .and_then(|s| s.strip_suffix("_functions"))
            {
                Some(num) => match num.parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                },
                None => continue,
            };
            let starts = covered.entry(mnum).or_default();
            for line in read(&p).lines() {
                if let Some(cap) = re.captures(line.trim()) {
                    let declared: u32 = cap
                        .get(1)
                        .map(|g| g.as_str().parse().unwrap_or(u32::MAX))
                        .unwrap_or(u32::MAX);
                    if cap.get(1).is_some() && declared == mnum {
                        starts.insert(u64::from_str_radix(&cap[2], 16).unwrap_or(0));
                    }
                }
            }
        }
        covered
    }

    fn load_csv(&self, path: &Path) -> Result<BTreeMap<u64, Acc>> {
        let text = String::from_utf8_lossy(&std::fs::read(path)?).into_owned();
        let mut lines = text.lines();
        let header = lines.next().unwrap_or("");
        let header_fields = parse_csv_line(header);
        if header_fields.first().map(|s| s.trim_matches('"')) != Some("Location") {
            return Err(HeatmapError::new(format!(
                "{}: unexpected CSV header {header:?}",
                path.display()
            )));
        }
        let mut out: BTreeMap<u64, Acc> = BTreeMap::new();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let row = parse_csv_line(line);
            if row.len() < 4 {
                continue;
            }
            let Some(addr) = parse_hex16(row[0].trim_matches('"')) else {
                continue;
            };
            let size = if row.len() > 4 {
                parse_hex16(row[4].trim_matches('"')).unwrap_or(0)
            } else {
                0
            };
            let name = &row[1];
            let namespace = &row[2];
            let acc = out.entry(addr).or_default();
            acc.size = acc.size.max(size);
            let real = !name.starts_with("FUN_") && !name.starts_with("thunk_FUN_");
            if real {
                if acc.name.is_empty() {
                    acc.name = name.clone();
                }
                acc.tier = acc.tier.max(TIER_NAMED);
                if !namespace.is_empty() && namespace != "Global" && acc.namespace.is_empty() {
                    acc.namespace = namespace.clone();
                }
            }
            // else: an anonymous FUN_ catalog row only proves the function
            // EXISTS (address + size). Tier stays raw — "decompiled" is
            // earned solely by a C file in the decomp corpus below.
        }
        Ok(out)
    }

    fn load_exports_tsv(path: &Path, column: &str) -> Result<BTreeMap<u64, Acc>> {
        let text = String::from_utf8_lossy(&std::fs::read(path)?).into_owned();
        let mut lines = text.lines();
        let header: Vec<&str> = lines
            .next()
            .unwrap_or("")
            .trim_end_matches('\r')
            .split('\t')
            .collect();
        let Some(col) = header.iter().position(|h| *h == column) else {
            return Err(HeatmapError::new(format!(
                "{}: unexpected TSV header {header:?}",
                path.display()
            )));
        };
        let Some(name_col) = header.iter().position(|h| *h == "name") else {
            return Err(HeatmapError::new(format!(
                "{}: unexpected TSV header {header:?}",
                path.display()
            )));
        };
        let mut names: BTreeMap<u64, Vec<&str>> = BTreeMap::new();
        for line in lines {
            let parts: Vec<&str> = line.trim_end_matches('\r').split('\t').collect();
            if parts.len() <= col.max(name_col) || parts[name_col].is_empty() {
                continue;
            }
            let Some(addr) = parse_hex16(parts[col]) else {
                continue;
            };
            names.entry(addr).or_default().push(parts[name_col]);
        }
        let mut accs = BTreeMap::new();
        for (addr, mut addrs) in names {
            addrs.sort();
            let mut acc = Acc::default();
            acc.name = addrs[0].to_string();
            acc.tier = TIER_NAMED;
            acc.namespace = demangle_namespace(&acc.name);
            accs.insert(addr, acc);
        }
        Ok(accs)
    }

    /// Dependency context: which modules this module imports from.
    /// `imports.tsv` rows: `module<TAB>indexed<TAB>anonymous`; the static
    /// module appears as `|static|` and is reported as `Main`.
    fn load_imports(path: &Path) -> Vec<String> {
        let text = String::from_utf8_lossy(&std::fs::read(path).unwrap_or_default()).into_owned();
        let mut lines = text.lines();
        let header = lines.next().unwrap_or("").trim_end_matches('\r');
        if header.split('\t').next() != Some("module") {
            return Vec::new();
        }
        let mut names = Vec::new();
        for line in lines {
            let first = line.trim_end_matches('\r').split('\t').next().unwrap_or("");
            if first.is_empty() {
                continue;
            }
            names.push(if first == "|static|" {
                "Main".to_string()
            } else {
                first.to_string()
            });
        }
        names.sort();
        names
    }

    fn decomp_addrs(directory: &Path) -> std::collections::BTreeSet<u64> {
        static DECOMP_NAME: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        let re = DECOMP_NAME.get_or_init(|| regex::Regex::new(r"^([0-9A-Fa-f]{8})_").unwrap());
        let mut addrs = std::collections::BTreeSet::new();
        let Ok(entries) = std::fs::read_dir(directory) else {
            return addrs;
        };
        for p in entries.flatten() {
            if let Some(cap) = re.captures(&p.file_name().to_string_lossy()) {
                addrs.insert(u64::from_str_radix(&cap[1], 16).unwrap_or(0));
            }
        }
        addrs
    }

    fn relink_map(&self) -> BTreeMap<String, bool> {
        let orig = self.root.join("pipeline").join("orig");
        let out = self.root.join("pipeline").join("out");
        let mut result = BTreeMap::new();
        let Ok(entries) = std::fs::read_dir(&orig) else {
            return result;
        };
        let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
        paths.sort();
        for p in paths {
            let Some(name) = p.file_name().and_then(|f| f.to_str()) else {
                continue;
            };
            let Some(other) = self.ci_child(&out, name) else {
                continue;
            };
            if !other.is_file() {
                continue;
            }
            let (Ok(a), Ok(b)) = (std::fs::read(&p), std::fs::read(&other)) else {
                continue;
            };
            result.insert(name.to_lowercase(), sha256_hex(&a) == sha256_hex(&b));
        }
        result
    }

    /// Load the armcc-matching evidence tree if one sits next to the data
    /// root; `None` (no warning — the matching workflow is optional) when
    /// the repo has none.
    fn load_armcc(&self) -> Option<(Armcc, PathBuf)> {
        let dir = find_armcc_dir(&self.root)?;
        let mut armcc = Armcc::default();
        for (sub, is_matched) in [("matched", true), ("relaxed", false)] {
            let Ok(entries) = std::fs::read_dir(dir.join(sub)) else {
                continue;
            };
            let mut dirs: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .collect();
            dirs.sort();
            for d in dirs {
                let Some(name) = d.file_name().and_then(|f| f.to_str()) else {
                    continue;
                };
                let funcs = scan_match_dir(&d);
                if funcs.is_empty() {
                    continue;
                }
                if is_matched {
                    armcc
                        .matched
                        .entry(name.to_lowercase())
                        .or_default()
                        .extend(funcs);
                } else {
                    // relaxed entries carry no size (the relaxed/ dirs name
                    // functions, not proof lengths)
                    armcc
                        .relaxed
                        .entry(name.to_lowercase())
                        .or_default()
                        .extend(funcs.into_iter().map(|(off, _)| off));
                }
            }
        }
        let attempted_md = dir.join("attempted.md");
        if attempted_md.is_file() {
            armcc.attempted = parse_attempted(&attempted_md);
        }
        Some((armcc, dir))
    }

    /// Module-frame address of an armcc offset: CRO matched offsets are
    /// already module-relative file offsets. Main (code.bin) offsets are
    /// accepted either as VAs (≥ 0x100000, the corpus frame) or as file
    /// offsets (< 0x100000, shifted up by the load base).
    fn armcc_addr(is_main: bool, off: u64) -> u64 {
        if is_main && off < GHIDRA_BASE {
            off + GHIDRA_BASE
        } else {
            off
        }
    }

    /// Build the full dataset (port of `ProjectSource.dataset`).
    pub fn dataset(&self) -> Result<Dataset> {
        let mut warnings: Vec<String> = Vec::new();
        let table = self.module_table()?;
        let rec_dir = self.root.join("recomp");
        let covered = Self::covered_starts(&rec_dir);

        let csv_dir = self.root.join("pipeline").join("symbols");
        let cro_dir = self.root.join("cro_extracted");
        let decomp_root = self.root.join("decomp");
        let relink = self.relink_map();
        let armcc = self.load_armcc();

        let mut modules = Vec::new();
        // Main first, then the CROs in entries.c order (builder = alphabetical).
        let mut specs: Vec<(String, u32)> = vec![(MAIN.to_string(), 0)];
        for (name, mnum) in &table {
            if name != MAIN {
                specs.push((name.clone(), *mnum));
            }
        }

        for (name, mnum) in &specs {
            let is_main = *mnum == 0;
            let mut accs: BTreeMap<u64, Acc> = BTreeMap::new();

            for addr in covered.get(mnum).into_iter().flatten() {
                accs.entry(*addr).or_default().flags |= FLAG_COVERED;
            }

            let csv_name = if is_main {
                "code.bin.csv".to_string()
            } else {
                format!("{name}.cro.csv")
            };
            let csv_path = self.ci_child(&csv_dir, &csv_name);
            match csv_path.filter(|p| p.is_file()) {
                Some(path) => {
                    for (addr, acc) in self.load_csv(&path)? {
                        let dst = accs.entry(addr).or_default();
                        dst.size = dst.size.max(acc.size);
                        dst.tier = dst.tier.max(acc.tier);
                        if dst.name.is_empty() {
                            dst.name = acc.name;
                        }
                        if dst.namespace.is_empty() {
                            dst.namespace = acc.namespace;
                        }
                    }
                }
                None => warnings.push(format!("{name}: no symbol CSV ({csv_name})")),
            }

            let mut tsv = if is_main {
                self.ci_child(&cro_dir, "static/symbols.tsv")
            } else {
                self.ci_child(&cro_dir, name)
            };
            if let (false, Some(dir)) = (is_main, tsv.as_ref()) {
                tsv = self.ci_child(dir, "symbols.tsv");
            }
            let shown = tsv
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_else(|| "symbols.tsv".to_string());
            match tsv.filter(|p| p.is_file()) {
                Some(path) => {
                    let column = if is_main { "load_addr" } else { "offset" };
                    for (mut addr, acc) in Self::load_exports_tsv(&path, column)? {
                        if is_main {
                            addr &= !1; // Thumb bit
                        }
                        let dst = accs.entry(addr).or_default();
                        dst.tier = dst.tier.max(acc.tier);
                        if dst.name.is_empty() {
                            dst.name = acc.name;
                        }
                        if dst.namespace.is_empty() {
                            dst.namespace = acc.namespace;
                        }
                    }
                }
                None => warnings.push(format!("{name}: no export symbols ({shown})")),
            }

            let dec = if is_main {
                Some(decomp_root.join("code_c"))
            } else {
                self.ci_child(&decomp_root.join("cro_c"), name)
            };
            let mut decomp = dec.map(|d| Self::decomp_addrs(&d)).unwrap_or_default();
            if !is_main && !decomp.is_empty() {
                // cro_c filenames carry absolute Ghidra addresses (each CRO
                // program was imported at the 0x100000 base); the canonical
                // CRO frame is module-relative. Anything that doesn't shift
                // cleanly is malformed — drop it rather than underflow.
                decomp = decomp
                    .into_iter()
                    .filter_map(|a| a.checked_sub(GHIDRA_BASE))
                    .collect();
            }
            if !decomp.is_empty() {
                for addr in &decomp {
                    let acc = accs.entry(*addr).or_default();
                    acc.tier = acc.tier.max(TIER_DECOMPILED);
                }
            } else {
                warnings.push(format!("{name}: no decompiled C corpus"));
            }

            // armcc-matching evidence: proven masked matches are the
            // verified tier; relaxed matches and attempted functions are
            // overlays. Offsets no other source knows still become cells —
            // prologue-less leaves are routinely missed by the Ghidra
            // function catalog (see armcc-matching/README.md).
            let lc = name.to_lowercase();
            let keys: Vec<&str> = if is_main {
                vec!["main", "static"]
            } else {
                vec![lc.as_str()]
            };
            if let Some((armcc, _)) = armcc.as_ref() {
                for key in &keys {
                    for (off, size) in armcc.matched.get(*key).into_iter().flatten() {
                        let cell = accs.entry(Self::armcc_addr(is_main, *off)).or_default();
                        cell.tier = cell.tier.max(TIER_VERIFIED);
                        if let Some(s) = size {
                            // the proven masked-compare length — the symbol
                            // CSV's guess can be wrong (literal-pool gotcha)
                            cell.size = *s;
                        }
                    }
                    for off in armcc.relaxed.get(*key).into_iter().flatten() {
                        accs.entry(Self::armcc_addr(is_main, *off))
                            .or_default()
                            .flags |= FLAG_HINT;
                    }
                }
                for (module, off) in &armcc.attempted {
                    if !keys.contains(&module.as_str()) {
                        continue;
                    }
                    let cell = accs.entry(Self::armcc_addr(is_main, *off)).or_default();
                    if cell.tier < TIER_VERIFIED {
                        // attempted.md prose also cites already-matched
                        // functions; only unproven addresses carry the
                        // "effort spent here" marker
                        cell.flags |= FLAG_ATTEMPTED;
                    }
                }
            }

            let cells: Vec<Cell> = accs
                .into_iter()
                .map(|(addr, acc)| Cell {
                    addr,
                    size: acc.size,
                    name: acc.name,
                    tier: acc.tier,
                    flags: acc.flags,
                    namespace: acc.namespace,
                })
                .collect();
            let relink_name = if is_main {
                "code.bin".to_string()
            } else {
                format!("{name}.cro")
            };
            let mut imp_path = if is_main {
                self.ci_child(&cro_dir, "static/imports.tsv")
            } else {
                self.ci_child(&cro_dir, name)
            };
            if let (false, Some(dir)) = (is_main, imp_path.as_ref()) {
                imp_path = self.ci_child(dir, "imports.tsv");
            }
            let imports = imp_path
                .filter(|p| p.is_file())
                .map(|p| Self::load_imports(&p))
                .unwrap_or_default();
            modules.push(Module {
                name: name.clone(),
                cells,
                relink_ok: relink.get(&relink_name.to_lowercase()).copied(),
                namespace: String::new(),
                imports,
            });
        }

        // armcc dirs/references naming no known module (casing, renamed
        // CROs, typos) — surface rather than silently drop
        if let Some((armcc, dir)) = armcc.as_ref() {
            let known: std::collections::HashSet<String> = specs
                .iter()
                .map(|(n, _)| n.to_lowercase())
                .chain(["main".to_string(), "static".to_string()])
                .collect();
            let mut unknown: Vec<&String> = armcc
                .matched
                .keys()
                .chain(armcc.relaxed.keys())
                .filter(|k| !known.contains(*k))
                .collect();
            unknown.sort();
            unknown.dedup();
            for k in unknown {
                warnings.push(format!(
                    "armcc-matching: matched/relaxed module {k:?} matches no known module \
                     (under {})",
                    dir.display()
                ));
            }
            let unknown_refs = armcc
                .attempted
                .iter()
                .filter(|(m, _)| !known.contains(m))
                .count();
            if unknown_refs > 0 {
                warnings.push(format!(
                    "armcc-matching attempted.md: {unknown_refs} reference(s) to unknown \
                     modules skipped"
                ));
            }
        }

        Ok(Dataset {
            title: "Pokémon X/Y recomp + decomp progress".to_string(),
            modules,
            fingerprint: self.fingerprint()?,
            warnings,
        })
    }
}

impl Source for ProjectSource {
    fn describe(&self) -> String {
        format!("project corpus at {}", self.root.display())
    }

    fn fingerprint(&self) -> Result<String> {
        ProjectSource::fingerprint(self)
    }

    fn dataset(&self) -> Result<Dataset> {
        ProjectSource::dataset(self)
    }
}

/// Serves a Dataset from one generic JSON file (the standalone path).
pub struct FileDatasetSource {
    pub path: PathBuf,
}

impl FileDatasetSource {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        if !path.is_file() {
            return Err(HeatmapError::new(format!(
                "dataset file not found: {}",
                path.display()
            )));
        }
        Ok(Self { path })
    }
}

impl Source for FileDatasetSource {
    fn describe(&self) -> String {
        format!("dataset {}", self.path.display())
    }

    fn fingerprint(&self) -> Result<String> {
        let st = std::fs::metadata(&self.path)?;
        Ok(format!("{}:{}", st.len(), StampedTime::from_metadata(&st)))
    }

    fn dataset(&self) -> Result<Dataset> {
        let mut ds = crate::dataset::load_dataset(&self.path)?;
        ds.fingerprint = Source::fingerprint(self)?;
        Ok(ds)
    }
}

/// nanosecond mtime as a string (port of `st_mtime_ns`).
struct StampedTime;
impl StampedTime {
    fn from_metadata(md: &std::fs::Metadata) -> String {
        use std::time::UNIX_EPOCH;
        md.modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_nanos().to_string())
            .unwrap_or_else(|| "0".to_string())
    }
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex(&h.finalize())
}

/// One CSV line -> fields, handling double-quoted fields with `""` escapes
/// (Ghidra symbol CSVs quote every field).
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                cur.push(c);
            }
        } else {
            match c {
                '"' => in_quotes = true,
                ',' => fields.push(std::mem::take(&mut cur)),
                _ => cur.push(c),
            }
        }
    }
    fields.push(cur);
    fields
}

#[cfg(test)]
mod tests;
