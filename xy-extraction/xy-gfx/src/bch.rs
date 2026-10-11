//! BCH (CtrH3D) decoder — port of SPICA's `Formats/CtrH3D` (gdkchan), the
//! reference implementation cloned to `/tmp` during development and validated
//! against the real Pokémon X assets (Bulbasaur `pm0001_00`).
//!
//! XY stores character/world models as BCH: raw `BCH\0` files and BCH
//! entries inside PC/PF/PT containers (`a/0/0/7/0004.bin` entries 0-1 =
//! high/low-poly Bulbasaur).
//!
//! Binary layout (all pointers are 32-bit; after relocation they are
//! absolute file offsets; verified against `a/0/0/7/0004.bin` entry 0 with
//! an instrumented SPICA run + hexdumps, 2026-10-06):
//!
//! ```text
//! Header (v7 — XY's version; v>=0x21 adds RawExt addr/len, v>=8 adds
//! flags+address-count u16s at the end):
//! 0x00 "BCH\0" | u8 backward-compat | u8 forward-compat | u16 converter
//! 0x08 i32 contents/string/commands/raw-data/relocation addresses
//! 0x1C i32 contents/string/commands/raw-data/relocation lengths
//! 0x30 i32 uninit-data / uninit-commands lengths
//!
//! Relocation table (RelocationLength/4 × u32): value = (target << 25) |
//! (source << 29) | ptr25. Target/Source are section ids (0 contents,
//! 1 strings, 2-3 commands, 4-8 raw data, 9-13 raw-ext, 14 base).
//! For each entry: field = base(source) + (ptr25 << 2 unless target ==
//! strings); *field += base(target). Legacy diffs: bc>7 && bc<0x21 &&
//! target>=RawDataVertex(6) → -1; bc<7 && target>=CommandsSrc(3) → +1.
//! (base(RawDataIndex16) = raw-data | 1<<31 — marks 16-bit index buffers.)
//!
//! Reference conventions:
//! - list fields: { u32 ptr, i32 count } (pointer first)
//! - single-object fields: { u32 ptr } only
//! - [Inline] classes serialize in place: dict = { ptr, count, treeptr };
//!   materials/meshes/submeshes/bones are inline objects; models,
//!   textures, animations, material-params are separate ref'd objects
//!   packed sequentially in the contents section.
//!
//! DICT (H3DDict): { ptr, count } + { treeptr }; values: models = count ×
//! {u32 ptr} refs, materials/meshes/bones = inline objects.
//! Root = 15 dicts: models, materials, shaders, textures, luts, lights,
//! cameras, fogs, skeletal-anims, material-anims, visibility-anims,
//! light-anims, camera-anims, fog-anims, scenes.
//!
//! H3DModel: flags u8, bone-scaling u8, silhouette-count u16,
//! world-transform 3×4 f32, materials DICT, meshes list, 4 layer lists
//! ([Range] — end-position delimiters, redundant), submesh cullings,
//! skeleton DICT, mesh-node-visibility list (bit-packed bools), name,
//! mesh-nodes-count i32, mesh-nodes-tree, user-address u32, meta.
//!
//! H3DMaterial (inline, 0x58 B): params {ptr}, tex0-2 {ptr}, tex-commands
//! {ptr,len}, 3 × 16 B mappers, tex0-2 name {ptr}, name {ptr}.
//! H3DMaterialParams (ref): unique-id u32, flags u16, fragment-flags u16,
//! frame-access u8, shader {ptr}, 3 × 24 B texcoords, light-set u16,
//! fog u16, then 11 RGBA u32 (emission, ambient, diffuse, specular0/1,
//! constant0-5) at +0x58, blend color, color-scale f32, ...
//!
//! H3DMesh (inline, 0x38 B): material-index u16, flags u8 (mesh type 0-1,
//! skinning 2-3), node-index u16, key u16 (priority 0-7, layer 8-9),
//! enable-commands {ptr,len}, submeshes {ptr,len} (inline), disable-
//! commands {ptr,len}, center vec3, parent {ptr}, user u32, meta {ptr}.
//! H3DSubMesh (inline, 0x34 B): skinning u8, bone-count u16, 20 × u16
//! bone indices, commands {ptr,len} (PICA: index buffer config 0x227 —
//! bit31 = 16-bit, value = absolute address post-relocation; vertex
//! count 0x228; primitive mode = 0x25e >> 8).
//!
//! PICA attribute commands (0x201/2 formats, 0x203 buffer address,
//! 0x204/5 buffer-0 permutation + stride/attr-count, 0x232-5 fixed
//! attributes, 0x242 total attributes, 0x2bb/c permutation, 0x2c0-8
//! float uniforms; uniform 6 = position offset, 7 = pos/normal/tangent/
//! color scales, 8 = uv0/uv1/uv2/weight scales — float24 packed).
//!
//! H3DTexture (ref): 3 × commands {ptr,len}, format u8 (PICA format ids,
//! same table as [`crate::texture::TextureFormat`]), mipmap u8, name
//! {ptr}; dimensions from TEXUNIT0_DIM (0x082: h | w << 16), data
//! pointers from TEXUNIT0_ADDR1..6 (0x085..0x08a, relocated), data
//! length = w × h × bpp(format).
//! ```

use crate::texture::TextureFormat;
use xy_core::error::{Error, Result};
use xy_core::util::{f32_le, i32_le, u16_le, u32_le, u8_at};

// ---- PICA registers (subset) ----
const REG_TEXUNIT0_DIM: u16 = 0x082;
const REG_TEXUNIT0_ADDR1: u16 = 0x085;
const REG_ATTRIBBUFFERS_FORMAT_LOW: u16 = 0x201;
const REG_ATTRIBBUFFERS_FORMAT_HIGH: u16 = 0x202;
const REG_ATTRIBBUFFER0_OFFSET: u16 = 0x203;
const REG_ATTRIBBUFFER0_CONFIG1: u16 = 0x204;
const REG_ATTRIBBUFFER0_CONFIG2: u16 = 0x205;
const REG_FIXEDATTRIB_INDEX: u16 = 0x232;
const REG_FIXEDATTRIB_DATA0: u16 = 0x233;
const REG_INDEXBUFFER_CONFIG: u16 = 0x227;
const REG_NUMVERTICES: u16 = 0x228;
const REG_PRIMITIVE_CONFIG: u16 = 0x25e;
const REG_VSH_NUM_ATTR: u16 = 0x242;
const REG_VSH_PERMUTATION_LOW: u16 = 0x2bb;
const REG_VSH_PERMUTATION_HIGH: u16 = 0x2bc;
const REG_VSH_FLOATUNIFORM_INDEX: u16 = 0x2c0;
const REG_VSH_FLOATUNIFORM_DATA0: u16 = 0x2c1;
const REG_VSH_FLOATUNIFORM_DATA7: u16 = 0x2c8;

