#!/bin/bash
# Full clean-slate rebuild of .Data/src_extracted (v2 — all fixes folded in).
set -u
ROOT=/mnt/dev/Dev/Coding_Projects/PokeXYZ-RDComp
DATA=$ROOT/.Data
SRC=$DATA/src_extracted
RFS=$DATA/3ds_extracted/romfs
XY=$ROOT/target/release/xy
AH=/opt/ghidra/support/analyzeHeadless
PROJ=$SRC/decomp/ghidra_project
SCRIPTS=$ROOT/xy-decomp/ghidra

log() { echo "[$(date +%H:%M:%S)] $*"; }

log "STAGE A: structure + fast extractions"
mkdir -p "$SRC"/{garc,cro,cro_extracted,shaders,audio,video,text,decomp/ghidra_project,manual}
cp "$RFS"/*.cro "$SRC/cro/"
cp "$RFS"/static.crs "$SRC/cro/"
cp "$RFS"/.crr/static.crr "$SRC/cro/"
cp "$RFS"/shaders/*.shbin "$SRC/shaders/"

# banner jingle: slice the CWAV out of the CBMD banner
python3 - <<'PYEOF'
import struct
b = open('/mnt/dev/Dev/Coding_Projects/PokeXYZ-RDComp/.Data/3ds_extracted/exefs/banner.bin','rb').read()
at = b.find(b'CWAV')
size = struct.unpack('<I', b[at+0xC:at+0x10])[0]
open('/mnt/dev/Dev/Coding_Projects/PokeXYZ-RDComp/.Data/src_extracted/banner_jingle.bcwav','wb').write(b[at:at+size])
print(f'  banner jingle: {size} bytes sliced from banner.bin @{at:#x}')
PYEOF

log "A1: garc extract-tree (romfs/a)"
"$XY" archive extract-tree -i "$RFS/a" -o "$SRC/garc" --decompress | tail -1
log "A2: root-level .garc files"
for g in "$RFS"/*.garc; do
  "$XY" archive unpack -i "$g" -o "$SRC/garc/$(basename "$g")" --decompress | tail -1
done

log "A3: cro extract (134 modules)"
for m in "$SRC/cro"/*.cro "$SRC/cro"/static.crs; do
  n=$(basename "$m" | sed 's/\.cro$//;s/\.crs$//')
  "$XY" cro extract "$m" -o "$SRC/cro_extracted/$n" >/dev/null || echo "CRO-EXTRACT-FAIL: $n"
done
echo "  cro_extracted modules: $(ls "$SRC/cro_extracted" | wc -l)"

log "A4: text conversion"
declare -A L=([2]=jpn [3]=jpn2 [4]=eng [5]=fre [6]=ita [7]=ger [8]=spa [9]=kor)
for n in 2 3 4 5 6 7 8 9; do
  mkdir -p "$SRC/text/gametext_${L[$n]}"
  ok=0
  for f in "$SRC/garc/a/0/7/$n"/*.bin; do
    "$XY" script text "$f" -o "$SRC/text/gametext_${L[$n]}/$(basename "$f" .bin).json" >/dev/null 2>&1 && ok=$((ok+1))
  done
  echo "  gametext_${L[$n]}: $ok"
done
declare -A S=([0]=jpn [1]=jpn2 [2]=eng [3]=fre [4]=ita [5]=ger [6]=spa [7]=kor)
for n in 0 1 2 3 4 5 6 7; do
  mkdir -p "$SRC/text/storytext_${S[$n]}"
  ok=0
  for f in "$SRC/garc/a/0/8/$n"/*.bin; do
    "$XY" script text "$f" -o "$SRC/text/storytext_${S[$n]}/$(basename "$f" .bin).json" >/dev/null 2>&1 && ok=$((ok+1))
  done
  echo "  storytext_${S[$n]}: $ok"
done

log "A5: manual + icon (1x and 8x)"
"$XY" archive manual "$DATA/3ds_extracted/part1_romfs/Manual.bcma" "$SRC/manual" | tail -1
"$XY" gfx icon "$DATA/3ds_extracted/exefs/icon.bin" -o "$SRC/icon.png" | tail -1
"$XY" gfx icon "$DATA/3ds_extracted/exefs/icon.bin" --scale 8 -o "$SRC/icon_8x.png" | tail -1

log "STAGE B: media (ffmpeg)"
for f in "$RFS"/sound/*.aac; do
  ffmpeg -n -loglevel error -i "$f" "$SRC/audio/$(basename "$f" .aac).wav" </dev/null || echo "AUDIO-FAIL: $f"
done
echo "  audio wavs: $(ls "$SRC/audio"/*.wav | wc -l)"
for f in "$RFS"/m/*.moflex; do
  ffmpeg -n -loglevel error -i "$f" "$SRC/video/$(basename "$f" .moflex).mp4" </dev/null || echo "VIDEO-FAIL: $f"
done
echo "  videos: $(ls "$SRC/video"/*.mp4 2>/dev/null | wc -l)"

log "STAGE C1: Ghidra code.bin"
cp "$DATA/3ds_extracted/exefs/code.bin" "$SRC/decomp/"
mkdir -p "$SRC/decomp/code_c"
"$AH" "$PROJ" PokemonX -import "$SRC/decomp/code.bin" -processor ARM:LE:32:v6 \
  -loader BinaryLoader -loader-baseAddr 0x00100000 -scriptPath "$SCRIPTS" \
  -postScript DecompileAll.java "$SRC/decomp/code_c" > /tmp/opencode/ghidra_codebin.log 2>&1
rc=$?
if [ $rc -ne 0 ] || [ ! -f "$SRC/decomp/code_c/functions.tsv" ]; then
  echo "C1 FAILED rc=$rc"; tail -5 /tmp/opencode/ghidra_codebin.log; exit 1
fi
echo "  code.bin: $(grep -c true "$SRC/decomp/code_c/functions.tsv") functions decompiled"

log "STAGE C2: Ghidra all CRO modules"
mkdir -p "$SRC/decomp/cro_src"
for d in "$SRC/cro_extracted"/*/; do
  n=$(basename "$d")
  t="$d/seg0_text.bin"
  [ -s "$t" ] || continue
  out="$SRC/decomp/cro_c/$n"
  if [ -f "$out/functions.tsv" ]; then echo "  $n already done"; continue; fi
  mkdir -p "$out"
  cp "$t" "$SRC/decomp/cro_src/$n.bin"
  base=$(awk -F'\t' '$1=="0" && $2==".text" {print $5}' "$d/segments.tsv")
  base=${base:-0x100180}
  if "$AH" "$PROJ" PokemonX -import "$SRC/decomp/cro_src/$n.bin" -processor ARM:LE:32:v6 \
      -loader BinaryLoader -loader-baseAddr "$base" -scriptPath "$SCRIPTS" \
      -postScript DecompileAll.java "$out" >> /tmp/opencode/ghidra_cros.log 2>&1 \
      && grep -q true "$out/functions.tsv" 2>/dev/null; then
    echo "  $n: $(grep -c true "$out/functions.tsv") fns"
  else
    echo "  GHIDRA-FAIL: $n"
  fi
