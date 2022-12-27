use crate::{Ray, Shape};

use super::{
    intersection::{ComputedIntersection, Intersection},
    IntersectionsFactor,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Intersections<'a> {
    data: Vec<ComputedIntersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn new(
        mut roots: IntersectionsFactor,
        object: &'a Shape,
        ray: &Ray,
    ) -> Intersections<'a> {
        roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let data = roots
            .iter()
            .filter_map(|&t| Intersection::new(t, object).prepare_computations(ray))
            .collect();
        Intersections { data }
    }

    pub(crate) fn get(&self, index: usize) -> Option<&ComputedIntersection> {
        self.data.get(index)
    }

    pub(crate) fn get_mut(&'a mut self, index: usize) -> Option<&mut ComputedIntersection> {
        self.data.get_mut(index)
    }

    pub fn hit(&self) -> Option<&ComputedIntersection> {
        let non_negative_index = self.data.partition_point(|i| i.t() < 0.0);
        self.get(non_negative_index)
    }

    pub(crate) fn merge(mut self, mut other: Intersections<'a>) -> Intersections {
        // use merge like merge sort but push backward
        let mut merged_data = Vec::with_capacity(self.data.len() + other.data.len());

        while !self.data.is_empty() && !other.data.is_empty() {
            let t1 = self.data.last().unwrap().t();
            let t2 = other.data.last().unwrap().t();
            if t1 > t2 {
                merged_data.push(self.data.pop().unwrap());
            } else {
                merged_data.push(other.data.pop().unwrap());
            }
        }

        while !self.data.is_empty() {
            merged_data.push(self.data.pop().unwrap());
        }

        while !other.data.is_empty() {
            merged_data.push(other.data.pop().unwrap());
        }

        merged_data.reverse();

        self.data = merged_data;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{util::assert_float_eq, Point, Sphere, Vector};

    use super::*;

    fn ray() -> Ray {
        Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0))
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::shape();
        let r = ray();
        let i1 = Intersections::new(vec![1.0], &s, &r);
        let i2 = Intersections::new(vec![2.0], &s, &r);
        let xs = i1.merge(i2);
        assert_eq!(xs.count(), 2);
        assert_float_eq!(xs.get(0).unwrap().t(), 1.0);
        assert_float_eq!(xs.get(1).unwrap().t(), 2.0);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::shape();
        let r = ray();
        let i1 = Intersections::new(vec![1.0], &s, &r);
        let i2 = Intersections::new(vec![2.0], &s, &r);
        let xs = i1.clone().merge(i2);
        let i = xs.hit();
        assert_eq!(i, i1.get(0));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::shape();
        let r = ray();
        let i1 = Intersections::new(vec![-1.0], &s, &r);
        let i2 = Intersections::new(vec![2.0], &s, &r);
        let xs = i1.merge(i2.clone());
        let i = xs.hit();
        assert_eq!(i, i2.get(0));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::shape();
        let r = ray();
        let i1 = Intersections::new(vec![-1.0], &s, &r);
        let i2 = Intersections::new(vec![-2.0], &s, &r);
        let xs = i1.merge(i2);
        assert!(xs.hit().is_none());
    }

    #[test]
    fn hit_is_lowest_nonnegative_intersection() {
        let s = Sphere::shape();
        let r = ray();
        let i1 = Intersections::new(vec![5.0], &s, &r);
        let i2 = Intersections::new(vec![7.0], &s, &r);
        let i3 = Intersections::new(vec![-3.0], &s, &r);
        let i4 = Intersections::new(vec![2.0], &s, &r);
        let xs = i1.merge(i2).merge(i3).merge(i4.clone());
        let i = xs.hit();
        assert_eq!(i, i4.get(0));
    }
}
