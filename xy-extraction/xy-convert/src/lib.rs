//! `xy-convert` — viewer routing, inspection, media orchestration and
//! one-shot conversion APIs on top of the format crates.
//!
//! This is the backend the GUI (and `xy convert` CLI) talks to: it knows how
//! to sniff a file into a [`FileKind`] (which viewer owns it), decode models
//! into the UI-neutral [`model::SceneModel`] intermediate, collect textures
//! as decoded RGBA, convert audio/video through ffmpeg, and produce
//! standard-format outputs (PNG / WAV / GLB / JSON) as in-memory bytes.
//!
//! No UI dependencies live here — everything is plain data + `std::process`.

pub mod convert;
pub mod glb_check;
pub mod inspect;
pub mod media;
pub mod model;
pub mod render;

/// What a file is, and therefore which viewer owns it.
///
/// Detection combines magic bytes (up to 0x84 read-ahead for CRO, whose
/// magic sits at 0x80), GameFreak container tags, and extension fallbacks
/// for standard formats (PNG/JSON/…).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileKind {
    /// GameFreak sub-file container (`PC`/`PF`/`PT`/…); flags say what its
    /// entries decode to.
    PokemonContainer {
        models: bool,
        textures: bool,
    },
    /// BCH (CtrH3D) model/texture container.
    Bch {
        models: bool,
        textures: bool,
    },
    /// CGFX (BCRES) model/texture container.
    Cgfx {
        models: bool,
        textures: bool,
    },
    Image,
    AudioWav,
    AudioBcwav,
    /// Other audio (ADTS AAC, …) — preview via ffmpeg decode.
    AudioMedia,
    Video,
    /// Decoded text bank JSON (already standard).
    TextJson,
    /// Source/text file (C, C#, TSV, MD, plain).
    Text,
    Cro,
    Garc,
    Darc,
    Romfs,
    Cci,
    /// Nintendo LZ10/LZ11 stream (probable).
    Compressed,
    Unknown,
}

impl FileKind {
    /// Short human label for UI lists.
    pub fn label(&self) -> &'static str {
        match self {
            FileKind::PokemonContainer { .. } => "GF container (PC/PF/PT)",
            FileKind::Bch { .. } => "BCH model",
            FileKind::Cgfx { .. } => "CGFX model",
            FileKind::Image => "image",
            FileKind::AudioWav => "WAV audio",
            FileKind::AudioBcwav => "BCWAV audio",
            FileKind::AudioMedia => "audio (ffmpeg)",
            FileKind::Video => "video",
            FileKind::TextJson => "JSON",
            FileKind::Text => "text/source",
            FileKind::Cro => "CRO module",
            FileKind::Garc => "GARC archive",
            FileKind::Darc => "DARC archive",
            FileKind::Romfs => "RomFS image",
            FileKind::Cci => "CCI card image",
            FileKind::Compressed => "LZ-compressed",
            FileKind::Unknown => "unknown",
        }
    }

    /// Does this kind carry 3D models (3D viewer applicable)?
    pub fn has_models(&self) -> bool {
        match self {
            FileKind::PokemonContainer { models, .. }
            | FileKind::Bch { models, .. }
            | FileKind::Cgfx { models, .. } => *models,
            _ => false,
        }
    }

    /// Does this kind carry decodable textures (texture viewer applicable)?
    pub fn has_textures(&self) -> bool {
        match self {
            FileKind::PokemonContainer { textures, .. }
            | FileKind::Bch { textures, .. }
            | FileKind::Cgfx { textures, .. } => *textures,
            FileKind::Image => true,
            _ => false,
        }
    }
}

