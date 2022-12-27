use crate::{object::ObjectWorld, Point, Ray, Shape, Vector};

pub(crate) type IntersectionsFactor = Vec<f64>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    object: &'a Shape,
    t: f64,
}

#[allow(dead_code)]
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

    pub(crate) fn prepare_computations(self, ray: &Ray) -> Option<ComputedIntersection<'a>> {
        let t = self.t;
        let object = self.object;
        let point = ray.position(t);
        let eye_vector = -ray.direction();
        let normal_vector = object.normal_at(&point)?;
        Some(ComputedIntersection {
            t,
            object,
            point,
            eye_vector,
            normal_vector,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ComputedIntersection<'a> {
    object: &'a Shape,
    t: f64,
    point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
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
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::shape();
        let i = Intersection::new(4.0, &s);
        let comps = i.prepare_computations(&r).unwrap();
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_vector, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_vector, Vector::new(0.0, 0.0, -1.0));
    }
}
