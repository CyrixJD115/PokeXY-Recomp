//! glTF 2.0 (GLB) export — bundles geometry, skeleton, materials and
//! embedded PNG textures into one Blender-ready file.
//!
//! Input is the neutral [`GltfScene`] type (plain attribute arrays), so
//! callers map their format structs onto it without this module knowing
//! about BCH/CGFX internals. [`from_bch`] / [`from_cgfx`] do that mapping
//! for the two decoded model formats.
//!
//! Layout choices:
//! - One node per source mesh (names preserved, mirrors the OBJ exporter's
//!   `o` objects), plus one node per bone under the scene root; skins
//!   reference those bone nodes and every mesh node carries the skin.
//! - UVs are written as `(u, 1 - v)` — the same flip as the OBJ export, so
//!   the Blender-validated OBJ+PNG pairing carries over to GLB.
//! - Bone locals are emitted as full `matrix` fields composed as
//!   `T · Rx · Ry · Rz · S` (column vectors). XY's real skeletal motions
//!   are the undecoded v2 GFMotion streams, so the skeleton is
//!   informational until those land.
//! - Materials: `pbrMetallicRoughness` with the diffuse color (sRGB values
//!   passed through unchanged, matching the MTL export), metallic 0,
//!   roughness 1, emissive from the emission color, `doubleSided` (3DS
//!   meshes have no consistent winding guarantee), `BLEND` alpha when the
//!   bound texture format carries alpha.

use std::fmt::Write as _;

use crate::bch;
use crate::cgfx;
use crate::error::Result;
use crate::texture::write_png;

