<script lang="ts">
  import { TIER_COLORS, type DatasetPayload } from "./lib/api";

  let {
    modules,
    selected,
    onopen,
  }: {
    modules: DatasetPayload["modules"];
    selected: string | null;
    onopen: (name: string) => void;
  } = $props();

  let search = $state("");
  let sortBy = $state<"size" | "name">("size");

  const filtered = $derived(
    modules.filter((m) => !search || m.name.toLowerCase().includes(search.toLowerCase()))
  );
  const sorted = $derived(
    sortBy === "name"
      ? [...filtered].sort((a, b) => a.name.localeCompare(b.name))
      : [...filtered].sort((a, b) => b.cells.length - a.cells.length || a.name.localeCompare(b.name))
  );

  function countsOf(m: DatasetPayload["modules"][number]): number[] {
    const counts = [0, 0, 0, 0];
    for (const c of m.cells) counts[c[3]]++;
    return counts;
  }

  // "/" focuses the filter from anywhere
  function onkeydown(e: KeyboardEvent) {
    if (e.key !== "/" || e.ctrlKey || e.altKey || e.metaKey) return;
    const t = e.target as HTMLElement;
    if (t.tagName === "INPUT" || t.tagName === "SELECT" || t.tagName === "TEXTAREA") return;
    e.preventDefault();
    searchInput?.focus();
  }

  let searchInput: HTMLInputElement | undefined = $state();
</script>

<svelte:window onkeydown={onkeydown} />

<aside class="side">
  <div class="head">
    <span class="label">MODULES</span>
    <span class="count">({filtered.length})</span>
    <span class="sort" role="group" aria-label="sort order">
      <button
        class:active={sortBy === "size"}
        onclick={() => (sortBy = "size")}
        title="largest first"
      >size</button>
      <button
        class:active={sortBy === "name"}
        onclick={() => (sortBy = "name")}
        title="alphabetical"
      >A–Z</button>
    </span>
  </div>
  <!-- svelte-ignore a11y_autofocus -->
  <input
    bind:this={searchInput}
    bind:value={search}
    type="search"
    placeholder="filter modules…  ( / )"
    aria-label="filter modules"
  />
  <div class="list">
    {#each sorted as m (m.name)}
      {@const counts = countsOf(m)}
      {@const total = m.cells.length || 1}
      <button class="row" class:sel={selected === m.name} onclick={() => onopen(m.name)}>
        <span class="top">
          <span class="nm" title={m.name}>{m.name}</span>
          {#if m.relink_ok === true}
            <span class="rl ok" title="relink byte-identical">✓</span>
          {:else if m.relink_ok === false}
            <span class="rl bad" title="relink MISMATCH">✗</span>
          {/if}
          <span class="ct">{m.cells.length.toLocaleString()}</span>
        </span>
        <span class="bar" aria-hidden="true">
          {#each counts as n, t}
            {#if n > 0}
              <i style="width:{((100 * n) / total).toFixed(2)}%;background:{TIER_COLORS[t]}"></i>
            {/if}
          {/each}
        </span>
      </button>
    {:else}
      <p class="empty">no modules match</p>
    {/each}
  </div>
</aside>

<style>
  .side {
    width: 272px;
    min-width: 232px;
    border-right: 1px solid var(--line);
    background: var(--panel);
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .head {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 10px 12px 8px;
    color: var(--muted);
    font-size: 11.5px;
    letter-spacing: 0.06em;
  }
  .count {
    color: var(--faint);
  }
  .sort {
    margin-left: auto;
    display: flex;
    gap: 2px;
  }
  .sort button {
    font-size: 11px;
    padding: 2px 8px;
    color: var(--faint);
    background: transparent;
    border-color: var(--line);
  }
  .sort button.active {
    color: var(--accent);
    background: var(--accent-soft);
    border-color: var(--accent);
  }
  input {
    margin: 0 10px 8px;
    width: calc(100% - 20px);
  }
  .list {
    overflow-y: auto;
    flex: 1;
    padding: 0 6px 10px;
    min-height: 0;
  }
  .row {
    display: flex;
    flex-direction: column;
    gap: 4px;
    width: 100%;
    padding: 6px 8px;
    margin-bottom: 2px;
    background: transparent;
    border: 1px solid transparent;
    border-left: 3px solid transparent;
    text-align: left;
  }
  .row:hover {
    background: var(--panel2);
    border-color: var(--line);
  }
  .row.sel {
    background: var(--panel2);
    border-left-color: var(--accent);
  }
  .top {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
  }
  .nm {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 13px;
  }
  .rl {
    font-size: 10px;
    line-height: 1;
  }
  .rl.ok {
    color: var(--accent);
  }
  .rl.bad {
    color: var(--red);
    font-weight: 700;
  }
  .ct {
    color: var(--muted);
    font-size: 11.5px;
    font-variant-numeric: tabular-nums;
  }
  .bar {
    display: flex;
    height: 3px;
    border-radius: 2px;
    overflow: hidden;
    background: #31363f;
  }
  .bar i {
    display: block;
    height: 100%;
  }
  .empty {
    color: var(--faint);
    text-align: center;
    padding: 18px 0;
  }
</style>
