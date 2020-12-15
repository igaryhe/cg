use glam::Vec3;
use serde::{Serialize, Deserialize};
use crate::structure::*;
use crate::triangle::Triangle;

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Bbox {
    min: Vec3,
    max: Vec3,
}

impl Bbox {
    pub fn expand(&mut self, triangle: &Triangle) {
        self.min = self.min.min(triangle.a.min(triangle.b.min(triangle.c)));
        self.max = self.max.max(triangle.a.max(triangle.b.max(triangle.c)));
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        // TODO (Assignment 3)
	//
	// Compute whether the ray intersects the given box.
	// There is no need to set the resulting normal and ray parameter, since
	// we are not testing with the real surface here anyway.
        todo!()
    }

    pub fn centroid(&self) -> Vec3 {
        (self.min + self.max) / 2.0
    }

    pub fn merge(&self, another_box: &Self) -> Self {
        Self {
            min: self.min.min(another_box.min),
            max: self.max.max(another_box.max)
        }
    }
}
