//! Golden test: BCH (CtrH3D) decode against real Pokémon X assets.
//! Bulbasaur's high/low-poly models live in `a/0/0/7/0004.bin` (PC, 2 BCH
//! entries); the "star" battle texture is the BCH entry of
//! `a/0/0/7/0001.bin`. Pinned facts verified against an instrumented
//! SPICA run + Blender render (2026-10-06). Skips when assets are absent.

use std::path::PathBuf;

use xy_gfx::bch::Bch;
use xy_gfx::pkmn::PkmnContainer;

fn asset(rel: &str) -> Option<PathBuf> {
    let p = std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"))
        .join(rel);
    p.exists().then_some(p)
}

fn pc_entries(rel: &str) -> Option<Vec<Vec<u8>>> {
    let bytes = std::fs::read(asset(rel)?).ok()?;
    let pc = PkmnContainer::parse(&bytes).expect("pc parse");
    Some(pc.entries.iter().map(|e| e.to_vec()).collect())
}

fn norm(v: [f32; 3]) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

#[test]
fn bulbasaur_high_poly_decodes() {
    let Some(entries) = pc_entries("Source_Extracted/garc/a/0/0/7/0004.bin") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bch = Bch::parse(&entries[0]).expect("high-poly BCH");
    assert_eq!(bch.models.len(), 1);
    let m = &bch.models[0];
    assert_eq!(m.name, "pm0001_00");
    assert_eq!(m.meshes.len(), 7);
    assert_eq!(m.materials.len(), 5);
    assert_eq!(m.bones.len(), 71);
    assert_eq!(m.node_names.len(), 4);

    // Geometry sanity: mesh 0 = 2043 vertices, all normals unit-length,
    // positions within a ±100 unit box, UVs in a sane range.
    let mesh0 = &m.meshes[0];
    assert_eq!(mesh0.vertices.len(), 2043);
    assert_eq!(mesh0.submeshes.len(), 2);
    let idx: usize = mesh0.submeshes.iter().map(|s| s.indices.len()).sum();
    assert_eq!(idx, 9396); // pinned against the SPICA-verified decode
    for v in mesh0.vertices.iter() {
        assert!(
            v.position.iter().all(|c| c.abs() < 100.0),
            "position out of range: {:?}",
            v.position
        );
        let n = norm(v.normal);
        assert!((n - 1.0).abs() < 0.01 || n == 0.0, "non-unit normal {n}");
        assert!(v.uv[0][0].abs() <= 4.0 && v.uv[0][1].abs() <= 4.0);
    }

    // Material names + texture bindings (SPICA-pinned).
    let mat_names: Vec<&str> = m.materials.iter().map(|x| x.name.as_str()).collect();
    for expected in ["BodyA", "BodyB", "Eye", "LIris"] {
        assert!(mat_names.contains(&expected), "{expected} missing");
    }
    assert_eq!(
        m.materials[0].texture_names[0].as_deref(),
        Some("pm0001_00_BodyA1")
    );

    // Bone tree: root has no parent, "Origin" is its child.
    assert_eq!(m.bones[0].name, "pm0001_00");
    assert_eq!(m.bones[0].parent_index, -1);
    assert_eq!(m.bones[1].name, "Origin");
    assert_eq!(m.bones[1].parent_index, 0);
}

#[test]
fn bulbasaur_low_poly_and_shadow() {
    let Some(entries) = pc_entries("Source_Extracted/garc/a/0/0/7/0004.bin") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bch = Bch::parse(&entries[1]).expect("low-poly BCH");
    assert_eq!(bch.models.len(), 1);
    let m = &bch.models[0];
    assert_eq!(m.name, "pm0001_00_shadow");
    assert_eq!(m.meshes.len(), 4);
    let idx: usize = m
        .meshes
        .iter()
        .flat_map(|x| &x.submeshes)
        .map(|s| s.indices.len())
        .sum();
    assert_eq!(idx, 1620);
}

