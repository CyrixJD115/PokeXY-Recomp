<script lang="ts">
  import {
    attrsOf,
    tierGroups,
    TIER_COLORS,
    TIER_NAMES,
    type DatasetPayload,
    type Hover,
    type Scene,
    type SceneCells,
  } from "./lib/api";

  let {
    scene,
    ds,
    minTier,
    onhover,
    onopenmodule,
  }: {
    scene: Scene;
    ds: DatasetPayload;
    minTier: number;
    onhover: (hover: Hover | null) => void;
    onopenmodule: (name: string) => void;
  } = $props();

  /// One square subpath per flagged cell — mirrors the Rust svg.rs overlay
  /// rendering (relaxed matches / attempted functions).
  function flagPath(item: SceneCells, flags: boolean[]): string {
    const stride = item.size + item.gap;
    let d = "";
    flags.forEach((f, i) => {
      if (!f) return;
      const cx = item.x + (i % item.cols) * stride;
      const cy = item.y + Math.floor(i / item.cols) * stride;
      d += `M${cx} ${cy}h${item.size}v${item.size}h-${item.size}z`;
    });
    return d;
  }

  // Live "what am I pointing at" decode — locate the [data-cells] group
  // under the pointer, invert the screen transform, and index the module's
  // FILTERED cell list (the scene only drew cells at tier >= minTier, so the
  // grid index is an index into that same filtered list).
  function onpointermove(e: PointerEvent) {
    const target = e.target as Element;
    const g = target.closest("[data-cells]");
    if (g) {
      const [base, stride, cols, px, n] = g
        .getAttribute("data-cells")!
        .split(":")
        .map(Number);
      const svg = g.ownerSVGElement!;
      const ctm = svg.getScreenCTM();
      if (!ctm) return;
      const pt = new DOMPoint(e.clientX, e.clientY).matrixTransform(ctm.inverse());
      const bbox = (g as SVGGElement).getBBox();
      const col = Math.floor((pt.x - bbox.x) / px);
      const row = Math.floor((pt.y - bbox.y) / px);
      const idx = row * cols + col;
      if (col < 0 || row < 0 || idx >= n) return;
      const module = g.getAttribute("data-module")!;
      const m = ds.modules.find((x) => x.name === module);
      const cells =
        m && minTier ? m.cells.filter((c) => c[3] >= minTier) : (m?.cells ?? []);
      const cell = cells[idx] ?? null;
      onhover({
        kind: "cell",
        module,
        addr: base + idx * stride,
        tier: cell ? cell[3] : 0,
        flags: cell ? cell[4] : 0,
        name: cell ? cell[2] : "",
      });
      return;
    }
    const modEl = target.closest("[data-module]");
    if (modEl && !modEl.getAttribute("data-cells")) {
      const module = modEl.getAttribute("data-module")!;
      const m = ds.modules.find((x) => x.name === module);
      if (m) onhover({ kind: "module", module, count: m.cells.length });
    }
  }

  function onpointerleave() {
    onhover(null);
  }

  function onclick(e: MouseEvent) {
    const modEl = (e.target as Element).closest("[data-module]");
    if (modEl) onopenmodule(modEl.getAttribute("data-module")!);
  }
</script>

{#key scene}
  <!-- the scene is a pointer-driven data display; all interactive controls
       (tabs, sidebar, filters) are real buttons/inputs for keyboard use -->
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
  <div class="scene-frame" style="max-width:{scene.width}px">
    <svg
      viewBox="0 0 {scene.width} {scene.height}"
      font-family={scene.font_family}
      role="img"
      onpointermove={onpointermove}
      onpointerleave={onpointerleave}
      onclick={onclick}
    >
      <rect x="0" y="0" width={scene.width} height={scene.height} fill={scene.background} />
      {#each scene.items as item}
        {#if item.type === "rect"}
          <rect
            x={item.x} y={item.y} width={item.w} height={item.h}
            fill={item.fill}
            rx={item.rx || null}
            stroke={item.stroke && item.stroke_width > 0 ? item.stroke : null}
            stroke-width={item.stroke && item.stroke_width > 0 ? item.stroke_width : null}
            shape-rendering="crispEdges"
            {...attrsOf(item)}
          >{#if item.tip}<title>{item.tip}</title>{/if}</rect>
        {:else if item.type === "text"}
          <text
            x={item.x} y={item.y}
            font-size={item.size}
            fill={item.fill}
            text-anchor={item.anchor}
            font-weight={item.weight !== "normal" ? item.weight : null}
            font-family={item.family ?? null}
          >{#if item.tip}<title>{item.tip}</title>{/if}{item.s}</text>
        {:else if item.type === "line"}
          <line
            x1={item.x1} y1={item.y1} x2={item.x2} y2={item.y2}
            stroke={item.stroke} stroke-width={item.width}
          />
        {:else if item.type === "cells"}
          {@const groups = tierGroups(item)}
          {#each groups as g}
            <g
              data-module={item.module}
              data-cells="{item.base_addr}:{item.addr_stride}:{item.cols}:{item.size + item.gap}:{item.tiers.length}"
            >
              {#if item.tip}<title>{item.tip}</title>{/if}
              <path d={g.d} fill={TIER_COLORS[g.tier]} shape-rendering="crispEdges" />
            </g>
          {/each}
          {#each [item.hints, item.attempted] as flags, fi}
            {#if flags?.some((f) => f)}
              <!-- overlay outlines: relaxed matches (red) / attempted (violet) -->
              <g
                data-module={item.module}
                data-cells="{item.base_addr}:{item.addr_stride}:{item.cols}:{item.size + item.gap}:{item.tiers.length}"
              >
                {#if item.tip}<title>{item.tip}</title>{/if}
                <path
                  d={flagPath(item, flags)}
                  fill="none"
                  stroke={fi === 0 ? "var(--hint, #e05252)" : "#8f7ff0"}
                  stroke-width="1"
                  shape-rendering="crispEdges"
                />
              </g>
            {/if}
          {/each}
        {/if}
      {/each}
    </svg>
  </div>
{/key}

<style>
  .scene-frame {
    width: 100%;
    margin: 0 auto;
    animation: scene-in 0.18s ease-out;
  }
  svg {
    display: block;
    width: 100%;
    height: auto;
    border-radius: var(--radius-sm);
  }
  @keyframes scene-in {
    from {
      opacity: 0;
      transform: translateY(4px);
    }
  }
</style>