/// Attribute names (PICAAttributeName — same ids as the CGFX numbering).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrName {
    Position,
    Normal,
    Tangent,
    Color,
    TexCoord0,
    TexCoord1,
    TexCoord2,
    BoneIndex,
    BoneWeight,
    Other(u8),
}

impl AttrName {
    fn from_id(id: u8) -> Self {
        match id {
            0 => Self::Position,
            1 => Self::Normal,
            2 => Self::Tangent,
            3 => Self::Color,
            4 => Self::TexCoord0,
            5 => Self::TexCoord1,
            6 => Self::TexCoord2,
            7 => Self::BoneIndex,
            8 => Self::BoneWeight,
            other => Self::Other(other),
        }
    }
}

// ---- reader over a relocated buffer ----

struct Rdr<'a> {
    d: &'a [u8],
    pos: usize,
}

impl<'a> Rdr<'a> {
    fn new(d: &'a [u8]) -> Self {
        Self { d, pos: 0 }
    }
    fn seek(&mut self, pos: usize) -> Result<()> {
        if pos > self.d.len() {
            return Err(Error::truncated(pos, 0, "bch seek"));
        }
        self.pos = pos;
        Ok(())
    }
    fn u8(&mut self) -> Result<u8> {
        let v = u8_at(self.d, self.pos)?;
        self.pos += 1;
        Ok(v)
    }
    fn u16(&mut self) -> Result<u16> {
        let v = u16_le(self.d, self.pos)?;
        self.pos += 2;
        Ok(v)
    }
    fn u32(&mut self) -> Result<u32> {
        let v = u32_le(self.d, self.pos)?;
        self.pos += 4;
        Ok(v)
    }
    fn i32(&mut self) -> Result<i32> {
        let v = i32_le(self.d, self.pos)?;
        self.pos += 4;
        Ok(v)
    }
    fn f32(&mut self) -> Result<f32> {
        let v = f32_le(self.d, self.pos)?;
        self.pos += 4;
        Ok(v)
    }
    fn skip(&mut self, n: usize) -> Result<()> {
        self.seek(self.pos + n)
    }
    /// NUL-terminated ASCII at an absolute offset.
    fn cstr(&self, at: usize) -> Result<String> {
        if at >= self.d.len() {
            return Err(Error::truncated(at, 1, "bch string"));
        }
        let end = self.d[at..]
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| Error::truncated(at, 0, "bch string (unterminated)"))?;
        Ok(String::from_utf8_lossy(&self.d[at..at + end]).into_owned())
    }
    fn ptr(&mut self) -> Result<usize> {
        Ok(self.u32()? as usize)
    }
    /// List reference: { u32 ptr, i32 count }.
    fn list_ref(&mut self) -> Result<(usize, usize)> {
        let ptr = self.ptr()?;
        let count = self.i32()?;
        Ok((ptr, count.max(0) as usize))
    }
}

// ---- relocation ----

fn section_base(header: &Header, raw_ext: i32, section: u32) -> i32 {
    match section {
        0 => header.contents_addr,
        1 => header.strings_addr,
        2 | 3 => header.commands_addr,
        4 | 5 | 6 | 8 => header.raw_data_addr,
        7 => header.raw_data_addr | (1 << 31),
        9..=13 => raw_ext,
        _ => 0,
    }
}

fn legacy_diff(section: u32, bc: u8) -> i32 {
    if (8..0x21).contains(&bc) && section >= 6 {
        -1
    } else if bc < 7 && section >= 3 {
        1
    } else {
        0
    }
}

struct Header {
    bc: u8,
    #[allow(dead_code)] // layout documentation
    fc: u8,
    contents_addr: i32,
    strings_addr: i32,
    commands_addr: i32,
    raw_data_addr: i32,
    raw_ext_addr: i32,
    relocation_addr: i32,
    #[allow(dead_code)] // layout documentation
    contents_len: i32,
    #[allow(dead_code)]
    strings_len: i32,
    #[allow(dead_code)]
    commands_len: i32,
    #[allow(dead_code)]
    raw_data_len: i32,
    #[allow(dead_code)]
    raw_ext_len: i32,
    #[allow(dead_code)]
    relocation_len: i32,
}

fn parse_header(d: &[u8]) -> Result<Header> {
    if d.len() < 0x38 {
        return Err(Error::truncated(0, 0x38, "bch header"));
    }
    if &d[0..4] != b"BCH\0" {
        return Err(Error::magic(0, "'BCH\\0'", &d[0..4]));
    }
    let bc = u8_at(d, 4)?;
    let fc = u8_at(d, 5)?;
    let ge_0x21 = (bc as i32) >= 0x21 || (fc as i32) >= 0x21;
    let mut o = 8usize;
    let contents_addr = i32_le(d, o)?;
    let strings_addr = i32_le(d, o + 4)?;
    let commands_addr = i32_le(d, o + 8)?;
    let raw_data_addr = i32_le(d, o + 12)?;
    o += 16;
    let raw_ext_addr = if ge_0x21 {
        let v = i32_le(d, o)?;
        o += 4;
        v
    } else {
        0
    };
    let relocation_addr = i32_le(d, o)?;
    o += 4;
    let contents_len = i32_le(d, o)?;
    let strings_len = i32_le(d, o + 4)?;
    let commands_len = i32_le(d, o + 8)?;
    let raw_data_len = i32_le(d, o + 12)?;
    o += 16;
    let raw_ext_len = if ge_0x21 {
        let v = i32_le(d, o)?;
        o += 4;
        v
    } else {
        0
    };
    let relocation_len = i32_le(d, o)?;
    Ok(Header {
        bc,
        fc,
        contents_addr,
        strings_addr,
        commands_addr,
        raw_data_addr,
        raw_ext_addr,
        relocation_addr,
        contents_len,
        strings_len,
        commands_len,
        raw_data_len,
        raw_ext_len,
        relocation_len,
    })
}

