use std::env;
use assignment_1::*;
use assignment_1::file::*;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => {
            eprintln!("Usage: {} points.xyz output.obj", args[0]);
        },
        _ => {
            let mut points = load_xyz(args[1].as_str())?;
            let hull = convex_hull(&mut points);
            save_obj(args[2].as_str(), hull?)?;
        }
    }
    Ok(())
}
