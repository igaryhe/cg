use crate::structure::*;
use glam::{vec3, Vec3, Mat3};
use serde::{Serialize, Deserialize};
use crate::bbox::Bbox;
use nalgebra::{Matrix3, Vector3};
use mint::ColumnMatrix3;
use crate::triangle::Triangle;

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
}


#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Node {
    bbox: Bbox,
    parent: usize,
    left: usize,
    right: usize,
    triangle: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AABBTree {
    nodes: Vec<Node>,
    root: usize,
}

impl AABBTree {
    pub fn new(triangles: &Vec<Triangle>) -> Self {
        // TODO (Assignment 3)

	    // Method (1): Top-down approach.
	    // Split each set of primitives into 2 sets of roughly equal size,
	    // based on sorting the centroids along one direction or another.

	    // Method (2): Bottom-up approach.
        // Merge nodes 2 by 2, starting from the leaves of the forest, until only 1 tree is left.
        let mut centroids = vec![];
        let mut nodes: Vec<Node> = vec![];
        triangles.iter().enumerate().for_each(|(i, triangle)| {
            centroids.push(triangle.centroid());
            nodes.push(Node {
                bbox: triangle.bbox(),
                parent: usize::MAX,
                left: usize::MAX,
                right: usize::MAX,
                triangle: i,
            });
        });
        
        for i in 0..centroids.len() {
            if nodes[i].parent == usize::MAX {
                let mut cloest = i + 1;
                for j in i + 1..centroids.len() {
                    if centroids[i] - centroids[j] < centroids[i] - centroids[cloest] {
                        cloest = j;
                    }
                    let node = Node {
                        bbox: nodes[i].bbox.merge(&nodes[cloest].bbox),
                        parent: usize::MAX,
                        left: i,
                        right: cloest,
                        triangle: usize::MAX,
                    };
                    centroids.push(node.bbox.centroid());
                    nodes.push(node);
                    nodes[i].parent = centroids.len() - 1;
                    nodes[cloest].parent = centroids.len() - 1;
                }
            }
        }
        let root = centroids.len() - 1;
        Self {
            nodes,
            root,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    material: Material,
    triangles: Vec<Triangle>,
    bvh: AABBTree,
}

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
        self.bvh.nodes[self.bvh.root].search(ray, &self.bvh.nodes, &self.triangles)
    }
}

impl Mesh {
    pub fn new(filename: &str, material: Material) -> Self {
        let triangles = load_off(filename);
        let bvh = AABBTree::new(&triangles);
        Self {
            material,
            triangles,
            bvh,
        }
    }
}

impl Node {
    fn search(&self, ray: &Ray, nodes: &Vec<Node>, triangles: &Vec<Triangle>) -> Option<Intersection> {
        if self.triangle != usize::MAX {
            triangles[self.triangle].intersect(ray)
        } else {
            if self.bbox.intersect(ray) {
                let left = nodes[self.left].search(ray, nodes, triangles);
                let right = nodes[self.right].search(ray, nodes, triangles);
                match left {
                    Some(lhit) => {
                        match right {
                            Some(rhit) => {
                                if (lhit.position - ray.origin).length() < (rhit.position - ray.origin).length() {
                                    Some(lhit)
                                } else { Some(rhit) }
                            },
                            None => Some(lhit),
                        }
                    },
                    None => {
                        match right {
                            Some(rhit) => Some(rhit),
                            None => None,
                        }
                    },
                }
            } else { None }
        }
    }
}