pub mod structure;
pub mod file;

pub use structure::*;
pub use file::*;
use anyhow::{Result, Context};

#[inline]
fn det(u: &Point, v: &Point) -> f32 {
    let vec = *v - *u;
    vec * vec2(1.0, 0.0)
}

#[inline]
fn salient_angle(a: &Point, b: &Point, c: &Point) -> i8 {
    match (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) {
        x if x < 0.0 => { -1 },
        x if x == 0.0 => { 0 },
        _ => { 1 }
    }
}

pub fn convex_hull(points: &mut Polygon) -> Result<Polygon> {
    let p0 = *points.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).context("")?;
    points.sort_by(|a, b| det(&p0, b).partial_cmp(&det(&p0, a)).unwrap());
    let mut hull: Vec<Point> = vec![];
    hull.push(p0);
    let pos = points.iter().position(|&p| p == p0).context("")?;
    points.remove(pos);
    hull.push(points[0]);
    hull.push(points[1]);
    points.remove(0);
    points.remove(0);
    for point in points {
        while salient_angle(&hull[hull.len() - 2], hull.last().context("")?, point) != 1 {
            hull.pop();
        }
        hull.push(*point);
    }
    Ok(hull)
}

#[inline]
fn on_segment(a: &Point, b: &Point, c: &Point) -> bool {
    if b.x <= a.x.max(c.x) && b.x >= a.x.max(c.x) && b.y <= a.y.max(c.y) && b.y >= a.y.min(c.y) {
        true
    } else { false }
}

fn intersect_segment(a: &Point, b: &Point, c: &Point, d: &Point) -> bool {
    // TODO
    let o1 = salient_angle(a, b, c);
    let o2 = salient_angle(a, b, d);
    let o3 = salient_angle(c, d, a);
    let o4 = salient_angle(c, d, b);
    if o1 != o2 && o3 != o4 { return true }
    if o1 == 0 && on_segment(a, c, b) { return true }
    if o2 == 0 && on_segment(a, c, d) { return true }
    if o3 == 0 && on_segment(c, a, d) { return true }
    if o4 == 0 && on_segment(c, b, d) { return true }
    false
}

pub fn is_inside(poly: &Vec<Point>, query: Point) -> bool {
    // 1. Compute bounding box and set coordinate of a point outside the polygon
    // TODO
    let mut outside = point(0.0, 0.0);
    poly.iter().for_each(|&p| {
        if p.x > outside.x {
            outside.x = p.x;
        }
        if p.y > outside.y {
            outside.y = p.y;
        }
    });
    outside *= 2.0;
    // 2. Cast a ray from the query point to the 'outside' point, count number of intersections
    // TODO
    let mut count: u32 = 0;
    for i in 0..poly.len() {
        if intersect_segment(&outside, &query, &poly[i], &poly[(i + 1) % poly.len()]) {
            count += 1;
        }
    }
    if count % 2 == 1 { true } else { false }
}
