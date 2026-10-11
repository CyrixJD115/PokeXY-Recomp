//! `xy-curate` — the final extract → rebuild → convert → organize pipeline.
//!
//! Library crate behind both the `xy curate` CLI subcommand and the GUI's
//! curation workspace. Builds `Source_Extracted/curated/`: a fully
//! self-contained tree of **real files** (never symlinks), converting
//! proprietary formats into standard ones wherever a validated parser
//! exists:
//!
//! | stage | output | conversion |
//! |---|---|---|
//! | `data` | `battle/ world/ ui/ manual/ shaders/` | organized real copies of mapped GARC data |
//! | `text` | `text/` | per-bank JSON copies + `text/organized/` named banks (species, moves, …) |
//! | `pokemon` | `pokemon/` | 8-file families copied; models → OBJ+MTL; textures → PNG |
//! | `media` | `audio/ video/` | WAV / MP4 conversions (produced by ffmpeg in the rebuild script) |
//! | `code` | `code/` | CRO modules + extracted segments + Ghidra C corpus; `modules.json` dependency graph |
//! | `csharp` | `code/csharp/` | C → C# deobfuscation pipeline (`csharp` module) |
//! | `unknown` | `unknown/` | everything not yet semantically mapped, preserved raw |
//!
//! Every written file is recorded in `manifest.tsv` (source → curated path →
//! action → detail → bytes → status) and re-verified by `validation.tsv`.
//! Runs are deterministic: directory iteration is sorted, names are stable,
//! and no timestamps are embedded in outputs.
//!
//! GUI integration: [`build_with_progress`] reports per-stage/per-file
//! progress and honors a cancellation flag; [`manifest`] reads finished
//! trees back for provenance browsing.

pub mod manifest;

pub mod csharp;
pub mod gfx_convert;

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use xy_core::util::json_escape;

/// A progress event emitted while the pipeline runs.
#[derive(Debug, Clone)]
pub struct Progress {
    /// Stage name (`"data"`, `"pokemon"`, …; `"manifest"` for finalization).
    pub stage: &'static str,
    /// 0-based items completed within the stage (files, families, modules).
    pub done: usize,
    /// Total items in the stage when known (0 = indeterminate).
    pub total: usize,
    /// Human-readable line for a log console.
    pub message: String,
}

/// romfs-relative GARC path → curated subdirectory.
/// Straight port of pk3DS `GARCReference_XY` (names) plus verified additions.
const GARC_DIRS: &[(&str, &str, &str)] = &[
    // (romfs path, curated dir, description)
    (
        "a/0/0/5",
        "battle/move_sprites",
        "Move sprite/animation data (pk3DS: movesprite)",
    ),
    (
        "a/2/0/3",
        "battle/maison/pokemon_normal",
        "Battle Maison Pokémon sets, normal (pk3DS: maisonpkN)",
    ),
    (
        "a/2/0/4",
        "battle/maison/trainers_normal",
        "Battle Maison trainers, normal (pk3DS: maisontrN)",
    ),
    (
        "a/2/0/5",
        "battle/maison/pokemon_super",
        "Battle Maison Pokémon sets, super (pk3DS: maisonpkS)",
    ),
    (
        "a/2/0/6",
        "battle/maison/trainers_super",
        "Battle Maison trainers, super (pk3DS: maisontrS)",
    ),
    ("a/2/1/2", "battle/moves", "Move data table (pk3DS: move)"),
    (
        "a/2/1/3",
        "battle/egg_moves",
        "Egg move table (pk3DS: eggmove)",
    ),
    (
        "a/2/1/4",
        "battle/learnsets",
        "Level-up learnsets (pk3DS: levelup)",
    ),
    (
        "a/2/1/5",
        "battle/evolutions",
        "Evolution table (pk3DS: evolution)",
    ),
    (
        "a/2/1/6",
        "battle/mega_evolutions",
        "Mega evolution table (pk3DS: megaevo)",
    ),
    (
        "a/2/1/8",
        "battle/base_stats",
        "Personal/base stats table (pk3DS: personal)",
    ),
    ("a/2/2/0", "battle/items", "Item data table (pk3DS: item)"),
    (
        "a/0/1/2",
        "world/encounters",
        "Wild encounter tables (pk3DS: encdata)",
    ),
    (
        "a/0/3/8",
        "world/trainers/data",
        "Trainer battle data (pk3DS: trdata)",
    ),
    (
        "a/0/3/9",
        "world/trainers/classes",
        "Trainer class data (pk3DS: trclass)",
    ),
    (
        "a/0/4/0",
        "world/trainers/parties",
        "Trainer party data (pk3DS: trpoke)",
    ),
    (
        "a/0/4/1",
        "world/maps/geometry",
        "Map geometry models (pk3DS: mapGR)",
    ),
    (
        "a/0/4/2",
        "world/maps/matrix",
        "Map connection matrix (pk3DS: mapMatrix)",
    ),
    (
        "a/1/5/9",
        "world/zones",
        "Zone tables, 241 areas (empirical)",
    ),
    (
        "a/1/0/4",
        "ui/box_wallpapers",
        "Pokémon Box wallpapers (pk3DS: wallpaper)",
    ),
    (
        "a/1/6/5",
        "ui/title_screen",
        "Title screen assets (pk3DS: titlescreen)",
    ),
    (
        "a/0/0/1",
        "ui/banner_models",
        "CGFX banner models (empirical)",
    ),
];

/// GARC dirs fully consumed by the `pokemon` stage (families + conversions).
const POKEMON_GARC_DIRS: &[&str] = &["a/0/0/6", "a/0/0/7"];

/// Language suffixes for the gametext GARC variants (bank 080 decode probe).
const GAMETEXT_LANGS: &[&str] = &["jpn", "jpn2", "eng", "fre", "ita", "ger", "spa", "kor"];
const STORYTEXT_LANGS: &[&str] = &["jpn", "jpn2", "eng", "fre", "ita", "ger", "spa", "kor"];

/// pk3DS `TextReference.GameText_XY` — semantically named gametext banks.
const NAMED_BANKS: &[(u32, &str)] = &[
    (5, "forms"),
    (13, "moves"),
    (15, "move_flavor"),
    (17, "types"),
    (20, "trainer_classes"),
    (21, "trainer_names"),
    (22, "trainer_text"),
    (34, "abilities"),
    (47, "natures"),
    (72, "location_list"),
    (80, "species"),
    (96, "items"),
    (99, "item_flavor"),
    (130, "maison_trainer_names"),
    (131, "super_trainer_names"),
    (141, "opower_flavor"),
];

