//! `xy` — unified CLI for the Pokémon X/Y (kujira) decompilation toolkit.
//!
//! Subcommand groups map 1:1 onto the workspace crates:
//!
//! | group | crate | contents |
//! |---|---|---|
//! | `xy core` | `xy-core` | LZ10/LZ11 stream decompression |
//! | `xy archive` | `xy-archive` | RomFS, ExeFS, BLZ (ExeFS .code), DARC (e-manual), GARC |
//! | `xy audio` | `xy-audio` | BCWAV → WAV |
//! | `xy gfx` | `xy-gfx` | SMDH icons |
//!
//! `xy-script` / `xy-battle` commands land once GARC payloads are parseable
//! (their data lives inside GARC containers; see PLAN.md Phase 1).
//!
//! Pokémon X is `0004000000055D00` ("kujira-1"); Pokémon Y is
//! `0004000000055E00` ("kujira-2"). Same formats, so every parser here
//! serves both.

use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use xy_archive::bcma::unique_name;
use xy_archive::{bcma, blz, exefs, garc, lzss, romfs};
use xy_audio::bcwav;
use xy_core::util::json_escape;
use xy_core::{lz10, sniff};
use xy_gfx::smdh;

#[derive(Parser)]
#[command(
    name = "xy",
    version,
    about = "Pokemon X/Y (kujira) decompilation toolkit",
    long_about = "Extraction and conversion tools for Pokemon X/Y game data.\n\
                  Parsers are game-agnostic; validated against a Pokemon X dump (0004000000055D00)."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Identify a file by its magic bytes
    Info { path: PathBuf },

    /// Shared primitives: compression streams
    Core {
        #[command(subcommand)]
        cmd: CoreCmd,
    },

    /// Container extraction: RomFS, DARC/BCMA, GARC
    Archive {
        #[command(subcommand)]
        cmd: ArchiveCmd,
    },

    /// Audio conversion: BCWAV -> WAV (streamed BGM is plain AAC -> ffmpeg)
    Audio {
        #[command(subcommand)]
        cmd: AudioCmd,
    },

    /// Graphics: SMDH icon + title metadata
    Gfx {
        #[command(subcommand)]
        cmd: GfxCmd,
    },

    /// Script & text: Gen6 text banks -> JSON
    Script {
        #[command(subcommand)]
        cmd: ScriptCmd,
    },

    /// One-shot conversions to standard formats (GLB/PNG/WAV/JSON)
    Convert {
        #[command(subcommand)]
        cmd: ConvertCmd,
    },

    /// File inspection: detected kind, magic, hashes, model/texture census
    Inspect {
        path: PathBuf,
        /// Also compute SHA-256/CRC32/FNV (reads the whole file)
        #[arg(long)]
        hashes: bool,
    },

    /// Search the extraction outputs by name or format kind
    Search {
        #[command(subcommand)]
        cmd: SearchCmd,
    },

    /// Media metadata, frame extraction, container conversion (ffmpeg)
    Media {
        #[command(subcommand)]
        cmd: MediaCmd,
    },

    /// Curated-tree manifest browsing, provenance, re-validation
    Manifest {
        #[command(subcommand)]
        cmd: ManifestCmd,
    },

    /// Hex dump a byte window (for unsupported / partially understood files)
    Hex {
        path: PathBuf,
        /// Start offset (hex or decimal)
        #[arg(long, default_value = "0")]
        offset: String,
        /// Number of bytes to dump
        #[arg(long, default_value_t = 512)]
        len: usize,
    },

    /// CRO code modules: info + segment/symbol extraction
    Cro {
        #[command(subcommand)]
        cmd: CroCmd,
    },

    /// Build the curated tree: real files, standard-format conversions,
    /// manifest + validation (never symlinks)
    Curate {
        /// Source_Extracted root
        #[arg(short, long)]
        input: PathBuf,
        /// Output root (default: <input>/curated)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Comma-separated stage list, or "all" (default): data,text,pokemon,media,code,csharp,unknown
        #[arg(long, default_value = "all")]
        stage: String,
    },
}

