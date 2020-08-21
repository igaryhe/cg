use crate::attributes::*;
use glam::{vec3, Vec3, Mat3};
use image::{RgbaImage, Rgba};

pub struct Program {
    pub vertex_shader: Box<dyn FnMut(VertexAttributes, UniformAttributes) -> VertexAttributes>,
    pub fragment_shader: Box<dyn FnMut(VertexAttributes, UniformAttributes) -> FragmentAttributes>,
    pub blending_shader: Box<dyn FnMut(FragmentAttributes, Rgba<u8>) -> Rgba<u8>>,
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

    for i in lx..ux + 1 {
        for j in ly..uy + 1 {
            let pixel = vec3(i as f32 + 0.5, j as f32 + 0.5, 1.0);
            let b = ai * pixel;
            if b.min_element() >= 0.0 {
                let va = VertexAttributes::interpolate(v1, v2, v3, b[0], b[1], b[2]);
                if va.position[2] >= -1.0 && va.position[2] <= 1.0 {
                    let frag = (program.fragment_shader)(va, uniform);
                    *frame_buffer.get_pixel_mut(i, j) = (program.blending_shader)(frag, *frame_buffer.get_pixel_mut(i, j));
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