//! Full BCRES/CGFX decoder — port of Ohana3DS-Rebirth `Models/CGFX.cs`
//! (by gdkchan), the reference implementation in `Tools & Libs/`.
//!
//! CGFX is Nintendo's CTR graphics container. In Pokémon X it appears as
//! GARC sub-files (`a/0/0/7` effect models + textures, `a/0/3/3` world
//! models, …) and inside PC/PF/PT containers. This port extracts meshes
//! (positions/normals/UVs/tangents/colors/skinning), materials with their
//! texture bindings, skeletons, textures (decoded via [`crate::texture`])
//! and skeletal animations.
//!
//! Binary layout (offsets are relative to the field's own position —
//! "rel"; 0 means null; verified against `garc/a/0/0/7/0001.bin` +
//! `0003.bin` on 2026-10-06):
//!
//! ```text
//! 0x00 "CGFX" | u16 BOM 0xFEFF | u16 header size 0x14 | u32 revision
//! 0x0C u32 total size (== file len) | 0x10 u32 content count
//! 0x14 "DATA" | u32 length, then 16 inline DICT references:
//!      models, textures, lookUpTables, materials, shaders, cameras,
//!      lights, fogs, scenes, skeletalAnims, materialAnims,
//!      visibilityAnims, cameraAnims, lightAnims, emitters
//!      (each inline ref: u32 entries + rel u32 dict offset)
//!
//! DICT (header 0x1c): "DICT" | u32 length | u32 entries | root node
//!      16 B { i32 ref, u16 left, u16 right, rel u32 name, rel u32 data }
//!      then `entries` × 16-byte nodes (radix tree; only name/data used).
//!      Names are NUL-terminated ASCII.
//!
//! TXOB (texture): flags u32 | "TXOB" | rev | rel name | userData… |
//!      u32 height | u32 width | glFormat | glType | mips | texObject |
//!      locationFlags | u32 format | 3 × u32 | u32 dataLength |
//!      rel u32 dataOffset | dynamicAllocator | bitsPerPixel | …
//!
//! CMDL (model): flags u32 (0x80 = hasSkeleton) | "CMDL" | rev | rel name |
//!      userData… | u32 | flags | childCount | u32 | animGroup… |
//!      scale/rotate/translate f32×3 | local & world 4×3 matrices |
//!      objectEntries + rel objectPtrTable | materials inline DICT |
//!      shapeEntries + rel shapePtrTable | objectNodes inline DICT |
//!      flags | cullMode | layerId | [rel skeletonOffset]
//!
//! SOBJ (shape): flags | "SOBJ" | rev | rel name | userData… | flags |
//!      rel boundingBox | positionOffset f32×3 | facesGroupEntries +
//!      rel facesGroupOffset | u32 | u32 vertexGroupEntries? + rel
//!      vertexGroupOffset | rel blendShapeOffset
//!
//! Vertex group (at rel(vertexGroupOffset)): rel VERT offset →
//!      3 × u32 | bufferObject | locationFlag | bufferLength + rel
//!      bufferOffset | locationAddress | memoryArea | u32 stride |
//!      componentsEntries + rel componentsOffset. Each component (at
//!      rel(componentsOffset + 4i)): flags | u32 attribute (0=pos,
//!      1=normal, 2=tangent, 3=color, 4..6=uv0..2, 7=boneIndex,
//!      8=boneWeight) | u32 (2 = interleaved) | 5 × u32 | u32 type
//!      (0=s8, 1=u8, 2=s16, 6=f32) | u32 componentCount | f32 scale |
//!      u32 offsetInStride
//!
//! Face group (at rel(facesGroupOffset + 4i)): nodeListEntries + rel
//!      nodeList | u32 skinning (0 none, 1 rigid, 2 smooth) |
//!      faceMainEntries + rel faceMain → rel faceDescriptor →
//!      { u32 (bit1: u16 indices) | u32 | u32 byteLength | rel data }
//!
//! CANM (skeletal animation): "CANM" | rev | rel name | rel target |
//!      u32 loopMode | f32 frameSize | members inline DICT | userData…
//!      Member (bone): u32 flags | rel bonePath | [rev<7: 8 B] |
//!      u32 segmentType (5 = Euler transform, 8 = quaternion transform)
//! ```

use crate::texture::{decode_rgba, TextureFormat};
use xy_core::error::{Error, Result};
use xy_core::util::{f32_le, u16_le, u32_le};

// ---- reader ----

/// Bounds-checked cursor. Every read advances; `rel_u32()` implements the
/// CGFX "relative offset" convention (0 stays 0, else + field position).
struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn seek(&mut self, pos: usize) -> Result<()> {
        if pos > self.data.len() {
            return Err(Error::truncated(pos, 0, "cgfx seek"));
        }
        self.pos = pos;
        Ok(())
    }

    fn u8(&mut self) -> Result<u8> {
        let v = xy_core::util::u8_at(self.data, self.pos)?;
        self.pos += 1;
        Ok(v)
    }

    fn u16(&mut self) -> Result<u16> {
        let v = u16_le(self.data, self.pos)?;
        self.pos += 2;
        Ok(v)
    }

    fn u32(&mut self) -> Result<u32> {
        let v = u32_le(self.data, self.pos)?;
        self.pos += 4;
        Ok(v)
    }

    fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }

    fn f32(&mut self) -> Result<f32> {
        let v = f32_le(self.data, self.pos)?;
        self.pos += 4;
        Ok(v)
    }

    fn skip(&mut self, n: usize) -> Result<()> {
        self.seek(self.pos + n)
    }

    /// Relative offset: reads a u32 at the cursor; 0 stays 0, else the
    /// absolute target is `value + field_position`.
    fn rel_u32(&mut self) -> Result<usize> {
        let at = self.pos;
        let v = self.u32()?;
        Ok(if v == 0 { 0 } else { at + v as usize })
    }

    /// NUL-terminated ASCII string at an absolute offset.
    fn cstr(&self, at: usize) -> Result<String> {
        if at >= self.data.len() {
            return Err(Error::truncated(at, 1, "cgfx string"));
        }
        let end = self.data[at..]
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| Error::truncated(at, 0, "cgfx string (unterminated)"))?;
        Ok(String::from_utf8_lossy(&self.data[at..at + end]).into_owned())
    }

    fn magic(&mut self, expect: &[u8; 4], label: &'static str) -> Result<()> {
        let at = self.pos;
        let got = self
            .data
            .get(at..at + 4)
            .ok_or_else(|| Error::truncated(at, 4, "cgfx chunk magic"))?;
        if got != expect {
            return Err(Error::magic(at, label, got));
        }
        self.pos += 4;
        Ok(())
    }
}

