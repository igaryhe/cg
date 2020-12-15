use crate::{structure::*, Object};
use glam::Vec3;
use serde::Deserialize;
use rayon::prelude::*;
use rand::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressBar};
use std::fs::File;
use anyhow::{Result, Error};

#[derive(Default, Deserialize)]
pub struct Scene {
    pub background_color: Vec3,
    pub ambient_light: Vec3,
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn load(filename: &str) -> Result<Self> {
        use std::fs;
        let file = fs::read_to_string(filename)?;
        serde_json::from_str(file.as_str()).map_err(Error::msg)
    }

    pub fn render(&mut self, img_type: Image) -> Result<()> {
        use image::{RgbaImage, Rgba, Frame, Delay,  gif::Encoder};
        use crate::function::*;

        println!("Simple ray tracer");
    
        let w = 640;
        let h = 480;

        // The camera always points in the direction -z
	// The sensor grid is at a distance 'focal_length' from the camera center,
	// and covers an viewing angle given by 'field_of_view'.
        let aspect_ratio = w as f32 / h as f32;
        // TODO: Stretch the pixel grid by the proper amount here
        let scale_y = 1.0;
        let scale_x = 1.0;

        // The pixel grid through which we shoot rays is at a distance 'focal_length'
	// from the sensor, and is scaled from the canonical [-1,1] in order
	// to produce the target field of view.
        let grid_origin = Vec3::new(-scale_x, scale_y, -self.camera.focal_length);
        let x_displacement = Vec3::new(2.0 / w as f32 * scale_x, 0.0, 0.0);
        let y_displacement = Vec3::new(0.0, -2.0 / h as f32 * scale_y, 0.0);

        let max_bounce = 5;
        let ray_count = 20;

        match img_type {
            Image::Gif => {
                let mut frames: Vec<Frame> = vec![];
                let gif = File::create("img/anim.gif")?;
                let mut encoder = Encoder::new(gif);
                for i in 0..12 {
                    // TODO
                    // draw each frame of gif
                    let mut img = RgbaImage::new(w, h);
                    let pb = ProgressBar::new((w * h) as u64);
                    let frame = Frame::from_parts(img, 0, 0,Delay::from_saturating_duration(
                        std::time::Duration::from_millis(500)));
                    frames.push(frame);
                }
                encoder.encode_frames(frames)?;
            },
            Image::Png => {
                let mut img = RgbaImage::new(w, h);
                let pb = ProgressBar::new((w * h) as u64);
                img.enumerate_pixels_mut().par_bridge().progress_with(pb).for_each(|(x, y, pixel)| {
                    // TODO: Implement depth of field
                    let shift = grid_origin + (x as f32 + 0.5) * x_displacement
                        + (y as f32 + 0.5) * y_displacement;

                    let ray = match self.camera.is_perspective {
                        true => {
                            // Perspective camera
                            // TODO
                            Ray::default()
                        },
                        false => {
                            // Orthographic camera
                            Ray {
                                origin: self.camera.position + Vec3::new(shift.x, shift.y, 0.0),
                                direction: Vec3::new(0.0, 0.0, -1.0),
                            }
                        }
                    };

                    let color = shoot_ray(self, ray, max_bounce);
                    *pixel = Rgba([(color.x * 255.0) as u8,
                                   (color.y * 255.0) as u8,
                                   (color.z * 255.0) as u8,
                                   255]);
                });
                
                // Save to png
                const FILENAME: &str = "img/raytracer.png";
                img.save(FILENAME)?;
            },
        };
        Ok(())
    }
}

pub enum Image {
    Png,
    Gif,
}