/// A PNG-encodable texture for embedding.
#[derive(Debug, Clone)]
pub struct GltfTexture {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

/// PBR-lite material.
#[derive(Debug, Clone)]
pub struct GltfMaterial {
    pub name: String,
    /// sRGB u8 colors passed through as factors (documented simplification
    /// — matches the OBJ/MTL export).
    pub base_color: [f32; 4],
    pub emissive: [f32; 3],
    /// Index into [`GltfScene::textures`].
    pub texture_index: Option<usize>,
    pub alpha_blend: bool,
}

/// One mesh: parallel attribute arrays + triangle indices.
#[derive(Debug, Clone, Default)]
pub struct GltfMesh {
    pub name: String,
    pub material_index: Option<usize>,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    pub joints: Vec<[u16; 4]>,
    pub weights: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}

/// A skeleton bone (`parent` = -1 for roots).
#[derive(Debug, Clone)]
pub struct GltfBone {
    pub name: String,
    pub parent: i32,
    pub translation: [f32; 3],
    pub rotation_euler: [f32; 3],
    pub scale: [f32; 3],
}

/// Everything needed to serialize one model to GLB.
#[derive(Debug, Clone, Default)]
pub struct GltfScene {
    pub name: String,
    pub meshes: Vec<GltfMesh>,
    pub materials: Vec<GltfMaterial>,
    pub bones: Vec<GltfBone>,
    pub textures: Vec<GltfTexture>,
    pub animations: Vec<GltfAnimation>,
}

/// One animation channel targeting a bone node's TRS.
#[derive(Debug, Clone)]
pub struct GltfAnimChannel {
    /// Target bone name (resolved to a node index at export).
    pub bone_name: String,
    /// `"translation"` | `"rotation"`.
    pub path: &'static str,
    /// Key times in seconds.
    pub times: Vec<f32>,
    /// vec3 per key (translation) or quat xyzw per key (rotation).
    pub values: Vec<f32>,
    /// `true` → STEP interpolation, else LINEAR.
    pub step: bool,
}

#[derive(Debug, Clone)]
pub struct GltfAnimation {
    pub name: String,
    pub channels: Vec<GltfAnimChannel>,
}

/// Map a decoded BCH model (+ its container's textures) to the neutral
/// scene type.
pub fn from_bch(b: &bch::Bch, model_index: usize) -> Option<GltfScene> {
    let m = b.models.get(model_index)?;
    let mut meshes = Vec::new();
    for (mi, mesh) in m.meshes.iter().enumerate() {
        let mut gm = GltfMesh {
            name: m
                .node_names
                .get(mesh.node_index)
                .cloned()
                .unwrap_or_else(|| format!("mesh_{mi}")),
            material_index: Some(mesh.material_index),
            ..Default::default()
        };
        for v in &mesh.vertices {
            gm.positions.push(v.position);
            gm.normals.push(v.normal);
            gm.uvs.push([v.uv[0][0], 1.0 - v.uv[0][1]]);
            gm.colors.push(v.color);
            gm.joints.push([
                v.bone_indices[0].clamp(0, i32::from(u16::MAX)) as u16,
                v.bone_indices[1].clamp(0, i32::from(u16::MAX)) as u16,
                v.bone_indices[2].clamp(0, i32::from(u16::MAX)) as u16,
                v.bone_indices[3].clamp(0, i32::from(u16::MAX)) as u16,
            ]);
            gm.weights.push(v.bone_weights);
        }
        for sub in &mesh.submeshes {
            gm.indices
                .extend(triangles(sub.primitive_mode, &sub.indices));
        }
        meshes.push(gm);
    }
    let materials = m
        .materials
        .iter()
        .map(|mat| GltfMaterial {
            name: mat.name.clone(),
            base_color: rgba_f32(mat.diffuse),
            emissive: rgb_f32(mat.emission),
            texture_index: mat.texture_names[0]
                .as_ref()
                .and_then(|n| b.textures.iter().position(|t| &t.name == n)),
            alpha_blend: mat.texture_names[0]
                .as_ref()
                .and_then(|n| b.textures.iter().find(|t| &t.name == n))
                .is_some_and(|t| t.format.has_alpha()),
        })
        .collect();
    let bones = m
        .bones
        .iter()
        .map(|b| GltfBone {
            name: b.name.clone(),
            parent: b.parent_index as i32,
            translation: b.translation,
            rotation_euler: b.rotation,
            scale: b.scale,
        })
        .collect();
    let textures = b
        .textures
        .iter()
        .filter_map(|t| {
            t.decode_rgba().ok().map(|rgba| GltfTexture {
                name: t.name.clone(),
                width: t.width,
                height: t.height,
                rgba,
            })
        })
        .collect();
    Some(GltfScene {
        name: m.name.clone(),
        meshes,
        materials,
        bones,
        textures,
        animations: Vec::new(),
    })
}

/// Map a decoded CGFX model (+ textures) to the neutral scene type.
pub fn from_cgfx(c: &cgfx::Cgfx, model_index: usize) -> Option<GltfScene> {
    let m = c.models.get(model_index)?;
    let mut meshes = Vec::new();
    for (mi, mesh) in m.meshes.iter().enumerate() {
        let Some(shape) = m.shapes.get(mesh.shape_index) else {
            continue;
        };
        let mut gm = GltfMesh {
            name: if mesh.name.is_empty() {
                format!("mesh_{mi}")
            } else {
                mesh.name.clone()
            },
            material_index: Some(mesh.material_id),
            ..Default::default()
        };
        for v in &shape.vertices {
            gm.positions.push([
                v.position[0] + shape.position_offset[0],
                v.position[1] + shape.position_offset[1],
                v.position[2] + shape.position_offset[2],
            ]);
            gm.normals.push(v.normal);
            gm.uvs.push([v.uv[0][0], 1.0 - v.uv[0][1]]);
            gm.colors.push(rgba_f32(v.color));
            let mut j = [0u16; 4];
            for (i, n) in v.nodes.iter().take(4).enumerate() {
                j[i] = *n as u16;
            }
            gm.joints.push(j);
            let mut w = [0f32; 4];
            for (i, x) in v.weights.iter().take(4).enumerate() {
                w[i] = *x;
            }
            gm.weights.push(w);
        }
        let n = gm.positions.len() as u32;
        gm.indices.extend(0..n);
        meshes.push(gm);
    }
    let materials = m
        .materials
        .iter()
        .map(|mat| GltfMaterial {
            name: mat.name.clone(),
            base_color: rgba_f32(mat.diffuse),
            emissive: rgb_f32(mat.emission),
            texture_index: mat.texture_names[0]
                .as_ref()
                .and_then(|n| c.textures.iter().position(|t| &t.name == n)),
            alpha_blend: mat.texture_names[0]
                .as_ref()
                .and_then(|n| c.textures.iter().find(|t| &t.name == n))
                .is_some_and(|t| t.format.has_alpha()),
        })
        .collect();
    let bones = m
        .bones
        .iter()
        .map(|b| GltfBone {
            name: b.name.clone(),
            parent: b.parent_id,
            translation: b.translation,
            rotation_euler: b.rotation,
            scale: b.scale,
        })
        .collect();
    let textures = c
        .textures
        .iter()
        .filter_map(|t| {
            t.decode_rgba().ok().map(|rgba| GltfTexture {
                name: t.name.clone(),
                width: t.width,
                height: t.height,
                rgba,
            })
        })
        .collect();
    let animations = animations_from_cgfx(c, model_index);
    Some(GltfScene {
        name: m.name.clone(),
        meshes,
        materials,
        bones,
        textures,
        animations,
    })
}

/// Build glTF animation channels from CGFX `CANM` skeletal animations
/// (euler rotation + translation curves; scale not present in CANM).
/// Times are frames ÷ 30 (the H3D native frame rate).
fn animations_from_cgfx(c: &cgfx::Cgfx, model_index: usize) -> Vec<GltfAnimation> {
    let Some(m) = c.models.get(model_index) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for anim in &c.skeletal_animations {
        // CANM anims target a model by name; adopt when it matches (or is empty).
        if !anim.target.is_empty() && anim.target != m.name {
            continue;
        }
        let mut channels = Vec::new();
        for bone in &anim.bones {
            let t_groups = [
                &bone.translation[0],
                &bone.translation[1],
                &bone.translation[2],
            ];
            if let Some(ch) = trs_channel(&bone.name, "translation", &t_groups) {
                channels.push(ch);
            }
            let r_groups = [&bone.rotation[0], &bone.rotation[1], &bone.rotation[2]];
            if let Some(ch) = rotation_channel(&bone.name, &r_groups) {
                channels.push(ch);
            }
        }
        if !channels.is_empty() {
            out.push(GltfAnimation {
                name: anim.name.clone(),
                channels,
            });
        }
    }
    out
}

/// Union key times across axis groups (seconds).
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

fn trs_channel(
    bone: &str,
    path: &'static str,
    groups: &[&cgfx::KeyFrameGroup; 3],
) -> Option<GltfAnimChannel> {
    if groups.iter().all(|g| g.key_frames.is_empty()) {
        return None;
    }
    let times = union_times(groups);
    let mut values = Vec::with_capacity(times.len() * 3);
    for t in &times {
        for g in groups {
            values.push(eval_group(g, *t));
        }
    }
    let step = groups.iter().all(|g| g.interpolation == 0);
    Some(GltfAnimChannel {
        bone_name: bone.to_string(),
        path,
        times,
        values,
        step,
    })
}

fn rotation_channel(bone: &str, groups: &[&cgfx::KeyFrameGroup; 3]) -> Option<GltfAnimChannel> {
    if groups.iter().all(|g| g.key_frames.is_empty()) {
        return None;
    }
    let times = union_times(groups);
    let mut values = Vec::with_capacity(times.len() * 4);
    for t in &times {
        let e = [
            eval_group(groups[0], *t),
            eval_group(groups[1], *t),
            eval_group(groups[2], *t),
        ];
        let q = quat_from_euler_xyz(&e);
        values.extend_from_slice(&q);
    }
    let step = groups.iter().all(|g| g.interpolation == 0);
    Some(GltfAnimChannel {
        bone_name: bone.to_string(),
        path: "rotation",
        times,
        values,
        step,
    })
}

fn rgba_f32(c: [u8; 4]) -> [f32; 4] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
        c[3] as f32 / 255.0,
    ]
}

