//! GameFreak motion files — port of SPICA `Formats/GFL2/Motion`
//! (`GFMotion.cs` + friends). Pokémon X/Y stores every Pokémon animation
//! as one of these (`a/0/9/3/*.bin` — battle/field motions, 945 files;
//! `a/0/9/4/*.bin` — 579 more; magic `02 00 xx 00`, version 2).
//!
//! Binary layout (verified against the SPICA source; all offsets u32 LE):
//!
//! ```text
//! 0x00 u32 magic (0x00090002-style version stamps, unvalidated)
//! 0x04 u32 section count
//!      sections × { u32 kind (0 sub-header, 1 skeletal, 3 material,
//!                              6 visibility), u32 length, u32 address }
//!      (addresses are absolute from the file start)
//!
//! Sub-header (kind 0): u32 framesCount | u16 flags (bit0 = looping) |
//!      u16 flags (bit0 = blended) | f32×3 region min | f32×3 region max |
//!      u32 hash
//!
//! Skeletal anim (kind 1): i32 bone-name count | u32 names block length |
//!      names (u8-length-prefixed strings, back to back) → seek
//!      (names block start + length) → per bone:
//!      { u32 flags (bit31 clear = axis-angle rotation; bits 0..26 = 3 per
//!        track: &7 == 3 constant f32, 4/5 keyframe list) | u32 length }
//!      then 9 tracks (scale xyz, rotation xyz, translation xyz):
//!        constant (3): one f32
//!        list (4/5): u32 count → count × frame ids (u16 if framesCount
//!        > 0xff else u8) → 4-align → bit0 of the track's flags:
//!          set  → f32 value + f32 slope per key
//!          clear→ f32 valueScale/valueOffset/slopeScale/slopeOffset, then
//!                 u16 value + u16 slope per key, normalized
//!                 (v / 0xffff * scale + offset)
//!
//! Visibility anim (kind 6): i32 mesh-name count | u32 names length |
//!      names → per mesh: (framesCount + 1) bits LSB-first (u8 stream).
//!
//! Material anim (kind 3): i32 name count | u32 names length | u32 units
//!      per material | names → per material × unit:
//!      { u32 unitIndex | u32 flags (5 tracks × 3 bits) | u32 length }
//!      then 5 tracks (scale uv, rotation, translation uv), same encoding.
//! ```

use xy_core::error::{Error, Result};
use xy_core::util::{f32_le, u16_le, u32_le, u8_at};

/// One key of a Hermite-interpolated track.
#[derive(Debug, Clone, Copy)]
pub struct KeyFrame {
    pub frame: i32,
    pub value: f32,
    pub slope: f32,
}

/// One 9-track bone channel set (`GFMotBoneTransform`).
#[derive(Debug, Clone)]
pub struct BoneTrack {
    pub name: String,
    /// Rotation encoded as axis-angle (bit31 of the flags clear) — else Euler.
    pub is_axis_angle: bool,
    /// `[scaleX, scaleY, scaleZ, rotX, rotY, rotZ, transX, transY, transZ]`.
    pub tracks: Vec<Vec<KeyFrame>>,
}

impl BoneTrack {
    /// Sample a track at `frame` (SPICA `SetFrameValue`: constant for
    /// single keys, Hermite between neighbours, clamped at the ends).
    pub fn sample(track: &[KeyFrame], frame: f32) -> Option<f32> {
        match track.len() {
            0 => None,
            1 => Some(track[0].value),
            _ => {
                let lhs = track
                    .iter()
                    .rev()
                    .find(|k| k.frame as f32 <= frame)
                    .unwrap_or(track.first().unwrap());
                let rhs = track
                    .iter()
                    .find(|k| k.frame as f32 >= frame)
                    .unwrap_or(track.last().unwrap());
                if lhs.frame == rhs.frame {
                    return Some(lhs.value);
                }
                let diff = frame - lhs.frame as f32;
                let weight = diff / (rhs.frame - lhs.frame) as f32;
                Some(herp(
                    lhs.value, rhs.value, lhs.slope, rhs.slope, diff, weight,
                ))
            }
        }
    }

    /// Track order constant: index into `tracks` for a channel name.
    pub const SCALE_X: usize = 0;
    pub const ROTATION_Z: usize = 5;
    pub const TRANSLATION_Z: usize = 8;
}

/// `Interpolation.Herp` — cubic Hermite between two keys.
#[must_use]
pub fn herp(lhs: f32, rhs: f32, l_slope: f32, r_slope: f32, diff: f32, weight: f32) -> f32 {
    let mut result = lhs + (lhs - rhs) * (2.0 * weight - 3.0) * weight * weight;
    result += (diff * (weight - 1.0)) * (l_slope * (weight - 1.0) + r_slope * weight);
    result
}

