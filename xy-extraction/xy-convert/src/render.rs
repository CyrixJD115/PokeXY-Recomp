//! Headless software 3D renderer — the CLI replacement for an interactive
//! viewer: rasterizes a decoded model (BCH/CGFX/container) straight to PNG.
//!
//! Classic z-buffered triangle rasterizer: edge-function barycentric
//! coverage, perspective-correct UV interpolation, nearest sampling, a
//! headlight + ambient lambert term. Pokémon 3DS meshes are small, so
//! turntables of dozens of frames render in well under a second each.

use xy_gfx::texture::write_png;

type PngResult<T> = std::result::Result<T, xy_gfx::Error>;

use crate::model::{RenderMesh, SceneBone, SceneMaterial, SceneModel};

/// Orbit camera around a target point.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub yaw_deg: f32,
    pub pitch_deg: f32,
    pub dist: f32,
    pub target: [f32; 3],
    pub fov_y_deg: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            yaw_deg: 35.0,
            pitch_deg: 18.0,
            dist: 10.0,
            target: [0.0, 0.0, 0.0],
            fov_y_deg: 42.0,
        }
    }
}

impl Camera {
    /// Camera fitted to the model bounds at the given angles.
    pub fn fit(scene: &SceneModel, yaw_deg: f32, pitch_deg: f32) -> Camera {
        let mut cam = Camera {
            yaw_deg,
            pitch_deg,
            ..Default::default()
        };
        if let Some((min, max)) = scene.bounds() {
            cam.target = [
                (min[0] + max[0]) * 0.5,
                (min[1] + max[1]) * 0.5,
                (min[2] + max[2]) * 0.5,
            ];
            let size =
                ((max[0] - min[0]).powi(2) + (max[1] - min[1]).powi(2) + (max[2] - min[2]).powi(2))
                    .sqrt();
            cam.dist = (size * 1.4).max(0.2);
        }
        cam
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shading {
    Textured,
    Material,
    Wireframe,
    Normals,
    Skeleton,
}

impl Shading {
    pub const NAMES: [&'static str; 5] =
        ["textured", "material", "wireframe", "normals", "skeleton"];

    pub fn from_name(s: &str) -> Option<Shading> {
        Some(match s {
            "textured" => Shading::Textured,
            "material" => Shading::Material,
            "wireframe" => Shading::Wireframe,
            "normals" => Shading::Normals,
            "skeleton" => Shading::Skeleton,
            _ => return None,
        })
    }
}

/// Render statistics alongside the image bytes.
pub struct RenderStats {
    pub width: u32,
    pub height: u32,
    pub triangles_drawn: usize,
    pub render_ms: f32,
}

/// One rasterized frame (RGBA in memory).
pub struct Frame {
    pub rgba: Vec<u8>,
    pub stats: RenderStats,
}

/// Render one scene with one camera.
pub fn render(
    scene: &SceneModel,
    cam: &Camera,
    shading: Shading,
    cull: bool,
    width: u32,
    height: u32,
) -> Frame {
    let t0 = std::time::Instant::now();
    let mut px = vec![0u8; (width as usize) * (height as usize) * 4];
    let mut zbuf = vec![f32::INFINITY; (width as usize) * (height as usize)];

    let eye = eye_of(cam);
    let (right, up) = basis(cam, &eye);
    let fwd = norm(&sub(&cam.target, &eye));
    let aspect = width as f32 / height as f32;
    let f = 1.0 / (cam.fov_y_deg.to_radians() / 2.0).tan();
    let near = 0.02;

    let project = |p: [f32; 3]| -> [f32; 3] {
        let d = sub(&p, &eye);
        let x = dot(&d, &right);
        let y = dot(&d, &up);
        // "forward distance" convention: w is positive for visible points
        // (fwd points from the eye toward the target), which keeps the
        // z-buffer ordering natural (bigger = farther).
        let w = dot(&d, &fwd);
        if w <= near {
            return [f32::NAN; 3];
        }
        let ndc_x = (x * f / (aspect * w) + 1.0) * 0.5;
        let ndc_y = (y * f / w + 1.0) * 0.5;
        [ndc_x * width as f32, (1.0 - ndc_y) * height as f32, w]
    };

    let mut tris = 0usize;

    if shading == Shading::Skeleton {
        let globals = bone_globals(&scene.bones);
        let joints: Vec<[f32; 3]> = globals.iter().map(|g| [g[12], g[13], g[14]]).collect();
        for (i, bone) in scene.bones.iter().enumerate() {
            let Some(&j) = joints.get(i) else { continue };
            let sp = project(j);
            if sp[0].is_nan() {
                continue;
            }
            fill_circle(
                &mut px,
                &mut zbuf,
                width,
                height,
                sp,
                3.5,
                [255, 140, 150, 255],
            );
            if bone.parent >= 0 {
                let Some(&pj) = joints.get(bone.parent as usize) else {
                    continue;
                };
                let pp = project(pj);
                if !pp[0].is_nan() {
                    draw_line(
                        &mut px,
                        &mut zbuf,
                        width,
                        height,
                        sp,
                        pp,
                        [200, 210, 235, 255],
                    );
                }
            }
        }
        return finish(px, width, height, tris, t0);
    }

    for mesh in &scene.meshes {
        if mesh.positions.is_empty() {
            continue;
        }
        let material: Option<&SceneMaterial> = scene.materials.get(mesh.material_index);
        let texture = material
            .and_then(|m| m.texture_name.as_ref())
            .and_then(|n| scene.textures.iter().find(|t| &t.name == n));
        let base = material
            .map(|m| {
                [
                    (m.diffuse[0] * 255.0) as u8,
                    (m.diffuse[1] * 255.0) as u8,
                    (m.diffuse[2] * 255.0) as u8,
                ]
            })
            .unwrap_or([180, 180, 190]);

        let screen: Vec<[f32; 3]> = mesh.positions.iter().map(|p| project(*p)).collect();

        for tri in mesh.indices.as_chunks::<3>().0 {
            let (ia, ib, ic) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
            let (a, b, c) = (screen[ia], screen[ib], screen[ic]);
            if a[0].is_nan() || b[0].is_nan() || c[0].is_nan() {
                continue; // near-plane-crossing triangles skipped (no clipping v1)
            }
            tris += 1;

            if shading == Shading::Wireframe {
                let col = [200, 215, 255, 255];
                draw_line(&mut px, &mut zbuf, width, height, a, b, col);
                draw_line(&mut px, &mut zbuf, width, height, b, c, col);
                draw_line(&mut px, &mut zbuf, width, height, c, a, col);
                continue;
            }

            let area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cull && area <= 0.0 {
                continue;
            }
            if area == 0.0 {
                continue;
            }
            let area_inv = 1.0 / area;

            let min_x = a[0].min(b[0]).min(c[0]).floor().max(0.0) as i32;
            let max_x = a[0].max(b[0]).max(c[0]).ceil().min(width as f32 - 1.0) as i32;
            let min_y = a[1].min(b[1]).min(c[1]).floor().max(0.0) as i32;
            let max_y = a[1].max(b[1]).max(c[1]).ceil().min(height as f32 - 1.0) as i32;
            if min_x > max_x || min_y > max_y {
                continue;
            }

            let pa = mesh.positions[ia];
            let pb = mesh.positions[ib];
            let pc = mesh.positions[ic];
            let face_n = norm(&cross(&sub(&pb, &pa), &sub(&pc, &pa)));
            let view_dir = norm(&sub(&eye, &centroid(&[pa, pb, pc])));
            let lambert = if shading == Shading::Normals {
                1.0
            } else {
                (dot(&face_n, &view_dir).max(0.0) * 0.72 + 0.3).min(1.0)
            };

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let px_f = x as f32 + 0.5;
                    let py_f = y as f32 + 0.5;
                    let w0 = (b[0] - px_f) * (c[1] - py_f) - (b[1] - py_f) * (c[0] - px_f);
                    let w1 = (c[0] - px_f) * (a[1] - py_f) - (c[1] - py_f) * (a[0] - px_f);
                    let w2 = (a[0] - px_f) * (b[1] - py_f) - (a[1] - py_f) * (b[0] - px_f);
                    let inside = (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0)
                        || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0);
                    if !inside {
                        continue;
                    }
                    let z0 = w0 * area_inv;
                    let z1 = w1 * area_inv;
                    let z2 = w2 * area_inv;
                    // perspective-correct interpolation
                    let iw0 = z0 / a[2];
                    let iw1 = z1 / b[2];
                    let iw2 = z2 / c[2];
                    let iw_sum = iw0 + iw1 + iw2;
                    let depth = z0 * a[2] + z1 * b[2] + z2 * c[2];
                    let idx = y as usize * width as usize + x as usize;
                    if depth >= zbuf[idx] {
                        continue;
                    }
                    let color: [u8; 3] = match shading {
                        Shading::Normals => {
                            let n = if !mesh.normals.is_empty() {
                                norm(&interp3(
                                    mesh.normals[ia],
                                    mesh.normals[ib],
                                    mesh.normals[ic],
                                    iw0,
                                    iw1,
                                    iw2,
                                    iw_sum,
                                ))
                            } else {
                                face_n
                            };
                            [
                                ((n[0] * 0.5 + 0.5) * 255.0) as u8,
                                ((n[1] * 0.5 + 0.5) * 255.0) as u8,
                                ((n[2] * 0.5 + 0.5) * 255.0) as u8,
                            ]
                        }
                        Shading::Textured if texture.is_some() => {
                            let t = texture.unwrap();
                            let uv = interp2(
                                mesh.uvs[ia],
                                mesh.uvs[ib],
                                mesh.uvs[ic],
                                iw0,
                                iw1,
                                iw2,
                                iw_sum,
                            );
                            sample_nearest(&t.rgba, t.width, t.height, uv[0], uv[1])
                        }
                        _ => base,
                    };
                    zbuf[idx] = depth;
                    let o = idx * 4;
                    px[o] = (color[0] as f32 * lambert) as u8;
                    px[o + 1] = (color[1] as f32 * lambert) as u8;
                    px[o + 2] = (color[2] as f32 * lambert) as u8;
                    px[o + 3] = 255;
                }
            }
        }
    }

