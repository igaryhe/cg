use crate::structure::*;
use std::fs::File;
use std::io::{prelude::*, BufRead, BufReader};

pub fn load_xyz(filename: &str) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    let file = File::open(filename).unwrap();
    // TODO
    let reader = BufReader::new(file);
    let mut first_line = true;
    for line in reader.lines() {
        if first_line {
            first_line = false;
            continue;
        }
        let nums: Vec<f32> = line.unwrap().split(" ").flat_map(str::parse).collect();
        points.push(point(nums[0], nums[1]));
    }
    points
}

pub fn save_xyz(filename: &str, points: Vec<Point>) {
    // TODO
    let mut file = File::create(filename).unwrap();
    writeln!(file, "{}", points.len()).unwrap();
    points.iter().for_each(|&point| {
        writeln!(file, "{} {} 0", point.x, point.y).unwrap();
    });
}

pub fn load_obj(filename: &str) -> Vec<Point> {
    // TODO
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut obj: Vec<Point> = vec![];
    for line in reader.lines() {
        let raw_line = line.unwrap();
        let mut words: Vec<&str> = raw_line.split(" ").collect();
        if words[0] == "v" {
            words.remove(0);
            obj.push(point(words[0].parse().unwrap(), words[1].parse().unwrap()));
        }
    }
    obj
}

pub fn save_obj(filename: &str, poly: Vec<Point>) {
    let mut file = File::create(filename).unwrap();
    poly.iter().for_each(|point| {
        writeln!(file, "v {} {} 0", point.x, point.y).unwrap();
    });
    for i in 1..=poly.len() {
        writeln!(file, "l {} {}", i, 1 + i % poly.len()).unwrap();
    }
    writeln!(file).unwrap();
}