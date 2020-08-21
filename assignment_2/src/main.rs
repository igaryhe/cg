use image::{GrayAlphaImage, LumaA};
use glam::Vec3;

fn raytrace_sphere() {
    println!("Simple ray tracer, one sphere whith orthographic projection");
    const FILENAME: &str = "img/sphere_orthographic.png";

    let mut img = GrayAlphaImage::new(800, 800);

    let origin = Vec3::new(-1.0, 1.0, 1.0);
    let x_displacement = Vec3::new(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = Vec3::new(0.0, -2.0 / img.height() as f32, 0.0);

    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

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
    let pgram_origin = Vec3::zero();
    let pgram_u = Vec3::zero();
    let pgram_v = Vec3::zero();

    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        // TODO: Check if the ray intersects with the parallelogram
        if true {
            // TODO: The ray hit the parallelogram, compute the exact intersection point
            let ray_intersection = Vec3::new(0.0, 0.0, 0.0);

            // TODO: Compute normal at the intersection point
            let ray_normal = ray_intersection.normalize();

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
    let pgram_origin = Vec3::zero();
    let pgram_u = Vec3::zero();
    let pgram_v = Vec3::zero();

    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);

    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        // TODO: Prepare the ray (origin point and direction)
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        // TODO: Check if the ray intersects with the parallelogram
        if true {
            // TODO: The ray hit the parallelogram, compute the exact intersection point
            let ray_intersection = Vec3::new(0.0, 0.0, 0.0);

            // TODO: Compute normal at the intersection point
            let ray_normal = ray_intersection.normalize();

            let color = (light_position - ray_intersection).normalize().dot(ray_normal).max(0.0);

            *pixel = LumaA([(color * 255.0) as u8, 255]);
        }
    });

    img.save(FILENAME).unwrap();
}

fn raytrace_shading() {
    println!("Simple ray tracer, one sphere with different shading");

    const FILENAME: &str = "img/shading.png";
    let mut img = GrayAlphaImage::new(800, 800);

    let origin = Vec3::new(-1.0, 1.0, 1.0);
    let x_displacement = Vec3::new(2.0 / img.width() as f32, 0.0, 0.0);
    let y_displacement = Vec3::new(0.0, -2.0 / img.height() as f32, 0.0);

    let light_position: Vec3 = Vec3::new(-1.0, 1.0, 1.0);
    let ambient = 0.1;
    let mut diffuse = vec![vec![0f32; 800]; 800];
    let mut specular = vec![vec![0f32; 800]; 800];

    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let ray_origin = origin + x_displacement * x as f32 + y_displacement * y as f32;
        let ray_direction = Vec3::new(0.0, 0.0, -1.0);

        let ray_on_xy = ray_origin.truncate();
        const SPHERE_RADIUS: f32 = 0.9;

        if ray_on_xy.length() < SPHERE_RADIUS {

            let ray_intersection = Vec3::new(ray_on_xy.x(), ray_on_xy.y(), (SPHERE_RADIUS * SPHERE_RADIUS - ray_on_xy.length_squared()).sqrt());

            let ray_normal = ray_intersection.normalize();

            let dx = x as usize;
            let dy = y as usize;
            diffuse[dx][dy] = (light_position - ray_intersection).normalize().dot(ray_normal);
            specular[dx][dy] = (light_position - ray_intersection).normalize().dot(ray_normal);

            let color = (ambient + diffuse[dx][dy] + specular[dx][dy]).max(0.0);

            *pixel = LumaA([(color * 255.0) as u8, 255]);
        }
    });

    img.save(FILENAME).unwrap();
}

fn main() {
    raytrace_sphere();
    // raytrace_parallelogram();
    // raytrace_perspective();
    // raytrace_shading();
}