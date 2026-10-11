//! GameFreak model loader — port of Ohana3DS `Models/PocketMonsters/
//! GfModel.cs` + the subset of `PICA200/PICACommandReader.cs` needed for
//! geometry extraction (vertex attribute formats, strides, index buffers).
//!
//! GF model section layout (inside a PkmnContainer sub-file):
//!
//! ```text
//! +0x00  0x10 unknown bytes
//!        u64 magic "gfmodel" | u32 length | u32 -1
//!        effect names table  | texture names | material names | mesh names
//!        (each: u32 count, then count × 0x44-byte entries
//!         { u32 hash, 0x40-byte NUL-padded name })
//!        0x20 unknown (2 float4) | 4×f32 transform matrix
//!        u32 unkLen | u32 unkRelStart | 2×u32 → skip unkRelStart+unkLen
//!        u32 bone count | u32[3] → bones: {u8 nameLen, name,
//!            u8 parentLen, parent, u8 flags, 3×f32 scale/rot/translate}
//! then   at (section start + length + 0x20): materials, then meshes
//! (meshes carry PICA command buffers that describe vertex attributes;
//!  vertex/index data blobs follow each sub-mesh descriptor)
//! ```
//!
//! Port notes: faithful to the C# control flow (Ohana3DS Rebirth by
//! gdkChan); bounds-checked via `xy-core` reads instead of exceptions.

use xy_core::error::{Error, Result};
use xy_core::util::{u16_le, u32_le, u64_le};

