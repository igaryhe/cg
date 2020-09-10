use std::ops::{Sub, Mul, MulAssign};
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn point(x: f32, y: f32) -> Point {
    Point { x, y }
}

impl Sub for Point {
    type Output = Vec2;
    
    fn sub(self, other: Point) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        self.y.partial_cmp(&other.y)
    }
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Mul for Vec2 {
    type Output = f32;

    fn mul(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}