//! `xy-gfx` — graphics and asset parsing.
//!
//! * [`smdh`] — home-menu icon + title metadata (ExeFS `icon.bin`),
//!   RGB565 → RGB888/PNG (pixel-identical to the PIL reference)
//! * [`texture`] — PICA200 texture decoder (all 14 formats, 8×8 Morton
//!   tiling, ETC1/ETC1A4) + the PNG writer
//! * [`bcres`] — BCRES (`CGFX`) container inspection: header validation,
//!   sub-chunk census, DICT name harvesting
//! * [`cgfx`] — full CGFX decode: textures (TXOB), materials (MTOB),
//!   skeletons, skeletal animations (Ohana3DS `CGFX.cs` port; XY's
//!   rev-9 CNOD-linked mesh variant is skipped via `geometry_partial`)
//! * [`bch`] — BCH (CtrH3D) decode: container + relocation + models,
//!   meshes (PICA command buffers), materials, bones, textures (SPICA
//!   port; validated against Bulbasaur `pm0001_00`)
//! * [`pkmn`] — GameFreak PC/PF/PT sub-file containers
//! * [`obj`] — Wavefront OBJ (+MTL for BCH) export
//!
//! Planned (PLAN.md Phase 2): BCH skeletal animations, the rev-9 CNOD
//! mesh variant, `bclim`, `.bclyt`, glTF export.
//!
//! This is the only crate that links `png` — the shared `xy-core` error type
//! stays codec-free, so conversion errors wrap both core and png errors.
pub mod bch;
pub mod bcres;
pub mod cgfx;
pub mod error;
pub mod gf_model;
pub mod gf_motion;
pub mod gltf;
pub mod obj;
pub mod pkmn;
pub mod smdh;
pub mod texture;

pub use error::{Error, Result};
