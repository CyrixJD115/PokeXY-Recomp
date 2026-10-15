//! xy-heatmap — standalone progress-heatmap studio for the Pokémon X/Y
//! recomp + decomp project. One Scene feeds the web UI AND both exporters.

pub mod cli;
pub mod collect;
pub mod config;
pub mod dataset;
pub mod error;
pub mod fmt;
pub mod layout;
pub mod palette;
pub mod png;
pub mod scene;
pub mod server;
pub mod svg;
pub mod views;
