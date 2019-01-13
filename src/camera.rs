use crate::Ray;
use crate::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Self {
        Self {
            origin,
            lower_left_corner,
            vertical,
            horizontal,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;

        Ray {
            origin: self.origin,
            direction,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_can_get_a_ray_at_a_point() {
        // TODO: Implement for get_ray()
    }
}
