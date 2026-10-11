//! UI-neutral decoded-model intermediate.
//!
//! Both BCH (SPICA port) and CGFX (Ohana3DS port) decode into
//! [`SceneModel`], which the 3D viewer rasterizes, the GLB exporter
//! serializes, and the metadata panels display — so no viewer code touches
//! the raw format structs.

use xy_core::error::Result;
use xy_gfx::bch;
use xy_gfx::cgfx;
use xy_gfx::pkmn;
use xy_gfx::texture::TextureFormat;

/// A decoded RGBA texture (already usable as an image).
#[derive(Debug, Clone)]
pub struct DecodedTexture {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub format: TextureFormat,
    pub rgba: Vec<u8>,
}

/// One drawable mesh: interleaved vertex attributes + a triangle index list.
#[derive(Debug, Clone, Default)]
pub struct RenderMesh {
    pub name: String,
    pub material_index: usize,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    /// UV set 0 (as stored; viewers flip as their convention requires).
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    /// Skinning (packed 4 influences per vertex; joint ids index into
    /// [`SceneModel::bones`]).
    pub joints: Vec<[u16; 4]>,
    pub weights: Vec<[f32; 4]>,
    /// Triangulated indices into the attribute arrays above.
    pub indices: Vec<u32>,
}

/// PBR-lite material description for viewers/exporters.
#[derive(Debug, Clone)]
pub struct SceneMaterial {
    pub name: String,
    pub diffuse: [f32; 4],
    pub emission: [f32; 4],
    /// Texture-unit-0 binding (name into [`SceneModel::textures`]).
    pub texture_name: Option<String>,
}

/// Skeleton bone with a parent index (`-1` = root).
#[derive(Debug, Clone)]
pub struct SceneBone {
    pub name: String,
    pub parent: i32,
    pub translation: [f32; 3],
    pub rotation_euler: [f32; 3],
    pub scale: [f32; 3],
}

/// One animation channel targeting a bone node's TRS.
#[derive(Debug, Clone)]
pub struct SceneAnimChannel {
    pub bone: String,
    /// `"translation"` | `"rotation"`.
    pub path: &'static str,
    /// Key times in seconds.
    pub times: Vec<f32>,
    /// vec3 per key (translation) or quaternion xyzw per key (rotation).
    pub values: Vec<f32>,
    pub step: bool,
}

/// A decoded animation (CGFX CANM skeletal curves; XY's real motions are
/// the still-undecoded v2 GFMotion bitstreams, so BCH models carry none).
#[derive(Debug, Clone)]
pub struct SceneAnimation {
    pub name: String,
    pub channels: Vec<SceneAnimChannel>,
}

/// A whole decoded model, ready for rendering/export/inspection.
#[derive(Debug, Clone, Default)]
pub struct SceneModel {
    pub name: String,
    pub meshes: Vec<RenderMesh>,
    pub materials: Vec<SceneMaterial>,
    pub bones: Vec<SceneBone>,
    pub textures: Vec<DecodedTexture>,
    pub animations: Vec<SceneAnimation>,
    /// `"BCH"` or `"CGFX"`.
    pub source_format: &'static str,
    /// Non-fatal notes for the UI (partial geometry, skipped variants, …).
    pub warnings: Vec<String>,
}

impl SceneModel {
    pub fn triangle_count(&self) -> usize {
        self.meshes.iter().map(|m| m.indices.len() / 3).sum()
    }
    pub fn vertex_count(&self) -> usize {
        self.meshes.iter().map(|m| m.positions.len()).sum()
    }
    /// Axis-aligned bounding box (min, max); `None` when no vertices.
    pub fn bounds(&self) -> Option<([f32; 3], [f32; 3])> {
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        let mut any = false;
        for m in &self.meshes {
            for p in &m.positions {
                any = true;
                for a in 0..3 {
                    min[a] = min[a].min(p[a]);
                    max[a] = max[a].max(p[a]);
                }
            }
        }
        any.then_some((min, max))
    }
}

/// Load every model from a file image: raw BCH, raw CGFX, or a GameFreak
/// container (`PC`/`PF`/`PT`/…) whose entries are unwrapped recursively.
/// Textures are attached to the models of the same source when present.
pub fn load_models(data: &[u8]) -> Result<Vec<SceneModel>> {
    let mut out = Vec::new();
    let mut warnings = Vec::new();
    collect_models(data, &mut out, &mut warnings, 0)?;
    if out.is_empty() {
        for w in warnings {
            for m in &mut out {
                m.warnings.push(w.clone());
            }
        }
    } else {
        for w in warnings {
            out[0].warnings.push(w);
        }
    }
    Ok(out)
}

