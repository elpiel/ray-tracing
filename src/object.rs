use crate::hitable::{HitRecord, Hitable};
use crate::Ray;
use crate::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_center = ray.origin - self.center;

        let a: f64 = Vec3::dot(ray.direction, ray.direction);
        let b: f64 = Vec3::dot(origin_center, ray.direction);
        let c: f64 = Vec3::dot(origin_center, origin_center) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            let discr_sqrt = discriminant.sqrt();
            let temp: f64 = (-b - discr_sqrt) / a;

            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord::new(temp, p, normal));
            }

            let temp: f64 = (-b + discr_sqrt) / a;

            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord::new(temp, p, normal));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_hits_sphere_on_one_point() {
        // TODO: Implement
    }

    #[test]
    fn it_hits_sphere_on_two_points() {
        // TODO: Implement
    }

    #[test]
    fn it_does_not_hit_sphere() {
        // TODO: Implement
    }
}