#[derive(Subcommand)]
enum CoreCmd {
    /// Decompress a Nintendo LZ10/LZ11 stream
    Lz {
        path: PathBuf,
        /// Output file (default: input with .dec appended)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ArchiveCmd {
    /// Extract RomFS from a CCI/.3ds, bare NCCH, or raw IVFC image
    Romfs { path: PathBuf, outdir: PathBuf },

    /// Extract ExeFS sections (.code/icon/banner/logo) from a CCI/.3ds, a
    /// standalone CXI (e.g. sliced out of a CIA's content region), or a raw
    /// ExeFS region
    Exefs { path: PathBuf, outdir: PathBuf },

    /// Decompress a 3DS ExeFS `.code` backward-LZSS ("BLZ") buffer
    Blz {
        path: PathBuf,
        /// Output file (default: input with .dec appended)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Extract a BCMA-DARC archive (XY e-manual container)
    Darc { path: PathBuf, outdir: PathBuf },

    /// Extract the XY e-manual: BCMA container -> LZ10 pages -> inner DARC assets
    Manual { path: PathBuf, outdir: PathBuf },

    /// List the entries of a GARC container
    List { path: PathBuf },

    /// Unpack a GARC container (pk3DS-compatible names: N.bin / N/NN.bin)
    Unpack {
        /// Input GARC file
        #[arg(short, long)]
        input: PathBuf,
        /// Output directory
        #[arg(short, long)]
        output: PathBuf,
        /// Decompress LZ-0x11 subfiles (keeps raw bytes when decoding fails)
        #[arg(long)]
        decompress: bool,
    },

    /// Walk a romfs tree and unpack every GARC found (mirrored paths)
    ExtractTree {
        /// Input tree root to walk (e.g. .../romfs/a)
        #[arg(short, long)]
        input: PathBuf,
        /// Output root for mirrored extraction
        #[arg(short, long)]
        output: PathBuf,
        /// Decompress LZ-0x11 subfiles
        #[arg(long)]
        decompress: bool,
        /// Also copy non-GARC files, mirrored
        #[arg(long)]
        copy_raw: bool,
    },

    /// Audit GARC sub-files: find 0x11-prefixed payloads that fail LZ-0x11
    /// decode (potential missed compressions) and print compression stats
    Audit {
        /// Input tree root to walk (e.g. .../romfs/a)
        #[arg(short, long)]
        input: PathBuf,
    },
}

#[derive(Subcommand)]
enum AudioCmd {
    /// Convert BCWAV audio to WAV (PCM8/PCM16 and DSP-ADPCM)
    Bcwav {
        path: PathBuf,
        /// Output WAV (default: input with .wav extension)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum GfxCmd {
    /// Dump SMDH title metadata and write the icon as PNG
    Icon {
        path: PathBuf,
        /// Output PNG (default: input with .png extension)
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Nearest-neighbor upscale factor (e.g. 8) for the dithered art
        #[arg(long)]
        scale: Option<u32>,
    },

    /// Inspect a BCRES (CGFX) model container: header, chunks, names
    BcresInfo { path: PathBuf },

    /// Decode a model container (BCH raw or PC/PF/PT) to OBJ+MTL
    ParseModel {
        path: PathBuf,
        /// Output directory (default: input's directory)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Extract and decode textures from a BCH/CGFX/PC container to PNG
    Texture {
        path: PathBuf,
        /// Output directory (default: input's directory)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Software-render a model to PNG (headless viewer: single shot or
    /// turntable sequence). Modes: textured|material|wireframe|normals|skeleton
    Render {
        path: PathBuf,
        /// Output PNG (default: input with .png extension) or directory
        /// when --turntable is set (default: input's directory)
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Shading mode (textured, material, wireframe, normals, skeleton)
        #[arg(long, default_value = "textured")]
        mode: String,
        /// Camera yaw in degrees
        #[arg(long, default_value_t = 35.0)]
        yaw: f32,
        /// Camera pitch in degrees
        #[arg(long, default_value_t = 18.0)]
        pitch: f32,
        /// Render width
        #[arg(long, default_value_t = 800)]
        width: u32,
        /// Render height
        #[arg(long, default_value_t = 600)]
        height: u32,
        /// Backface culling
        #[arg(long)]
        cull: bool,
        /// Render N frames rotating 360° (turntable)
        #[arg(long)]
        turntable: Option<usize>,
        /// Which model in the container to render (default: all)
        #[arg(long)]
        model: Option<usize>,
        /// Texture container to merge in (family PT/CGFX/BCH pack — the
        /// model PC itself carries no textures)
        #[arg(long)]
        textures: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ScriptCmd {
    /// Decode a Gen6 text bank (gametext/storytext) to JSON
    Text {
        path: PathBuf,
        /// Output JSON file (default: stdout)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ConvertCmd {
    /// Convert a model container (BCH/CGFX/PC) to Blender-ready GLB
    /// (glTF 2.0, textures + skeleton embedded)
    Glb {
        path: PathBuf,
        /// Output directory (default: input's directory)
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Texture container to merge in (family PT/CGFX/BCH pack —
        /// model PCs carry no textures)
        #[arg(long)]
        textures: Option<PathBuf>,
        /// Validate each written GLB with the gltf crate and print a report
        #[arg(long)]
        validate: bool,
    },

    /// Convert textures of a model container to PNG
    Png {
        path: PathBuf,
        /// Output directory (default: input's directory)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Convert any audio (BCWAV direct; others via ffmpeg) to WAV
    Audio {
        path: PathBuf,
        /// Output WAV (default: input with .wav extension)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// Decode a Gen6 text bank to pretty JSON
    Text {
        path: PathBuf,
        /// Output JSON (default: stdout)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum SearchCmd {
    /// Find files by name substring under a root (case-insensitive)
    Name {
        /// Root directory to walk
        root: PathBuf,
        /// Substring to match against file names
        query: String,
        /// Stop after N matches
        #[arg(long, default_value_t = 200)]
        limit: usize,
    },
    /// Find files whose detected format kind matches (e.g. model, garc)
    Kind {
        root: PathBuf,
        /// Kind label: model, texture, image, audio, video, text, json, cro, garc, ...
        kind: String,
        #[arg(long, default_value_t = 200)]
        limit: usize,
    },
}

#[derive(Subcommand)]
enum MediaCmd {
    /// Container/stream metadata via ffprobe
    Info { path: PathBuf },

    /// Extract preview frames from a video (ffmpeg)
    Frames {
        path: PathBuf,
        /// Output directory
        #[arg(short, long)]
        out: PathBuf,
        /// Frames per second to sample
        #[arg(long, default_value_t = 10.0)]
        fps: f32,
        /// Maximum number of frames
        #[arg(long, default_value_t = 360)]
        max: usize,
        /// Scale frames to this width (keeps aspect)
        #[arg(long)]
        width: Option<u32>,
    },

    /// Convert a video to H.264+AAC MP4 (e.g. moflex)
    Mp4 {
        path: PathBuf,
        /// Output MP4 (default: input with .mp4 extension)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ManifestCmd {
    /// List manifest rows (optionally filtered by a path/source substring)
    List {
        /// Curated tree root (contains manifest.tsv)
        root: PathBuf,
        /// Case-insensitive substring filter
        #[arg(default_value = "")]
        query: String,
        /// Maximum rows to print
        #[arg(long, default_value_t = 100)]
        limit: usize,
    },

    /// Provenance of one file: curated → raw source, or raw → curated
    Lookup { root: PathBuf, path: PathBuf },

    /// Re-verify manifest rows on disk (existence + size)
    Validate { root: PathBuf },
}

#[derive(Subcommand)]
enum CroCmd {
    /// Show header, segments, exports and imports of a CRO/CRS module
    Info { path: PathBuf },

    /// Extract segments (raw bins for Ghidra) + symbol/import TSVs
    Extract {
        path: PathBuf,
        /// Output directory
        #[arg(short, long)]
        out: PathBuf,
    },
}

/// Unified CLI error: parsers use `xy_core::Error` (directly or via `From`),
/// plus the graphics crate's own png-aware wrapper.
#[derive(Debug, thiserror::Error)]
enum CliError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Core(#[from] xy_core::Error),
    #[error(transparent)]
    Gfx(#[from] xy_gfx::Error),
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli.cmd) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run(cmd: Cmd) -> Result<(), CliError> {
    match cmd {
        Cmd::Info { path } => {
            let data = fs::read(&path)?;
            let kind = xy_convert::detect_kind(&data, &path);
            let sniffed = sniff(&data).unwrap_or("unknown");
            println!(
                "{}: {} [{}] ({} bytes)",
                path.display(),
                kind.label(),
                sniffed,
                data.len()
            );
            Ok(())
        }

        Cmd::Core { cmd } => core_cmd(cmd),
        Cmd::Archive { cmd } => archive_cmd(cmd),
        Cmd::Audio { cmd } => audio_cmd(cmd),
        Cmd::Gfx { cmd } => gfx_cmd(cmd),
        Cmd::Script { cmd } => script_cmd(cmd),
        Cmd::Convert { cmd } => convert_cmd(cmd),
        Cmd::Inspect { path, hashes } => inspect_cmd(path, hashes),
        Cmd::Search { cmd } => search_cmd(cmd),
        Cmd::Media { cmd } => media_cmd(cmd),
        Cmd::Manifest { cmd } => manifest_cmd(cmd),
        Cmd::Hex { path, offset, len } => {
            let off = if let Some(h) = offset.strip_prefix("0x") {
                u64::from_str_radix(h, 16)
            } else {
                offset.parse()
            }
            .map_err(|e| CliError::Io(std::io::Error::other(format!("bad offset: {e}"))))?;
            let window = xy_convert::inspect::read_window(&path, off, len)?;
            for (i, chunk) in window.chunks(16).enumerate() {
                let addr = off as usize + i * 16;
                let mut hex = String::with_capacity(48);
                for (b, byte) in chunk.iter().enumerate() {
                    if b == 8 {
                        hex.push(' ');
                    }
                    hex.push_str(&format!("{byte:02X} "));
                }
                let ascii: String = chunk
                    .iter()
                    .map(|&b| {
                        if (0x20..0x7f).contains(&b) {
                            b as char
                        } else {
                            '·'
                        }
                    })
                    .collect();
                println!("{addr:08X}  {hex:<48} {ascii}");
            }
            Ok(())
        }
        Cmd::Cro { cmd } => cro_cmd(cmd),
        Cmd::Curate {
            input,
            output,
            stage,
        } => {
            let output = output.unwrap_or_else(|| input.join("curated"));
            let Some(stages) = xy_curate::parse_stages(&stage) else {
                eprintln!("error: unknown stage list '{stage}' (data,text,pokemon,media,code,csharp,unknown,all)");
                std::process::exit(1);
            };
            match xy_curate::build(&input, &output, &stages) {
                Ok(s) => {
                    println!(
                        "{} -> {}\n  entries: {}\n  pokemon: {} families, {} OBJ/MTL, {} PNGs\n  csharp: {} modules, {}/{} functions converted\n  manifest: {} rows; validation: {} checks, {} failures",
                        input.display(),
                        output.display(),
                        s.entries,
                        s.families,
                        s.models_obj,
                        s.textures_png,
                        s.csharp_modules,
                        s.csharp_converted,
                        s.csharp_total,
                        s.manifest_rows,
                        s.validation_checks,
                        s.validation_failures,
                    );
                    Ok(())
                }
                Err(e) => Err(CliError::Io(e)),
            }
        }
    }
}

fn cro_cmd(cmd: CroCmd) -> Result<(), CliError> {
    use xy_archive::cro::{Cro, SegmentType};

    let (path, out) = match cmd {
        CroCmd::Info { path } => (path, None),
        CroCmd::Extract { path, out } => (path, Some(out)),
    };
    let data = fs::read(&path)?;
    let cro = Cro::parse(&data)?;
    let name = cro.name()?;
    println!(
        "{}: \"{}\", {:#x} bytes, code {:#x}+{:#x}, data {:#x}+{:#x}, bss {:#x}",
        path.display(),
        name,
        cro.file_size,
        cro.code_offset,
        cro.code_size,
        cro.data_offset,
        cro.data_size,
        cro.bss_size
    );
    for (i, seg) in cro.segments.iter().enumerate() {
        println!(
            "  seg {i}: {} @{:#x} +{:#x}",
            seg.ty.name(),
            seg.offset,
            seg.size
        );
    }
    println!(
        "  exports: {} named + {} indexed; imports: {} modules, {} named; {} internal relocs",
        cro.named_exports.len(),
        cro.indexed_export_count,
        cro.import_modules.len(),
        cro.named_import_count,
        cro.internal_relocation_count
    );
    let mods: Vec<String> = cro.import_modules.iter().map(|m| m.name.clone()).collect();
    if !mods.is_empty() {
        println!("  imports from: {}", mods.join(", "));
    }

    if let Some(out) = out {
        fs::create_dir_all(&out)?;
        // Segments as raw binaries (base address for Ghidra: 0x00100000).
        let mut manifest = String::from("segment\ttype\tfile_offset\tsize\tload_addr\n");
        for (i, seg) in cro.segments.iter().enumerate() {
            let load = 0x0010_0000u64 + seg.offset as u64;
            if seg.ty == SegmentType::Bss {
                manifest.push_str(&format!(
                    "{i}\t.bss\t{:#x}\t{:#x}\t{:#x}\n",
                    seg.offset, seg.size, load
                ));
                continue;
            }
            // CRS segments use virtual addresses of the *main executable*
            // and can exceed this file — record as virtual, don't dump.
            let start = seg.offset as usize;
            let end = start.saturating_add(seg.size as usize);
            match cro.segment_data(i) {
                Ok(blob) => {
                    let file = out.join(format!(
                        "seg{i}_{}.bin",
                        seg.ty.name().trim_start_matches('.')
                    ));
                    fs::write(&file, blob)?;
                    manifest.push_str(&format!(
                        "{i}\t{}\t{:#x}\t{:#x}\t{:#x}\n",
                        seg.ty.name(),
                        seg.offset,
                        seg.size,
                        load
                    ));
                }
                Err(_) if end > data.len() => {
                    manifest.push_str(&format!(
                        "{i}\t{}\tvirtual\t{:#x}\t{:#x}\n",
                        seg.ty.name(),
                        seg.size,
                        seg.offset as u64
                    ));
                }
                Err(e) => return Err(e.into()),
            }
        }
        fs::write(out.join("segments.tsv"), manifest)?;
        // Symbols: name, offset, load address. For CROs the load base is
        // 0x00100000; for the CRS the offsets are already code.bin VAs.
        let is_crs = cro.code_size == 0;
        let mut symbols = String::from("name\toffset\tload_addr\n");
        for e in &cro.named_exports {
            let addr = if is_crs {
                e.offset
            } else {
                0x0010_0000 + e.offset
            };
            symbols.push_str(&format!("{}\t{:#x}\t{:#x}\n", e.name, e.offset, addr));
        }
        fs::write(out.join("symbols.tsv"), symbols)?;
        // Imports.
        let mut imports = String::from("module\tindexed\tanonymous\n");
        for m in &cro.import_modules {
            imports.push_str(&format!(
                "{}\t{}\t{}\n",
                m.name, m.indexed_count, m.anonymous_count
            ));
        }
        fs::write(out.join("imports.tsv"), imports)?;
        println!(
            "extracted to {} ({} symbols)",
            out.display(),
            cro.named_exports.len()
        );
    }
    Ok(())
}

fn script_cmd(cmd: ScriptCmd) -> Result<(), CliError> {
    let ScriptCmd::Text { path, out } = cmd;
    let data = fs::read(&path)?;
    let tf = xy_script::textfile::TextFile::parse(&data)?;
    let lines = tf.lines()?;
    let mut json = String::with_capacity(lines.len() * 32 + 64);
    json.push_str("{\"file\":");
    json.push_str(&format!("\"{}\"", json_escape(&path.display().to_string())));
    json.push_str(",\"line_count\":");
    json.push_str(&tf.line_count().to_string());
    json.push_str(",\"lines\":[");
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&format!(
            "{{\"index\":{i},\"text\":\"{}\"}}",
            json_escape(line)
        ));
    }
    json.push_str("]}\n");
    match out {
        Some(outfile) => {
            fs::write(&outfile, json)?;
            println!(
                "{} -> {} ({} lines)",
                path.display(),
                outfile.display(),
                lines.len()
            );
            Ok(())
        }
        None => {
            print!("{json}");
            Ok(())
        }
    }
}

fn core_cmd(cmd: CoreCmd) -> Result<(), CliError> {
    let CoreCmd::Lz { path, out } = cmd;
    let data = fs::read(&path)?;
    let dec = lz10::decompress(&data)?;
    let out = out.unwrap_or_else(|| {
        let mut s = path.clone().into_os_string();
        s.push(".dec");
        PathBuf::from(s)
    });
    fs::write(&out, &dec)?;
    println!(
        "{} -> {} ({} bytes)",
        path.display(),
        out.display(),
        dec.len()
    );
    Ok(())
}

fn archive_cmd(cmd: ArchiveCmd) -> Result<(), CliError> {
    match cmd {
        ArchiveCmd::Romfs { path, outdir } => {
            let data = fs::read(&path)?;
            let romfs = romfs::Romfs::open(&data)?;
            let n = romfs.extract(&outdir)?;
            println!("extracted {n} files -> {}", outdir.display());
            Ok(())
        }

        ArchiveCmd::Exefs { path, outdir } => {
            let data = fs::read(&path)?;
            let exefs = exefs::Exefs::open(&data)?;
            let n = exefs.extract(&outdir)?;
            println!("extracted {n} files -> {}", outdir.display());
            Ok(())
        }

        ArchiveCmd::Blz { path, out } => {
            let data = fs::read(&path)?;
            let dec = blz::decompress(&data)?;
            let out = out.unwrap_or_else(|| {
                let mut s = path.clone().into_os_string();
                s.push(".dec");
                PathBuf::from(s)
            });
            fs::write(&out, &dec)?;
            println!(
                "{} -> {} ({} -> {} bytes)",
                path.display(),
                out.display(),
                data.len(),
                dec.len()
            );
            Ok(())
        }

        ArchiveCmd::Darc { path, outdir } => {
            let data = fs::read(&path)?;
            let archive = bcma::Bcma::parse(&data)?;
            let mut seen = HashSet::new();
            let mut count = 0usize;
            for entry in archive.files() {
                let (name, blob) = entry?;
                let name = unique_name(name, &mut seen);
                let dest = outdir.join(&name);
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&dest, blob)?;
                count += 1;
            }
            println!("extracted {count} files -> {}", outdir.display());
            Ok(())
        }

        ArchiveCmd::Manual { path, outdir } => {
            let data = fs::read(&path)?;
            let archive = bcma::Bcma::parse(&data)?;
            let mut seen = HashSet::new();
            let mut pages = 0usize;
            let mut assets = 0usize;
            for entry in archive.files() {
                let (name, blob) = entry?;
                let page = unique_name(&page_stem(name), &mut seen);
                let inner = lz10::decompress(blob)?;
                let darc = bcma::Bcma::parse(&inner)?;
                let page_dir = outdir.join(&page);
                let mut page_seen = HashSet::new();
                for inner_entry in darc.files() {
                    let (iname, iblob) = inner_entry?;
                    let iname = unique_name(iname, &mut page_seen);
                    let dest = page_dir.join(&iname);
                    if let Some(parent) = dest.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&dest, iblob)?;
                    assets += 1;
                }
                pages += 1;
            }
            println!(
                "extracted {assets} assets from {pages} manual pages -> {}",
                outdir.display()
            );
            Ok(())
        }

        ArchiveCmd::List { path } => {
            let data = fs::read(&path)?;
            let garc = garc::Garc::parse(&data)?;
            println!(
                "{}: v4, {} entries, {} files",
                path.display(),
                garc.entries().len(),
                garc.file_count()
            );
            for (i, e) in garc.entries().iter().enumerate() {
                for s in &e.subs {
                    let blob = garc.subfile(i, s.id)?;
                    let magic = blob
                        .get(0..4)
                        .filter(|m| {
                            String::from_utf8_lossy(m)
                                .chars()
                                .all(|c| c.is_ascii_graphic())
                        })
                        .map(|m| String::from_utf8_lossy(m).into_owned())
                        .unwrap_or_default();
                    println!(
                        "  [{i}]{} sub {:02} [{:#08x}..{:#08x}) len {:#7x} {}",
                        if e.is_folder() { "/" } else { " " },
                        s.id,
                        s.start,
                        s.end,
                        s.len,
                        if magic.is_empty() {
                            String::new()
                        } else {
                            format!("({magic})")
                        }
                    );
                }
            }
            Ok(())
        }

        ArchiveCmd::Unpack {
            input,
            output,
            decompress,
        } => {
            let data = fs::read(&input)?;
            let garc = garc::Garc::parse(&data)?;
            let digits = garc.file_count().to_string().len().max(1);
            let mut written = 0usize;
            let mut decompressed = 0usize;
            for (i, e) in garc.entries().iter().enumerate() {
                let stem = format!("{i:0width$}", width = digits);
                let dir = if e.is_folder() {
                    let d = output.join(&stem);
                    fs::create_dir_all(&d)?;
                    d
                } else {
                    fs::create_dir_all(&output)?;
                    output.clone()
                };
                for s in &e.subs {
                    let blob = garc.subfile(i, s.id)?;
                    let name = if e.is_folder() {
                        format!("{:02}.bin", s.id)
                    } else {
                        format!("{stem}.bin")
                    };
                    let dest = dir.join(&name);
                    if decompress {
                        if let Some(dec) = lzss::try_decompress(blob) {
                            fs::write(&dest, &dec)?;
                            decompressed += 1;
                            written += 1;
                            continue;
                        }
                    }
                    fs::write(&dest, blob)?;
                    written += 1;
                }
            }
            println!(
                "{} -> {} ({} files, {} decompressed)",
                input.display(),
                output.display(),
                written,
                decompressed
            );
            Ok(())
        }

        ArchiveCmd::ExtractTree {
            input,
            output,
            decompress,
            copy_raw,
        } => {
            let mut files = Vec::new();
            collect_files(&input, &mut files)?;
            files.sort();
            let mut manifest_tsv =
                String::from("garc\tentries\tfiles\tdecompressed\tin_bytes\tout_bytes\n");
            let mut total = (0usize, 0usize, 0usize, 0u64, 0u64);
            for path in &files {
                let rel = path
                    .strip_prefix(input.parent().unwrap_or(input.as_path()))
                    .unwrap_or(path);
                let dest_root = output.join(rel);
                let data = fs::read(path)?;
                match garc::Garc::parse(&data) {
                    Ok(garc) => {
                        let (written, decompressed, out_bytes) =
                            unpack_garc_to(&garc, &dest_root, decompress)?;
                        manifest_tsv.push_str(&format!(
                            "{}\t{}\t{}\t{}\t{}\t{}\n",
                            rel.display(),
                            garc.entries().len(),
                            written,
                            decompressed,
                            data.len(),
                            out_bytes
                        ));
                        total.0 += 1;
                        total.1 += written;
                        total.2 += decompressed;
                        total.3 += data.len() as u64;
                        total.4 += out_bytes;
                        println!(
                            "  [garc] {} ({} files, {} decompressed)",
                            rel.display(),
                            written,
                            decompressed
                        );
                    }
                    Err(_) if copy_raw => {
                        fs::create_dir_all(dest_root.parent().unwrap_or(&output))?;
                        fs::copy(path, &dest_root)?;
                        total.3 += data.len() as u64;
                        total.4 += data.len() as u64;
                        println!("  [raw ] {}", rel.display());
                    }
                    Err(_) => println!("  [skip] {} (not a GARC)", rel.display()),
                }
            }
            fs::create_dir_all(&output)?;
            fs::write(output.join("manifest.tsv"), manifest_tsv)?;
            println!(
                "extract-tree: {} GARCs, {} files, {} decompressed, {} -> {} bytes; manifest at {}",
                total.0,
                total.1,
                total.2,
                total.3,
                total.4,
                output.join("manifest.tsv").display()
            );
            Ok(())
        }

        ArchiveCmd::Audit { input } => {
            let mut files = Vec::new();
            collect_files(&input, &mut files)?;
            files.sort();
            let (mut garcs, mut subs, mut decodable, mut eleven_prefixed, mut failed) =
                (0usize, 0usize, 0usize, 0usize, Vec::new());
            for path in &files {
                let rel = path.strip_prefix(&input).unwrap_or(path);
                let data = fs::read(path)?;
                let Ok(garc) = garc::Garc::parse(&data) else {
                    continue;
                };
                garcs += 1;
                for (i, e) in garc.entries().iter().enumerate() {
                    for s in &e.subs {
                        subs += 1;
                        let blob = garc.subfile(i, s.id)?;
                        if blob.first() != Some(&0x11) {
                            continue;
                        }
                        eleven_prefixed += 1;
                        match lzss::try_decompress(blob) {
                            Some(_) => decodable += 1,
                            None => failed.push(format!(
                                "  {} entry {i} sub {:02} ({} bytes) — kept raw",
                                rel.display(),
                                s.id,
                                blob.len()
                            )),
                        }
                    }
                }
            }
            println!(
                "audit: {garcs} GARCs, {subs} sub-files; {eleven_prefixed} are 0x11-prefixed, {decodable} decode as LZ-0x11, {} stay raw:",
                failed.len()
            );
            for line in &failed {
                println!("{line}");
            }
            Ok(())
        }
    }
}

/// Recursively collect file paths under `dir`.
fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

/// Shared GARC-to-disk writer used by `unpack` and `extract-tree`.
/// Returns `(files_written, decompressed, bytes_written)`.
fn unpack_garc_to(
    garc: &garc::Garc,
    output: &Path,
    decompress: bool,
) -> Result<(usize, usize, u64), CliError> {
    let digits = garc.file_count().to_string().len().max(1);
    let mut written = 0usize;
    let mut decompressed_count = 0usize;
    let mut out_bytes = 0u64;
    for (i, e) in garc.entries().iter().enumerate() {
        let stem = format!("{i:0width$}", width = digits);
        let dir = if e.is_folder() {
            let d = output.join(&stem);
            fs::create_dir_all(&d)?;
            d
        } else {
            fs::create_dir_all(output)?;
            output.to_path_buf()
        };
        for s in &e.subs {
            let blob = garc.subfile(i, s.id)?;
            let name = if e.is_folder() {
                format!("{:02}.bin", s.id)
            } else {
                format!("{stem}.bin")
            };
            let dest = dir.join(&name);
            if decompress {
                if let Some(dec) = lzss::try_decompress(blob) {
                    out_bytes += dec.len() as u64;
                    fs::write(&dest, &dec)?;
                    decompressed_count += 1;
                    written += 1;
                    continue;
                }
            }
            out_bytes += blob.len() as u64;
            fs::write(&dest, blob)?;
            written += 1;
        }
    }
    Ok((written, decompressed_count, out_bytes))
}

fn audio_cmd(cmd: AudioCmd) -> Result<(), CliError> {
    let AudioCmd::Bcwav { path, out } = cmd;
    let data = fs::read(&path)?;
    let wav = bcwav::Bcwav::parse(&data)?;
    println!(
        "{} ch, {:?}, {} Hz, {} samples ({:.2}s){}",
        wav.channels,
        wav.codec,
        wav.sample_rate,
        wav.sample_count,
        wav.sample_count as f64 / wav.sample_rate as f64,
        if wav.looped { " [looped]" } else { "" }
    );
    let out = out.unwrap_or_else(|| path.with_extension("wav"));
    let mut f = fs::File::create(&out)?;
    wav.write_wav(&mut f)?;
    println!("wrote {}", out.display());
    Ok(())
}

fn gfx_cmd(cmd: GfxCmd) -> Result<(), CliError> {
    match cmd {
        GfxCmd::Render {
            path,
            out,
            mode,
            yaw,
            pitch,
            width,
            height,
            cull,
            turntable,
            model,
            textures,
        } => {
            use xy_convert::render;
            let shading = render::Shading::from_name(&mode).ok_or_else(|| {
                CliError::Io(std::io::Error::other(format!(
                    "unknown mode '{mode}' (textured|material|wireframe|normals|skeleton)"
                )))
            })?;
            let data = fs::read(&path)?;
            let mut scenes = xy_convert::model::load_models(&data)?;
            if scenes.is_empty() {
                eprintln!("no decodable models in {}", path.display());
                return Ok(());
            }
            if let Some(tex_path) = &textures {
                let tex_data = fs::read(tex_path)?;
                let decoded = xy_convert::model::collect_decoded_textures(&tex_data);
                if decoded.is_empty() {
                    eprintln!("warning: no decodable textures in {}", tex_path.display());
                }
                let n = decoded.len();
                for scene in &mut scenes {
                    scene.textures.extend(decoded.iter().cloned());
                }
                let _ = n;
            }
            let selected: Vec<usize> = match model {
                Some(i) => vec![i],
                None => (0..scenes.len()).collect(),
            };
            let frames_n = turntable.unwrap_or(1);
            let out_dir = if turntable.is_some() {
                out.clone().unwrap_or_else(|| parent_dir(&path))
            } else {
                PathBuf::from(".")
            };
            fs::create_dir_all(&out_dir)?;
            let stem = path
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "model".into());
            let mut written = 0usize;
            for si in &selected {
                let Some(scene) = scenes.get(*si) else {
                    continue;
                };
                if scene.meshes.is_empty() && shading != render::Shading::Skeleton {
                    continue;
                }
                let cam = render::Camera::fit(scene, yaw, pitch);
                let frames =
                    render::render_turntable(scene, &cam, shading, cull, width, height, frames_n);
                let suffix = if selected.len() > 1 {
                    format!("_{}", scene.name)
                } else {
                    String::new()
                };
                for (fi, frame) in frames.iter().enumerate() {
                    let file = if frames_n > 1 {
                        out_dir.join(format!("{stem}{suffix}_{fi:03}.png"))
                    } else if let Some(o) = &out {
                        if turntable.is_some() {
                            o.join(format!("{stem}{suffix}.png"))
                        } else if suffix.is_empty() {
                            o.clone()
                        } else {
                            // keep the extension, mark the model name
                            o.with_file_name(format!(
                                "{}{suffix}{}",
                                o.file_stem().and_then(|s| s.to_str()).unwrap_or("model"),
                                o.extension()
                                    .map(|e| format!(".{}", e.to_string_lossy()))
                                    .unwrap_or_default()
                            ))
                        }
                    } else {
                        out_dir.join(format!("{stem}{suffix}.png"))
                    };
                    let png = render::frame_to_png(frame)?;
                    fs::write(&file, png)?;
                    println!(
                        "{} -> {} ({} tris, {:.1} ms)",
                        path.display(),
                        file.display(),
                        frame.stats.triangles_drawn,
                        frame.stats.render_ms
                    );
                    written += 1;
                }
            }
            if written == 0 {
                eprintln!("nothing rendered (empty meshes? try --mode skeleton)");
            }
            Ok(())
        }
        GfxCmd::Icon { path, out, scale } => {
            let data = fs::read(&path)?;
            let smdh = smdh::Smdh::parse(&data)?;
            for t in smdh.titles() {
                println!(
                    "[{:2}] {:<40} (short: {:<16} pub: {})",
                    t.region, t.long_name, t.short_name, t.long_publisher
                );
            }
            let out = out.unwrap_or_else(|| path.with_extension("png"));
            let mut f = fs::File::create(&out)?;
            match scale {
                Some(s) if s > 1 => smdh.write_png_scaled(&mut f, s)?,
                _ => smdh.write_png(&mut f)?,
            }
            println!(
                "icon -> {}{}",
                out.display(),
                scale.map(|s| format!(" ({s}x)")).unwrap_or_default()
            );
            Ok(())
        }
        GfxCmd::BcresInfo { path } => gfx_bcres_info(path),
        GfxCmd::ParseModel { path, out } => gfx_parse_model(path, out),
        GfxCmd::Texture { path, out } => gfx_texture(path, out),
    }
}

/// Decode models from BCH or PC/PF/PT containers to OBJ+MTL files.
fn gfx_parse_model(path: PathBuf, out: Option<PathBuf>) -> Result<(), CliError> {
    let data = fs::read(&path)?;
    let out_dir = out.unwrap_or_else(|| path.parent().map(Path::to_path_buf).unwrap_or_default());
    fs::create_dir_all(&out_dir)?;
    let mut written = Vec::new();

    if data.len() >= 4 && &data[0..4] == b"BCH\0" {
        let bch = xy_gfx::bch::Bch::parse(&data)?;
        println!(
            "{}: {} model(s), {} texture(s), {} animation(s)",
            path.display(),
            bch.models.len(),
            bch.textures.len(),
            bch.animations.len()
        );
        for anim in &bch.animations {
            println!(
                "  anim \"{}\": type {}, {:.0} frames{}, {} element(s)",
                anim.name,
                anim.animation_type,
                anim.frames_count,
                if anim.is_looping { ", looping" } else { "" },
                anim.elements.len()
            );
            for el in &anim.elements {
                let keys = match &el.content {
                    xy_gfx::bch::AnimContent::Float(g) => {
                        format!("float, {} keys", g.key_frames.len())
                    }
                    xy_gfx::bch::AnimContent::Vector2D(g) => format!(
                        "vec2, {}+{} keys",
                        g[0].key_frames.len(),
                        g[1].key_frames.len()
                    ),
                    xy_gfx::bch::AnimContent::Vector3D(g) => format!(
                        "vec3, {}+{}+{} keys",
                        g[0].key_frames.len(),
                        g[1].key_frames.len(),
                        g[2].key_frames.len()
                    ),
                    xy_gfx::bch::AnimContent::Boolean { values, .. } => {
                        format!("bool, {} frames", values.len())
                    }
                    xy_gfx::bch::AnimContent::Other(_) => "raw".to_string(),
                };
                println!("    \"{}\": target {:#x}, {keys}", el.name, el.target_type);
            }
        }
        for model in &bch.models {
            let verts: usize = model.meshes.iter().map(|m| m.vertices.len()).sum();
            let idx: usize = model
                .meshes
                .iter()
                .flat_map(|m| &m.submeshes)
                .map(|s| s.indices.len())
                .sum();
            println!(
                "  model \"{}\": {} mesh(es), {verts} vertices, {idx} indices, {} bones",
                model.name,
                model.meshes.len(),
                model.bones.len()
            );
            written.extend(xy_gfx::obj::export_bch_model(model, &out_dir)?);
        }
    } else {
        // PC/PF/PT container: BCH entries → models, CGFX entries → skip
        // (effect-container meshes use the unmapped CNOD variant).
        let container = xy_gfx::pkmn::PkmnContainer::parse(&data)?;
        for entry in &container.entries {
            if entry.len() >= 4 && &entry[0..4] == b"BCH\0" {
                let bch = xy_gfx::bch::Bch::parse(entry)?;
                for model in &bch.models {
                    let verts: usize = model.meshes.iter().map(|m| m.vertices.len()).sum();
                    println!(
                        "  model \"{}\": {} mesh(es), {verts} vertices, {} bones",
                        model.name,
                        model.meshes.len(),
                        model.bones.len()
                    );
                    written.extend(xy_gfx::obj::export_bch_model(model, &out_dir)?);
                }
                for anim in &bch.animations {
                    println!(
                        "  anim \"{}\": type {}, {:.0} frames, {} element(s)",
                        anim.name,
                        anim.animation_type,
                        anim.frames_count,
                        anim.elements.len()
                    );
                }
            }
        }
        if written.is_empty() {
            // Fall back to the GF-model port (older Sun/Moon-era formats).
            if let Some(group) = xy_gfx::gf_model::load_any(&data)? {
                let out_path = path.with_extension("obj");
                written.extend(xy_gfx::obj::export_group(&group, &out_dir)?);
                let _ = out_path;
            } else {
                return Err(CliError::Core(xy_core::Error::invalid(
                    "model",
                    format!("{}: no BCH/GF models found", path.display()),
                )));
            }
        }
    }

    println!(
        "{} -> {} file(s) in {}",
        path.display(),
        written.len(),
        out_dir.display()
    );
    Ok(())
}

/// Extract + decode textures from BCH/CGFX (or PC containers of either)
/// and write PNGs.
fn gfx_texture(path: PathBuf, out: Option<PathBuf>) -> Result<(), CliError> {
    let data = fs::read(&path)?;
    let out_dir = out.unwrap_or_else(|| path.parent().map(Path::to_path_buf).unwrap_or_default());
    fs::create_dir_all(&out_dir)?;

    let mut textures: Vec<xy_gfx::bch::Texture> = Vec::new();
    if data.len() >= 4 && &data[0..4] == b"BCH\0" {
        textures.extend(xy_gfx::bch::Bch::parse(&data)?.textures);
    } else if data.len() >= 4 && &data[0..4] == b"CGFX" {
        let cgfx = xy_gfx::cgfx::Cgfx::parse(&data)?;
        for tex in &cgfx.textures {
            textures.push(xy_gfx::bch::Texture {
                name: tex.name.clone(),
                width: tex.width,
                height: tex.height,
                format: tex.format,
                data: tex.data.clone(),
            });
        }
    } else {
        let container = xy_gfx::pkmn::PkmnContainer::parse(&data)?;
        for entry in &container.entries {
            if entry.len() < 4 {
                continue;
            }
            match &entry[0..4] {
                b"BCH\0" => textures.extend(xy_gfx::bch::Bch::parse(entry)?.textures),
                b"CGFX" => {
                    let cgfx = xy_gfx::cgfx::Cgfx::parse(entry)?;
                    for tex in &cgfx.textures {
                        textures.push(xy_gfx::bch::Texture {
                            name: tex.name.clone(),
                            width: tex.width,
                            height: tex.height,
                            format: tex.format,
                            data: tex.data.clone(),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    if textures.is_empty() {
        return Err(CliError::Core(xy_core::Error::invalid(
            "texture",
            format!("{}: no textures found", path.display()),
        )));
    }

    // De-duplicate names (textures repeat across containers).
    let mut used: HashSet<String> = HashSet::new();
    for tex in &textures {
        let mut name = if tex.name.is_empty() {
            "texture".to_string()
        } else {
            tex.name.clone()
        };
        let mut n = 1;
        while !used.insert(name.clone()) {
            n += 1;
            name = format!("{}_{}", tex.name, n);
        }
        let rgba = tex.decode_rgba()?;
        let out_path = out_dir.join(format!("{}_{}x{}.png", name, tex.width, tex.height));
        let f = fs::File::create(&out_path)?;
        xy_gfx::texture::write_png(
            &rgba,
            tex.width as u32,
            tex.height as u32,
            &mut std::io::BufWriter::new(f),
        )
        .map_err(xy_gfx::Error::from)?;
        println!(
            "  {name}: {}x{} {} -> {}",
            tex.width,
            tex.height,
            tex.format.name(),
            out_path.display()
        );
    }
    println!("{} -> {} texture(s)", path.display(), textures.len());
    Ok(())
}

fn gfx_bcres_info(path: PathBuf) -> Result<(), CliError> {
    use xy_gfx::bcres::Bcres;

    let data = fs::read(&path)?;
    let bcres = Bcres::parse(&data)?;
    println!(
        "{}: BCRES v{:#x}, {} content(s), {} bytes",
        path.display(),
        bcres.version_stamp,
        bcres.content_count,
        data.len()
    );
    let tags = bcres.chunk_tags();
    let tag_names: Vec<String> = tags
        .iter()
        .map(|(off, t)| format!("{}@{off:#x}", String::from_utf8_lossy(t)))
        .collect();
    println!("  chunks: {}", tag_names.join(" "));
    let names = bcres.names();
    if !names.is_empty() {
        let preview: Vec<&str> = names.iter().take(8).map(String::as_str).collect();
        println!(
            "  names ({}): {}{}",
            names.len(),
            preview.join(", "),
            if names.len() > 8 { ", …" } else { "" }
        );
    }
    Ok(())
}

/// "12-1.arc" -> "12-1"
fn page_stem(name: &str) -> String {
    match name.rsplit_once('.') {
        Some((stem, _)) => stem.to_string(),
        None => name.to_string(),
    }
}

fn convert_cmd(cmd: ConvertCmd) -> Result<(), CliError> {
    match cmd {
        ConvertCmd::Glb {
            path,
            out,
            textures,
            validate,
        } => {
            let out = out.unwrap_or_else(|| parent_dir(&path));
            let data = fs::read(&path)?;
            let tex_data = textures.map(fs::read).transpose()?;
            let outputs =
                xy_convert::convert::models_to_glb_with_textures(&data, tex_data.as_deref())
                    .map_err(CliError::Core)?;
            if outputs.is_empty() {
                eprintln!("no decodable models in {}", path.display());
                return Ok(());
            }
            let written = xy_convert::convert::write_outputs(&outputs, &out)?;
            for w in &written {
                println!("{} -> {}", path.display(), w.display());
            }
            if validate {
                let mut fail = false;
                for w in &written {
                    let bytes = fs::read(w)?;
                    match xy_convert::glb_check::validate(&bytes) {
                        Ok(r) => {
                            println!("  ✓ {} — {}", w.display(), r.summary());
                            for e in &r.errors {
                                println!("    ERROR: {e}");
                                fail = true;
                            }
                            for warn in &r.warnings {
                                println!("    warn : {warn}");
                            }
                        }
                        Err(e) => {
                            println!("  ✗ {} — PARSE FAILED: {e}", w.display());
                            fail = true;
                        }
                    }
                }
                if fail {
                    std::process::exit(1);
                }
            }
        }
        ConvertCmd::Png { path, out } => {
            let out = out.unwrap_or_else(|| parent_dir(&path));
            let data = fs::read(&path)?;
            let outputs = xy_convert::convert::textures_to_png(&data);
            if outputs.is_empty() {
                eprintln!("no decodable textures in {}", path.display());
                return Ok(());
            }
            let written = xy_convert::convert::write_outputs(&outputs, &out)?;
            for w in &written {
                println!("{} -> {}", path.display(), w.display());
            }
        }
        ConvertCmd::Audio { path, out } => {
            let out = out.unwrap_or_else(|| path.with_extension("wav"));
            let data = fs::read(&path)?;
            if data.starts_with(b"CWAV") {
                let wav = xy_convert::convert::bcwav_to_wav(&data)?;
                fs::write(&out, wav)?;
            } else {
                xy_convert::media::decode_audio_wav(&path, &out)?;
            }
            println!("{} -> {}", path.display(), out.display());
        }
        ConvertCmd::Text { path, out } => {
            let data = fs::read(&path)?;
            let json = xy_convert::convert::text_bank_to_json(&data)?;
            match out {
                Some(o) => {
                    fs::write(&o, json)?;
                    println!("{} -> {}", path.display(), o.display());
                }
                None => print!("{json}"),
            }
        }
    }
    Ok(())
}

fn parent_dir(path: &Path) -> PathBuf {
    path.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn inspect_cmd(path: PathBuf, hashes: bool) -> Result<(), CliError> {
    let info = xy_convert::inspect::inspect(&path, hashes)?;
    println!("path     {}", info.path.display());
    println!("kind     {}", info.kind.label());
    println!(
        "magic    {}",
        if info.magic_hex.is_empty() {
            "(empty)".into()
        } else {
            info.magic_hex
        }
    );
    println!("size     {} bytes", info.size);
    if let Some(m) = info.modified {
        let secs = m
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        println!("modified {secs}");
    }
    println!("crc32    {:08X}", info.crc32);
    println!("fnv64    {:016X}", info.fnv64);
    if path.extension().and_then(|e| e.to_str()) == Some("glb") {
        if let Ok(bytes) = fs::read(&path) {
            match xy_convert::glb_check::validate(&bytes) {
                Ok(r) => {
                    println!("glTF     {}", r.summary());
                    for e in &r.errors {
                        println!("  glTF ERROR: {e}");
                    }
                }
                Err(e) => println!("glTF     INVALID: {e}"),
            }
        }
    }
    if let Some(sha) = info.sha256 {
        println!("sha256   {sha}");
    }
    if info.kind.has_models() || info.kind.has_textures() {
        // Content census for model/texture containers.
        let data = fs::read(&path)?;
        if info.kind.has_models() {
            if let Ok(models) = xy_convert::model::load_models(&data) {
                for m in &models {
                    println!(
                        "model    {} — {} meshes, {} verts, {} tris, {} bones, {} textures, {} anims",
                        m.name,
                        m.meshes.len(),
                        m.vertex_count(),
                        m.triangle_count(),
                        m.bones.len(),
                        m.textures.len(),
                        m.animations.len()
                    );
                    for w in &m.warnings {
                        println!("  note    {w}");
                    }
                }
            }
        }
        if info.kind.has_textures() {
            for t in xy_convert::model::collect_decoded_textures(&data) {
                println!(
                    "texture  {} — {}x{} {}",
                    t.name,
                    t.width,
                    t.height,
                    t.format.name()
                );
            }
        }
    }
    Ok(())
}

fn search_cmd(cmd: SearchCmd) -> Result<(), CliError> {
    match cmd {
        SearchCmd::Name { root, query, limit } => {
            let q = query.to_lowercase();
            for p in walk_matches(
                &root,
                &|p: &Path| {
                    p.file_name()
                        .map(|n| n.to_string_lossy().to_lowercase().contains(&q))
                        .unwrap_or(false)
                },
                limit,
            ) {
                println!("{}", p.display());
            }
            Ok(())
        }
        SearchCmd::Kind { root, kind, limit } => {
            let want = kind.to_lowercase();
            let pred = |p: &Path| -> bool {
                let head = std::fs::File::open(p)
                    .map(|mut f| {
                        use std::io::Read;
                        let mut buf = vec![0u8; 0x84];
                        let n = f.read(&mut buf).unwrap_or(0);
                        buf.truncate(n);
                        buf
                    })
                    .unwrap_or_default();
                let k = xy_convert::detect_kind(&head, p);
                let label = k.label().to_lowercase();

                label.contains(&want)
                    || (want == "model" && k.has_models())
                    || (want == "texture" && k.has_textures())
            };
            for p in walk_matches(&root, &pred, limit) {
                println!("{}", p.display());
            }
            Ok(())
        }
    }
}

/// Sorted, depth-first walk collecting files matching `pred` (by path).
fn walk_matches(root: &Path, pred: &dyn Fn(&Path) -> bool, limit: usize) -> Vec<PathBuf> {
    fn walk(dir: &Path, pred: &dyn Fn(&Path) -> bool, out: &mut Vec<PathBuf>, limit: usize) {
        if out.len() >= limit {
            return;
        }
        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
            .map(|rd| rd.filter_map(|e| e.ok().map(|e| e.path())).collect())
            .unwrap_or_default();
        entries.sort();
        for p in entries {
            if out.len() >= limit {
                return;
            }
            if p.is_dir() {
                walk(&p, pred, out, limit);
            } else if pred(&p) {
                out.push(p);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, pred, &mut out, limit);
    out
}

fn media_cmd(cmd: MediaCmd) -> Result<(), CliError> {
    match cmd {
        MediaCmd::Info { path } => {
            let info = xy_convert::media::probe(&path).map_err(CliError::Io)?;
            println!("format   {}", info.format_name);
            if let Some(d) = info.duration {
                println!("duration {d:.3} s");
            }
            for s in &info.streams {
                match s.codec_type.as_str() {
                    "video" => println!(
                        "video    {} {}x{}{}",
                        s.codec_name,
                        s.width,
                        s.height,
                        s.duration.map(|d| format!(" {d:.3}s")).unwrap_or_default()
                    ),
                    "audio" => println!(
                        "audio    {} {} Hz {} ch{}",
                        s.codec_name,
                        s.sample_rate,
                        s.channels,
                        s.duration.map(|d| format!(" {d:.3}s")).unwrap_or_default()
                    ),
                    other => println!("{other:<8}{}", s.codec_name),
                }
            }
            Ok(())
        }
        MediaCmd::Frames {
            path,
            out,
            fps,
            max,
            width,
        } => {
            let frames = xy_convert::media::extract_frames(&path, &out, fps, max, width)
                .map_err(CliError::Io)?;
            for f in &frames {
                println!("{} -> {}", path.display(), f.display());
            }
            Ok(())
        }
        MediaCmd::Mp4 { path, out } => {
            let out = out.unwrap_or_else(|| path.with_extension("mp4"));
            xy_convert::media::convert_video_mp4(&path, &out).map_err(CliError::Io)?;
            println!("{} -> {}", path.display(), out.display());
            Ok(())
        }
    }
}

fn manifest_cmd(cmd: ManifestCmd) -> Result<(), CliError> {
    match cmd {
        ManifestCmd::List { root, query, limit } => {
            let rows = xy_curate::manifest::read_manifest(&root.join("manifest.tsv"))?;
            let q = query.to_lowercase();
            let mut n = 0;
            for r in &rows {
                if !q.is_empty()
                    && !r.path.to_lowercase().contains(&q)
                    && !r.source.to_lowercase().contains(&q)
                {
                    continue;
                }
                println!("{:<4} {:<8} {} <- {}", n, r.status, r.path, r.source);
                n += 1;
                if n >= limit {
                    break;
                }
            }
            println!("({n} rows shown of {})", rows.len());
            Ok(())
        }
        ManifestCmd::Lookup { root, path } => {
            let rows = xy_curate::manifest::read_manifest(&root.join("manifest.tsv"))?;
            let rel = path
                .strip_prefix(&root)
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| path.display().to_string());
            // curated → source
            use std::io::Write as _;
            let mut out = std::io::stdout();
            let mut found = false;
            for r in &rows {
                if r.path == rel {
                    if writeln!(out, "curated   {}", r.path).is_err() {
                        return Ok(());
                    }
                    let _ = writeln!(out, "source    {}", r.source);
                    let _ = writeln!(out, "action    {} ({})", r.action, r.detail);
                    let _ = writeln!(out, "bytes     {} -> {}", r.in_bytes, r.out_bytes);
                    found = true;
                }
            }
            if found {
                return Ok(());
            }
            // Aggregate rows (family/module dirs) cover files that only
            // share a path segment (e.g. pokemon/models/<family>/x.obj is
            // tracked by the pokemon/families/<family>/ row), so match on
            // exact/prefix paths first, then on directory-name segments.
            // A bare file-name hit also resolves raw-side lookups passed as
            // absolute paths outside the tree.
            let basename = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            for r in &rows {
                let path_hit = rel.starts_with(&r.path) || r.path.starts_with(&rel);
                let source_hit = rel.contains(&r.source)
                    || r.source.contains(&rel)
                    || (!basename.is_empty() && r.source.contains(&basename));
                let segment_hit = r
                    .path
                    .split('/')
                    .filter(|seg| seg.ends_with('/'))
                    .any(|seg| seg.trim_end_matches('/').len() > 4 && rel.contains(seg))
                    || r.path
                        .split('/')
                        .any(|seg| seg.len() > 8 && rel.contains(seg));
                if path_hit || source_hit || segment_hit {
                    if writeln!(out, "curated   {}", r.path).is_err() {
                        return Ok(());
                    }
                    let _ = writeln!(out, "source    {}", r.source);
                    let _ = writeln!(out, "action    {} ({})", r.action, r.detail);
                    found = true;
                }
            }
            if !found {
                eprintln!("no manifest row references {}", path.display());
            }
            Ok(())
        }
        ManifestCmd::Validate { root } => {
            let rows = xy_curate::manifest::read_manifest(&root.join("manifest.tsv"))?;
            let mut checked = 0usize;
            let mut missing = 0usize;
            let mut size_mismatch = 0usize;
            for r in &rows {
                if r.status != "ok" || r.path.ends_with('/') {
                    continue;
                }
                checked += 1;
                let p = root.join(&r.path);
                match fs::metadata(&p) {
                    Ok(m) => {
                        if m.len() != r.out_bytes {
                            size_mismatch += 1;
                            println!(
                                "size   {} (disk {}, manifest {})",
                                r.path,
                                m.len(),
                                r.out_bytes
                            );
                        }
                    }
                    Err(_) => {
                        missing += 1;
                        println!("miss   {}", r.path);
                    }
                }
            }
            println!(
                "{checked} files checked, {missing} missing, {size_mismatch} size mismatches — {}",
                if missing + size_mismatch == 0 {
                    "PASS"
                } else {
                    "FAIL"
                }
            );
            Ok(())
        }
    }
}
