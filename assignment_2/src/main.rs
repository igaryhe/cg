use image::{GrayAlphaImage, LumaA, RgbaImage, Rgba};
use glam::{Vec3, Mat3};
use nalgebra::{Matrix3, Vector3};
use mint::ColumnMatrix3;
use rayon::prelude::*;

fn raytrace_sphere() {
    println!("Simple ray tracer, one sphere whith orthographic projection");
    const FILENAME: &str = "img/sphere_orthographic.png";

    let mut img = GrayAlphaImage::new(800, 800);

    let origin = Vec3::new(-1.0, 1.0, 1.0);
    let x_displacement = Vec3::new(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = Vec3::new(0.0, -2.0 / img.height() as f32, 0.0);

    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        // let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        let ray_on_xy = ray_origin.truncate();
        const SPHERE_RADIUS: f32 = 0.9;

        if ray_on_xy.length() < SPHERE_RADIUS {

            let ray_intersection = Vec3::new(ray_on_xy.x(), ray_on_xy.y(), (SPHERE_RADIUS * SPHERE_RADIUS - ray_on_xy.length_squared()).sqrt());

            let ray_normal = ray_intersection.normalize();

            let color = (light_position - ray_intersection).normalize().dot(ray_normal).max(0.0);
            
            *pixel = LumaA([(color * 255.0) as u8, 255]);
        }
    });

    img.save(FILENAME).unwrap();
}

fn raytrace_parallelogram() {
    println!("Simple ray tracer, one parallelogram with orthographic projection");

    const FILENAME: &str = "img/plane_orthographic.png";
    let mut img = GrayAlphaImage::new(800, 800);

    let origin = Vec3::new(-1.0, 1.0, 1.0);
    let x_displacement = Vec3::new(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = Vec3::new(0.0, -2.0 / img.height() as f32, 0.0);

    // TODO: Parameters of the parallelogram (position of the lower-left corner + two sides)
    let pgram_origin = Vec3::new(-0.5, -0.5, -1.0);
    let pgram_u = Vec3::new(0.3, 1.0, -0.2);
    let pgram_v = Vec3::new(1.0, 0.3, 0.2);

    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        // TODO: Check if the ray intersects with the parallelogram
        let a = Mat3::from_cols(pgram_u, pgram_v, -ray_direction);
        let a = Matrix3::from(ColumnMatrix3::from(a));
        let decomp = a.lu();
        let b = ray_origin - pgram_origin;
        let b = Vector3::from(mint::Vector3::from(b));
        let x = decomp.solve(&b).unwrap();
        let eq = x[0] >= 0.0 && x[0] <= 1.0 && x[1] >= 0.0 && x[1] <= 1.0 && x[2] >= 0.0;
        if eq {
            // TODO: The ray hit the parallelogram, compute the exact intersection point
            let ray_intersection = ray_origin + x[2] * ray_direction;

            // TODO: Compute normal at the intersection point
            let ray_normal = pgram_v.cross(pgram_u).normalize();

            let color = (light_position - ray_intersection).normalize().dot(ray_normal).max(0.0);
            
            *pixel = LumaA([(color * 255.0) as u8, 255]);
        }
    });

    img.save(FILENAME).unwrap();
}

fn raytrace_perspective() {
    println!("Simple ray tracer, one parallelogram with perspective projection");

    const FILENAME: &str = "img/plane_perspective.png";
    let mut img = GrayAlphaImage::new(800, 800);

    let origin = Vec3::new(-1.0, 1.0, 1.0);
    let x_displacement = Vec3::new(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = Vec3::new(0.0, -2.0 / img.height() as f32, 0.0);

    // TODO: Parameters of the parallelogram (position of the lower-left corner + two sides)
    let pgram_origin = Vec3::new(-0.5, -0.5, -1.0);
    let pgram_u = Vec3::new(0.3, 1.0, -0.2);
    let pgram_v = Vec3::new(1.0, 0.3, 0.2);

    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        // TODO: Prepare the ray (origin point and direction)
        let ray_origin = Vec3::new(0.0, 0.0, 2.0);
        let ray_direction = (origin + x_displacement * x as f32 + y_displacement * y as f32 - ray_origin).normalize();

        // TODO: Check if the ray intersects with the parallelogram
        let a = Mat3::from_cols(pgram_u, pgram_v, -ray_direction);
        let a = Matrix3::from(ColumnMatrix3::from(a));
        let decomp = a.lu();
        let b = ray_origin - pgram_origin;
        let b = Vector3::from(mint::Vector3::from(b));
        let x = decomp.solve(&b).unwrap();
        let eq = x[0] >= 0.0 && x[0] <= 1.0 && x[1] >= 0.0 && x[1] <= 1.0 && x[2] >= 0.0;
        if eq {
            // TODO: The ray hit the parallelogram, compute the exact intersection point
            let ray_intersection = ray_origin + x[2] * ray_direction;

            // TODO: Compute normal at the intersection point
            let ray_normal = pgram_v.cross(pgram_u).normalize();

            let color = (light_position - ray_intersection).normalize().dot(ray_normal).max(0.0);

            *pixel = LumaA([(color * 255.0) as u8, 255]);
        }
    });

    img.save(FILENAME).unwrap();
}

fn raytrace_shading() {
    println!("Simple ray tracer, one sphere with different shading");

    const FILENAME: &str = "img/shading.png";
    let mut img = RgbaImage::new(800, 800);

    let origin = Vec3::new(-1.0, 1.0, 1.0);
    let x_displacement = Vec3::new(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = Vec3::new(0.0, -2.0 / img.height() as f32, 0.0);

    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    let kd = Vec3::new(0.9, 0.2, 0.9);
    let ks = Vec3::new(0.5, 0.2, 0.3);
    let i = Vec3::new(0.7, 0.7, 0.7);
    let ambient = Vec3::new(0.1, 0.1, 0.1);
    let p = 10;

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, pixel)| {
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        // let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        let ray_on_xy = ray_origin.truncate();
        const SPHERE_RADIUS: f32 = 0.9;

        if ray_on_xy.length() < SPHERE_RADIUS {

            let ray_intersection = Vec3::new(ray_on_xy.x(), ray_on_xy.y(), (SPHERE_RADIUS * SPHERE_RADIUS - ray_on_xy.length_squared()).sqrt());

            let n = ray_intersection.normalize();
            let l = (light_position - ray_intersection).normalize();
            let v = (ray_origin - ray_intersection).normalize();

            let diffuse = kd * i * n.dot(l).max(0.0);
            let specular = ks * i * n.dot((v + l).normalize()).max(0.0).powi(p);

            let color = ambient + diffuse + specular;

            *pixel = Rgba([
                (color.x().max(0.0) * 255.0) as u8,
                (color.y().max(0.0) * 255.0) as u8,
                (color.z().max(0.0) * 255.0) as u8, 255]);
        }
    });

    img.save(FILENAME).unwrap();
}

fn main() {
    raytrace_sphere();
    raytrace_parallelogram();
    raytrace_perspective();
    raytrace_shading();
}