/// Relative pointer stored at `at`: 0 stays 0, else `at + value`.
fn rel_at(data: &[u8], at: usize) -> Result<usize> {
    let v = u32_le(data, at)? as usize;
    Ok(if v == 0 { 0 } else { at + v })
}

/// Consume a PICA command buffer purely for stream advancement
/// (`PICACommandReader(data, N, ignoreAlign: true)` in the reference).
fn skip_pica(r: &mut Reader, words: u32) -> Result<()> {
    const BLOCK_END: u16 = 0x23d;
    let mut read = 0u32;
    while read < words {
        r.u32()?; // parameter
        let header = r.u32()?;
        read += 2;
        let id = (header & 0xffff) as u16;
        let extra = (header >> 20) & 0x7ff;
        let consecutive = header & 0x8000_0000 != 0;
        if id == BLOCK_END {
            break;
        }
        let mut id = id;
        for _ in 0..extra {
            if consecutive {
                id = id.wrapping_add(1);
            }
            r.u32()?;
            read += 1;
        }
    }
    Ok(())
}

// ---- matrix (RenderBase.OMatrix: M{row}{col}, identity default) ----

/// 4×4 matrix, row-major `[row][col]`, matching Ohana's `OMatrix`
/// semantics exactly (including its `transform`, which multiplies as
/// `out.r = Σ v.c * M[c][r]`).
#[derive(Debug, Clone, Copy)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mat4 {
    pub fn identity() -> Self {
        let mut m = [[0f32; 4]; 4];
        m[0][0] = 1.0;
        m[1][1] = 1.0;
        m[2][2] = 1.0;
        m[3][3] = 1.0;
        Self(m)
    }

    /// Row-major multiply `a * b` (as `OMatrix.operator *`).
    pub fn mul(&self, b: &Self) -> Self {
        let a = &self.0;
        let b = &b.0;
        let mut c = [[0f32; 4]; 4];
        for (i, row) in c.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                let mut sum = 0f32;
                for k in 0..4 {
                    sum += a[i][k] * b[k][j];
                }
                *cell = sum;
            }
        }
        Self(c)
    }

    /// `OVector3.transform(v, m)` — Ohana's exact formula.
    pub fn transform(&self, v: [f32; 3]) -> [f32; 3] {
        let m = &self.0;
        [
            v[0] * m[0][0] + v[1] * m[1][0] + v[2] * m[2][0] + m[3][0],
            v[0] * m[0][1] + v[1] * m[1][1] + v[2] * m[2][1] + m[3][1],
            v[0] * m[0][2] + v[1] * m[1][2] + v[2] * m[2][2] + m[3][2],
        ]
    }

    fn scale(s: [f32; 3]) -> Self {
        let mut m = Self::identity();
        m.0[0][0] = s[0];
        m.0[1][1] = s[1];
        m.0[2][2] = s[2];
        m
    }

    fn rotate_x(a: f32) -> Self {
        let mut m = Self::identity();
        m.0[1][1] = a.cos();
        m.0[2][1] = -a.sin();
        m.0[1][2] = a.sin();
        m.0[2][2] = a.cos();
        m
    }

    fn rotate_y(a: f32) -> Self {
        let mut m = Self::identity();
        m.0[0][0] = a.cos();
        m.0[2][0] = a.sin();
        m.0[0][2] = -a.sin();
        m.0[2][2] = a.cos();
        m
    }

    fn rotate_z(a: f32) -> Self {
        let mut m = Self::identity();
        m.0[0][0] = a.cos();
        m.0[1][0] = -a.sin();
        m.0[0][1] = a.sin();
        m.0[1][1] = a.cos();
        m
    }

    fn translate(t: [f32; 3]) -> Self {
        let mut m = Self::identity();
        m.0[3][0] = t[0];
        m.0[3][1] = t[1];
        m.0[3][2] = t[2];
        m
    }
}

// ---- decoded output types ----

/// A decoded vertex (export-relevant subset of `RenderBase.OVertex`).
#[derive(Debug, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub color: [u8; 4],
    pub uv: [[f32; 2]; 3],
    pub nodes: Vec<u32>,
    pub weights: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct Bone {
    pub name: String,
    pub parent_id: i32,
    pub scale: [f32; 3],
    pub rotation: [f32; 3],
    pub translation: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub emission: [u8; 4],
    pub ambient: [u8; 4],
    pub diffuse: [u8; 4],
    pub specular0: [u8; 4],
    pub specular1: [u8; 4],
    /// Texture names bound to units 0..2 (`None` where not mapped).
    pub texture_names: [Option<String>; 3],
    pub shader_reference: Option<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub name: String,
    pub position_offset: [f32; 3],
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub is_visible: bool,
    pub material_id: usize,
    pub render_priority: u8,
    pub shape_index: usize,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub world_matrix: Mat4,
    pub materials: Vec<Material>,
    pub bones: Vec<Bone>,
    pub shapes: Vec<Shape>,
    pub meshes: Vec<Mesh>,
    /// XY effect models link geometry through CNOD chunks (GameFreak's
    /// rev-9 variant — layout unmapped); when their SOBJ shapes don't
    /// follow the standard SDK layout they are skipped and this flag set.
    pub geometry_partial: bool,
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub format: TextureFormat,
    /// Raw (still tiled/compressed) payload.
    pub data: Vec<u8>,
}

impl Texture {
    pub fn decode_rgba(&self) -> Result<Vec<u8>> {
        decode_rgba(&self.data, self.width, self.height, self.format)
    }
}

/// One animated channel of a bone.
#[derive(Debug, Clone, Default)]
pub struct KeyFrameGroup {
    pub key_frames: Vec<KeyFrame>,
    pub interpolation: u8, // 0 step, 4 linear, 8 hermite
}

#[derive(Debug, Clone, Copy)]
pub struct KeyFrame {
    pub frame: f32,
    pub value: f32,
    pub in_slope: f32,
    pub out_slope: f32,
}

/// Quaternion-mode track (`OAnimationKeyFrameGroup.vector`).
#[derive(Debug, Clone, Default)]
pub struct Vec4Track {
    pub start_frame: f32,
    pub end_frame: f32,
    pub vectors: Vec<[f32; 4]>,
}

#[derive(Debug, Clone, Default)]
pub struct AnimatedBone {
    pub name: String,
    pub rotation: [KeyFrameGroup; 3],
    pub translation: [KeyFrameGroup; 3],
    pub quat_scale: Option<Vec4Track>,
    pub quat_rotation: Option<Vec4Track>,
    pub quat_translation: Option<Vec4Track>,
}

