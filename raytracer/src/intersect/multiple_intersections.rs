use crate::Shape;

use super::{intersection::Intersection, IntersectionsFactor};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Intersections<'a> {
    data: Vec<Intersection<'a>>,
}

#[allow(dead_code)]
impl<'a> Intersections<'a> {
    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn new(mut roots: IntersectionsFactor, object: &'a Shape) -> Intersections<'a> {
        roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let data = roots
            .iter()
            .map(|&t| Intersection::new(t, object))
            .collect();
        Intersections { data }
    }

    pub(crate) fn get(&self, index: usize) -> Option<&Intersection> {
        self.data.get(index)
    }

    pub(crate) fn get_mut(&'a mut self, index: usize) -> Option<&mut Intersection> {
        self.data.get_mut(index)
    }

    pub fn hit(&self) -> Option<&Intersection> {
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
    use crate::Sphere;

    use super::*;

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::shape();
        let i1 = Intersections::new(vec![1.0], &s);
        let i2 = Intersections::new(vec![2.0], &s);
        let xs = i1.merge(i2);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 1.0);
        assert_eq!(xs.get(1).unwrap().t(), 2.0);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::shape();
        let i1 = Intersections::new(vec![1.0], &s);
        let i2 = Intersections::new(vec![2.0], &s);
        let xs = i1.clone().merge(i2);
        let i = xs.hit();
        assert_eq!(i, i1.get(0));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::shape();
        let i1 = Intersections::new(vec![-1.0], &s);
        let i2 = Intersections::new(vec![2.0], &s);
        let xs = i1.merge(i2.clone());
        let i = xs.hit();
        assert_eq!(i, i2.get(0));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::shape();
        let i1 = Intersections::new(vec![-1.0], &s);
        let i2 = Intersections::new(vec![-2.0], &s);
        let xs = i1.merge(i2);
        assert!(xs.hit().is_none());
    }

    #[test]
    fn hit_is_lowest_nonnegative_intersection() {
        let s = Sphere::shape();
        let i1 = Intersections::new(vec![5.0], &s);
        let i2 = Intersections::new(vec![7.0], &s);
        let i3 = Intersections::new(vec![-3.0], &s);
        let i4 = Intersections::new(vec![2.0], &s);
        let xs = i1.merge(i2).merge(i3).merge(i4.clone());
        let i = xs.hit();
        assert_eq!(i, i4.get(0));
    }
}