fn rgb_f32(c: [u8; 4]) -> [f32; 3] {
    [
        c[0] as f32 / 255.0,
        c[1] as f32 / 255.0,
        c[2] as f32 / 255.0,
    ]
}

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

// ---------------------------------------------------------------------------
// GLB writer
// ---------------------------------------------------------------------------

/// Growing BIN chunk + parallel bufferView/accessor JSON arrays.
#[derive(Default)]
struct Writer {
    bin: Vec<u8>,
    views_json: String,
    accessors_json: String,
}

impl Writer {
    fn view(&mut self, bytes: &[u8], align: usize) -> usize {
        while !self.bin.len().is_multiple_of(align) {
            self.bin.push(0);
        }
        let off = self.bin.len();
        self.bin.extend_from_slice(bytes);
        if !self.views_json.is_empty() {
            self.views_json.push(',');
        }
        let _ = write!(
            self.views_json,
            "{{\"buffer\":0,\"byteOffset\":{off},\"byteLength\":{}}}",
            bytes.len()
        );
        // index = number of views so far minus this one
        self.views_json.matches("\"buffer\":0").count() - 1
    }

    fn accessor(
        &mut self,
        view: usize,
        component: u32,
        ty: &str,
        count: usize,
        minmax: Option<(Vec<f32>, Vec<f32>)>,
    ) -> usize {
        if !self.accessors_json.is_empty() {
            self.accessors_json.push(',');
        }
        let fmt_vec =
            |v: &[f32]| -> String { v.iter().map(|f| fmt_f32(*f)).collect::<Vec<_>>().join(",") };
        let mm = match minmax {
            Some((mn, mx)) => {
                format!(",\"min\":[{}],\"max\":[{}]", fmt_vec(&mn), fmt_vec(&mx))
            }
            None => String::new(),
        };
        let _ = write!(
            self.accessors_json,
            "{{\"bufferView\":{view},\"componentType\":{component},\"count\":{count},\"type\":\"{ty}\"{mm}}}"
        );
        self.accessors_json.matches("\"bufferView\"").count() - 1
    }
}