#[derive(Debug, Clone, Default)]
pub struct SkeletalAnimation {
    pub name: String,
    pub target: String,
    pub loop_mode: u32,
    pub frame_size: f32,
    pub bones: Vec<AnimatedBone>,
}

/// A fully decoded CGFX file.
#[derive(Debug, Default)]
pub struct Cgfx {
    pub models: Vec<Model>,
    pub textures: Vec<Texture>,
    pub skeletal_animations: Vec<SkeletalAnimation>,
}

impl Cgfx {
    /// Parse a CGFX buffer (magic at offset 0 — unwrap containers first).
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut r = Reader::new(data);
        r.magic(b"CGFX", "'CGFX'")?;
        let bom = r.u16()?;
        if bom != 0xFEFF {
            return Err(Error::invalid("cgfx header", format!("BOM {bom:#06x}")));
        }
        let header_size = r.u16()? as usize;
        if header_size != 0x14 {
            return Err(Error::invalid(
                "cgfx header",
                format!("header size {header_size:#x}"),
            ));
        }
        let _revision = r.u32()?;
        let total = r.u32()? as usize;
        if total != data.len() {
            return Err(Error::invalid(
                "cgfx size",
                format!("header says {total:#x}, file is {:#x}", data.len()),
            ));
        }
        let _content_count = r.u32()?;
        r.seek(header_size)?;
        r.magic(b"DATA", "'DATA'")?;
        let _data_length = r.u32()?;

        // 16 dictionary references: models … emitters.
        let mut dict_offsets = [0usize; 16];
        for slot in dict_offsets.iter_mut() {
            let entries = r.u32()?;
            let dict_off = r.rel_u32()?;
            *slot = if entries > 0 { dict_off } else { 0 };
        }

        let mut out = Cgfx::default();
        // textures
        for (_, data_off) in read_dict(data, dict_offsets[1])? {
            out.textures.push(parse_texture(data, data_off)?);
        }
        // skeletal animations
        for (_, data_off) in read_dict(data, dict_offsets[9])? {
            out.skeletal_animations
                .push(parse_skeletal_animation(data, data_off)?);
        }
        // models
        for (_, data_off) in read_dict(data, dict_offsets[0])? {
            out.models.push(parse_model(data, data_off)?);
        }
        Ok(out)
    }
}

/// Read a DICT at an absolute offset → (name, absolute data offset) pairs.
fn read_dict(data: &[u8], off: usize) -> Result<Vec<(String, usize)>> {
    if off == 0 {
        return Ok(Vec::new());
    }
    let mut r = Reader::new(data);
    r.seek(off)?;
    r.magic(b"DICT", "'DICT'")?;
    let _length = r.u32()?;
    let entries = r.u32()? as usize;
    if entries > 0x10000 {
        return Err(Error::invalid("cgfx dict", format!("{entries} entries")));
    }
    r.skip(16)?; // root node (name/data unused by the reference)
    let mut out = Vec::with_capacity(entries);
    for _ in 0..entries {
        r.skip(8)?; // radix-tree reference + left/right links
        let name_off = r.rel_u32()?;
        let data_off = r.rel_u32()?;
        let name = if name_off != 0 {
            r.cstr(name_off)?
        } else {
            String::new()
        };
        out.push((name, data_off));
    }
    Ok(out)
}

/// Inline dictionary reference at the cursor (`getDictionary(input)`):
/// u32 entries + rel u32 offset; advances the cursor by 8 bytes total.
fn inline_dict(r: &mut Reader) -> Result<Vec<(String, usize)>> {
    let entries = r.u32()?;
    let dict_off = r.rel_u32()?;
    if entries == 0 {
        return Ok(Vec::new());
    }
    read_dict(r.data, dict_off)
}

/// TXOB texture entry.
fn parse_texture(data: &[u8], off: usize) -> Result<Texture> {
    let mut r = Reader::new(data);
    r.seek(off)?;
    let _flags = r.u32()?;
    r.magic(b"TXOB", "'TXOB'")?;
    let _rev = r.u32()?;
    let name_off = r.rel_u32()?;
    let _user_data_entries = r.u32()?;
    let _user_data_off = r.rel_u32()?;
    let height = r.u32()? as usize;
    let width = r.u32()? as usize;
    let _gl_format = r.u32()?;
    let _gl_type = r.u32()?;
    let _mipmaps = r.u32()?;
    let _texture_object = r.u32()?;
    let _location_flags = r.u32()?;
    let format = TextureFormat::from_u32(r.u32()?)?;
    r.skip(12)?;
    let data_length = r.u32()? as usize;
    let data_off = r.rel_u32()?;
    let _dynamic_allocator = r.u32()?;
    let _bits_per_pixel = r.u32()?;
    let _location_address = r.u32()?;
    let _memory_address = r.u32()?;

    if data_off == 0 || data_off + data_length > data.len() {
        return Err(Error::invalid(
            "cgfx texture",
            format!("payload [{data_off:#x}, +{data_length:#x}) out of bounds"),
        ));
    }
    Ok(Texture {
        name: if name_off != 0 {
            r.cstr(name_off)?
        } else {
            String::new()
        },
        width,
        height,
        format,
        data: data[data_off..data_off + data_length].to_vec(),
    })
}