    finish(px, width, height, tris, t0)
}

/// Render a full turntable (N frames around the Y axis) starting at the
/// camera's yaw. Frame i is at `yaw + 360·i/N`.
pub fn render_turntable(
    scene: &SceneModel,
    cam: &Camera,
    shading: Shading,
    cull: bool,
    width: u32,
    height: u32,
    frames: usize,
) -> Vec<Frame> {
    (0..frames.max(1))
        .map(|i| {
            let mut c = *cam;
            c.yaw_deg += 360.0 * i as f32 / frames.max(1) as f32;
            render(scene, &c, shading, cull, width, height)
        })
        .collect()
}

/// Encode a frame to PNG bytes.
pub fn frame_to_png(frame: &Frame) -> PngResult<Vec<u8>> {
    let mut out = Vec::new();
    write_png(
        &frame.rgba,
        frame.stats.width,
        frame.stats.height,
        &mut std::io::BufWriter::new(&mut out),
    )?;
    Ok(out)
}

fn eye_of(cam: &Camera) -> [f32; 3] {
    let yaw = cam.yaw_deg.to_radians();
    let pitch = cam.pitch_deg.to_radians();
    let dir = [
        pitch.cos() * yaw.sin(),
        pitch.sin(),
        pitch.cos() * yaw.cos(),
    ];
    [
        cam.target[0] + dir[0] * cam.dist,
        cam.target[1] + dir[1] * cam.dist,
        cam.target[2] + dir[2] * cam.dist,
    ]
}

