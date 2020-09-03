mod viewer;
mod attributes;
mod raster;

use viewer::*;
use attributes::*;
use raster::*;
use image::{RgbaImage, Rgba};
use glam::vec4;

struct ViewerData {
    vertices: Vec<VertexAttributes>,
    uniform: UniformAttributes,
    program: Program,
    w: f32,
    h: f32,
}

impl ViewerStrategy for ViewerData {
    fn new() -> Self {
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
        Self {
            vertices: vec![],
            program,
            uniform,
            w: 0.0,
            h: 0.0,
        }
    }

    fn mouse_move(&self, x: f32, y: f32) {}

    fn mouse_pressed(&mut self, x: f32, y: f32, button: minifb::MouseButton) {
        self.vertices[2].position = vec4((x / self.w * 2.0) - 1.0,
                                         ((self.h - 1.0 - y) / self.h * 2.0) - 1.0,
                                         0.0,
                                         1.0);
    }

    fn mouse_wheel(dx: u32, dy: u32, is_direction_normal: bool) {}

    fn key_pressed(&self, key: minifb::Key) {}

    fn redraw(&mut self, buffer: &mut RgbaImage) {
        buffer.pixels_mut().into_iter().for_each(|pixel|
                                                 *pixel = Rgba([0, 0, 0, 0]));
        rasterize_triangles(&mut self.program,
                            self.uniform,
                            self.vertices.clone(),
                            buffer);
    }
}

fn main() {
    let mut vertices: Vec<VertexAttributes> = vec![];
    vertices.push(VertexAttributes::new(-1.0, -1.0, 0.0));
    vertices.push(VertexAttributes::new(1.0, -1.0, 0.0));
    vertices.push(VertexAttributes::new(0.0, 1.0, 0.0));

    vertices[0].color = vec4(1.0, 0.0, 0.0, 1.0);
    vertices[1].color = vec4(0.0, 1.0, 0.0, 1.0);
    vertices[2].color = vec4(0.0, 0.0, 1.0, 1.0);

    let width = 500;
    let height = 500;

    let mut viewer = Viewer::<ViewerData>::new("RasterViewer", width, height);
    viewer.s.vertices = vertices;
    viewer.s.w = width as f32;
    viewer.s.h = height as f32;
    
    viewer.launch(std::time::Duration::from_millis(16));
}