/// CMDL model entry.
fn parse_model(data: &[u8], off: usize) -> Result<Model> {
    let mut r = Reader::new(data);
    r.seek(off)?;
    let flags = r.u32()?;
    let has_skeleton = flags & 0x80 > 0;
    r.magic(b"CMDL", "'CMDL'")?;
    let _revision = r.u32()?;
    let name_off = r.rel_u32()?;
    let _user_data_entries = r.u32()?;
    let _user_data_off = r.rel_u32()?;
    r.u32()?;
    let _flags2 = r.u32()?;
    let _child_count = r.u32()?;
    r.u32()?;
    let _anim_group_entries = r.u32()?;
    let _anim_group_off = r.rel_u32()?;
    let _scale = [r.f32()?, r.f32()?, r.f32()?];
    let _rotate = [r.f32()?, r.f32()?, r.f32()?];
    let _translate = [r.f32()?, r.f32()?, r.f32()?];
    let _local_matrix = read_matrix(&mut r)?;
    let world_matrix = read_matrix(&mut r)?;
    let object_entries = r.u32()? as usize;
    let object_table_off = r.rel_u32()?;
    let materials = inline_dict(&mut r)?;
    let shape_entries = r.u32()? as usize;
    let shape_table_off = r.rel_u32()?;
    let object_nodes = inline_dict(&mut r)?;
    let _flags3 = r.u32()?;
    let _cull_mode = r.u32()?;
    let _layer_id = r.u32()?;
    let skeleton_off = if has_skeleton { r.rel_u32()? } else { 0 };

    let mut model = Model {
        name: if name_off != 0 {
            r.cstr(name_off)?
        } else {
            String::new()
        },
        world_matrix,
        materials: Vec::new(),
        bones: Vec::new(),
        shapes: Vec::new(),
        meshes: Vec::new(),
        geometry_partial: false,
    };

    if std::env::var_os("XY_CGFX_DEBUG").is_some() {
        eprintln!(
            "[dbg] CMDL {:?}: obj={}@{:#x} mats={} shapes={}@{:#x} nodes={}",
            r.cstr(name_off),
            object_entries,
            object_table_off,
            materials.len(),
            shape_entries,
            shape_table_off,
            object_nodes.len()
        );
    }
    for (mi, (_, mat_off)) in materials.iter().enumerate() {
        if std::env::var_os("XY_CGFX_DEBUG").is_some() {
            eprintln!("[dbg] material {mi} @{mat_off:#x}");
        }
        model.materials.push(parse_material(data, *mat_off)?);
    }

    let mut skeleton_transforms: Vec<Mat4> = Vec::new();
    if has_skeleton && skeleton_off != 0 {
        model.bones = parse_skeleton(data, skeleton_off)?;
        for index in 0..model.bones.len() {
            let mut transform = Mat4::identity();
            transform_skeleton(&model.bones, index, &mut transform, 0)?;
            skeleton_transforms.push(transform);
        }
    }

    let empty_shape = || Shape {
        name: String::new(),
        position_offset: [0.0; 3],
        vertices: Vec::new(),
    };
    for i in 0..shape_entries {
        let entry_off = match rel_at(data, shape_table_off + i * 4) {
            Ok(off) if off != 0 => off,
            _ => {
                model.geometry_partial = true;
                model.shapes.push(empty_shape());
                continue;
            }
        };
        match parse_shape_header(data, entry_off)
            .and_then(|header| parse_shape(data, header, &skeleton_transforms))
        {
            Ok(shape) => model.shapes.push(shape),
            Err(e) => {
                // Unmapped shape variant (see `geometry_partial`).
                if std::env::var_os("XY_CGFX_DEBUG").is_some() {
                    eprintln!("[dbg] shape skipped: {e}");
                }
                model.geometry_partial = true;
                model.shapes.push(empty_shape());
            }
        }
    }

    let mut node_names: Vec<(String, bool)> = Vec::new();
    for (_, node_off) in &object_nodes {
        let mut nr = Reader::new(data);
        nr.seek(*node_off)?;
        let name_off = nr.rel_u32()?;
        let visible = nr.u32()? == 1;
        node_names.push((
            if name_off != 0 {
                nr.cstr(name_off)?
            } else {
                String::new()
            },
            visible,
        ));
    }
    for i in 0..object_entries {
        let entry_off = match rel_at(data, object_table_off + i * 4) {
            Ok(off) if off != 0 => off,
            _ => {
                model.geometry_partial = true;
                continue;
            }
        };
        let mut or = Reader::new(data);
        or.seek(entry_off)?;
        let _flags = or.u32()?;
        // XY's rev-9 effect models reference shapes through GfxMesh
        // objects (TypeId 0x01000000, SOBJ magic) whose layout differs
        // from the SDK MSOB the reference port expects.
        if or.magic(b"MSOB", "'MSOB'").is_err() {
            model.geometry_partial = true;
            continue;
        }
        let _rev = or.u32()?;
        let _name_off = or.rel_u32()?;
        or.u32()?; // userData entries
        or.rel_u32()?; // userData offset
        let shape_index = or.u32()? as usize;
        let material_id = or.u32()? as usize;
        or.rel_u32()?; // owner model offset
        let is_visible = or.u8()? & 1 > 0;
        let render_priority = or.u8()?;
        let node_visibility_index = or.u16()? as usize;

        let (name, node_visible) = if !node_names.is_empty() {
            node_names
                .get(node_visibility_index)
                .map(|&(ref n, v)| (n.clone(), v))
                .unwrap_or_default()
        } else {
            (
                model
                    .shapes
                    .get(shape_index)
                    .map(|s| s.name.clone())
                    .unwrap_or_default(),
                true,
            )
        };
        if shape_index >= model.shapes.len() {
            return Err(Error::invalid(
                "cgfx object",
                format!("shape index {shape_index} out of range"),
            ));
        }
        model.meshes.push(Mesh {
            name,
            is_visible: is_visible && node_visible,
            material_id,
            render_priority,
            shape_index,
        });
    }
    Ok(model)
}