fn basis(cam: &Camera, eye: &[f32; 3]) -> ([f32; 3], [f32; 3]) {
    let fwd = norm(&sub(&cam.target, eye));
    let world_up = [0.0, 1.0, 0.0];
    let mut right = cross(&fwd, &world_up);
    if right == [0.0; 3] {
        right = [1.0, 0.0, 0.0];
    }
    let right = norm(&right);
    let up = cross(&right, &fwd);
    (right, norm(&up))
}

fn finish(px: Vec<u8>, width: u32, height: u32, tris: usize, t0: std::time::Instant) -> Frame {
    Frame {
        rgba: px,
        stats: RenderStats {
            width,
            height,
            triangles_drawn: tris,
            render_ms: t0.elapsed().as_secs_f32() * 1000.0,
        },
    }
}

// ---- math helpers ----

fn sub(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn norm(a: &[f32; 3]) -> [f32; 3] {
    let l = dot(a, a).sqrt();
    if l > 1e-12 {
        [a[0] / l, a[1] / l, a[2] / l]
    } else {
        [0.0, 1.0, 0.0]
    }
}
fn centroid(ps: &[[f32; 3]; 3]) -> [f32; 3] {
    [
        (ps[0][0] + ps[1][0] + ps[2][0]) / 3.0,
        (ps[0][1] + ps[1][1] + ps[2][1]) / 3.0,
        (ps[0][2] + ps[1][2] + ps[2][2]) / 3.0,
    ]
}
fn interp2(a: [f32; 2], b: [f32; 2], c: [f32; 2], w0: f32, w1: f32, w2: f32, s: f32) -> [f32; 2] {
    [
        (a[0] * w0 + b[0] * w1 + c[0] * w2) / s,
        (a[1] * w0 + b[1] * w1 + c[1] * w2) / s,
    ]
}
fn interp3(a: [f32; 3], b: [f32; 3], c: [f32; 3], w0: f32, w1: f32, w2: f32, s: f32) -> [f32; 3] {
    [
        (a[0] * w0 + b[0] * w1 + c[0] * w2) / s,
        (a[1] * w0 + b[1] * w1 + c[1] * w2) / s,
        (a[2] * w0 + b[2] * w1 + c[2] * w2) / s,
    ]
}

/// Nearest texture sample with clamp addressing (RGBA buffer).
pub fn sample_nearest(rgba: &[u8], w: usize, h: usize, u: f32, v: f32) -> [u8; 3] {
    if w == 0 || h == 0 {
        return [255, 0, 255];
    }
    let x = (u.clamp(0.0, 1.0) * (w - 1) as f32).round() as usize;
    let y = (v.clamp(0.0, 1.0) * (h - 1) as f32).round() as usize;
    let o = (y * w + x) * 4;
    [rgba[o], rgba[o + 1], rgba[o + 2]]
}

fn put(px: &mut [u8], zbuf: &mut [f32], w: u32, x: i32, y: i32, z: f32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= w as i32 || y >= (zbuf.len() / w as usize) as i32 {
        return;
    }
    let idx = y as usize * w as usize + x as usize;
    if z < zbuf[idx] {
        zbuf[idx] = z;
        let o = idx * 4;
        px[o] = color[0];
        px[o + 1] = color[1];
        px[o + 2] = color[2];
        px[o + 3] = color[3];
    }
}

fn draw_line(
    px: &mut [u8],
    zbuf: &mut [f32],
    w: u32,
    _h: u32,
    a: [f32; 3],
    b: [f32; 3],
    color: [u8; 4],
) {
    let steps = ((b[0] - a[0]).abs().max((b[1] - a[1]).abs()).ceil() as i32).max(1);
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = a[0] + (b[0] - a[0]) * t;
        let y = a[1] + (b[1] - a[1]) * t;
        let z = a[2] + (b[2] - a[2]) * t;
        put(px, zbuf, w, x.round() as i32, y.round() as i32, z, color);
    }
}

