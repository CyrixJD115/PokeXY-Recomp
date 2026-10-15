//! The xy-heatmap command line — the package's one entry point (port of
//! `cli.py`): serve | export | summary | readme.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::collect::{discover_data_root, FileDatasetSource, ProjectSource, Source};
use crate::config::Config;
use crate::error::{HeatmapError, Result};

#[derive(Parser)]
#[command(
    name = "xy-heatmap",
    version,
    about = "Standalone progress-heatmap studio: local web UI + SVG/PNG export."
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(ValueEnum, Clone, Copy)]
enum SortMode {
    Name,
    Count,
}

#[derive(Subcommand)]
enum Cmd {
    /// host the interactive web UI (writes nothing to disk)
    Serve {
        #[arg(long)]
        data_root: Option<PathBuf>,
        #[arg(long)]
        dataset: Option<PathBuf>,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 8765)]
        port: u16,
        #[arg(long)]
        watch: bool,
        #[arg(long)]
        open_browser: bool,
    },
    /// write SVG/PNG exports directly
    Export {
        #[arg(long)]
        data_root: Option<PathBuf>,
        #[arg(long)]
        dataset: Option<PathBuf>,
        #[arg(long)]
        width: Option<i64>,
        #[arg(long)]
        view: String,
        #[arg(long, default_value = "svg")]
        format: String,
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value_t = 2)]
        scale: i64,
        #[arg(long)]
        dpi: Option<i64>,
        #[arg(long, default_value_t = 0)]
        min_tier: usize,
        #[arg(long)]
        cols: Option<i64>,
        #[arg(long)]
        minor_max: Option<i64>,
        #[arg(long)]
        quiet: bool,
    },
    /// per-module text table to stdout
    Summary {
        #[arg(long)]
        data_root: Option<PathBuf>,
        #[arg(long)]
        dataset: Option<PathBuf>,
        #[arg(value_enum, long, default_value_t = SortMode::Name)]
        sort: SortMode,
        #[arg(long)]
        quiet: bool,
    },
    /// regenerate the README heatmap assets (treemap + progress)
    Readme {
        #[arg(long)]
        data_root: Option<PathBuf>,
        #[arg(long)]
        dataset: Option<PathBuf>,
        #[arg(long)]
        width: Option<i64>,
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long)]
        quiet: bool,
    },
}

fn resolve_source(
    data_root: &Option<PathBuf>,
    dataset: &Option<PathBuf>,
) -> Result<(Box<dyn Source + Send + Sync>, Option<PathBuf>)> {
    if let Some(path) = dataset {
        return Ok((Box::new(FileDatasetSource::new(path)?), None));
    }
    if let Some(root) = data_root {
        return Ok((Box::new(ProjectSource::new(root)?), Some(root.clone())));
    }
    if let Ok(env) = std::env::var("XY_HEATMAP_DATA_ROOT") {
        if !env.is_empty() {
            let root = PathBuf::from(env);
            return Ok((Box::new(ProjectSource::new(&root)?), Some(root)));
        }
    }
    let root = discover_data_root(None)?;
    let src_root = root.clone();
    Ok((Box::new(ProjectSource::new(root)?), Some(src_root)))
}

/// The view spec for `--view`, clamped tier filter, and render options.
struct ExportOpts {
    view: String,
    format: String,
    out: PathBuf,
    scale: i64,
    dpi: Option<i64>,
    min_tier: usize,
    cols: Option<i64>,
    minor_max: Option<i64>,
    quiet: bool,
}

