"""The single-page web UI as one in-memory string.

A structured, fixed dashboard — deliberately NOT a canvas:

    header  (title · global stats · watch dot)
    toolbar (view tabs · search · tier filter · density · export)
    body    ( sidebar: module list   |   main: the current view )
    readout (live "what am I pointing at" line)
    footer  (legend explanation · sources · warnings)

There is no panning, dragging or zooming anywhere: every view is an SVG
laid out in normal document flow that scales to the available width; the
page scrolls like any web page. Hover details come from a readout bar
computed from ``data-cells`` metadata (module · address · tier), so even
70,000-cell views stay light. Clicking a module anywhere opens its detail
page (facts + imports + address grid).
"""

PAGE = r"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>xy-heatmap</title>
<style>
  :root {
    --bg:#121419; --panel:#1a1d24; --panel2:#20242d; --line:#272c35;
    --text:#e8eaed; --muted:#98a1ac; --accent:#2fa39e; --gold:#eec84a; --red:#e05252;
  }
  * { box-sizing: border-box; }
  html, body { margin:0; height:100%; background:var(--bg); color:var(--text);
    font: 14px/1.45 "Noto Sans","DejaVu Sans","Liberation Sans",Arial,sans-serif; }
  body { display:flex; flex-direction:column; }

  header { display:flex; align-items:baseline; gap:14px; padding:10px 16px 8px;
    background:var(--panel); border-bottom:1px solid var(--line); flex-wrap:wrap; }
  header h1 { font-size:16px; margin:0; letter-spacing:.3px; }
  header h1 b { color:var(--accent); }
  #stats { color:var(--muted); font-size:12px; }
  #watchdot { width:9px; height:9px; border-radius:50%; background:#3a4048;
    display:inline-block; margin-left:auto; }
  #watchdot.live { background:var(--accent); }
  #watchdot.stale { background:var(--gold); }

  #toolbar { display:flex; align-items:center; gap:8px; padding:8px 16px;
    background:var(--panel); border-bottom:1px solid var(--line); flex-wrap:wrap; }
  .tabs { display:flex; gap:4px; }
  .tabs button { background:transparent; color:var(--muted); border:1px solid var(--line);
    border-radius:6px; padding:5px 12px; cursor:pointer; font:inherit; font-size:13px; }
  .tabs button.active { color:var(--text); border-color:var(--accent); background:#203034; }
  select, input[type=search] { background:#101216; color:var(--text); border:1px solid var(--line);
    border-radius:6px; padding:5px 8px; font:inherit; font-size:13px; }
  label.opt { color:var(--muted); font-size:12px; display:flex; align-items:center; gap:5px; }
  a.btn { color:var(--text); text-decoration:none; border:1px solid var(--line);
    border-radius:6px; padding:5px 10px; font-size:13px; }
  a.btn:hover { border-color:var(--accent); }
  #crumb { color:var(--muted); font-size:13px; }
  #crumb a { color:var(--accent); text-decoration:none; cursor:pointer; }
  #crumb b { color:var(--text); }

  #body { flex:1; display:flex; min-height:0; }
  #side { width:265px; min-width:220px; border-right:1px solid var(--line);
    background:var(--panel); display:flex; flex-direction:column; }
  #sidehead { padding:8px 12px 4px; color:var(--muted); font-size:12px; }
  #modsearch { margin:0 10px 8px; width:calc(100% - 20px); }
  #modlist { overflow-y:auto; flex:1; padding-bottom:10px; }
  .mi { display:flex; align-items:center; gap:8px; padding:4px 12px; cursor:pointer;
        border-left:3px solid transparent; }
  .mi:hover { background:var(--panel2); }
  .mi.sel { background:var(--panel2); border-left-color:var(--accent); }
  .mi .nm { flex:1; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; font-size:13px; }
  .mi .ct { color:var(--muted); font-size:11.5px; }
  .mi .rl { font-size:10px; }
  .rl.ok { color:var(--accent); } .rl.bad { color:var(--red); }
  .mibar { height:4px; width:52px; border-radius:2px; overflow:hidden; display:flex; background:#3a4048; }
  .mibar i { display:block; height:100%; }

  #main { flex:1; overflow:auto; padding:14px 18px 24px; }
  #view svg { max-width:100%; height:auto; display:block; }
  #facts { margin:0 0 14px; padding:12px 14px; background:var(--panel);
    border:1px solid var(--line); border-radius:8px; font-size:13.5px; }
  #facts h2 { margin:0 0 8px; font-size:16px; }
  #facts .grid { display:flex; flex-wrap:wrap; gap:6px 26px; color:var(--muted); font-size:12.5px; }
  #facts .grid b { color:var(--text); font-weight:600; }
  #facts .imp { margin-top:8px; font-size:12.5px; color:var(--muted); }
  #facts .imp a { color:var(--accent); cursor:pointer; text-decoration:none; margin-right:8px; }

  #readout { padding:6px 16px; background:var(--panel); border-top:1px solid var(--line);
    color:var(--muted); font-size:12.5px; min-height:28px; white-space:pre-wrap; }
  #readout b { color:var(--text); }
  footer { display:flex; gap:18px; padding:5px 16px 7px; border-top:1px solid var(--line);
    color:var(--muted); font-size:11.5px; background:var(--panel); flex-wrap:wrap; }
  #warn { color:var(--gold); }
  .hint { margin-left:auto; }
