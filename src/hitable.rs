use crate::ray::Ray;
use crate::Vec3;
use crate::material::Material;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord<'mat> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'mat Material,
}

impl<'mat> HitRecord<'mat> {
    pub fn new(t: f64, p: Vec3, normal: Vec3, material: &'mat Material) -> Self {
        Self { t, p, normal, material }
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
    struct HitDummy {}
    impl HitDummy {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Hitable for HitDummy {
        fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
            None
        }
    }

    #[test]
    fn hitable_list_iterates_over_hitables() {
        let dummy_one = HitDummy::new();
        let dummy_two = HitDummy::new();
        let slice = vec![dummy_one, dummy_two];
        let mut hitable_list = HitableList::new(&slice);

        assert_eq!(Some(&HitDummy::new()), hitable_list.next());
        assert_eq!(Some(&HitDummy::new()), hitable_list.next());
        assert_eq!(None, hitable_list.next());
    }

    #[test]
    fn hitable_list_from_a_vec() {
        let vector = vec![HitDummy::new(), HitDummy::new()];

        let hitable_list = HitableList::from(&vector);

        assert_eq!(2, hitable_list.count());
    }
}
