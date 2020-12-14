use crate::structure::*;
use std::fs::File;
use std::io::{prelude::*, BufRead, BufReader};
use anyhow::Result;

pub fn load_xyz(filename: &str) -> Result<Vec<Point>> {
    let mut points: Vec<Point> = vec![];
    let file = File::open(filename)?;
    // TODO
    Ok(points)
}

pub fn save_xyz(filename: &str, points: Vec<Point>) -> Result<()> {
    todo!()
}

pub fn load_obj(filename: &str) -> Result<Vec<Point>> {
    // TODO
    let file = File::open(filename)?;
    todo!()
}

pub fn save_obj(filename: &str, poly: Vec<Point>) -> Result<()> {
    let mut file = File::create(filename).unwrap();
    poly.iter().for_each(|point| {
        writeln!(file, "v {} {} 0", point.x, point.y).unwrap();
    });
    for i in 1..=poly.len() {
        writeln!(file, "l {} {}", i, 1 + i % poly.len()).unwrap();
    }
    writeln!(file)?;
    Ok(())
}