</style>
</head>
<body>
<header>
  <h1><b>xy</b>-heatmap</h1>
  <span id="stats">loading…</span>
  <span id="watchdot" title="live reload"></span>
</header>
<div id="toolbar">
  <nav class="tabs">
    <button data-tab="treemap" class="active">Treemap</button>
    <button data-tab="overview">Overview</button>
    <button data-tab="grid">Grid</button>
    <button data-tab="namespaces">Namespaces</button>
  </nav>
  <span id="crumb"></span>
  <label class="opt">tier
    <select id="min">
      <option value="0" selected>all functions</option>
      <option value="1">decompiled+</option>
      <option value="2">named+</option>
      <option value="3">verified</option>
    </select>
  </label>
  <label class="opt">density
    <select id="cols">
      <option value="48">coarse</option>
      <option value="88" selected>normal</option>
      <option value="128">fine</option>
    </select>
  </label>
  <a class="btn" id="exsvg" download>SVG</a>
  <a class="btn" id="expng" download>PNG</a>
</div>
<div id="body">
  <aside id="side">
    <div id="sidehead">MODULES <span id="sidehint"></span></div>
    <input id="modsearch" type="search" placeholder="filter modules…">
    <div id="modlist"></div>
  </aside>
  <main id="main">
    <div id="facts" style="display:none"></div>
    <div id="view"></div>
  </main>
</div>
<div id="readout">point at any cell — its module, address and status show up here</div>
<footer>
  <span>ladder: raw → decompiled → named → verified (color = highest stage reached)</span>
  <span id="warn"></span>
  <span class="hint">__SOURCES__</span>
</footer>
<script>
"use strict";
const S = { tab:"treemap", module:null, fp:null, watch:__WATCH__, ds:null };
const $ = (id) => document.getElementById(id);
const TIER_NAMES = ["raw", "decompiled", "named", "verified"];

async function j(p) {
  const r = await fetch(p);
  if (!r.ok) throw new Error((await r.text()) || r.statusText);
  return r.json();
}

/* ---------------- sidebar ---------------- */
function buildSidebar() {
  const list = $("modlist"); list.innerHTML = "";
  const q = $("modsearch").value.trim().toLowerCase();
  let shown = 0;
  for (const m of S.ds.modules) {
    if (q && !m.name.toLowerCase().includes(q)) continue;
    shown++;
    const total = m.cells.length || 1;
    const counts = [0,0,0,0];
    for (const c of m.cells) counts[c[3]]++;
    const el = document.createElement("div");
    el.className = "mi" + (S.module === m.name ? " sel" : "");
    el.dataset.module = m.name;
    const segs = counts.map((n, t) =>
      `<i style="width:${(100 * n / total).toFixed(2)}%;background:${
        ["#454b57","#2fa39e","#58b368","#eec84a"][t]}"></i>`).join("");
    const rl = m.relink_ok === true ? `<span class="rl ok" title="relink byte-identical">✓</span>`
             : m.relink_ok === false ? `<span class="rl bad" title="relink MISMATCH">✗</span>` : "";
    el.innerHTML = `<span class="nm">${m.name}</span>` +
      `<span class="mibar">${segs}</span><span class="ct">${m.cells.length}</span>${rl}`;
    el.addEventListener("click", () => openModule(m.name));
    list.appendChild(el);
  }
  $("sidehint").textContent = `(${shown})`;
}

