use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

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

    for j in (0..height).rev() {
        for i in 0..width {
            let u = f64::from(i) / f64::from(width);
            let v = f64::from(j) / f64::from(height);
            let destination = lower_left_corner + u * horizontal + v * vertical;

            let ray = Ray::new(origin, destination);
            let col = color(&ray);

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            let pixel_color = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(pixel_color.as_bytes())?;
        }
    }

    Ok(())
}

fn color(ray: &Ray) -> Vec3 {
    let sphere_center = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&sphere_center, 0.5, &ray);
    if t > 0.0 {
        let n = Vec3::unit_vector(ray.point_at_parameter(t) - sphere_center);

        return 0.5 * (n + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_direction = Vec3::unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let origin_center = ray.origin - *center;

    let a: f64 = Vec3::dot(ray.direction, ray.direction);
    let b: f64 = 2.0 * Vec3::dot(origin_center, ray.direction);
    let c: f64 = Vec3::dot(origin_center, origin_center) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    (-b - discriminant.sqrt()) / (2.0 * a)
}
