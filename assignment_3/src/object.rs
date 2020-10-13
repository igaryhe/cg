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
        let distance = ray.origin - self.position;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * distance.dot(ray.direction);
        let c = distance.dot(distance) - self.radius.powi(2);
        let delta = b.powi(2) - 4.0 * a * c;
        if delta < 0.0 {
            None
        } else if delta == 0.0 {
            let t = -b / (2.0 * a);
            if t >= 0.0 {
                let position = ray.origin + t * ray.direction;
                let normal = (position - self.position).normalize();
                Some(Intersection {
                    position,
                    normal,
                    ray_param: 0.0
                })
            } else {
                None
            }
        } else {
            let t1 = (-b + delta.sqrt()) / (2.0 * a);
            let t2 = (-b - delta.sqrt()) / (2.0 * a);
            if t1 < 0.0 && t2 < 0.0 {
                None
            } else {
                let t = t1.min(t2);
                let position = ray.origin + t * ray.direction;
                let normal = (position - self.position).normalize();
                Some(Intersection {
                    position,
                    normal,
                    ray_param: 0.0
                })
            }
        }
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
        let a = Mat3::from_cols(self.u, self.v, -ray.direction);
        let a = Matrix3::from(ColumnMatrix3::from(a));
        let decomp = a.lu();
        let b = ray.origin - self.origin;
        let b = Vector3::from(mint::Vector3::from(b));
        let x = decomp.solve(&b).unwrap();
        match x[0] >= 0.0 && x[0] <= 1.0 && x[1] >= 0.0 && x[1] <= 1.0 && x[2] >= 0.0 {
            true => {
                let position = ray.origin + x[2] * ray.direction;
                let normal = self.v.cross(self.u).normalize();
                Some(Intersection {
                    position,
                    normal,
                    ray_param: 0.0
                })
            },
            false => None
        }
    }

    fn set_position(&mut self, pos: Vec3) {
        self.origin = pos
    }
}