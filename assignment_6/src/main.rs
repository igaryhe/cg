mod viewer;
mod attributes;
mod raster;

use viewer::*;
use attributes::*;
use raster::*;
use image::{RgbaImage, Rgba};
use glam::vec4;

fn main() {
    let width: usize = 500;
    let height: usize = 500;

    let uniform = UniformAttributes::default();

    let vertex_shader = Box::new(
        |va: VertexAttributes, uniform: UniformAttributes|  va );

    let fragment_shader = Box::new(
        |va: VertexAttributes, uniform|
        FragmentAttributes::new(va.color[0], va.color[1], va.color[2]));

    let blending_shader = Box::new(
        |fa: FragmentAttributes, previous: Rgba<u8>| 
        Rgba(
            [(fa.color[0] * 255.0) as u8,
            (fa.color[1] * 255.0) as u8,
            (fa.color[2] * 255.0) as u8,
            (fa.color[3] * 255.0)as u8]));

    let mut program = Program {
        vertex_shader,
        fragment_shader,
        blending_shader,
    };

    let mut vertices: Vec<VertexAttributes> = vec![];
    vertices.push(VertexAttributes::new(-1.0, -1.0, 0.0));
    vertices.push(VertexAttributes::new(1.0, -1.0, 0.0));
    vertices.push(VertexAttributes::new(0.0, 1.0, 0.0));

    vertices[0].color = vec4(1.0, 0.0, 0.0, 1.0);
    vertices[1].color = vec4(0.0, 1.0, 0.0, 1.0);
    vertices[2].color = vec4(0.0, 0.0, 1.0, 1.0);

    let mut viewer = Viewer::init("test", 400, 400);
    let mouse_pressed= Box::new(|x: f32, y: f32, button: minifb::MouseButton| {
        vertices[2].position = vec4((x / y * 2.0) - 1.0, ((height as f32 - 1.0 - y) / height as f32 * 2.0) - 1.0, 0.0, 1.0);
        viewer.redraw_next = true;
    });
    let redraw = Box::new(|buffer: &mut RgbaImage| {
        rasterize_triangles(&mut program, uniform, vertices.clone(), buffer);
    });
    viewer.mouse_pressed = mouse_pressed;
    viewer.redraw = redraw;
    viewer.launch(std::time::Duration::from_millis(16));
}