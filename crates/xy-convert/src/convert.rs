//! One-shot conversion APIs: proprietary in → standard-format bytes out.
//!
//! Every function returns in-memory outputs ([`OutputFile`]) so callers
//! (GUI curation workspace, CLI) decide where they land — the curated tree,
//! an export dialog target, or a cache directory.

use std::io::Write;
use std::path::Path;

use xy_core::error::Result;
use xy_gfx::texture::write_png;

use crate::model::{self};

/// A named output artifact ready to write to disk.
#[derive(Debug, Clone)]
pub struct OutputFile {
    pub name: String,
    pub bytes: Vec<u8>,
}

/// Decode a WAV header + samples into RIFF bytes (44-byte canonical header,
/// same layout as `xy-audio`'s writer — duplicated here so the GUI backend
/// needs no `Write` target).
pub fn wav_bytes(sample_rate: u32, channels: u16, samples_i16: &[i16]) -> Vec<u8> {
    let data_len = samples_i16.len() * 2;
    let mut out = Vec::with_capacity(44 + data_len);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len as u32).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&channels.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&(sample_rate * channels as u32 * 2).to_le_bytes());
    out.extend_from_slice(&(channels * 2).to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data_len as u32).to_le_bytes());
    for s in samples_i16 {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

/// BCWAV → WAV bytes (decode via the DSP-ADPCM/PCM decoder).
pub fn bcwav_to_wav(data: &[u8]) -> Result<Vec<u8>> {
    let bcwav = xy_audio::bcwav::Bcwav::parse(data)?;
    let mut out = Vec::new();
    bcwav.write_wav(&mut out)?;
    Ok(out)
}

/// Decode a text bank to pretty JSON (one array entry per line), matching
/// `xy script text` output.
pub fn text_bank_to_json(data: &[u8]) -> Result<String> {
    let tf = xy_script::textfile::TextFile::parse(data)?;
    let lines = tf.lines()?;
    let mut out = String::from("[\n");
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            out.push_str(",\n");
        }
        out.push_str("  ");
        out.push_str(&xy_core::util::json_escape(line));
    }
    out.push_str("\n]\n");
    Ok(out)
}

/// Collect and PNG-encode every texture in a payload
/// (`<name>_<WxH>.png` outputs; unique suffixes on collision).
pub fn textures_to_png(data: &[u8]) -> Vec<OutputFile> {
    let mut used = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for tex in model::collect_decoded_textures(data) {
        let dims = format!("{}x{}", tex.width, tex.height);
        let mut name = tex.name.clone();
        let mut n = 1;
        while !used.insert(format!("{name}_{dims}")) {
            n += 1;
            name = format!("{}_{n}", tex.name);
        }
        let mut png = Vec::new();
        let mut cur = std::io::Cursor::new(&mut png);
        if write_png(
            &tex.rgba,
            tex.width as u32,
            tex.height as u32,
            &mut std::io::BufWriter::new(&mut cur),
        )
        .is_ok()
        {
            out.push(OutputFile {
                name: format!("{name}_{dims}.png"),
                bytes: png,
            });
        }
    }
    out
}

/// Models → one GLB (binary glTF 2.0) per model, textures embedded.
pub fn models_to_glb(data: &[u8]) -> Result<Vec<OutputFile>> {
    models_to_glb_with_textures(data, None)
}

/// Like [`models_to_glb`], but first merges the decoded textures of a
/// texture container (family PT/CGFX/BCH pack) into every scene — model
/// PCs carry no textures, and materials bind them by name.
pub fn models_to_glb_with_textures(
    data: &[u8],
    texture_pack: Option<&[u8]>,
) -> Result<Vec<OutputFile>> {
    let extra: Vec<model::DecodedTexture> = texture_pack
        .map(model::collect_decoded_textures)
        .unwrap_or_default();
    let mut out = Vec::new();
    let mut used = std::collections::BTreeSet::new();
    for mut scene in model::load_models(data)? {
        if scene.meshes.is_empty() && scene.textures.is_empty() {
            continue;
        }
        if !extra.is_empty() {
            scene.textures.extend(extra.iter().cloned());
        }
        let glb = xy_gfx::gltf::export_glb(&gltf_scene_from(&scene))
            .map_err(|e| xy_core::Error::invalid("glb", e.to_string()))?;
        // Containers repeat model names (e.g. 4× TestPlane) — suffix them.
        let base = safe_stem(&scene.name);
        let mut name = format!("{base}.glb");
        let mut n = 1;
        while !used.insert(name.clone()) {
            n += 1;
            name = format!("{base}_{n}.glb");
        }
        out.push(OutputFile { name, bytes: glb });
    }
    Ok(out)
}