/// Apply the relocation table in place (SPICA `H3DRelocator.ToAbsolute`).
fn relocate(d: &mut [u8], header: &Header) -> Result<()> {
    let mut off = header.relocation_addr as usize;
    let end = off + header.relocation_len as usize;
    while off + 4 <= end.min(d.len()) {
        let v = u32_le(d, off)?;
        off += 4;
        let mut ptr = (v & 0x1ff_ffff) as usize;
        let target = ((v >> 25) & 0xf) as i32;
        let source = (v >> 29) as i32;
        let target = target + legacy_diff(target as u32, header.bc);
        if target != 1 {
            ptr <<= 2;
        }
        let addr = (section_base(header, header.raw_ext_addr, source as u32) as u32)
            .wrapping_add(ptr as u32) as usize;
        let value = section_base(header, header.raw_ext_addr, target as u32);
        if addr + 4 <= d.len() {
            let old = u32_le(d, addr)?;
            let new = old.wrapping_add(value as u32);
            d[addr..addr + 4].copy_from_slice(&new.to_le_bytes());
        }
    }
    Ok(())
}

// ---- dict + patricia tree ----

struct Dict {
    values: usize,
    count: usize,
}

/// H3DDict = { list ref {ptr,count}, tree ref {ptr} }.
fn read_dict(r: &mut Rdr) -> Result<Dict> {
    let (values, count) = r.list_ref()?;
    r.ptr()?; // name tree (names come from the objects themselves)
    Ok(Dict {
        values,
        count: count.min(0x10000),
    })
}

/// Patricia tree nodes: { u32 ref-bit, u16 left, u16 right, {ptr} name }.
/// Names live in nodes 1..; loop while node index <= max child index.
fn read_tree_names(d: &[u8], at: usize) -> Result<Vec<String>> {
    let mut r = Rdr::new(d);
    r.seek(at)?;
    let mut names = Vec::new();
    let mut max_index = 0usize;
    let mut index = 0usize;
    loop {
        if index > max_index || names.len() > 0x1000 {
            break;
        }
        let node = r.pos;
        if node + 12 > d.len() {
            return Err(Error::truncated(node, 12, "bch tree node"));
        }
        r.skip(4)?;
        let left = r.u16()? as usize;
        let right = r.u16()? as usize;
        max_index = max_index.max(left).max(right);
        if index > 0 {
            let name_ptr = r.ptr()?;
            names.push(if name_ptr != 0 {
                r.cstr(name_ptr)?
            } else {
                String::new()
            });
        } else {
            r.skip(4)?;
        }
        index += 1;
    }
    Ok(names)
}

// ---- PICA command reader with uniform tracking ----

