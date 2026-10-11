"""Emit the single-page dashboard: heatmap.html (self-contained, data
embedded, works from file://), heatmap_data.json (live-poll payload) and
summary.md (plain-text table)."""
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
#layout{display:flex;min-height:calc(100vh - 58px)}
#side{width:270px;min-width:270px;border-right:1px solid var(--border);
  padding:12px;position:sticky;top:58px;height:calc(100vh - 58px);overflow-y:auto}
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
main{flex:1;padding:16px 20px 60px;max-width:1200px}
h2{font-size:13px;color:var(--dim);text-transform:uppercase;letter-spacing:1px;
  margin:22px 0 8px}
#overview .stackbar{display:flex;height:22px;border-radius:5px;overflow:hidden;
  border:1px solid var(--border)}
#overview .stackbar i{display:block;height:100%}
#overview .legend{display:flex;gap:14px;flex-wrap:wrap;margin-top:8px;font-size:12px;
  color:var(--dim)}
#overview .legend .sw{display:inline-block;width:10px;height:10px;
  border-radius:2px;margin-right:5px;vertical-align:-1px}
#central{border:1px solid var(--border);border-radius:10px;background:var(--panel);
  padding:14px;display:flex;gap:18px;flex-wrap:wrap;align-items:flex-start}
#central .cwrap{flex:0 1 620px;min-width:340px}
#radial{width:100%;max-width:620px;height:auto;display:block;cursor:crosshair}
#central .cmeta{flex:1 1 300px;min-width:260px;font-size:12px;color:var(--dim);
  line-height:1.6}
#central .cmeta h3{margin:0 0 6px;font-size:14px;color:var(--text)}
#central .cmeta .btns{margin-top:10px;display:flex;gap:8px;flex-wrap:wrap}
#nslist{display:flex;flex-wrap:wrap;gap:6px}
.nschip{background:var(--panel2);border:1px solid var(--border);border-radius:14px;
  padding:3px 11px;font-size:12px;cursor:pointer;color:var(--dim)}
