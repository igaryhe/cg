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
        let mut t1 = (self.min.x() - ray.origin.x()) * (1.0 / ray.direction.x());
        let mut t2 = (self.max.x() - ray.origin.x()) * (1.0 / ray.direction.x());
        let mut tmin = t1.min(t2);
        let mut tmax = t1.max(t2);
        for i in 1..3 {
            t1 = (self.min[i] - ray.origin[i]) * (1.0 / ray.direction[i]);
            t2 = (self.max[i] - ray.origin[i]) * (1.0 / -ray.direction[i]);
            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }
        tmax > tmin.max(0.0)
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