/// IEEE-754 float from a 24-bit PICA float (sign | exp-64 | 16-bit mantissa).
fn float24(bits: u32) -> f32 {
    if bits & 0x7f_ffff != 0 {
        let mantissa = bits & 0xff_ff;
        let exponent = ((bits >> 16) & 0x7f) + 64;
        let sign = (bits >> 23) & 1;
        f32::from_bits((mantissa << 7) | (exponent << 23) | (sign << 31))
    } else {
        f32::from_bits((bits & 0x80_0000) << 8)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

/// Decoded PICA command list (uniforms + geometry registers).
struct CommandReader {
    regs: Vec<(u16, u32)>,
}

impl CommandReader {
    /// Parse a u32 command stream (parameter/header pairs, 8-byte blocks).
    fn parse(d: &[u8], at: usize, word_count: usize) -> Result<Self> {
        let mut regs = Vec::new();
        let mut r = Rdr::new(d);
        r.seek(at)?;
        let mut index = 0usize;
        while index < word_count {
            if at + (index + 1) * 4 > d.len() {
                break;
            }
            let mut parameter = r.u32()?;
            let header = r.u32()?;
            index += 2;
            let mut id = (header & 0xffff) as u16;
            let extra = ((header >> 20) & 0x7ff) as usize;
            let consecutive = header & 0x8000_0000 != 0;
            if consecutive {
                for i in 0..=extra {
                    regs.push((id, parameter));
                    id = id.wrapping_add(1);
                    if i < extra {
                        parameter = r.u32()?;
                        index += 1;
                    }
                }
            } else {
                regs.push((id, parameter));
                for _ in 0..extra {
                    regs.push((id, r.u32()?));
                    index += 1;
                }
            }
            // Commands are padded to 8-byte blocks.
            if index & 1 != 0 {
                r.u32()?;
                index += 1;
            }
        }
        Ok(Self { regs })
    }
}

/// Uniform manager (SPICA UniformManager): float24 triples or 32-bit floats.
struct Uniforms {
    uniforms: [Vec4; 96],
    index: u32,
    bits32: bool,
    pending: Vec4,
    pending_words: u32,
}

impl Uniforms {
    fn new() -> Self {
        Self {
            uniforms: [Vec4::default(); 96],
            index: 0,
            bits32: false,
            pending: Vec4::default(),
            pending_words: 0,
        }
    }

    fn set_index(&mut self, cmd: u32) {
        self.index = (cmd & 0xff) << 2;
        self.bits32 = cmd >> 31 != 0;
        self.pending = Vec4::default();
        self.pending_words = 0;
    }

    fn set_value(&mut self, param: u32) {
        let uid = ((self.index >> 2) & 0x5f) as usize;
        if self.bits32 {
            let value = f32::from_bits(param);
            match self.index & 3 {
                0 => self.uniforms[uid].w = value,
                1 => self.uniforms[uid].z = value,
                2 => self.uniforms[uid].y = value,
                3 => self.uniforms[uid].x = value,
                _ => {}
            }
        } else {
            match self.index & 3 {
                0 => self.pending.x = float24(param & 0xff_ffff),
                1 => self.pending.y = float24(param & 0xff_ffff),
                2 => self.pending.z = float24(param & 0xff_ffff),
                _ => {}
            }
            // float24 vectors pack X/Y/Z into 3 words; W is unused here
            // (SPICA stores the triple and commits on the third word).
            if self.index & 3 == 2 {
                self.uniforms[uid] = self.pending;
                self.index += 1;
                self.pending = Vec4::default();
            }
        }
        self.index = self.index.wrapping_add(1);
    }
}

/// Run a command list through the uniform manager (as the SPICA
/// deserializers do for meshes and textures).
fn collect_uniforms(cmds: &CommandReader) -> [Vec4; 96] {
    let mut u = Uniforms::new();
    for &(reg, param) in &cmds.regs {
        if reg == REG_VSH_FLOATUNIFORM_INDEX {
            u.set_index(param);
        } else if (REG_VSH_FLOATUNIFORM_DATA0..=REG_VSH_FLOATUNIFORM_DATA7).contains(&reg) {
            u.set_value(param);
        }
    }
    u.uniforms
}

// ---- decoded types ----

#[derive(Debug, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub color: [f32; 4],
    pub uv: [[f32; 2]; 3],
    pub bone_indices: [i32; 4],
    pub bone_weights: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct SubMesh {
    pub skinning: u8, // 0 none, 1 rigid, 2 smooth, 3 mixed
    pub primitive_mode: u8,
    pub bone_indices: Vec<u16>,
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub material_index: usize,
    pub node_index: usize,
    pub priority: u8,
    pub layer: u8,
    pub vertices: Vec<Vertex>,
    pub submeshes: Vec<SubMesh>,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub emission: [u8; 4],
    pub ambient: [u8; 4],
    pub diffuse: [u8; 4],
    pub specular0: [u8; 4],
    pub specular1: [u8; 4],
    pub texture_names: [Option<String>; 3],
}

#[derive(Debug, Clone)]
pub struct Bone {
    pub name: String,
    pub parent_index: i16,
    pub scale: [f32; 3],
    pub rotation: [f32; 3],
    pub translation: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh>,
    pub bones: Vec<Bone>,
    pub node_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn decode_rgba(&self) -> Result<Vec<u8>> {
        crate::texture::decode_rgba(&self.data, self.width, self.height, self.format)
    }
}

/// One quantized key of a float animation curve.
#[derive(Debug, Clone, Copy)]
pub struct FloatKeyFrame {
    pub frame: f32,
    pub value: f32,
    pub in_slope: f32,
    pub out_slope: f32,
}

/// A float keyframe curve (`H3DFloatKeyFrameGroup`).
#[derive(Debug, Clone, Default)]
pub struct KeyFrameGroup {
    pub start_frame: f32,
    pub end_frame: f32,
    pub key_frames: Vec<FloatKeyFrame>,
}

/// Animation content, by primitive type (subset XY uses).
#[derive(Debug, Clone)]
pub enum AnimContent {
    /// One float curve (material constant/color targets).
    Float(KeyFrameGroup),
    /// Two curves (texture-coordinate scale/rotation/translation targets).
    Vector2D([KeyFrameGroup; 2]),
    /// Three curves.
    Vector3D([KeyFrameGroup; 3]),
    /// Per-frame booleans (mesh-node visibility).
    Boolean {
        start_frame: f32,
        end_frame: f32,
        values: Vec<bool>,
    },
    /// Primitive types not ported (Transform/RGBA/QuatTransform/…): raw
    /// bytes of the content region.
    Other(Vec<u8>),
}

/// One animated target (`H3DAnimationElement`).
#[derive(Debug, Clone)]
pub struct AnimElement {
    pub name: String,
    /// `H3DTargetType` (bone 0, MaterialTexCoord0Trans 0x15, …).
    pub target_type: u16,
    /// `H3DPrimitiveType` (Float 0, Vector2D 2, Vector3D 3, Boolean 8).
    pub primitive_type: u16,
    pub content: AnimContent,
}

/// A decoded animation (`H3DAnimation` family).
#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    /// `H3DAnimationType` (skeletal 0, material 1, visibility 2, …).
    pub animation_type: u8,
    pub is_looping: bool,
    pub frames_count: f32,
    pub elements: Vec<AnimElement>,
}

/// Decoded BCH container (models, textures, animations).
#[derive(Debug, Default)]
pub struct Bch {
    pub models: Vec<Model>,
    pub textures: Vec<Texture>,
    /// Animations from dicts 8..13 (skeletal, material, visibility,
    /// light, camera, fog) in dict order.
    pub animations: Vec<Animation>,
}

impl Bch {
    /// Parse a BCH image. The buffer is copied and relocated internally
    /// (SPICA's ToAbsolute mutates the file before deserialization).
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut d = data.to_vec();
        let header = parse_header(&d)?;
        relocate(&mut d, &header)?;
        let d = d;

        let mut r = Rdr::new(&d);
        r.seek(header.contents_addr as usize)?;

        // 15 root dicts (models … scenes); we keep models + textures.
        let mut dicts = Vec::with_capacity(15);
        for _ in 0..15 {
            dicts.push(read_dict(&mut r)?);
        }

        let mut out = Bch::default();

        // Models: {ptr} refs.
        for i in 0..dicts[0].count {
            if dicts[0].values + i * 4 + 4 > d.len() {
                break;
            }
            let ptr = u32_le(&d, dicts[0].values + i * 4)? as usize;
            if ptr == 0 {
                continue;
            }
            out.models.push(parse_model(&d, ptr)?);
        }

        // Animations: dicts 8..13 (skeletal, material, visibility,
        // light, camera, fog) — {ptr} refs.
        for dict in &dicts[8..14] {
            for i in 0..dict.count {
                if dict.values + i * 4 + 4 > d.len() {
                    break;
                }
                let ptr = u32_le(&d, dict.values + i * 4)? as usize;
                if ptr == 0 {
                    continue;
                }
                out.animations.push(parse_animation(&d, ptr)?);
            }
        }

        // Textures: {ptr} refs.
        for i in 0..dicts[3].count {
            if dicts[3].values + i * 4 + 4 > d.len() {
                break;
            }
            let ptr = u32_le(&d, dicts[3].values + i * 4)? as usize;
            if ptr == 0 {
                continue;
            }
            out.textures.push(parse_texture(&d, ptr)?);
        }

        Ok(out)
    }
}

fn rgba_u32(v: u32) -> [u8; 4] {
    // RGBA stored as four bytes in file order.
    [
        (v & 0xff) as u8,
        (v >> 8) as u8,
        (v >> 16) as u8,
        (v >> 24) as u8,
    ]
}