fn fill_circle(
    px: &mut [u8],
    zbuf: &mut [f32],
    w: u32,
    _h: u32,
    c: [f32; 3],
    r: f32,
    color: [u8; 4],
) {
    let x0 = (c[0] - r).floor() as i32;
    let x1 = (c[0] + r).ceil() as i32;
    let y0 = (c[1] - r).floor() as i32;
    let y1 = (c[1] + r).ceil() as i32;
    for y in y0..=y1 {
        for x in x0..=x1 {
            let dx = x as f32 - c[0];
            let dy = y as f32 - c[1];
            if dx * dx + dy * dy <= r * r {
                put(px, zbuf, w, x, y, c[2], color);
            }
        }
    }
}

/// Global bind-pose transforms per bone (`T·Rx·Ry·Rz·S`, column-major;
/// parents precede children in BCH bone order).
pub fn bone_globals(bones: &[SceneBone]) -> Vec<[f32; 16]> {
    let mut globals = Vec::with_capacity(bones.len());
    for (i, b) in bones.iter().enumerate() {
        let local = compose(&b.translation, &b.rotation_euler, &b.scale);
        let g = if b.parent >= 0 && (b.parent as usize) < i {
            mul(&globals[b.parent as usize], &local)
        } else {
            local
        };
        globals.push(g);
    }
    globals
}

