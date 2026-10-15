<script lang="ts">
  import { pct, type ModuleFacts } from "./lib/api";

  let {
    facts,
    onopen,
  }: {
    facts: ModuleFacts;
    onopen: (name: string) => void;
  } = $props();

  const relinkLabel = $derived(
    facts.relink_ok == null ? "—" : facts.relink_ok ? "byte-identical" : "MISMATCH"
  );
</script>

<section class="facts">
  <header>
    <h2>{facts.name}</h2>
    <span
      class="relink"
      class:ok={facts.relink_ok === true}
      class:bad={facts.relink_ok === false}
    >
      relink: {relinkLabel}
    </span>
  </header>

  <div class="stats">
    <div class="stat">
      <b>{facts.total.toLocaleString()}</b>
      <span>functions</span>
    </div>
    <div class="stat">
      <b>{pct(facts.covered, facts.total)}</b>
      <span>recomp-covered</span>
    </div>
    <div class="stat">
      <b>{pct(facts.tiers.decompiled, facts.total)}</b>
      <span>decompiled</span>
    </div>
    <div class="stat">
      <b>{facts.tiers.named.toLocaleString()}</b>
      <span>named ({pct(facts.tiers.named, facts.total)})</span>
    </div>
    <div class="stat">
      <b>{facts.tiers.verified.toLocaleString()}</b>
      <span>verified</span>
    </div>
    <div class="stat">
      <b class="mono">{facts.addr_range ? `${facts.addr_range[0]} – ${facts.addr_range[1]}` : "—"}</b>
      <span>address range</span>
    </div>
  </div>

  {#if facts.imports.length}
    <div class="imp">
      <span>imports from</span>
      {#each facts.imports as imp}
        <button class="chip" onclick={() => onopen(imp)} title="open {imp}">{imp}</button>
      {/each}
    </div>
  {/if}

  <p class="hint">one cell = one function, address-ordered left → right, top → bottom</p>
</section>

<style>
  .facts {
    margin: 0 0 14px;
    padding: 14px 16px;
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: var(--radius);
  }
  header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
  }
  h2 {
    margin: 0;
    font-size: 17px;
    letter-spacing: 0.02em;
  }
  .relink {
    font-size: 11.5px;
    padding: 2px 9px;
    border-radius: 999px;
    border: 1px solid var(--line);
    color: var(--muted);
  }
  .relink.ok {
    color: var(--accent);
    border-color: var(--accent);
    background: var(--accent-soft);
  }
  .relink.bad {
    color: var(--red);
    border-color: var(--red);
    background: rgba(224, 82, 82, 0.12);
  }
  .stats {
    display: flex;
    flex-wrap: wrap;
    gap: 8px 0;
  }
  .stat {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 118px;
    padding-right: 22px;
    border-right: 1px solid var(--line);
  }
  .stat:last-child {
    border-right: none;
  }
  .stat b {
    font-size: 17px;
    font-weight: 650;
    font-variant-numeric: tabular-nums;
  }
  .stat b.mono {
    font-family: var(--mono);
    font-size: 13.5px;
    line-height: 1.7;
  }
  .stat span {
    color: var(--muted);
    font-size: 11px;
    letter-spacing: 0.04em;
  }
  .imp {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 12px;
    color: var(--muted);
    font-size: 12.5px;
  }
  .chip {
    font-size: 12px;
    padding: 2px 10px;
    border-radius: 999px;
    color: var(--accent);
    background: var(--accent-soft);
    border: 1px solid transparent;
  }
  .chip:hover {
    border-color: var(--accent);
  }
  .hint {
    margin: 10px 0 0;
    color: var(--faint);
    font-size: 11.5px;
  }
</style>
