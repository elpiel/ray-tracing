use crate::ray::Ray;
use crate::Vec3;

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> Self {
        Self { t, p, normal }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList<'a, H: Hitable> {
    hitable: &'a [H],
    index: usize,
}

impl<'a, H: Hitable> HitableList<'a, H> {
    pub fn new(hitable: &'a [H]) -> Self {
        Self { hitable, index: 0 }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.hitable.iter().fold(None, |closest, hitable| {
            let closest_so_far = closest.as_ref().map_or(t_max, |hr| hr.t);

            hitable.hit(ray, t_min, closest_so_far).or(closest)
        })
    }
}

impl<'a, H: Hitable> Iterator for HitableList<'a, H> {
    type Item = &'a H;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.hitable.get(self.index);
        self.index += 1;

        current
    }
}

impl<'a, H: Hitable> From<&'a Vec<H>> for HitableList<'a, H> {
    fn from(vec: &'a Vec<H>) -> HitableList<'a, H> {
        HitableList::new(vec)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct HitTrueDummy {}
    impl HitTrueDummy {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Hitable for HitTrueDummy {
        fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
            None
        }
    }

    #[test]
    fn hitable_list_iterates_over_hitables() {
        let dummy_one = HitTrueDummy::new();
        let dummy_two = HitTrueDummy::new();
        let slice = vec![dummy_one, dummy_two];
        let mut hitable_list = HitableList::new(&slice);

        assert_eq!(Some(&HitTrueDummy::new()), hitable_list.next());
        assert_eq!(Some(&HitTrueDummy::new()), hitable_list.next());
        assert_eq!(None, hitable_list.next());
    }

    fn hitable_list_from_a_vec_reference() {
        // TODO: Implement this test
    }
}
