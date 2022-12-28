use crate::{shapes::ShapeMaterial, Ray, Shape};

use super::{
    intersection::{ComputedIntersection, Intersection, DEFAULT_REFRACTIVE_INDEX},
    IntersectionsFactor,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Intersections<'a> {
    data: Vec<ComputedIntersection<'a>>,
}

#[allow(dead_code)]
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

    pub(crate) fn update_refractive_index(mut self) -> Self {
        let mut container: Vec<&Shape> = Vec::with_capacity(self.count());
        for comp in self.data.iter_mut() {
            if container.is_empty() {
                let exit_index = DEFAULT_REFRACTIVE_INDEX;
                let enter_index = comp.object().material().refractive_index();
                comp.set_n1(exit_index);
                comp.set_n2(enter_index);
                container.push(comp.object());
            } else {
                let presented = container.iter().position(|&object| object == comp.object());
                if let Some(index) = presented {
                    let exit_index = container.last().unwrap().material().refractive_index();
                    container.remove(index);
                    let enter_index = match container.last() {
                        Some(&object) => object.material().refractive_index(),
                        None => DEFAULT_REFRACTIVE_INDEX,
                    };
                    comp.set_n1(exit_index);
                    comp.set_n2(enter_index);
                } else {
                    let exit_index = match container.last() {
                        Some(&object) => object.material().refractive_index(),
                        None => DEFAULT_REFRACTIVE_INDEX,
                    };
                    let enter_index = comp.object().material().refractive_index();
                    comp.set_n1(exit_index);
                    comp.set_n2(enter_index);
                    container.push(comp.object());
                }
            }
        }

        self
    }
}

#[cfg(test)]
mod test {
    use crate::{
        shapes::ShapeMaterial, util::assert_float_eq, Point, Sphere, Transform, Transformable,
        Vector, World,
    };

    use super::*;

    fn ray() -> Ray {
        Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0))
    }

    fn glassy_sphere() -> Shape {
        Sphere::shape()
            .with_transparency(1.0)
            .with_refractive_index(1.5)
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

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = glassy_sphere()
            .with_refractive_index(1.5)
            .with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let b = glassy_sphere()
            .with_refractive_index(2.0)
            .with_transform(Transform::translation(0.0, 0.0, -0.25));
        let c = glassy_sphere()
            .with_refractive_index(2.5)
            .with_transform(Transform::translation(0.0, 0.0, 0.25));

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));

        let w = World::new(vec![], vec![a, b, c]);
        let xs = w.intersect(&r);

        let expected_n1_n2 = [
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for (index, i) in xs.data.iter().enumerate() {
            let (n1, n2) = expected_n1_n2[index];
            assert_float_eq!(n1, i.n1().unwrap());
            assert_float_eq!(n2, i.n2().unwrap());
        }
    }
}
