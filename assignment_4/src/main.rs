mod structure;
mod object;
mod scene;
mod function;

pub use scene::Scene;
pub use object::Object;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            eprintln!("Usage: {} scene.json", args[0]);
        },
        _ => {
            let scene = Scene::load(args[1].as_str());
            scene.render();
        }
    }
}