fn cmd_export(
    data_root: &Option<PathBuf>,
    dataset: &Option<PathBuf>,
    width: Option<i64>,
    opts: ExportOpts,
) -> Result<()> {
    let ds = resolve_source(data_root, dataset)?.0.dataset()?;
    let cfg = config(width);
    let fmt = opts.format.as_str();
    let out = &opts.out;
    let min_tier = opts.min_tier.min(3);

    struct Target {
        view: String,
        module: Option<String>,
        path: PathBuf,
    }
    let targets: Vec<Target> = if opts.view == "all" {
        if out.extension().is_some() {
            return Err(HeatmapError::new(
                "--out must be a directory when --view all",
            ));
        }
        crate::views::VIEW_NAMES
            .iter()
            .map(|name| Target {
                view: name.to_string(),
                module: None,
                path: out.join(crate::views::view_filename(name, None, fmt)),
            })
            .collect()
    } else {
        let (view, module) = crate::views::parse_view(&opts.view)?;
        vec![Target {
            view,
            module,
            path: out.clone(),
        }]
    };

    let mut written = 0u32;
    for t in &targets {
        let scene = crate::views::build_scene(
            &ds,
            &t.view,
            t.module.as_deref(),
            &cfg,
            min_tier,
            opts.cols,
            false,
            opts.minor_max,
        )?;
        if let Some(parent) = t.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if fmt == "svg" {
            let data = crate::svg::render_svg(&scene);
            std::fs::write(&t.path, &data)?;
            let kb = data.len() as f64 / 1024.0;
            println!(
                "wrote {} ({}×{}, {kb:.1} KB)",
                t.path.display(),
                scene.width as i64,
                scene.height as i64,
            );
        } else if fmt == "png" {
            let png =
                crate::png::png_bytes(&scene, opts.scale, opts.dpi, cfg.font_path.as_deref())?;
            std::fs::write(&t.path, &png.data)?;
            let kb = png.data.len() as f64 / 1024.0;
            println!(
                "wrote {} ({}×{} px @ dpi {}, {kb:.1} KB)",
                t.path.display(),
                png.width,
                png.height,
                png.dpi,
            );
        } else {
            return Err(HeatmapError::new(format!(
                "unknown format {fmt:?} (svg | png)"
            )));
        }
        written += 1;
    }
    if !opts.quiet {
        println!(
            "{} file{} written.",
            written,
            if written != 1 { "s" } else { "" }
        );
    }
    Ok(())
}

fn cmd_readme(
    data_root: &Option<PathBuf>,
    dataset: &Option<PathBuf>,
    width: Option<i64>,
    out: &Option<PathBuf>,
    quiet: bool,
) -> Result<()> {
    let out = match out {
        Some(o) => o.clone(),
        None => {
            // default: <repo>/docs/assets next to the discovered data root
            match resolve_source(data_root, dataset)?.1 {
                Some(root) => root
                    .parent()
                    .and_then(|p| p.parent())
                    .map(|repo| repo.join("docs").join("assets"))
                    .unwrap_or_else(|| PathBuf::from("docs").join("assets")),
                None => PathBuf::from("docs").join("assets"),
            }
        }
    };
    std::fs::create_dir_all(&out)?;
    let targets = [
        ("treemap", "svg", "heatmap-treemap.svg"),
        ("overview", "svg", "heatmap-progress.svg"),
        // the overall progress banner goes into the README as a PNG
        ("progress", "png", "heatmap-progress-bar.png"),
    ];
    for (view, format, file) in targets {
        cmd_export(
            data_root,
            dataset,
            width,
            ExportOpts {
                view: view.to_string(),
                format: format.to_string(),
                out: out.join(file),
                scale: 2,
                dpi: None,
                min_tier: 0,
                cols: None,
                minor_max: None,
                quiet,
            },
        )?;
    }
    Ok(())
}

#[allow(dead_code)] // used by export/readme once layout lands
fn config(width: Option<i64>) -> Config {
    let mut cfg = Config::default();
    if let Some(w) = width {
        cfg.width = w;
    }
    cfg
}