/// Serialize the scene to a binary glTF 2.0 file (GLB).
pub fn export_glb(scene: &GltfScene) -> Result<Vec<u8>> {
    let mut w = Writer::default();
    let skin_needed = !scene.bones.is_empty();

    // ---- meshes & primitives ----
    let mut meshes_json = String::new();
    let mut mesh_count = 0usize;
    for mesh in &scene.meshes {
        if mesh.positions.is_empty() || mesh.indices.is_empty() {
            continue;
        }
        let mut attrs: Vec<String> = Vec::new();

        // POSITION (min/max required by the spec)
        let mut pos_bytes = Vec::with_capacity(mesh.positions.len() * 12);
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];
        for p in &mesh.positions {
            for a in 0..3 {
                min[a] = min[a].min(p[a]);
                max[a] = max[a].max(p[a]);
            }
            for x in p {
                pos_bytes.extend_from_slice(&x.to_le_bytes());
            }
        }
        let v = w.view(&pos_bytes, 4);
        let a = w.accessor(
            v,
            5126,
            "VEC3",
            mesh.positions.len(),
            Some((min.to_vec(), max.to_vec())),
        );
        attrs.push(format!("\"POSITION\":{a}"));

        if mesh.normals.len() == mesh.positions.len() && !mesh.normals.is_empty() {
            let mut b = Vec::with_capacity(mesh.normals.len() * 12);
            for n in &mesh.normals {
                for x in n {
                    b.extend_from_slice(&x.to_le_bytes());
                }
            }
            let v = w.view(&b, 4);
            let a = w.accessor(v, 5126, "VEC3", mesh.normals.len(), None);
            attrs.push(format!("\"NORMAL\":{a}"));
        }
        if mesh.uvs.len() == mesh.positions.len() && !mesh.uvs.is_empty() {
            let mut b = Vec::with_capacity(mesh.uvs.len() * 8);
            for u in &mesh.uvs {
                b.extend_from_slice(&u[0].to_le_bytes());
                b.extend_from_slice(&u[1].to_le_bytes());
            }
            let v = w.view(&b, 4);
            let a = w.accessor(v, 5126, "VEC2", mesh.uvs.len(), None);
            attrs.push(format!("\"TEXCOORD_0\":{a}"));
        }
        let colored = mesh.colors.iter().any(|c| c != &[1.0, 1.0, 1.0, 1.0]);
        if colored && mesh.colors.len() == mesh.positions.len() {
            let mut b = Vec::with_capacity(mesh.colors.len() * 16);
            for c in &mesh.colors {
                for x in c {
                    b.extend_from_slice(&x.to_le_bytes());
                }
            }
            let v = w.view(&b, 4);
            let a = w.accessor(v, 5126, "VEC4", mesh.colors.len(), None);
            attrs.push(format!("\"COLOR_0\":{a}"));
        }
        let skinned = mesh.weights.iter().any(|x| x[0] + x[1] + x[2] + x[3] > 0.0);
        if skinned && skin_needed && mesh.joints.len() == mesh.positions.len() {
            let nb = scene.bones.len().clamp(1, u16::MAX as usize) as u16;
            let mut b = Vec::with_capacity(mesh.joints.len() * 8);
            for j in &mesh.joints {
                for &x in j {
                    b.extend_from_slice(&x.min(nb - 1).to_le_bytes());
                }
            }
            let v = w.view(&b, 2);
            let a = w.accessor(v, 5123, "VEC4", mesh.joints.len(), None);
            attrs.push(format!("\"JOINTS_0\":{a}"));
            let mut b = Vec::with_capacity(mesh.weights.len() * 16);
            for x in &mesh.weights {
                for f in x {
                    b.extend_from_slice(&f.to_le_bytes());
                }
            }
            let v = w.view(&b, 4);
            let a = w.accessor(v, 5126, "VEC4", mesh.weights.len(), None);
            attrs.push(format!("\"WEIGHTS_0\":{a}"));
        }

        let mut ib = Vec::with_capacity(mesh.indices.len() * 4);
        for i in &mesh.indices {
            ib.extend_from_slice(&i.to_le_bytes());
        }
        let v = w.view(&ib, 4);
        let idx = w.accessor(v, 5125, "SCALAR", mesh.indices.len(), None);

        let material = mesh
            .material_index
            .filter(|&m| scene.materials.get(m).is_some())
            .map(|m| format!(",\"material\":{m}"))
            .unwrap_or_default();
        if !meshes_json.is_empty() {
            meshes_json.push(',');
        }
        let _ = write!(
            meshes_json,
            "{{\"name\":\"{}\",\"primitives\":[{{\"attributes\":{{{}}},\"indices\":{idx}{}}}]}}",
            json_str(&mesh.name),
            attrs.join(","),
            material
        );
        mesh_count += 1;
    }

    // ---- skin (inverse bind matrices) ----
    let mut skins_json = String::new();
    let mut ibm_accessor = 0usize;
    if skin_needed {
        let globals = bone_globals(&scene.bones);
        let mut b = Vec::with_capacity(scene.bones.len() * 64);
        for g in &globals {
            for x in invert4(g) {
                b.extend_from_slice(&x.to_le_bytes());
            }
        }
        let v = w.view(&b, 4);
        ibm_accessor = w.accessor(v, 5126, "MAT4", scene.bones.len(), None);
        skins_json = format!(
            ",\"skins\":[{{\"inverseBindMatrices\":{ibm_accessor},\"joints\":[{}]}}]",
            bone_node_ids(scene)
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
    }

    // ---- animations ----
    // Node ids: 0 = root, 1..=mesh_count = mesh nodes, bones after.
    let bone_base = 1 + mesh_count;
    let mut animations_json = String::new();
    if !skin_needed {
        // Without bones there are no animatable targets.
    } else {
        let mut anims: Vec<String> = Vec::new();
        for anim in &scene.animations {
            let mut channels_json = String::new();
            let mut samplers_json = String::new();
            let mut n_samplers = 0usize;
            for ch in &anim.channels {
                let Some(bone_idx) = scene.bones.iter().position(|b| b.name == ch.bone_name) else {
                    continue;
                };
                if ch.times.len() < 2
                    || ch.times.len() * 3 != ch.values.len() && ch.path == "translation"
                    || ch.path == "rotation" && ch.times.len() * 4 != ch.values.len()
                {
                    continue;
                }
                // input accessor (times)
                let mut tb = Vec::with_capacity(ch.times.len() * 4);
                let mut tmin = f32::MAX;
                let mut tmax = f32::MIN;
                for t in &ch.times {
                    tb.extend_from_slice(&t.to_le_bytes());
                    tmin = tmin.min(*t);
                    tmax = tmax.max(*t);
                }
                let v = w.view(&tb, 4);
                let input_acc = w.accessor(
                    v,
                    5126,
                    "SCALAR",
                    ch.times.len(),
                    Some((vec![tmin], vec![tmax])),
                );
                // output accessor (values)
                let mut vb = Vec::with_capacity(ch.values.len() * 4);
                for x in &ch.values {
                    vb.extend_from_slice(&x.to_le_bytes());
                }
                let (ty, count) = if ch.path == "rotation" {
                    ("VEC4", ch.times.len())
                } else {
                    ("VEC3", ch.times.len())
                };
                let vv = w.view(&vb, 4);
                let output_acc = w.accessor(vv, 5126, ty, count, None);
                if !channels_json.is_empty() {
                    channels_json.push(',');
                    samplers_json.push(',');
                }
                let interp = if ch.step { "STEP" } else { "LINEAR" };
                let _ = write!(
                    samplers_json,
                    "{{\"input\":{input_acc},\"output\":{output_acc},\"interpolation\":\"{interp}\"}}"
                );
                let node = bone_base + bone_idx;
                let _ = write!(
                    channels_json,
                    "{{\"sampler\":{n_samplers},\"target\":{{\"node\":{node},\"path\":\"{}\"}}}}",
                    ch.path
                );
                n_samplers += 1;
            }
            if n_samplers > 0 {
                anims.push(format!(
                    "{{\"name\":\"{}\",\"channels\":[{channels_json}],\"samplers\":[{samplers_json}]}}",
                    json_str(&anim.name)
                ));
            }
        }
        if !anims.is_empty() {
            animations_json = format!(",\"animations\":[{}]", anims.join(","));
        }
    }

    // ---- textures / images ----
    let mut images_json = String::new();
    let mut textures_json = String::new();
    for (i, t) in scene.textures.iter().enumerate() {
        let mut png = Vec::new();
        {
            let mut cur = std::io::Cursor::new(&mut png);
            write_png(
                &t.rgba,
                t.width as u32,
                t.height as u32,
                &mut std::io::BufWriter::new(&mut cur),
            )
            .map_err(|e| xy_core::Error::invalid("glb png", format!("{}: {e}", t.name)))?;
        }
        let v = w.view(&png, 1);
        if !images_json.is_empty() {
            images_json.push(',');
            textures_json.push(',');
        }
        let _ = write!(
            images_json,
            "{{\"bufferView\":{v},\"mimeType\":\"image/png\",\"name\":\"{}\"}}",
            json_str(&t.name)
        );
        let _ = write!(textures_json, "{{\"sampler\":0,\"source\":{i}}}");
    }

    // ---- materials ----
    let mut materials_json = String::new();
    for m in &scene.materials {
        if !materials_json.is_empty() {
            materials_json.push(',');
        }
        let tex = m
            .texture_index
            .filter(|&t| t < scene.textures.len())
            .map(|t| format!(",\"baseColorTexture\":{{\"index\":{t}}}"))
            .unwrap_or_default();
        let alpha = if m.alpha_blend {
            ",\"alphaMode\":\"BLEND\""
        } else {
            ""
        };
        let _ = write!(
            materials_json,
            "{{\"name\":\"{}\",\"doubleSided\":true{},\"pbrMetallicRoughness\":{{\"baseColorFactor\":[{},{},{},{}],\"metallicFactor\":0,\"roughnessFactor\":1{}}},\"emissiveFactor\":[{},{},{}]}}",
            json_str(&m.name),
            alpha,
            fmt_f32(m.base_color[0]),
            fmt_f32(m.base_color[1]),
            fmt_f32(m.base_color[2]),
            fmt_f32(m.base_color[3]),
            tex,
            fmt_f32(m.emissive[0]),
            fmt_f32(m.emissive[1]),
            fmt_f32(m.emissive[2])
        );
    }

    // ---- nodes: 0 = root, 1..=mesh_count = mesh nodes, then bones ----
    let mut nodes_json = String::new();
    let mut root_children: Vec<String> = (0..mesh_count).map(|i| (1 + i).to_string()).collect();
    if skin_needed {
        for (i, b) in scene.bones.iter().enumerate() {
            if b.parent < 0 {
                root_children.push((bone_base + i).to_string());
            }
        }
    }
    let _ = write!(
        nodes_json,
        "{{\"name\":\"{}\",\"children\":[{}]}}",
        json_str(&scene.name),
        root_children.join(",")
    );
    for i in 0..mesh_count {
        let skin = if skin_needed { ",\"skin\":0" } else { "" };
        let _ = write!(nodes_json, ",{{\"mesh\":{i}{skin}}}");
    }
    if skin_needed {
        for (i, b) in scene.bones.iter().enumerate() {
            let kids: Vec<String> = scene
                .bones
                .iter()
                .enumerate()
                .filter(|(_, bb)| bb.parent == i as i32)
                .map(|(j, _)| (bone_base + j).to_string())
                .collect();
            // TRS form (not matrix): keeps the rig animatable and matches
            // how glTF animation targets nodes.
            let q = quat_from_euler_xyz(&b.rotation_euler);
            let t = b.translation;
            let sc = b.scale;
            let children = if kids.is_empty() {
                String::new()
            } else {
                format!(",\"children\":[{}]", kids.join(","))
            };
            let _ = write!(
                nodes_json,
                ",{{\"name\":\"{}\"{},\"translation\":[{},{},{}],\"rotation\":[{},{},{},{}],\"scale\":[{},{},{}]}}",
                json_str(&b.name),
                children,
                fmt_f32(t[0]),
                fmt_f32(t[1]),
                fmt_f32(t[2]),
                fmt_f32(q[0]),
                fmt_f32(q[1]),
                fmt_f32(q[2]),
                fmt_f32(q[3]),
                fmt_f32(sc[0]),
                fmt_f32(sc[1]),
                fmt_f32(sc[2]),
            );
        }
    }

    let json = format!(
        "{{\"asset\":{{\"version\":\"2.0\",\"generator\":\"xy (PokeXYZ-RDComp)\"}},\"scene\":0,\
         \"scenes\":[{{\"name\":\"{}\",\"nodes\":[0]}}],\
         \"nodes\":[{nodes_json}],\
         \"meshes\":[{meshes_json}]\
         {skins_json}{animations_json},\
         \"materials\":[{materials_json}],\
         \"textures\":[{textures_json}],\
         \"images\":[{images_json}],\
         \"samplers\":[{{\"magFilter\":9729,\"minFilter\":9987,\"wrapS\":33071,\"wrapT\":33071}}],\
         \"accessors\":[{}],\
         \"bufferViews\":[{}],\
         \"buffers\":[{{\"byteLength\":{}}}]}}",
        json_str(&scene.name),
        w.accessors_json,
        w.views_json,
        w.bin.len(),
    );
    let _ = ibm_accessor;

    // ---- assemble the binary container ----
    let mut json_chunk = json.into_bytes();
    while json_chunk.len() % 4 != 0 {
        json_chunk.push(b' ');
    }
    let mut bin = std::mem::take(&mut w.bin);
    while bin.len() % 4 != 0 {
        bin.push(0);
    }
    let has_bin = !bin.is_empty();
    let total = 12 + 8 + json_chunk.len() + if has_bin { 8 + bin.len() } else { 0 };
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(&0x46546C67u32.to_le_bytes()); // "glTF"
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&(total as u32).to_le_bytes());
    out.extend_from_slice(&(json_chunk.len() as u32).to_le_bytes());
    out.extend_from_slice(&0x4E4F534Au32.to_le_bytes()); // "JSON"
    out.extend_from_slice(&json_chunk);
    if has_bin {
        out.extend_from_slice(&(bin.len() as u32).to_le_bytes());
        out.extend_from_slice(&0x004E4942u32.to_le_bytes()); // "BIN\0"
        out.extend_from_slice(&bin);
    }
    Ok(out)
}