/// Mesh visibility track: one bool per frame, LSB-first bit stream.
#[derive(Debug, Clone)]
pub struct VisibilityTrack {
    pub name: String,
    pub values: Vec<bool>,
}

/// A decoded GFMotion file (material tracks are skipped — XY only uses
/// them for UV offsets; skeletal + visibility cover the interesting data).
#[derive(Debug, Default)]
pub struct GfMotion {
    pub frames_count: u32,
    pub is_looping: bool,
    pub is_blended: bool,
    pub region_min: [f32; 3],
    pub region_max: [f32; 3],
    pub skeletal: Option<Vec<BoneTrack>>,
    pub visibility: Option<Vec<VisibilityTrack>>,
}

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
            return Err(Error::truncated(pos, 0, "gf_motion seek"));
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
    fn f32(&mut self) -> Result<f32> {
        let v = f32_le(self.d, self.pos)?;
        self.pos += 4;
        Ok(v)
    }
    /// u8-length-prefixed string (`ReadByteLengthString`).
    fn lp_string(&mut self) -> Result<String> {
        let len = self.u8()? as usize;
        let s = self
            .d
            .get(self.pos..self.pos + len)
            .ok_or_else(|| Error::truncated(self.pos, len, "gf_motion string"))?;
        self.pos += len;
        Ok(String::from_utf8_lossy(s).into_owned())
    }
}

impl GfMotion {
    /// Parse a GFMotion file (magic `02 00 …` — XY version 2).
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < 8 {
            return Err(Error::truncated(0, 8, "gf_motion header"));
        }
        let mut r = Rdr::new(data);
        let _magic = r.u32()?;
        let section_count = r.u32()? as usize;
        if section_count == 0 || section_count > 16 {
            return Err(Error::invalid(
                "gf_motion header",
                format!("section count {section_count}"),
            ));
        }
        let mut kinds = Vec::with_capacity(section_count);
        let mut addresses = Vec::with_capacity(section_count);
        for _ in 0..section_count {
            let kind = r.u32()?;
            let _length = r.u32()?;
            let address = r.u32()? as usize;
            kinds.push(kind);
            addresses.push(address);
        }

        // Sub-header (kind 0).
        r.seek(addresses[0])?;
        let frames_count = r.u32()?;
        let is_looping = r.u16()? & 1 != 0;
        let is_blended = r.u16()? & 1 != 0;
        let region_min = [r.f32()?, r.f32()?, r.f32()?];
        let region_max = [r.f32()?, r.f32()?, r.f32()?];

        let mut out = GfMotion {
            frames_count,
            is_looping,
            is_blended,
            region_min,
            region_max,
            skeletal: None,
            visibility: None,
        };

        for (&kind, &address) in kinds.iter().zip(&addresses).skip(1) {
            match kind {
                1 => out.skeletal = Some(read_skeletal(&mut r, address, frames_count)?),
                6 => out.visibility = Some(read_visibility(&mut r, address, frames_count)?),
                _ => {}
            }
        }
        Ok(out)
    }
}

/// Track lists with the shared `(count, frames, payload)` encoding.
fn read_track(r: &mut Rdr, flags: u32, frames_count: u32) -> Result<Vec<KeyFrame>> {
    let mut out = Vec::new();
    match flags & 7 {
        3 => out.push(KeyFrame {
            frame: 0,
            value: r.f32()?,
            slope: 0.0,
        }),
        4 | 5 => {
            let count = r.u32()? as usize;
            if count > 0x1_0000 {
                return Err(Error::invalid("gf_motion track", format!("{count} keys")));
            }
            let mut frames = Vec::with_capacity(count);
            for _ in 0..count {
                let f = if frames_count > 0xff {
                    r.u16()? as i32
                } else {
                    r.u8()? as i32
                };
                frames.push(f);
            }
            // 4-byte align.
            while r.pos & 3 != 0 {
                r.u8()?;
            }
            if flags & 1 != 0 {
                for &f in &frames {
                    out.push(KeyFrame {
                        frame: f,
                        value: r.f32()?,
                        slope: r.f32()?,
                    });
                }
            } else {
                let value_scale = r.f32()?;
                let value_offset = r.f32()?;
                let slope_scale = r.f32()?;
                let slope_offset = r.f32()?;
                for &f in &frames {
                    let value = (r.u16()? as f32 / 0xffff as f32) * value_scale + value_offset;
                    let slope = (r.u16()? as f32 / 0xffff as f32) * slope_scale + slope_offset;
                    out.push(KeyFrame {
                        frame: f,
                        value,
                        slope,
                    });
                }
            }
        }
        _ => {} // 0-2: track absent
    }
    Ok(out)
}

