use crate::structure::*;
use glam::{vec3, Vec3, Mat3};
use serde::{Serialize, Deserialize};
use crate::bbox::Bbox;
use nalgebra::{Matrix3, Vector3};
use mint::ColumnMatrix3;
use crate::triangle::Triangle;
use indicatif::ProgressBar;
use rayon::prelude::*;

#[typetag::serde(tag = "type")]
pub trait Object: Sync {
    fn material(&self) -> Material;
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
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
        // TODO (Assignment 2)
        todo!()
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
        // TODO (Assignment 2)
        todo!()
    }
}


#[derive(Serialize, Deserialize)]
pub struct Node {
    bbox: Bbox,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    triangle: Option<Triangle>,
}

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    material: Material,
    bvh: Node,
}

// Read a triangle mesh from an off file
pub fn load_off(filename: &str) -> Vec<Triangle> {
    use std::io::BufRead;
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut i = 0;
    let mut nv = 0;
    let mut vertices: Vec<Vec3> = vec![];
    let mut triangles: Vec<Triangle> = vec![];
    for line in reader.lines() {
        if i == 0 {
            i += 1;
            continue;
        }
        if i == 1 {
            let nums: Vec<u32> = line.unwrap().split(" ").flat_map(str::parse).collect();
            nv = nums[0];
            i += 1;
            continue;
        }
        if i > 1 && i <= 1 + nv {
            let nums: Vec<f32> = line.unwrap().split(" ").flat_map(str::parse).collect();
            vertices.push(vec3(nums[0], nums[1], nums[2]));
            i += 1;
            continue;
        }
        if i > 1 + nv {
            let nums: Vec<usize> = line.unwrap().split(" ").flat_map(str::parse).collect();
            triangles.push(Triangle::new(vertices[nums[1]], vertices[nums[2]], vertices[nums[3]]));
            i += 1;
            continue;
        }
    }
    triangles
}

#[typetag::serde]
impl Object for Mesh {
    fn material(&self) -> Material {
        self.material
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.bvh.search(ray)
    }
}

impl Mesh {
    pub fn new(filename: &str, material: Material) -> Self {
        let triangles = load_off(filename);
        let bvh = Node::tree(&triangles);
        Self {
            material,
            bvh,
        }
    }
}

impl Node {
    fn search(&self, ray: &Ray) -> Option<Intersection> {
        // TODO (Assignment 3)

	// Method (1): Traverse every triangle and return the closest hit.

	// Method (2): Traverse the BVH tree and test the intersection with a
	// triangles at the leaf nodes that intersects the input ray.
        todo!()
    }

    fn cost(&self, node: &Node) -> f32 {
        (self.bbox.centroid() - node.bbox.centroid()).length()
    }

    pub fn tree(triangles: &Vec<Triangle>) -> Self {
        // TODO (Assignment 3)

        // Method (1): Top-down approach.
        // Split each set of primitives into 2 sets of roughly equal size,
        // based on sorting the centroids along one direction or another.

        // Method (2): Bottom-up approach.
        // Merge nodes 2 by 2, starting from the leaves of the forest, until only 1 tree is left.
        let mut nodes: Vec<Node> = vec![];
        nodes.remove(0)
    }
}

pub fn cost(centroid: Vec3, node: &Node) -> f32 {
    (centroid - node.bbox.centroid()).length()
}