fn bone_node_ids(scene: &GltfScene) -> impl Iterator<Item = usize> + '_ {
    let bone_base = 1 + scene
        .meshes
        .iter()
        .filter(|m| !m.positions.is_empty() && !m.indices.is_empty())
        .count();
    (0..scene.bones.len()).map(move |i| bone_base + i)
}

/// Column-major `T · Rx · Ry · Rz · S`.
fn compose_trs(t: &[f32; 3], r: &[f32; 3], s: &[f32; 3]) -> [f32; 16] {
    let (sin_x, cos_x) = r[0].sin_cos();
    let (sin_y, cos_y) = r[1].sin_cos();
    let (sin_z, cos_z) = r[2].sin_cos();
    // R = Rx · Ry · Rz (column-vector convention)
    let r00 = cos_y * cos_z;
    let r01 = -cos_y * sin_z;
    let r02 = sin_y;
    let r10 = sin_x * sin_y * cos_z + cos_x * sin_z;
    let r11 = -sin_x * sin_y * sin_z + cos_x * cos_z;
    let r12 = -sin_x * cos_y;
    let r20 = -cos_x * sin_y * cos_z + sin_x * sin_z;
    let r21 = cos_x * sin_y * sin_z + sin_x * cos_z;
    let r22 = cos_x * cos_y;
    [
        r00 * s[0],
        r10 * s[0],
        r20 * s[0],
        0.0, //
        r01 * s[1],
        r11 * s[1],
        r21 * s[1],
        0.0, //
        r02 * s[2],
        r12 * s[2],
        r22 * s[2],
        0.0, //
        t[0],
        t[1],
        t[2],
        1.0,
    ]
}

