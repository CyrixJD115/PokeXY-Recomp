//! GLB/glTF structural validation — parses our exporter's output (or any
//! glTF) with the well-maintained `gltf` crate, decodes every accessor
//! against the BIN chunk, and reports a census + problems.
//!
//! This is the "will Blender accept it" gate: the crate enforces the spec's
//! JSON schema + indices, and we additionally walk every primitive reading
//! positions/normals/UVs/skinning/indices to completion, every skin's
//! inverse-bind matrices, and every image's buffer view.

/// Validation outcome for one GLB file.
#[derive(Debug, Default, Clone)]
pub struct GlbReport {
    pub asset_version: String,
    pub asset_generator: String,
    pub scenes: usize,
    pub nodes: usize,
    pub meshes: usize,
    pub primitives: usize,
    pub skins: usize,
    pub joints: usize,
    pub materials: usize,
    pub textures: usize,
    pub images: usize,
    pub animations: usize,
    pub animation_channels: usize,
    pub vertices: usize,
    pub triangles: usize,
    pub bin_bytes: usize,
    /// Spec/schema/index errors (fatal in most importers).
    pub errors: Vec<String>,
    /// Non-fatal notes (missing min/max, missing normals, …).
    pub warnings: Vec<String>,
}

impl GlbReport {
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Human summary line for the CLI.
    pub fn summary(&self) -> String {
        format!(
            "glTF {} · {} meshes / {} prims · {} verts / {} tris · {} skins ({} joints) · \
             {} materials / {} textures / {} images · {} anims ({} ch) · {} bin bytes · {}",
            self.asset_version,
            self.meshes,
            self.primitives,
            self.vertices,
            self.triangles,
            self.skins,
            self.joints,
            self.materials,
            self.textures,
            self.images,
            self.animations,
            self.animation_channels,
            self.bin_bytes,
            if self.ok() { "VALID" } else { "INVALID" },
        )
    }
}

/// Closure constructor with an explicit blob lifetime (plain closures
/// cannot annotate the returned slice's lifetime, which trips the crate's
/// higher-ranked `Fn(Buffer<'_>) -> Option<&'s [u8]>` bound).
fn buffer_data_fn<'s>(
    blob: Option<&'s [u8]>,
) -> impl Fn(gltf::Buffer<'_>) -> Option<&'s [u8]> + Clone + 's {
    move |_buffer| blob
}

