use crate::structure::*;
use glam::Vec3;
use serde::{Serialize, Deserialize};

#[typetag::serde(tag = "type")]
pub trait Object {
    fn material(&self) -> Material;
    fn intersect(&self, ray: &Ray, hit: &Intersection) -> bool;
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

    fn intersect(&self, ray: &Ray, hit: &Intersection) -> bool {
        // TODO:
        //
        // Compute the intersection between the ray and the sphere
        // If the ray hits the sphere, set the result of the intersection in the
        // struct 'hit'
        false
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

    fn intersect(&self, ray: &Ray, hit: &Intersection) -> bool {
        // TODO
        false
    }
}