.nschip:hover{border-color:var(--accent);color:var(--text)}
.nschip.sel{border-color:#2ecc71;color:#2ecc71}
#nsfilter{display:none;margin:8px 0;padding:7px 12px;background:#182417;
  border:1px solid #2e5b2e;border-radius:6px;font-size:12px}
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
<body>
<header>
  <h1>PokeXYZ-RDComp heatmap<small class="mono" id="game"></small></h1>
  <div class="chips" id="stats"></div>
  <div class="spacer"></div>
  <span class="live" id="live"><span class="dot"></span><span id="livetxt">
    static</span></span>
  <span class="chip mono" id="stamp"></span>
  <button class="btn" id="expSvgAll">Export SVG (all)</button>
  <button class="btn" id="expPngAll">Export PNG (all)</button>
</header>
<div id="layout">
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
  <main>
    <div id="overview">
      <div class="stackbar" id="stackbar"></div>
      <div class="legend" id="legend"></div>
    </div>
    <h2>Central heatmap — every module a sector of one canvas</h2>
    <div id="central">
      <div class="cwrap"><canvas id="radial" width="940" height="940"></canvas></div>
      <div class="cmeta">
        <h3>How to read it</h3>
        Sector angle ∝ function count (12 o'clock, clockwise, alphabetical).
        Cells fill each sector from the center outward in <b>address
        order</b> — inner rings are the module's lowest addresses. Colour =
        highest achieved stage: dark grey raw → teal decompiled → green
        identified → gold verified; red = interpreted at runtime.
        Hover any cell for module/address/symbol; click a sector to jump to
        its detail grid below. The center disc carries the corpus totals.
        <div class="btns">
          <button class="btn" id="expSvgRad">Export radial SVG</button>
          <button class="btn" id="expPngRad">Export radial PNG</button>
        </div>
      </div>
    </div>
    <h2>Namespaces (identified symbols)</h2>
    <div id="nsfilter"></div>
    <div id="nslist"></div>
    <h2>Modules — address-ordered function grid</h2>
    <div id="mods"></div>
    <footer>Cell = one function. Grid cells: address order left→right,
      top→bottom. Sources: 3dsrecomp port headers · pipeline symbol CSVs ·
      CRO/static export tables · Ghidra decomp corpus · pipeline
      byte-identical relinks. Regenerate with
      <span class="mono">xy-heatmap-py/heatmap.py</span>.</footer>
  </main>
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

/* ---- central radial heatmap ---- */
const RAD={size:940, r0:104, dr:0, pad:70};
let RL=null;            /* radial layout */

function buildRadialLayout(){
  const N=DATA.totals.functions, K=M.length;
  const minA=0.006;                       /* ~0.34° floor so tiny modules show */
  const maxR=RAD.size/2-RAD.pad;
  /* one global ring thickness: the largest module must fit inside maxR.
     sum_k dA*(r0+k*dr) ≈ dA*(r0*rows + dr*rows²/2) = n  ->  solve dr */
  const big=M.reduce((a,b)=>a.n>b.n?a:b);
  const dAmax=minA+(2*Math.PI-K*minA)*(big.n/N);
  RAD.dr=Math.max(3.2,(maxR-RAD.r0)*Math.sqrt(dAmax/(2*big.n)));
  let a=-Math.PI/2;                        /* 12 o'clock, clockwise */
  const mods=[];
  const list=M.slice().sort((x,y)=>x.name.localeCompare(y.name));
  for(const m of list){
    const dA=minA+(2*Math.PI-K*minA)*(m.n/N);
    const rings=[]; let off=0,k=0;
    while(off<m.n){
      const rMid=RAD.r0+(k+0.5)*RAD.dr;
      const cap=Math.max(1,Math.floor(dA*rMid/RAD.dr));
      rings.push({r1:RAD.r0+k*RAD.dr, r2:RAD.r0+(k+1)*RAD.dr, cap, off});
      off+=Math.min(cap,m.n-off); k++;
    }
    mods.push({m,a0:a,dA,rings});
    a+=dA;
  }
  RL={mods,maxR};
}
function wedge(g,cx,cy,r1,r2,a1,a2){
  g.beginPath();
  g.arc(cx,cy,r2,a1,a2);
  g.arc(cx,cy,r1,a2,a1,true);
  g.closePath(); g.fill();
}
function drawRadial(){
  const cv=document.getElementById("radial");
  const dpr=window.devicePixelRatio||1;
  cv.width=RAD.size*dpr; cv.height=RAD.size*dpr;
  const g=cv.getContext("2d"); g.scale(dpr,dpr);
  const c=RAD.size/2;
  g.fillStyle="#101216"; g.fillRect(0,0,RAD.size,RAD.size);
  for(const sec of RL.mods){
    const m=sec.m;
    for(const ring of sec.rings){
      const inRing=Math.min(ring.cap,m.n-ring.off);
      const cellA=sec.dA/ring.cap;
      const gap=Math.min(0.3*cellA,0.004);
      let run=null;                        /* run-length wedges */
      const flush=()=>{
        if(!run) return;
        wedge(g,c,c,ring.r1+0.5,ring.r2-0.5,
          sec.a0+run.j0*cellA+gap, sec.a0+(run.j1+1)*cellA-gap);
        run=null;
      };
      for(let j=0;j<inRing;j++){
        const col=cellColor(m,ring.off+j);
        if(run && run.col===col && run.j1===j-1){ run.j1=j; }
        else{ flush(); run={col,j0:j,j1:j}; }
      }
      flush();
    }
  }
  /* sector separators */
  g.strokeStyle="#1c1f26"; g.lineWidth=1;
  for(const sec of RL.mods){
    const depth=RAD.r0+sec.rings.length*RAD.dr;
    g.beginPath();
    g.moveTo(c+Math.cos(sec.a0)*RAD.r0, c+Math.sin(sec.a0)*RAD.r0);
    g.lineTo(c+Math.cos(sec.a0)*depth, c+Math.sin(sec.a0)*depth);
    g.stroke();
  }
  /* outer labels: sectors wide enough to fit a name */
  g.fillStyle="#8b929c"; g.font="9px ui-monospace,Consolas,monospace";
  for(const sec of RL.mods){
    if(sec.dA<0.055) continue;
    const depth=RAD.r0+sec.rings.length*RAD.dr+8;
    const mid=sec.a0+sec.dA/2;
    const x=c+Math.cos(mid)*depth, y=c+Math.sin(mid)*depth;
    g.save(); g.translate(x,y); g.rotate(mid);
    if(mid>Math.PI/2&&mid<Math.PI*1.5){ g.rotate(Math.PI); g.textAlign="right"; }
    else g.textAlign="left";
    let nm=sec.m.name; if(nm.length>22) nm=nm.slice(0,21)+"…";
    g.fillText(nm, mid>Math.PI/2&&mid<Math.PI*1.5?-3:3, 3);
    g.restore();
  }
  /* center disc with totals */
  g.beginPath(); g.arc(c,c,RAD.r0-8,0,2*Math.PI);
  g.fillStyle="#14161a"; g.fill();
  g.strokeStyle="#23262d"; g.stroke();
  const t=DATA.totals;
  g.fillStyle="#e8e8e8"; g.textAlign="center";
  g.font="600 26px ui-monospace,Consolas,monospace";
  g.fillText(fmt(t.functions),c,c-14);
  g.font="11px ui-monospace,Consolas,monospace"; g.fillStyle="#9aa0aa";
  g.fillText("functions · "+t.modules+" modules",c,c+8);
  g.fillText("identified "+(100*t.named/Math.max(t.functions,1)).toFixed(1)
    +"%",c,c+24);
}
function radialHit(e){
  const cv=e.target;
  const r=cv.getBoundingClientRect();
  const sx=RAD.size/r.width;               /* css -> layout scale */
  const dx=(e.clientX-r.left)*sx-RAD.size/2;
  const dy=(e.clientY-r.top)*sx-RAD.size/2;
  const rad=Math.hypot(dx,dy);
  let ang=Math.atan2(dy,dx)+Math.PI/2;     /* 0 at 12 o'clock, cw */
  if(ang<0) ang+=2*Math.PI;
  for(const sec of RL.mods){
    if(ang<sec.a0+Math.PI/2||ang>=sec.a0+sec.dA+Math.PI/2) continue;
    const k=Math.floor((rad-RAD.r0)/RAD.dr);
    const ring=sec.rings[k];
    if(!ring) return null;
    const inRing=Math.min(ring.cap,sec.m.n-ring.off);
    const j=Math.min(inRing-1,
      Math.floor((ang-(sec.a0+Math.PI/2))/sec.dA*ring.cap));
    if(j<0) return null;
    return {m:sec.m,i:ring.off+j,sec};
  }
  return null;
}
function onRadialMove(e){
  const hit=radialHit(e);
  if(!hit) return hideTip();
  showTipFor(hit.m,hit.i,e);
}
function radialSvg(){
  const s=RAD.size,c=s/2;
  const P=[]; const pol=(r,a)=>[c+r*Math.cos(a),c+r*Math.sin(a)];
  P.push(`<svg xmlns="http://www.w3.org/2000/svg" width="${s}" height="${s}" viewBox="0 0 ${s} ${s}">`,
    `<rect width="100%" height="100%" fill="#101216"/>`);
  for(const sec of RL.mods){
    const m=sec.m;
    for(const ring of sec.rings){
      const inRing=Math.min(ring.cap,m.n-ring.off);
      const cellA=sec.dA/ring.cap;
      const gap=Math.min(0.3*cellA,0.004);
      let run=null;
      const flush=()=>{
        if(!run) return;
        const a1=sec.a0+run.j0*cellA+gap, a2=sec.a0+(run.j1+1)*cellA-gap;
        const [x1,y1]=pol(ring.r2,a1),[x2,y2]=pol(ring.r2,a2);
        const [x3,y3]=pol(ring.r1,a2),[x4,y4]=pol(ring.r1,a1);
        P.push(`<path d="M ${x1.toFixed(1)} ${y1.toFixed(1)} `+
          `A ${ring.r2} ${ring.r2} 0 0 1 ${x2.toFixed(1)} ${y2.toFixed(1)} `+
          `L ${x3.toFixed(1)} ${y3.toFixed(1)} `+
          `A ${ring.r1} ${ring.r1} 0 0 0 ${x4.toFixed(1)} ${y4.toFixed(1)} Z" `+
          `fill="${run.col}"/>`);
        run=null;
      };
      for(let j=0;j<inRing;j++){
        const col=cellColor(m,ring.off+j);
        if(run && run.col===col && run.j1===j-1){ run.j1=j; }
        else{ flush(); run={col,j0:j,j1:j}; }
      }
      flush();
    }
  }
  for(const sec of RL.mods){
    const depth=RAD.r0+sec.rings.length*RAD.dr;
    const [x1,y1]=pol(RAD.r0,sec.a0),[x2,y2]=pol(depth,sec.a0);
    P.push(`<line x1="${x1.toFixed(1)}" y1="${y1.toFixed(1)}" `+
      `x2="${x2.toFixed(1)}" y2="${y2.toFixed(1)}" stroke="#1c1f26"/>`);
    if(sec.dA>=0.055){
      const mid=sec.a0+sec.dA/2, lr=depth+10;
      let nm=esc(sec.m.name); if(nm.length>22) nm=nm.slice(0,21)+"…";
      const [lx,ly]=pol(lr,mid);
      const deg=mid*180/Math.PI;
      const flip=mid>Math.PI/2&&mid<Math.PI*1.5;
      P.push(`<text x="${lx.toFixed(1)}" y="${ly.toFixed(1)}" fill="#8b929c" `+
        `font-family="monospace" font-size="9" text-anchor="${flip?"end":"start"}" `+
        `transform="rotate(${(flip?deg+180:deg).toFixed(1)} ${lx.toFixed(1)} ${ly.toFixed(1)})">`+
        `${nm}</text>`);
    }
  }
  const t=DATA.totals;
  P.push(`<circle cx="${c}" cy="${c}" r="${RAD.r0-8}" fill="#14161a" stroke="#23262d"/>`,
    `<text x="${c}" y="${c-6}" fill="#e8e8e8" font-family="monospace" font-size="26" text-anchor="middle" font-weight="600">${fmt(t.functions)}</text>`,
    `<text x="${c}" y="${c+12}" fill="#9aa0aa" font-family="monospace" font-size="11" text-anchor="middle">functions · ${t.modules} modules</text>`,
    `<text x="${c}" y="${c+28}" fill="#9aa0aa" font-family="monospace" font-size="11" text-anchor="middle">identified ${(100*t.named/Math.max(t.functions,1)).toFixed(1)}% · ${DATA.generated}</text>`,
    `</svg>`);
  return P.join("\n");
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
    row.onclick=()=>jumpTo(cssId(m.name));
    box.appendChild(row);
  }
}
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
function namedN(m){ let n=0; for(const s of m.st) if(s&1)n++; return n; }
function pct(m){ return 100*namedN(m)/Math.max(m.n,1); }
function colorOf(k){ return STAGES.find(s=>s.key===k).color; }
function esc(s){ return s.replace(/&/g,"&amp;").replace(/</g,"&lt;"); }
function cssId(s){ return "mod-"+s.replace(/[^A-Za-z0-9_-]/g,"_"); }

/* ---- namespaces ---- */
function renderNs(){
  const box=document.getElementById("nslist"); box.innerHTML="";
  const top=DATA.namespaces.slice(0,30);
  if(!top.length){ box.innerHTML=
    `<span class="chip">no mangled-namespace symbols found</span>`; return; }
  for(const rec of top){
    const chip=document.createElement("span"); chip.className="nschip mono";
    chip.textContent=`${rec.ns} · ${rec.count}`;
    if(nsFilter===rec.ns) chip.classList.add("sel");
    chip.onclick=()=>{ nsFilter=(nsFilter===rec.ns)?null:rec.ns;
      renderNs(); rerenderCanvases(); drawRadial(); };
    box.appendChild(chip);
  }
  const flt=document.getElementById("nsfilter");
  if(nsFilter){ flt.style.display="block";
    flt.innerHTML=`namespace filter: <b class="mono">${esc(nsFilter)}</b> — `+
      `non-matching cells dimmed · <a href="#" onclick="nsFilter=null;`+
      `renderNs();rerenderCanvases();drawRadial();return false">clear</a>`;
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
        d.onclick=()=>jumpTo(cssId(dep));
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
          d.onclick=()=>jumpTo(cssId(dep));
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
  download(`pokexyz-heatmap-${stamp()}.svg`,
    new Blob([allSvg().svg],{type:"image/svg+xml"}));
}
function exportAllPng(){
  const {svg,w,h}=allSvg();
  const maxArea=2.0e8;
  let scale=2;
  if(w*h*scale*scale>maxArea) scale=Math.sqrt(maxArea/(w*h));
  rasterize(svg,scale).then(canvas=>
    canvas.toBlob(b=>download(`pokexyz-heatmap-${stamp()}.png`,b)));
}
function exportRadialSvg(){
  download(`pokexyz-radial-${stamp()}.svg`,
    new Blob([radialSvg()],{type:"image/svg+xml"}));
}
function exportRadialPng(){
  rasterize(radialSvg(),3).then(canvas=>
    canvas.toBlob(b=>download(`pokexyz-radial-${stamp()}.png`,b)));
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
document.getElementById("expSvgAll").onclick=exportAllSvg;
document.getElementById("expPngAll").onclick=exportAllPng;
document.getElementById("expSvgRad").onclick=exportRadialSvg;
document.getElementById("expPngRad").onclick=exportRadialPng;

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
        decode(); renderHeader(); renderSide(); renderNs();
        buildRadialLayout(); drawRadial(); renderMods();
      } else {
        document.getElementById("stamp").textContent=d.generated;
      }
    }catch(_e){/* transient */}
  },3000);
}

/* ---- boot ---- */
document.getElementById("q").addEventListener("input",renderSide);
document.getElementById("sort").addEventListener("change",renderSide);
const radCv=document.getElementById("radial");
radCv.addEventListener("mousemove",onRadialMove);
radCv.addEventListener("mouseleave",hideTip);
radCv.addEventListener("click",e=>{
  const hit=radialHit(e);
  if(hit) jumpTo(cssId(hit.m.name));
});
let rszT=null;
window.addEventListener("resize",()=>{ clearTimeout(rszT);
  rszT=setTimeout(rerenderCanvases,150); });
decode(); renderHeader(); renderSide(); renderNs();
buildRadialLayout(); drawRadial(); renderMods(); goLive();
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
