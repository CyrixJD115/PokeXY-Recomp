<script lang="ts">
  import { onMount } from "svelte";
  import {
    BASE_TABS,
    TIER_COLORS,
    TIER_NAMES,
    j,
    pct,
    type DatasetPayload,
    type Hover,
    type ModuleFacts,
    type Scene,
    type ViewTab,
  } from "./lib/api";
  import SceneView from "./SceneView.svelte";
  import Sidebar from "./Sidebar.svelte";
  import Facts from "./Facts.svelte";

  let ds = $state<DatasetPayload | null>(null);
  let tab = $state<ViewTab>("treemap");
  let module = $state<string | null>(null);
  let minTier = $state(0);
  let cols = $state(88);
  let scene = $state<Scene | null>(null);
  let facts = $state<ModuleFacts | null>(null);
  let viewError = $state("");
  let hover = $state<Hover | null>(null);
  let watchStale = $state(false);
  let loadError = $state("");
  let mainEl: HTMLElement | undefined = $state();

  const tierFilters = [
    { value: 0, label: "all" },
    { value: 1, label: "decompiled+" },
    { value: 2, label: "named+" },
    { value: 3, label: "verified" },
  ];

  // global aggregates for the header chips + footer legend
  const totals = $derived.by(() => {
    if (!ds) return null;
    const counts = [0, 0, 0, 0];
    let total = 0;
    let covered = 0;
    let hint = 0;
    let attempted = 0;
    let relinkOk = 0;
    let relinkKnown = 0;
    for (const m of ds.modules) {
      total += m.cells.length;
      if (m.relink_ok != null) {
        relinkKnown++;
        if (m.relink_ok) relinkOk++;
      }
      for (const c of m.cells) {
        counts[c[3]]++;
        if (c[4] & 1) covered++;
        if (c[4] & 2) hint++;
        if (c[4] & 4) attempted++;
      }
    }
    return { counts, total, covered, hint, attempted, relinkOk, relinkKnown };
  });

  function baseName(): string {
    return tab === "module" ? "module-" + encodeURIComponent(module ?? "") : tab;
  }

  function params(): string {
    const p = new URLSearchParams();
    if (minTier) p.set("min", String(minTier));
    if (tab === "grid") p.set("cols", String(cols));
    if (tab === "module") p.set("bare", "1"); // the facts card replaces in-scene chrome
    return p.toString();
  }

  async function render(): Promise<void> {
    const qs = params();
    try {
      scene = await j<Scene>(`/api/view/${baseName()}${qs ? "?" + qs : ""}`);
      viewError = "";
    } catch (e) {
      scene = null;
      viewError = String((e as Error).message ?? e);
    }
  }

  async function showFacts(): Promise<void> {
    if (tab !== "module" || !module) {
      facts = null;
      return;
    }
    try {
      facts = await j<ModuleFacts>(`/api/module/${encodeURIComponent(module)}`);
    } catch {
      facts = null;
    }
  }

  function setHash(): void {
    location.hash = tab === "module" && module ? "module-" + module : tab;
  }

  async function navigate(): Promise<void> {
    setHash();
    await showFacts();
    await render();
    mainEl?.scrollTo({ top: 0 });
  }

  async function setTab(t: ViewTab): Promise<void> {
    tab = t;
    if (t !== "module") module = null;
    await navigate();
  }

  async function openModule(name: string): Promise<void> {
    module = name;
    tab = "module";
    await navigate();
  }

  async function refresh(): Promise<void> {
    try {
      ds = await j<DatasetPayload>("/api/dataset");
      watchStale = false;
      await showFacts();
      await render();
    } catch (e) {
      loadError = String((e as Error).message ?? e);
    }
  }

  async function poll(): Promise<void> {
    try {
      const f = await j<{ fingerprint: string }>("/api/fingerprint");
      if (ds && f.fingerprint !== ds.fingerprint) {
        watchStale = true;
        await refresh();
      }
    } catch {
      /* server restarting */
    }
  }

  onMount(async () => {
    try {
      ds = await j<DatasetPayload>("/api/dataset");
    } catch (e) {
      loadError = String((e as Error).message ?? e);
      return;
    }
    // deep link: #treemap / #overview / #grid / #namespaces / #module-<Name>
    const h = decodeURIComponent(location.hash.slice(1));
    if ((BASE_TABS as readonly string[]).includes(h)) tab = h as ViewTab;
    else if (h.startsWith("module-")) {
      tab = "module";
      module = h.slice(7);
    }
    await showFacts();
    await render();
    if (ds.watch) setInterval(poll, 3000);
  });

  window.addEventListener("hashchange", () => {
    const h = decodeURIComponent(location.hash.slice(1));
    if (h.startsWith("module-")) openModule(h.slice(7));
    else if ((BASE_TABS as readonly string[]).includes(h)) setTab(h as ViewTab);
  });
