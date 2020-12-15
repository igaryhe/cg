mod attributes;
mod raster;

use std::fs::*;
use image::{Rgba, Frame, Delay, gif::Encoder};
use glam::{vec3, Vec3, vec4};
use attributes::*;
use raster::*;
use anyhow::Result;

fn main() -> Result <()> {
    // Read uniform data from external json file
    // let file = read_to_string("data/uniform.json")?;
    // Render type
    let render_type = RenderType::Png;
    // Primitive type
    let primitive_type = PrimitiveType::Line;
    // Global Constants (empty in this example)
    let mut uniform = UniformAttributes{};
    
    // The Framebuffer storing the image rendered by the rasterizer
    let mut frame_buffer = FrameBuffer::new(500, 500);

    // The vertex shader is the identity
    let vertex_shader = Box::new(
        |va: VertexAttributes, uniform: UniformAttributes| va);

    // The fragment shader uses a fixed color
    let fragment_shader = Box::new(
        |va: VertexAttributes, uniform: UniformAttributes| {
            let out = FragmentAttributes::new(vec4(1.0, 0.0, 0.0, 1.0));
            out
        });

    // The blending shader converts colors between 0 and 1 to u8
    let blending_shader = Box::new(
        |fa: FragmentAttributes, previous: FrameBufferAttributes| {
            let alpha = fa.color[3];
            let out = fa.color * alpha + previous.get_color() * (1.0 - alpha);
                FrameBufferAttributes {
                    color: Rgba(
                        [(out[0] * 255.0) as u8,
                         (out[1] * 255.0) as u8,
                         (out[2] * 255.0) as u8,
                         (out[3] * 255.0)as u8]),
                }});

    // Basic rasterization program
    let mut program = Program {
        vertex_shader,
        fragment_shader,
        blending_shader,
    };

    // One triangle in the center of the screen
    let mut vertices = vec![];
    vertices.push(VertexAttributes::new(vec3(1.0, -1.0, 0.0)));
    vertices.push(VertexAttributes::new(vec3(1.0, -1.0, 0.0)));
    vertices.push(VertexAttributes::new(vec3(0.0, 1.0, 0.0)));
    let indices = vec![0, 1, 2];
    
    match render_type {
        RenderType::Png => {
            rasterize(&mut program, uniform, &vertices, &indices, &mut frame_buffer, primitive_type)?;
            const FILENAME: &str = "img/bunny.png";
            frame_buffer.render().save(FILENAME)?;
        },
        RenderType::Gif => {
            let mut frames: Vec<Frame> = vec![];
            let gif = File::create("img/anim.gif")?;
            let mut encoder = Encoder::new(gif);
            for _ in 0..24 {
                frame_buffer.clear();
                // TODO
                // render a frame
                let frame = Frame::from_parts(frame_buffer.render(), 0, 0,
                                              Delay::from_saturating_duration(std::time::Duration::from_millis(75)));
                frames.push(frame);
            }
            encoder.encode_frames(frames)?;
        },
    }
    Ok(())
}

fn load_off(filename: &str, primitive_type: PrimitiveType) -> Result<(Vec<VertexAttributes>, Vec<usize>)> {
    use std::io::BufRead;
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let mut vertices: Vec<VertexAttributes> = vec![];
    let mut indices: Vec<usize> = vec![];
    // TODO
    // load mesh from off file to vertices and indices
    Ok((vertices, indices))
}
