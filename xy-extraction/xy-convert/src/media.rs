//! ffmpeg/ffprobe orchestration for media preview and conversion.
//!
//! House rule (AGENTS.md): `.aac`/`.moflex` are standard-enough containers
//! that belong to ffmpeg — we shell out, never reimplement codecs.
//! Everything here is synchronous; callers run it on background threads.

use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// `true` when an `ffmpeg` binary is on PATH.
pub fn ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// `true` when an `ffprobe` binary is on PATH.
pub fn ffprobe_available() -> bool {
    Command::new("ffprobe")
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Cache root for media conversions (`$XY_UI_CACHE` override, else
/// `~/.cache/xy-ui`). Created on demand by the callers.
pub fn media_cache_dir() -> PathBuf {
    if let Ok(p) = std::env::var("XY_UI_CACHE") {
        return PathBuf::from(p);
    }
    dirs_cache().join("media")
}

fn dirs_cache() -> PathBuf {
    // dirs isn't a dependency here; resolve XDG by hand (Linux-first).
    std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(home).join(".cache")
        })
        .join("xy-ui")
}

/// One decoded audio track / video stream, from `ffprobe`.
#[derive(Debug, Clone, Default)]
pub struct StreamInfo {
    pub codec_type: String, // "audio" | "video"
    pub codec_name: String,
    pub width: u32,
    pub height: u32,
    pub sample_rate: u32,
    pub channels: u32,
    pub duration: Option<f64>,
    pub frame_count: Option<f64>,
}

/// Container-level metadata from `ffprobe`.
#[derive(Debug, Clone, Default)]
pub struct MediaInfo {
    pub format_name: String,
    pub duration: Option<f64>,
    pub streams: Vec<StreamInfo>,
}

impl MediaInfo {
    pub fn audio(&self) -> Option<&StreamInfo> {
        self.streams.iter().find(|s| s.codec_type == "audio")
    }
    pub fn video(&self) -> Option<&StreamInfo> {
        self.streams.iter().find(|s| s.codec_type == "video")
    }
    pub fn audio_duration(&self) -> Option<Duration> {
        self.audio()
            .and_then(|s| s.duration.or(self.duration))
            .map(Duration::from_secs_f64)
    }
}

/// Probe a media file with `ffprobe -print_format json`.
///
/// The JSON is parsed with a minimal scanner (the toolkit keeps serde out of
/// its library crates); we only need flat scalar fields of stream objects.
pub fn probe(path: &Path) -> io::Result<MediaInfo> {
    let out = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(path)
        .output()
        .map_err(|e| io::Error::other(format!("ffprobe spawn failed: {e}")))?;
    if !out.status.success() {
        return Err(io::Error::other("ffprobe failed"));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Ok(parse_ffprobe_json(&text))
}

/// Extract the flat fields we care about from ffprobe JSON: split the stream
/// objects out of the `"streams": [ … ]` array, then read `"key" : value`
/// pairs inside each.
fn parse_ffprobe_json(text: &str) -> MediaInfo {
    let mut info = MediaInfo::default();
    if let Some(name) = json_field(text, "format_name") {
        info.format_name = name.trim_matches('"').to_string();
    }
    info.duration = json_field(text, "duration").and_then(|v| v.trim_matches('"').parse().ok());

    let streams = slice_of(text, "\"streams\"");
    for obj in objects_in(&streams) {
        let mut s = StreamInfo {
            codec_type: json_field(&obj, "codec_type")
                .map(|v| v.trim_matches('"').to_string())
                .unwrap_or_default(),
            codec_name: json_field(&obj, "codec_name")
                .map(|v| v.trim_matches('"').to_string())
                .unwrap_or_default(),
            ..Default::default()
        };
        s.width = json_field(&obj, "width")
            .and_then(|v| v.trim_matches('"').parse().ok())
            .unwrap_or(0);
        s.height = json_field(&obj, "height")
            .and_then(|v| v.trim_matches('"').parse().ok())
            .unwrap_or(0);
        s.sample_rate = json_field(&obj, "sample_rate")
            .and_then(|v| v.trim_matches('"').parse().ok())
            .unwrap_or(0);
        s.channels = json_field(&obj, "channels")
            .and_then(|v| v.trim_matches('"').parse().ok())
            .unwrap_or(0);
        s.duration = json_field(&obj, "duration").and_then(|v| v.trim_matches('"').parse().ok());
        if s.codec_type.is_empty() && !s.codec_name.is_empty() {
            // codec_type missing: infer from which fields exist
            s.codec_type = if s.width > 0 {
                "video".into()
            } else {
                "audio".into()
            };
        }
        info.streams.push(s);
    }
    info
}

fn slice_of(text: &str, key: &str) -> String {
    let Some(pos) = text.find(key) else {
        return String::new();
    };
    let rest = &text[pos + key.len()..];
    let Some(open) = rest.find('[') else {
        return String::new();
    };
    let mut depth = 0usize;
    let mut end = open;
    for (i, c) in rest[open..].char_indices() {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end = open + i;
                    break;
                }
            }
            _ => {}
        }
    }
    rest[open..=end].to_string()
}