/// Python `{:,}` — thousands separators (summary parity).
fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    let digits = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (digits - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

fn pct(n: u64, d: u64) -> String {
    if d == 0 {
        "0.0%".to_string()
    } else {
        format!("{:.1}%", 100.0 * n as f64 / d as f64)
    }
}

fn cmd_summary(
    data_root: &Option<PathBuf>,
    dataset: &Option<PathBuf>,
    sort: SortMode,
    quiet: bool,
) -> Result<()> {
    let ds = resolve_source(data_root, dataset)?.0.dataset()?;
    let tier_names = ["raw", "decompiled", "named", "verified"];
    let mut modules = ds.modules.clone();
    match sort {
        SortMode::Name => modules.sort_by(|a, b| a.name.cmp(&b.name)),
        SortMode::Count => {
            modules.sort_by(|a, b| b.cells.len().cmp(&a.cells.len()).then(a.name.cmp(&b.name)))
        }
    }

    let name_w = modules
        .iter()
        .map(|m| m.name.len())
        .chain(std::iter::once("module".len()))
        .max()
        .unwrap_or(6)
        + 2;
    let header = format!(
        "{:<name_w$}{:>8}{:>9}{:>11}{:>8}{:>9}  relink",
        "module", "fns", "covered", "decompiled", "named", "verified",
    );
    println!("{header}");
    println!("{}", "-".repeat(header.len()));
    for m in &modules {
        let total = m.cells.len() as u64;
        let counts = m.tier_counts();
        let relink = match m.relink_ok {
            None => "-",
            Some(true) => "ok",
            Some(false) => "MISMATCH",
        };
        println!(
            "{:<name_w$}{:>8}{:>9}{:>11}{:>8}{:>9}  {relink}",
            m.name,
            thousands(total),
            pct(m.covered_count(), total),
            pct(counts[1], total),
            pct(counts[2], total),
            pct(counts[3], total),
        );
    }
    let total = ds.total_cells();
    let counts = ds.tier_counts();
    let relink_s = ds
        .relink_summary()
        .map(|(ok, all)| format!("  {ok}/{all}"))
        .unwrap_or_default();
    println!("{}", "-".repeat(header.len()));
    println!(
        "{:<name_w$}{:>8}{:>9}{:>11}{:>8}{:>9}{relink_s}",
        "TOTAL",
        thousands(total),
        pct(ds.covered_count(), total),
        pct(counts[1], total),
        pct(counts[2], total),
        pct(counts[3], total),
    );
    let _ = tier_names;
    if !ds.warnings.is_empty() && !quiet {
        eprintln!("\n{} collector warning(s):", ds.warnings.len());
        for w in ds.warnings.iter().take(8) {
            eprintln!("  - {w}");
        }
        if ds.warnings.len() > 8 {
            eprintln!("  … and {} more", ds.warnings.len() - 8);
        }
    }
    Ok(())
}

pub fn run() {
    let cli = Cli::parse();
    let result = match &cli.command {
        Cmd::Summary {
            data_root,
            dataset,
            sort,
            quiet,
        } => cmd_summary(data_root, dataset, *sort, *quiet),
        Cmd::Export {
            data_root,
            dataset,
            width,
            view,
            format,
            out,
            scale,
            dpi,
            min_tier,
            cols,
            minor_max,
            quiet,
        } => cmd_export(
            data_root,
            dataset,
            *width,
            ExportOpts {
                view: view.clone(),
                format: format.clone(),
                out: out.clone(),
                scale: *scale,
                dpi: *dpi,
                min_tier: *min_tier,
                cols: *cols,
                minor_max: *minor_max,
                quiet: *quiet,
            },
        ),
        Cmd::Readme {
            data_root,
            dataset,
            width,
            out,
            quiet,
        } => cmd_readme(data_root, dataset, *width, out, *quiet),
        Cmd::Serve {
            data_root,
            dataset,
            host,
            port,
            watch,
            open_browser,
        } => match resolve_source(data_root, dataset) {
            Ok((source, _)) => {
                let describe = source.describe();
                match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt.block_on(crate::server::serve(
                        source,
                        describe,
                        Config::default(),
                        host,
                        *port,
                        *watch,
                        *open_browser,
                    )),
                    Err(e) => Err(HeatmapError::new(format!(
                        "cannot start async runtime: {e}"
                    ))),
                }
            }
            Err(e) => Err(e),
        },
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
