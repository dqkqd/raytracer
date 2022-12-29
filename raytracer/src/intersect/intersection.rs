use crate::{shapes::ShapeWorld, Point, Ray, Shape, Vector};

pub(crate) type IntersectionsFactor = Vec<f64>;

const OFFSET_FACTOR: f64 = 1E-10;
pub(crate) const DEFAULT_REFRACTIVE_INDEX: f64 = 1.0;

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

        let over_point = point + normal_vector * OFFSET_FACTOR;
        let under_point = point + normal_vector * (-OFFSET_FACTOR);
        let reflect_vector = ray.direction().reflect(&normal_vector);

        Some(ComputedIntersection {
            t,
            object,
            point,
            over_point,
            under_point,
            eye_vector,
            normal_vector,
            reflect_vector,
            inside,
            n1: None,
            n2: None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputedIntersection<'a> {
    object: &'a Shape,
    t: f64,
    point: Point,
    over_point: Point,
    under_point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
    reflect_vector: Vector,
    n1: Option<f64>,
    n2: Option<f64>,
    inside: bool,
}

impl<'a> ComputedIntersection<'a> {
    pub fn t(&self) -> f64 {
        self.t
    }

    pub(crate) fn object(&self) -> &Shape {
        self.object
    }

    pub(crate) fn eye_vector(&self) -> &Vector {
        &self.eye_vector
    }

    pub(crate) fn normal_vector(&self) -> &Vector {
        &self.normal_vector
    }

    pub(crate) fn reflect_vector(&self) -> &Vector {
        &self.reflect_vector
    }

    pub(crate) fn over_point(&self) -> &Point {
        &self.over_point
    }

    pub(crate) fn under_point(&self) -> &Point {
        &self.under_point
    }

    pub(crate) fn n1(&self) -> Option<f64> {
        self.n1
    }

    pub(crate) fn set_n1(&mut self, n1: f64) {
        self.n1 = Some(n1);
    }

    pub(crate) fn n2(&self) -> Option<f64> {
        self.n2
    }

    pub(crate) fn set_n2(&mut self, n2: f64) {
        self.n2 = Some(n2);
    }

    pub(crate) fn schlick(&self) -> f64 {
        let n1 = self
            .n1
            .expect("`schlick` should only be called after n1 calculated");
        let n2 = self
            .n2
            .expect("`schlick` should only be called after n2 calculated");

        let mut cos = self.eye_vector.dot(&self.normal_vector);

        if n1 > n2 {
            let n = n1 / n2;
            let sin2_t = n * n * (1.0 - cos * cos);
            if sin2_t > 1.0 {
                return 1.0;
            }
            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = (n1 - n2) / (n1 + n2);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{util::assert_float_eq, Transform, Transformable};

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Shape::sphere();
        let i = Intersection::new(3.5, &s);
        assert_float_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::sphere();
        let i = Intersection::new(4.0, &s);
        let comp = i.prepare_computations(&r).unwrap();
        assert_float_eq!(comp.t, i.t);
        assert_eq!(comp.object, i.object);
        assert_eq!(comp.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comp.eye_vector, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comp.normal_vector, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::sphere();
        let comp = Intersection::new(4.0, &s).prepare_computations(&r).unwrap();
        assert!(!comp.inside);
    }

    #[test]
    fn hit_when_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::sphere();
        let comp = Intersection::new(1.0, &s).prepare_computations(&r).unwrap();
        assert_eq!(comp.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comp.eye_vector, Vector::new(0.0, 0.0, -1.0));
        assert!(comp.inside);
        assert_eq!(comp.normal_vector, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::sphere().with_transform(Transform::translation(0.0, 0.0, 1.0));
        let comp = Intersection::new(5.0, &s).prepare_computations(&r).unwrap();
        assert!(comp.over_point.z() < -OFFSET_FACTOR / 2.0);
        assert!(comp.point.z() > comp.over_point.z());
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::sphere().with_transform(Transform::translation(0.0, 0.0, 1.0));
        let comp = Intersection::new(5.0, &s).prepare_computations(&r).unwrap();
        assert!(comp.under_point.z() > OFFSET_FACTOR / 2.0);
        assert!(comp.point.z() < comp.under_point.z());
    }

    #[test]
    fn precomputing_reflection_vector() {
        let shape = Shape::plane();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(
                0.0,
                -std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
            ),
        );
        let comp = Intersection::new(2.0, &shape)
            .prepare_computations(&r)
            .unwrap();
        assert_eq!(
            comp.reflect_vector,
            Vector::new(
                0.0,
                std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2
            )
        );
    }
}
