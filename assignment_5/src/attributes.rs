use glam::{vec4, Vec3, Vec4, Mat4};
use image::{Rgba, RgbaImage};
use serde::Deserialize;
use std::f32::consts::*;

#[derive(Copy, Clone, Default)]
pub struct VertexAttributes {
    pub position: Vec4,
    pub normal: Vec3,
    pub frag_pos: Vec3,
}

impl VertexAttributes {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self {
            position: position.extend(1.0),
            normal,
            frag_pos: Vec3::zero(),
        }
    }

    pub fn interpolate(a: VertexAttributes, b: VertexAttributes,
                       c: VertexAttributes, alpha: f32, beta: f32, gamma: f32) -> Self {
        let mut r = VertexAttributes::default();
        r.position = alpha * (a.position / a.position.w)
            + beta * (b.position / b.position.w)
            + gamma * (c.position / c.position.w);
        r.normal = (alpha * a.normal + beta * b.normal + gamma * c.normal).normalize();
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
    pub light: Light,
    pub material: Material,
    pub camera: Camera,
    pub transform: Transform,
    pub view_matrix: Mat4,
    pub model_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub normal_matrix: Mat4,
}

impl UniformAttributes {
    pub fn calc_model_matrix(&mut self) {
        let mut transform = Mat4::identity();
        let cos = (self.transform.angle * PI).cos();
        let sin = (self.transform.angle * PI).sin();
        transform.w_axis.z = -self.transform.distance;
        transform.x_axis.x = cos;
        transform.x_axis.z = sin;
        transform.z_axis.x = -sin;
        transform.z_axis.z = cos;
        self.model_matrix = transform;
        self.normal_matrix = transform.inverse().transpose();
    }

    pub fn calc_view_matrix(&mut self) {
        let mut transform = Mat4::identity();
        transform.w_axis.x = -self.camera.position.x;
        transform.w_axis.y = -self.camera.position.y;
        transform.w_axis.z = -self.camera.position.z;
        self.view_matrix = transform;
    }

    pub fn calc_projection_matrix(&mut self) {
        self.projection_matrix = match self.camera.is_perspective {
            true => Mat4::perspective_rh(self.camera.field_of_view, self.camera.aspect_ratio, -1.0, 1.0),
            false => Mat4::identity(),// Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0),
        };
    }

    pub fn calc_matrices(&mut self) {
        self.calc_model_matrix();
        self.calc_view_matrix();
        self.calc_projection_matrix();
    }
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
    pub aspect_ratio: f32,
}

#[derive(Copy, Clone, Default, Deserialize)]
pub struct Transform {
    pub angle: f32,
    pub distance: f32,
}
