//! The data model: modules of address-ordered function cells with status
//! (port of `dataset.py`).
//!
//! A [`Dataset`] is a list of [`Module`]s; a module is a list of [`Cell`]s
//! (one per function in the union inventory) sorted by address. Datasets
//! come from the project collectors ([`crate::collect`]) or from a generic,
//! documented JSON document ([`dataset_from_value`]), so the renderer is
//! testable without any project data on disk.

use serde_json::Value;

use crate::error::{HeatmapError, Result};
use crate::palette;

pub const FLAG_COVERED: u8 = 1;
/// armcc masked-match that only passes with `--relax-nops` — excluded from
/// the verified total (armcc-matching/README.md rule), shown as an overlay.
pub const FLAG_HINT: u8 = 2;
/// A function an armcc-matching agent attempted (up to its time cap) without
/// reaching a masked match — recorded in `attempted.md` so nobody retried it.
pub const FLAG_ATTEMPTED: u8 = 4;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cell {
    pub addr: u64,
    pub size: u64,
    pub name: String,
    pub tier: usize,
    pub flags: u8,
    pub namespace: String,
}

impl Cell {
    pub fn covered(&self) -> bool {
        self.flags & FLAG_COVERED != 0
    }

    pub fn hint(&self) -> bool {
        self.flags & FLAG_HINT != 0
    }

    pub fn attempted(&self) -> bool {
        self.flags & FLAG_ATTEMPTED != 0
    }