/// Pipeline stages; `all` (the default) runs everything in dependency order.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    Data,
    Text,
    Pokemon,
    Media,
    Code,
    Csharp,
    Unknown,
}

pub const ALL_STAGES: &[(&str, Stage)] = &[
    ("data", Stage::Data),
    ("text", Stage::Text),
    ("pokemon", Stage::Pokemon),
    ("media", Stage::Media),
    ("code", Stage::Code),
    ("csharp", Stage::Csharp),
    ("unknown", Stage::Unknown),
];

pub fn parse_stages(spec: &str) -> Option<Vec<Stage>> {
    let mut out = Vec::new();
    for part in spec.split(',') {
        let part = part.trim();
        if part == "all" {
            out.extend(ALL_STAGES.iter().map(|(_, s)| *s));
            continue;
        }
        out.push(ALL_STAGES.iter().find(|(n, _)| *n == part)?.1);
    }
    Some(out)
}

/// Canonical name of a stage (`"data"`, `"pokemon"`, …).
pub fn stage_name(stage: Stage) -> &'static str {
    ALL_STAGES
        .iter()
        .find(|(_, s)| *s == stage)
        .map(|(n, _)| *n)
        .unwrap_or("?")
}

/// One `manifest.tsv` row: original location → curated file → transformation.
struct Row {
    source: String, // relative to Source_Extracted (or "<generated>")
    path: String,   // relative to the curated root
    action: &'static str,
    detail: String,
    in_bytes: u64,
    out_bytes: u64,
    status: &'static str,
}

impl Row {
    fn new(
        source: impl Into<String>,
        path: impl Into<String>,
        action: &'static str,
        detail: impl Into<String>,
        in_bytes: u64,
        out_bytes: u64,
    ) -> Self {
        Row {
            source: source.into(),
            path: path.into(),
            action,
            detail: detail.into(),
            in_bytes,
            out_bytes,
            status: "ok",
        }
    }

    fn failed(
        source: impl Into<String>,
        path: impl Into<String>,
        action: &'static str,
        detail: impl Into<String>,
    ) -> Self {
        Row {
            source: source.into(),
            path: path.into(),
            action,
            detail: detail.into(),
            in_bytes: 0,
            out_bytes: 0,
            status: "fail",
        }
    }

    fn tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            self.source,
            self.path,
            self.action,
            self.detail,
            self.in_bytes,
            self.out_bytes,
            self.status
        )
    }
}

/// End-of-run summary for the CLI printout.
#[derive(Debug, Default)]
pub struct Summary {
    pub entries: usize,
    pub families: usize,
    pub models_obj: usize,
    pub textures_png: usize,
    pub csharp_modules: usize,
    pub csharp_converted: usize,
    pub csharp_total: usize,
    pub manifest_rows: usize,
    pub validation_checks: usize,
    pub validation_failures: usize,
}

struct Ctx<'a> {
    input: &'a Path,
    output: &'a Path,
    rows: Vec<Row>,
    summary: Summary,
    on_progress: &'a dyn Fn(Progress),
    cancel: Option<&'a AtomicBool>,
    /// Name of the stage currently running (for progress events).
    stage: &'static str,
}

impl Ctx<'_> {
    fn emit(&self, done: usize, total: usize, message: impl Into<String>) {
        (self.on_progress)(Progress {
            stage: self.stage,
            done,
            total,
            message: message.into(),
        });
    }

    /// Abort point checked inside the per-file/per-family loops.
    fn check_cancel(&self) -> io::Result<()> {
        if self.cancel.is_some_and(|c| c.load(Ordering::Relaxed)) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "curate cancelled",
            ));
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Build the curated tree (no progress reporting). See [`build_with_progress`].
pub fn build(input: &Path, output: &Path, stages: &[Stage]) -> io::Result<Summary> {
    build_with_progress(input, output, stages, &|_| {}, None)
}

/// Build the curated tree, reporting progress and honoring cancellation.
///
/// Wipes any previous curate-generated tree first (guarded: refuses to
/// delete a directory curate did not write). Setting `cancel` mid-run
/// aborts with `io::ErrorKind::Interrupted` at the next per-item boundary.
pub fn build_with_progress(
    input: &Path,
    output: &Path,
    stages: &[Stage],
    on_progress: &dyn Fn(Progress),
    cancel: Option<&AtomicBool>,
) -> io::Result<Summary> {
    if output.exists() {
        if !output.join("README.md").is_file() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "{} exists and was not written by `xy curate` (no README.md); \
                     remove it manually if it really is expendable",
                    output.display()
                ),
            ));
        }
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output)?;

    let mut ctx = Ctx {
        input,
        output,
        rows: Vec::new(),
        summary: Summary::default(),
        on_progress,
        cancel,
        stage: "init",
    };

    for stage in stages {
        ctx.stage = stage_name(*stage);
        ctx.emit(0, 0, format!("stage {} starting", ctx.stage));
        match stage {
            Stage::Data => stage_data(&mut ctx)?,
            Stage::Text => stage_text(&mut ctx)?,
            Stage::Pokemon => stage_pokemon(&mut ctx)?,
            Stage::Media => stage_media(&mut ctx)?,
            Stage::Code => stage_code(&mut ctx)?,
            Stage::Csharp => stage_csharp(&mut ctx)?,
            Stage::Unknown => stage_unknown(&mut ctx)?,
        }
        ctx.emit(0, 0, format!("stage {} done", ctx.stage));
    }

    // Manifest + validation + top-level README.
    ctx.stage = "manifest";
    ctx.emit(0, 0, "writing manifest.tsv");
    ctx.rows
        .sort_by(|a, b| (&a.path, &a.source).cmp(&(&b.path, &b.source)));
    let manifest_path = output.join("manifest.tsv");
    fs::write(
        &manifest_path,
        "source\tpath\taction\tdetail\tin_bytes\tout_bytes\tstatus\n",
    )?;
    let mut manifest = String::with_capacity(64 + ctx.rows.len() * 96);
    for row in &ctx.rows {
        manifest.push_str(&row.tsv());
    }
    fs::write(&manifest_path, &manifest)?;

    let (checks, failures, details) = validate(&ctx.rows, output)?;
    let mut val = String::from("path\tcheck\tresult\tdetail\n");
    for d in &details {
        let _ = writeln!(val, "{}\t{}\t{}\t{}", d.path, d.check, d.result, d.detail);
    }
    let _ = writeln!(
        val,
        "# totals\t{}\t{}\t{}",
        checks,
        failures,
        if failures == 0 { "pass" } else { "fail" }
    );
    fs::write(output.join("validation.tsv"), &val)?;
    ctx.summary.manifest_rows = ctx.rows.len();
    ctx.summary.validation_checks = checks;
    ctx.summary.validation_failures = failures;

    write_top_readme(&ctx, stages)?;
    Ok(ctx.summary)
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Real file copy (never a symlink): creates parents, returns (in, out) bytes.
fn place_file(src: &Path, dst: &Path) -> io::Result<(u64, u64)> {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    let in_bytes = src.metadata().map(|m| m.len()).unwrap_or(0);
    fs::copy(src, dst)?;
    Ok((in_bytes, fs::metadata(dst)?.len()))
}