// ---- PICA command IDs (subset, from PICACommand.cs) ----
const CMD_VSH_ATTR_FORMAT_LOW: usize = 0x201;
const CMD_VSH_ATTR_FORMAT_HIGH: usize = 0x202;
const CMD_VSH_ATTR_BUF0_PERM: usize = 0x204;
const CMD_VSH_ATTR_BUF0_STRIDE: usize = 0x205;
const CMD_INDEX_BUFFER_CONFIG: usize = 0x227;
const CMD_INDEX_BUFFER_TOTAL_VERTICES: usize = 0x228;
const CMD_BLOCK_END: usize = 0x23d;
const CMD_VSH_TOTAL_ATTRIBUTES: usize = 0x242;
const CMD_VSH_PERMUTATION_LOW: usize = 0x2bb;
const CMD_VSH_PERMUTATION_HIGH: usize = 0x2bc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VshAttribute {
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

impl VshAttribute {
    fn from_id(id: u8) -> Self {
        match id {
            0 => VshAttribute::Position,
            1 => VshAttribute::Normal,
            2 => VshAttribute::Tangent,
            3 => VshAttribute::Color,
            4 => VshAttribute::TexCoord0,
            5 => VshAttribute::TexCoord1,
            6 => VshAttribute::TexCoord2,
            7 => VshAttribute::BoneIndex,
            8 => VshAttribute::BoneWeight,
            other => VshAttribute::Other(other),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AttributeFormat {
    pub ty: u8,  // 0 = s8, 1 = u8, 2 = s16, 3 = f32
    pub len: u8, // components - 1
}

/// Decoded PICA GPU command buffer (geometry-relevant accessors only).
pub struct PicaCommandReader {
    commands: Vec<u32>,
}

impl PicaCommandReader {
    /// Parse `word_count` u32 words of PICA commands. Odd half-words are
    /// parameters, even ones headers — i.e. pairs of (parameter, header).
    pub fn parse(data: &[u8], word_count: u32) -> Result<Self> {
        let need = word_count as usize * 4;
        if data.len() < need {
            return Err(Error::truncated(0, need, "pica command buffer"));
        }
        let mut commands = vec![0u32; 0x10000];
        let mut pos = 0usize;
        let mut read_words = 0u32;
        while read_words < word_count {
            let parameter = u32_le(data, pos)?;
            let header = u32_le(data, pos + 4)?;
            pos += 8;
            read_words += 2;

            let id = (header & 0xffff) as usize;
            let mask = ((header >> 16) & 0xf) as u32;
            let extra = ((header >> 20) & 0x7ff) as usize;
            let consecutive = header & 0x8000_0000 != 0;

            if id < commands.len() {
                let preserved = !mask & 0xf;
                commands[id] = (commands[id] & preserved) | (parameter & (0xffff_fff0 | mask));
            }
            if id == CMD_BLOCK_END {
                break;
            }
            let mut id = id;
            for _ in 0..extra {
                if consecutive {
                    id += 1;
                }
                let v = u32_le(data, pos)?;
                pos += 4;
                read_words += 1;
                if id < commands.len() {
                    let preserved = !mask & 0xf;
                    commands[id] = (commands[id] & preserved) | (v & (0xffff_fff0 | mask));
                }
            }
            // 8-byte alignment: skip padding u32s until 8-byte boundary.
            while !pos.is_multiple_of(8) {
                pos += 4;
            }
        }
        Ok(Self { commands })
    }

    fn parameter(&self, id: usize) -> u32 {
        self.commands.get(id).copied().unwrap_or(0)
    }

    pub fn vsh_total_attributes(&self) -> u32 {
        self.parameter(CMD_VSH_TOTAL_ATTRIBUTES)
    }

    /// Main permutation: 23 × 4-bit attribute ids.
    pub fn vsh_permutation(&self) -> [u8; 23] {
        let lo = self.parameter(CMD_VSH_PERMUTATION_LOW) as u64;
        let hi = self.parameter(CMD_VSH_PERMUTATION_HIGH) as u64;
        let perm = lo | (hi << 32);
        let mut out = [0u8; 23];
        for (i, a) in out.iter_mut().enumerate() {
            *a = ((perm >> (i * 4)) & 0xf) as u8;
        }
        out
    }

    /// Main attribute formats: 23 × 4-bit entries (2-bit type, 2-bit len-1).
    pub fn vsh_formats(&self) -> [AttributeFormat; 23] {
        let lo = self.parameter(CMD_VSH_ATTR_FORMAT_LOW) as u64;
        let hi = self.parameter(CMD_VSH_ATTR_FORMAT_HIGH) as u64;
        let fmt = lo | (hi << 32);
        let mut out = [AttributeFormat::default(); 23];
        for (i, f) in out.iter_mut().enumerate() {
            let v = ((fmt >> (i * 4)) & 0xf) as u8;
            f.ty = v & 3;
            f.len = v >> 2;
        }
        out
    }

    pub fn buffer0_stride(&self) -> u32 {
        (self.parameter(CMD_VSH_ATTR_BUF0_STRIDE) >> 16) & 0xff
    }

    pub fn buffer0_total_attributes(&self) -> u32 {
        self.parameter(CMD_VSH_ATTR_BUF0_STRIDE) >> 28
    }

    /// Buffer-0 permutation: 23 × 4-bit indices into the main permutation.
    pub fn buffer0_permutation(&self) -> [u32; 23] {
        let lo = self.parameter(CMD_VSH_ATTR_BUF0_PERM) as u64;
        let hi = (self.parameter(CMD_VSH_ATTR_BUF0_STRIDE) & 0xffff) as u64;
        let perm = lo | (hi << 32);
        let mut out = [0u32; 23];
        for (i, a) in out.iter_mut().enumerate() {
            *a = ((perm >> (i * 4)) & 0xf) as u32;
        }
        out
    }

    pub fn index_format_short(&self) -> bool {
        self.parameter(CMD_INDEX_BUFFER_CONFIG) >> 31 != 0
    }

    pub fn index_total_vertices(&self) -> u32 {
        self.parameter(CMD_INDEX_BUFFER_TOTAL_VERTICES)
    }
}

// ---- decoded model structures (OBJ-relevant subset of RenderBase.OModel) ----

#[derive(Debug, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub bones: [u16; 4],
    pub weights: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct Bone {
    pub name: String,
    pub parent: i16,
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub material_id: i32,
    pub vertices: Vec<Vertex>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub bones: Vec<Bone>,
    pub meshes: Vec<Mesh>,
}

#[derive(Debug, Default)]
pub struct ModelGroup {
    pub models: Vec<Model>,
}

// ---- the GfModel loader ----

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8], pos: usize) -> Self {
        Self { data, pos }
    }
    fn u32(&mut self) -> Result<u32> {
        let v = u32_le(self.data, self.pos)?;
        self.pos += 4;
        Ok(v)
    }
    fn u16(&mut self) -> Result<u16> {
        let v = u16_le(self.data, self.pos)?;
        self.pos += 2;
        Ok(v)
    }
    fn u8(&mut self) -> Result<u8> {
        let v = xy_core::util::u8_at(self.data, self.pos)?;
        self.pos += 1;
        Ok(v)
    }
    fn f32(&mut self) -> Result<f32> {
        let v = f32::from_le_bytes([
            xy_core::util::u8_at(self.data, self.pos)?,
            xy_core::util::u8_at(self.data, self.pos + 1)?,
            xy_core::util::u8_at(self.data, self.pos + 2)?,
            xy_core::util::u8_at(self.data, self.pos + 3)?,
        ]);
        self.pos += 4;
        Ok(v)
    }
    fn skip(&mut self, n: usize) {
        self.pos += n;
    }
    fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }
    fn sized_string(&mut self, len: usize) -> Result<String> {
        let start = self.pos;
        let end = (start + len).min(self.data.len());
        let raw = &self.data[start..end];
        let stop = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        self.pos = end;
        Ok(String::from_utf8_lossy(&raw[..stop]).into_owned())
    }
}