fn compose(t: &[f32; 3], r: &[f32; 3], s: &[f32; 3]) -> [f32; 16] {
    let (sin_x, cos_x) = r[0].sin_cos();
    let (sin_y, cos_y) = r[1].sin_cos();
    let (sin_z, cos_z) = r[2].sin_cos();
    let r00 = cos_y * cos_z;
    let r01 = -cos_y * sin_z;
    let r02 = sin_y;
    let r10 = sin_x * sin_y * cos_z + cos_x * sin_z;
    let r11 = -sin_x * sin_y * sin_z + cos_x * cos_z;
    let r12 = -sin_x * cos_y;
    let r20 = -cos_x * sin_y * cos_z + sin_x * sin_z;
    let r21 = cos_x * sin_y * sin_z + sin_x * cos_z;
    let r22 = cos_x * cos_y;
    [
        r00 * s[0],
        r10 * s[0],
        r20 * s[0],
        0.0, //
        r01 * s[1],
        r11 * s[1],
        r21 * s[1],
        0.0, //
        r02 * s[2],
        r12 * s[2],
        r22 * s[2],
        0.0, //
        t[0],
        t[1],
        t[2],
        1.0,
    ]
}

fn mul(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut o = [0f32; 16];
    for c in 0..4 {
        for r in 0..4 {
            o[c * 4 + r] = a[r] * b[c * 4]
                + a[4 + r] * b[c * 4 + 1]
                + a[8 + r] * b[c * 4 + 2]
                + a[12 + r] * b[c * 4 + 3];
        }
    }
    o
}

#[allow(dead_code)]
fn _mesh_used(_: &RenderMesh) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RenderMesh;

    fn scene() -> SceneModel {
        let mut m = RenderMesh {
            name: "quad".into(),
            material_index: 0,
            ..Default::default()
        };
        m.positions = vec![
            [-1.0, -1.0, 0.0],
            [1.0, -1.0, 0.0],
            [1.0, 1.0, 0.0],
            [-1.0, -1.0, 0.0],
            [1.0, 1.0, 0.0],
            [-1.0, 1.0, 0.0],
        ];
        m.normals = vec![[0.0, 0.0, 1.0]; 6];
        m.uvs = vec![[0.0, 0.0]; 6];
        m.indices = vec![0, 1, 2, 3, 4, 5];
        SceneModel {
            name: "t".into(),
            meshes: vec![m],
            bones: vec![SceneBone {
                name: "root".into(),
                parent: -1,
                translation: [0.0; 3],
                rotation_euler: [0.0; 3],
                scale: [1.0; 3],
            }],
            ..Default::default()
        }
    }

    #[test]
    fn renders_something() {
        let r = render(
            &scene(),
            &Camera::default(),
            Shading::Material,
            false,
            64,
            64,
        );
        assert_eq!(r.rgba.len(), 64 * 64 * 4);
        let opaque = r.rgba.chunks_exact(4).filter(|c| c[3] != 0).count();
        assert!(opaque > 50, "expected covered pixels, got {opaque}");
        assert_eq!(r.stats.triangles_drawn, 2);
    }

    #[test]
    fn camera_moves_change_pixels() {
        let a = render(
            &scene(),
            &Camera::default(),
            Shading::Material,
            false,
            64,
            64,
        );
        let mut cam = Camera::default();
        cam.yaw_deg += 70.0;
        let b = render(&scene(), &cam, Shading::Material, false, 64, 64);
        assert_ne!(a.rgba, b.rgba);
    }

    #[test]
    fn skeleton_mode_draws_joints() {
        let r = render(
            &scene(),
            &Camera::default(),
            Shading::Skeleton,
            false,
            64,
            64,
        );
        assert!(r.rgba.chunks_exact(4).any(|c| c[3] != 0));
    }

    #[test]
    fn turntable_frames_advance() {
        let frames = render_turntable(
            &scene(),
            &Camera::default(),
            Shading::Material,
            false,
            32,
            32,
            4,
        );
        assert_eq!(frames.len(), 4);
        assert_ne!(frames[0].rgba, frames[1].rgba);
    }

    #[test]
    fn texture_sampling_clamps() {
        let rgba = vec![10, 20, 30, 255];
        assert_eq!(sample_nearest(&rgba, 1, 1, -5.0, 9.0), [10, 20, 30]);
    }
}