fn parse_model(d: &[u8], at: usize) -> Result<Model> {
    let mut r = Rdr::new(d);
    r.seek(at)?;
    let _flags = r.u8()?;
    let _bone_scaling = r.u8()?;
    let _silhouette = r.u16()?;
    let mut world_transform = [0f32; 12];
    for v in &mut world_transform {
        *v = r.f32()?;
    }
    let _ = world_transform;

    // Materials dict → inline materials.
    let (mats_at, mat_count) = {
        let (p, c) = r.list_ref()?;
        r.ptr()?; // tree
        (p, c)
    };
    let mat_count = mat_count.min(0x1000);
    let mut materials = Vec::with_capacity(mat_count);
    {
        let mut mr = Rdr::new(d);
        mr.seek(mats_at)?;
        for _ in 0..mat_count {
            materials.push(parse_material(d, &mut mr)?);
        }
    }

    // Meshes list → inline meshes.
    let (meshes_at, mesh_count) = r.list_ref()?;
    let mesh_count = mesh_count.min(0x1000);
    let mut meshes = Vec::with_capacity(mesh_count);
    {
        let mut msr = Rdr::new(d);
        msr.seek(meshes_at)?;
        for _ in 0..mesh_count {
            meshes.push(parse_mesh(d, &mut msr)?);
        }
    }
    // Layer lists (4 × {ptr,end} — redundant with meshes; skip).
    for _ in 0..4 {
        r.list_ref()?;
    }
    // Submesh cullings (v>=7, empty struct): {ptr,count} — skip.
    r.list_ref()?;

    // Skeleton dict → inline bones.
    let (bones_at, bone_count) = {
        let (p, c) = r.list_ref()?;
        r.ptr()?; // tree
        (p, c)
    };
    let bone_count = bone_count.min(0x1000);
    let mut bones = Vec::with_capacity(bone_count);
    {
        let mut br = Rdr::new(d);
        br.seek(bones_at)?;
        for _ in 0..bone_count {
            bones.push(parse_bone(d, &mut br)?);
        }
    }

    // Mesh-node visibility list (bit-packed): {ptr,count} — skip.
    r.list_ref()?;
    // Name
    let name_ptr = r.ptr()?;
    let name = if name_ptr != 0 {
        r.cstr(name_ptr)?
    } else {
        String::new()
    };
    let _mesh_nodes_count = r.i32()?;
    let tree_ptr = r.ptr()?;
    let node_names = if tree_ptr != 0 {
        read_tree_names(d, tree_ptr)?
    } else {
        Vec::new()
    };

    Ok(Model {
        name,
        materials,
        meshes,
        bones,
        node_names,
    })
}

/// H3DMaterial (inline, 0x58 bytes). `r` must be positioned at the start.
fn parse_material(d: &[u8], r: &mut Rdr) -> Result<Material> {
    let params_ptr = r.ptr()?;
    let _tex0 = r.ptr()?;
    let _tex1 = r.ptr()?;
    let _tex2 = r.ptr()?;
    let _tex_commands = r.list_ref()?;
    // 3 inline mappers, 16 bytes each (sampler/wrap/filter config).
    let mut mappers = [[0u8; 8]; 3];
    for mapper in &mut mappers {
        for b in mapper.iter_mut() {
            *b = r.u8()?;
        }
        r.skip(8)?;
    }
    let tex0_name = r.ptr()?;
    let tex1_name = r.ptr()?;
    let tex2_name = r.ptr()?;
    let name_ptr = r.ptr()?;

    let texture_names = [tex0_name, tex1_name, tex2_name].map(|p| {
        if p != 0 {
            Some(r.cstr(p).unwrap_or_default())
        } else {
            None
        }
    });

    // MaterialParams: colors at fixed offsets (+0x58 emission … +0x78
    // constant5) — parse standalone at the pointer.
    let (emission, ambient, diffuse, specular0, specular1) = if params_ptr != 0 {
        let mut pr = Rdr::new(d);
        pr.seek(params_ptr + 0x58)?;
        let mut colors = [[0xffu8; 4]; 5];
        for (i, color) in colors.iter_mut().enumerate() {
            *color = rgba_u32(pr.u32()?);
            let _ = i;
        }
        (colors[0], colors[1], colors[2], colors[3], colors[4])
    } else {
        ([0u8; 4], [0xff; 4], [0xff; 4], [0u8; 4], [0u8; 4])
    };

    Ok(Material {
        name: if name_ptr != 0 {
            r.cstr(name_ptr)?
        } else {
            String::new()
        },
        emission,
        ambient,
        diffuse,
        specular0,
        specular1,
        texture_names,
    })
}