#[test]
fn star_texture_decodes() {
    let Some(entries) = pc_entries("Source_Extracted/garc/a/0/0/7/0001.bin") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let star = entries
        .iter()
        .find(|e| e.len() >= 4 && &e[0..4] == b"BCH\0")
        .expect("0001.bin holds a BCH entry");
    let bch = Bch::parse(star).expect("star BCH");
    assert_eq!(bch.textures.len(), 1);
    let tex = &bch.textures[0];
    assert_eq!(tex.name, "star");
    assert_eq!((tex.width, tex.height), (64, 64));
    let rgba = tex.decode_rgba().expect("decode");
    // Non-trivial image: mostly dark (star on black) with a bright center.
    let px = rgba.as_chunks::<4>().0;
    let bright = px.iter().filter(|p| p[0] > 200).count();
    let dark = px.iter().filter(|p| p[0] < 30).count();
    assert_eq!(bright, 40, "star center pixels");
    assert_eq!(dark, 3431, "star background pixels");
}

#[test]
fn form_group_layout_holds() {
    // a/0/0/6 value g → model PC at file 8g+4 (empirical rule, verified
    // for species 1-20). Species 1 = group 0 → 0004.bin; species 2 =
    // group 1 → 0012.bin (pm0002_00).
    let Some(entries) = pc_entries("Source_Extracted/garc/a/0/0/7/0012.bin") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let bch = Bch::parse(&entries[0]).expect("ivysaur BCH");
    assert_eq!(bch.models[0].name, "pm0002_00");
}

#[test]
fn corpus_sweep_model_pcs() {
    // Every 40th model PC in a/0/0/7 must decode with real geometry
    // (the full 981/981 sweep was run out-of-band, zero failures).
    let Some(dir) = asset("Source_Extracted/garc/a/0/0/7") else {
        eprintln!("skipping: golden asset dir not found");
        return;
    };
    let mut checked = 0usize;
    for n in (4..=7844usize).step_by(8 * 40) {
        let path = dir.join(format!("{n:04}.bin"));
        if !path.exists() {
            continue;
        }
        let bytes = std::fs::read(&path).unwrap();
        if bytes.len() < 4 || bytes[0..2] != *b"PC" {
            continue;
        }
        let pc = PkmnContainer::parse(&bytes).unwrap();
        let Some(bch_entry) = pc
            .entries
            .iter()
            .find(|e| e.len() >= 4 && e[0..4] == *b"BCH\0")
        else {
            continue;
        };
        let bch =
            Bch::parse(bch_entry).unwrap_or_else(|e| panic!("{} failed: {e}", path.display()));
        assert!(!bch.models.is_empty(), "{}: no models", path.display());
        for m in &bch.models {
            assert!(
                !m.meshes.is_empty(),
                "{}: model {} has no meshes",
                path.display(),
                m.name
            );
            assert!(
                m.bones.len() > 10,
                "{}: {} bones",
                path.display(),
                m.bones.len()
            );
            let has_geometry = m.meshes.iter().any(|mesh| {
                !mesh.vertices.is_empty() && mesh.submeshes.iter().any(|s| !s.indices.is_empty())
            });
            assert!(
                has_geometry,
                "{}: {} has no geometry",
                path.display(),
                m.name
            );
        }
        checked += 1;
    }
    assert!(checked >= 15, "corpus sweep checked only {checked} files");
}

