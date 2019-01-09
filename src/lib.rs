use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::Index;
use std::ops::Div;
use std::ops::Mul;
use std::ops::IndexMut;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    fn squared_length(&self) -> f64 {
        self.e.iter().map(|val| val.powi(2)).sum::<f64>()
    }

    fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    fn dot(left: &Vec3, right: &Vec3) -> f64 {
        left.e.iter().enumerate().map(|(index, val)| val * right[index]).sum()
    }

    fn cross(left: &Vec3, right: &Vec3) -> Vec3 {
        Vec3::new(
            left.e[1] * right.e[2] - left.e[2] * right.e[1],
            -(left.e[0] * right.e[2] - left.e[2] * right.e[0]),
            left.e[0] * right.e[1] - left.e[1] * right.e[2],
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl Index<usize> for &Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

// Add/Sub - Altering the &Vec3
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

// Add/AddAssign/Sub/SubAssign - With Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2])
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2])
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

// Mul/Div - Altering the &Vec3
impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2])
    }
}

impl Div for &Vec3 {
    type Output = Vec3;

    fn div(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2])
    }
}

// Mul/Div - With Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(other * self.e[0], other * self.e[1], other * self.e[2])
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, multiplier: Vec3) -> Vec3 {
        Vec3::new(self.e[0] / multiplier.e[0], self.e[1] / multiplier.e[1], self.e[2] / multiplier.e[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, divider: f64) -> Vec3 {
        Vec3::new(self.e[0] / divider, self.e[1] / divider, self.e[2] / divider)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_new_vec3_with_valid_properties() {
        let mut vec3 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, vec3.x());
        assert_eq!(2.0, vec3.y());
        assert_eq!(3.0, vec3.z());

        assert_eq!(1.0, vec3.r());
        assert_eq!(2.0, vec3.g());
        assert_eq!(3.0, vec3.b());

        assert_eq!(14.0, vec3.squared_length());
        assert_eq!(14.0, vec3.length().powi(2));

        vec3.make_unit_vector();
        assert_eq!(Vec3::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372732), vec3);
        assert_eq!(1.0, vec3.length());
        assert_eq!(1.0, vec3.squared_length());
    }

    #[test]
    fn it_makes_new_negative_vec3() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(Vec3::new(-1.0, -2.0, -3.0), -vec3);
    }

    #[test]
    fn it_performs_add_operation_on_two_vec3s_and_returns_new_vec3() {
        let vec3_first = Vec3::new(1.0, 2.0, 3.0);
        let vec3_second = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(Vec3::new(5.0, 7.0, 9.0), &vec3_first + &vec3_second);
        assert_eq!(Vec3::new(5.0, 7.0, 9.0), vec3_first + vec3_second);

        // make sure we haven't changed the original vec3s
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), vec3_first);
        assert_eq!(Vec3::new(4.0, 5.0, 6.0), vec3_second);
    }

    #[test]
    fn it_can_use_indices_and_assign_them() {
        let mut vec3 = Vec3::new(1.0, 2.0, 3.0);

        // check reference indices
        assert_eq!(&1.0, &vec3[0]);
        assert_eq!(&2.0, &vec3[1]);
        assert_eq!(&3.0, &vec3[2]);

        vec3[0] = 4.0;
        vec3[1] = 5.0;
        vec3[2] = 6.0;

        assert_eq!(&4.0, &vec3[0]);
        assert_eq!(&5.0, &vec3[1]);
        assert_eq!(&6.0, &vec3[2]);
        // check moved indices with a reference
        {
            let reference = &mut vec3;
            reference[0] = 7.0;
            reference[1] = 8.0;
            reference[2] = 9.0;

            assert_eq!(7.0, reference[0]);
            assert_eq!(8.0, reference[1]);
            assert_eq!(9.0, reference[2]);
        }

        assert_eq!(7.0, vec3[0]);
        assert_eq!(8.0, vec3[1]);
        assert_eq!(9.0, vec3[2]);
    }

    #[test]
    fn it_performs_sub_operation_on_two_vec3s_and_returns_new_vec3() {
        let vec3_first = Vec3::new(3.0, 5.0, 7.0);
        let vec3_second = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(Vec3::new(1.0, 2.0, 3.0), &vec3_first - &vec3_second);
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), vec3_first - vec3_second);

        // make sure we haven't changed the original vec3s
        assert_eq!(Vec3::new(3.0, 5.0, 7.0), vec3_first);
        assert_eq!(Vec3::new(2.0, 3.0, 4.0), vec3_second);
    }

    #[test]
    fn it_performs_add_assign_operation_on_two_vec3s_and_returns_new_vec3() {
        let mut vec3_first = Vec3::new(1.0, 2.0, 3.0);
        let vec3_second = Vec3::new(4.0, 5.0, 6.0);
        let vec3_ref = Vec3::new(7.0, 8.0, 9.0);

        vec3_first += vec3_second;
        assert_eq!(Vec3::new(5.0, 7.0, 9.0), vec3_first);
        assert_eq!(Vec3::new(4.0, 5.0, 6.0), vec3_second);

        vec3_first += &vec3_ref;
        assert_eq!(Vec3::new(12.0, 15.0, 18.0), vec3_first);
        assert_eq!(Vec3::new(7.0, 8.0, 9.0), vec3_ref);
    }

    #[test]
    fn it_performs_sub_assign_operation_on_two_vec3s_and_returns_new_vec3() {
        let mut vec3_first = Vec3::new(7.0, 10.0, 11.0);
        let vec3_second = Vec3::new(1.0, 2.0, 4.0);
        let vec3_ref = Vec3::new(4.0, 5.0, 6.0);

        vec3_first -= vec3_second;
        assert_eq!(Vec3::new(6.0, 8.0, 7.0), vec3_first);
        assert_eq!(Vec3::new(1.0, 2.0, 4.0), vec3_second);

        vec3_first -= &vec3_ref;
        assert_eq!(Vec3::new(2.0, 3.0, 1.0), vec3_first);
        assert_eq!(Vec3::new(4.0, 5.0, 6.0), vec3_ref);
    }

    #[test]
    fn it_performs_mul_operation_on_two_vec3s_and_returns_new_vec3() {
        let vec3_first = Vec3::new(2.0, 3.0, 4.0);
        let vec3_second = Vec3::new(5.0, 6.0, 7.0);

        assert_eq!(Vec3::new(10.0, 18.0, 28.0), &vec3_first * &vec3_second);
        assert_eq!(Vec3::new(10.0, 18.0, 28.0), vec3_first * vec3_second);

        // make sure we haven't changed the original vec3s
        assert_eq!(Vec3::new(2.0, 3.0, 4.0), vec3_first);
        assert_eq!(Vec3::new(5.0, 6.0, 7.0), vec3_second);
    }

    #[test]
    fn it_performs_div_operation_on_two_vec3s_and_returns_new_vec3() {
        let vec3_first = Vec3::new(10.0, 18.0, 28.0);
        let vec3_second = Vec3::new(5.0, 6.0, 7.0);

        assert_eq!(Vec3::new(2.0, 3.0, 4.0), &vec3_first / &vec3_second);
        assert_eq!(Vec3::new(2.0, 3.0, 4.0), vec3_first / vec3_second);

        // make sure we haven't changed the original vec3s
        assert_eq!(Vec3::new(10.0, 18.0, 28.0), vec3_first);
        assert_eq!(Vec3::new(5.0, 6.0, 7.0), vec3_second);
    }

    #[test]
    fn it_performs_mul_operation_on_vec3_with_f64_and_returns_new_vec3() {
        let vec3 = Vec3::new(2.0, 3.0, 4.0);
        let multiplier: f64 = 3.0;

        assert_eq!(Vec3::new(6.0, 9.0, 12.0), vec3 * multiplier);

        // make sure we haven't changed the original values
        assert_eq!(Vec3::new(2.0, 3.0, 4.0), vec3);
        assert_eq!(3.0, multiplier);
    }

    #[test]
    fn it_performs_div_operation_on_vec3_with_f64_and_returns_new_vec3() {
        let vec3 = Vec3::new(12.0, 4.0, 8.0);
        let divider: f64 = 4.0;

        assert_eq!(Vec3::new(3.0, 1.0, 2.0), vec3 / divider);

        // make sure we haven't changed the original values
        assert_eq!(Vec3::new(12.0, 4.0, 8.0), vec3);
        assert_eq!(4.0, divider);
    }

    #[test]
    fn it_performs_dot_on_two_vec3s() {
        let vec3_left = Vec3::new(2.0, 4.0, 6.0);
        let vec3_right = Vec3::new(3.0, 5.0, 7.0);

        assert_eq!(68.0, Vec3::dot(&vec3_left, &vec3_right));
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), vec3_left);
        assert_eq!(Vec3::new(3.0, 5.0, 7.0), vec3_right);
    }

    #[test]
    fn it_performs_crosses_two_vec3s_and_returns_new_vec3() {
        let vec3_left = Vec3::new(2.0, 4.0, 6.0);
        let vec3_right = Vec3::new(3.0, 5.0, 7.0);

        assert_eq!(Vec3::new(-2.0, 4.0, -18.0), Vec3::cross(&vec3_left, &vec3_right));
        assert_eq!(Vec3::new(2.0, 4.0, 6.0), vec3_left);
        assert_eq!(Vec3::new(3.0, 5.0, 7.0), vec3_right);
    }
}
