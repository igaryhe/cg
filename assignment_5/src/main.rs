mod attributes;
mod raster;

use std::fs::*;
use std::f32::consts::*;
use image::{Rgba, Frame, Delay, gif::Encoder};
use glam::{vec3, Vec3, Mat4};
use attributes::*;
use raster::*;

fn main() {
    let mut frame_buffer = FrameBuffer::new(300, 300);
    let aspect_ratio = frame_buffer.width() as f32 / frame_buffer.height() as f32;
    let file = read_to_string("data/uniform.json").unwrap();
    let mut uniform: UniformAttributes = serde_json::from_str(file.as_str()).unwrap();
    if aspect_ratio < 1.0 {
        uniform.view.y_axis_mut()[1] = aspect_ratio;
    } else {
        uniform.view.x_axis_mut()[0] = 1.0 / aspect_ratio;
    }

    let vertex_shader = Box::new(
        |va: VertexAttributes, uniform: UniformAttributes| {
            let mut transform = Mat4::identity();
            let cos = (uniform.transform.angle * PI).cos();
            let sin = (uniform.transform.angle * PI).sin();
            transform.w_axis_mut()[2] = -uniform.transform.distance;
            transform.x_axis_mut()[0] = cos;
            transform.x_axis_mut()[2] = sin;
            transform.z_axis_mut()[0] = -sin;
            transform.z_axis_mut()[2] = cos;
            let normal_transform = transform.inverse().transpose();
            let transformed = transform * va.position;
            VertexAttributes {
                position: uniform.view * transformed,
                normal: normal_transform * va.normal,
            }});

    let fragment_shader = Box::new(
        |va: VertexAttributes, uniform: UniformAttributes| {
            let n: Vec3 = va.normal.truncate().into();
            let v = match uniform.camera.is_perspective {
                true => (uniform.camera.position - va.position.truncate().into()).normalize(),
                false => vec3(0.0, 0.0, -1.0),
            };
            let li: Vec3 = (uniform.light.position - va.position.truncate().into()).normalize();
            let diffuse = uniform.material.diffuse_color * li.dot(n).max(0.0);
            let specular = uniform.material.specular_color * n.dot((li + v).normalize()).max(0.0).powf(uniform.material.specular_exponent);
            let d = uniform.light.position - va.position.truncate().into();
            let color = (diffuse + specular) * uniform.light.intensity / d.length_squared();
            let out = FragmentAttributes::new(color.extend(1.0), va.position.truncate().into(), n);
            out
        });

    let blending_shader = Box::new(
        |fa: FragmentAttributes, previous: FrameBufferAttributes| {
            let alpha = fa.color[3];
            let out = fa.color * alpha + previous.get_color() * (1.0 - alpha);
            if fa.position.z() < previous.depth {
                FrameBufferAttributes {
                    color: Rgba(
                        [(out[0] * 255.0) as u8,
                         (out[1] * 255.0) as u8,
                         (out[2] * 255.0) as u8,
                         (out[3] * 255.0)as u8]),
                    depth: fa.position.z()
                }
            } else { previous }});

    let mut program = Program {
        vertex_shader,
        fragment_shader,
        blending_shader,
    };
    let render_type = RenderType::Png;
    let primitive_type = PrimitiveType::Triangle;
    let (vertices, indices) = load_off("data/bunny.off", primitive_type);
    match render_type {
        RenderType::Png => {
            rasterize(&mut program, uniform, &vertices, &indices, &mut frame_buffer, primitive_type);
            const FILENAME: &str = "img/bunny.png";
            frame_buffer.render().save(FILENAME).unwrap();
        },
        RenderType::Gif => {
            let mut frames: Vec<Frame> = vec![];
            let gif = File::create("img/anim.gif").unwrap();
            let mut encoder = Encoder::new(gif);
            for _ in 0..20 {
                uniform.transform.angle += 0.1;
                // uniform.transform.distance -= 0.05;
                frame_buffer.clear();
                rasterize(&mut program, uniform, &vertices, &indices, &mut frame_buffer, primitive_type);
                let frame = Frame::from_parts(frame_buffer.render(), 0, 0, Delay::from_saturating_duration(std::time::Duration::from_millis(75)));
                frames.push(frame);
            }
            encoder.encode_frames(frames).unwrap();
        },
    }
}

fn load_off(filename: &str, primitive_type: PrimitiveType) -> (Vec<VertexAttributes>, Vec<usize>) {
    use std::io::BufRead;
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut i = 0;
    let mut nv = 0;
    let mut vertices: Vec<VertexAttributes> = vec![];
    let mut indices: Vec<usize> = vec![];
    for line in reader.lines() {
        if i == 0 {
            i += 1;
            continue;
        }
        if i == 1 {
            let nums: Vec<u32> = line.unwrap().split(" ").flat_map(str::parse).collect();
            nv = nums[0];
            i += 1;
            continue;
        }
        if i > 1 && i <= 1 + nv {
            let nums: Vec<f32> = line.unwrap().split(" ").flat_map(str::parse).collect();
            vertices.push(VertexAttributes::new(vec3(nums[0], nums[1], nums[2]), Vec3::zero()));
            i += 1;
            continue;
        }
        if i > 1 + nv {
            let id: Vec<usize> = line.unwrap().split(" ").flat_map(str::parse).collect();
            let normal = (vertices[id[2]].position - vertices[id[1]].position).truncate()
                .cross((vertices[id[3]].position - vertices[id[1]].position).truncate()).normalize().extend(0.0);
            vertices[id[1]].normal += normal;
            vertices[id[2]].normal += normal;
            vertices[id[3]].normal += normal;
            match primitive_type.clone() {
                PrimitiveType::Triangle => {
                    indices.push(id[1] as usize);
                    indices.push(id[2] as usize);
                    indices.push(id[3] as usize);
                },
                PrimitiveType::Line => {
                    indices.push(id[1] as usize);
                    indices.push(id[2] as usize);
                    indices.push(id[2] as usize);
                    indices.push(id[3] as usize);
                    indices.push(id[3] as usize);
                    indices.push(id[1] as usize);
                },
            }
            i += 1;
            continue;
        }
    }
    vertices.iter_mut().for_each(|v| {
        v.normal = v.normal.normalize();
    });
    (vertices, indices)
}
