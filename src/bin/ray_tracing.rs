use std::f64;
use std::fs::File;
use std::io::LineWriter;
use std::io::prelude::*;
use std::ops::Div;

use rand::{Rng, thread_rng};

use ray_trace::Camera;
use ray_trace::hitable::Hitable;
use ray_trace::hitable::HitableList;
use ray_trace::material::Lambertian;
use ray_trace::object::Sphere;
use ray_trace::Ray;
use ray_trace::Vec3;

fn main() -> Result<(), std::io::Error> {
    let width = 200;
    let height = 100;
    let samples_per_pixel = 100;
    let max_color = 255;

    let file = File::create("image.ppm")?;
    let mut file = LineWriter::new(file);

    let beginning_file = format!("P3\n{} {}\n{}\n", width, height, max_color);
    file.write_all(beginning_file.as_bytes())?;

    let objects = vec!
    [
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian { albedo: Vec3::new(0.8, 0.3, 0.3) })),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) })),
    ];
    let world = HitableList::from(&objects);
    let camera = Camera::default();
    let mut rng = thread_rng();

    for j in (0..height).rev() {
        for i in 0..width {
            let col = (0..samples_per_pixel)
                .fold(Vec3::new(0.0, 0.0, 0.0), |mut col, _| {
                    let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(width);
                    let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(height);

                    let ray = camera.get_ray(u, v);
                    let _p = ray.point_at_parameter(2.0);

                    col += color(&ray, &world, 0);
                    col
                })
                .div(f64::from(samples_per_pixel))
                // Add `Gamma: 2` for fixing the color
                .sqrt();

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            let pixel_color = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(pixel_color.as_bytes())?;
        }
    }

    Ok(())
}

fn color<T: Hitable>(ray: &Ray, world: &HitableList<T>, depth: i32) -> Vec3 {
    let vec3_1_1_1 = Vec3::new(1.0, 1.0, 1.0);

    match world.hit(ray, 0.001, f64::MAX) {
        Some(hit_record) => {
            let scatter = hit_record.material.scatter(&ray, &hit_record);
            match (depth < 50, scatter.1) {
                (true, Some(scattered_ray)) => color(&scattered_ray, world, depth + 1),
                (_, _) => Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = Vec3::unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - t) * vec3_1_1_1 + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}
