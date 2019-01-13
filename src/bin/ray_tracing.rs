use std::f64;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

use ray_trace::hitable::Hitable;
use ray_trace::hitable::HitableList;
use ray_trace::object::Sphere;
use ray_trace::Ray;
use ray_trace::Vec3;

fn main() -> Result<(), std::io::Error> {
    let width = 200;
    let height = 100;
    let max_color = 255;

    let file = File::create("image.ppm")?;
    let mut file = LineWriter::new(file);

    let beginning_file = format!("P3\n{} {}\n{}\n", width, height, max_color);
    file.write_all(beginning_file.as_bytes())?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let objects = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];
    let world = HitableList::from(&objects);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = f64::from(i) / f64::from(width);
            let v = f64::from(j) / f64::from(height);
            let destination = lower_left_corner + u * horizontal + v * vertical;

            let ray = Ray::new(origin, destination);
            let col = color(&ray, &world);

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            let pixel_color = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(pixel_color.as_bytes())?;
        }
    }

    Ok(())
}

fn color<T: Hitable>(ray: &Ray, world: &HitableList<T>) -> Vec3 {
    let vec3_1_1_1 = Vec3::new(1.0, 1.0, 1.0);

    match world.hit(ray, 0.0, f64::MAX) {
        Some(hit_result) => {
            0.5 * Vec3::new(
                hit_result.normal.x() + 1.0,
                hit_result.normal.y() + 1.0,
                hit_result.normal.z() + 1.0,
            )
        }
        None => {
            let unit_direction = Vec3::unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - t) * vec3_1_1_1 + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}