fn string_table(cur: &mut Cursor) -> Result<Vec<String>> {
    let count = cur.u32()? as usize;
    if count > 100_000 {
        return Err(Error::invalid(
            "gfmodel",
            format!("string table count {count}"),
        ));
    }
    let base = cur.pos;
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        cur.seek(base + i * 0x44);
        cur.u32()?; // hash
        out.push(cur.sized_string(0x40)?);
    }
    cur.seek(base + count * 0x44);
    Ok(out)
}

/// Load one GF model *section* (the blob whose +0x10 magic is "gfmodel").
/// `data` must start at the section start (Ohana's `loadModel` entry).
pub fn load_model_section(data: &[u8]) -> Result<Model> {
    let mut mdl = Model {
        name: "model".into(),
        bones: Vec::new(),
        meshes: Vec::new(),
    };
    if data.len() < 0x28 {
        return Err(Error::truncated(0, 0x28, "gfmodel"));
    }
    let mut cur = Cursor::new(data, 0);
    cur.skip(0x10);
    let magic = u64_le(data, cur.pos)?;
    cur.skip(8);
    if &magic.to_le_bytes()[..7] != b"gfmodel" {
        return Err(Error::magic(
            cur.pos - 8,
            "'gfmodel'",
            &magic.to_le_bytes()[..4],
        ));
    }
    let mdl_length = cur.u32()? as usize;
    cur.skip(4); // -1

    let _effect_names = string_table(&mut cur)?;
    let _texture_names = string_table(&mut cur)?;
    let material_names = string_table(&mut cur)?;
    let mesh_names = string_table(&mut cur)?;

    cur.skip(0x20); // 2 float4
    for _ in 0..16 {
        cur.f32()?; // transform matrix
    }
    let unk_len = cur.u32()? as usize;
    let unk_rel = cur.u32()? as usize;
    cur.skip(8);
    cur.skip(unk_rel + unk_len);

    let bones_count = cur.u32()? as usize;
    cur.skip(0xc);
    if bones_count > 4096 {
        return Err(Error::invalid(
            "gfmodel",
            format!("bone count {bones_count}"),
        ));
    }
    let mut bone_names: Vec<String> = Vec::with_capacity(bones_count);
    for _ in 0..bones_count {
        let name_len = cur.u8()? as usize;
        let name = cur.sized_string(name_len)?;
        let parent_len = cur.u8()? as usize;
        let parent_name = cur.sized_string(parent_len)?;
        cur.u8()?; // flags
        let mut bone = Bone {
            name: name.clone(),
            parent: -1,
            translation: [0.0; 3],
            rotation: [0.0; 3],
            scale: [0.0; 3],
        };
        for v in &mut bone.scale {
            *v = cur.f32()?;
        }
        for v in &mut bone.rotation {
            *v = cur.f32()?;
        }
        for v in &mut bone.translation {
            *v = cur.f32()?;
        }
        bone.parent = bone_names
            .iter()
            .position(|n| n == &parent_name)
            .map(|i| i as i16)
            .unwrap_or(-1);
        bone_names.push(name);
        mdl.bones.push(bone);
    }

    // Materials (mesh → material binding via shared names).
    // C#: Seek(mdlStart + mdlLength + 0x20) — mdlStart is the section start.
    let mut mat_mesh_binding: Vec<String> = Vec::new();
    cur.seek(mdl_length + 0x20);
    for _mname in &material_names {
        cur.skip(8); // "material" magic u64
        let mat_length = cur.u32()? as usize;
        cur.skip(4);
        let mat_start = cur.pos;
        let mut first_name = String::new();
        for n in 0..4 {
            cur.u32()?; // hash
            let nl = cur.u8()? as usize;
            let name = cur.sized_string(nl)?;
            if n == 0 {
                first_name = name;
            }
        }
        mat_mesh_binding.push(first_name);
        cur.skip(0xac);
        let tex_coords_start = cur.pos;
        for unit in 0..3 {
            cur.seek(tex_coords_start + unit * 0x42);
            cur.u32()?; // hash
            let tl = cur.u8()? as usize;
            let tex_name = cur.sized_string(tl)?;
            if tex_name.is_empty() {
                break;
            }
            cur.u16()?; // unit index
            cur.skip(20); // 5 × f32 coordinator
            cur.skip(8); // wrap U/V
        }
        cur.seek(mat_start + mat_length);
    }

    // Meshes.
    for _ in &mesh_names {
        cur.skip(8); // "mesh" magic u64
        let mesh_length = cur.u32()? as usize;
        cur.skip(4);
        let mesh_start = cur.pos;
        cur.skip(0x80);

        let info = sub_mesh_info(&mut cur)?;
        for sm in 0..info.count {
            let mut mesh = Mesh {
                name: info.names[sm].clone(),
                material_id: mat_mesh_binding
                    .iter()
                    .position(|n| n == &info.names[sm])
                    .map(|i| i as i32)
                    .unwrap_or(-1),
                vertices: Vec::new(),
            };
            let node_list = &info.node_lists[sm];
            let vtx_reader = &info.cmd_buffers[sm * 3];
            let idx_reader = &info.cmd_buffers[sm * 3 + 2];

            let stride = vtx_reader.buffer0_stride() as usize;
            let total_attrs = vtx_reader.buffer0_total_attributes();
            let main_perm = vtx_reader.vsh_permutation();
            let buf_perm = vtx_reader.buffer0_permutation();
            let formats = vtx_reader.vsh_formats();

            let idx_short = idx_reader.index_format_short();
            let idx_total = idx_reader.index_total_vertices() as usize;

            let vtx_buffer_start = cur.pos;
            cur.skip(info.vtx_lengths[sm] as usize);

            for _ in 0..idx_total {
                let index = if idx_short {
                    cur.u16()? as usize
                } else {
                    cur.u8()? as usize
                };
                let data_pos = cur.pos;
                cur.seek(vtx_buffer_start + index * stride);

                let mut vertex = Vertex {
                    bones: [0; 4],
                    weights: [1.0, 0.0, 0.0, 0.0],
                    ..Default::default()
                };
                for attribute in 0..total_attrs as usize {
                    let att = main_perm[buf_perm[attribute] as usize];
                    let mut fmt = formats[buf_perm[attribute] as usize];
                    if att == 8 {
                        fmt.ty = 1; // bone weight is always unsigned byte
                    }
                    let mut vec = [0f32; 4];
                    for v in vec.iter_mut().take(fmt.len as usize + 1) {
                        *v = match fmt.ty {
                            0 => cur.u8()? as i8 as f32,
                            1 => cur.u8()? as f32,
                            2 => cur.u16()? as i16 as f32,
                            _ => cur.f32()?,
                        };
                    }
                    match VshAttribute::from_id(att) {
                        VshAttribute::Position => vertex.position = [vec[0], vec[1], vec[2]],
                        VshAttribute::Normal => vertex.normal = [vec[0], vec[1], vec[2]],
                        VshAttribute::TexCoord0 => vertex.uv = [vec[0], vec[1]],
                        VshAttribute::BoneIndex => {
                            // addNode skips 0xff entries and maps through the
                            // sub-mesh node list.
                            for (c, slot) in vertex.bones.iter_mut().enumerate() {
                                if c > fmt.len as usize {
                                    break;
                                }
                                let idx = vec[c] as usize;
                                if idx != 0xff && idx < node_list.len() {
                                    *slot = node_list[idx];
                                }
                            }
                        }
                        VshAttribute::BoneWeight => {
                            for (c, w) in vertex.weights.iter_mut().enumerate() {
                                if c <= fmt.len as usize {
                                    *w = vec[c] / 255.0;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                cur.seek(data_pos);
                if mesh.vertices.len() < 10_000_000 {
                    mesh.vertices.push(vertex);
                } else {
                    return Err(Error::invalid("gfmodel", "vertex count runaway"));
                }
            }
            mdl.meshes.push(mesh);
        }
        cur.seek(mesh_start + mesh_length);
    }
    Ok(mdl)
}

/// Load a full GF model *container* (peek magic u32 0x00010000 — the
/// 5-section layout from `GfModel.load`).
pub fn load_container(data: &[u8]) -> Result<ModelGroup> {
    let mut group = ModelGroup::default();
    if data.len() < 0x28 {
        return Err(Error::truncated(0, 0x28, "gfmodel container"));
    }
    let mut cur = Cursor::new(data, 0);
    cur.skip(4);
    let mut counts = [0u32; 5];
    for c in counts.iter_mut() {
        *c = cur.u32()?;
    }
    let mut base = cur.pos;
    for count in counts {
        for i in 0..count as usize {
            cur.seek(base + i * 4);
            let desc = cur.u32()? as usize;
            cur.seek(desc);
            let name_len = cur.u8()? as usize;
            let name = cur.sized_string(name_len)?;
            let desc_address = cur.u32()? as usize;
            if desc_address >= data.len() {
                return Err(Error::invalid(
                    "gfmodel container",
                    "descriptor out of bounds",
                ));
            }
            let mut model = load_model_section(&data[desc_address..])?;
            model.name = name;
            group.models.push(model);
        }
        base += count as usize * 4;
    }
    Ok(group)
}

/// Load a PkmnContainer sub-file, routing by magic like Ohana's `FileIO`.
pub fn load_subfile(data: &[u8]) -> Result<Option<ModelGroup>> {
    if data.len() < 0x28 {
        return Ok(None);
    }
    let magic = u32_le(data, 0)?;
    match magic {
        0x0001_0000 => Ok(Some(load_container(data)?)),
        0x1512_2117 => Ok(Some(ModelGroup {
            models: vec![load_model_section(data)?],
        })),
        _ => Ok(None), // BCH and texture/anim formats pending (PLAN.md Phase 2)
    }
}

/// Route any Pokémon graphics blob: PkmnContainer tags (`PC`, `PF`, `PT`, …)
/// merge all model-bearing sub-files; bare GF sections/containers decode
/// directly. `Ok(None)` = recognized-but-unported (BCH, textures, anims).
pub fn load_any(data: &[u8]) -> Result<Option<ModelGroup>> {
    if data.len() >= 4 {
        let magic = u32_le(data, 0)?;
        if magic == 0x0001_0000 {
            return Ok(Some(load_container(data)?));
        }
        if magic == 0x1512_2117 {
            return Ok(Some(ModelGroup {
                models: vec![load_model_section(data)?],
            }));
        }
        let two = [data[0], data[1]];
        if two.iter().all(|b| b.is_ascii_alphabetic()) {
            let container = crate::pkmn::PkmnContainer::parse(data)?;
            let mut group = ModelGroup::default();
            for entry in &container.entries {
                if let Some(g) = load_subfile(entry)? {
                    group.models.extend(g.models);
                }
            }
            return Ok(Some(group));
        }
    }
    Ok(None)
}

struct SubMeshInfo {
    cmd_buffers: Vec<PicaCommandReader>,
    node_lists: Vec<Vec<u16>>,
    vtx_lengths: Vec<u32>,
    idx_lengths: Vec<u32>,
    names: Vec<String>,
    count: usize,
}

fn sub_mesh_info(cur: &mut Cursor) -> Result<SubMeshInfo> {
    let mut info = SubMeshInfo {
        cmd_buffers: Vec::new(),
        node_lists: Vec::new(),
        vtx_lengths: Vec::new(),
        idx_lengths: Vec::new(),
        names: Vec::new(),
        count: 0,
    };
    let mut curr_cmd_idx = 0i32;
    let mut total_cmds = 0i32;
    while (curr_cmd_idx + 1 < total_cmds) || curr_cmd_idx == 0 {
        let cmd_length = cur.u32()?;
        curr_cmd_idx = cur.u32()? as i32;
        total_cmds = cur.u32()? as i32;
        cur.skip(4);
        let words = cmd_length / 4;
        if words > 0x100000 {
            return Err(Error::invalid("gfmodel", "command buffer too large"));
        }
        let start = cur.pos;
        info.cmd_buffers
            .push(PicaCommandReader::parse(&cur.data[start..], words)?);
        cur.skip(words as usize * 4);
    }
    info.count = (total_cmds / 3) as usize;
    if info.count > 4096 {
        return Err(Error::invalid(
            "gfmodel",
            format!("sub-mesh count {}", info.count),
        ));
    }
    for _ in 0..info.count {
        cur.u32()?; // hash
        let name_len = cur.u32()? as usize;
        let name = cur.sized_string(name_len)?;
        cur.u8()?; // node list length
        let node_list_start = cur.pos;
        let node_list_len = xy_core::util::u8_at(cur.data, node_list_start - 1)? as usize;
        let mut node_list: Vec<u16> = Vec::with_capacity(node_list_len);
        for n in 0..node_list_len {
            node_list.push(u16::from(xy_core::util::u8_at(
                cur.data,
                node_list_start + n,
            )?));
        }
        cur.seek(node_list_start + 0x20);
        cur.u32()?; // vtx count
        cur.u32()?; // idx count
        info.vtx_lengths.push(cur.u32()?);
        info.idx_lengths.push(cur.u32()?);
        info.names.push(name);
        info.node_lists.push(node_list);
    }
    Ok(info)
}