/// Quaternion (x, y, z, w) from XYZ euler radians. Matches the
/// column-vector matrix convention `Rx·Ry·Rz` used by [`compose_trs`]
/// (Hamilton product `qx ⊗ qy ⊗ qz`).
pub fn quat_from_euler_xyz(r: &[f32; 3]) -> [f32; 4] {
    let (sx, cx) = (r[0] * 0.5).sin_cos();
    let (sy, cy) = (r[1] * 0.5).sin_cos();
    let (sz, cz) = (r[2] * 0.5).sin_cos();
    // q = qx ⊗ qy ⊗ qz
    let q = quat_mul(
        quat_mul([sx, 0.0, 0.0, cx], [0.0, sy, 0.0, cy]),
        [0.0, 0.0, sz, cz],
    );
    let l = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if l > 1e-12 {
        [q[0] / l, q[1] / l, q[2] / l, q[3] / l]
    } else {
        [0.0, 0.0, 0.0, 1.0]
    }
}

/// Hamilton product of two quaternions (x, y, z, w).
fn quat_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
        a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
        a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
        a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
    ]
}

/// Rotate a vector by a quaternion (for parity tests vs the matrix path):
/// `v' = v + 2qw·(qv×v) + 2qv×(qv×v)`.
#[cfg(test)]
fn quat_rotate(q: [f32; 4], v: [f32; 3]) -> [f32; 3] {
    let qv = [q[0], q[1], q[2]];
    let cross = |a: [f32; 3], b: [f32; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let uv = cross(qv, v);
    let uuv = cross(qv, uv);
    [
        v[0] + 2.0 * (q[3] * uv[0] + uuv[0]),
        v[1] + 2.0 * (q[3] * uv[1] + uuv[1]),
        v[2] + 2.0 * (q[3] * uv[2] + uuv[2]),
    ]
}

/// Global (bind-pose) transform per bone (parents precede children).
fn bone_globals(bones: &[GltfBone]) -> Vec<[f32; 16]> {
    let mut globals = Vec::with_capacity(bones.len());
    for (i, b) in bones.iter().enumerate() {
        let local = compose_trs(&b.translation, &b.rotation_euler, &b.scale);
        let g = if b.parent >= 0 && (b.parent as usize) < i {
            mul4(&globals[b.parent as usize], &local)
        } else {
            local
        };
        globals.push(g);
    }
    globals
}

fn mul4(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut o = [0f32; 16];
    for c in 0..4 {
        for r in 0..4 {
            o[c * 4 + r] = a[r] * b[c * 4]
                + a[4 + r] * b[c * 4 + 1]
                + a[8 + r] * b[c * 4 + 2]
                + a[12 + r] * b[c * 4 + 3];
        }
    }
    o
}

/// General 4×4 inverse via Gauss-Jordan elimination (column-major
/// input/output; identity on singular input).
fn invert4(m: &[f32; 16]) -> [f32; 16] {
    let a = |r: usize, c: usize| m[c * 4 + r];
    let mut aug = [[0f32; 8]; 4];
    for (r, row) in aug.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate().take(4) {
            *v = a(r, c);
        }
        row[4 + r] = 1.0;
    }
    // Forward elimination with partial pivoting.
    for col in 0..4 {
        let pivot = (col..4)
            .max_by(|x, y| aug[*x][col].abs().total_cmp(&aug[*y][col].abs()))
            .unwrap_or(col);
        if aug[pivot][col].abs() < 1e-12 {
            return [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ];
        }
        aug.swap(col, pivot);
        let inv_pivot = 1.0 / aug[col][col];
        for v in &mut aug[col] {
            *v *= inv_pivot;
        }
        for r in 0..4 {
            if r == col {
                continue;
            }
            let factor = aug[r][col];
            if factor != 0.0 {
                let pivot_row = aug[col];
                for (v, pv) in aug[r].iter_mut().zip(pivot_row.iter()) {
                    *v -= factor * pv;
                }
            }
        }
    }
    let mut inv = [0f32; 16];
    for (r, row) in aug.iter().enumerate() {
        for c in 0..4 {
            inv[c * 4 + r] = row[4 + c];
        }
    }
    inv
}

