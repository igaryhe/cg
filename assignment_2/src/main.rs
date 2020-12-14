use image::{GrayAlphaImage, LumaA, RgbaImage, Rgba};
use glam::{Vec3, Mat3, vec3};
use nalgebra::{Matrix3, Vector3};
use mint::ColumnMatrix3;
use rayon::prelude::*;

fn raytrace_sphere() {
    println!("Simple ray tracer, one sphere whith orthographic projection");
    
    const FILENAME: &str = "img/sphere_orthographic.png";
    let mut img = GrayAlphaImage::new(800, 800); // Store the greyscale image

    // The camera is orthographic, pointing in the direction -z and covering the unit square (-1,1) in x and y
    let origin = vec3(-1.0, 1.0, 1.0);
    let x_displacement = vec3(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = vec3(0.0, -2.0 / img.height() as f32, 0.0);

    // Single light source
    let light_position = vec3(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        // Prepare the ray
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;

        // Intersect with the sphere
	// NOTE: this is a special case of a sphere centered in the origin and for orthographic rays aligned with the z axis
        let ray_on_xy = ray_origin.truncate();
        const SPHERE_RADIUS: f32 = 0.9;

        if ray_on_xy.length() < SPHERE_RADIUS {
            // The ray hit the sphere, compute the exact intersection point
            let ray_intersection = Vec3::new(ray_on_xy.x, ray_on_xy.y,
                                             (SPHERE_RADIUS.powi(2) - ray_on_xy.length_squared()).sqrt());

            // Compute normal at the intersection point
            let ray_normal = ray_intersection.normalize();

            // Simple diffuse model
            // Clamp to zero
            let color = (light_position - ray_intersection).normalize().dot(ray_normal).max(0.0);

            // Disable the alpha mask for this pixel
            *pixel = LumaA([(color * 255.0) as u8, 255]);
        }
    });

    // Save to png
    img.save(FILENAME).unwrap();
}

fn raytrace_parallelogram() {
    println!("Simple ray tracer, one parallelogram with orthographic projection");

    const FILENAME: &str = "img/plane_orthographic.png";    
    let mut img = GrayAlphaImage::new(800, 800); // Store the greyscale image

    // The camera is orthographic, pointing in the direction -z and covering the unit square (-1,1) in x and y
    let origin = vec3(-1.0, 1.0, 1.0);
    let x_displacement = vec3(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = vec3(0.0, -2.0 / img.height() as f32, 0.0);

    // TODO: Parameters of the parallelogram (position of the lower-left corner + two sides)
    let pgram_origin = Vec3::zero();
    let pgram_u = Vec3::zero();
    let pgram_v = Vec3::zero();

    // Single light source
    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        // Prepare the ray
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        // TODO: Check if the ray intersects with the parallelogram
        if true {
            // TODO: The ray hit the parallelogram, compute the exact intersection point
            let ray_intersection = Vec3::zero();

            // TODO: Compute normal at the intersection point
            let ray_normal = Vec3::one();

            // Simple diffuse model
            // Clamp to zero
            let color = (light_position - ray_intersection).normalize().dot(ray_normal).max(0.0);

            // Disable the alpha mask for this pixel
            *pixel = LumaA([(color * 255.0) as u8, 255]);
        }
    });

    // Save to png
    img.save(FILENAME).unwrap();
}

fn raytrace_perspective() {
    println!("Simple ray tracer, one parallelogram with perspective projection");

    const FILENAME: &str = "img/plane_perspective.png";
    let mut img = GrayAlphaImage::new(800, 800); // Store the greyscale image

    // The camera is perspective, pointing in the direction -z and covering the unit square (-1,1) in x and y
    let origin = Vec3::new(-1.0, 1.0, 1.0);
    let x_displacement = Vec3::new(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = Vec3::new(0.0, -2.0 / img.height() as f32, 0.0);

    // TODO: Parameters of the parallelogram (position of the lower-left corner + two sides)
    let pgram_origin = Vec3::zero();
    let pgram_u = Vec3::zero();
    let pgram_v = Vec3::zero();

    // Single light source
    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        // TODO: Prepare the ray (origin point and direction)
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        // TODO: Check if the ray intersects with the parallelogram
        if true {
            // TODO: The ray hit the parallelogram, compute the exact intersection point
            let ray_intersection = Vec3::zero();

            // TODO: Compute normal at the intersection point
            let ray_normal = Vec3::one();

            // Simple diffuse model
            // Clamp to zero
            let color = (light_position - ray_intersection).normalize().dot(ray_normal).max(0.0);

            // Disable the alpha mask for this pixel
            *pixel = LumaA([(color * 255.0) as u8, 255]);
        }
    });

    // Save to png
    img.save(FILENAME).unwrap();
}

fn raytrace_shading() {
    println!("Simple ray tracer, one sphere with different shading");

    const FILENAME: &str = "img/shading.png";
    let mut img = RgbaImage::new(800, 800); // Store the rgba image

    // The camera is perspective, pointing in the direction -z and covering the unit square (-1,1) in x and y
    let origin = Vec3::new(-1.0, 1.0, 1.0);
    let x_displacement = Vec3::new(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = Vec3::new(0.0, -2.0 / img.height() as f32, 0.0);

    // Single light source
    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);
    
    let kd = Vec3::new(0.9, 0.2, 0.9);
    let ks = Vec3::new(0.5, 0.2, 0.3);
    let i = Vec3::new(0.7, 0.7, 0.7);
    let ambient = Vec3::new(0.1, 0.1, 0.1);
    let p = 10;

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        // Prepare the ray
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;

        // Intersect with the sphere
	// NOTE: this is a special case of a sphere centered in the origin and for orthographic rays aligned with the z axis
        let ray_on_xy = ray_origin.truncate();
        const SPHERE_RADIUS: f32 = 0.9;

        if ray_on_xy.length() < SPHERE_RADIUS {
            // The ray hit the sphere, compute the exact intersection point
            let ray_intersection = Vec3::new(ray_on_xy.x, ray_on_xy.y,
                                             (SPHERE_RADIUS.powi(2) - ray_on_xy.length_squared()).sqrt());

            let ray_normal = ray_intersection.normalize();
            
            // TODO: Add shading parameter here
            let diffuse = Vec3::zero();
            let specular = Vec3::zero();

            // Simple diffuse model
            let color = ambient + diffuse + specular;

            // Write color to image buffer
            *pixel = Rgba([
                (color.x.max(0.0) * 255.0) as u8,
                (color.y.max(0.0) * 255.0) as u8,
                (color.z.max(0.0) * 255.0) as u8, 255]);
        }
    });

    // Save to png
    img.save(FILENAME).unwrap();
}

fn main() {
    raytrace_sphere();
    // raytrace_parallelogram();
    // raytrace_perspective();
    // raytrace_shading();
}