/// H3DMesh (inline, 0x38 bytes).
fn parse_mesh(d: &[u8], r: &mut Rdr) -> Result<Mesh> {
    let material_index = r.u16()? as usize;
    let _mesh_flags = r.u8()?; // bits 0-1 type, 2-3 skinning
    r.skip(1)?; // [Padding(2)] — node index is 2-aligned
    let node_index = r.u16()? as usize;
    let key = r.u16()?;
    let enable_cmds = r.list_ref()?;
    let (sub_at, sub_count) = r.list_ref()?;
    let _disable_cmds = r.list_ref()?;
    let _center = [r.f32()?, r.f32()?, r.f32()?];
    r.ptr()?; // parent
    let _user = r.u32()?;
    r.ptr()?; // meta data

    // Enable commands → attributes, buffer address, stride, position offset.
    let cmds = CommandReader::parse(d, enable_cmds.0, enable_cmds.1)?;
    let mut uniforms = collect_uniforms(&cmds);

    let mut buffer_formats: u64 = 0;
    let mut buffer_attributes: u64 = 0;
    let mut buffer_permutation: u64 = 0;
    let mut buffer_address = 0usize;
    let mut stride = 0usize;
    let mut attributes_count = 0usize;
    let mut attributes_total = 0usize;
    let mut fixed_index = 0usize;
    let mut fixed: [Vec4; 12] = [Vec4::default(); 12];

    for &(reg, param) in &cmds.regs {
        match reg {
            REG_ATTRIBBUFFERS_FORMAT_LOW => buffer_formats |= param as u64,
            REG_ATTRIBBUFFERS_FORMAT_HIGH => buffer_formats |= (param as u64) << 32,
            REG_ATTRIBBUFFER0_OFFSET => buffer_address = param as usize,
            REG_ATTRIBBUFFER0_CONFIG1 => buffer_attributes |= param as u64,
            REG_ATTRIBBUFFER0_CONFIG2 => {
                buffer_attributes |= ((param & 0xffff) as u64) << 32;
                stride = ((param >> 16) & 0xff) as usize;
                attributes_count = (param >> 28) as usize;
            }
            REG_FIXEDATTRIB_INDEX => fixed_index = (param as usize) % 12,
            r if (REG_FIXEDATTRIB_DATA0..=REG_FIXEDATTRIB_DATA0 + 2).contains(&r) => {
                let slot = r - REG_FIXEDATTRIB_DATA0;
                let value = float24(param & 0xff_ffff);
                match slot {
                    0 => fixed[fixed_index].x = value,
                    1 => fixed[fixed_index].y = value,
                    _ => fixed[fixed_index].z = value,
                }
            }
            REG_VSH_NUM_ATTR => attributes_total = param as usize + 1,
            REG_VSH_PERMUTATION_LOW => buffer_permutation |= param as u64,
            REG_VSH_PERMUTATION_HIGH => buffer_permutation |= (param as u64) << 32,
            _ => {}
        }
    }
    let _ = attributes_count;

    // Attribute list (SPICA H3DMesh.Deserialize).
    #[derive(Clone, Copy)]
    struct Attr {
        name: AttrName,
        format: u8, // 0 s8, 1 u8, 2 s16, 3 f32
        elements: usize,
        scale: f32,
    }
    let attributes_total = attributes_total.min(16);
    let mut attributes: Vec<Attr> = Vec::with_capacity(attributes_total);
    for index in 0..attributes_total {
        if (buffer_formats >> (48 + index)) & 1 != 0 {
            // Fixed attribute (no data in the stream buffer).
            let _ = fixed;
            continue;
        }
        let permutation_idx = ((buffer_attributes >> (index * 4)) & 0xf) as usize;
        let name_id = ((buffer_permutation >> (permutation_idx * 4)) & 0xf) as u8;
        let fmt = ((buffer_formats >> (permutation_idx * 4)) & 0xf) as u8;
        let mut scale = 1f32;
        let name = AttrName::from_id(name_id);
        match name {
            AttrName::Position => scale = uniforms[7].x,
            AttrName::Normal => scale = uniforms[7].y,
            AttrName::Tangent => scale = uniforms[7].z,
            AttrName::Color => scale = uniforms[7].w,
            AttrName::TexCoord0 => scale = uniforms[8].x,
            AttrName::TexCoord1 => scale = uniforms[8].y,
            AttrName::TexCoord2 => scale = uniforms[8].z,
            AttrName::BoneWeight => scale = uniforms[8].w,
            _ => {}
        }
        attributes.push(Attr {
            name,
            format: fmt & 3,
            elements: ((fmt >> 2) & 3) as usize + 1,
            scale,
        });
    }
    let _ = &mut uniforms;

    // Submeshes (inline) — index buffers + primitive modes.
    let sub_count = sub_count.min(0x1000);
    let mut submeshes = Vec::with_capacity(sub_count);
    let mut max_index = 0usize;
    {
        let mut sr = Rdr::new(d);
        sr.seek(sub_at)?;
        for _ in 0..sub_count {
            let _skinning_pad = sr.u8()?;
            let skinning = sr.u8()?;
            let bone_count = sr.u16()? as usize;
            let mut bone_indices = Vec::with_capacity(bone_count.min(20));
            for i in 0..20 {
                let b = sr.u16()?;
                if i < bone_count {
                    bone_indices.push(b);
                }
            }
            let sub_cmds = sr.list_ref()?;
            let reader = CommandReader::parse(d, sub_cmds.0, sub_cmds.1)?;
            let mut idx_address = 0usize;
            let mut idx_count = 0usize;
            let mut primitive_mode = 0u8;
            for &(reg, param) in &reader.regs {
                match reg {
                    REG_INDEXBUFFER_CONFIG => idx_address = param as usize,
                    REG_NUMVERTICES => idx_count = param as usize,
                    REG_PRIMITIVE_CONFIG => primitive_mode = ((param >> 8) & 0xff) as u8,
                    _ => {}
                }
            }
            let is_16bit = idx_address >> 31 != 0;
            let addr = idx_address & 0x7fff_ffff;
            let idx_count = idx_count.min(1 << 22);
            let mut indices = Vec::with_capacity(idx_count);
            for i in 0..idx_count {
                let v = if is_16bit {
                    u16_le(d, addr + i * 2)
                } else {
                    u8_at(d, addr + i).map(|v| v as u16)
                };
                match v {
                    Ok(v) => {
                        max_index = max_index.max(v as usize);
                        indices.push(v);
                    }
                    Err(_) => break,
                }
            }
            submeshes.push(SubMesh {
                skinning,
                primitive_mode,
                bone_indices,
                indices,
            });
        }
    }

    // Raw vertex buffer.
    let vertex_count = max_index + 1;
    let mut vertices = Vec::with_capacity(vertex_count.min(1 << 20));
    if stride > 0 && buffer_address > 0 {
        for v in 0..vertex_count {
            let mut cursor = buffer_address + v * stride;
            let mut vertex = Vertex {
                bone_indices: [-1; 4],
                ..Vertex::default()
            };
            for attr in &attributes {
                if attr.format >= 2 && cursor & 1 != 0 {
                    cursor += 1; // shorts/floats align to 2
                }
                let mut comps = [0f32; 4];
                for (e, comp) in comps.iter_mut().enumerate().take(attr.elements) {
                    *comp = match attr.format {
                        0 => u8_at(d, cursor)? as i8 as f32,
                        1 => u8_at(d, cursor)? as f32,
                        2 => u16_le(d, cursor)? as i16 as f32,
                        3 => f32_le(d, cursor)?,
                        _ => 0.0,
                    };
                    cursor += match attr.format {
                        2 => 2,
                        3 => 4,
                        _ => 1,
                    };
                    let _ = e;
                }
                let mut v4 = comps;
                for c in &mut v4 {
                    *c *= attr.scale;
                }
                match attr.name {
                    AttrName::Position => {
                        vertex.position = [v4[0], v4[1], v4[2]];
                    }
                    AttrName::Normal => vertex.normal = [v4[0], v4[1], v4[2]],
                    AttrName::Tangent => vertex.tangent = [v4[0], v4[1], v4[2]],
                    AttrName::Color => vertex.color = v4,
                    AttrName::TexCoord0 => vertex.uv[0] = [v4[0], v4[1]],
                    AttrName::TexCoord1 => vertex.uv[1] = [v4[0], v4[1]],
                    AttrName::TexCoord2 => vertex.uv[2] = [v4[0], v4[1]],
                    AttrName::BoneIndex => {
                        vertex.bone_indices[0] = v4[0] as i32;
                        if attr.elements == 1 {
                            continue;
                        }
                        vertex.bone_indices[1] = v4[1] as i32;
                        if attr.elements == 2 {
                            continue;
                        }
                        vertex.bone_indices[2] = v4[2] as i32;
                        if attr.elements == 3 {
                            continue;
                        }
                        vertex.bone_indices[3] = v4[3] as i32;
                    }
                    AttrName::BoneWeight => {
                        vertex.bone_weights[0] = v4[0];
                        if attr.elements == 1 {
                            continue;
                        }
                        vertex.bone_weights[1] = v4[1];
                        if attr.elements == 2 {
                            continue;
                        }
                        vertex.bone_weights[2] = v4[2];
                        if attr.elements == 3 {
                            continue;
                        }
                        vertex.bone_weights[3] = v4[3];
                    }
                    _ => {}
                }
            }
            vertices.push(vertex);
        }
    }

    Ok(Mesh {
        material_index,
        node_index,
        priority: (key & 0xff) as u8,
        layer: ((key >> 8) & 3) as u8,
        vertices,
        submeshes,
    })
}

