mod structure;
mod object;
mod scene;
mod function;
mod bbox;
mod triangle;

pub use scene::Scene;
pub use object::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            eprintln!("Usage: {} scene.json", args[0]);
        },
        _ => {
            let mut scene = Scene::load(args[1].as_str());
            let mesh = Mesh::new("data/bunny.off", scene.materials[0]);
            scene.objects.clear();
            scene.objects.push(Box::new(mesh));
            scene.render();
        }
    }
}
