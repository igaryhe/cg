mod attributes;
mod raster;

use image::{RgbaImage, Rgba};
use attributes::*;
use raster::*;

fn main() {
    let mut img = RgbaImage::new(500, 500);
    let uniform = UniformAttributes::default();

    let vertex_shader = Box::new(
        |va: VertexAttributes, uniform: UniformAttributes|  va );

    let fragment_shader = Box::new(
        |va: VertexAttributes, uniform|
        FragmentAttributes::new(1.0, 0.0,0.0));

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

    rasterize_triangles(&mut program, uniform, vertices, &mut img);
    const FILENAME: &str = "img/triangle.png";
    img.save(FILENAME).unwrap();
}