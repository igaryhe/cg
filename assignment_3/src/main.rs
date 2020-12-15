mod structure;
mod function;
mod scene;
mod object;

pub use scene::{Scene, Image};
pub use object::Object;
use std::env;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            eprintln!("Usage: {} scene.json", args[0]);
        },
        _ => {
            let mut scene = Scene::load(args[1].as_str())?;
            println!("{}", scene.objects.len());
            scene.render(Image::Png)?;
        }
    }
    Ok(())
}
