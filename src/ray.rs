use crate::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_creates_a_ray_and_points_at_parameter() {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(Vec3::new(9.0, 12.0, 15.0), ray.point_at_parameter(2.0));
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0), &ray.origin());
        assert_eq!(&Vec3::new(4.0, 5.0, 6.0), &ray.direction());

    }
}