fn objects_in(array: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    let bytes: Vec<char> = array.chars().collect();
    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            '{' => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    out.push(bytes[start..=i].iter().collect());
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

fn json_field(obj: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\"");
    let pos = obj.find(&pat)?;
    let rest = obj[pos + pat.len()..].trim_start();
    let rest = rest.strip_prefix(':')?.trim_start();
    let end = rest.find([',', '}', '\n']).unwrap_or(rest.len());
    Some(rest[..end].trim().to_string())
}

/// Decode any audio input to a 16-bit PCM WAV at `out` (native rate/channels).
pub fn decode_audio_wav(input: &Path, out: &Path) -> io::Result<()> {
    run_ffmpeg(&[
        "-y".into(),
        "-i".into(),
        input.display().to_string(),
        "-c:a".into(),
        "pcm_s16le".into(),
        out.display().to_string(),
    ])
}

/// Convert a video container (e.g. `.moflex`) to H.264+AAC MP4 at `out`.
pub fn convert_video_mp4(input: &Path, out: &Path) -> io::Result<()> {
    run_ffmpeg(&[
        "-y".into(),
        "-i".into(),
        input.display().to_string(),
        "-c:v".into(),
        "libx264".into(),
        "-preset".into(),
        "fast".into(),
        "-crf".into(),
        "20".into(),
        "-c:a".into(),
        "aac".into(),
        "-movflags".into(),
        "+faststart".into(),
        out.display().to_string(),
    ])
}

/// Extract preview frames (`out_dir/frame_%05d.png`) at `fps`, capped at
/// `max_frames`. Returns the frame paths in order.
pub fn extract_frames(
    input: &Path,
    out_dir: &Path,
    fps: f32,
    max_frames: usize,
    scale_width: Option<u32>,
) -> io::Result<Vec<PathBuf>> {
    std::fs::create_dir_all(out_dir)?;
    let mut vf = format!("fps={fps}");
    if let Some(w) = scale_width {
        vf.push_str(&format!(",scale={w}:-1"));
    }
    run_ffmpeg(&[
        "-y".into(),
        "-i".into(),
        input.display().to_string(),
        "-vf".into(),
        vf,
        "-frames:v".into(),
        max_frames.to_string(),
        out_dir.join("frame_%05d.png").display().to_string(),
    ])?;
    let mut frames: Vec<PathBuf> = std::fs::read_dir(out_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .collect();
    frames.sort();
    Ok(frames)
}

/// Extract a single frame at timestamp `secs` to `out` (PNG).
pub fn extract_frame_at(input: &Path, secs: f64, out: &Path) -> io::Result<()> {
    run_ffmpeg(&[
        "-y".into(),
        "-ss".into(),
        format!("{secs:.3}"),
        "-i".into(),
        input.display().to_string(),
        "-frames:v".into(),
        "1".into(),
        out.display().to_string(),
    ])
}

fn run_ffmpeg(args: &[String]) -> io::Result<()> {
    let out = Command::new("ffmpeg")
        .args(["-hide_banner", "-loglevel", "error"])
        .args(args)
        .output()
        .map_err(|e| io::Error::other(format!("ffmpeg spawn failed: {e}")))?;
    if !out.status.success() {
        return Err(io::Error::other(format!(
            "ffmpeg failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ffprobe_json_parses() {
        let json = r#"{
  "streams": [
    { "codec_type": "video", "codec_name": "mobiclip", "width": 400, "height": 240, "duration": "12.500000" },
    { "codec_type": "audio", "codec_name": "adpcm_ima_moflex", "sample_rate": "32768", "channels": 2 }
  ],
  "format": { "format_name": "moflex", "duration": "12.500000" }
}"#;
        let info = parse_ffprobe_json(json);
        assert_eq!(info.format_name, "moflex");
        assert_eq!(info.duration, Some(12.5));
        assert_eq!(info.streams.len(), 2);
        let v = info.video().unwrap();
        assert_eq!((v.width, v.height), (400, 240));
        let a = info.audio().unwrap();
        assert_eq!(a.sample_rate, 32768);
        assert_eq!(a.channels, 2);
    }
}