/// MTOB material entry — colors + texture mapper bindings.
fn parse_material(data: &[u8], off: usize) -> Result<Material> {
    let mut r = Reader::new(data);
    r.seek(off)?;
    let _flags = r.u32()?;
    r.magic(b"MTOB", "'MTOB'")?;
    let _rev = r.u32()?;
    let name_off = r.rel_u32()?;
    let _user_data_entries = r.u32()?;
    let _user_data_off = r.rel_u32()?;
    let _lighting_flags = r.u32()?;
    let _tex_coord_config = r.u32()?;
    let _translucency = r.u32()?;

    // 11 float colors, then 11 byte colors (RGBA each).
    let mut _float_colors: Vec<[u8; 4]> = Vec::with_capacity(11);
    for _ in 0..11 {
        _float_colors.push([
            byte_of_float(r.f32()?),
            byte_of_float(r.f32()?),
            byte_of_float(r.f32()?),
            byte_of_float(r.f32()?),
        ]);
    }
    let mut byte_colors: Vec<[u8; 4]> = Vec::with_capacity(11);
    for _ in 0..11 {
        byte_colors.push([r.u8()?, r.u8()?, r.u8()?, r.u8()?]);
    }

    // Rasterization
    let _polygon_offset = r.u32()?;
    let _cull_mode = r.u32()?;
    let _polygon_offset_unit = r.f32()?;
    r.skip(0xc)?;

    // Fragment operations (PICA command buffers — advancement only)
    r.u32()?; // depth flags
    skip_pica(&mut r, 4)?;
    r.u32()?; // blend mode
    for _ in 0..4 {
        byte_of_float(r.f32()?); // blend color
    }
    skip_pica(&mut r, 5)?;
    r.u32()?; // stencil
    skip_pica(&mut r, 4)?;
    r.u32()?; // usedTextureCoordinates

    // Texture coordinators: 3 × {4 × u32 + 5 × f32 + u32 + 4×3 matrix}
    for _ in 0..3 {
        r.skip(4 * 4 + 5 * 4 + 4 + 12 * 4)?;
    }

    // Texture mappers
    if std::env::var_os("XY_CGFX_DEBUG").is_some() {
        eprintln!("[dbg]   mtof walk @ {:#x}", r.pos);
    }
    let mut mapper_offsets = [0usize; 4];
    for slot in mapper_offsets.iter_mut() {
        *slot = r.rel_u32()?;
    }
    let resume = r.pos;
    let mut texture_names = [None, None, None];
    for (i, &mapper_off) in mapper_offsets.iter().enumerate().take(3) {
        if mapper_off == 0 {
            continue;
        }
        let mut mr = Reader::new(data);
        mr.seek(mapper_off)?;
        let _flags = mr.u32()?;
        let _dynamic_allocator = mr.u32()?;
        let texture_header_off = mr.rel_u32()?;
        let _sampler_off = mr.rel_u32()?;
        skip_pica(&mut mr, 13)?;
        if texture_header_off != 0 {
            // Texture name at textureHeader + 0x18 (rel offset to string).
            let name_off = rel_at(data, texture_header_off + 0x18)?;
            if name_off != 0 {
                texture_names[i] = Some(mr.cstr(name_off)?);
            }
        }
    }
    r.seek(resume)?;

    let shader_off = r.rel_u32()?;
    let _fragment_shader_off = r.rel_u32()?;
    let _shader_program_index = r.u32()?;
    let _shader_param_count = r.u32()?;
    let _shader_param_table_off = r.rel_u32()?;
    let _light_set_index = r.u32()?;
    let _fog_index = r.u32()?;
    for _ in 0..17 {
        r.u32()?; // hashes + material id
    }

    let shader_reference = if shader_off != 0 {
        let mut sr = Reader::new(data);
        sr.seek(shader_off)?;
        sr.u32()?; // flags
        sr.magic(b"SHDR", "'SHDR'")?;
        sr.u32()?; // revision
        let name_off = sr.rel_u32()?;
        sr.u32()?; // userData entries
        sr.rel_u32()?; // userData offset
        let ref_off = sr.rel_u32()?;
        Some((
            if name_off != 0 {
                sr.cstr(name_off)?
            } else {
                String::new()
            },
            if ref_off != 0 {
                sr.cstr(ref_off)?
            } else {
                String::new()
            },
        ))
    } else {
        None
    };

    let pick = |i: usize| -> [u8; 4] { byte_colors.get(i).copied().unwrap_or([0xff; 4]) };
    Ok(Material {
        name: if name_off != 0 {
            r.cstr(name_off)?
        } else {
            String::new()
        },
        emission: pick(0),
        ambient: pick(1),
        diffuse: pick(2),
        specular0: pick(3),
        specular1: pick(4),
        texture_names,
        shader_reference,
    })
}

fn byte_of_float(v: f32) -> u8 {
    // MeshUtils.getColorFloat: (byte)(f * 0xff) with saturate semantics.
    (v * 255.0).clamp(0.0, 255.0) as u8
}

fn read_matrix(r: &mut Reader) -> Result<Mat4> {
    // File order: column-major 4×3 (M11,M21,M31,M41, M12,M22,M32,M42, …).
    let mut m = Mat4::identity();
    for col in 0..3 {
        for row in 0..4 {
            m.0[row][col] = r.f32()?;
        }
    }
    Ok(m)
}

/// SOBJ skeleton chunk (own bone dictionary).
fn parse_skeleton(data: &[u8], off: usize) -> Result<Vec<Bone>> {
    let mut r = Reader::new(data);
    r.seek(off)?;
    let _flags = r.u32()?;
    r.magic(b"SOBJ", "'SOBJ'")?;
    let _rev = r.u32()?;
    let _name_off = r.rel_u32()?;
    r.u32()?;
    r.u32()?;
    let bones = inline_dict(&mut r)?;
    let _root_bone_off = r.rel_u32()?;
    let _scaling_rule = r.u32()?;
    let _translate_anim = r.u32()?;

    let mut out = Vec::with_capacity(bones.len());
    for (_, bone_off) in bones {
        let mut br = Reader::new(data);
        br.seek(bone_off)?;
        let name_off = br.rel_u32()?;
        let _bone_flags = br.u32()?;
        let _bone_id = br.u32()?;
        let parent_id = br.i32()?;
        br.i32()?; // parent offset
        br.i32()?; // child offset
        br.i32()?; // previous sibling offset
        br.i32()?; // next sibling offset
        let scale = [br.f32()?, br.f32()?, br.f32()?];
        let rotation = [br.f32()?, br.f32()?, br.f32()?];
        let translation = [br.f32()?, br.f32()?, br.f32()?];
        read_matrix(&mut br)?; // local
        read_matrix(&mut br)?; // world
        read_matrix(&mut br)?; // inverse base
        br.i32()?; // billboard mode
        br.u32()?; // userData entries
        br.rel_u32()?; // userData offset
        out.push(Bone {
            name: if name_off != 0 {
                br.cstr(name_off)?
            } else {
                String::new()
            },
            parent_id,
            scale,
            rotation,
            translation,
        });
    }
    Ok(out)
}

/// `transformSkeleton`: target = target * S * Rx * Ry * Rz * T, recursing
/// to the parent (depth-capped to survive malformed parent cycles).
fn transform_skeleton(bones: &[Bone], index: usize, target: &mut Mat4, depth: usize) -> Result<()> {
    if depth > bones.len() {
        return Err(Error::invalid("cgfx skeleton", "parent chain cycle"));
    }
    let b = &bones[index];
    *target = target
        .mul(&Mat4::scale(b.scale))
        .mul(&Mat4::rotate_x(b.rotation[0]))
        .mul(&Mat4::rotate_y(b.rotation[1]))
        .mul(&Mat4::rotate_z(b.rotation[2]))
        .mul(&Mat4::translate(b.translation));
    let parent = b.parent_id as usize;
    if b.parent_id > -1 && parent < bones.len() {
        transform_skeleton(bones, parent, target, depth + 1)?;
    }
    Ok(())
}

struct ShapeHeader {
    name: String,
    position_offset: [f32; 3],
    faces_group_offset: usize,
    faces_group_entries: usize,
    vertex_group_offset: usize,
}