fn collect_models(
    data: &[u8],
    out: &mut Vec<SceneModel>,
    warnings: &mut Vec<String>,
    depth: usize,
) -> Result<()> {
    if depth > 4 {
        return Ok(());
    }
    if data.len() >= 4 && &data[0..4] == b"BCH\x00" {
        match bch::Bch::parse(data) {
            Ok(b) => out.extend(b.models.iter().map(model_from_bch)),
            Err(e) => warnings.push(format!("BCH parse failed: {e}")),
        }
        return Ok(());
    }
    if data.len() >= 4 && &data[0..4] == b"CGFX" {
        match cgfx::Cgfx::parse(data) {
            Ok(c) => {
                for m in &c.models {
                    let mut sm = model_from_cgfx(m);
                    sm.animations = animations_from_cgfx(&c, &m.name);
                    if m.geometry_partial {
                        sm.warnings.push(
                            "rev-9 CNOD-linked geometry is an open RE project; \
                             materials/textures/anims decode but meshes were skipped"
                                .to_string(),
                        );
                    }
                    out.push(sm);
                }
                // Standalone texture-only CGFX: synthesize an empty model so
                // the texture viewer still has an anchor.
                if out
                    .iter()
                    .all(|m| !m.meshes.is_empty() || m.name != "<textures>")
                    && !c.textures.is_empty()
                    && c.models.is_empty()
                {
                    let mut sm = SceneModel {
                        name: "<textures>".into(),
                        source_format: "CGFX",
                        ..Default::default()
                    };
                    sm.textures = collect_decoded_textures(data);
                    out.push(sm);
                }
            }
            Err(e) => warnings.push(format!("CGFX parse failed: {e}")),
        }
        return Ok(());
    }
    if let Ok(container) = pkmn::PkmnContainer::parse(data) {
        for entry in &container.entries {
            let _ = collect_models(entry, out, warnings, depth + 1);
        }
        return Ok(());
    }
    Ok(())
}

fn model_from_bch(m: &bch::Model) -> SceneModel {
    let mut meshes = Vec::new();
    for (mi, mesh) in m.meshes.iter().enumerate() {
        let mut rm = RenderMesh {
            name: m
                .node_names
                .get(mesh.node_index)
                .cloned()
                .unwrap_or_else(|| format!("mesh_{mi}")),
            material_index: mesh.material_index,
            ..Default::default()
        };
        for v in &mesh.vertices {
            rm.positions.push(v.position);
            rm.normals.push(v.normal);
            rm.uvs.push(v.uv[0]);
            rm.colors.push(v.color);
            rm.joints.push([
                v.bone_indices[0].max(0) as u16,
                v.bone_indices[1].max(0) as u16,
                v.bone_indices[2].max(0) as u16,
                v.bone_indices[3].max(0) as u16,
            ]);
            rm.weights.push(v.bone_weights);
        }
        for sub in &mesh.submeshes {
            rm.indices
                .extend(triangles(sub.primitive_mode, &sub.indices));
        }
        meshes.push(rm);
    }
    let materials = m
        .materials
        .iter()
        .map(|mat| SceneMaterial {
            name: mat.name.clone(),
            diffuse: color_f32(mat.diffuse),
            emission: color_f32(mat.emission),
            texture_name: mat.texture_names[0].clone(),
        })
        .collect();
    let bones = m
        .bones
        .iter()
        .map(|b| SceneBone {
            name: b.name.clone(),
            parent: b.parent_index as i32,
            translation: b.translation,
            rotation_euler: b.rotation,
            scale: b.scale,
        })
        .collect();
    SceneModel {
        name: m.name.clone(),
        meshes,
        materials,
        bones,
        animations: Vec::new(),
        source_format: "BCH",
        ..Default::default()
    }
}

/// Union key times across axis groups (seconds, 30 fps H3D native).
fn union_times(groups: &[&cgfx::KeyFrameGroup]) -> Vec<f32> {
    let mut t: Vec<f32> = groups
        .iter()
        .flat_map(|g| g.key_frames.iter().map(|k| k.frame / 30.0))
        .collect();
    t.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    t.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
    t
}

