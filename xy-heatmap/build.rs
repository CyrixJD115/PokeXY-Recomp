use std::path::Path;

fn main() {
    // rust-embed requires the embedded folder to exist at compile time; the
    // real SPA lands here via `pnpm --dir ui build`. A stub keeps plain
    // `cargo build/test` working before the first UI build.
    let dist = Path::new("ui/dist");
    if !dist.join("index.html").is_file() {
        let _ = std::fs::create_dir_all(dist.join("assets"));
        let _ = std::fs::write(
            dist.join("index.html"),
            "<!doctype html><title>xy-heatmap</title><p>UI not built yet — run <code>pnpm --dir ui build</code>.</p>",
        );
    }
    println!("cargo:rerun-if-changed=ui/dist/index.html");
}