fn parse_shape_header(data: &[u8], off: usize) -> Result<ShapeHeader> {
    let mut r = Reader::new(data);
    r.seek(off)?;
    let _flags = r.u32()?;
    r.magic(b"SOBJ", "'SOBJ'")?;
    let _rev = r.u32()?;
    let name_off = r.rel_u32()?;
    r.u32()?; // userData entries
    r.rel_u32()?; // userData offset
    r.u32()?; // flags
    r.rel_u32()?; // bounding box offset
    let position_offset = [r.f32()?, r.f32()?, r.f32()?];
    let faces_group_entries = r.u32()? as usize;
    let faces_group_offset = r.rel_u32()?;
    r.u32()?;
    let _vertex_group_entries = r.u32()?;
    let vertex_group_offset = r.rel_u32()?;
    let _blend_shape_off = r.rel_u32()?;
    Ok(ShapeHeader {
        name: if name_off != 0 {
            r.cstr(name_off)?
        } else {
            String::new()
        },
        position_offset,
        faces_group_offset,
        faces_group_entries,
        vertex_group_offset,
    })
}

#[derive(Clone, Copy)]
struct AttributeFormat {
    attribute: u32,
    ty: u32,
    length: usize,
    scale: f32,
    offset: usize,
}

/// SOBJ shape → vertices (the VERT buffer + faces pipeline).
fn parse_shape(data: &[u8], header: ShapeHeader, skeleton_transforms: &[Mat4]) -> Result<Shape> {
    let mut shape = Shape {
        name: header.name.clone(),
        position_offset: header.position_offset,
        vertices: Vec::new(),
    };

    // Vertex group: vertexGroupOffset → rel → VERT descriptor.
    let vert_group_off = rel_at(data, header.vertex_group_offset)?;
    if std::env::var_os("XY_CGFX_DEBUG").is_some() {
        eprintln!(
            "[dbg] shape {:?}: vert_group {:#x} (from {:#x}), faces {}@{:#x}",
            header.name,
            vert_group_off,
            header.vertex_group_offset,
            header.faces_group_entries,
            header.faces_group_offset
        );
    }
    if vert_group_off == 0 {
        return Err(Error::invalid("cgfx shape", "missing vertex group"));
    }
    let mut r = Reader::new(data);
    r.seek(vert_group_off)?;
    r.u32()?; // name offset (unused)
    r.u32()?; // userData entries
    r.u32()?; // userData offset
    r.u32()?; // buffer object
    r.u32()?; // location flag
    let _buffer_length = r.u32()?;
    let buffer_offset = r.rel_u32()?;
    r.u32()?; // location address
    r.u32()?; // memory area
    let stride = r.u32()? as usize;
    let components_entries = r.u32()? as usize;
    let components_offset = r.rel_u32()?;

    if buffer_offset == 0 || stride == 0 {
        return Err(Error::invalid("cgfx shape", "missing vertex buffer"));
    }
    if std::env::var_os("XY_CGFX_DEBUG").is_some() {
        eprintln!(
            "[dbg]   buffer {buffer_offset:#x} stride {stride} comps {}@{components_offset:#x}",
            components_entries
        );
    }

    let mut formats = Vec::with_capacity(components_entries);
    for i in 0..components_entries {
        let comp_off = rel_at(data, components_offset + i * 4)?;
        if comp_off == 0 {
            continue;
        }
        let mut cr = Reader::new(data);
        cr.seek(comp_off)?;
        cr.u32()?; // flags
        let attribute = cr.u32()?;
        cr.u32()?; // interleaved flag
        cr.u32()?; // buffer object
        cr.u32()?; // location flag
        cr.u32()?; // stream length
        cr.rel_u32()?; // stream offset
        cr.u32()?; // location address
        cr.u32()?; // memory area
        let ty = cr.u32()? & 0xf;
        let length = cr.u32()? as usize;
        let scale = cr.f32()?;
        let offset = cr.u32()? as usize;
        formats.push(AttributeFormat {
            attribute,
            ty,
            length,
            scale,
            offset,
        });
    }

    for face_index in 0..header.faces_group_entries {
        let face_off = rel_at(data, header.faces_group_offset + face_index * 4)?;
        if face_off == 0 {
            continue;
        }
        let mut fr = Reader::new(data);
        fr.seek(face_off)?;
        let node_list_entries = fr.u32()? as usize;
        let node_list_off = fr.rel_u32()?;
        let skinning_mode = fr.u32()?;
        let _face_main_entries = fr.u32()?;
        let face_main_off = fr.rel_u32()?;

        let mut node_list = Vec::with_capacity(node_list_entries);
        if node_list_off != 0 {
            let mut nr = Reader::new(data);
            nr.seek(node_list_off)?;
            for _ in 0..node_list_entries {
                node_list.push(nr.u32()?);
            }
        }

        let face_hdr_off = rel_at(data, face_main_off)?;
        let mut hr = Reader::new(data);
        hr.seek(face_hdr_off)?;
        let _descriptor_entries = hr.u32()?;
        let descriptor_off = hr.rel_u32()?;
        hr.u32()?;
        hr.u32()?;
        hr.u32()?;

        let idx_hdr_off = rel_at(data, descriptor_off)?;
        let mut ir = Reader::new(data);
        ir.seek(idx_hdr_off)?;
        let index_format = (ir.u32()? & 2) >> 1; // 1 = u16, 0 = u8
        ir.u32()?;
        let idx_length = ir.u32()? as usize;
        let idx_offset = ir.rel_u32()?;
        if idx_offset == 0 {
            return Err(Error::invalid("cgfx shape", "missing index buffer"));
        }

        let mut index = 0usize;
        while index < idx_length {
            let vidx: usize = if index_format == 1 {
                index += 2;
                u16_le(data, idx_offset + index - 2)? as usize
            } else {
                index += 1;
                xy_core::util::u8_at(data, idx_offset + index - 1)? as usize
            };
            let vertex_offset = buffer_offset + vidx * stride;

            let mut vertex = Vertex {
                color: [0xff; 4],
                ..Vertex::default()
            };
            for format in &formats {
                let mut ty = format.ty;
                if format.attribute == 8 {
                    ty = 1; // boneWeight is always read as u8
                }
                let vec = read_vector(data, vertex_offset + format.offset, ty, format.length)?;
                match format.attribute {
                    0 => {
                        vertex.position = [
                            vec[0] * format.scale + shape.position_offset[0],
                            vec[1] * format.scale + shape.position_offset[1],
                            vec[2] * format.scale + shape.position_offset[2],
                        ];
                    }
                    1 => {
                        vertex.normal = [
                            vec[0] * format.scale,
                            vec[1] * format.scale,
                            vec[2] * format.scale,
                        ];
                    }
                    2 => {
                        vertex.tangent = [
                            vec[0] * format.scale,
                            vec[1] * format.scale,
                            vec[2] * format.scale,
                        ];
                    }
                    3 => {
                        vertex.color = [
                            saturate(vec[0] * format.scale * 255.0),
                            saturate(vec[1] * format.scale * 255.0),
                            saturate(vec[2] * format.scale * 255.0),
                            saturate(vec[3] * format.scale * 255.0),
                        ];
                    }
                    4..=6 => {
                        let uv = &mut vertex.uv[(format.attribute - 4) as usize];
                        uv[0] = vec[0] * format.scale;
                        uv[1] = vec[1] * format.scale;
                    }
                    7 => {
                        if (vec[0] as usize) < node_list.len() && format.length > 0 {
                            vertex.nodes.push(node_list[vec[0] as usize]);
                        }
                        if skinning_mode == 2 {
                            if (vec[1] as usize) < node_list.len() && format.length > 1 {
                                vertex.nodes.push(node_list[vec[1] as usize]);
                            }
                            if (vec[2] as usize) < node_list.len() && format.length > 2 {
                                vertex.nodes.push(node_list[vec[2] as usize]);
                            }
                            if (vec[3] as usize) < node_list.len() && format.length > 3 {
                                vertex.nodes.push(node_list[vec[3] as usize]);
                            }
                        }
                    }
                    8 => {
                        if format.length > 0 {
                            vertex.weights.push(vec[0] * format.scale);
                        }
                        if skinning_mode == 2 {
                            if format.length > 1 {
                                vertex.weights.push(vec[1] * format.scale);
                            }
                            if format.length > 2 {
                                vertex.weights.push(vec[2] * format.scale);
                            }
                            if format.length > 3 {
                                vertex.weights.push(vec[3] * format.scale);
                            }
                        }
                    }
                    _ => {}
                }
            }

            // ≤4 bones in the node list: use all of them (weight 1 if absent).
            if vertex.nodes.is_empty() && node_list.len() <= 4 {
                for &n in &node_list {
                    vertex.nodes.push(n);
                }
                if vertex.weights.is_empty() {
                    vertex.weights.push(1.0);
                }
            }

            if skinning_mode != 2 && !vertex.nodes.is_empty() {
                // Rigid skinning: transform by the single bone.
                let node = vertex.nodes[0] as usize;
                if node < skeleton_transforms.len() {
                    if vertex.weights.is_empty() {
                        vertex.weights.push(1.0);
                    }
                    vertex.position = skeleton_transforms[node].transform(vertex.position);
                }
            }

            shape.vertices.push(vertex);
        }
    }
    Ok(shape)
}