/// Map the UI-neutral scene onto xy-gfx's GLB input type.
fn gltf_scene_from(s: &model::SceneModel) -> xy_gfx::gltf::GltfScene {
    xy_gfx::gltf::GltfScene {
        name: s.name.clone(),
        meshes: s
            .meshes
            .iter()
            .map(|m| xy_gfx::gltf::GltfMesh {
                name: m.name.clone(),
                material_index: if m.material_index < s.materials.len() {
                    Some(m.material_index)
                } else {
                    None
                },
                positions: m.positions.clone(),
                normals: m.normals.clone(),
                // SceneModel stores UVs as in the file; glTF's texture origin
                // is the top-left, so flip v on the way out (same as OBJ).
                uvs: m.uvs.iter().map(|u| [u[0], 1.0 - u[1]]).collect(),
                colors: m.colors.clone(),
                joints: m.joints.clone(),
                weights: m.weights.clone(),
                indices: m.indices.clone(),
            })
            .collect(),
        materials: s
            .materials
            .iter()
            .map(|m| xy_gfx::gltf::GltfMaterial {
                name: m.name.clone(),
                base_color: m.diffuse,
                emissive: [m.emission[0], m.emission[1], m.emission[2]],
                texture_index: m
                    .texture_name
                    .as_ref()
                    .and_then(|n| s.textures.iter().position(|t| &t.name == n)),
                alpha_blend: m
                    .texture_name
                    .as_ref()
                    .and_then(|n| s.textures.iter().find(|t| &t.name == n))
                    .is_some_and(|t| t.format.has_alpha()),
            })
            .collect(),
        bones: s
            .bones
            .iter()
            .map(|b| xy_gfx::gltf::GltfBone {
                name: b.name.clone(),
                parent: b.parent,
                translation: b.translation,
                rotation_euler: b.rotation_euler,
                scale: b.scale,
            })
            .collect(),
        textures: s
            .textures
            .iter()
            .map(|t| xy_gfx::gltf::GltfTexture {
                name: t.name.clone(),
                width: t.width,
                height: t.height,
                rgba: t.rgba.clone(),
            })
            .collect(),
        animations: s
            .animations
            .iter()
            .map(|a| xy_gfx::gltf::GltfAnimation {
                name: a.name.clone(),
                channels: a
                    .channels
                    .iter()
                    .map(|ch| xy_gfx::gltf::GltfAnimChannel {
                        bone_name: ch.bone.clone(),
                        path: ch.path,
                        times: ch.times.clone(),
                        values: ch.values.clone(),
                        step: ch.step,
                    })
                    .collect(),
            })
            .collect(),
    }
}

/// Write a set of outputs under `dir`, creating it. Returns the paths.
pub fn write_outputs(
    outputs: &[OutputFile],
    dir: &Path,
) -> std::io::Result<Vec<std::path::PathBuf>> {
    std::fs::create_dir_all(dir)?;
    let mut written = Vec::new();
    for o in outputs {
        let path = dir.join(&o.name);
        let mut f = std::io::BufWriter::new(std::fs::File::create(&path)?);
        f.write_all(&o.bytes)?;
        written.push(path);
    }
    Ok(written)
}

fn safe_stem(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if s.is_empty() {
        "model".into()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wav_header_is_canonical() {
        let wav = wav_bytes(32728, 2, &[0i16; 100]);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(wav.len(), 44 + 200);
        let data_len = u32::from_le_bytes(wav[40..44].try_into().unwrap());
        assert_eq!(data_len, 200);
    }

    #[test]
    fn safe_stems() {
        assert_eq!(safe_stem("pm0001_00"), "pm0001_00");
        assert_eq!(safe_stem(""), "model");
        assert_eq!(safe_stem("a b/c"), "a_b_c");
    }
}
