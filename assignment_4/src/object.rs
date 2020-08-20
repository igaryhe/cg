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
        // TODO (Assignment 2)
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
        // TODO (Assignment 2)
        false
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Bbox {
    min: Vec3,
    max: Vec3,
}

impl Bbox {
    pub fn expand(&mut self, point: Vec3) {

    }
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    bbox: Bbox,
    parent: u32,
    left: u32,
    right: u32,
    triangle: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AABBTree {
    nodes: Vec<Node>,
    root: u32,
}

impl AABBTree {
    pub fn new() -> Self {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    material: Material,
    vertices: Vec3,
    facets: Vec3,
    bvh: AABBTree,
}

impl Mesh {
    pub fn new(filename: &str) -> Self {
        unimplemented!()
    }
}

#[typetag::serde]
impl Object for Mesh {
    fn material(&self) -> Material {
        self.material
    }

    fn intersect(&self, ray: &Ray, hit: &Intersection) -> bool {
        // TODO (Assignment 2)
        false
    }
}