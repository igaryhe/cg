use crate::structure::*;
use std::fs::File;
use std::io::{prelude::*, BufRead, BufReader};
use anyhow::Result;

pub fn load_xyz(filename: &str) -> Result<Vec<Point>> {
    let mut points: Vec<Point> = vec![];
    let file = File::open(filename)?;
    // TODO
    let reader = BufReader::new(file);
    let mut first_line = true;
    for line in reader.lines() {
        if first_line {
            first_line = false;
            continue;
        }
        let nums: Vec<f32> = line?.split(" ").flat_map(str::parse).collect();
        points.push(point(nums[0], nums[1]));
    }
    Ok(points)
}

pub fn save_xyz(filename: &str, points: Vec<Point>) -> Result<()> {
    // TODO
    let mut file = File::create(filename)?;
    writeln!(file, "{}", points.len())?;
    points.iter().for_each(|&point| {
        writeln!(file, "{} {} 0", point.x, point.y).unwrap();
    });
    Ok(())
}

pub fn load_obj(filename: &str) -> Result<Vec<Point>> {
    // TODO
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut obj: Vec<Point> = vec![];
    for line in reader.lines() {
        let raw_line = line?;
        let mut words: Vec<&str> = raw_line.split(" ").collect();
        if words[0] == "v" {
            words.remove(0);
            obj.push(point(words[0].parse()?, words[1].parse()?));
        }
    }
    Ok(obj)
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
