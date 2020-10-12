use crate::{structure::*, Object};
use glam::Vec3;
use serde::Deserialize;
use rayon::prelude::*;
use rand::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressBar};

#[derive(Default, Deserialize)]
pub struct Scene {
    pub background_color: Vec3,
    pub ambient_light: Vec3,
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn load(filename: &str) -> Scene {
        use std::fs;
        let file = fs::read_to_string(filename).unwrap();
        serde_json::from_str(file.as_str()).unwrap()
    }

    pub fn render(&self) {
        use image::{RgbaImage, Rgba};
        use crate::function::*;

        println!("Simple ray tracer");
    
        let w = 640;
        let h = 480;
        let mut img = RgbaImage::new(w, h);
    
        let aspect_ratio = w as f32 / h as f32;
        // TODO: Stretch the pixel grid by the proper amount here
        let scale_y = self.camera.focal_length / ((std::f32::consts::PI - self.camera.field_of_view) / 2.0).tan();
        let scale_x = scale_y * aspect_ratio;
    
        let grid_origin = Vec3::new(-scale_x, scale_y, -self.camera.focal_length);
        let x_displacement = Vec3::new(2.0 / w as f32 * scale_x, 0.0, 0.0);
        let y_displacement = Vec3::new(0.0, -2.0 / h as f32 * scale_y, 0.0);

        let pb = ProgressBar::new((w * h) as u64);
    
        img.enumerate_pixels_mut().par_bridge().progress_with(pb).for_each(|(x, y, pixel)| {
            let shift = grid_origin + (x as f32 + 0.5) * x_displacement + (y as f32 + 0.5) * y_displacement;

            let ray_count = 5;

            let mut colors = vec![Vec3::zero(); ray_count];

            colors.par_iter_mut().for_each(|color| {
                let mut rng = rand::thread_rng();
                let r = self.camera.lens_radius * rng.gen::<f32>().sqrt();
                let theta = 2.0 * std::f32::consts::PI * rng.gen::<f32>();
                let offset = Vec3::new(r * theta.cos(), r * theta.sin(), 0.0);

                let max_bounce = 5;

                let ray = match self.camera.is_perspective {
                    true => {
                        // Perspective camera
                        // TODO
                        let origin = self.camera.position + offset;
                        let direction = (shift - offset).normalize();
                        Ray {
                            origin,
                            direction,
                        }
                    },
                    false => {
                        Ray {
                            origin: self.camera.position + Vec3::new(shift.x(), shift.y(), 0.0),
                            direction: Vec3::new(0.0, 0.0, -1.0),
                        }
                    }
                };

                *color = shoot_ray(self, ray, max_bounce);
            });
            let color: Vec3 = colors.into_iter().fold(Vec3::zero(), |sum, c| sum + c) / ray_count as f32;
            *pixel = Rgba([(color.x() * 255.0) as u8, (color.y() * 255.0) as u8, (color.z() * 255.0) as u8, 255]);
        });
    
        const FILENAME: &str = "img/raytracer.png";
        img.save(FILENAME).unwrap();
    }
}
