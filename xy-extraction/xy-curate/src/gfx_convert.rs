//! Pokémon graphics conversion stage for `xy curate`: organize the `a/0/0/7`
//! 8-file families under per-form directories and convert every model to
//! OBJ+MTL and every model/texture pack to PNG — real files, no symlinks.

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use xy_gfx::bch::{self, Texture};
use xy_gfx::texture::write_png;
use xy_gfx::{cgfx, obj, pkmn};
use xy_script::textfile::TextFile;

/// Family roles within an 8-file group (`a/0/0/7` group id g = files 8g..8g+7).
const FAMILY_SIZE: usize = 8;
const MODEL_SLOT: usize = 4; // hi/lo model PC
const TEXTURE_SLOTS: &[usize] = &[2, 6, 7]; // CGFX textures, PT packs

/// Aggregate counters for the stage summary.
#[derive(Debug, Default)]
pub struct PokemonStats {
    pub families: usize,
    pub family_files: usize,
    pub models_obj: usize,
    pub textures_png: usize,
    pub orphans: usize,
}

/// Discover Pokémon model families in the graphics GARC dump and name them
/// from each model BCH's own `pm####_form` name.
///
/// Returns `(family_file_no, pm_id, form, dir_name)` per family, sorted by
/// file number. `orphans_out` receives every file not covered by a family.
pub fn discover_families(
    gfx_dir: &Path,
    species: &[String],
    orphans_out: &mut Vec<PathBuf>,
) -> io::Result<Vec<(usize, usize, u16, String)>> {
    let mut numbers: Vec<usize> = fs::read_dir(gfx_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let stem = e.path().file_stem()?.to_str()?.to_string();
            stem.parse::<usize>().ok()
        })
        .collect();
    numbers.sort();
    numbers.dedup();

    let mut covered: BTreeSet<usize> = BTreeSet::new();
    let mut fallbacks: Vec<(usize, usize, u16)> = Vec::new();
    for &file_no in &numbers {
        if file_no % FAMILY_SIZE == MODEL_SLOT {
            let family = file_no / FAMILY_SIZE;
            for s in family * FAMILY_SIZE..(family + 1) * FAMILY_SIZE {
                covered.insert(s);
            }
            let (pm, form) = read_pm_identity(&gfx_dir.join(format!("{file_no:04}.bin")))
                .unwrap_or_else(|| (file_no / FAMILY_SIZE + 1, 0));
            fallbacks.push((file_no, pm, form));
        }
    }
    orphans_out.extend(
        numbers
            .iter()
            .filter(|n| !covered.contains(n))
            .map(|n| gfx_dir.join(format!("{n:04}.bin"))),
    );

    // Deterministic unique directory names.
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut out = Vec::new();
    for (file_no, pm, form) in fallbacks {
        let label = if pm <= 649 {
            species
                .get(pm)
                .map(|s| safe_name(s))
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| format!("pm{pm:04}"))
        } else {
            format!("pm{pm:04}")
        };
        let mut dir = format!("{pm:04}_{label}_{form:02}");
        let mut n = 2;
        while !used.insert(dir.clone()) {
            dir = format!("{pm:04}_{label}_{form:02}_{n}");
            n += 1;
        }
        out.push((file_no, pm, form, dir));
    }
    Ok(out)
}

/// Read `(pm_id, form)` from a model PC's hi-poly BCH name. `None` when the
/// file or its model does not parse.
fn read_pm_identity(path: &Path) -> Option<(usize, u16)> {
    let bytes = fs::read(path).ok()?;
    let container = pkmn::PkmnContainer::parse(&bytes).ok()?;
    let entry = container.entries.first()?;
    if entry.len() < 4 || entry[0..4] != *b"BCH\0" {
        return None;
    }
    let bch = bch::Bch::parse(entry).ok()?;
    let name = bch.models.first()?.name.clone();
    parse_pm_name(&name)
}

