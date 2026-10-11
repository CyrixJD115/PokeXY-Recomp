//! `xy-script` — script and text disassembly for Gen 6 Pokémon titles.
//!
//! * [`textfile`] — GameFreak Gen 6 text banks (gametext GARCs `a/0/7/*`,
//!   storytext `a/0/8/*`), ported from pk3DS `TextFile.cs`: per-line XOR
//!   obfuscation (`0x7C89 + 0x2983·i`, ROL-3 per unit), `0x0010` variable
//!   records with the XY variable-code table, formatting controls, and the
//!   `E07F/E08D/E08E/E08F` → Unicode character remap.
//!
//! Event-script bytecode disassembly is still planned (blocked on mapping
//! the opcodes — recon from the zone GARCs first, per house rules).
pub mod textfile;