</script>

<header>
  <div class="brand">
    <h1><b>xy</b>-heatmap</h1>
    {#if ds}<span class="title">{ds.title}</span>{/if}
  </div>
  {#if totals}
    <div class="chips" role="status">
      <span class="chip"><b>{ds!.modules.length}</b> modules</span>
      <span class="chip"><b>{totals.total.toLocaleString()}</b> functions</span>
      <span class="chip"><b>{pct(totals.covered, totals.total)}</b> covered</span>
      <span class="chip"><b>{pct(totals.counts[1], totals.total)}</b> decompiled</span>
      <span class="chip"><b>{pct(totals.counts[2], totals.total)}</b> named</span>
      <span class="chip"><b>{pct(totals.counts[3], totals.total)}</b> verified</span>
      {#if totals.relinkKnown}
        <span class="chip"><b>{totals.relinkOk}/{totals.relinkKnown}</b> relink</span>
      {/if}
    </div>
  {:else}
    <span class="loading">loading…</span>
  {/if}
  {#if ds?.watch}
    <span
      class="watch"
      class:live={!watchStale}
      class:stale={watchStale}
      title={watchStale ? "sources changed — reloading" : "live reload active"}
    >{watchStale ? "reloading" : "live"}</span>
  {/if}
</header>

<div id="toolbar">
  <nav class="tabs" aria-label="views">
    {#each BASE_TABS as t}
      <button class:active={tab === t} onclick={() => setTab(t)}>{t[0].toUpperCase() + t.slice(1)}</button>
    {/each}
  </nav>
  <span id="crumb">
    {#if tab === "module"}
      <button class="link" onclick={() => setTab("treemap")}>← modules</button>
      <span class="sep">/</span>
      <b>{module}</b>
    {/if}
  </span>
  <span class="spacer"></span>
  <div class="seg" role="group" aria-label="tier filter">
    {#each tierFilters as f}
      <button class:active={minTier === f.value} onclick={() => { minTier = f.value; render(); }}
      >{f.label}</button>
    {/each}
  </div>
  {#if tab === "grid"}
    <label class="opt">density
      <select bind:value={cols} onchange={render}>
        <option value={48}>coarse</option>
        <option value={88}>normal</option>
        <option value={128}>fine</option>
      </select>
    </label>
  {/if}
  <div class="exports">
    <a class="btn" href={`/view/${baseName()}.svg${params() ? "?" + params() : ""}`}
       download={`${baseName()}.svg`} title="download as SVG">SVG</a>
    <a class="btn" href={`/view/${baseName()}.png?scale=3${params() ? "&" + params() : ""}`}
       download={`${baseName()}.png`} title="download as PNG">PNG</a>
  </div>
</div>

<div id="body">
  {#if ds}
    <Sidebar modules={ds.modules} selected={tab === "module" ? module : null} onopen={openModule} />
  {:else}
    <aside class="side-loading"></aside>
  {/if}
  <main id="main" bind:this={mainEl}>
    {#if loadError}
      <div class="state error">
        <h2>could not load the dataset</h2>
        <pre>{loadError}</pre>
      </div>
    {:else if !ds}
      <div class="state"><p>loading dataset…</p></div>
    {:else}
      {#if facts}
        <Facts {facts} onopen={openModule} />
      {/if}
      {#if viewError}
        <div class="state error">
          <h2>view failed</h2>
          <pre>{viewError}</pre>
        </div>
      {:else if scene}
        <SceneView {scene} {ds} {minTier} onhover={(h) => (hover = h)} onopenmodule={openModule} />
      {/if}
    {/if}
  </main>
</div>

<div id="readout">
  {#if hover?.kind === "cell"}
    <span class="ro-module">{hover.module}</span>
    <span class="ro-sep">·</span>
    <span class="ro-addr">0x{hover.addr.toString(16).toUpperCase().padStart(6, "0")}</span>
    <span class="ro-sep">·</span>
    <span class="ro-tier" style="color:{TIER_COLORS[hover.tier]}">{TIER_NAMES[hover.tier]}</span>
    {#if hover.tier === 3}
      <span class="ro-flag" title="armcc masked-match proven — recompiles byte-identical to the retail binary">armcc</span>
    {/if}
    {#if hover.flags & 2}
      <span class="ro-flag ro-flag-relaxed" title="matches only with --relax-nops — not counted as verified">relaxed</span>
    {/if}
    {#if hover.flags & 4}
      <span class="ro-flag ro-flag-attempted" title="an armcc-matching agent attempted this without reaching a match">attempted</span>
    {/if}
    {#if hover.name}
      <span class="ro-sep">·</span>
      <span class="ro-name">{hover.name}</span>
    {/if}
  {:else if hover?.kind === "module"}
    <span class="ro-module">{hover.module}</span>
    <span class="ro-sep">·</span>
    <span>{hover.count.toLocaleString()} functions</span>
    <span class="ro-sep">·</span>
    <span class="ro-hint">click for module detail</span>
  {:else}
    <span class="ro-hint">point at any cell — its module, address and status show up here</span>
  {/if}
</div>

<footer>
  <span class="legend">
    {#each TIER_NAMES as t, i}
      <span class="sw" title="{t} — color = highest stage reached{t === 'verified' ? ' (armcc masked-match proven)' : ''}">
        <i style="background:{TIER_COLORS[i]}"></i>{t}
        {#if totals}<em>{totals.counts[i].toLocaleString()}</em>{/if}
      </span>
    {/each}
    {#if totals?.hint}
      <span class="sw" title="armcc match only with --relax-nops — excluded from verified">
        <i class="ring ring-relaxed"></i>relaxed<em>{totals.hint.toLocaleString()}</em>
      </span>
    {/if}
    {#if totals?.attempted}
      <span class="sw" title="armcc-matching agents attempted these without a match — don't blindly retry">
        <i class="ring ring-attempted"></i>attempted<em>{totals.attempted.toLocaleString()}</em>
      </span>
    {/if}
  </span>
  {#if ds?.warnings.length}
    <span id="warn" title={ds.warnings.join("\n")}>{ds.warnings.length} collector warnings</span>
  {/if}
  <span class="hint">{ds?.sources ?? ""}</span>
</footer>

<style>
  /* ------------------------------------------------------------ header */
  header {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 9px 16px;
    background: var(--panel);
    border-bottom: 1px solid var(--line);
    flex-wrap: wrap;
  }
  .brand {
    display: flex;
    align-items: baseline;
    gap: 10px;
  }
  header h1 {
    font-size: 16px;
    margin: 0;
    letter-spacing: 0.3px;
    white-space: nowrap;
  }
  header h1 b {
    color: var(--accent);
  }
  .title {
    color: var(--faint);
    font-size: 12px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 300px;
  }
  .chips {
    display: flex;
    gap: 6px;
    flex-wrap: wrap;
    margin-left: auto;
  }
  .chip {
    font-size: 11.5px;
    color: var(--muted);
    background: var(--panel2);
    border: 1px solid var(--line);
    border-radius: 999px;
    padding: 2px 10px;
    white-space: nowrap;
  }
  .chip b {
    color: var(--text);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }
  .loading {
    color: var(--faint);
    margin-left: auto;
  }
  .watch {
    font-size: 11px;
    letter-spacing: 0.05em;
    padding: 2px 10px 2px 8px;
    border-radius: 999px;
    border: 1px solid var(--line);
    color: var(--faint);
    white-space: nowrap;
  }
  .watch::before {
    content: "";
    display: inline-block;
    width: 7px;
    height: 7px;
    border-radius: 50%;
    margin-right: 6px;
    background: var(--faint);
    vertical-align: 1px;
  }
  .watch.live {
    color: var(--accent);
    border-color: var(--accent);
  }
  .watch.live::before {
    background: var(--accent);
  }
  .watch.stale {
    color: var(--gold);
    border-color: var(--gold);
  }
  .watch.stale::before {
    background: var(--gold);
  }

  /* ----------------------------------------------------------- toolbar */
  #toolbar {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 16px;
    background: var(--panel);
    border-bottom: 1px solid var(--line);
    flex-wrap: wrap;
  }
  .tabs {
    display: flex;
    gap: 2px;
    background: var(--panel2);
    border: 1px solid var(--line);
    border-radius: var(--radius-sm);
    padding: 2px;
  }
  .tabs button {
    background: transparent;
    color: var(--muted);
    border: none;
    border-radius: 5px;
    padding: 4px 12px;
    font-size: 13px;
  }
  .tabs button.active {
    color: var(--text);
    background: var(--accent-soft);
    box-shadow: inset 0 0 0 1px var(--accent);
  }
  #crumb {
    color: var(--muted);
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 7px;
    min-width: 0;
  }
  #crumb .viewname {
    color: var(--faint);
    letter-spacing: 0.04em;
  }
  #crumb b {
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  button.link {
    background: none;
    border: none;
    color: var(--accent);
    padding: 0;
    font-size: 13px;
  }
  button.link:hover {
    text-decoration: underline;
  }
  .sep {
    color: var(--faint);
  }
  .spacer {
    flex: 1;
  }
  .seg {
    display: flex;
    gap: 2px;
    background: var(--panel2);
    border: 1px solid var(--line);
    border-radius: var(--radius-sm);
    padding: 2px;
  }
  .seg button {
    background: transparent;
    color: var(--muted);
    border: none;
    border-radius: 5px;
    padding: 3px 10px;
    font-size: 12px;
  }
  .seg button.active {
    color: var(--text);
    background: var(--panel3);
    box-shadow: inset 0 0 0 1px var(--border);
  }
  .opt {
    color: var(--muted);
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .exports {
    display: flex;
    gap: 4px;
  }
  .btn {
    color: var(--text);
    text-decoration: none;
    border: 1px solid var(--line);
    background: var(--panel2);
    border-radius: var(--radius-sm);
    padding: 4px 11px;
    font-size: 12.5px;
  }
  .btn:hover {
    border-color: var(--accent);
    color: var(--accent);
  }

  /* --------------------------------------------------------------- body */
  #body {
    flex: 1;
    display: flex;
    min-height: 0;
  }
  .side-loading {
    width: 272px;
    border-right: 1px solid var(--line);
    background: var(--panel);
  }
  #main {
    flex: 1;
    overflow: auto;
    padding: 16px 20px 28px;
    min-width: 0;
  }
  .state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 200px;
    color: var(--faint);
  }
  .state.error {
    align-items: flex-start;
    color: var(--red);
  }
  .state.error h2 {
    font-size: 15px;
    margin: 0 0 8px;
  }
  .state pre {
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: var(--radius-sm);
    padding: 12px 14px;
    overflow: auto;
    max-width: 100%;
    white-space: pre-wrap;
  }

  /* ------------------------------------------------------------ readout */
  #readout {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 16px;
    background: var(--panel);
    border-top: 1px solid var(--line);
    color: var(--muted);
    font-size: 12.5px;
    min-height: 29px;
    white-space: nowrap;
    overflow: hidden;
  }
  .ro-module {
    color: var(--text);
    font-weight: 600;
  }
  .ro-addr {
    font-family: var(--mono);
    color: var(--text);
  }
  .ro-tier {
    font-weight: 600;
  }
  .ro-name {
    color: var(--text);
    font-family: var(--mono);
    font-size: 12px;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .ro-hint {
    color: var(--faint);
  }
  .ro-sep {
    color: var(--faint);
  }

  /* ------------------------------------------------------------- footer */
  footer {
    display: flex;
    align-items: center;
    gap: 18px;
    padding: 5px 16px 7px;
    border-top: 1px solid var(--line);
    color: var(--muted);
    font-size: 11.5px;
    background: var(--panel);
    flex-wrap: wrap;
  }
  .legend {
    display: flex;
    align-items: center;
    gap: 14px;
    flex-wrap: wrap;
  }
  .sw {
    display: inline-flex;
    align-items: center;
    gap: 5px;
  }
  .sw i {
    width: 10px;
    height: 10px;
    border-radius: 2px;
    display: inline-block;
  }
  .sw i.ring {
    background: var(--panel2);
    border: 1.4px solid var(--hint);
  }
  .sw i.ring-attempted {
    border-color: #8f7ff0;
  }
  .ro-flag {
    font-size: 10.5px;
    font-weight: 600;
    letter-spacing: 0.03em;
    border: 1px solid var(--line);
    border-radius: 999px;
    padding: 0 7px;
    color: var(--gold);
  }
  .ro-flag-relaxed {
    color: var(--hint);
  }
  .ro-flag-attempted {
    color: #8f7ff0;
  }
  .sw em {
    color: var(--faint);
    font-style: normal;
    font-variant-numeric: tabular-nums;
  }
  #warn {
    color: var(--gold);
  }
  .hint {
    margin-left: auto;
    color: var(--faint);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 40%;
  }
</style>
