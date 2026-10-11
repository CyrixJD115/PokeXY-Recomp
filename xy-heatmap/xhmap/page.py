"""Emit the dashboard: heatmap.html (tabbed, self-contained, data
embedded, works from file://), heatmap_data.json (live-poll payload) and
summary.md (plain-text table).

Tabs: Overview (progress + central treemap) · Modules (per-module
address-ordered grids + import graph) · Namespaces (identified symbols).
The central treemap layout comes embedded in the data (computed by
xhmap.assets.treemap_layout) — the canvas and the committed README SVG
consume the same rects."""
import json
import os

HTML_TEMPLATE = r"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>PokeXYZ-RDComp · decomp/recomp heatmap</title>
<style>
:root{
  --bg:#0f1115;--panel:#16181d;--panel2:#1b1e24;--border:#23262d;
  --text:#e8e8e8;--dim:#9aa0aa;--accent:#4fc3f7;
  --c-raw:#414650;--c-decomp:#2b6f8a;--c-named:#2ecc71;
  --c-verified:#f0b429;--c-hint:#e74c3c;
}
*{box-sizing:border-box}
body{margin:0;background:var(--bg);color:var(--text);
  font:14px/1.45 "Segoe UI",system-ui,sans-serif}
.mono{font-family:ui-monospace,"Cascadia Mono",Consolas,monospace}
header{position:sticky;top:0;z-index:50;background:linear-gradient(
  180deg,#14171c 0%,#14171cf2 100%);border-bottom:1px solid var(--border);
  padding:10px 18px;display:flex;flex-wrap:wrap;gap:10px 22px;align-items:center}
header h1{font-size:16px;margin:0;font-weight:600;letter-spacing:.3px}
header h1 small{color:var(--dim);font-weight:400;margin-left:8px}
.chips{display:flex;gap:8px;flex-wrap:wrap}
.chip{background:var(--panel2);border:1px solid var(--border);border-radius:6px;
  padding:3px 9px;font-size:12px;color:var(--dim)}
.chip b{color:var(--text);font-weight:600}
.spacer{flex:1}
.btn{background:#20242c;border:1px solid #2f343e;border-radius:6px;color:var(--text);
  padding:5px 11px;font-size:12px;cursor:pointer}
.btn:hover{background:#282d37;border-color:var(--accent)}
.live{display:inline-flex;align-items:center;gap:6px;font-size:12px;color:var(--dim)}
.live .dot{width:8px;height:8px;border-radius:50%;background:#555;transition:.3s}
.live.on .dot{background:#2ecc71;box-shadow:0 0 6px #2ecc71;animation:pulse 2s infinite}
@keyframes pulse{50%{opacity:.5}}
#tabbar{position:sticky;top:57px;z-index:49;background:#12151a;
  border-bottom:1px solid var(--border);padding:0 18px;display:flex;gap:2px}
#tabbar button{background:none;border:none;border-bottom:2px solid transparent;
  color:var(--dim);font:600 13px/1 "Segoe UI",system-ui,sans-serif;
  padding:11px 16px;cursor:pointer;letter-spacing:.3px}
#tabbar button:hover{color:var(--text)}
#tabbar button.on{color:var(--accent);border-bottom-color:var(--accent)}
.pane{display:none;padding:16px 20px 60px;max-width:1240px}
body[data-tab=overview] #pane-overview{display:block}
body[data-tab=modules] #pane-modules{display:flex;gap:18px;max-width:none;
  padding-right:20px}
body[data-tab=namespaces] #pane-namespaces{display:block}
#side{width:270px;min-width:270px;position:sticky;top:104px;
  height:calc(100vh - 120px);overflow-y:auto}
#side input,#side select{width:100%;background:var(--panel2);color:var(--text);
  border:1px solid var(--border);border-radius:6px;padding:6px 8px;font-size:12px;
  margin-bottom:8px}
.modrow{padding:6px 8px;border-radius:6px;cursor:pointer;display:flex;
  flex-wrap:wrap;justify-content:space-between;gap:2px 6px;align-items:baseline}
.modrow:hover{background:var(--panel2)}
.modrow .nm{font-size:12px;overflow:hidden;text-overflow:ellipsis;
  white-space:nowrap;flex:1;min-width:0}
.modrow .pct{font-size:11px;color:var(--dim);white-space:nowrap}
.bar{height:4px;border-radius:2px;background:var(--c-raw);overflow:hidden;
  display:flex;margin-top:3px;flex-basis:100%}
.bar i{display:block;height:100%}
#modcol{flex:1;min-width:0;max-width:1060px}
h2{font-size:13px;color:var(--dim);text-transform:uppercase;letter-spacing:1px;
  margin:20px 0 8px}
#overview .stackbar{display:flex;height:24px;border-radius:5px;overflow:hidden;
  border:1px solid var(--border)}
#overview .stackbar i{display:block;height:100%}
#overview .legend{display:flex;gap:16px;flex-wrap:wrap;margin-top:8px;
  font-size:12px;color:var(--dim)}
#overview .legend .sw{display:inline-block;width:10px;height:10px;
  border-radius:2px;margin-right:5px;vertical-align:-1px}
#treecard{border:1px solid var(--border);border-radius:10px;background:var(--panel);
  padding:14px;display:flex;gap:18px;flex-wrap:wrap;align-items:flex-start;margin-top:10px}
