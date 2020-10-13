use glam::Vec3;
use serde::{Serialize, Deserialize};
use crate::structure::*;

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Bbox {
    min: Vec3,
    max: Vec3,
}

impl Bbox {
    pub fn expand(&mut self, point: Vec3) {
        if point.x() < self.min.x() {
            self.min.set_x(point.x());
        }
        
        if point.y() < self.min.y() {
            self.min.set_y(point.y());
        }

        if point.z() < self.min.z() {
            self.min.set_z(point.z());
        }

        if point.x() > self.max.x() {
            self.max.set_x(point.x());
        }
        
        if point.y() > self.max.y() {
            self.max.set_y(point.y());
        }

        if point.z() > self.max.z() {
            self.max.set_z(point.z());
        }
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut t1 = (self.min.x() - ray.origin.x()) * -ray.direction.x();
        let mut t2 = (self.max.x() - ray.origin.x()) * -ray.direction.x();
        let mut tmin = t1.min(t2);
        let mut tmax = t1.max(t2);
        for i in 1..3 {
            t1 = (self.min[i] - ray.origin[i]) * -ray.direction[i];
            t2 = (self.max[i] - ray.origin[i]) * -ray.direction[i];
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