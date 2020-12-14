pub mod structure;
pub mod file;

pub use structure::*;
pub use file::*;
use anyhow::Result;

#[inline]
fn salient_angle(a: &Point, b: &Point, c: &Point) -> i8 {
    // TODO
    1
}

pub fn convex_hull(points: &mut Polygon) -> Result<Polygon> {
    // TODO
    let p0 = point(0.0, 0.0);
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut hull: Vec<Point> = vec![];
    // TODO
    // use salient_angle(a, b, c) here
    Ok(hull)
}

// TODO
// Check if point b is on segment ac
#[inline]
fn on_segment(a: &Point, b: &Point, c: &Point) -> bool {
    todo!()
}

// Return true if [a,b] intersects [c,d]
fn intersect_segment(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
    todo!()
}

pub fn is_inside(poly: &Vec<Point>, query: Point) -> bool {
    // 1. Compute bounding box and set coordinate of a point outside the polygon
    // TODO
    let mut outside = point(0.0, 0.0);
    // 2. Cast a ray from the query point to the 'outside' point, count number of intersections
    // TODO
    todo!()
}