#treecard .twrap{flex:1 1 640px;min-width:340px}
#treemap{width:100%;height:auto;display:block;cursor:crosshair;border-radius:4px}
#treecard .tmeta{flex:0 1 300px;min-width:250px;font-size:12px;color:var(--dim);
  line-height:1.6}
#treecard .tmeta h3{margin:0 0 6px;font-size:14px;color:var(--text)}
#treecard .tmeta .btns{margin-top:10px;display:flex;gap:8px;flex-wrap:wrap}
#nslist{display:flex;flex-wrap:wrap;gap:6px}
.nschip{background:var(--panel2);border:1px solid var(--border);border-radius:14px;
  padding:3px 11px;font-size:12px;cursor:pointer;color:var(--dim)}
.nschip:hover{border-color:var(--accent);color:var(--text)}
.nschip.sel{border-color:#2ecc71;color:#2ecc71}
#nsfilter{display:none;margin:8px 0;padding:7px 12px;background:#182417;
  border:1px solid #2e5b2e;border-radius:6px;font-size:12px}
#nstable{width:100%;border-collapse:collapse;margin-top:14px;font-size:12px}
#nstable th,#nstable td{text-align:left;padding:6px 10px;border-bottom:
  1px solid var(--border)}
#nstable th{color:var(--dim);font-weight:600;font-size:11px;
  text-transform:uppercase;letter-spacing:.6px}
#nstable td .imp{margin:1px 3px 1px 0;display:inline-block}
section.mod{margin-bottom:26px;border:1px solid var(--border);border-radius:10px;
  background:var(--panel);overflow:hidden}
section.mod>header{position:static;background:var(--panel2);border-bottom:
  1px solid var(--border);padding:8px 14px;display:flex;flex-wrap:wrap;
  gap:6px 14px;align-items:center}
section.mod h3{margin:0;font-size:14px;font-weight:600}
section.mod .meta{font-size:11px;color:var(--dim)}
section.mod .meta b{color:var(--text)}
section.mod canvas{display:block;margin:10px 14px 14px;cursor:crosshair;
  border-radius:4px}
section.mod .imports{padding:0 14px 12px;display:flex;flex-wrap:wrap;gap:5px}
.imp{font-size:11px;background:var(--panel2);border:1px solid var(--border);
  border-radius:4px;padding:2px 7px;cursor:pointer;color:var(--dim)}
.imp:hover{color:var(--accent);border-color:var(--accent)}
#tooltip{position:fixed;z-index:99;background:#0b0d10f2;border:1px solid #2f343e;
  border-radius:6px;padding:7px 10px;font-size:12px;pointer-events:none;
  display:none;max-width:480px}
#tooltip .t1{color:var(--accent)}#tooltip .t2{color:var(--dim)}
footer{color:var(--dim);font-size:11px;margin-top:30px;border-top:
  1px solid var(--border);padding-top:10px}