fn read_skeletal(r: &mut Rdr, at: usize, frames_count: u32) -> Result<Vec<BoneTrack>> {
    r.seek(at)?;
    let bone_names_count = r.u32()? as usize;
    let bone_names_length = r.u32()? as usize;
    if bone_names_count > 0x1000 {
        return Err(Error::invalid(
            "gf_motion skeleton",
            format!("{bone_names_count} bones"),
        ));
    }
    let names_start = r.pos;
    let mut names = Vec::with_capacity(bone_names_count);
    for _ in 0..bone_names_count {
        names.push(r.lp_string()?);
    }
    // The names block is variable length; bones follow at start + length.
    r.seek(names_start + bone_names_length)?;

    let mut bones = Vec::with_capacity(bone_names_count);
    for name in names {
        let flags = r.u32()?;
        let _length = r.u32()?;
        let is_axis_angle = flags >> 31 == 0;
        let mut tracks = Vec::with_capacity(9);
        let mut track_flags = flags;
        for _ in 0..9 {
            tracks.push(read_track(r, track_flags, frames_count)?);
            track_flags >>= 3;
        }
        bones.push(BoneTrack {
            name,
            is_axis_angle,
            tracks,
        });
    }
    Ok(bones)
}

fn read_visibility(r: &mut Rdr, at: usize, frames_count: u32) -> Result<Vec<VisibilityTrack>> {
    r.seek(at)?;
    let mesh_names_count = r.u32()? as usize;
    let mesh_names_length = r.u32()? as usize;
    if mesh_names_count > 0x1000 {
        return Err(Error::invalid(
            "gf_motion visibility",
            format!("{mesh_names_count} meshes"),
        ));
    }
    let names_start = r.pos;
    let mut names = Vec::with_capacity(mesh_names_count);
    for _ in 0..mesh_names_count {
        names.push(r.lp_string()?);
    }
    r.seek(names_start + mesh_names_length)?;

    let count = frames_count as usize + 1;
    let mut tracks = Vec::with_capacity(mesh_names_count);
    for name in names {
        let mut values = Vec::with_capacity(count);
        let mut value = 0u8;
        for index in 0..count {
            let bit = index & 7;
            if bit == 0 {
                value = r.u8()?;
            }
            values.push(value & (1 << bit) != 0);
        }
        tracks.push(VisibilityTrack { name, values });
    }
    Ok(tracks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn herp_matches_reference() {
        // SPICA Interpolation.Herp: LHS + (LHS-RHS)(2w-3)w² + diff(w-1)(LS(w-1)+RS·w)
        let v = herp(1.0, 3.0, 0.0, 0.0, 2.0, 0.5);
        // w=0.5: 1 + (1-3)(1-3)(0.25) = 1 + 1 = 2
        assert!((v - 2.0).abs() < 1e-6, "{v}");
        assert_eq!(herp(5.0, 9.0, 0.0, 0.0, 4.0, 0.0), 5.0);
        assert_eq!(herp(5.0, 9.0, 0.0, 0.0, 4.0, 1.0), 9.0);
    }

    #[test]
    fn sample_track_constant_and_interp() {
        let constant = vec![KeyFrame {
            frame: 0,
            value: 2.5,
            slope: 0.0,
        }];
        assert_eq!(BoneTrack::sample(&constant, 10.0), Some(2.5));
        assert_eq!(BoneTrack::sample(&[], 3.0), None);

        let linear_ish = vec![
            KeyFrame {
                frame: 0,
                value: 0.0,
                slope: 0.0,
            },
            KeyFrame {
                frame: 10,
                value: 10.0,
                slope: 0.0,
            },
        ];
        // Hermite with zero slopes at the midpoint sags (w=0.5 → 0.5 exactly
        // for symmetric values).
        let mid = BoneTrack::sample(&linear_ish, 5.0).unwrap();
        assert!((mid - 5.0).abs() < 1e-6, "{mid}");
        // Clamping outside the key range.
        assert_eq!(BoneTrack::sample(&linear_ish, 99.0), Some(10.0));
        assert_eq!(BoneTrack::sample(&linear_ish, -1.0), Some(0.0));
    }

    #[test]
    fn rejects_bogus_header() {
        assert!(GfMotion::parse(&[0u8; 4]).is_err());
        let mut data = vec![2u8, 0, 9, 0, 99, 0, 0, 0]; // 99 sections
        data.resize(64, 0);
        assert!(GfMotion::parse(&data).is_err());
    }
}
