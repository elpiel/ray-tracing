use rand::{thread_rng, Rng};

use crate::{hitable::HitRecord, Ray, Vec3};
use std::fmt::Debug;

pub trait Material: Debug {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Vec3, Option<Ray>);
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Vec3, Option<Ray>) {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

        (
            self.albedo,
            Some(Ray::new(hit_record.p, target - hit_record.p)),
        )
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Vec3, Option<Ray>) {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();

        let scattered = Ray::new(hit_record.p, target - hit_record.p);

        (self.albedo, Some(scattered))
    }
}
//
#[derive(Debug, PartialEq, Clone)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (Vec3, Option<Ray>) {
        let unit_vec3 = Vec3::unit_vector(ray_in.direction);
        let reflected = reflect(&unit_vec3, &hit_record.normal);

        let scattered_ray = Ray::new(hit_record.p, reflected);

        let scattered = match Vec3::dot(scattered_ray.direction, hit_record.normal) > 0.0 {
            true => Some(scattered_ray),
            false => None,
        };
        let attenuation = self.albedo;

        (attenuation, scattered)
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(*v, *n) * *n
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let mut p: Vec3;
    loop {
        let random_vec3 = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
        p = 2.0 * random_vec3 - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}