/* ---------------- views ------------------ */
function baseName() {
  return S.tab === "module" ? "module-" + encodeURIComponent(S.module) : S.tab;
}
function params() {
  const p = new URLSearchParams();
  if (+$("min").value) p.set("min", $("min").value);
  if (S.tab === "grid") p.set("cols", $("cols").value);
  if (S.tab === "module") p.set("bare", "1");  // facts card replaces the header
  return p.toString();
}
async function render() {
  const qs = params();
  const r = await fetch("/view/" + baseName() + ".svg" + (qs ? "?" + qs : ""));
  if (!r.ok) { $("view").innerHTML = "<pre style='color:var(--red)'>" + (await r.text()) + "</pre>"; return; }
  $("view").innerHTML = await r.text();
  bindHover();
  markExport();
}
async function showModuleFacts() {
  if (S.tab !== "module" || !S.module) { $("facts").style.display = "none"; $("crumb").textContent = ""; return; }
  $("crumb").innerHTML = `<a id="crumbback" style="color:var(--accent);cursor:pointer">← modules</a> / <b>${S.module}</b>`;
  $("crumbback").addEventListener("click", () => setTab("treemap"));
  try {
    const f = await j("/api/module/" + encodeURIComponent(S.module));
    const pct = (n) => f.total ? (100 * n / f.total).toFixed(1) + "%" : "0.0%";
    const imp = f.imports.length
      ? `<div class="imp">imports from: ` +
        f.imports.map((n) => `<a data-mod="${n}">${n}</a>`).join("") + `</div>`
      : "";
    $("facts").innerHTML = `<h2>${f.name}</h2>
      <div class="grid">
        <span><b>${f.total.toLocaleString()}</b> functions</span>
        <span>recomp-covered <b>${pct(f.covered)}</b></span>
        <span>decompiled <b>${pct(f.tiers.decompiled)}</b></span>
        <span>named <b>${f.tiers.named.toLocaleString()}</b> (${pct(f.tiers.named)})</span>
        <span>verified <b>${f.tiers.verified}</b></span>
        <span>address range <b>${f.addr_range ? f.addr_range[0] + " – " + f.addr_range[1] : "—"}</b></span>
        <span>relink <b>${f.relink_ok == null ? "—" : f.relink_ok ? "byte-identical" : "MISMATCH"}</b></span>
      </div>${imp}
      <div class="imp" style="color:var(--muted)">grid below: one cell = one function, address-ordered left→right, top→bottom.</div>`;
    $("facts").style.display = "";
    $("facts").querySelectorAll("[data-mod]").forEach((a) =>
      a.addEventListener("click", () => openModule(a.dataset.mod)));
  } catch (e) { $("facts").style.display = "none"; }
}
async function openModule(name) {
  S.module = name; S.tab = "module";
  location.hash = "module-" + name;
  document.querySelectorAll(".tabs button").forEach((b) => b.classList.toggle("active", false));
  buildSidebar();
  await showModuleFacts();
  await render();
}
async function setTab(tab) {
  S.tab = tab;
  location.hash = tab === "module" && S.module ? "module-" + S.module : tab;
  document.querySelectorAll(".tabs button").forEach((b) => b.classList.toggle("active", b.dataset.tab === tab));
  if (tab !== "module") S.module = null;
  buildSidebar();
  await showModuleFacts();
  await render();
}
document.querySelectorAll(".tabs button").forEach((b) =>
  b.addEventListener("click", () => setTab(b.dataset.tab)));
$("modsearch").addEventListener("input", buildSidebar);
$("min").addEventListener("change", render);
$("cols").addEventListener("change", render);