fn saturate(v: f32) -> u8 {
    // MeshUtils.saturate
    if v > 255.0 {
        255
    } else if v < 0.0 {
        0
    } else {
        v as u8
    }
}

/// `getVector`: reads `length` components of the given type.
fn read_vector(data: &[u8], off: usize, ty: u32, length: usize) -> Result<[f32; 4]> {
    let mut out = [0f32; 4];
    let mut pos = off;
    if length > 4 {
        return Err(Error::invalid(
            "cgfx attribute",
            format!("{length} components"),
        ));
    }
    for slot in out.iter_mut().take(length) {
        match ty {
            0 => {
                *slot = xy_core::util::u8_at(data, pos)? as i8 as f32;
                pos += 1;
            }
            1 => {
                *slot = xy_core::util::u8_at(data, pos)? as f32;
                pos += 1;
            }
            2 => {
                *slot = u16_le(data, pos)? as i16 as f32;
                pos += 2;
            }
            6 => {
                *slot = f32_le(data, pos)?;
                pos += 4;
            }
            other => return Err(Error::invalid("cgfx attribute type", format!("{other:#x}"))),
        }
    }
    Ok(out)
}

/// CANM skeletal animation entry.
fn parse_skeletal_animation(data: &[u8], off: usize) -> Result<SkeletalAnimation> {
    let mut r = Reader::new(data);
    r.seek(off)?;
    r.magic(b"CANM", "'CANM'")?;
    let revision = r.u32()?;
    let name_off = r.rel_u32()?;
    let target_off = r.rel_u32()?;
    let loop_mode = r.u32()?;
    let frame_size = r.f32()?;
    let members = inline_dict(&mut r)?;
    r.u32()?; // userData entries
    r.rel_u32()?; // userData offset

    let mut anim = SkeletalAnimation {
        name: if name_off != 0 {
            r.cstr(name_off)?
        } else {
            String::new()
        },
        target: if target_off != 0 {
            r.cstr(target_off)?
        } else {
            String::new()
        },
        loop_mode,
        frame_size,
        bones: Vec::with_capacity(members.len()),
    };

    for (bone_name, bone_off) in members {
        let mut br = Reader::new(data);
        br.seek(bone_off)?;
        let bone_flags = br.u32()?;
        let _path_off = br.rel_u32()?;
        if (revision >> 24) < 7 {
            br.skip(8)?;
        }
        let segment_type = br.u32()?;

        let mut bone = AnimatedBone {
            name: bone_name,
            ..Default::default()
        };
        match segment_type {
            5 => {
                // Euler transform: rotation xyz, then translation xyz
                br.skip(0xc)?;
                let mut not_exist_mask: u32 = 0x80000;
                let mut constant_mask: u32 = 0x200;
                for j in 0..2 {
                    for axis in 0..3 {
                        let not_exist = bone_flags & not_exist_mask > 0;
                        let constant = bone_flags & constant_mask > 0;
                        let mut frame = KeyFrameGroup::default();
                        if !not_exist {
                            if constant {
                                frame.interpolation = 4; // linear
                                let value = br.f32()?;
                                frame.key_frames.push(KeyFrame {
                                    frame: 0.0,
                                    value,
                                    in_slope: 0.0,
                                    out_slope: 0.0,
                                });
                            } else {
                                let frame_off = br.rel_u32()?;
                                let resume = br.pos;
                                br.seek(frame_off)?;
                                read_key_frames(&mut br, &mut frame)?;
                                br.seek(resume)?;
                            }
                        } else {
                            br.skip(4)?;
                        }
                        if j == 0 {
                            bone.rotation[axis] = frame;
                        } else {
                            bone.translation[axis] = frame;
                        }
                        not_exist_mask <<= 1;
                        constant_mask <<= 1;
                    }
                    not_exist_mask <<= 1;
                    constant_mask <<= 1;
                    br.skip(4)?;
                }
            }
            8 => {
                // Quaternion transform: scale / rotation / translation tracks
                let rotation_off = br.rel_u32()?;
                let translation_off = br.rel_u32()?;
                let scale_off = br.rel_u32()?;

                if bone_flags & 0x20 == 0 && scale_off != 0 {
                    br.seek(scale_off)?;
                    bone.quat_scale = Some(read_vec_track(&mut br, 3)?);
                }
                if bone_flags & 0x10 == 0 && rotation_off != 0 {
                    br.seek(rotation_off)?;
                    bone.quat_rotation = Some(read_vec_track(&mut br, 4)?);
                }
                if bone_flags & 8 == 0 && translation_off != 0 {
                    br.seek(translation_off)?;
                    bone.quat_translation = Some(read_vec_track(&mut br, 3)?);
                }
            }
            _ => {}
        }
        anim.bones.push(bone);
    }
    Ok(anim)
}

