// API types + helpers shared by the app shell.

export const TIER_NAMES = ["raw", "decompiled", "named", "verified"] as const;
export const TIER_COLORS = ["#454b57", "#2fa39e", "#58b368", "#eec84a"] as const;

/// Compact cell encoding: [addr, size, name, tier, flags, namespace].
export type Cell = [number, number, string, number, number, string];

export interface ModuleData {
  name: string;
  relink_ok: boolean | null;
  imports: string[];
  cells: Cell[];
}

export interface DatasetPayload {
  title: string;
  fingerprint: string;
  modules: ModuleData[];
  warnings: string[];
  watch: boolean;
  sources: string;
}

/// Cell flag bits (dataset FLAG_*): covered = recomp port, hint = relaxed
/// armcc match (excluded from verified), attempted = an armcc-matching
/// agent tried here without a match.
export const FLAG_COVERED = 1;
export const FLAG_HINT = 2;
export const FLAG_ATTEMPTED = 4;

export interface ModuleFacts {
  name: string;
  total: number;
  tiers: { raw: number; decompiled: number; named: number; verified: number };
  covered: number;
  named_symbols: number;
  relaxed: number;
  attempted: number;
  relink_ok: boolean | null;
  addr_range: [string, string] | null;
  imports: string[];
}

export type ViewTab = "treemap" | "overview" | "grid" | "namespaces" | "module";
export const BASE_TABS = ["treemap", "progress", "overview", "grid", "namespaces"] as const;

// ------------------------------------------------------------------ scene

export interface SceneRect {
  type: "rect";
  x: number; y: number; w: number; h: number;
  fill: string;
  stroke?: string | null; stroke_width?: number; rx?: number;
  tip?: string | null;
  attrs: [string, string][];
}
export interface SceneText {
  type: "text";
  x: number; y: number; s: string; size: number; fill: string;
  anchor: "start" | "middle" | "end";
  weight: string;
  family?: string | null;
  tip?: string | null;
}
export interface SceneLine {
  type: "line";
  x1: number; y1: number; x2: number; y2: number;
  stroke: string; width: number;
}
export interface SceneCells {
  type: "cells";
  x: number; y: number; size: number; cols: number;
  tiers: number[];
  gap: number; addr_stride: number; base_addr: number;
  module: string; names: string[];
  /// overlay outlines, parallel to tiers (omitted when empty)
  hints?: boolean[];
  attempted?: boolean[];
  tip?: string | null;
  attrs: [string, string][];
}
export type SceneItem = SceneRect | SceneText | SceneLine | SceneCells;

export interface Scene {
  width: number;
  height: number;
  background: string;
  font_family: string;
  items: SceneItem[];
}

/// One <path> `d` string per tier run (compact for tens of thousands of
/// cells) — mirrors the Rust svg.rs grouping.
export function tierGroups(item: SceneCells): { tier: number; d: string }[] {
  const stride = item.size + item.gap;
  const byTier = new Map<number, number[]>();
  item.tiers.forEach((t, i) => {
    const idxs = byTier.get(t);
    if (idxs) idxs.push(i);
    else byTier.set(t, [i]);
  });
  const parts: { tier: number; d: string }[] = [];
  for (const tier of [...byTier.keys()].sort((a, b) => a - b)) {
    let d = "";
    for (const i of byTier.get(tier)!) {
      const cx = item.x + (i % item.cols) * stride;
      const cy = item.y + Math.floor(i / item.cols) * stride;
      d += `M${cx} ${cy}h${item.size}v${item.size}h-${item.size}z`;
    }
    parts.push({ tier, d });
  }
  return parts;
}

export function attrsOf(item: SceneRect | SceneCells): Record<string, string> {
  return Object.fromEntries(item.attrs ?? []);
}

// ------------------------------------------------------------------ fetch

export async function j<T>(url: string): Promise<T> {
  const r = await fetch(url);
  if (!r.ok) throw new Error((await r.text()) || r.statusText);
  return r.json() as Promise<T>;
}

export function pct(n: number, d: number): string {
  return d ? `${((100.0 * n) / d).toFixed(1)}%` : "0.0%";
}

/// What the pointer is over, structured so the readout bar can render
/// chips (tier color, mono address) instead of one flat string.
export type Hover =
  | { kind: "cell"; module: string; addr: number; tier: number; flags: number; name: string }
  | { kind: "module"; module: string; count: number };
