use crate::structure::*;
use glam::{Vec3, Mat3};
use serde::{Serialize, Deserialize};
use nalgebra::{Matrix3, Vector3};
use mint::ColumnMatrix3;

#[typetag::serde(tag = "type")]
pub trait Object: Sync {
    fn material(&self) -> Material;
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn set_position(&mut self, pos: Vec3);
}

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    material: Material,
    position: Vec3,
    radius: f32,
}

#[typetag::serde]
impl Object for Sphere {
    fn material(&self) -> Material {
        self.material
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // TODO:
        //
        // Compute the intersection between the ray and the sphere
        // If the ray hits the sphere, set the result of the intersection in the
        // struct 'hit'
        todo!()
    }

    fn set_position(&mut self, pos: Vec3) {
        self.position = pos
    }
}

#[derive(Serialize, Deserialize)]
pub struct Parallelogram {
    material: Material,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
}

#[typetag::serde]
impl Object for Parallelogram {
    fn material(&self) -> Material {
        self.material
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // TODO
        todo!()
    }

    fn set_position(&mut self, pos: Vec3) {
        self.origin = pos
    }
}