/// H3DBone (inline, 100 bytes).
fn parse_bone(_d: &[u8], r: &mut Rdr) -> Result<Bone> {
    let _flags = r.u32()?;
    let parent_index = r.u16()? as i16;
    r.skip(2)?;
    let scale = [r.f32()?, r.f32()?, r.f32()?];
    let rotation = [r.f32()?, r.f32()?, r.f32()?];
    let translation = [r.f32()?, r.f32()?, r.f32()?];
    let mut inverse = [0f32; 12];
    for v in &mut inverse {
        *v = r.f32()?;
    }
    let _ = inverse;
    let name_ptr = r.ptr()?;
    r.ptr()?; // meta data
    Ok(Bone {
        name: if name_ptr != 0 {
            r.cstr(name_ptr)?
        } else {
            String::new()
        },
        parent_index,
        scale,
        rotation,
        translation,
    })
}

/// H3DTexture (ref): commands triple + format + name; data via TEXUNIT0
/// registers.
fn parse_texture(d: &[u8], at: usize) -> Result<Texture> {
    let mut r = Rdr::new(d);
    r.seek(at)?;
    let cmd0 = r.list_ref()?;
    let _cmd1 = r.list_ref()?;
    let _cmd2 = r.list_ref()?;
    let format_id = r.u8()?;
    let _mipmap = r.u8()?;
    r.skip(2)?;
    let name_ptr = r.ptr()?;
    let format = TextureFormat::from_u32(format_id as u32)?;

    let reader = CommandReader::parse(d, cmd0.0, cmd0.1)?;
    let mut width = 0usize;
    let mut height = 0usize;
    let mut address = 0usize;
    for &(reg, param) in &reader.regs {
        match reg {
            REG_TEXUNIT0_DIM => {
                height = (param & 0x7ff) as usize;
                width = ((param >> 16) & 0x7ff) as usize;
            }
            REG_TEXUNIT0_ADDR1 => address = param as usize,
            _ => {}
        }
    }
    if width == 0 || height == 0 || address == 0 {
        return Err(Error::invalid(
            "bch texture",
            format!("missing texture data ({width}x{height} @ {address:#x})"),
        ));
    }
    let len = width * height * format.bits_per_pixel() as usize / 8;
    if address + len > d.len() {
        return Err(Error::truncated(address, len, "bch texture data"));
    }
    Ok(Texture {
        name: if name_ptr != 0 {
            r.cstr(name_ptr)?
        } else {
            String::new()
        },
        width,
        height,
        format,
        data: d[address..address + len].to_vec(),
    })
}

// ---- animations (H3DAnimation family) ----

/// Read a float keyframe group: either an inline constant or a group
/// object referenced by a self-relative pointer at the cursor.
fn read_key_group(d: &[u8], r: &mut Rdr, constant: bool) -> Result<KeyFrameGroup> {
    if constant {
        return Ok(KeyFrameGroup {
            key_frames: vec![FloatKeyFrame {
                frame: 0.0,
                value: r.f32()?,
                in_slope: 0.0,
                out_slope: 0.0,
            }],
            ..KeyFrameGroup::default()
        });
    }
    let target = u32_le(d, r.pos)? as usize;
    r.skip(4)?;
    if target == 0 {
        return Ok(KeyFrameGroup::default());
    }
    // Group object: StartFrame f32 | EndFrame f32 | PreRepeat u8 |
    // PostRepeat u8 | CurveIndex u16 | {ptr payload}.
    let start_frame = f32_le(d, target)?;
    let end_frame = f32_le(d, target + 4)?;
    let payload_ptr = u32_le(d, target + 12)? as usize;
    if payload_ptr == 0 {
        return Ok(KeyFrameGroup {
            start_frame,
            end_frame,
            key_frames: Vec::new(),
        });
    }
    // Payload: StartFrame f32 | EndFrame f32 | interp u8 | quant u8 |
    // count u16 | InvDuration f32 | ValueScale f32 | ValueOffset f32 |
    // FrameScale f32 | {ptr keys}.
    let quant = u8_at(d, payload_ptr + 9)?;
    let count = u16_le(d, payload_ptr + 10)? as usize;
    let value_scale = f32_le(d, payload_ptr + 16)?;
    let value_offset = f32_le(d, payload_ptr + 20)?;
    let frame_scale = f32_le(d, payload_ptr + 24)?;
    let keys_ptr = u32_le(d, payload_ptr + 28)? as usize;
    let mut group = KeyFrameGroup {
        start_frame,
        end_frame,
        key_frames: Vec::with_capacity(count.min(0x1_0000)),
    };
    let _ = end_frame;
    if keys_ptr == 0 {
        return Ok(group);
    }
    let mut pos = keys_ptr;
    for _ in 0..count {
        let mut kf = match quant {
            0 => FloatKeyFrame {
                // Hermite128: frame, value, in, out (f32 each).
                frame: f32_le(d, pos)?,
                value: f32_le(d, pos + 4)?,
                in_slope: f32_le(d, pos + 8)?,
                out_slope: f32_le(d, pos + 12)?,
            },
            1 => {
                // Hermite64: u32 (frame 12b | value 20b), i16 in, i16 out.
                let frame_val = u32_le(d, pos)?;
                let frame = (frame_val & 0xfff) as f32;
                let value = ((frame_val >> 12) & 0xf_ffff) as f32;
                let in_slope = i16::from_le_bytes([d[pos + 4], d[pos + 5]]) as f32 / 256.0;
                let out_slope = i16::from_le_bytes([d[pos + 6], d[pos + 7]]) as f32 / 256.0;
                pos += 8;
                FloatKeyFrame {
                    frame,
                    value,
                    in_slope,
                    out_slope,
                }
            }
            2 => {
                // Hermite48: u8 frame, u16 value, u24 slopes (12-bit
                // sign-extended each, / 32).
                let frame = u8_at(d, pos)? as f32;
                let value = u16_le(d, pos + 1)? as f32;
                let slopes = (u32_le(d, pos + 2)? & 0xff_ffff) as i32;
                let in_slope = (((slopes << 20) >> 20) as f32) / 32.0;
                let out_slope = (((slopes << 8) >> 20) as f32) / 32.0;
                pos += 6;
                FloatKeyFrame {
                    frame,
                    value,
                    in_slope,
                    out_slope,
                }
            }
            3 => FloatKeyFrame {
                // UnifiedHermite96: f32 frame, value, slope (in = out).
                frame: f32_le(d, pos)?,
                value: f32_le(d, pos + 4)?,
                in_slope: f32_le(d, pos + 8)?,
                out_slope: f32_le(d, pos + 8)?,
            },
            4 => {
                // UnifiedHermite48: u16 frame (/32), u16 value, i16 slope.
                let frame = u16_le(d, pos)? as f32 / 32.0;
                let value = u16_le(d, pos + 2)? as f32;
                let slope = i16::from_le_bytes([d[pos + 4], d[pos + 5]]) as f32 / 256.0;
                pos += 6;
                FloatKeyFrame {
                    frame,
                    value,
                    in_slope: slope,
                    out_slope: slope,
                }
            }
            5 => {
                // UnifiedHermite32: u8 frame, u24 (value 12b | slope 12b).
                let frame = u8_at(d, pos)? as f32;
                let val_slope = (u32_le(d, pos + 1)? & 0xff_ffff) as i32;
                let value = (val_slope & 0xfff) as f32;
                let slope = (((val_slope << 8) >> 20) as f32) / 32.0;
                pos += 4;
                FloatKeyFrame {
                    frame,
                    value,
                    in_slope: slope,
                    out_slope: slope,
                }
            }
            6 => FloatKeyFrame {
                // StepLinear64: f32 frame, f32 value.
                frame: f32_le(d, pos)?,
                value: f32_le(d, pos + 4)?,
                in_slope: 0.0,
                out_slope: 0.0,
            },
            7 => {
                // StepLinear32: u32 (frame 12b | value 20b).
                let frame_val = u32_le(d, pos)?;
                pos += 4;
                FloatKeyFrame {
                    frame: (frame_val & 0xfff) as f32,
                    value: ((frame_val >> 12) & 0xf_ffff) as f32,
                    in_slope: 0.0,
                    out_slope: 0.0,
                }
            }
            other => {
                return Err(Error::invalid(
                    "bch animation",
                    format!("quantization {other}"),
                ))
            }
        };
        match quant {
            0 => pos += 16,
            6 => pos += 8,
            1 | 4 | 5 => {}
            2 => pos += 0, // advanced in-branch
            3 => pos += 12,
            7 => {}
            _ => {}
        }
        // Apply scales (SPICA: frame *= FrameScale; value = v*scale+off).
        kf.frame *= frame_scale;
        kf.value = kf.value * value_scale + value_offset;
        group.key_frames.push(kf);
    }
    Ok(group)
}