/// Evaluate one axis group at time t (step / linear; hermite → linear).
fn eval_group(g: &cgfx::KeyFrameGroup, t: f32) -> f32 {
    let ks = &g.key_frames;
    if ks.is_empty() {
        return 0.0;
    }
    let tf = t * 30.0;
    if tf <= ks[0].frame || g.interpolation == 0 || ks.len() == 1 {
        return ks[0].value;
    }
    if tf >= ks[ks.len() - 1].frame {
        return ks[ks.len() - 1].value;
    }
    for w in ks.windows(2) {
        if tf >= w[0].frame && tf <= w[1].frame {
            let span = w[1].frame - w[0].frame;
            let a = if span.abs() < 1e-6 {
                0.0
            } else {
                (tf - w[0].frame) / span
            };
            return w[0].value + (w[1].value - w[0].value) * a;
        }
    }
    ks[ks.len() - 1].value
}

/// Quaternion (x, y, z, w) from XYZ euler — Hamilton `qx ⊗ qy ⊗ qz`,
/// matching the column-vector `Rx·Ry·Rz` convention.
fn quat_from_euler_xyz(r: &[f32; 3]) -> [f32; 4] {
    let (sx, cx) = (r[0] * 0.5).sin_cos();
    let (sy, cy) = (r[1] * 0.5).sin_cos();
    let (sz, cz) = (r[2] * 0.5).sin_cos();
    let qm = |a: [f32; 4], b: [f32; 4]| {
        [
            a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
            a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
            a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
            a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
        ]
    };
    let q = qm(
        qm([sx, 0.0, 0.0, cx], [0.0, sy, 0.0, cy]),
        [0.0, 0.0, sz, cz],
    );
    let l = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if l > 1e-12 {
        [q[0] / l, q[1] / l, q[2] / l, q[3] / l]
    } else {
        [0.0, 0.0, 0.0, 1.0]
    }
}

fn model_from_cgfx(m: &cgfx::Model) -> SceneModel {
    let mut meshes = Vec::new();
    for (mi, mesh) in m.meshes.iter().enumerate() {
        let Some(shape) = m.shapes.get(mesh.shape_index) else {
            continue;
        };
        let mut rm = RenderMesh {
            name: if mesh.name.is_empty() {
                format!("mesh_{mi}")
            } else {
                mesh.name.clone()
            },
            material_index: mesh.material_id,
            ..Default::default()
        };
        for v in &shape.vertices {
            rm.positions.push([
                v.position[0] + shape.position_offset[0],
                v.position[1] + shape.position_offset[1],
                v.position[2] + shape.position_offset[2],
            ]);
            rm.normals.push(v.normal);
            rm.uvs.push(v.uv[0]);
            rm.colors.push(color_f32(v.color));
            let mut j = [0u16; 4];
            for (i, n) in v.nodes.iter().take(4).enumerate() {
                j[i] = *n as u16;
            }
            rm.joints.push(j);
            let mut w = [0f32; 4];
            for (i, x) in v.weights.iter().take(4).enumerate() {
                w[i] = *x;
            }
            rm.weights.push(w);
        }
        // CGFX shapes store triangle soup in draw order (mirrors the OBJ
        // export for this format).
        let n = rm.positions.len();
        rm.indices.extend((0..n).map(|i| i as u32));
        meshes.push(rm);
    }
    let materials = m
        .materials
        .iter()
        .map(|mat| SceneMaterial {
            name: mat.name.clone(),
            diffuse: color_f32(mat.diffuse),
            emission: color_f32(mat.emission),
            texture_name: mat.texture_names[0].clone(),
        })
        .collect();
    let bones = m
        .bones
        .iter()
        .map(|b| SceneBone {
            name: b.name.clone(),
            parent: b.parent_id,
            translation: b.translation,
            rotation_euler: b.rotation,
            scale: b.scale,
        })
        .collect();
    SceneModel {
        name: m.name.clone(),
        meshes,
        materials,
        bones,
        animations: Vec::new(), // set by load_models (needs the container)
        source_format: "CGFX",
        ..Default::default()
    }
}

