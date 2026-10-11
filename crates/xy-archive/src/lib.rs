//! `xy-archive` — archive and container handling for Gen 6 Pokémon titles.
//!
//! * [`romfs`] — RomFS extraction, directly from a CCI/`.3ds`, NCCH, or raw
//!   IVFC image (byte-identical to `ctrtool --romfsdir` on the XY dump)
//! * [`bcma`]  — the DARC-family archive used by the XY e-manual
//!   (`Manual.bcma`; 12-byte nodes — *not* the standard CTR DARC layout)
//! * [`garc`]  — Game Archive containers (GARC) under `romfs/a/`
pub mod bcma;
pub mod cro;
pub mod garc;
pub mod lzss;
pub mod romfs;
