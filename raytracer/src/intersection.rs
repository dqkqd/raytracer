use crate::{Ray, Shape};

pub(crate) type IntersectionsFactor = Vec<f64>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    object: &'a Shape,
    t: f64,
}

impl<'a> Intersection<'a> {
    pub(crate) fn new(t: f64, object: &Shape) -> Intersection {
        Intersection { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub(crate) fn object(&self) -> &Shape {
        self.object
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Intersections<'a> {
    data: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn intersect(roots: IntersectionsFactor, object: &'a Shape) -> Intersections<'a> {
        let data = roots
            .iter()
            .map(|&t| Intersection::new(t, object))
            .collect();
        Intersections { data }
    }

    pub(crate) fn from_intersection(data: Vec<Intersection>) -> Intersections {
        let mut sorted_data = data;
        sorted_data.sort_unstable_by(|a, b| {
            let x = a.t();
            let y = b.t();
            x.partial_cmp(&y).unwrap()
        });
        Intersections { data: sorted_data }
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Sphere;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::shape();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t(), 3.5);
        assert_eq!(i.object(), &s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::shape();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::from_intersection(vec![i1, i2]);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 1.0);
        assert_eq!(xs.get(1).unwrap().t(), 2.0);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Sphere::shape();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::from_intersection(vec![i1, i2]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i1);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Sphere::shape();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::from_intersection(vec![i1, i2]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i2);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Sphere::shape();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(-2.0, &s);
        let xs = Intersections::from_intersection(vec![i1, i2]);
        assert!(xs.hit().is_none());
    }

    #[test]
    fn hit_is_lowest_nonnegative_intersection() {
        let s = Sphere::shape();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::from_intersection(vec![i1, i2, i3, i4]);
        let i = xs.hit().unwrap();
        assert_eq!(*i, i4);
    }
}