/* ------------- export links -------------- */
function markExport() {
  const qs = params();
  const q = qs ? "?" + qs : "";
  $("exsvg").href = "/view/" + baseName() + ".svg" + q;
  $("expng").href = "/view/" + baseName() + ".png?scale=3" + (qs ? "&" + qs : "");
  $("exsvg").download = baseName() + ".svg";
  $("expng").download = baseName() + ".png";
}

/* ------ hover readout (no pan/zoom) ------ */
function describe(module, addr, tier, name) {
  return `${module} · 0x${addr.toString(16).toUpperCase().padStart(6, "0")} · ` +
         `${TIER_NAMES[tier]}` + (name ? ` · ${name}` : "");
}
function bindHover() {
  const view = $("view");
  const groups = [];
  view.querySelectorAll("[data-cells]").forEach((g) => {
    const [base, stride, cols, px, n] = g.getAttribute("data-cells").split(":").map(Number);
    groups.push({ g, base, stride, cols, px, n });
  });
  view.addEventListener("mousemove", (e) => {
    for (const it of groups) {
      if (!it.g.contains(e.target)) continue;
      const svg = it.g.ownerSVGElement;
      const pt = new DOMPoint(e.clientX, e.clientY).matrixTransform(svg.getScreenCTM().inverse());
      const col = Math.floor((pt.x - it.g.getBBox().x) / it.px);
      const row = Math.floor((pt.y - it.g.getBBox().y) / it.px);
      const idx = row * it.cols + col;
      if (col < 0 || row < 0 || idx >= it.n) { return; }
      const m = S.ds.modules.find((x) => x.name === it.g.getAttribute("data-module"));
      const cell = m ? m.cells[idx] : null;
      $("readout").innerHTML = describe(
        it.g.getAttribute("data-module"), it.base + idx * it.stride,
        cell ? cell[3] : 0, cell ? cell[2] : "");
      return;
    }
    const modEl = e.target.closest("[data-module]");
    if (modEl && !modEl.getAttribute("data-cells")) {
      const name = modEl.getAttribute("data-module");
      const m = S.ds.modules.find((x) => x.name === name);
      if (m) $("readout").innerHTML = `<b>${m.name}</b> — ${m.cells.length.toLocaleString()} functions · click for module detail`;
    }
  });
  view.addEventListener("click", (e) => {
    const modEl = e.target.closest("[data-module]");
    if (modEl) openModule(modEl.getAttribute("data-module"));
  });
}

/* -------------- live reload --------------- */
async function poll() {
  try {
    const f = await j("/api/fingerprint");
    if (S.fp && f.fingerprint !== S.fp) {
      $("watchdot").classList.add("stale");
      await load();
      $("watchdot").classList.remove("stale");
    }
  } catch (err) { /* server restarting */ }
}
async function load() {
  const d = await j("/api/dataset");
  S.ds = d; S.fp = d.fingerprint;
  let total = 0;
  for (const m of d.modules) total += m.cells.length;
  $("stats").textContent = `${d.modules.length} modules · ${total.toLocaleString()} functions`;
  $("warn").textContent = d.warnings && d.warnings.length ? d.warnings.length + " collector warnings" : "";
  $("warn").title = d.warnings ? d.warnings.join("\n") : "";
  // deep link: #treemap / #overview / #grid / #namespaces / #module-<Name>
  const h = decodeURIComponent(location.hash.slice(1));
  if (h === "overview" || h === "grid" || h === "namespaces" || h === "treemap") S.tab = h;
  else if (h.startsWith("module-")) { S.tab = "module"; S.module = h.slice(7); }
  document.querySelectorAll(".tabs button").forEach((b) =>
    b.classList.toggle("active", b.dataset.tab === S.tab));
  buildSidebar();
  await showModuleFacts();
  await render();
}
if (S.watch) { $("watchdot").classList.add("live"); setInterval(poll, 3000); }
window.addEventListener("hashchange", () => {
  const h = decodeURIComponent(location.hash.slice(1));
  if (h.startsWith("module-")) openModule(h.slice(7));
  else if (["treemap", "overview", "grid", "namespaces"].includes(h)) setTab(h);
});
load();
</script>
</body>
</html>
"""