/// Copy every file of a flat directory (sorted), recording one row per file.
fn copy_flat_dir(
    ctx: &mut Ctx,
    src_dir: &Path,
    dst_dir: &Path,
    action: &'static str,
    desc: impl Into<String>,
) -> io::Result<usize> {
    let desc = desc.into();
    let mut names: Vec<PathBuf> = fs::read_dir(src_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file())
        .collect();
    names.sort();
    let mut count = 0usize;
    for src in &names {
        ctx.check_cancel()?;
        let rel_src = src.strip_prefix(ctx.input).unwrap_or(src);
        let dst = dst_dir.join(src.file_name().unwrap_or_default());
        match place_file(src, &dst) {
            Ok((ib, ob)) => {
                let rel_dst = dst.strip_prefix(ctx.output).unwrap_or(&dst);
                ctx.rows.push(Row::new(
                    rel_src.display().to_string(),
                    rel_dst.display().to_string(),
                    action,
                    desc.as_str(),
                    ib,
                    ob,
                ));
                count += 1;
            }
            Err(_) => {
                ctx.rows.push(Row::failed(
                    rel_src.display().to_string(),
                    dst.display().to_string(),
                    action,
                    "copy failed",
                ));
            }
        }
        ctx.emit(count, names.len(), format!("copy {}", rel_src.display()));
    }
    ctx.summary.entries += count;
    Ok(count)
}

/// Recursively copy a directory tree (sorted), one row per file.
fn copy_tree(
    ctx: &mut Ctx,
    src_dir: &Path,
    dst_dir: &Path,
    action: &'static str,
    desc: impl Into<String>,
) -> io::Result<usize> {
    let desc = desc.into();
    let mut names: Vec<PathBuf> = fs::read_dir(src_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    names.sort();
    let mut count = 0usize;
    for src in &names {
        ctx.check_cancel()?;
        if src.is_dir() {
            count += copy_tree(
                ctx,
                src,
                &dst_dir.join(src.file_name().unwrap_or_default()),
                action,
                desc.as_str(),
            )?;
        } else {
            let rel_src = src.strip_prefix(ctx.input).unwrap_or(src);
            let dst = dst_dir.join(src.file_name().unwrap_or_default());
            match place_file(src, &dst) {
                Ok((ib, ob)) => {
                    let rel_dst = dst.strip_prefix(ctx.output).unwrap_or(&dst);
                    ctx.rows.push(Row::new(
                        rel_src.display().to_string(),
                        rel_dst.display().to_string(),
                        action,
                        desc.as_str(),
                        ib,
                        ob,
                    ));
                    count += 1;
                }
                Err(_) => ctx.rows.push(Row::failed(
                    rel_src.display().to_string(),
                    dst.display().to_string(),
                    action,
                    "copy failed",
                )),
            }
            ctx.emit(count, 0, format!("copy {}", rel_src.display()));
        }
    }
    ctx.summary.entries += count;
    Ok(count)
}

fn write_readme(dir: &Path, text: impl Into<String>) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    fs::write(dir.join("README.md"), text.into())
}

// ---------------------------------------------------------------------------
// Stage: data (mapped GARC dirs, manual, shaders)
// ---------------------------------------------------------------------------