    pub fn escalate(&mut self, tier: usize) {
        if tier > self.tier {
            self.tier = tier;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub name: String,
    pub cells: Vec<Cell>,
    pub relink_ok: Option<bool>,
    pub namespace: String,
    /// modules this module imports from (dependency context)
    pub imports: Vec<String>,
}

impl Module {
    /// Tier-name bucket counts in ladder order: [raw, decompiled, named, verified].
    pub fn tier_counts(&self) -> [u64; 4] {
        let mut counts = [0u64; 4];
        for c in &self.cells {
            counts[c.tier.min(3)] += 1;
        }
        counts
    }

    pub fn covered_count(&self) -> u64 {
        self.cells.iter().filter(|c| c.covered()).count() as u64
    }

    pub fn hint_count(&self) -> u64 {
        self.cells.iter().filter(|c| c.hint()).count() as u64
    }

    pub fn attempted_count(&self) -> u64 {
        self.cells.iter().filter(|c| c.attempted()).count() as u64
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dataset {
    pub title: String,
    pub modules: Vec<Module>,
    pub fingerprint: String,
    pub warnings: Vec<String>,
}

impl Dataset {
    /// Exact-name lookup, falling back to case-insensitive (entries.c casing
    /// differs from disk in places).
    pub fn module(&self, name: &str) -> Result<&Module> {
        if let Some(m) = self.modules.iter().find(|m| m.name == name) {
            return Ok(m);
        }
        let low = name.to_lowercase();
        if let Some(m) = self.modules.iter().find(|m| m.name.to_lowercase() == low) {
            return Ok(m);
        }
        Err(HeatmapError::new(format!(
            "unknown module {name:?}; known modules: {}",
            self.modules
                .iter()
                .map(|m| m.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )))
    }

    pub fn total_cells(&self) -> u64 {
        self.modules.iter().map(|m| m.cells.len() as u64).sum()
    }

    pub fn tier_counts(&self) -> [u64; 4] {
        let mut counts = [0u64; 4];
        for m in &self.modules {
            for (i, v) in m.tier_counts().iter().enumerate() {
                counts[i] += v;
            }
        }
        counts
    }

    pub fn covered_count(&self) -> u64 {
        self.modules.iter().map(|m| m.covered_count()).sum()
    }

    pub fn hint_count(&self) -> u64 {
        self.modules.iter().map(|m| m.hint_count()).sum()
    }

    pub fn attempted_count(&self) -> u64 {
        self.modules.iter().map(|m| m.attempted_count()).sum()
    }

    /// (ok, total) over modules that carry relink data, else None.
    pub fn relink_summary(&self) -> Option<(u64, u64)> {
        let known: Vec<_> = self
            .modules
            .iter()
            .filter(|m| m.relink_ok.is_some())
            .collect();
        if known.is_empty() {
            return None;
        }
        let ok = known.iter().filter(|m| m.relink_ok == Some(true)).count() as u64;
        Some((ok, known.len() as u64))
    }

    /// Per-namespace tier counts + which modules the symbols live in. Cells
    /// without any namespace are bucketed under "(unnamed)". `min_tier`
    /// filters which cells are aggregated.
    pub fn namespace_details(&self, min_tier: usize) -> Vec<(String, NamespaceDetail)> {
        use std::collections::BTreeMap;
        let mut details: BTreeMap<String, NamespaceDetail> = BTreeMap::new();
        for m in &self.modules {
            for c in &m.cells {
                if c.tier < min_tier {
                    continue;
                }
                let ns = if c.namespace.is_empty() {
                    "(unnamed)"
                } else {
                    &c.namespace
                };
                let d = details
                    .entry(ns.to_string())
                    .or_insert_with(|| NamespaceDetail {
                        counts: [0; 4],
                        modules: BTreeMap::new(),
                    });
                d.counts[c.tier.min(3)] += 1;
                *d.modules.entry(m.name.clone()).or_insert(0) += 1;
            }
        }
        details.into_iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct NamespaceDetail {
    pub counts: [u64; 4],
    /// module name -> cell count
    pub modules: std::collections::BTreeMap<String, u64>,
}

impl NamespaceDetail {
    pub fn total(&self) -> u64 {
        self.counts.iter().sum()
    }
}

// --------------------------------------------------------------- JSON input

fn parse_addr(v: &Value, where_: &str) -> Result<u64> {
    match v {
        Value::Bool(_) => Err(HeatmapError::new(format!(
            "{where_}: addr must be an integer, got boolean"
        ))),
        Value::Number(n) => n
            .as_u64()
            .or_else(|| {
                n.as_i64()
                    .and_then(|i| if i >= 0 { Some(i as u64) } else { None })
            })
            .ok_or_else(|| HeatmapError::new(format!("{where_}: addr must be non-negative"))),
        Value::String(s) => {
            let low = s.to_lowercase();
            let parsed = if let Some(hex) = low.strip_prefix("0x") {
                u64::from_str_radix(hex, 16)
            } else {
                s.parse::<u64>()
            };
            parsed.map_err(|_| {
                HeatmapError::new(format!("{where_}: cannot parse addr {s:?} as an integer"))
            })
        }
        other => Err(HeatmapError::new(format!(
            "{where_}: addr must be an integer or hex string, got json type {:?}",
            json_type_name(other)
        ))),
    }
}

fn json_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn get_bool(spec: &serde_json::Map<String, Value>, key: &str, where_: &str) -> Result<bool> {
    match spec.get(key) {
        None | Some(Value::Null) => Ok(false),
        Some(Value::Bool(b)) => Ok(*b),
        Some(other) => Err(HeatmapError::new(format!(
            "{where_}: {key} must be a boolean, got {:?}",
            json_type_name(other)
        ))),
    }
}

fn escalate_from_flags(
    cell: &mut Cell,
    spec: &serde_json::Map<String, Value>,
    where_: &str,
) -> Result<()> {
    match spec.get("tier") {
        None | Some(Value::Null) => {}
        Some(Value::String(s)) => match palette::tier_index(s) {
            Some(idx) => cell.escalate(idx),
            None => {
                return Err(HeatmapError::new(format!(
                    "{where_}: tier must be one of {}, got {s:?}",
                    palette::TIER_LADDER.join(", ")
                )))
            }
        },
        Some(other) => {
            return Err(HeatmapError::new(format!(
                "{where_}: tier must be a string, got {:?}",
                json_type_name(other)
            )))
        }
    }
    for (name, idx) in [("decompiled", 1usize), ("named", 2), ("verified", 3)] {
        if get_bool(spec, name, where_)? {
            cell.escalate(idx);
        }
    }
    if get_bool(spec, "attempted", where_)? {
        cell.flags |= FLAG_ATTEMPTED;
    }
    Ok(())
}

/// Validate a generic dataset document (port of `dataset_from_dict`); raises
/// on any problem with a precise, located message.
pub fn dataset_from_value(obj: &Value, source: &str) -> Result<Dataset> {
    let obj = obj
        .as_object()
        .ok_or_else(|| HeatmapError::new(format!("{source}: top level must be a JSON object")))?;
    let title = match obj.get("title") {
        None => "heatmap".to_string(),
        Some(Value::String(s)) => s.clone(),
        Some(_) => {
            return Err(HeatmapError::new(format!(
                "{source}: title must be a string"
            )))
        }
    };
    let raw_modules = obj
        .get("modules")
        .and_then(|m| m.as_array())
        .filter(|m| !m.is_empty())
        .ok_or_else(|| {
            HeatmapError::new(format!("{source}: 'modules' must be a non-empty list"))
        })?;

    let mut modules = Vec::new();
    for (mi, raw_m) in raw_modules.iter().enumerate() {
        let where_m = format!("{source}: module[{mi}]");
        let raw_m = raw_m
            .as_object()
            .ok_or_else(|| HeatmapError::new(format!("{where_m}: must be an object")))?;
        let name = match raw_m.get("name") {
            Some(Value::String(s)) if !s.trim().is_empty() => s.clone(),
            _ => {
                return Err(HeatmapError::new(format!(
                    "{where_m}: 'name' must be a non-empty string"
                )))
            }
        };
        let relink_ok = raw_m.get("relink_ok").and_then(|v| v.as_bool());
        let namespace = raw_m
            .get("namespace")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let mut module = Module {
            name: name.clone(),
            relink_ok,
            namespace,
            ..Default::default()
        };

        let empty_cells = Value::Array(Vec::new());
        let raw_cells = match raw_m.get("cells") {
            None => &empty_cells,
            Some(v) if v.is_array() => v,
            Some(_) => {
                return Err(HeatmapError::new(format!(
                    "{where_m}: 'cells' must be a list"
                )))
            }
        };
        let mut seen = std::collections::HashSet::new();
        for (ci, raw_c) in raw_cells.as_array().unwrap().iter().enumerate() {
            let where_ = format!("{where_m} cell[{ci}]");
            let raw_c = raw_c
                .as_object()
                .ok_or_else(|| HeatmapError::new(format!("{where_}: must be an object")))?;
            let addr = parse_addr(raw_c.get("addr").unwrap_or(&Value::Null), &where_)?;
            if !seen.insert(addr) {
                return Err(HeatmapError::new(format!(
                    "{where_}: duplicate address 0x{addr:X} in module {name:?}"
                )));
            }
            let size = match raw_c.get("size") {
                None | Some(Value::Null) => 0,
                Some(v) => parse_addr(v, &format!("{where_} size"))?,
            };
            let name_s = match raw_c.get("name") {
                None | Some(Value::Null) => String::new(),
                Some(Value::String(s)) => s.clone(),
                Some(_) => {
                    return Err(HeatmapError::new(format!(
                        "{where_}: name must be a string"
                    )))
                }
            };
            let ns = match raw_c.get("namespace") {
                None | Some(Value::Null) => String::new(),
                Some(Value::String(s)) => s.clone(),
                Some(_) => {
                    return Err(HeatmapError::new(format!(
                        "{where_}: namespace must be a string"
                    )))
                }
            };
            let mut flags = 0u8;
            if get_bool(raw_c, "covered", &where_)? {
                flags |= FLAG_COVERED;
            }
            if get_bool(raw_c, "hint", &where_)? {
                flags |= FLAG_HINT;
            }
            let mut cell = Cell {
                addr,
                size,
                name: name_s,
                tier: 0,
                flags,
                namespace: ns,
            };
            escalate_from_flags(&mut cell, raw_c, &where_)?;
            module.cells.push(cell);
        }
        module.cells.sort_by_key(|c| c.addr);
        modules.push(module);
    }
    Ok(Dataset {
        title,
        modules,
        ..Default::default()
    })
}

pub fn load_dataset(path: &std::path::Path) -> Result<Dataset> {
    let data = std::fs::read(path)
        .map_err(|e| HeatmapError::new(format!("cannot read dataset {}: {e}", path.display())))?;
    let obj: Value = serde_json::from_slice(&data).map_err(|e| {
        HeatmapError::new(format!("dataset {} is not valid JSON: {e}", path.display()))
    })?;
    dataset_from_value(&obj, &path.display().to_string())
}

/// Compact JSON-safe projection for the web UI payload: cells as
/// `[addr, size, name, tier, flags, namespace]` tuples — the encoding the
/// client-side hover readout indexes into.
pub fn dataset_to_api(ds: &Dataset) -> Value {
    use serde_json::json;
    let modules: Vec<Value> = ds
        .modules
        .iter()
        .map(|m| {
            let cells: Vec<Value> = m
                .cells
                .iter()
                .map(|c| json!([c.addr, c.size, c.name, c.tier, c.flags, c.namespace]))
                .collect();
            json!({
                "name": m.name,
                "relink_ok": m.relink_ok,
                "imports": m.imports,
                "cells": cells,
            })
        })
        .collect();
    json!({"title": ds.title, "fingerprint": ds.fingerprint, "modules": modules})
}