@media(max-width:860px){#side{display:none}}
</style>
</head>
<body data-tab="overview">
<header>
  <h1>PokeXYZ-RDComp heatmap<small class="mono" id="game"></small></h1>
  <div class="chips" id="stats"></div>
  <div class="spacer"></div>
  <span class="live" id="live"><span class="dot"></span><span id="livetxt">
    static</span></span>
  <span class="chip mono" id="stamp"></span>
  <button class="btn" id="expStripSvg">Export SVG (modules)</button>
  <button class="btn" id="expStripPng">Export PNG (modules)</button>
</header>
<nav id="tabbar">
  <button data-tab="overview" class="on">Overview</button>
  <button data-tab="modules">Modules</button>
  <button data-tab="namespaces">Namespaces</button>
</nav>

<div class="pane" id="pane-overview">
  <div id="overview">
    <div class="stackbar" id="stackbar"></div>
    <div class="legend" id="legend"></div>
  </div>
  <h2>Central treemap — every module a box of one canvas</h2>
  <div id="treecard">
    <div class="twrap"><canvas id="treemap" width="1000" height="640"></canvas></div>
    <div class="tmeta">
      <h3>How to read it</h3>
      Box area ∝ function count. Each box carries its functions in
      <b>address order</b> (left→right, top→bottom); colour = highest
      achieved stage: dark grey raw → teal decompiled → green identified →
      gold verified; red = interpreted at runtime. Hover for
      module/address/symbol; click a box to open it in the Modules tab.
      <div class="btns">
        <button class="btn" id="expTreeSvg">Export treemap SVG</button>
        <button class="btn" id="expTreePng">Export treemap PNG</button>
      </div>
    </div>
  </div>
  <footer>Cell = one function. Sources: 3dsrecomp port headers ·
    pipeline symbol CSVs · CRO/static export tables · Ghidra decomp
    corpus · pipeline byte-identical relinks. Regenerate with
    <span class="mono">xy-heatmap/heatmap.py</span>.</footer>
</div>

<div class="pane" id="pane-modules">
  <aside id="side">
    <input id="q" placeholder="filter modules…" spellcheck="false">
    <select id="sort">
      <option value="name">sort: name</option>
      <option value="funcs">sort: functions ↓</option>
      <option value="named">sort: named % ↓</option>
      <option value="unnamed">sort: unnamed ↓</option>
    </select>
    <div id="modlist"></div>
  </aside>
  <div id="modcol"><div id="mods"></div></div>
</div>

<div class="pane" id="pane-namespaces">
  <h2>Namespaces (identified symbols)</h2>
  <div id="nsfilter"></div>
  <div id="nslist"></div>
  <table id="nstable">
    <thead><tr><th>namespace</th><th>functions</th><th>modules</th></tr></thead>
    <tbody id="nsbody"></tbody>
  </table>
</div>

<div id="tooltip"></div>
<script>
"use strict";
const DATA = __DATA_JSON__;

/* ---- stage model ---- */
const STAGES = [
  {bit:8, key:"hint",     label:"Interpreted (hints)",  color:"#e74c3c"},
  {bit:4, key:"verified", label:"Natively Verified",    color:"#f0b429"},
  {bit:1, key:"named",    label:"Identified / Named",   color:"#2ecc71"},
  {bit:2, key:"decomp",   label:"Decompiled to C (unnamed)", color:"#2b6f8a"},
  {bit:0, key:"raw",      label:"Raw Binary",           color:"#414650"},
];
function clsOf(st){ for(const s of STAGES){ if(st & s.bit) return s; }
  return STAGES[STAGES.length-1]; }
const CELL=7, COLS=112, PAD=10, GAPT=16;

let M=[];               /* decoded modules */
let nsFilter=null;

function decode(){
  M = DATA.modules.map(m=>{
    const addrs=m.addrs? m.addrs.split(" "):[];
    return {...m, addrs, st:new Uint8Array(m.st)};
  });
}
function counts(m){
  const c={raw:0,named:0,decomp:0,verified:0,hint:0,recomp:0};
  for(const s of m.st){ if(s&8)c.hint++; if(s&4)c.verified++;
    if(s&16)c.recomp++;
    if(s&1)c.named++; else if(s&2)c.decomp++; else c.raw++; }
  return c;
}

/* ---- header / overview ---- */
function fmt(n){ return n.toLocaleString("en-US"); }
function renderHeader(){
  const t=DATA.totals;
  document.getElementById("game").textContent=DATA.game;
  document.getElementById("stamp").textContent=DATA.generated;
  const pct=(x)=>(100*x/Math.max(t.functions,1)).toFixed(1)+"%";
  document.getElementById("stats").innerHTML=
    `<span class="chip">modules <b>${t.modules}</b></span>`+
    `<span class="chip">functions <b>${fmt(t.functions)}</b></span>`+
    `<span class="chip">identified <b>${fmt(t.named)}</b> (${pct(t.named)})</span>`+
    `<span class="chip">decompiled <b>${fmt(t.decompiled)}</b> (${pct(t.decompiled)})</span>`+
    `<span class="chip">recomp-covered <b>${fmt(t.recomp)}</b> (${pct(t.recomp)})</span>`+
    `<span class="chip">verified <b>${fmt(t.verified)}</b></span>`+
    `<span class="chip">link-ok <b>${t.linkVerified}/${t.modules}</b></span>`+
    (t.hints?`<span class="chip">interpreted <b>${fmt(t.hints)}</b></span>`:"");
  const T={raw:0,named:0,decomp:0,verified:0,hint:0};
  for(const m of M){ const c=counts(m); for(const k in T) T[k]+=c[k]; }
  const bar=document.getElementById("stackbar"), lg=document.getElementById("legend");
  bar.innerHTML=""; lg.innerHTML="";
  for(const s of STAGES){
    const n=T[s.key]||0;
    if(!n && s.key!=="raw" && s.key!=="decomp") continue;
    const w=100*n/Math.max(t.functions,1);
    const i=document.createElement("i");
    i.style.width=w+"%"; i.style.background=s.color; i.title=s.label+": "+fmt(n);
    bar.appendChild(i);
    const l=document.createElement("span");
    l.innerHTML=`<span class="sw" style="background:${s.color}"></span>`+
      `${s.label} <b class="mono">${fmt(n)}</b>`;
    lg.appendChild(l);
  }
}

/* ---- tabs ---- */
function activateTab(name){
  document.body.dataset.tab=name;
  for(const b of document.querySelectorAll("#tabbar button"))
    b.classList.toggle("on", b.dataset.tab===name);
  if(name==="modules") setTimeout(rerenderCanvases, 30);
}
for(const b of document.querySelectorAll("#tabbar button"))
  b.onclick=()=>activateTab(b.dataset.tab);
function jumpToModule(name){
  activateTab("modules");
  setTimeout(()=>jumpTo(cssId(name)), 30);
}

/* ---- central treemap (layout arrives precomputed in DATA.treemap) ---- */
const TREE={cv:null, dpr:1};
let TREEL=null;
function decodeTree(){
  TREEL = DATA.treemap || {width:1000,height:640,rects:[]};
  TREEL.byName = {};
  for(const r of TREEL.rects) TREEL.byName[r.name]=r;
}
function treeRectOf(m){
  return TREEL && TREEL.byName ? TREEL.byName[m.name] : null;
}
function treeCellColor(st){
  const base=clsOf(st).color;
  if(nsFilter) return shade(base,0.22);   /* treemap: dim all while filtering */
  return base;
}
function drawTreemap(){
  const cv=document.getElementById("treemap");
  const dpr=window.devicePixelRatio||1;
  cv.width=TREEL.width*dpr; cv.height=TREEL.height*dpr;
  const g=cv.getContext("2d"); g.scale(dpr,dpr);
  g.fillStyle="#101216"; g.fillRect(0,0,TREEL.width,TREEL.height);
  for(const r of TREEL.rects){
    const m=M.find(x=>x.name===r.name); if(!m) continue;
    g.fillStyle="#181b21"; g.strokeStyle="#23262d";
    g.fillRect(r.x,r.y,r.w,r.h); g.strokeRect(r.x+.5,r.y+.5,r.w-1,r.h-1);
    for(let i=0;i<m.n;i++){
      const row=Math.floor(i/r.cols), col=i%r.cols;
      const cx=r.x+2+col*r.cell_w, cy=r.y+r.strip+row*r.cell_h;
      if(cy+r.cell_h>r.y+r.h) break;
      g.fillStyle=treeCellColor(m.st[i]);
      g.fillRect(cx,cy,r.cell_w-1,r.cell_h-1);
    }
    if(r.strip && r.w>=44){
      g.fillStyle="#c9ced6"; g.font="9px ui-monospace,Consolas,monospace";
      g.textAlign="left";
      let nm=m.name; const mc=Math.max(3,Math.floor((r.w-10)/5.6));
      if(nm.length>mc) nm=nm.slice(0,mc-1)+"…";
      g.fillText(nm, r.x+4, r.y+r.strip-3);
      if(r.w>=110){
        g.fillStyle="#79818c"; g.textAlign="right";
        g.fillText(fmt(m.n), r.x+r.w-4, r.y+r.strip-3);
        g.textAlign="left";
      }
    }
  }
  const t=DATA.totals;
  g.fillStyle="#79818c"; g.font="10px ui-monospace,Consolas,monospace";
  g.textAlign="right";
  g.fillText(`${fmt(t.functions)} functions · ${t.modules} modules · identified `+
    `${(100*t.named/Math.max(t.functions,1)).toFixed(1)}% · ${DATA.generated}`,
    TREEL.width-4, TREEL.height-4);
  g.textAlign="left";
}
function treeHit(e){
  const cv=e.target, r=cv.getBoundingClientRect();
  const sx=TREEL.width/r.width, sy=TREEL.height/r.height;
  const x=(e.clientX-r.left)*sx, y=(e.clientY-r.top)*sy;
    for(const rc of TREEL.rects){
    if(x>=rc.x&&x<rc.x+rc.w&&y>=rc.y&&y<rc.y+rc.h){
      const m=M.find(v=>v.name===rc.name); if(!m) return null;
      const col=Math.floor((x-rc.x-2)/rc.cell_w);
      const row=Math.floor((y-rc.y-rc.strip)/rc.cell_h);
      const i=row*rc.cols+col;
      if(i>=0&&i<m.n) return {m,i};
      return {m,i:null};
    }
  }
  return null;
}
function onTreeMove(e){
  const hit=treeHit(e);
  if(!hit){ hideTip(); return; }
  if(hit.i===null){ hideTip(); return; }
  showTipFor(hit.m,hit.i,e);
}

/* ---- sidebar ---- */
function renderSide(){
  const q=document.getElementById("q").value.toLowerCase();
  const sort=document.getElementById("sort").value;
  let list=M.slice();
  if(q) list=list.filter(m=>m.name.toLowerCase().includes(q));
  list.sort((a,b)=>{
    if(sort==="funcs") return b.n-a.n;
    if(sort==="named") return pct(b)-pct(a);
    if(sort==="unnamed") return (b.n-namedN(b))-(a.n-namedN(a));
    return a.name.localeCompare(b.name);
  });
  const box=document.getElementById("modlist"); box.innerHTML="";
  for(const m of list){
    const c=counts(m), nNamed=c.named, p=100*nNamed/Math.max(m.n,1);
    const row=document.createElement("div"); row.className="modrow";
    row.innerHTML=`<span class="nm mono">${esc(m.name)}</span>`+
      `<span class="pct mono">${p.toFixed(1)}% · ${fmt(m.n)}</span>`+
      `<div class="bar"><i style="width:${100*c.verified/m.n}%;`+
      `background:${colorOf("verified")}"></i>`+
      `<i style="width:${100*nNamed/m.n}%;background:${colorOf("named")}"></i>`+
      `<i style="width:${100*c.decomp/m.n}%;background:${colorOf("decomp")}"></i>`+
      `<i style="width:${100*c.raw/m.n}%;background:${colorOf("raw")}"></i></div>`;
    row.onclick=()=>jumpToModule(m.name);
    box.appendChild(row);
  }
}
function namedN(m){ let n=0; for(const s of m.st) if(s&1)n++; return n; }
function pct(m){ return 100*namedN(m)/Math.max(m.n,1); }
function colorOf(k){ return STAGES.find(s=>s.key===k).color; }
function esc(s){ return s.replace(/&/g,"&amp;").replace(/</g,"&lt;"); }
function cssId(s){ return "mod-"+s.replace(/[^A-Za-z0-9_-]/g,"_"); }
function jumpTo(id){
  const t=document.getElementById(id);
  if(!t) return;
  t.scrollIntoView({behavior:"smooth",block:"start"});
  /* environments without a smooth-scroll animator (headless, some
     kiosks): if nothing moved, settle instantly */
  setTimeout(()=>{
    if(Math.abs(t.getBoundingClientRect().top)>window.innerHeight*0.9)
      t.scrollIntoView({behavior:"auto",block:"start"});
  },350);
}

/* ---- namespaces ---- */
function renderNs(){
  const box=document.getElementById("nslist"); box.innerHTML="";
  const body=document.getElementById("nsbody"); body.innerHTML="";
  const top=DATA.namespaces.slice(0,30);
  if(!top.length){ box.innerHTML=
    `<span class="chip">no mangled-namespace symbols found</span>`; return; }
  /* module breakdown per ns */
  const perNs={};
  for(const m of M){
    for(const k in m.ns){
      const ns=DATA.nsIndex[m.ns[k]];
      (perNs[ns]=perNs[ns]||{})[m.name]=(perNs[ns][m.name]||0)+1;
    }
  }
  for(const rec of top){
    const chip=document.createElement("span"); chip.className="nschip mono";
    chip.textContent=`${rec.ns} · ${rec.count}`;
    if(nsFilter===rec.ns) chip.classList.add("sel");
    chip.onclick=()=>{ nsFilter=(nsFilter===rec.ns)?null:rec.ns;
      renderNs(); rerenderCanvases(); drawTreemap(); };
    box.appendChild(chip);
    const tr=document.createElement("tr");
    const mods=Object.entries(perNs[rec.ns]||{})
      .sort((a,b)=>b[1]-a[1]);
    tr.innerHTML=`<td class="mono">${esc(rec.ns)}</td>`+
      `<td class="mono">${fmt(rec.count)}</td><td>`+
      mods.map(([mn,c])=>`<span class="imp mono" data-mod="${esc(mn)}">`+
        `${esc(mn)} (${c})</span>`).join("")+`</td>`;
    body.appendChild(tr);
  }
  for(const el of body.querySelectorAll("[data-mod]"))
    el.onclick=()=>jumpToModule(el.dataset.mod);
  const flt=document.getElementById("nsfilter");
  if(nsFilter){ flt.style.display="block";
    flt.innerHTML=`namespace filter: <b class="mono">${esc(nsFilter)}</b> — `+
      `non-matching cells dimmed · <a href="#" onclick="nsFilter=null;`+
      `renderNs();rerenderCanvases();drawTreemap();return false">clear</a>`;
  } else flt.style.display="none";
}
function nsOfCell(m,i){
  const id=m.ns[String(i)];
  return id===undefined?null:DATA.nsIndex[id];
}

/* ---- module sections + canvas ---- */
const rendered=new Set();
function renderMods(){
  const host=document.getElementById("mods"); host.innerHTML="";
  rendered.clear();
  for(const m of M){
    const c=counts(m);
    const sec=document.createElement("section");
    sec.className="mod"; sec.id=cssId(m.name);
    const head=document.createElement("header");
    head.innerHTML=`<h3 class="mono">${esc(m.name)}</h3>`+
      `<span class="meta"><b>${fmt(m.n)}</b> fns</span>`+
      `<span class="meta">identified <b>${fmt(c.named)}</b></span>`+
      `<span class="meta">decompiled <b>${fmt(c.decomp+c.named)}</b></span>`+
      `<span class="meta">recomp <b>${fmt(c.recomp)}</b></span>`+
      (c.verified?`<span class="meta">verified <b>${fmt(c.verified)}</b></span>`:"")+
      (c.hint?`<span class="meta">interpreted <b style="color:#e74c3c">`+
        `${fmt(c.hint)}</b></span>`:"")+
      `<span class="meta">${m.linkOk?"🔗 link-verified":"link —"}</span>`+
      `<span class="spacer"></span>`+
      `<button class="btn" data-svg>SVG</button>`+
      `<button class="btn" data-png>PNG</button>`;
    const cv=document.createElement("canvas");
    sec.appendChild(cv);
    if(m.imports.length){
      const im=document.createElement("div"); im.className="imports";
      im.innerHTML=`<span class="meta" style="align-self:center">imports:</span>`;
      for(const dep of m.imports.slice(0,40)){
        const d=document.createElement("span"); d.className="imp mono";
        d.textContent=dep;
        d.onclick=()=>jumpToModule(dep);
        im.appendChild(d);
      }
      if(m.imports.length>40){ const more=document.createElement("span");
        more.className="meta"; more.textContent=`+${m.imports.length-40} more`;
        im.appendChild(more); }
      const impBy=m.importers;
      if(impBy.length){
        const lbl=document.createElement("span"); lbl.className="meta";
        lbl.style.alignSelf="center"; lbl.textContent="imported by:";
        im.appendChild(lbl);
        for(const dep of impBy.slice(0,20)){
          const d=document.createElement("span"); d.className="imp mono";
          d.textContent=dep;
          d.onclick=()=>jumpToModule(dep);
          im.appendChild(d);
        }
      }
      sec.appendChild(im);
    }
    host.appendChild(sec);

    const layout={cv,sec,m};
    head.querySelector("[data-svg]").onclick=()=>exportModuleSvg(m);
    head.querySelector("[data-png]").onclick=()=>exportModulePng(m);
    cv.addEventListener("mousemove",e=>onMove(e,layout));
    cv.addEventListener("mouseleave",hideTip);

    const io=new IntersectionObserver(es=>{
      for(const e of es) if(e.isIntersecting){ drawModule(layout); }
    },{rootMargin:"300px"});
    io.observe(cv);
  }
}
function layoutOf(m, availW){
  let cols=COLS;
  if(availW){ cols=Math.floor((availW-2*PAD)/CELL); }
  cols=Math.max(1,Math.min(cols,m.n));
  const rows=Math.ceil(m.n/cols);
  const w=PAD*2+cols*CELL, h=PAD*2+rows*CELL+GAPT;
  return {cols,rows,w,h};
}
function cellColor(m,i){
  const base=clsOf(m.st[i]).color;
  if(nsFilter && nsOfCell(m,i)!==nsFilter){
    return shade(base,0.22);          /* dimmed */
  }
  return base;
}
function shade(hex,f){
  const n=parseInt(hex.slice(1),16);
  const r=(n>>16)&255,g=(n>>8)&255,b=n&255;
  const mix=c=>Math.round(c*f+18*(1-f));
  return `rgb(${mix(r)},${mix(g)},${mix(b)})`;
}
function drawModule({cv,sec,m}){
  const avail=cv.parentNode ? cv.parentNode.clientWidth-30 : 0;
  const {cols,rows,w,h}=layoutOf(m, avail||undefined);
  const dpr=window.devicePixelRatio||1;
  cv.width=w*dpr; cv.height=h*dpr;
  cv.style.width=w+"px"; cv.style.height=h+"px";
  const g=cv.getContext("2d"); g.scale(dpr,dpr);
  g.fillStyle="#14161a"; g.fillRect(0,0,w,h);
  for(let i=0;i<m.n;i++){
    const x=PAD+(i%cols)*CELL, y=PAD+GAPT+Math.floor(i/cols)*CELL;
    g.fillStyle=cellColor(m,i);
    g.fillRect(x,y,CELL-1,CELL-1);
  }
  rendered.add(m.name);
}
function rerenderCanvases(){
  for(const {cv,sec,m} of allLayouts()) if(rendered.has(m.name)) drawModule({cv,sec,m});
  renderSide();
}
function allLayouts(){
  const out=[];
  for(const sec of document.querySelectorAll("section.mod")){
    const m=M.find(x=>cssId(x.name)===sec.id);
    if(m) out.push({cv:sec.querySelector("canvas"),sec,m});
  }
  return out;
}

/* ---- tooltip ---- */
const tip=document.getElementById("tooltip");
function showTipFor(m,i,e){
  const addr=parseInt(m.addrs[i],16);
  const nm=m.names[String(i)];
  const st=clsOf(m.st[i]);
  const ns=nsOfCell(m,i);
  tip.style.display="block";
  tip.innerHTML=`<div class="t1 mono">${esc(m.name)} · f_${addr.toString(16)}`+
    `</div>`+
    (nm?`<div class="mono">${esc(nm)}</div>`:"")+
    (ns?`<div class="t2 mono">ns ${esc(ns)}</div>`:"")+
    `<div class="t2">${st.label}${m.st[i]&16?" · recomp-covered":""}</div>`;
  const tx=Math.min(e.clientX+14,window.innerWidth-tip.offsetWidth-8);
  const ty=Math.min(e.clientY+14,window.innerHeight-tip.offsetHeight-8);
  tip.style.left=tx+"px"; tip.style.top=ty+"px";
}
function onMove(e,{cv,m}){
  const r=cv.getBoundingClientRect();
  const {cols}=layoutOf(m, r.width);
  const x=Math.floor((e.clientX-r.left-PAD)/CELL);
  const y=Math.floor((e.clientY-r.top-PAD-GAPT)/CELL);
  if(x<0||x>=cols||y<0) return hideTip();
  const i=y*cols+x;
  if(i>=m.n) return hideTip();
  showTipFor(m,i,e);
}
function hideTip(){ tip.style.display="none"; }

/* ---- exports ---- */
function download(name,blob){
  const a=document.createElement("a");
  a.href=URL.createObjectURL(blob); a.download=name;
  document.body.appendChild(a); a.click();
  setTimeout(()=>{URL.revokeObjectURL(a.href);a.remove();},400);
}
function stamp(){
  const d=new Date(DATA.generated.replace(" ","T"));
  const p=n=>String(n).padStart(2,"0");
  return `${d.getFullYear()}${p(d.getMonth()+1)}${p(d.getDate())}`+
    `-${p(d.getHours())}${p(d.getMinutes())}`;
}
function svgEsc(s){ return s.replace(/&/g,"&amp;").replace(/</g,"&lt;"); }

function treemapSvg(){
  const w=TREEL.width, h=TREEL.height;
  const P=[`<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}">`,
    `<rect width="${w}" height="${h}" fill="#101216"/>`];
  for(const r of TREEL.rects){
    const m=M.find(x=>x.name===r.name); if(!m) continue;
    P.push(`<rect x="${r.x}" y="${r.y}" width="${+r.w.toFixed(1)}" height="${+r.h.toFixed(1)}" fill="#181b21" stroke="#23262d"/>`);
    for(let i=0;i<m.n;i++){
      const row=Math.floor(i/r.cols), col=i%r.cols;
      const cx=r.x+2+col*r.cell, cy=r.y+r.strip+row*r.cell;
      if(cy+r.cell>r.y+r.h) break;
      let fill=treeCellColor(m.st[i]);
      P.push(`<rect x="${+cx.toFixed(1)}" y="${+cy.toFixed(1)}" width="${r.cell-1}" height="${r.cell-1}" fill="${fill}"/>`);
    }
    if(r.strip && r.w>=44){
      let nm=svgEsc(m.name); const mc=Math.max(3,Math.floor((r.w-10)/5.6));
      if(m.name.length>mc) nm=svgEsc(m.name.slice(0,mc-1))+"…";
      P.push(`<text x="${+(r.x+4).toFixed(1)}" y="${+(r.y+r.strip-3).toFixed(1)}" fill="#c9ced6" font-family="monospace" font-size="9">${nm}</text>`);
      if(r.w>=110) P.push(`<text x="${+(r.x+r.w-4).toFixed(1)}" y="${+(r.y+r.strip-3).toFixed(1)}" fill="#79818c" font-family="monospace" font-size="9" text-anchor="end">${fmt(m.n)}</text>`);
    }
  }
  const t=DATA.totals;
  P.push(`<text x="${w-4}" y="${h-4}" fill="#79818c" font-family="monospace" font-size="10" text-anchor="end">${fmt(t.functions)} functions · ${t.modules} modules · identified ${(100*t.named/Math.max(t.functions,1)).toFixed(1)}% · ${DATA.generated}</text>`);
  P.push("</svg>");
  return P.join("\n");
}

function moduleSvg(m,cell,withHeader=true){
  const cols=Math.min(COLS,m.n), rows=Math.ceil(m.n/cols);
  const pad=cell===CELL?PAD:12;
  const headH=withHeader?26:0;
  const w=pad*2+cols*cell, h=pad*2+rows*cell+headH+ (withHeader?10:0);
  const c=counts(m);
  const out=[`<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="0 0 ${w} ${h}">`,
    `<rect width="100%" height="100%" fill="#14161a"/>`];
  if(withHeader){
    out.push(`<text x="${pad}" y="${pad+8}" fill="#e8e8e8" font-family="monospace" font-size="12">`+
      `${svgEsc(m.name)} — ${m.n} functions · identified ${c.named}`+
      `${c.verified?` · verified ${c.verified}`:""}${m.linkOk?" · link-verified":""}</text>`);
  }
  for(let i=0;i<m.n;i++){
    const x=pad+(i%cols)*cell, y=pad+headH+(Math.floor(i/cols))*cell;
    const nm=m.names[String(i)];
    const t=`${svgEsc(m.name)} f_${parseInt(m.addrs[i],16).toString(16)}`+
      (nm?` — ${svgEsc(nm)}`:"")+` [${clsOf(m.st[i]).label}]`;
    out.push(`<rect x="${x}" y="${y}" width="${cell-1}" height="${cell-1}" fill="${cellColor(m,i)}"><title>${t}</title></rect>`);
  }
  out.push("</svg>");
  return out.join("\n");
}

function allSvg(){
  const cell=6, cols=COLS, pad=12, headH=30;
  const t=DATA.totals;
  const parts=[];
  const modBlocks=[];
  let y=72;
  for(const m of M){
    const rows=Math.ceil(m.n/cols);
    modBlocks.push({m,y});
    y+=headH+rows*cell+18;
  }
  const w=pad*2+cols*cell;
  parts.push(`<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${y}" viewBox="0 0 ${w} ${y}">`,
    `<rect width="100%" height="100%" fill="#0f1115"/>`,
    `<text x="${pad}" y="24" fill="#e8e8e8" font-family="monospace" font-size="15">PokeXYZ-RDComp heatmap — ${svgEsc(DATA.game)}</text>`,
    `<text x="${pad}" y="44" fill="#9aa0aa" font-family="monospace" font-size="11">${DATA.generated} · ${fmt(t.functions)} functions · identified ${fmt(t.named)} · verified ${fmt(t.verified)} · link-ok ${t.linkVerified}/${t.modules}</text>`);
  let lx=pad;
  for(const s of STAGES){
    parts.push(`<rect x="${lx}" y="52" width="9" height="9" fill="${s.color}"/>`,
      `<text x="${lx+13}" y="60" fill="#9aa0aa" font-family="monospace" font-size="10">${s.label}</text>`);
    lx+=13+s.label.length*6.2+18;
  }
  for(const {m,y:my} of modBlocks){
    const c=counts(m);
    parts.push(`<text x="${pad}" y="${my+14}" fill="#e8e8e8" font-family="monospace" font-size="11">${svgEsc(m.name)} — ${m.n} fns · identified ${c.named}${m.linkOk?" · link-verified":""}</text>`);
    for(let i=0;i<m.n;i++){
      const x=pad+(i%cols)*cell, yy=my+headH+Math.floor(i/cols)*cell;
      const nm=m.names[String(i)];
      const t=`${svgEsc(m.name)} f_${parseInt(m.addrs[i],16).toString(16)}`+
        (nm?` — ${svgEsc(nm)}`:"")+` [${clsOf(m.st[i]).label}]`;
      parts.push(`<rect x="${x}" y="${yy}" width="${cell-1}" height="${cell-1}" fill="${cellColor(m,i)}"><title>${t}</title></rect>`);
    }
  }
  parts.push("</svg>");
  return {svg:parts.join("\n"), w, h:y};
}

function exportTreemapSvg(){
  download(`heatmap-treemap-${stamp()}.svg`,
    new Blob([treemapSvg()],{type:"image/svg+xml"}));
}
function exportTreemapPng(){
  const cv=document.getElementById("treemap");
  cv.toBlob(b=>download(`heatmap-treemap-${stamp()}.png`,b));
}
function exportModuleSvg(m){
  download(`heatmap-${m.name.replace(/[^\w.-]/g,"_")}-${stamp()}.svg`,
    new Blob([moduleSvg(m,7)],{type:"image/svg+xml"}));
}
function exportModulePng(m){
  const svg=moduleSvg(m,7);
  rasterize(svg,3).then(canvas=>
    canvas.toBlob(b=>download(
      `heatmap-${m.name.replace(/[^\w.-]/g,"_")}-${stamp()}.png`,b)));
}
function exportAllSvg(){
  download(`pokexyz-heatmap-modules-${stamp()}.svg`,
    new Blob([allSvg().svg],{type:"image/svg+xml"}));
}
function exportAllPng(){
  const {svg,w,h}=allSvg();
  const maxArea=2.0e8;
  let scale=2;
  if(w*h*scale*scale>maxArea) scale=Math.sqrt(maxArea/(w*h));
  rasterize(svg,scale).then(canvas=>
    canvas.toBlob(b=>download(`pokexyz-heatmap-modules-${stamp()}.png`,b)));
}
function rasterize(svgStr,scale){
  return new Promise((res,rej)=>{
    const blob=new Blob([svgStr],{type:"image/svg+xml;charset=utf-8"});
    const url=URL.createObjectURL(blob);
    const img=new Image();
    img.onload=()=>{
      const cv=document.createElement("canvas");
      cv.width=Math.round(img.width*scale);
      cv.height=Math.round(img.height*scale);
      const g=cv.getContext("2d");
      g.fillStyle="#0f1115"; g.fillRect(0,0,cv.width,cv.height);
      g.drawImage(img,0,0,cv.width,cv.height);
      URL.revokeObjectURL(url); res(cv);
    };
    img.onerror=rej;
    img.src=url;
  });
}
document.getElementById("expStripSvg").onclick=exportAllSvg;
document.getElementById("expStripPng").onclick=exportAllPng;
document.getElementById("expTreeSvg").onclick=exportTreemapSvg;
document.getElementById("expTreePng").onclick=exportTreemapPng;

/* ---- live polling ---- */
function goLive(){
  const el=document.getElementById("live");
  if(!location.protocol.startsWith("http")){
    document.getElementById("livetxt").textContent=
      "static — serve with `serve`";
    return;
  }
  el.classList.add("on");
  document.getElementById("livetxt").textContent="live";
  setInterval(async()=>{
    try{
      const r=await fetch(`heatmap_data.json?v=${Date.now()}`,{cache:"no-store"});
      if(!r.ok) return;
      const d=await r.json();
      if(d.generated!==DATA.generated){
        Object.assign(DATA,d);
        decode(); decodeTree(); renderHeader(); renderSide(); renderNs();
        drawTreemap(); renderMods();
      } else {
        document.getElementById("stamp").textContent=d.generated;
      }
    }catch(_e){/* transient */}
  },3000);
}

/* ---- boot ---- */
document.getElementById("q").addEventListener("input",renderSide);
document.getElementById("sort").addEventListener("change",renderSide);
const treeCv=document.getElementById("treemap");
treeCv.addEventListener("mousemove",onTreeMove);
treeCv.addEventListener("mouseleave",hideTip);
treeCv.addEventListener("click",e=>{
  const hit=treeHit(e);
  if(hit) jumpToModule(hit.m.name);
});
let rszT=null;
window.addEventListener("resize",()=>{ clearTimeout(rszT);
  rszT=setTimeout(rerenderCanvases,150); });
decode(); decodeTree(); renderHeader(); renderSide(); renderNs();
drawTreemap(); renderMods(); activateTab("overview"); goLive();
</script>
</body>
</html>
"""


def render_summary(data, out_dir=None, stream=None):
    rows = ["| module | functions | named | named % | decompiled | recomp | "
            "verified | hints | link-ok |",
            "|---|---:|---:|---:|---:|---:|---:|---:|---|"]
    for m in data["modules"]:
        named = sum(1 for s in m["st"] if s & 1)
        dec = sum(1 for s in m["st"] if s & 2)
        rec = sum(1 for s in m["st"] if s & 16)
        ver = sum(1 for s in m["st"] if s & 4)
        hint = sum(1 for s in m["st"] if s & 8)
        pct = 100.0 * named / max(m["n"], 1)
        rows.append(f"| {m['name']} | {m['n']} | {named} | {pct:.1f}% | "
                    f"{dec} | {rec} | {ver} | {hint} | "
                    f"{'yes' if m['linkOk'] else '—'} |")
    t = data["totals"]
    rows.append("")
    rows.append(f"**Total: {t['functions']} functions, {t['named']} named "
                f"({100.0 * t['named'] / max(t['functions'], 1):.1f}%), "
                f"{t['decompiled']} decompiled, {t['recomp']} recomp-covered, "
                f"{t['verified']} verified, {t['hints']} interpreter hits, "
                f"{t['linkVerified']}/{t['modules']} modules link-verified.**")
    text = "# RDComp heatmap\n\n" + "\n".join(rows) + "\n"
    if stream is not None:
        print(text, file=stream)
        return
    with open(os.path.join(out_dir, "summary.md"), "w") as f:
        f.write(text)


def build(data, out_dir):
    """Write heatmap.html + heatmap_data.json + summary.md."""
    os.makedirs(out_dir, exist_ok=True)
    payload = json.dumps(data, separators=(",", ":"))
    with open(os.path.join(out_dir, "heatmap_data.json"), "w") as f:
        f.write(payload)
    page = HTML_TEMPLATE.replace("__DATA_JSON__", payload)
    with open(os.path.join(out_dir, "heatmap.html"), "w") as f:
        f.write(page)
    render_summary(data, out_dir=out_dir)
    t = data["totals"]
    print(f"wrote {out_dir}/heatmap.html + heatmap_data.json + summary.md")
    print(f"  {data['game']}")
    print(f"  {t['modules']} modules · {t['functions']:,} functions · "
          f"{t['named']:,} identified "
          f"({100.0 * t['named'] / max(t['functions'], 1):.1f}%) · "
          f"{t['decompiled']:,} decompiled · {t['recomp']:,} recomp-covered · "
          f"{t['verified']} verified · link-ok {t['linkVerified']}/{t['modules']}")