/// Quaternion-mode track payload: start/end frames, then either one
/// constant vector (`comps` floats) or `(end - start)` vectors of
/// `comps` floats each followed by a u32.
fn read_vec_track(r: &mut Reader, comps: usize) -> Result<Vec4Track> {
    let start = r.f32()?;
    let end = r.f32()?;
    r.u32()?;
    let constant_flags = r.u32()?;
    let mut track = Vec4Track {
        start_frame: start,
        end_frame: end,
        vectors: Vec::new(),
    };
    let push = |r: &mut Reader, out: &mut Vec<[f32; 4]>| -> Result<()> {
        let mut v = [0f32; 4];
        for slot in v.iter_mut().take(comps) {
            *slot = r.f32()?;
        }
        out.push(v);
        Ok(())
    };
    if constant_flags & 1 > 0 {
        push(r, &mut track.vectors)?;
    } else {
        let entries = (end - start) as usize;
        for _ in 0..entries {
            push(r, &mut track.vectors)?;
            r.u32()?;
        }
    }
    Ok(track)
}

/// `getAnimationKeyFrame` — quantization values per OSegmentQuantization:
/// 0 hermite128, 1 hermite64, 2 hermite48, 3 unifiedHermite96,
/// 4 unifiedHermite48, 5 unifiedHermite32, 6 stepLinear64, 7 stepLinear32.
fn read_key_frames(r: &mut Reader, frame: &mut KeyFrameGroup) -> Result<()> {
    let start_frame = r.f32()?;
    let end_frame = r.f32()?;
    r.skip(0x10)?;
    let _group_start = r.f32()?;
    let _group_end = r.f32()?;
    let segment_flags = r.u32()?;
    frame.interpolation = match segment_flags & 0xf {
        0 => 0, // step
        4 => 4, // linear
        8 => 8, // hermite
        other => other as u8,
    };
    let quantization = segment_flags >> 5;
    let entries = r.u32()?;
    let _inv_duration = r.f32()?;
    let mut value_scale = 1f32;
    let mut value_offset = 0f32;
    let mut frame_scale = 1f32;
    if !matches!(quantization, 0 | 3 | 6) {
        // hermite128 / unifiedHermite96 / stepLinear64 carry no scale block
        value_scale = r.f32()?;
        value_offset = r.f32()?;
        frame_scale = r.f32()?;
    }

    for _ in 0..entries {
        let mut kf = KeyFrame {
            frame: 0.0,
            value: 0.0,
            in_slope: 0.0,
            out_slope: 0.0,
        };
        match quantization {
            0 => {
                // hermite128
                kf.frame = r.f32()?;
                kf.value = r.f32()?;
                kf.in_slope = r.f32()?;
                kf.out_slope = r.f32()?;
            }
            1 => {
                // hermite64
                let v = r.u32()?;
                kf.frame = (v & 0xfff) as f32;
                kf.value = (v >> 12) as f32;
                kf.in_slope = r.u16()? as i16 as f32 / 256.0;
                kf.out_slope = r.u16()? as i16 as f32 / 256.0;
            }
            2 => {
                // hermite48
                kf.frame = r.u8()? as f32;
                kf.value = r.u16()? as f32;
                let s0 = r.u8()? as u32;
                let s1 = r.u8()? as u32;
                let s2 = r.u8()? as u32;
                kf.in_slope = sign_extend(s0 | ((s1 & 0xf) << 8), 12) as f32 / 32.0;
                kf.out_slope = sign_extend((s1 >> 4) | (s2 << 4), 12) as f32 / 32.0;
            }
            3 => {
                // unifiedHermite96
                kf.frame = r.f32()?;
                kf.value = r.f32()?;
                kf.in_slope = r.f32()?;
                kf.out_slope = kf.in_slope;
            }
            4 => {
                // unifiedHermite48
                kf.frame = r.u16()? as f32 / 32.0;
                kf.value = r.u16()? as f32;
                kf.in_slope = r.u16()? as i16 as f32 / 256.0;
                kf.out_slope = kf.in_slope;
            }
            5 => {
                // unifiedHermite32
                kf.frame = r.u8()? as f32;
                let v = r.u16()? as u32;
                kf.value = (v & 0xfff) as f32;
                kf.in_slope = sign_extend((v >> 12) | ((r.u8()? as u32) << 4), 12) as f32 / 32.0;
                kf.out_slope = kf.in_slope;
            }
            6 => {
                // stepLinear64
                kf.frame = r.f32()?;
                kf.value = r.f32()?;
            }
            7 => {
                // stepLinear32
                let v = r.u32()?;
                kf.frame = (v & 0xfff) as f32;
                kf.value = (v >> 12) as f32;
            }
            other => {
                return Err(Error::invalid(
                    "cgfx animation",
                    format!("quantization {other}"),
                ))
            }
        }
        kf.frame = kf.frame * frame_scale + start_frame;
        kf.value = kf.value * value_scale + value_offset;
        frame.key_frames.push(kf);
    }
    let _ = end_frame;
    Ok(())
}

fn sign_extend(value: u32, bits: i32) -> i32 {
    let output = value as i32;
    let sign = value & (1 << (bits - 1)) > 0;
    if sign {
        output - (1 << bits)
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_bad_magic() {
        assert!(Cgfx::parse(b"NOPE").is_err());
    }

    #[test]
    fn matrix_transform_matches_ohana_formula() {
        let t = Mat4::translate([1.0, 2.0, 3.0]);
        assert_eq!(t.transform([1.0, 1.0, 1.0]), [2.0, 3.0, 4.0]);
        let s = Mat4::scale([2.0, 3.0, 4.0]);
        assert_eq!(s.transform([1.0, 1.0, 1.0]), [2.0, 3.0, 4.0]);
        // rotate_x(π/2): y → z per Ohana's matrix layout
        let rx = Mat4::rotate_x(std::f32::consts::FRAC_PI_2);
        let out = rx.transform([0.0, 1.0, 0.0]);
        assert!((out[2] - 1.0).abs() < 1e-6, "y maps to z: {out:?}");
        assert!((out[1] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn sign_extend_negative() {
        assert_eq!(sign_extend(0xfffu32 & 0xfff, 12), -1);
        assert_eq!(sign_extend(0x7ff, 12), 0x7ff);
    }
}
