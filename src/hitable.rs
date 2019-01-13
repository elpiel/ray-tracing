use crate::ray::Ray;
use crate::Vec3;

pub struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub fn main(t: f64, p: Vec3, normal: Vec3) -> Self {
        Self { t, p, normal }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &HitRecord) -> bool;
}
