use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
     e: [f64; 3]
}

impl Vec3 {
    fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
    fn x(&self) -> f64 {
        self.e[0]
    }
    fn y(&self) -> f64 {
        self.e[1]
    }
    fn z(&self) -> f64 {
        self.e[2]
    }
    fn r(&self) -> f64 {
        self.e[0]
    }
    fn g(&self) -> f64 {
        self.e[1]
    }
    fn b(&self) -> f64 {
        self.e[2]
    }
}

// Altering the Vec3 itself
impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, other: &Vec3) -> Self {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];

        self
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(mut self, other: &Vec3) -> Self {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];

        self
    }
}

/*impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-e[0], -3[])
        match self {
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
            Sign::Positive => Sign::Negative,
        }
    }
}*/

// Altering the &Vec3
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2])
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2])
    }
}

//impl Clone for Vec3 {
//    fn clone(&self) -> Vec3 { *self }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two_vec3s_by_ref_and_returns_new_vec3() {
        let vec3_first = Vec3::new(1.0, 2.0, 3.0);
        let vec3_second = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(Vec3::new(5.0,7.0,9.0), &vec3_first + &vec3_second);
        // make sure we haven't changed the original vec3s
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), vec3_first);
        assert_eq!(Vec3::new(4.0, 5.0, 6.0), vec3_second);
    }

    #[test]
    fn it_adds_vec3_to_the_first_one() {
        let vec3_first = Vec3::new(1.0, 2.0, 3.0);
        let vec3_second = Vec3::new(4.0, 5.0, 6.0);

        let vec3_result = vec3_first + &vec3_second;
        assert_eq!(&Vec3::new(5.0,7.0,9.0), &vec3_first);
        assert_eq!(&Vec3::new(5.0,7.0,9.0), &vec3_result);
        assert_eq!(Vec3::new(4.0, 5.0, 6.0), vec3_second);
    }

    #[test]
    fn it_subtracts_two_vec3s() {
        let vec3_first = Vec3::new(4.0, 6.0, 8.0);
        let vec3_second = Vec3::new(3.0, 4.0, 5.0);

        assert_eq!(Vec3::new(1.0,2.0,3.0), &vec3_first - &vec3_second);
        // make sure we haven't changed the original vec3s
        assert_eq!(Vec3::new(4.0, 6.0, 8.0), vec3_first);
        assert_eq!(Vec3::new(3.0, 4.0, 5.0), vec3_second);
    }
}
