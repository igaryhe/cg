
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
        // TODO (Assignment 3)
	//
	// Compute whether the ray intersects the given triangle.
	// If you have done the parallelogram case, this should be very similar to it.
        todo!()
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
