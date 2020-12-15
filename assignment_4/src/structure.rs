use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Self {
            origin: o,
            direction: d
        }
    }
}

#[derive(Default, Deserialize)]
pub struct Light {
    pub position: Vec3,
    pub intensity: Vec3,
}

#[derive(Default)]
pub struct Intersection {
    pub position: Vec3,
    pub normal: Vec3,
    pub ray_param: f32,
}

#[derive(Default, Deserialize)]
pub struct Camera {
    pub is_perspective: bool,
    pub position: Vec3,
    pub field_of_view: f32, // between 0 and PI
    pub focal_length: f32,
    pub lens_radius: f32, // for depth of field
}

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Material {
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
    pub specular_exponent: f32, // Also called "shininess"

    pub reflection_color: Vec3,
    pub refraction_color: Vec3,
    pub refraction_index: f32,
}