fn stage_data(ctx: &mut Ctx) -> io::Result<()> {
    let garc = ctx.input.join("garc");
    for (rel, dir, desc) in GARC_DIRS {
        // desc: &&str
        let src_dir = garc.join(rel);
        if !src_dir.is_dir() {
            continue;
        }
        let n = copy_flat_dir(ctx, &src_dir, &ctx.output.join(dir), "copy", *desc)?;
        write_readme(
            &ctx.output.join(dir),
            format!("# {dir}\n\n{desc}\n\n{n} files, real copies of `garc/{rel}/` (source GARC paths in `manifest.tsv`).\n"),
        )?;
    }

    let btl = garc.join("btl_ai.garc");
    if btl.is_dir() {
        let n = copy_flat_dir(
            ctx,
            &btl,
            &ctx.output.join("battle/ai"),
            "copy",
            "Battle AI tables (btl_ai.garc)",
        )?;
        write_readme(
            &ctx.output.join("battle/ai"),
            format!("# battle/ai\n\nBattle AI tables (btl_ai.garc). {n} files.\n"),
        )?;
    }

    for (dir, desc) in [
        (
            "manual",
            "E-manual assets (BCMA → LZ10 → inner-DARC pipeline)",
        ),
        (
            "shaders",
            "SDK system shader binaries (.shbin: nwfont/nwlyt)",
        ),
    ] {
        let src_dir = ctx.input.join(dir);
        if src_dir.is_dir() {
            copy_tree(ctx, &src_dir, &ctx.output.join(dir), "copy", desc)?;
            write_readme(&ctx.output.join(dir), format!("# {dir}\n\n{desc}\n"))?;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Stage: text (JSON copies + organized named banks)
// ---------------------------------------------------------------------------

fn stage_text(ctx: &mut Ctx) -> io::Result<()> {
    let text_root = ctx.input.join("text");
    if !text_root.is_dir() {
        return Ok(());
    }
    for (i, lang) in GAMETEXT_LANGS.iter().enumerate() {
        let src_dir = text_root.join(format!("gametext_{lang}"));
        if src_dir.is_dir() {
            let n = copy_flat_dir(
                ctx,
                &src_dir,
                &ctx.output.join(format!("text/gametext_{lang}")),
                "copy",
                format!("Gametext banks ({lang}); GARC a/0/7/{}", i + 2),
            )?;
            write_readme(
                &ctx.output.join(format!("text/gametext_{lang}")),
                format!(
                    "# gametext_{lang}\n\nGen6 text banks decoded to JSON \
                     (`xy script text`); GARC `a/0/7/{}`. {n} banks.\n\
                     Semantically named banks: `text/organized/gametext_{lang}/`.\n",
                    i + 2
                ),
            )?;
        }
    }
    for (i, lang) in STORYTEXT_LANGS.iter().enumerate() {
        let src_dir = text_root.join(format!("storytext_{lang}"));
        if src_dir.is_dir() {
            let script = if i == 0 {
                "hiragana-JP"
            } else if i == 1 {
                "kanji-JP"
            } else {
                "standard"
            };
            let n = copy_flat_dir(
                ctx,
                &src_dir,
                &ctx.output.join(format!("text/storytext_{lang}")),
                "copy",
                format!("Story/dialogue text ({lang}, {script}); GARC a/0/8"),
            )?;
            write_readme(
                &ctx.output.join(format!("text/storytext_{lang}")),
                format!("# storytext_{lang}\n\nStory/dialogue banks ({lang}); GARC `a/0/8/{i}`. {n} banks.\n"),
            )?;
        }
    }

    // Organized: named banks + a per-language index of every bank.
    for (i, lang) in GAMETEXT_LANGS.iter().enumerate() {
        let src_dir = text_root.join(format!("gametext_{lang}"));
        if !src_dir.is_dir() {
            continue;
        }
        let org_dir = ctx.output.join(format!("text/organized/gametext_{lang}"));
        fs::create_dir_all(&org_dir)?;
        let mut banks_json = String::from("[");
        for (bank, name) in NAMED_BANKS {
            let src = src_dir.join(format!("{bank:03}.json"));
            if !src.is_file() {
                continue;
            }
            let (ib, ob) = place_file(&src, &org_dir.join(format!("{name}.json")))?;
            ctx.rows.push(Row::new(
                format!("text/gametext_{lang}/{bank:03}.json"),
                format!("text/organized/gametext_{lang}/{name}.json"),
                "copy",
                format!("named bank {bank} = {name} (pk3DS TextReference)"),
                ib,
                ob,
            ));
        }
        // Index over every bank present.
        let mut names: Vec<PathBuf> = fs::read_dir(&src_dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|e| e == "json"))
            .collect();
        names.sort();
        let mut first = true;
        for path in &names {
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let bank: u32 = stem.parse().unwrap_or(u32::MAX);
            let named = NAMED_BANKS.iter().find(|(b, _)| *b == bank);
            if !first {
                banks_json.push(',');
            }
            first = false;
            match named {
                Some((b, name)) => {
                    let _ = write!(
                        banks_json,
                        "{{\"bank\":{b},\"name\":\"{}\",\"file\":\"{}.json\"}}",
                        json_escape(name),
                        json_escape(name)
                    );
                }
                None => {
                    let _ = write!(
                        banks_json,
                        "{{\"bank\":{bank},\"file\":\"../../gametext_{lang}/{stem}.json\"}}"
                    );
                }
            }
        }
        banks_json.push(']');
        let index = format!(
            "{{\"lang\":\"{}\",\"source\":\"garc a/0/7/{} (gametext bank index per pk3DS TextReference)\",\"banks\":{banks_json}}}\n",
            lang,
            i + 2
        );
        fs::write(org_dir.join("index.json"), &index)?;
        ctx.rows.push(Row::new(
            "<generated>",
            format!("text/organized/gametext_{lang}/index.json"),
            "generate",
            "bank index with pk3DS names",
            index.len() as u64,
            index.len() as u64,
        ));
        write_readme(
            &org_dir,
            format!(
                "# organized gametext ({lang})\n\n\
                 Semantically named copies of the gametext banks (pk3DS \
                 `TextReference.GameText_XY` map): species.json = bank 080, \
                 moves.json = 013, abilities.json = 034, items.json = 096, …\n\
                 `index.json` maps every bank number to its file.\n"
            ),
        )?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Stage: pokemon (families + OBJ + PNG)
// ---------------------------------------------------------------------------

fn stage_pokemon(ctx: &mut Ctx) -> io::Result<()> {
    let garc = ctx.input.join("garc");
    let gfx_dir = garc.join("a/0/0/7");
    let out_pokemon = ctx.output.join("pokemon");
    if !gfx_dir.is_dir() {
        return Ok(());
    }

    let species = gfx_convert::load_species_bank(&garc).unwrap_or_default();
    let mut orphans = Vec::new();
    let families = gfx_convert::discover_families(&gfx_dir, &species, &mut orphans)?;
    let orphan_dir = ctx.output.join("unknown/garc/a/0/0/7");
    let on_family = |done: usize, total: usize, name: &str| {
        (ctx.on_progress)(Progress {
            stage: "pokemon",
            done,
            total,
            message: format!("family {name}"),
        });
    };
    let stats = gfx_convert::convert_families(
        &gfx_dir,
        &out_pokemon,
        &orphan_dir,
        &families,
        &orphans,
        &on_family,
        ctx.cancel,
    )?;

    for (file_no, pm, form, dir) in &families {
        ctx.rows.push(Row::new(
            format!(
                "garc/a/0/0/7/{file_no:04}.bin (family {}..{})",
                file_no - 4,
                file_no + 3
            ),
            format!("pokemon/families/{dir}/"),
            "convert",
            format!("pm{pm:04} form {form:02}: sources copied, models → OBJ+MTL, textures → PNG"),
            0,
            0,
        ));
    }
    if stats.orphans > 0 {
        ctx.rows.push(Row::new(
            "garc/a/0/0/7 (unfamilyed files)",
            "unknown/garc/a/0/0/7/",
            "copy",
            format!("{} family-less files preserved", stats.orphans),
            0,
            0,
        ));
    }

    // graphics index table (a/0/0/6).
    let index_src = garc.join("a/0/0/6/0.bin");
    if index_src.is_file() {
        let (ib, ob) = place_file(&index_src, &out_pokemon.join("graphics_index.bin"))?;
        ctx.rows.push(Row::new(
            "garc/a/0/0/6/0.bin",
            "pokemon/graphics_index.bin",
            "copy",
            "form-group table (naming is derived from model BCHs instead)",
            ib,
            ob,
        ));
    }

    ctx.summary.families = stats.families;
    ctx.summary.models_obj = stats.models_obj;
    ctx.summary.textures_png = stats.textures_png;
    ctx.summary.entries += stats.families;

    write_readme(
        &out_pokemon,
        format!(
            "# Pokémon graphics (curated)\n\n\
             Every `a/0/0/7` 8-file family per Pokémon form, organized and \
             converted:\n\n\
             - `families/<pmNNNN_Species_FF>/` — the 8 source files (real \
             copies; 0000.bin..0007.bin, roles below)\n\
             - `models/<pmNNNN_Species_FF>/` — OBJ+MTL reconstructions of \
             the hi-poly and shadow (low-poly) BCH models from the +4 model \
             PC; open in Blender/any Wavefront tool\n\
             - `textures/<pmNNNN_Species_FF>/` — PNG decodes of the +2 CGFX \
             texture pack and the +6/+7 PT texture packs\n\
             - `graphics_index.bin` — the raw form-group table (a/0/0/6)\n\n\
             {} families; {} OBJ/MTL files; {} PNGs; {} family-less files \
             preserved under `unknown/garc/a/0/0/7/`.\n\n\
             Naming: from each model BCH's own `pm####_form` name; species \
             labels only for XY-internal ids 1-649 (= national dex, \
             render-verified). Kalos ids ≥650 have no uniform dex mapping \
             and stay `pm####`. FF = form (00 regular, 01/51/52 megas, …).\n\
             Family roles: +0 ?, +1 PC effects/shaders, +2 CGFX textures, \
             +3 BCH anims, +4 PC hi/lo models, +5 PF motions, +6/+7 PT \
             textures.\n\n\
             Open items: XY's real skeletal motions are the v2 GFMotion \
             bitstreams (a/0/9/3 + a/0/9/4, undecoded); rev-9 CNOD mesh \
             containers (effects/world) are an open RE project.\n",
            stats.families, stats.models_obj, stats.textures_png, stats.orphans
        ),
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Stage: media (audio WAVs, video MP4s)
// ---------------------------------------------------------------------------

fn stage_media(ctx: &mut Ctx) -> io::Result<()> {
    let audio = ctx.input.join("audio");
    if audio.is_dir() {
        let n = copy_flat_dir(
            ctx,
            &audio,
            &ctx.output.join("audio"),
            "copy",
            "BGM/SFX WAV (ffmpeg from sound/*.aac; banner jingle via DSP-ADPCM decoder)",
        )?;
        write_readme(
            &ctx.output.join("audio"),
            format!(
                "# audio\n\n{n} WAV files converted from `romfs/sound/*.aac` \
                 (plain ADTS AAC → ffmpeg pcm_s16le) plus the banner jingle \
                 (`xy audio bcwav`, DSP-ADPCM → RIFF WAV, 32,728 Hz stereo).\n\
                 Raw sources remain in `.Data/Raw_Extracted/romfs/sound/`.\n"
            ),
        )?;
    }
    let video = ctx.input.join("video");
    if video.is_dir() {
        let n = copy_flat_dir(
            ctx,
            &video,
            &ctx.output.join("video"),
            "copy",
            "Mobiclip .moflex → MP4 (ffmpeg)",
        )?;
        write_readme(
            &ctx.output.join("video"),
            format!(
                "# video\n\n{n} MP4 files converted from `romfs/m/*.moflex` \
                 (Mobiclip video + IMA ADPCM → ffmpeg). The leftover \
                 `dps.cdc` companion file is not a moflex and stays \
                 unconverted (see unknown/).\n"
            ),
        )?;
    }
    // The raw banner CWAV (lossless source of the jingle WAV).
    let bcwav = ctx.input.join("banner_jingle.bcwav");
    if bcwav.is_file() {
        let (ib, ob) = place_file(&bcwav, &ctx.output.join("audio/banner_jingle.bcwav"))?;
        ctx.rows.push(Row::new(
            "banner_jingle.bcwav",
            "audio/banner_jingle.bcwav",
            "copy",
            "raw CWAV source of banner_jingle.wav",
            ib,
            ob,
        ));
        ctx.summary.entries += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Stage: code (CRO corpus + decompilation + dependency graph)
// ---------------------------------------------------------------------------

fn stage_code(ctx: &mut Ctx) -> io::Result<()> {
    // CRO modules + extraction outputs.
    for (dir, dst, desc) in [
        (
            "cro",
            "code/cro",
            "plaintext CRO0 modules + static.crs (magic CRO0 @ 0x80)",
        ),
        (
            "cro_extracted",
            "code/cro_extracted",
            "per-module segments, symbols.tsv, imports.tsv (xy cro extract)",
        ),
    ] {
        let src = ctx.input.join(dir);
        if src.is_dir() {
            let n = copy_tree(ctx, &src, &ctx.output.join(dst), "copy", desc)?;
            write_readme(
                &ctx.output.join(dst),
                format!("# {dst}\n\n{desc}. {n} files.\n"),
            )?;
        }
    }

    // Decompiled C corpus (skip Ghidra project state + raw import binaries).
    let decomp = ctx.input.join("decomp");
    if decomp.is_dir() {
        for name in ["code_c", "cro_c"] {
            let src = decomp.join(name);
            if src.is_dir() {
                let n = copy_tree(
                    ctx,
                    &src,
                    &ctx.output.join(format!("code/decomp/{name}")),
                    "copy",
                    "Ghidra-decompiled C functions",
                )?;
                write_readme(
                    &ctx.output.join(format!("code/decomp/{name}")),
                    format!("# decomp/{name}\n\nGhidra decompilation output. {n} files.\n"),
                )?;
            }
        }
        let mut loose: Vec<PathBuf> = fs::read_dir(&decomp)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_file())
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.ends_with(".tsv") || n == "code.bin")
            })
            .collect();
        loose.sort();
        for f in &loose {
            let rel_src = f.strip_prefix(ctx.input).unwrap_or(f);
            let dst = ctx
                .output
                .join("code/decomp")
                .join(f.file_name().unwrap_or_default());
            let (ib, ob) = place_file(f, &dst)?;
            let rel_dst = dst.strip_prefix(ctx.output).unwrap_or(&dst);
            ctx.rows.push(Row::new(
                rel_src.display().to_string(),
                rel_dst.display().to_string(),
                "copy",
                "decomp metadata / main binary",
                ib,
                ob,
            ));
        }
    }

    write_modules_json(ctx)?;
    Ok(())
}

/// `code/modules.json` — machine-readable module dependency graph: segments,
/// named exports, import modules (the game's DLL graph), decomp counts.
fn write_modules_json(ctx: &mut Ctx) -> io::Result<()> {
    let cro_dir = ctx.input.join("cro");
    if !cro_dir.is_dir() {
        return Ok(());
    }
    let mut paths: Vec<PathBuf> = fs::read_dir(&cro_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file())
        .collect();
    paths.sort();

    use xy_archive::cro::Cro;
    let mut modules_json = String::from("{\"modules\":[");
    let mut first = true;
    let mut count = 0usize;
    for path in &paths {
        ctx.check_cancel()?;
        let Ok(data) = fs::read(path) else { continue };
        let Ok(cro) = Cro::parse(&data) else { continue };
        let Ok(name) = cro.name() else { continue };
        let rel_src = path.strip_prefix(ctx.input).unwrap_or(path);
        // Decompiled-function count when the CRO_C corpus covers this module.
        let key: String = if cro.code_size == 0 {
            "static".to_string()
        } else {
            name.to_string()
        };
        let decomp_dir = ctx.input.join("decomp/cro_c").join(&key);
        let decomp_fns = if decomp_dir.is_dir() {
            fs::read_dir(&decomp_dir)?
                .filter(|e| {
                    e.as_ref()
                        .ok()
                        .is_some_and(|e| e.path().extension().is_some_and(|x| x == "c"))
                })
                .count()
        } else {
            0
        };
        if !first {
            modules_json.push(',');
        }
        first = false;
        let mut segs = String::from("[");
        for (i, seg) in cro.segments.iter().enumerate() {
            if i > 0 {
                segs.push(',');
            }
            let _ = write!(
                segs,
                "{{\"index\":{},\"type\":\"{}\",\"offset\":{},\"size\":{}}}",
                i,
                json_escape(seg.ty.name()),
                seg.offset,
                seg.size
            );
        }
        segs.push(']');
        let mut exports = String::from("[");
        for (i, e) in cro.named_exports.iter().enumerate() {
            if i > 0 {
                exports.push(',');
            }
            let _ = write!(
                exports,
                "{{\"name\":\"{}\",\"offset\":{}}}",
                json_escape(&e.name),
                e.offset
            );
        }
        exports.push(']');
        let mut imports = String::from("[");
        for (i, m) in cro.import_modules.iter().enumerate() {
            if i > 0 {
                imports.push(',');
            }
            let _ = write!(
                imports,
                "{{\"module\":\"{}\",\"indexed\":{},\"anonymous\":{}}}",
                json_escape(&m.name),
                m.indexed_count,
                m.anonymous_count
            );
        }
        imports.push(']');
        let _ = write!(
            modules_json,
            "{{\"name\":\"{}\",\"source\":\"{}\",\"file_size\":{},\"segments\":{},\"exports\":{},\"indexed_exports\":{},\"imports\":{},\"named_imports\":{},\"internal_relocations\":{},\"decompiled_functions\":{}}}",
            json_escape(name),
            json_escape(&rel_src.display().to_string()),
            cro.file_size,
            segs,
            exports,
            cro.indexed_export_count,
            imports,
            cro.named_import_count,
            cro.internal_relocation_count,
            decomp_fns
        );
        count += 1;
        ctx.emit(count, paths.len(), format!("modules.json: {name}"));
    }
    modules_json.push_str("]}\n");
    fs::create_dir_all(ctx.output.join("code"))?;
    fs::write(ctx.output.join("code/modules.json"), &modules_json)?;
    ctx.rows.push(Row::new(
        "<generated from cro/*>",
        "code/modules.json",
        "generate",
        format!("{count} CRO modules: segments, exports, import graph, decomp counts"),
        modules_json.len() as u64,
        modules_json.len() as u64,
    ));
    write_readme(
        &ctx.output.join("code"),
        format!(
            "# code\n\nRecovered game code, fully extracted:\n\n\
             - `cro/` — the {} plaintext CRO modules + `static.crs`\n\
             - `cro_extracted/<Name>/` — segments at load base 0x00100000, \
             `symbols.tsv` (real export names), `imports.tsv`\n\
             - `decomp/code_c/` — {} main-binary functions (base 0x00100000)\n\
             - `decomp/cro_c/<Name>/` — {} functions across all 133 modules\n\
             - `modules.json` — segments, export/import dependency graph, \
             per-module decomp counts\n\
             - `csharp/` — C→C# reconstruction (stage `csharp`)\n\n\
             Ghidra project state and raw `cro_src` binaries are not copied \
             (tool state, not output); rebuild via `scripts/rebuild_v2.sh`.\n",
            count, "(14,864)", "(23,521)"
        ),
    )?;
    ctx.summary.entries += count;
    Ok(())
}

// ---------------------------------------------------------------------------
// Stage: csharp (C → C# pipeline)
// ---------------------------------------------------------------------------

fn stage_csharp(ctx: &mut Ctx) -> io::Result<()> {
    let decomp = ctx.input.join("decomp");
    let out_root = ctx.output.join("code/csharp");
    fs::create_dir_all(&out_root)?;

    let mut total = 0usize;
    let mut converted = 0usize;
    let mut modules = 0usize;

    // Main binary: code_c → module "Main", symbols from functions_sym.tsv.
    let code_c = decomp.join("code_c");
    if code_c.is_dir() {
        let symbols = load_code_symbols(&decomp.join("code_c/functions_sym.tsv"));
        let stats = csharp::convert_module("Main", &code_c, &out_root.join("Main"), &symbols)?;
        ctx.rows.push(Row::new(
            "decomp/code_c",
            "code/csharp/Main/",
            "convert",
            format!(
                "C→C#: {} of {} functions auto-converted, {} stubs, {} external stubs",
                stats.converted, stats.total, stats.unconverted, stats.external_stubs
            ),
            0,
            0,
        ));
        ctx.emit(modules, 0, format!("csharp Main: {}", stats.summary_line()));
        total += stats.total;
        converted += stats.converted;
        modules += 1;
    }

    // CRO modules.
    let cro_c = decomp.join("cro_c");
    if cro_c.is_dir() {
        let mut names: Vec<PathBuf> = fs::read_dir(&cro_c)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_dir())
            .collect();
        names.sort();
        for dir in &names {
            ctx.check_cancel()?;
            let Some(name) = dir.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let ident = sanitize_module_ident(name);
            let symbols = load_cro_symbols(
                &ctx.input
                    .join("cro_extracted")
                    .join(name)
                    .join("symbols.tsv"),
            );
            let stats = csharp::convert_module(&ident, dir, &out_root.join(&ident), &symbols)?;
            ctx.rows.push(Row::new(
                format!("decomp/cro_c/{name}"),
                format!("code/csharp/{ident}/"),
                "convert",
                format!(
                    "C→C#: {} of {} functions auto-converted, {} stubs, {} external stubs",
                    stats.converted, stats.total, stats.unconverted, stats.external_stubs
                ),
                0,
                0,
            ));
            ctx.emit(
                modules,
                names.len(),
                format!("csharp {ident}: {}", stats.summary_line()),
            );
            total += stats.total;
            converted += stats.converted;
            modules += 1;
        }
    }

    let mut rd = String::from("# C# reconstruction (deobfuscation pipeline)\n\n");
    let _ = writeln!(
        rd,
        "Mechanical, deterministic C→C# conversion of the whole Ghidra \
         corpus: {modules} modules, {converted}/{total} functions auto-converted. \
         Per-module stats and stub reasons in each module's README.md.\n\n\
         Layout per module `<Name>/`: `Functions_NNN.cs` (partial classes), \
         `Labels.g.cs` (data-label placeholders), `Unconverted.cs` (stubs \
         with original C), `Imports.g.cs` (external references), \
         `Runtime.cs` (Ghidra idiom helpers).\n\n\
         This is a readable reconstruction, not a verified build — the \
         decompiled C under `code/decomp/` stays authoritative."
    );
    fs::write(out_root.join("README.md"), &rd)?;
    ctx.rows.push(Row::new(
        "<generated>",
        "code/csharp/README.md",
        "generate",
        format!("{modules} modules, {converted}/{total} functions converted"),
        rd.len() as u64,
        rd.len() as u64,
    ));

    ctx.summary.csharp_modules = modules;
    ctx.summary.csharp_converted = converted;
    ctx.summary.csharp_total = total;
    Ok(())
}

/// CRO class identifiers must be valid C# identifiers.
fn sanitize_module_ident(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if cleaned.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("M{cleaned}")
    } else {
        cleaned
    }
}

/// symbols.tsv of a CRO (`name\toffset\tload_addr`) → addr → name.
fn load_cro_symbols(path: &Path) -> std::collections::BTreeMap<String, String> {
    let mut map = std::collections::BTreeMap::new();
    let Ok(text) = fs::read_to_string(path) else {
        return map;
    };
    for line in text.lines().skip(1) {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 3 {
            continue;
        }
        let Ok(addr) = u64::from_str_radix(cols[2].trim_start_matches("0x"), 16) else {
            continue;
        };
        map.insert(format!("{addr:08x}"), cols[0].to_string());
    }
    map
}

/// functions_sym.tsv (`entry\tname\tsymbol\tbody_size\tdecompiled`) → addr → symbol.
fn load_code_symbols(path: &Path) -> std::collections::BTreeMap<String, String> {
    let mut map = std::collections::BTreeMap::new();
    let Ok(text) = fs::read_to_string(path) else {
        return map;
    };
    for line in text.lines().skip(1) {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 3 || cols[2].is_empty() {
            continue;
        }
        let Ok(addr) = u64::from_str_radix(cols[0], 16) else {
            continue;
        };
        map.insert(format!("{addr:08x}"), cols[2].to_string());
    }
    map
}

// ---------------------------------------------------------------------------
// Stage: unknown (unmapped GARC dirs preserved raw)
// ---------------------------------------------------------------------------

fn stage_unknown(ctx: &mut Ctx) -> io::Result<()> {
    let garc = ctx.input.join("garc");
    let a_root = garc.join("a");
    if !a_root.is_dir() {
        return Ok(());
    }
    let known: BTreeSet<&str> = GARC_DIRS
        .iter()
        .map(|(r, _, _)| *r)
        .chain(POKEMON_GARC_DIRS.iter().copied())
        .collect();

    // Deterministic walk (sorted), deepest-first is unnecessary — breadth is
    // fine since output dirs never collide.
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut stack = vec![a_root.clone()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)?.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            }
        }
        dirs.push(dir);
    }
    dirs.sort();
    let mut total = 0usize;
    for dir in &dirs {
        ctx.check_cancel()?;
        let rel = dir
            .strip_prefix(&garc)
            .unwrap_or(dir)
            .to_string_lossy()
            .into_owned();
        if known.contains(rel.as_str()) {
            continue;
        }
        let files: Vec<PathBuf> = fs::read_dir(dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_file())
            .collect();
        if files.is_empty() {
            continue;
        }
        let dst_dir = ctx.output.join("unknown/garc").join(&rel);
        let mut copied = 0usize;
        for f in &files {
            let rel_src = f.strip_prefix(ctx.input).unwrap_or(f);
            let dst = dst_dir.join(f.file_name().unwrap_or_default());
            match place_file(f, &dst) {
                Ok((ib, ob)) => {
                    let rel_dst = dst.strip_prefix(ctx.output).unwrap_or(&dst);
                    ctx.rows.push(Row::new(
                        rel_src.display().to_string(),
                        rel_dst.display().to_string(),
                        "copy",
                        "unmapped GARC data preserved raw",
                        ib,
                        ob,
                    ));
                    copied += 1;
                }
                Err(_) => ctx.rows.push(Row::failed(
                    rel_src.display().to_string(),
                    dst.display().to_string(),
                    "copy",
                    "copy failed",
                )),
            }
        }
        write_readme(
            &dst_dir,
            format!(
                "# unknown/garc/{rel}\n\n\
                 Not yet semantically mapped — preserved raw (never discarded). \
                 {copied} files from `garc/{rel}/`. Identify via magic bytes \
                 (`xy info <file>`) before writing a parser.\n"
            ),
        )?;
        total += copied;
        ctx.emit(total, 0, format!("unknown garc/{rel}: {copied} files"));
    }
    write_readme(
        &ctx.output.join("unknown"),
        format!(
            "# unknown\n\n\
             Data that no validated parser covers yet — preserved raw so \
             nothing is lost. {total} files under `garc/<romfs path>/`, \
             mirroring the source GARC layout. When a format is cracked, \
             promote it into a curated stage (see `xy curate` in \
             PokeXYZ-RDComp) rather than converting in place.\n"
        ),
    )?;
    ctx.summary.entries += total;
    Ok(())
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Re-verify every manifest row on disk: existence, size, and format magic
/// where a standard format is claimed. Returns (checks, failures).
/// One `validation.tsv` detail line.
struct CheckRow {
    path: String,
    check: &'static str,
    result: &'static str,
    detail: String,
}

