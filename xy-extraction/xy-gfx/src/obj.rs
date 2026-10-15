//! Minimal Wavefront OBJ export for decoded models (Blender-ready).

use crate::bch;
use crate::gf_model::ModelGroup;
use std::io::Write;
use xy_core::error::Result;

/// Export every model in the group as `<base>_<model name>.obj`.
/// Each decoded draw-order vertex is emitted (`v`/`vn`/`vt`) with faces as
/// sequential triples — the CTR index buffer lists vertices in draw order.
pub fn export_group(group: &ModelGroup, base: &std::path::Path) -> Result<Vec<std::path::PathBuf>> {
    let mut written = Vec::new();
    for model in &group.models {
        let safe: String = model
            .name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let path = base.join(format!("{}_{}.obj", safe, model.meshes.len()));
        let mut f = std::io::BufWriter::new(std::fs::File::create(&path)?);
        writeln!(f, "# exported by xy (GF model port)")?;
        let mut v_out = 0usize;
        for mesh in &model.meshes {
            writeln!(f, "o {}", mesh.name)?; // placeholder; replaced below
            for v in &mesh.vertices {
                writeln!(f, "v {} {} {}", v.position[0], v.position[1], v.position[2])?;
                writeln!(f, "vn {} {} {}", v.normal[0], v.normal[1], v.normal[2])?;
                writeln!(f, "vt {} {}", v.uv[0], 1.0 - v.uv[1])?;
                v_out += 1;
            }
            // faces: sequential triples over this mesh's vertices
            let start = v_out - mesh.vertices.len() + 1;
            let tris = mesh.vertices.len() / 3;
            for t in 0..tris {
                let (a, b, c) = (start + t * 3, start + t * 3 + 1, start + t * 3 + 2);
                writeln!(f, "f {a}/{a}/{a} {b}/{b}/{b} {c}/{c}/{c}")?;
            }
        }
        written.push(path);
    }
    Ok(written)
}

// ---- BCH model export (OBJ + MTL) ----

fn safe_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Expand a PICA primitive run into a triangle list (draw-order indices).
fn triangles(mode: u8, indices: &[u16]) -> Vec<usize> {
    let mut out = Vec::new();
    match mode {
        1 => {
            // triangle strip: alternate winding per triangle
            for (i, w) in indices.windows(3).enumerate() {
                if i & 1 == 0 {
                    out.extend_from_slice(&[w[0] as usize, w[1] as usize, w[2] as usize]);
                } else {
                    out.extend_from_slice(&[w[1] as usize, w[0] as usize, w[2] as usize]);
                }
            }
        }
        2 => {
            // triangle fan
            if let Some((&first, rest)) = indices.split_first() {
                for w in rest.windows(2) {
                    out.extend_from_slice(&[first as usize, w[0] as usize, w[1] as usize]);
                }
            }
        }
        // 0 = triangles (3 = geometry primitive, not triangulable)
        _ => out.extend(indices.iter().map(|&i| i as usize)),
    }
    out
}

/// Export a BCH model as `<base>_<model name>.obj` (+ `.mtl` when any
/// material references textures or colors). Returns the files written.
pub fn export_bch_model(
    model: &bch::Model,
    base: &std::path::Path,
) -> Result<Vec<std::path::PathBuf>> {
    let stem = safe_name(&model.name);
    let mtl_path = base.join(format!("{stem}.mtl"));
    let obj_path = base.join(format!("{stem}.obj"));

    // MTL (always written: carries the material colors even without maps).
    let mut mtl = std::io::BufWriter::new(std::fs::File::create(&mtl_path)?);
    for mat in &model.materials {
        writeln!(mtl, "newmtl {}", mat.name)?;
        let kd = |c: [u8; 4]| {
            format!(
                "{:.6} {:.6} {:.6}",
                c[0] as f32 / 255.0,
                c[1] as f32 / 255.0,
                c[2] as f32 / 255.0
            )
        };
        writeln!(mtl, "Ka {}", kd(mat.ambient))?;
        writeln!(mtl, "Kd {}", kd(mat.diffuse))?;
        writeln!(mtl, "Ks {}", kd(mat.specular0))?;
        writeln!(mtl, "Ke {}", kd(mat.emission))?;
        if let Some(tex) = &mat.texture_names[0] {
            writeln!(mtl, "map_Kd {tex}.png")?;
        }
        writeln!(mtl)?;
    }
    drop(mtl);

    // OBJ
    let mut obj = std::io::BufWriter::new(std::fs::File::create(&obj_path)?);
    writeln!(obj, "# exported by xy (BCH/CtrH3D port)")?;
    writeln!(obj, "mtllib {stem}.mtl")?;
    let mut v_out = 0usize;
    for (mi, mesh) in model.meshes.iter().enumerate() {
        let object = model
            .node_names
            .get(mesh.node_index)
            .cloned()
            .unwrap_or_else(|| format!("mesh_{mi}"));
        writeln!(obj, "o {}", safe_name(&object))?;
        for v in &mesh.vertices {
            writeln!(
                obj,
                "v {} {} {}",
                v.position[0], v.position[1], v.position[2]
            )?;
        }
        for v in &mesh.vertices {
            writeln!(obj, "vn {} {} {}", v.normal[0], v.normal[1], v.normal[2])?;
        }
        for v in &mesh.vertices {
            writeln!(obj, "vt {} {}", v.uv[0][0], 1.0 - v.uv[0][1])?;
        }
        if let Some(mat) = model.materials.get(mesh.material_index) {
            writeln!(obj, "usemtl {}", mat.name)?;
        }
        // Faces reference the mesh's shared vertex buffer; OBJ indices are
        // 1-based and include the accumulated offset.
        for sub in &mesh.submeshes {
            let tris = triangles(sub.primitive_mode, &sub.indices);
            for t in tris.as_chunks::<3>().0 {
                let (a, b, c) = (t[0] + v_out + 1, t[1] + v_out + 1, t[2] + v_out + 1);
                writeln!(obj, "f {a}/{a}/{a} {b}/{b}/{b} {c}/{c}/{c}")?;
            }
        }
        v_out += mesh.vertices.len();
    }
    Ok(vec![obj_path, mtl_path])
}