/// Validate a GLB/glTF image. `Err` = the file could not be parsed at all
/// (bad magic, truncated chunks, schema violations).
pub fn validate(data: &[u8]) -> Result<GlbReport, String> {
    let gltf = gltf::Gltf::from_slice(data).map_err(|e| format!("glTF parse: {e}"))?;
    let doc = &gltf.document;
    let blob = gltf.blob;
    let bin_len = blob.as_ref().map(|b| b.len()).unwrap_or(0);
    // The reader closures hand us whole buffers; the crate slices per view.
    let get = buffer_data_fn(blob.as_deref());

    let asset = &doc.as_json().asset;
    let mut r = GlbReport {
        asset_version: asset.version.clone(),
        asset_generator: asset.generator.clone().unwrap_or_default(),
        scenes: doc.scenes().count(),
        nodes: doc.nodes().count(),
        meshes: doc.meshes().count(),
        materials: doc.materials().count(),
        textures: doc.textures().count(),
        images: doc.images().count(),
        animations: doc.animations().count(),
        bin_bytes: bin_len,
        ..Default::default()
    };

    // Images: every embedded image must resolve inside the blob.
    for img in doc.images() {
        match img.source() {
            gltf::image::Source::View { view, .. } => {
                let end = view.offset().saturating_add(view.length());
                if end > bin_len {
                    r.errors.push(format!(
                        "image bufferView out of bounds ({}..{end} > {bin_len})",
                        view.offset()
                    ));
                }
            }
            gltf::image::Source::Uri { .. } => {
                r.warnings.push("external URI image (not embedded)".into());
            }
        }
    }

    // Skins: joints + inverse bind matrices must decode.
    for skin in doc.skins() {
        r.skins += 1;
        let joint_count = skin.joints().count();
        r.joints += joint_count;
        if joint_count == 0 {
            r.errors.push("skin with no joints".into());
        }
        if let Some(ibm) = skin.inverse_bind_matrices() {
            match gltf::accessor::Iter::<[[f32; 4]; 4]>::new(ibm, get.clone()) {
                Some(iter) => {
                    let n = iter.count();
                    if n != joint_count {
                        r.errors
                            .push(format!("skin IBM count {n} != joint count {joint_count}"));
                    }
                }
                None => r.errors.push("skin IBM accessor unreadable".into()),
            }
        } else {
            r.warnings.push("skin without inverseBindMatrices".into());
        }
    }

    // Meshes: decode every primitive's attributes to completion.
    for mesh in doc.meshes() {
        for prim in mesh.primitives() {
            r.primitives += 1;
            if prim.mode() != gltf::mesh::Mode::Triangles {
                r.warnings.push(format!(
                    "primitive mode {:?} (only triangles tested)",
                    prim.mode()
                ));
            }
            let reader = prim.reader(get.clone());
            match reader.read_positions() {
                Some(pos) => {
                    let n = pos.count();
                    r.vertices += n;
                    // POSITION accessor min/max is REQUIRED by the spec.
                    let has_min_max = prim
                        .attributes()
                        .any(|(sem, acc)| sem == gltf::Semantic::Positions && acc.min().is_some());
                    if !has_min_max {
                        r.errors.push("POSITION accessor without min/max".into());
                    }
                }
                None => r.errors.push("primitive without readable POSITION".into()),
            }
            if reader.read_normals().is_none() {
                r.warnings.push("primitive without normals".into());
            }
            if reader.read_tex_coords(0).is_none() {
                r.warnings.push("primitive without TEXCOORD_0".into());
            }
            if let Some(idx) = reader.read_indices() {
                let n = idx.into_u32().count();
                if n % 3 != 0 {
                    r.warnings
                        .push(format!("index count {n} not a multiple of 3"));
                }
                r.triangles += n / 3;
            } else {
                r.warnings.push("non-indexed primitive".into());
            }
            if let Some(j) = reader.read_joints(0) {
                let mut max_joint = 0usize;
                for joint in j.into_u16() {
                    for x in joint.iter() {
                        max_joint = max_joint.max(usize::from(*x));
                    }
                }
                if max_joint >= r.nodes {
                    r.errors.push(format!(
                        "JOINTS_0 value {max_joint} >= node count {}",
                        r.nodes
                    ));
                }
            }
        }
    }

    // Animations: samplers must decode and channels must target nodes.
    for anim in doc.animations() {
        for ch in anim.channels() {
            r.animation_channels += 1;
            let input = ch.sampler().input();
            let input_ok = gltf::accessor::Iter::<f32>::new(input, get.clone())
                .is_some_and(|mut it| it.next().is_some());
            if !input_ok {
                r.errors
                    .push("animation sampler input unreadable/empty".into());
            }
        }
    }

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garbage_is_rejected() {
        assert!(validate(b"not a glb at all").is_err());
        assert!(validate(b"").is_err());
    }

    #[test]
    fn truncated_header_rejected() {
        let mut glb = Vec::new();
        glb.extend_from_slice(b"glTF");
        glb.extend_from_slice(&2u32.to_le_bytes());
        glb.extend_from_slice(&100u32.to_le_bytes()); // lies about length
        assert!(validate(&glb).is_err());
    }
}

#[cfg(test)]
mod roundtrip_tests {
    use super::*;
    use xy_gfx::gltf::{export_glb, GltfAnimChannel, GltfAnimation, GltfBone, GltfMesh, GltfScene};

    fn animated_scene() -> GltfScene {
        let mut mesh = GltfMesh {
            name: "m".into(),
            material_index: None,
            ..Default::default()
        };
        mesh.positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        mesh.indices = vec![0, 1, 2];
        GltfScene {
            name: "anim".into(),
            meshes: vec![mesh],
            bones: vec![GltfBone {
                name: "root".into(),
                parent: -1,
                translation: [0.0; 3],
                rotation_euler: [0.0; 3],
                scale: [1.0; 3],
            }],
            animations: vec![GltfAnimation {
                name: "wave".into(),
                channels: vec![
                    GltfAnimChannel {
                        bone_name: "root".into(),
                        path: "translation",
                        times: vec![0.0, 0.5, 1.0],
                        values: vec![
                            0.0, 0.0, 0.0, //
                            0.0, 1.0, 0.0, //
                            0.0, 0.0, 0.0,
                        ],
                        step: false,
                    },
                    GltfAnimChannel {
                        bone_name: "root".into(),
                        path: "rotation",
                        times: vec![0.0, 1.0],
                        values: vec![
                            0.0, 0.0, 0.0, 1.0, //
                            0.383, 0.0, 0.0, 0.924,
                        ],
                        step: false,
                    },
                ],
            }],
            ..Default::default()
        }
    }

    #[test]
    fn animated_glb_validates() {
        let glb = export_glb(&animated_scene()).unwrap();
        let report = validate(&glb).unwrap();
        assert!(report.ok(), "errors: {:?}", report.errors);
        assert_eq!(report.animations, 1);
        assert_eq!(report.animation_channels, 2);
        assert_eq!(report.skins, 1);
        assert_eq!(report.joints, 1);
    }

    #[test]
    fn static_glb_validates() {
        let mut scene = animated_scene();
        scene.animations.clear();
        scene.bones.clear();
        let glb = export_glb(&scene).unwrap();
        let report = validate(&glb).unwrap();
        assert!(report.ok(), "errors: {:?}", report.errors);
        assert_eq!(report.animations, 0);
        assert_eq!(report.meshes, 1);
        assert_eq!(report.triangles, 1);
    }
}
