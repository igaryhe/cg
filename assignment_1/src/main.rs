use glam::Vec2;
use std::fs;
use std::io::prelude::*;
use std::env;

fn det(u: &Vec2, v: &Vec2) -> f32 {
    // TODO
    0.0
}
fn salientAngle(a: &Vec2, b: &Vec2, c: &Vec2) -> bool {
    // TODO
    false
}

fn convex_hull(points: &mut Vec<Vec2>) -> Vec<Vec2> {
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let hull: Vec<Vec2> = vec![];
    hull
}

fn load_xyz(filename: &str) -> Vec<Vec2> {
    let points: Vec<Vec2> = vec![];
    let file = fs::read_to_string(filename).unwrap();
    // TODO
    points
}

fn save_xyz(filename: &str, points: Vec<Vec2>) {
    // TODO
}

fn load_obj(filename: &str) -> Vec<Vec2> {
    let file = fs::read_to_string(filename).unwrap();
    // TODO
    vec![]
}

fn save_obj(filename: &str, poly: Vec<Vec2>) {
    let mut file = fs::OpenOptions::new().append(true).open(filename).unwrap();
    poly.iter().for_each(|point| {
        writeln!(file, "v {} {} 0", point.x(), point.y()).unwrap();
    });
    for i in 0..poly.len() {
        writeln!(file, "l {} {}", i + 1, 1 + (i + 1) % poly.len()).unwrap();
    }
    writeln!(file).unwrap();
}

fn intersect_segment(a: Vec2, b: Vec2, c: Vec2, d: Vec2, ans: Vec2) -> bool {
    // TODO
    true
}

fn is_inside(poly: &Vec<Vec2>, query: Vec2) -> bool {
    // 1. Compute bounding box and set coordinate of a point outside the polygon
    // TODO
    let outside = Vec2::zero();
    // 2. Cast a ray from the query point to the 'outside' point, count number of intersections
    // TODO
    true
}


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => {
            eprintln!("Usage: {} points.xyz output.obj", args[0]);
        },
        _ => {
            let mut points = load_xyz(args[1].as_str());
            let hull = convex_hull(&mut points);
            save_obj(args[2].as_str(), hull);
        }
    }

    match args.len() {
        0 | 1 | 2 | 3 => {
            eprintln!("Usage: {} points.xyz poly.obj result.xyz", args[0]);   
        },
        _ => {
            let points = load_xyz(args[1].as_str());
            let poly = load_obj(args[2].as_str());
            let mut result: Vec<Vec2> = vec![];
            points.into_iter().for_each(|point| {
                if is_inside(&poly, point) {
                    result.push(point);
                }
            });
            save_obj(args[3].as_str(), result);
        }
    }
}