/// Convert all discovered families: copy the 8 source files, export models as
/// OBJ+MTL, decode textures to PNG. `orphans` (files not in any family) are
/// copied to `orphan_dir` so nothing is lost. `on_family` reports progress
/// `(done, total, name)` per family; `cancel` aborts between families.
pub fn convert_families(
    gfx_dir: &Path,
    out_pokemon: &Path,
    orphan_dir: &Path,
    families: &[(usize, usize, u16, String)],
    orphans: &[PathBuf],
    on_family: &dyn Fn(usize, usize, &str),
    cancel: Option<&AtomicBool>,
) -> io::Result<PokemonStats> {
    let mut stats = PokemonStats::default();
    let families_dir = out_pokemon.join("families");
    let models_dir = out_pokemon.join("models");
    let textures_dir = out_pokemon.join("textures");
    fs::create_dir_all(&families_dir)?;
    fs::create_dir_all(&models_dir)?;
    fs::create_dir_all(&textures_dir)?;

    let total = families.len();
    for (fi, (file_no, _pm, _form, dir)) in families.iter().enumerate() {
        if cancel.is_some_and(|c| c.load(Ordering::Relaxed)) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "curate cancelled",
            ));
        }
        on_family(fi, total, dir);
        let fam_dir = families_dir.join(dir);
        let fam_models = models_dir.join(dir);
        let fam_textures = textures_dir.join(dir);
        fs::create_dir_all(&fam_dir)?;
        fs::create_dir_all(&fam_models)?;
        fs::create_dir_all(&fam_textures)?;

        // 1. Copy the whole 8-file family (lossless source context).
        let mut copied = 0usize;
        for slot in 0..FAMILY_SIZE {
            let src = gfx_dir.join(format!(
                "{:04}.bin",
                file_no / FAMILY_SIZE * FAMILY_SIZE + slot
            ));
            if src.is_file() {
                fs::copy(&src, fam_dir.join(src.file_name().unwrap_or_default()))?;
                copied += 1;
            }
        }
        stats.family_files += copied;

        // 2. Models: every BCH entry in the model PC (0 = hi-poly, 1 = shadow).
        let model_bytes = fs::read(gfx_dir.join(format!("{file_no:04}.bin")))?;
        let mut model_count = 0usize;
        if let Ok(container) = pkmn::PkmnContainer::parse(&model_bytes) {
            for entry in &container.entries {
                if entry.len() < 4 || entry[0..4] != *b"BCH\0" {
                    continue;
                }
                let Ok(bch) = bch::Bch::parse(entry) else {
                    continue;
                };
                for model in &bch.models {
                    let written = obj::export_bch_model(model, &fam_models)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
                    model_count += written.len();
                }
            }
        }
        stats.models_obj += model_count;

        // 3. Textures: CGFX pack (+2) and the PT packs (+6, +7). The dedup
        //    set spans the whole family — identical names recur across the
        //    packs, and same-named textures with different pixels must both
        //    survive (suffix) instead of silently overwriting.
        let mut tex_count = 0usize;
        let mut used: BTreeSet<String> = BTreeSet::new();
        let mut seen_content: Vec<(String, Vec<u8>)> = Vec::new();
        for slot in TEXTURE_SLOTS {
            let src = gfx_dir.join(format!("{:04}.bin", file_no - MODEL_SLOT + slot));
            if !src.is_file() {
                continue;
            }
            let bytes = match fs::read(&src) {
                Ok(b) => b,
                Err(_) => continue,
            };
            let textures = collect_textures(&bytes);
            for tex in &textures {
                if tex.data.is_empty() || tex.width == 0 || tex.height == 0 {
                    continue;
                }
                let base = if tex.name.is_empty() {
                    "texture".to_string()
                } else {
                    tex.name.clone()
                };
                let Ok(rgba) = tex.decode_rgba() else {
                    continue;
                };
                let dims = format!("{}x{}", tex.width, tex.height);
                // True duplicates (same name, dims, and pixels across packs)
                // are written once; different pixels get a numeric suffix.
                if seen_content.iter().any(|(k, v)| k == &base && v == &rgba) {
                    continue;
                }
                seen_content.push((base.clone(), rgba.clone()));
                let mut name = base.clone();
                let mut n = 1;
                while !used.insert(format!("{name}_{dims}")) {
                    n += 1;
                    name = format!("{base}_{n}");
                }
                let out_path = fam_textures.join(format!("{name}_{dims}.png"));
                let f = fs::File::create(&out_path)?;
                if write_png(
                    &rgba,
                    tex.width as u32,
                    tex.height as u32,
                    &mut io::BufWriter::new(f),
                )
                .is_ok()
                {
                    tex_count += 1;
                }
            }
        }
        stats.textures_png += tex_count;
        stats.families += 1;
    }
    on_family(total, total, "done");

    // 4. Orphans: family-less files stay reachable under unknown/.
    if !orphans.is_empty() {
        fs::create_dir_all(orphan_dir)?;
        for src in orphans {
            fs::copy(src, orphan_dir.join(src.file_name().unwrap_or_default()))?;
            stats.orphans += 1;
        }
    }
    Ok(stats)
}