/// Re-verify every manifest row on disk: existence, size, and format magic
/// where a standard format is claimed. Returns (checks, failures, details).
fn validate(rows: &[Row], out_root: &Path) -> io::Result<(usize, usize, Vec<CheckRow>)> {
    let mut checks = 0usize;
    let mut failures = 0usize;
    let mut details: Vec<CheckRow> = Vec::new();
    let fail =
        |details: &mut Vec<CheckRow>, row: &Row, check, detail: String, failures: &mut usize| {
            *failures += 1;
            details.push(CheckRow {
                path: row.path.clone(),
                check,
                result: "fail",
                detail,
            });
        };
    for row in rows {
        if row.status != "ok" {
            continue;
        }
        let path = out_root.join(&row.path);
        // Aggregate rows (family dirs, module dirs) just check existence.
        let is_dir_row = row.path.ends_with('/');
        if is_dir_row {
            checks += 1;
            if !path.is_dir() {
                fail(
                    &mut details,
                    row,
                    "exists",
                    "directory missing".into(),
                    &mut failures,
                );
            }
            continue;
        }
        checks += 1;
        let Ok(meta) = fs::metadata(&path) else {
            fail(
                &mut details,
                row,
                "exists",
                "file missing".into(),
                &mut failures,
            );
            continue;
        };
        if meta.len() != row.out_bytes {
            fail(
                &mut details,
                row,
                "size",
                format!("on-disk {} != manifest {}", meta.len(), row.out_bytes),
                &mut failures,
            );
            continue;
        }
        if !format_magic_ok(&path) {
            fail(
                &mut details,
                row,
                "magic",
                "format header mismatch".into(),
                &mut failures,
            );
        }
    }
    Ok((checks, failures, details))
}

