use crate::attributes::*;
use glam::{vec2, vec3, Vec3, Mat3};
use image::{RgbaImage, Rgba};

pub struct Program {
    pub vertex_shader: Box<dyn Fn(VertexAttributes, UniformAttributes) -> VertexAttributes>,
    pub fragment_shader: Box<dyn Fn(VertexAttributes, UniformAttributes) -> FragmentAttributes>,
    pub blending_shader: Box<dyn Fn(FragmentAttributes, Rgba<u8>) -> Rgba<u8>>,
}

pub fn rasterize_triangle(program: &mut Program, uniform: UniformAttributes, v1: VertexAttributes, v2: VertexAttributes, v3: VertexAttributes, frame_buffer: &mut RgbaImage) {
    let mut p = vec![];
    p.push(v1.position / v1.position.w());
    p.push(v2.position / v2.position.w());
    p.push(v3.position / v3.position.w());

    p.iter_mut().for_each(|point| {
        point[0] = ((point[0] + 1.0) / 2.0) * frame_buffer.height() as f32;
        point[1] = ((point[1] + 1.0) / 2.0) * frame_buffer.width() as f32;
    });

    let f32_cmp = |a: &f32, b: &f32| a.partial_cmp(b).unwrap();
    let x: Vec<f32> = p.iter().map(|point| point[0]).collect();
    let y: Vec<f32> = p.iter().map(|point| point[1]).collect();

    let lx = x.clone().into_iter().min_by(f32_cmp).unwrap().max(0.0) as u32;
    let ly = y.clone().into_iter().min_by(f32_cmp).unwrap().max(0.0) as u32;
    let ux = x.into_iter().max_by(f32_cmp).unwrap().min(frame_buffer.height() as f32 - 1.0) as u32;
    let uy = y.into_iter().max_by(f32_cmp).unwrap().min(frame_buffer.width() as f32 - 1.0) as u32;

    let mut a = Mat3::zero();
    a.set_x_axis(p[0].truncate().into());
    a.set_y_axis(p[1].truncate().into());
    a.set_z_axis(p[2].truncate().into());
    a = a.transpose();
    a.set_z_axis(Vec3::one());
    a = a.transpose();

    let ai = a.inverse();

    for i in lx..=ux {
        for j in ly..=uy {
            let pixel = vec3(i as f32 + 0.5, j as f32 + 0.5, 1.0);
            let b = ai * pixel;
            if b.min_element() >= 0.0 {
                let va = VertexAttributes::interpolate(v1, v2, v3, b.x(), b.y(), b.z());
                if va.position[2] >= -1.0 && va.position[2] <= 1.0 {
                    let frag = (program.fragment_shader)(va, uniform);
                    let h = frame_buffer.height() - 1;
                    *frame_buffer.get_pixel_mut(i, h - j) = (program.blending_shader)(frag, *frame_buffer.get_pixel_mut(i, h - j));
                }
            }
        }
    }
}

pub fn rasterize_triangles(program: &mut Program, uniform: UniformAttributes, vertices: Vec<VertexAttributes>, frame_buffer: &mut RgbaImage) {
    let mut v: Vec<VertexAttributes> = vec![];
    vertices.clone().into_iter().for_each(|vertex| v.push((program.vertex_shader)(vertex, uniform)));
    for i in 0..vertices.len() / 3 {
        rasterize_triangle(program, uniform, v[i * 3 + 0], v[i * 3 + 1], v[i * 3 + 2], frame_buffer);
    }
}

pub fn rasterize_line(program: &mut Program, uniform: UniformAttributes, v1: VertexAttributes, v2: VertexAttributes, line_thickness: f32, frame_buffer: &mut RgbaImage) {
    let mut p = vec![];
    p.push(v1.position / v1.position.w());
    p.push(v2.position / v2.position.w());

    p.iter_mut().for_each(|point| {
        point[0] = ((point[0] + 1.0) / 2.0) * frame_buffer.height() as f32;
        point[1] = ((point[1] + 1.0) / 2.0) * frame_buffer.width() as f32;
    });

    let f32_cmp = |a: &f32, b: &f32| a.partial_cmp(b).unwrap();
    let x: Vec<f32> = p.iter().map(|point| point[0]).collect();
    let y: Vec<f32> = p.iter().map(|point| point[1]).collect();

    let lx = (x.clone().into_iter().min_by(f32_cmp).unwrap() - line_thickness).max(0.0) as u32;
    let ly = (y.clone().into_iter().min_by(f32_cmp).unwrap() - line_thickness).max(0.0) as u32;
    let ux = (x.into_iter().max_by(f32_cmp).unwrap() + line_thickness).min(frame_buffer.height() as f32 - 1.0) as u32;
    let uy = (y.into_iter().max_by(f32_cmp).unwrap() + line_thickness).min(frame_buffer.width() as f32 - 1.0) as u32;

    let l1 = vec2(p[0][0], p[0][1]);
    let l2 = vec2(p[1][0], p[1][1]);

    let mut t = -1.0;
    let ll = (l1 - l2).length_squared();

    for i in lx..=ux {
        for j in ly..=uy {
            let pixel = vec2(i as f32 + 0.5, j as f32 + 0.5);
            if ll == 0.0 {
                t = 0.0;
            } else {
                t = ((pixel - l1).dot(l2 - l1) / ll).min(1.0).max(0.0);
            }
            
            let pixel_p = l1 + t * (l2 - l1);

            if (pixel - pixel_p).length_squared() < line_thickness * line_thickness {
                let va = VertexAttributes::interpolate(v1, v2, v1, 1.0 - t, t, 0.0);
                let frag = (program.fragment_shader)(va, uniform);
                let h = frame_buffer.height() - 1;
                *frame_buffer.get_pixel_mut(i, h - j) = (program.blending_shader)(frag, *frame_buffer.get_pixel_mut(i, h - j));
            }
        }
    }
}

pub fn rasterize_lines(program: &mut Program, uniform: UniformAttributes, vertices: Vec<VertexAttributes>, line_thickness: f32, frame_buffer: &mut RgbaImage) {
    let mut v: Vec<VertexAttributes> = vec![];
    vertices.clone().into_iter().for_each(|vertex| v.push((program.vertex_shader)(vertex, uniform)));
    for i in 0..vertices.len() / 2 {
        rasterize_line(program, uniform, v[i * 2 + 0], v[i * 2 + 1], line_thickness, frame_buffer);
    }
}