/// Read one vector-style content block (flags + per-axis groups) with
/// `axes` tracks; cursor is positioned at the flags word.
fn read_vector_content(d: &[u8], r: &mut Rdr, axes: usize) -> Result<Vec<KeyFrameGroup>> {
    let flags = r.u32()?;
    let mut groups = Vec::with_capacity(axes);
    for axis in 0..axes {
        let constant = flags & (1 << axis) != 0;
        let exists = flags & (1 << (8 + axis)) == 0;
        if exists {
            groups.push(read_key_group(d, r, constant)?);
        } else {
            // Inexistent axis: still occupies the 4-byte slot.
            r.skip(4)?;
            groups.push(KeyFrameGroup::default());
        }
    }
    while groups.len() < axes {
        groups.push(KeyFrameGroup::default());
    }
    Ok(groups)
}

fn parse_animation(d: &[u8], at: usize) -> Result<Animation> {
    let mut r = Rdr::new(d);
    r.seek(at)?;
    let name_ptr = r.ptr()?;
    let flags = r.u8()?;
    let animation_type = r.u8()?;
    let _curves_count = r.u16()?;
    let frames_count = r.f32()?;
    let (elem_list, elem_count) = r.list_ref()?;
    let _meta = r.ptr()?;

    let mut elements = Vec::with_capacity(elem_count.min(0x1000));
    for i in 0..elem_count {
        let elem_off = u32_le(d, elem_list + i * 4)? as usize;
        if elem_off == 0 {
            continue;
        }
        let mut er = Rdr::new(d);
        er.seek(elem_off)?;
        let name_ptr = er.ptr()?;
        let target_type = er.u16()?;
        let primitive_type = er.u16()?;
        let name = if name_ptr != 0 {
            er.cstr(name_ptr)?
        } else {
            String::new()
        };

        let content = match primitive_type {
            0 => AnimContent::Float(read_key_group(d, &mut er, false)?),
            2 => {
                let groups = read_vector_content(d, &mut er, 2)?;
                AnimContent::Vector2D([groups[0].clone(), groups[1].clone()])
            }
            3 => {
                let groups = read_vector_content(d, &mut er, 3)?;
                AnimContent::Vector3D([groups[0].clone(), groups[1].clone(), groups[2].clone()])
            }
            8 => {
                // Boolean: curve header then a {count, bit-packed bools} list.
                let start_frame = er.f32()?;
                let end_frame = er.f32()?;
                let count = er.i32()?.max(0) as usize;
                let mut values = Vec::with_capacity(count.min(0x1_0000));
                let mut byte = 0u8;
                for index in 0..count {
                    if index & 7 == 0 {
                        byte = er.u8()?;
                    }
                    values.push(byte & (1 << (index & 7)) != 0);
                }
                AnimContent::Boolean {
                    start_frame,
                    end_frame,
                    values,
                }
            }
            _other => {
                // Unported primitive — content parsing left to a future pass.
                AnimContent::Other(Vec::new())
            }
        };
        elements.push(AnimElement {
            name,
            target_type,
            primitive_type,
            content,
        });
    }

    Ok(Animation {
        name: if name_ptr != 0 {
            r.cstr(name_ptr)?
        } else {
            String::new()
        },
        animation_type,
        is_looping: flags & 1 != 0,
        frames_count,
        elements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float24_roundtrip() {
        assert_eq!(float24(0), 0.0);
        assert_eq!(float24(0x800000), 0.0); // negative zero
                                            // IEEE 754 1.0 = exp 127 → stored 127-64 = 63 = 0x3f, mantissa 0.
        assert!(
            (float24(0x3f0000) - 1.0).abs() < 1e-6,
            "{}",
            float24(0x3f0000)
        );
        assert!(
            (float24(0xbf0000) + 1.0).abs() < 1e-6,
            "{}",
            float24(0xbf0000)
        );
        // 2.0: exp 128 → stored 64, mantissa 0.
        assert!(
            (float24(0x400000) - 2.0).abs() < 1e-6,
            "{}",
            float24(0x400000)
        );
        // 0.5: exp 126 → stored 62, mantissa 0.
        assert!(
            (float24(0x3e0000) - 0.5).abs() < 1e-6,
            "{}",
            float24(0x3e0000)
        );
    }

    #[test]
    fn rejects_bad_magic() {
        assert!(Bch::parse(b"NOPE").is_err());
    }
}
