//! The local web UI host — axum, zero files written (port of `server.py`).
//!
//! Serves from memory only: a compact dataset JSON payload (gzipped when
//! the browser offers it), a cheap fingerprint endpoint for live reload,
//! scene JSON + `/view/*.svg|*.png` renderings produced by the very same
//! layout + renderer code the CLI exporter uses, and the built Svelte SPA
//! (embedded from `ui/dist` at compile time).

use std::sync::{Arc, Mutex};

use axum::extract::{Path, Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::collect::Source;
use crate::config::Config;
use crate::dataset::{dataset_to_api, Dataset};
use crate::error::{HeatmapError, Result};
use crate::views::{self, VIEW_NAMES};

/// Embedded SPA (built by `pnpm --dir ui build` — build.rs guarantees the
/// folder exists).
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/ui/dist"]
struct UiAssets;

/// Fingerprint-gated dataset cache shared across request threads.
struct Cache {
    source: Box<dyn Source + Send + Sync>,
    inner: Mutex<Option<(String, Arc<Dataset>)>>,
}

impl Cache {
    fn fingerprint(&self) -> Result<String> {
        self.source.fingerprint()
    }

    fn dataset(&self) -> Result<Arc<Dataset>> {
        let fp = self.source.fingerprint()?;
        let mut guard = self.inner.lock().unwrap();
        if let Some((cached_fp, ds)) = guard.as_ref() {
            if *cached_fp == fp {
                return Ok(ds.clone());
            }
        }
        let mut ds = self.source.dataset()?;
        ds.fingerprint = fp.clone();
        let ds = Arc::new(ds);
        *guard = Some((fp, ds.clone()));
        Ok(ds)
    }
}

type App = Arc<AppState>;

struct AppState {
    cache: Cache,
    watch: bool,
    config: Config,
}

fn err_response(status: StatusCode, message: String) -> Response {
    (status, Json(json!({"error": message}))).into_response()
}

fn heatmap_error_response(e: HeatmapError) -> Response {
    err_response(StatusCode::NOT_FOUND, e.message)
}

async fn api_dataset(State(app): State<App>) -> Response {
    match app.cache.dataset() {
        Ok(ds) => {
            let mut payload = dataset_to_api(&ds);
            if let Some(obj) = payload.as_object_mut() {
                obj.insert("warnings".into(), json!(ds.warnings));
                obj.insert("watch".into(), json!(app.watch));
                obj.insert("sources".into(), json!(app.cache.source.describe()));
            }
            Json(payload).into_response()
        }
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, e.message),
    }
}

async fn api_fingerprint(State(app): State<App>) -> Response {
    match app.cache.fingerprint() {
        Ok(fp) => Json(json!({"fingerprint": fp})).into_response(),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, e.message),
    }
}

