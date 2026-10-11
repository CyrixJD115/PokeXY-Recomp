//! Golden test: full CGFX decode against the real Bulbasaur PC container
//! (`src_extracted/garc/a/0/0/7/0001.bin`, recon 2026-10-06). Skips when
//! the asset is absent.
//!
//! Pinned facts (hand-verified via hexdump before the port):
//! - PC container: 11 entries; entry 1 = CGFX (shaders + 1 texture
//!   "star"), entries 3..7 = CGFX effect models es001..es005_ca, entry 2
//!   + 8..10 = BCH (skeleton + anims, separate port).
//! - es001_ca (entry 3) carries 3 textures (etxp_circle02, etxp_ring01,
//!   etxp_sleep01) and geometry.

use std::path::PathBuf;

use xy_gfx::cgfx::Cgfx;
use xy_gfx::pkmn::PkmnContainer;

fn asset(rel: &str) -> Option<PathBuf> {
    let p = std::env::var("XY_DATA_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.Data"))
        .join(rel);
    p.exists().then_some(p)
}

fn bulbasaur() -> Option<Vec<Vec<u8>>> {
    let path = asset("src_extracted/garc/a/0/0/7/0001.bin")?;
    let bytes = std::fs::read(&path).unwrap();
    let pc = PkmnContainer::parse(&bytes).expect("pc parse");
    Some(pc.entries.iter().map(|e| e.to_vec()).collect())
}

#[test]
fn shader_bundle_entry_parses() {
    let Some(entries) = bulbasaur() else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let cgfx = Cgfx::parse(&entries[0]).expect("entry 1 CGFX");
    assert_eq!(cgfx.models.len(), 0, "entry 1 has no models (recon)");
    assert_eq!(cgfx.textures.len(), 1);
    assert_eq!(cgfx.textures[0].name, "star");
}

#[test]
fn effect_model_decodes_materials_and_textures() {
    let Some(entries) = bulbasaur() else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    let cgfx = Cgfx::parse(&entries[2]).expect("entry 2 CGFX (es001_ca)");
    assert_eq!(cgfx.models.len(), 1);
    let model = &cgfx.models[0];
    assert_eq!(model.name, "es001_ca");
    assert_eq!(model.materials.len(), 3);
    // XY rev-9 effect models link geometry through CNOD chunks — the SOBJ
    // shape layout differs from the SDK and is skipped
    // (`geometry_partial`); materials/textures still decode.
    assert!(model.geometry_partial);
    assert!(model.meshes.is_empty());

    let names: Vec<&str> = cgfx.textures.iter().map(|t| t.name.as_str()).collect();
    for expected in ["etxp_circle02", "etxp_ring01", "etxp_sleep01"] {
        assert!(
            names.contains(&expected),
            "texture {expected} missing from {names:?}"
        );
    }

    // At least one material binds one of those textures by name.
    let bound: Vec<&str> = model
        .materials
        .iter()
        .flat_map(|m| m.texture_names.iter().flatten())
        .map(String::as_str)
        .collect();
    assert!(
        bound.iter().any(|n| names.contains(n)),
        "no material texture binding matches {names:?}: {bound:?}"
    );
}

#[test]
fn textures_decode_to_non_trivial_images() {
    let Some(entries) = bulbasaur() else {
        eprintln!("skipping: golden asset not found");
        return;
    };
    for entry in [0usize, 2] {
        let cgfx = Cgfx::parse(&entries[entry]).expect("CGFX parse");
        for tex in &cgfx.textures {
            let rgba = tex
                .decode_rgba()
                .unwrap_or_else(|e| panic!("texture {} decode: {e}", tex.name));
            assert_eq!(rgba.len(), tex.width * tex.height * 4);
            // Non-trivial: more than one distinct RGBA color.
            let first = &rgba[..4];
            assert!(
                rgba.chunks_exact(4).any(|px| px != first),
                "texture {} decoded to a flat image",
                tex.name
            );
        }
    }
}
