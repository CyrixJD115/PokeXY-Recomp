//! `xy-battle` — gameplay data extraction for Gen 6 Pokémon titles.
//! **Planned; intentionally empty.**
//!
//! Targets once GARC payloads are extractable (`xy-archive`, PLAN.md Phase 1):
//! * Pokémon base stats, learnsets, evolutions, form variations
//! * overworld wild encounter tables, trainer teams, battle AI matrices
//! * item definitions, move effects, type matchup matrices
//!
//! These are data tables *inside* GARC containers — nothing to parse or
//! validate until GARC extraction is byte-parity-verified. Output will be
//! structured (JSON/RON) via the CLI, with layouts documented per module
//! before any struct is written from memory.
