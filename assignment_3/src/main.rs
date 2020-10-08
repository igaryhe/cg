mod structure;
mod function;
mod scene;
mod object;

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
            println!("{}", scene.objects.len());
            scene.render();
        }
    }
}