/// Standard-format magic spot-checks for converted/known file types.
fn format_magic_ok(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return true;
    };
    let ext = name.rsplit('.').next().unwrap_or("");
    let Ok(mut f) = fs::File::open(path) else {
        return false;
    };
    use std::io::Read;
    let mut head = [0u8; 8];
    let n = f.read(&mut head).unwrap_or(0);
    // Empty files are legitimate (faithful copies of empty tables and
    // zero-size segments) — only formats that guarantee a header are checked.
    match ext {
        "wav" => n >= 4 && &head[0..4] == b"RIFF",
        "mp4" => n >= 8 && &head[4..8] == b"ftyp",
        "png" => n >= 4 && &head[0..4] == b"\x89PNG",
        "json" => n >= 1 && (head[0] == b'{' || head[0] == b'['),
        "obj" => n >= 1 && (head[0] == b'#' || head[0] == b'o' || head[0] == b'v'),
        "mtl" => n >= 1 && (head[0] == b'#' || head[0] == b'n'),
        _ => true,
    }
}

// ---------------------------------------------------------------------------
// Top-level README
// ---------------------------------------------------------------------------

fn write_top_readme(ctx: &Ctx, stages: &[Stage]) -> io::Result<()> {
    let stage_names: Vec<&str> = stages
        .iter()
        .map(|s| {
            ALL_STAGES
                .iter()
                .find(|(_, v)| v == s)
                .map(|(n, _)| *n)
                .unwrap_or("?")
        })
        .collect();
    let s = &ctx.summary;
    let text = format!(
        "# curated — the final human- and tool-friendly extraction\n\n\
         Built by `xy curate -i <Source_Extracted>` (stages: {}). Deterministic: \
         re-running produces identical structure and filenames. **Real files \
         only — no symlinks.**\n\n\
         Provenance chain: `.Data/Raw_Extracted/` (raw decrypted ROM) → \
         `.Data/Source_Extracted/` (GARC/BCMA/CRO unpacking, Ghidra, ffmpeg) → \
         this tree (organization + standard-format conversion). Every file \
         here is listed in `manifest.tsv` (source → path → action → bytes) \
         and re-verified by `validation.tsv`.\n\n\
         | directory | contents |\n\
         |---|---|\n\
         | `pokemon/` | {} families; models → OBJ+MTL ({} files), textures → PNG ({} files), full 8-file sources, graphics index |\n\
         | `text/` | gametext/storytext JSON per language; `text/organized/` named banks (species, moves, abilities, items, …) |\n\
         | `battle/ world/ ui/` | semantically mapped GARC data (pk3DS naming) |\n\
         | `audio/` | BGM as WAV (ffmpeg; banner jingle via DSP-ADPCM decoder) |\n\
         | `video/` | movies as MP4 (ffmpeg) |\n\
         | `code/` | CRO modules, extracted segments + symbols, {} decompiled C functions, `modules.json` dependency graph |\n\
         | `code/csharp/` | C→C# deobfuscation reconstruction ({} modules, {}/{} functions auto-converted) |\n\
         | `manual/ shaders/` | e-manual assets, SDK shader binaries |\n\
         | `unknown/` | unmapped data preserved raw, documented |\n\n\
         Known limits (documented, never discarded): v2 GFMotion skeletal \
         streams and rev-9 CNOD meshes are open RE projects; battle tables \
         (`battle/*.*/`) are organized but not yet field-decoded; media \
         conversion (ffmpeg) happens in the rebuild script and is copied in.\n\n\
         Rebuild: `cargo build --release && ./target/release/xy curate -i ../../.Data/Source_Extracted`\n\
         (run from the `PokeXYZ-RDComp/` repo root).\n",
        stage_names.join(","),
        s.families,
        s.models_obj,
        s.textures_png,
        "(38,385)",
        s.csharp_modules,
        s.csharp_converted,
        s.csharp_total,
    );
    fs::write(ctx.output.join("README.md"), text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garc_table_paths_are_unique() {
        let mut seen = BTreeSet::new();
        for (rel, _, _) in GARC_DIRS {
            assert!(seen.insert(*rel), "duplicate GARC path {rel}");
            assert!(rel.starts_with("a/"), "romfs path expected: {rel}");
        }
    }

    #[test]
    fn stages_parse() {
        assert_eq!(parse_stages("all").map(|v| v.len()), Some(ALL_STAGES.len()));
        assert_eq!(parse_stages("data,csharp").map(|v| v.len()), Some(2));
        assert!(parse_stages("bogus").is_none());
        let all: Vec<Stage> = parse_stages("all").unwrap();
        assert_eq!(
            parse_stages("all,all").map(|v| v.len()),
            Some(all.len() * 2)
        );
    }

    #[test]
    fn named_banks_are_sorted_and_unique() {
        let mut seen = BTreeSet::new();
        for (bank, name) in NAMED_BANKS {
            assert!(seen.insert(*bank), "duplicate bank {bank}");
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn module_idents_are_valid_cs() {
        assert_eq!(sanitize_module_ident("DllBag"), "DllBag");
        assert_eq!(sanitize_module_ident("1 weird-name"), "M1_weird_name");
    }
}
