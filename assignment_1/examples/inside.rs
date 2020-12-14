use std::env;
use assignment_1::*;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 | 2 | 3 => {
            eprintln!("Usage: {} points.xyz poly.obj result.xyz", args[0]);   
        },
        _ => {
            let points = load_xyz(args[1].as_str())?;
            let poly = load_obj(args[2].as_str())?;
            let mut result: Polygon = vec![];
            points.into_iter().for_each(|point| {
                if is_inside(&poly, point) {
                    result.push(point);
                }
            });
            save_xyz(args[3].as_str(), result)?;
        }
    }
    Ok(())
}
