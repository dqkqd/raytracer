use crate::{
    intersection::IntersectFactor, transform::transformable, util::solve_quadratic_equation,
    Material, Point, Ray, Transform, Vector,
};

use super::shape::Shape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    origin: Point,
    radius: f64,
    inversed_transform: Option<Transform>,
    material: Material,
}

transformable!(Sphere);

impl Sphere {
    pub fn shape() -> Shape {
        let origin = Point::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        Shape::Sphere(Sphere {
            origin,
            radius,
            inversed_transform: Transform::identity().inverse(),
            material: Material::default(),
        })
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material
    }

    pub fn local_normal_at(&self, point: &Point) -> Vector {
        (*point - self.origin).normalize()
    }

    pub fn intersect_factor(&self, ray: &Ray) -> IntersectFactor {
        let sphere_to_ray = ray.origin() - self.origin;
        let a = ray.direction().dot(&ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        solve_quadratic_equation(a, b, c)
    }
}

#[cfg(test)]
mod test {

    use crate::{transform::Transformable, Vector};

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections.get(0).unwrap().t(), 4.0);
        assert_eq!(intersections.get(1).unwrap().t(), 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 1);
        assert_eq!(intersections.get(0).unwrap().t(), 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 0);
    }

    #[test]
    fn ray_originate_inside_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections.get(0).unwrap().t(), -1.0);
        assert_eq!(intersections.get(1).unwrap().t(), 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::shape();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.count(), 2);
        assert_eq!(intersections.get(0).unwrap().t(), -6.0);
        assert_eq!(intersections.get(1).unwrap().t(), -4.0);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::shape();
        assert_eq!(s.inversed_transform(), Transform::identity().inverse());
    }

    #[test]
    fn changing_sphere_transformation() {
        let t = Transform::translation(2.0, 3.0, 4.0);
        let s = Sphere::shape().with_transform(t);
        assert_eq!(s.inversed_transform(), t.inverse());
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::shape().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 3.0);
        assert_eq!(xs.get(1).unwrap().t(), 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::shape().with_transform(Transform::scaling(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Sphere::shape();
        let n = s.normal_at(&Point::new(1.0, 0.0, 0.0));
        assert_eq!(n.unwrap(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Sphere::shape();
        let n = s.normal_at(&Point::new(0.0, 1.0, 0.0));
        assert_eq!(n.unwrap(), Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Sphere::shape();
        let n = s.normal_at(&Point::new(0.0, 0.0, 1.0));
        assert_eq!(n.unwrap(), Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_at_nonaxial_point() {
        let s = Sphere::shape();
        let v = f64::sqrt(3.0) / 3.0;
        let n = s.normal_at(&Point::new(v, v, v));
        assert_eq!(n.unwrap(), Vector::new(v, v, v));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::shape();
        let v = f64::sqrt(3.0);
        let n = s.normal_at(&Point::new(v, v, v)).unwrap();
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = Sphere::shape().with_transform(Transform::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Point::new(
            0.0,
            1.0 + std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        ));

        assert_eq!(
            n.unwrap(),
            Vector::new(
                0.0,
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2
            )
        );
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let m = Transform::rotation_z(std::f64::consts::PI / 5.0).scale(1.0, 0.5, 1.0);
        let s = Sphere::shape().with_transform(m);
        let n = s.normal_at(&Point::new(
            0.0,
            std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        ));

        assert_eq!(n.unwrap(), Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn default_sphere_has_default_material() {
        let s = Sphere::shape();
        let m = s.material();
        assert_eq!(m, &Material::default());
    }

    #[test]
    fn sphere_can_assign_material() {
        let mut s = Sphere::shape();
        let m = Material::default().with_ambient(1.0);
        s.set_material(m);
        assert_eq!(s.material(), &m);
    }
}