done

log "STAGE D: symbolication + curate"
python3 - <<'PYEOF'
import csv
SRC = '/mnt/dev/Dev/Coding_Projects/PokeXYZ-RDComp/.Data/src_extracted'
syms = {}
with open(f'{SRC}/cro_extracted/static/symbols.tsv') as f:
    for row in csv.DictReader(f, delimiter='\t'):
        a = int(row['load_addr'], 16)
        syms[a] = row['name']; syms[a & ~1] = row['name']
matched = 0; total = 0
with open(f'{SRC}/decomp/code_c/functions.tsv') as f, open(f'{SRC}/decomp/code_c/functions_sym.tsv', 'w') as out:
    r = csv.DictReader(f, delimiter='\t'); w = csv.writer(out, delimiter='\t', lineterminator='\n')
    w.writerow(['entry', 'name', 'symbol', 'body_size', 'decompiled'])
    for row in r:
        total += 1
        a = int(row['entry'], 16); s = syms.get(a, syms.get(a & ~1, ''))
        matched += bool(s)
        w.writerow([row['entry'], row['name'], s, row['body_size'], row['decompiled']])
print(f'symbolicated {matched}/{total}')
PYEOF
"$XY" curate -i "$SRC" | tail -1
log "REBUILD COMPLETE"