async fn api_module(State(app): State<App>, Path(name): Path<String>) -> Response {
    let ds = match app.cache.dataset() {
        Ok(ds) => ds,
        Err(e) => return err_response(StatusCode::INTERNAL_SERVER_ERROR, e.message),
    };
    let m = match ds.module(&name) {
        Ok(m) => m,
        Err(e) => return heatmap_error_response(e),
    };
    let counts = m.tier_counts();
    let (first, last) = match (m.cells.first(), m.cells.last()) {
        (Some(f), Some(l)) => (format!("0x{:X}", f.addr), format!("0x{:X}", l.addr)),
        _ => (String::new(), String::new()),
    };
    let addr_range = if m.cells.is_empty() {
        None
    } else {
        Some(vec![first, last])
    };
    let named = m.cells.iter().filter(|c| !c.name.is_empty()).count() as u64;
    Json(json!({
        "name": m.name,
        "total": m.cells.len(),
        "tiers": { "raw": counts[0], "decompiled": counts[1], "named": counts[2], "verified": counts[3] },
        "covered": m.covered_count(),
        "named_symbols": named,
        "relaxed": m.hint_count(),
        "attempted": m.attempted_count(),
        "relink_ok": m.relink_ok,
        "addr_range": addr_range,
        "imports": m.imports,
    }))
    .into_response()
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
struct ViewParams {
    min: Option<i64>,
    cols: Option<i64>,
    bare: Option<String>,
    minor: Option<i64>,
    scale: Option<i64>,
    dpi: Option<i64>,
}

fn clamp(v: Option<i64>, lo: i64, hi: i64, default: i64) -> i64 {
    match v {
        Some(v) if (lo..=hi).contains(&v) => v,
        Some(v) => v.clamp(lo, hi),
        None => default,
    }
}

/// Shared view-spec parsing for /api/view (scene JSON) and /view (rendered).
fn resolve_spec(spec: &str) -> Result<(String, Option<String>)> {
    if let Some(module) = spec.strip_prefix("module-") {
        return Ok(("module".to_string(), Some(module.to_string())));
    }
    if VIEW_NAMES.contains(&spec) {
        return Ok((spec.to_string(), None));
    }
    Err(HeatmapError::new(format!("unknown view {spec:?}")))
}

async fn api_view(
    State(app): State<App>,
    Path(spec): Path<String>,
    Query(q): Query<ViewParams>,
) -> Response {
    let ds = match app.cache.dataset() {
        Ok(ds) => ds,
        Err(e) => return err_response(StatusCode::INTERNAL_SERVER_ERROR, e.message),
    };
    let (view, module) = match resolve_spec(&spec) {
        Ok(v) => v,
        Err(e) => return heatmap_error_response(e),
    };
    let min_tier = clamp(q.min, 0, 3, 0) as usize;
    let cols = clamp(q.cols, 16, 256, 0);
    let cols = if cols == 0 { None } else { Some(cols) };
    let minor_max = match q.minor {
        Some(m) if m < 0 => None,
        Some(m) => Some(m.min(100_000)),
        None => None,
    };
    match views::build_scene(
        &ds,
        &view,
        module.as_deref(),
        &app.config,
        min_tier,
        cols,
        q.bare.as_deref() == Some("1"),
        minor_max,
    ) {
        Ok(scene) => Json(&scene).into_response(),
        Err(e) => heatmap_error_response(e),
    }
}

async fn view_file(
    State(app): State<App>,
    Path(spec): Path<String>,
    Query(q): Query<ViewParams>,
) -> Response {
    let (base, ext) = match spec.rsplit_once('.') {
        Some((b, e)) if (b.is_empty() || e.is_empty()) => {
            return err_response(StatusCode::NOT_FOUND, format!("bad view path {spec:?}"))
        }
        Some((b, e)) if e == "svg" || e == "png" => (b.to_string(), e.to_string()),
        _ => return err_response(StatusCode::NOT_FOUND, format!("bad view path {spec:?}")),
    };
    let ds = match app.cache.dataset() {
        Ok(ds) => ds,
        Err(e) => return err_response(StatusCode::INTERNAL_SERVER_ERROR, e.message),
    };
    let (view, module) = match resolve_spec(&base) {
        Ok(v) => v,
        Err(e) => return heatmap_error_response(e),
    };
    let min_tier = clamp(q.min, 0, 3, 0) as usize;
    let cols = clamp(q.cols, 16, 256, 0);
    let cols = if cols == 0 { None } else { Some(cols) };
    let bare = q.bare.as_deref() == Some("1");
    let minor_max = match q.minor {
        Some(m) if m < 0 => None,
        Some(m) => Some(m.min(100_000)),
        None => None,
    };
    let scene = match views::build_scene(
        &ds,
        &view,
        module.as_deref(),
        &app.config,
        min_tier,
        cols,
        bare,
        minor_max,
    ) {
        Ok(s) => s,
        Err(e) => return heatmap_error_response(e),
    };
    if ext == "svg" {
        let body = crate::svg::render_svg(&scene);
        return (
            [(header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
            body,
        )
            .into_response();
    }
    let scale = clamp(q.scale, 1, 8, 2);
    let dpi = q.dpi.map(|d| d.clamp(36, 1200));
    match crate::png::png_bytes(&scene, scale, dpi, app.config.font_path.as_deref()) {
        Ok(png) => ([(header::CONTENT_TYPE, "image/png")], png.data).into_response(),
        Err(e) => err_response(StatusCode::INTERNAL_SERVER_ERROR, e.message),
    }
}

async fn spa_index() -> Response {
    match UiAssets::get("index.html") {
        Some(content) => {
            let body = content.data.into_owned();
            ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], body).into_response()
        }
        None => err_response(
            StatusCode::NOT_FOUND,
            "UI not built — run `pnpm --dir ui build` first".to_string(),
        ),
    }
}

async fn spa_asset(Path(path): Path<String>) -> Response {
    // the route wildcard captures relative to /assets/, but the embed keys
    // are relative to ui/dist — re-join the prefix
    match UiAssets::get(&format!("assets/{path}")) {
        Some(content) => {
            let mime = mime_for(&path);
            let body = content.data.into_owned();
            ([(header::CONTENT_TYPE, mime)], body).into_response()
        }
        None => spa_index().await, // SPA fallback for client-side routes
    }
}

fn mime_for(path: &str) -> &'static str {
    match path.rsplit_once('.').map(|(_, e)| e) {
        Some("js") | Some("mjs") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("html") => "text/html; charset=utf-8",
        Some("json") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}

async fn favicon() -> Response {
    StatusCode::NO_CONTENT.into_response()
}

pub fn router(source: Box<dyn Source + Send + Sync>, watch: bool, config: Config) -> Router {
    let state: App = Arc::new(AppState {
        cache: Cache {
            source,
            inner: Mutex::new(None),
        },
        watch,
        config,
    });
    let api = Router::new()
        .route("/dataset", get(api_dataset))
        .route("/fingerprint", get(api_fingerprint))
        .route("/module/{name}", get(api_module))
        .route("/view/{spec}", get(api_view))
        .layer(tower_http::compression::CompressionLayer::new().gzip(true));
    Router::new()
        .nest("/api", api)
        .route("/", get(spa_index))
        .route("/favicon.ico", get(favicon))
        .route("/view/{spec}", get(view_file))
        .route("/assets/{*path}", get(spa_asset))
        .with_state(state)
}

/// Bind, print the URL, and block until Ctrl-C (port of `serve()`).
pub async fn serve(
    source: Box<dyn Source + Send + Sync>,
    describe: String,
    config: Config,
    host: &str,
    port: u16,
    watch: bool,
    open_browser: bool,
) -> Result<()> {
    let app = router(source, watch, config);
    let listener = tokio::net::TcpListener::bind((host, port))
        .await
        .map_err(|e| HeatmapError::new(format!("cannot bind {host}:{port}: {e}")))?;
    let addr = listener.local_addr().ok();
    let url = format!("http://{}", addr.map(|a| a.to_string()).unwrap_or_default());
    println!(
        "xy-heatmap {} serving {describe}",
        env!("CARGO_PKG_VERSION")
    );
    println!("  -> {url}   (Ctrl-C to stop; nothing is written to disk)");
    if open_browser {
        let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
    }
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .map_err(|e| HeatmapError::new(format!("server error: {e}")))?;
    println!("stopped.");
    Ok(())
}

#[cfg(test)]
mod embed_tests {
    use super::UiAssets;
    #[test]
    fn embed_lists_files() {
        let all: Vec<String> = UiAssets::iter().map(|p| p.to_string()).collect();
        eprintln!("embedded files: {all:?}");
        assert!(
            UiAssets::get("index.html").is_some(),
            "index.html missing from embed"
        );
        // pick whatever hashed bundle the last `pnpm build` produced
        let js = all
            .iter()
            .find(|p| p.starts_with("assets/") && p.ends_with(".js"));
        assert!(
            js.is_some(),
            "no JS bundle in embed (run pnpm --dir ui build)"
        );
        eprintln!(
            "index.html bytes: {}",
            UiAssets::get("index.html").unwrap().data.len()
        );
    }
}

#[cfg(test)]
mod router_tests {
    use super::router;
    use crate::collect::FileDatasetSource;
    use crate::config::Config;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // oneshot

    fn test_app() -> axum::Router {
        let dir = std::env::temp_dir().join("xy-heatmap-router-test");
        std::fs::create_dir_all(&dir).unwrap();
        let ds = dir.join("ds.json");
        std::fs::write(
            &ds,
            r#"{"modules":[{"name":"M","cells":[{"addr":"0x180","decompiled":true}]}]}"#,
        )
        .unwrap();
        router(
            Box::new(FileDatasetSource::new(ds).unwrap()),
            false,
            Config::default(),
        )
    }

    #[tokio::test]
    async fn serves_embedded_assets() {
        // pick whatever hashed bundle the last `pnpm build` produced
        let js = super::UiAssets::iter()
            .find_map(|p| {
                let s = p.to_string();
                s.strip_prefix("assets/")
                    .filter(|_| s.ends_with(".js"))
                    .map(str::to_string)
            })
            .expect("no JS bundle in embed (run pnpm --dir ui build)");
        let app = test_app();
        let res = app
            .oneshot(
                Request::builder()
                    .uri(format!("/assets/{js}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let ctype = res.headers().get("content-type").unwrap().to_str().unwrap();
        assert!(ctype.starts_with("text/javascript"), "got {ctype}");
    }

    #[tokio::test]
    async fn serves_index_and_api() {
        let app = test_app();
        for uri in ["/", "/api/fingerprint", "/api/view/treemap"] {
            let res = app
                .clone()
                .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(res.status(), StatusCode::OK, "{uri}");
        }
    }
}