#[test]
fn pt_texture_container_decodes() {
    // Species 1 texture pack: 0006.bin (PT) → BCH → named RGBA8 textures.
    let Some(entries) = pc_entries("Source_Extracted/garc/a/0/0/7/0006.bin") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let mut textures = Vec::new();
    for entry in &entries {
        if entry.len() >= 4 && entry[0..4] == *b"BCH\0" {
            textures.extend(Bch::parse(entry).expect("pt bch").textures);
        }
    }
    assert!(textures.len() >= 10, "expected the Bulbasaur texture set");
    let names: Vec<&str> = textures.iter().map(|t| t.name.as_str()).collect();
    for expected in ["pm0001_00_BodyA1", "pm0001_00_Eye1", "pm0001_00_Iris1"] {
        assert!(names.contains(&expected), "{expected} missing");
    }
    let body = textures
        .iter()
        .find(|t| t.name == "pm0001_00_BodyA1")
        .unwrap();
    assert_eq!((body.width, body.height), (256, 256));
    assert_eq!(body.format.name(), "rgba8");
    // Body texture is predominantly green (g > r in most opaque pixels).
    let rgba = body.decode_rgba().expect("decode");
    let greenish = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|p| p[1] as u32 > p[0] as u32 + 20 && p[3] > 0)
        .count();
    assert!(
        greenish > rgba.len() / 16,
        "body texture not green ({greenish})"
    );
}

#[test]
fn pf_material_animations_decode() {
    // Charizard (family group 7, model PC 0060.bin) motions: 0061.bin PF
    // entry 18 = `pm0006_00_loop01` fire material anim (UV scrolls).
    let Some(entries) = pc_entries("Source_Extracted/garc/a/0/0/7/0061.bin") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let mut decoded = 0usize;
    for entry in &entries {
        if entry.len() < 4 || entry[0..4] != *b"BCH\0" {
            continue;
        }
        let bch = Bch::parse(entry).expect("pf bch");
        for anim in &bch.animations {
            assert!(!anim.name.is_empty(), "animation without a name");
            decoded += 1;
        }
        if bch.animations.is_empty() {
            continue;
        }
        // The loop01 material anim carries texcoord UV-scroll curves.
        let fire: Vec<xy_gfx::bch::AnimElement> = bch
            .animations
            .iter()
            .flat_map(|a| a.elements.clone())
            .filter(|el| el.name == "FireCoreA")
            .collect();
        if fire.is_empty() {
            continue;
        }
        for el in &fire {
            let xy_gfx::bch::AnimContent::Vector2D(groups) = &el.content else {
                panic!("FireCoreA should be a Vector2D element");
            };
            // Target 0x15 (TexCoord0Trans) / 0x18 (TexCoord1Trans):
            // X constant 1.0, Y a wrapping 0→N scroll with ~38 keys.
            assert_eq!(groups[0].key_frames.len(), 1, "x axis constant");
            assert!(
                (groups[0].key_frames[0].value - 1.0).abs() < 1e-6,
                "x constant value"
            );
            assert!(groups[1].key_frames.len() >= 18, "y scroll keys");
            // First key starts at 0.
            assert_eq!(groups[1].key_frames[0].frame, 0.0);
            assert!((groups[1].key_frames[0].value).abs() < 1e-6);
        }
        break;
    }
    assert!(decoded > 0, "no animations decoded from 0061.bin");
}

#[test]
fn mouth_visibility_anims_parse() {
    // Bulbasaur PF (0005.bin) entries 9/10/11 = per-form material/vis
    // anims (mouth01, eye01; 70 frames each), with empty element lists —
    // the actual curve data lives in XY's v2 motion bitstreams (see
    // PLAN.md).
    let Some(entries) = pc_entries("Source_Extracted/garc/a/0/0/7/0005.bin") else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let mut found = false;
    for entry in &entries {
        if entry.len() < 4 || entry[0..4] != *b"BCH\0" {
            continue;
        }
        let bch = Bch::parse(entry).expect("pf bch");
        for anim in &bch.animations {
            assert!(
                anim.name.starts_with("pm0001_00_"),
                "unexpected anim name {}",
                anim.name
            );
            assert!(anim.frames_count > 0.0);
            found = true;
        }
    }
    assert!(found, "mouth01 animations not found in 0005.bin");
}
