use glam::{vec4, Vec3, Vec4, Mat4};
use image::{Rgba, RgbaImage};
use serde::Deserialize;

#[derive(Copy, Clone, Default)]
pub struct VertexAttributes {
    pub position: Vec4,
    pub normal: Vec4,
}

impl VertexAttributes {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self {
            position: position.extend(1.0),
            normal: normal.extend(0.0),
        }
    }

    pub fn interpolate(a: VertexAttributes, b: VertexAttributes,
                       c: VertexAttributes, alpha: f32, beta: f32, gamma: f32) -> Self {
        let mut r = VertexAttributes::default();
        r.position = alpha * a.position + beta * b.position + gamma * c.position;
        r.normal = alpha * a.normal + beta * b.normal + gamma * c.normal;
        r
        }
}

#[derive(Copy, Clone, Default)]
pub struct FragmentAttributes {
    pub color: Vec4,
    pub position: Vec3,
    pub normal: Vec3,
}

impl FragmentAttributes {
    pub fn new(color: Vec4, position: Vec3, normal: Vec3) -> Self {
        Self {
            color,
            position,
            normal,
        }
    }
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct UniformAttributes {
    pub view: Mat4,
    pub light: Light,
    pub material: Material,
    pub camera: Camera,
    pub transform: Transform,
}

#[derive(Copy, Clone)]
pub struct FrameBufferAttributes {
    pub color: Rgba<u8>,
    pub depth: f32,
}

impl FrameBufferAttributes {
    pub fn new() -> Self {
        FrameBufferAttributes {
            color: Rgba([255, 255, 255, 255]),
            depth: 100.0,
        }
    }

    pub fn get_color(&self) -> Vec4 {
        vec4(self.color[0] as f32 / 255.0,
             self.color[1] as f32 / 255.0,
             self.color[2] as f32 / 255.0,
             self.color[3] as f32 / 255.0)
    }
}

pub struct FrameBuffer {
    frame_buffer: Vec<Vec<FrameBufferAttributes>>,
}

impl FrameBuffer {
    pub fn new(w: usize, h: usize) -> Self {
        FrameBuffer { frame_buffer: vec![vec![FrameBufferAttributes::new(); w]; h] }
    }

    pub fn height(&self) -> usize {
        self.frame_buffer.len()
    }

    pub fn width(&self) -> usize {
        self.frame_buffer[0].len()
    }

    pub fn get(&self, x: usize, y: usize) -> FrameBufferAttributes {
        self.frame_buffer[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: FrameBufferAttributes) {
        self.frame_buffer[y][x] = val;
    }

    pub fn clear(&mut self) {
        self.frame_buffer = vec![vec![FrameBufferAttributes::new(); self.width()]; self.height()];
    }

    pub fn render(&self) -> RgbaImage {
        let mut img = RgbaImage::new(self.width() as u32, self.height() as u32);
        img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            *pixel = self.get(x as usize, y as usize).color
        });
        img
    }
}

#[derive(Copy, Clone)]
pub enum PrimitiveType {
    Triangle,
    Line,
}

#[derive(Copy, Clone)]
pub enum RenderType {
    Png,
    Gif,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Light {
    pub position: Vec3,
    pub intensity: Vec3,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Material {
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
    pub specular_exponent: f32,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Camera {
    pub is_perspective: bool,
    pub position: Vec3,
    pub field_of_view: f32,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Transform {
    pub angle: f32,
    pub distance: f32,
}
