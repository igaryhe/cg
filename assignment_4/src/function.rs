use crate::{structure::*, Object, Scene};
use glam::Vec3;

pub fn ray_color(scene: &Scene, ray: Ray, obj: &Box<dyn Object>, hit: Intersection) -> Vec3 {
    let mat: Material = obj.material();

    let ambient_color: Vec3 = mat.ambient_color * scene.ambient_light;

    let mut lights_color = Vec3::zero();
    let n = hit.normal;
    let v = (ray.origin - hit.position).normalize();
    scene.lights.iter().for_each(|light| {
        let li: Vec3 = (light.position - hit.position).normalize();

        // TODO: Shoot a shadow ray to determine if the light should affect the intersection point
        if is_light_visible(scene, &hit, light) {
            
        }

        // Diffuse contribution
        let diffuse = mat.diffuse_color * li.dot(n).max(0.0);

        // TODO: Specular contribution
        let specular = mat.specular_color * n.dot((li + v).normalize()).max(0.0).powf(mat.specular_exponent);

        let d = light.position - hit.position;
        lights_color += (diffuse + specular) * light.intensity / d.length_squared();
    });

    /*
    // TODO: Compute the color of the reflected ray and add its contribution to the current point color.
    let r = 2.0 * n.dot(v) * n - v;
    let reflect_ray = Ray {
        origin: hit.position + r * 0.0001,
        direction: r,
    };
    let reflection_color = match find_nearest_object(scene, &reflect_ray) {
        Some((obj, hit)) => {
            if max_bounce != 0 {
                mat.reflection_color * ray_color(scene, reflect_ray, obj, hit, max_bounce - 1)
            } else { Vec3::zero() }
        },
        None => Vec3::zero(),
    };

    // TODO: Compute the color of the refracted ray and add its contribution to the current point color.
    //       Make sure to check for total internal reflection before shooting a new ray.
    let dt = v.dot(n);
    let discriminant = 1.0 - mat.refraction_index.powi(2) * (1.0 - dt * dt);
    let refraction_color = match discriminant {
        x if x > 0.0 => {
            let t = (mat.refraction_index * (v - dt * n) - n * discriminant.sqrt()).normalize();
            let refract_ray = Ray {
                origin: hit.position + t * 0.0001,
                direction: r,
            };
            match find_nearest_object(scene, &refract_ray) {
                Some((obj, hit)) => {
                    if max_bounce != 0 {
                        mat.refraction_color * ray_color(scene, refract_ray, obj, hit, max_bounce - 1)
                    } else { Vec3::zero() }
                },
                None => Vec3::zero(),
            }
        },
        _ => Vec3::zero(),
    };
    */

    ambient_color + lights_color // + reflection_color + refraction_color
}

pub fn find_nearest_object<'a>(scene: &'a Scene, ray: &Ray) -> Option<(&'a Box<dyn Object>, Intersection)> {
    let mut nearest: Option<(&'a Box<dyn Object>, Intersection)> = None;
    let mut distance = -1.0;
    // TODO:
    //
    // Find the object in the scene that intersects the ray first
    // The function must return 'nullptr' if no object is hit, otherwise it must
    // return a pointer to the hit object, and set the parameters of the argument
    // 'hit' to their expected values.
    scene.objects.iter().for_each(|obj| {
        match obj.intersect(ray) {
            Some(i) => {
                let current_distance = (i.position - ray.origin).length();
                match nearest {
                    None => {
                        nearest = Some((obj, i));
                        distance = current_distance;
                    },
                    Some(_) => {
                        if current_distance < distance {
                            nearest = Some((obj, i));
                            distance = current_distance;
                        }
                    },
                }
            },
            None => {},
        }
    });
    nearest
}

pub fn is_light_visible(scene: &Scene, hit: &Intersection, light: &Light) -> bool {
    // TODO: Determine if the light is visible here
    let direction = (light.position - hit.position).normalize();
    let shadow_ray = Ray {
        origin: hit.position + direction * 0.0001,
        direction,
    };
    match find_nearest_object(scene, &shadow_ray) {
        Some(_) => false,
        None => true,
    }
}

pub fn shoot_ray(scene: &Scene, ray: Ray) -> Vec3 {
    match find_nearest_object(scene, &ray) {
        Some((obj, hit)) => ray_color(scene, ray, obj, hit),
        None => scene.background_color
    }
}
