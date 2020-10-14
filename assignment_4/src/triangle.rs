
use glam::{Vec3, Mat3};
use serde::{Serialize, Deserialize};
use crate::structure::*;
use crate::bbox::Bbox;
use nalgebra::{Matrix3, Vector3};
use mint::ColumnMatrix3;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
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
        let x = decomp.solve(&b);
        match x {
            Some(xr) => {
                match xr[0] >= 0.0 && xr[1] >= 0.0 && xr[0] + xr[1] <= 1.0 && xr[2] >= 0.0 {
                    true => {
                        let position = ray.origin + xr[2] * ray.direction;
                        let normal = v.cross(u).normalize();
                        Some(Intersection {
                            position,
                            normal,
                            ray_param: 0.0
                        })
                    },
                    false => None,
                }
            },
            None => None,
        }
    }

    pub fn centroid(&self) -> Vec3 {
        (self.a + self.b + self.c) / 3.0
    }

    pub fn bbox(&self) -> Bbox {
        let mut bbox = Bbox::default();
        bbox.expand(self);
        bbox
    }
}