/// Collect textures from any supported payload: raw BCH, raw CGFX, or a
/// PC/PF/PT container of either (mirrors `xy gfx texture`).
fn collect_textures(data: &[u8]) -> Vec<Texture> {
    let mut textures = Vec::new();
    if data.len() >= 4 && &data[0..4] == b"BCH\0" {
        if let Ok(bch) = bch::Bch::parse(data) {
            textures.extend(bch.textures);
        }
        return textures;
    }
    if data.len() >= 4 && &data[0..4] == b"CGFX" {
        if let Ok(cgfx) = cgfx::Cgfx::parse(data) {
            for tex in &cgfx.textures {
                textures.push(Texture {
                    name: tex.name.clone(),
                    width: tex.width,
                    height: tex.height,
                    format: tex.format,
                    data: tex.data.clone(),
                });
            }
        }
        return textures;
    }
    if let Ok(container) = pkmn::PkmnContainer::parse(data) {
        for entry in &container.entries {
            if entry.len() < 4 {
                continue;
            }
            match &entry[0..4] {
                b"BCH\0" => {
                    if let Ok(bch) = bch::Bch::parse(entry) {
                        textures.extend(bch.textures);
                    }
                }
                b"CGFX" => {
                    if let Ok(cgfx) = cgfx::Cgfx::parse(entry) {
                        for tex in &cgfx.textures {
                            textures.push(Texture {
                                name: tex.name.clone(),
                                width: tex.width,
                                height: tex.height,
                                format: tex.format,
                                data: tex.data.clone(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }
    textures
}

/// Load the English species-name bank (a/0/7/4/080) for model labeling.
pub fn load_species_bank(garc_root: &Path) -> io::Result<Vec<String>> {
    let bytes = fs::read(garc_root.join("a/0/7/4/080.bin"))?;
    let tf = TextFile::parse(&bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    tf.lines()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

/// Sanitize a text-bank string into a filename-safe component.
pub fn safe_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut last_was_underscore = false;
    for c in name.chars() {
        let ok = c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '\'');
        if ok {
            out.push(c);
            last_was_underscore = false;
        } else if !last_was_underscore {
            out.push('_');
            last_was_underscore = true;
        }
    }
    out.trim_matches('_').to_string()
}

/// Split a GameFreak model name `pm####_form` into (XY-internal id, form).
/// Returns `None` for names that don't follow the convention.
fn parse_pm_name(name: &str) -> Option<(usize, u16)> {
    let rest = name.strip_prefix("pm")?;
    let (pm, form) = rest.split_once('_')?;
    let pm = pm.parse::<usize>().ok()?;
    let form = form.parse::<u16>().ok()?;
    if pm > 10_000 || form > 99 {
        return None;
    }
    Some((pm, form))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_name_sanitizes() {
        assert_eq!(safe_name("Bulbasaur"), "Bulbasaur");
        assert_eq!(safe_name("Nidoran♀"), "Nidoran");
        assert_eq!(safe_name("Farfetch'd"), "Farfetch'd");
        assert_eq!(safe_name("Ho-Oh"), "Ho-Oh");
        assert_eq!(safe_name("???"), "");
    }

    #[test]
    fn pm_names_parse() {
        assert!(parse_pm_name("pm0001_00").is_some_and(|(p, f)| p == 1 && f == 0));
        assert!(parse_pm_name("pm0006_51").is_some_and(|(p, f)| p == 6 && f == 51));
        assert!(parse_pm_name("es001_ca").is_none());
        assert!(parse_pm_name("pm99999_00").is_none());
    }

    #[test]
    fn texture_slots_match_family_layout() {
        // +4 model PC; textures come from +2 (CGFX) and +6/+7 (PT).
        assert_eq!(MODEL_SLOT, 4);
        assert!(TEXTURE_SLOTS.iter().all(|s| *s != MODEL_SLOT));
        assert!(TEXTURE_SLOTS.iter().all(|s| *s < FAMILY_SIZE));
    }
}