fn fmt_f32(v: f32) -> String {
    if v == 0.0 {
        "0".into()
    } else {
        format!("{v}")
    }
}

fn json_str(s: &str) -> String {
    xy_core::util::json_escape(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tri_scene() -> GltfScene {
        GltfScene {
            name: "tri".into(),
            animations: Vec::new(),
            meshes: vec![GltfMesh {
                name: "m".into(),
                material_index: Some(0),
                positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
                normals: vec![[0.0, 0.0, 1.0]; 3],
                uvs: vec![[0.0, 1.0], [1.0, 1.0], [0.0, 0.0]],
                colors: vec![[1.0, 1.0, 1.0, 1.0]; 3],
                joints: vec![[0, 0, 0, 0]; 3],
                weights: vec![[0.0; 4]; 3],
                indices: vec![0, 1, 2],
            }],
            materials: vec![GltfMaterial {
                name: "mat".into(),
                base_color: [1.0, 1.0, 1.0, 1.0],
                emissive: [0.0; 3],
                texture_index: Some(0),
                alpha_blend: false,
            }],
            bones: vec![GltfBone {
                name: "root".into(),
                parent: -1,
                translation: [0.0; 3],
                rotation_euler: [0.0; 3],
                scale: [1.0; 3],
            }],
            textures: vec![GltfTexture {
                name: "tex".into(),
                width: 1,
                height: 1,
                rgba: vec![255, 0, 0, 255],
            }],
        }
    }

    #[test]
    fn glb_structure_is_valid() {
        let glb = export_glb(&tri_scene()).unwrap();
        assert_eq!(&glb[0..4], b"glTF");
        assert_eq!(u32::from_le_bytes(glb[4..8].try_into().unwrap()), 2);
        let total = u32::from_le_bytes(glb[8..12].try_into().unwrap()) as usize;
        assert_eq!(total, glb.len(), "declared length matches file size");
        let json_len = u32::from_le_bytes(glb[12..16].try_into().unwrap()) as usize;
        assert_eq!(&glb[16..20], b"JSON");
        assert_eq!(json_len % 4, 0, "JSON chunk padded to 4");
        let json = std::str::from_utf8(&glb[20..20 + json_len]).unwrap();
        assert!(json.contains("\"POSITION\""));
        assert!(json.contains("\"skins\""));
        assert!(json.contains("\"image/png\""));
        assert!(json.contains("\"inverseBindMatrices\""));
        // BIN chunk follows
        let bin_hdr = 20 + json_len;
        assert_eq!(&glb[bin_hdr + 4..bin_hdr + 8], b"BIN\x00");
    }

    #[test]
    fn empty_scene_still_valid() {
        let glb = export_glb(&GltfScene::default()).unwrap();
        assert_eq!(&glb[0..4], b"glTF");
    }

    #[test]
    fn matrix_compose_identity() {
        let m = compose_trs(&[0.0; 3], &[0.0; 3], &[1.0; 3]);
        assert_eq!(m[0], 1.0);
        assert_eq!(m[5], 1.0);
        assert_eq!(m[10], 1.0);
        assert_eq!(m[15], 1.0);
        let inv = invert4(&m);
        assert!((inv[0] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn matrix_inverse_roundtrip() {
        let m = compose_trs(&[3.0, -2.0, 5.0], &[0.3, 0.8, -0.4], &[1.0, 2.0, 0.5]);
        let inv = invert4(&m);
        let id = mul4(&m, &inv);
        for (i, x) in id.iter().enumerate() {
            let expect = if i % 5 == 0 { 1.0 } else { 0.0 };
            assert!((x - expect).abs() < 1e-4, "id[{i}]={x}");
        }
    }

    #[test]
    fn quat_matches_matrix_rotation() {
        // For the column-vector convention M = Rx·Ry·Rz, the quaternion
        // qx ⊗ qy ⊗ qz must rotate vectors identically.
        for r in [
            [0.3, 0.8, -0.4],
            [1.2, 0.0, 0.0],
            [0.0, 0.0, 2.2],
            [-0.9, 0.5, 1.7],
        ] {
            let m = compose_trs(&[0.0; 3], &r, &[1.0; 3]);
            let q = quat_from_euler_xyz(&r);
            for v in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.2, -0.9]] {
                let by_matrix = [
                    m[0] * v[0] + m[4] * v[1] + m[8] * v[2],
                    m[1] * v[0] + m[5] * v[1] + m[9] * v[2],
                    m[2] * v[0] + m[6] * v[1] + m[10] * v[2],
                ];
                let by_quat = quat_rotate(q, v);
                for a in 0..3 {
                    assert!(
                        (by_matrix[a] - by_quat[a]).abs() < 1e-5,
                        "r={r:?} v={v:?} m={by_matrix:?} q={by_quat:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn translation_lands_in_matrix() {
        let m = compose_trs(&[7.0, 8.0, 9.0], &[0.0; 3], &[1.0; 3]);
        assert_eq!((m[12], m[13], m[14]), (7.0, 8.0, 9.0));
    }
}