/// Build channel-form animations from a CGFX container's `CANM` curves
/// (euler rotation + translation per bone; scale is not present in CANM).
fn animations_from_cgfx(c: &cgfx::Cgfx, model_name: &str) -> Vec<SceneAnimation> {
    let mut out = Vec::new();
    for anim in &c.skeletal_animations {
        if !anim.target.is_empty() && anim.target != model_name {
            continue;
        }
        let mut channels = Vec::new();
        for bone in &anim.bones {
            let t_groups = [
                &bone.translation[0],
                &bone.translation[1],
                &bone.translation[2],
            ];
            if !t_groups.iter().all(|g| g.key_frames.is_empty()) {
                let times = union_times(&t_groups);
                let mut values = Vec::with_capacity(times.len() * 3);
                for t in &times {
                    for g in t_groups {
                        values.push(eval_group(g, *t));
                    }
                }
                channels.push(SceneAnimChannel {
                    bone: bone.name.clone(),
                    path: "translation",
                    times,
                    values,
                    step: t_groups.iter().all(|g| g.interpolation == 0),
                });
            }
            let r_groups = [&bone.rotation[0], &bone.rotation[1], &bone.rotation[2]];
            if !r_groups.iter().all(|g| g.key_frames.is_empty()) {
                let times = union_times(&r_groups);
                let mut values = Vec::with_capacity(times.len() * 4);
                for t in &times {
                    let e = [
                        eval_group(r_groups[0], *t),
                        eval_group(r_groups[1], *t),
                        eval_group(r_groups[2], *t),
                    ];
                    values.extend_from_slice(&quat_from_euler_xyz(&e));
                }
                channels.push(SceneAnimChannel {
                    bone: bone.name.clone(),
                    path: "rotation",
                    times,
                    values,
                    step: r_groups.iter().all(|g| g.interpolation == 0),
                });
            }
        }
        if !channels.is_empty() {
            out.push(SceneAnimation {
                name: anim.name.clone(),
                channels,
            });
        }
    }
    out
}

/// Attach BCH/CGFX textures (decoded to RGBA) to a scene model after load.
pub fn attach_textures(model: &mut SceneModel, data: &[u8]) {
    model.textures = collect_decoded_textures(data);
}

/// Collect every decodable texture from a payload (raw BCH / raw CGFX /
/// GF container of either), mirroring `xy gfx texture`.
pub fn collect_decoded_textures(data: &[u8]) -> Vec<DecodedTexture> {
    let mut out = Vec::new();
    collect_textures_inner(data, &mut out, 0);
    out
}

fn collect_textures_inner(data: &[u8], out: &mut Vec<DecodedTexture>, depth: usize) {
    if depth > 4 || data.len() < 4 {
        return;
    }
    if &data[0..4] == b"BCH\x00" {
        if let Ok(b) = bch::Bch::parse(data) {
            for t in &b.textures {
                push_texture(out, &t.name, t.width, t.height, t.format, || {
                    t.decode_rgba()
                });
            }
        }
    } else if &data[0..4] == b"CGFX" {
        if let Ok(c) = cgfx::Cgfx::parse(data) {
            for t in &c.textures {
                push_texture(out, &t.name, t.width, t.height, t.format, || {
                    t.decode_rgba()
                });
            }
        }
    } else if let Ok(container) = pkmn::PkmnContainer::parse(data) {
        for entry in &container.entries {
            collect_textures_inner(entry, out, depth + 1);
        }
    }
}

fn push_texture(
    out: &mut Vec<DecodedTexture>,
    name: &str,
    width: usize,
    height: usize,
    format: TextureFormat,
    decode: impl FnOnce() -> Result<Vec<u8>>,
) {
    if width == 0 || height == 0 {
        return;
    }
    if let Ok(rgba) = decode() {
        if rgba.len() == width * height * 4 {
            out.push(DecodedTexture {
                name: if name.is_empty() {
                    "texture".into()
                } else {
                    name.into()
                },
                width,
                height,
                format,
                rgba,
            });
        }
    }
}

fn color_f32(c: [u8; 4]) -> [f32; 4] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
        c[3] as f32 / 255.0,
    ]
}

/// Expand a PICA primitive run into a triangle list (shared with the OBJ
/// exporter semantics).
fn triangles(mode: u8, indices: &[u16]) -> Vec<u32> {
    let mut out = Vec::new();
    match mode {
        1 => {
            for (i, w) in indices.windows(3).enumerate() {
                if i & 1 == 0 {
                    out.extend_from_slice(&[w[0] as u32, w[1] as u32, w[2] as u32]);
                } else {
                    out.extend_from_slice(&[w[1] as u32, w[0] as u32, w[2] as u32]);
                }
            }
        }
        2 => {
            if let Some((&first, rest)) = indices.split_first() {
                for w in rest.windows(2) {
                    out.extend_from_slice(&[first as u32, w[0] as u32, w[1] as u32]);
                }
            }
        }
        _ => out.extend(indices.iter().map(|&i| i as u32)),
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_expansion() {
        assert_eq!(triangles(0, &[0, 1, 2]), vec![0, 1, 2]);
        // strip of 4 → 2 triangles with alternating winding
        assert_eq!(triangles(1, &[0, 1, 2, 3]), vec![0, 1, 2, 2, 1, 3]);
        // fan
        assert_eq!(triangles(2, &[0, 1, 2, 3]), vec![0, 1, 2, 0, 2, 3]);
    }
}
