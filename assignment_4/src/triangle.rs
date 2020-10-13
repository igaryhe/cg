
use glam::{Vec3, Mat3};
use serde::{Serialize, Deserialize};
use crate::structure::*;
use crate::bbox::Bbox;
use nalgebra::{Matrix3, Vector3};
use mint::ColumnMatrix3;

#[derive(Serialize, Deserialize)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self {
            a,
            b,
            c,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let u = self.b - self.a;
        let v = self.c - self.a;
        let origin = self.a;
        let a = Mat3::from_cols(u, v, -ray.direction);
        let a = Matrix3::from(ColumnMatrix3::from(a));
        let decomp = a.lu();
        let b = ray.origin - origin;
        let b = Vector3::from(mint::Vector3::from(b));
        let x = decomp.solve(&b).unwrap();
        match x[0] >= 0.0 && x[1] >= 0.0 && x[0] + x[1] <= 1.0 && x[2] >= 0.0 {
            true => {
                println!("Triangle!");
                let position = ray.origin + x[2] * ray.direction;
                let normal = v.cross(u).normalize();
                Some(Intersection {
                    position,
                    normal,
                    ray_param: 0.0
                })
            },
            false => None
        }
    }

    pub fn centroid(&self) -> Vec3 {
        (self.a + self.b + self.c) / 3.0
    }

    pub fn bbox(&self) -> Bbox {
        let mut bbox = Bbox::default();
        bbox.expand(self.a);
        bbox.expand(self.b);
        bbox.expand(self.c);
        bbox
    }
}