use crate::{object::ObjectWorld, Point, Ray, Shape, Vector};

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

    pub(crate) fn prepare_computations(self, ray: &Ray) -> Option<ComputedIntersection<'a>> {
        let t = self.t;
        let object = self.object;

        let point = ray.position(t);
        let eye_vector = -ray.direction();
        let mut normal_vector = object.normal_at(&point)?;

        let inside = match normal_vector.dot(&eye_vector) < 0.0 {
            false => false,
            true => {
                normal_vector = -normal_vector;
                true
            }
        };

        Some(ComputedIntersection {
            t,
            object,
            point,
            eye_vector,
            normal_vector,
            inside,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputedIntersection<'a> {
    object: &'a Shape,
    t: f64,
    point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
    inside: bool,
}

impl<'a> ComputedIntersection<'a> {
    pub fn t(&self) -> f64 {
        self.t
    }

    pub(crate) fn object(&self) -> &Shape {
        self.object
    }

    pub(crate) fn point(&self) -> &Point {
        &self.point
    }

    pub(crate) fn eye_vector(&self) -> &Vector {
        &self.eye_vector
    }

    pub(crate) fn normal_vector(&self) -> &Vector {
        &self.normal_vector
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
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
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

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::shape();
        let comp = Intersection::new(4.0, &s).prepare_computations(&r).unwrap();
        assert!(!comp.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::shape();
        let comp = Intersection::new(1.0, &s).prepare_computations(&r).unwrap();
        assert_eq!(comp.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comp.eye_vector, Vector::new(0.0, 0.0, -1.0));
        assert!(comp.inside);
        assert_eq!(comp.normal_vector, Vector::new(0.0, 0.0, -1.0));
    }
}