/// Classify a file from its first bytes (+ extension fallback).
///
/// `head` should carry at least 0x84 bytes when available (CRO magic lives
/// at 0x80); shorter slices are fine for the other formats.
pub fn detect_kind(head: &[u8], path: &std::path::Path) -> FileKind {
    let _ = path; // reserved: extension tie-breaks for unknown magics below
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();

    if head.len() > 0x84 && &head[0x80..0x84] == b"CRO0" {
        return FileKind::Cro;
    }
    if head.starts_with(b"BCH\x00") {
        let (models, textures) = bch_content(head);
        return FileKind::Bch { models, textures };
    }
    if head.starts_with(b"CGFX") {
        let (models, textures) = cgfx_content(head);
        return FileKind::Cgfx { models, textures };
    }
    if head.len() >= 2 && head[0].is_ascii_uppercase() && head[1].is_ascii_uppercase() {
        if let Ok(c) = xy_gfx::pkmn::PkmnContainer::parse(head) {
            let (models, textures) = container_content(&c);
            return FileKind::PokemonContainer { models, textures };
        }
    }
    match head {
        _ if head.starts_with(b"CRAG") => FileKind::Garc,
        _ if head.starts_with(b"darc") => FileKind::Darc,
        _ if head.starts_with(b"NCSD") => FileKind::Cci,
        _ if head.starts_with(b"NCCH") || head.starts_with(b"IVFC") => FileKind::Romfs,
        _ if head.starts_with(b"CWAV") => FileKind::AudioBcwav,
        _ if head.starts_with(b"RIFF") => FileKind::AudioWav,
        _ if head.starts_with(b"\x89PNG") => FileKind::Image,
        _ if !head.is_empty() && (head[0] == 0x10 || head[0] == 0x11) => FileKind::Compressed,
        _ => match ext.as_str() {
            "png" | "jpg" | "jpeg" | "bmp" | "gif" | "webp" => FileKind::Image,
            "json" => FileKind::TextJson,
            "wav" => FileKind::AudioWav,
            "bcwav" => FileKind::AudioBcwav,
            "aac" | "m4a" | "mp3" | "ogg" | "flac" | "bcstm" | "brstm" | "bwav" => {
                FileKind::AudioMedia
            }
            "mp4" | "moflex" | "mkv" | "webm" | "avi" | "mov" => FileKind::Video,
            "c" | "h" | "cs" | "txt" | "tsv" | "md" | "csv" | "sh" | "rs" | "py" | "gcs"
            | "csproj" | "asm" | "s" => FileKind::Text,
            "cro" | "crs" => FileKind::Cro,
            "garc" => FileKind::Garc,
            _ => FileKind::Unknown,
        },
    }
}

fn bch_content(data: &[u8]) -> (bool, bool) {
    match xy_gfx::bch::Bch::parse(data) {
        Ok(b) => (!b.models.is_empty(), !b.textures.is_empty()),
        Err(_) => (false, false),
    }
}

fn cgfx_content(data: &[u8]) -> (bool, bool) {
    match xy_gfx::cgfx::Cgfx::parse(data) {
        Ok(c) => (!c.models.is_empty(), !c.textures.is_empty()),
        Err(_) => (false, false),
    }
}

fn container_content(c: &xy_gfx::pkmn::PkmnContainer<'_>) -> (bool, bool) {
    let mut models = false;
    let mut textures = false;
    for e in &c.entries {
        if e.starts_with(b"BCH\x00") {
            if let Ok(b) = xy_gfx::bch::Bch::parse(e) {
                models |= !b.models.is_empty();
                textures |= !b.textures.is_empty();
            }
        } else if e.starts_with(b"CGFX") {
            if let Ok(g) = xy_gfx::cgfx::Cgfx::parse(e) {
                models |= !g.models.is_empty();
                textures |= !g.textures.is_empty();
            }
        }
    }
    (models, textures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn magics_route() {
        assert_eq!(
            detect_kind(b"CGFX", Path::new("a.bin")),
            FileKind::Cgfx {
                models: false,
                textures: false
            }
        );
        assert_eq!(detect_kind(b"CRAG", Path::new("a")), FileKind::Garc);
        assert_eq!(detect_kind(b"\x89PNG", Path::new("a.png")), FileKind::Image);
        assert_eq!(detect_kind(b"RIFF", Path::new("a.wav")), FileKind::AudioWav);
        assert_eq!(detect_kind(b"garbage", Path::new("a.c")), FileKind::Text);
        assert_eq!(
            detect_kind(b"garbage", Path::new("a.xyz")),
            FileKind::Unknown
        );
        let mut cro = vec![0u8; 0x84];
        cro[0x80..0x84].copy_from_slice(b"CRO0");
        assert_eq!(detect_kind(&cro, Path::new("a.cro")), FileKind::Cro);
    }
}
