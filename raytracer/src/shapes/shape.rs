use crate::{
    transform::{transformable, InversedTransform},
    Color, Intersections, IntersectionsFactor, Material, Point, Ray, Transform, Vector,
};

use super::{
    dummy_shape::TestShape,
    object::{ObjectLocal, ObjectMaterial, ObjectWorld},
    plane::Plane,
    sphere::Sphere,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeKind {
    Sphere(Sphere),
    Plane(Plane),
    TestShape(TestShape),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    shape: ShapeKind,
    inversed_transform: InversedTransform,
    material: Material,
}

transformable!(Shape);

impl Shape {
    pub fn new(shape: ShapeKind) -> Shape {
        Shape {
            shape,
            inversed_transform: Some(Transform::identity()),
            material: Material::default(),
        }
    }
}

impl ObjectWorld for Shape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        self.transform_ray(ray)
            .map_or(Default::default(), |local_ray| {
                let roots = self.local_intersection(&local_ray);
                Intersections::new(roots, self, ray)
            })
    }
}

impl ObjectLocal for Shape {
    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor {
        match self.shape {
            ShapeKind::Sphere(s) => s.local_intersection(local_ray),
            ShapeKind::TestShape(s) => s.local_intersection(local_ray),
            ShapeKind::Plane(p) => p.local_intersection(local_ray),
        }
    }

    fn local_normal_at(&self, object_point: &Point) -> Vector {
        match self.shape {
            ShapeKind::Sphere(s) => s.local_normal_at(object_point),
            ShapeKind::TestShape(s) => s.local_normal_at(object_point),
            ShapeKind::Plane(p) => p.local_normal_at(object_point),
        }
    }
}

impl ObjectMaterial for Shape {
    fn material(&self) -> &Material {
        &self.material
    }

    fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    fn with_color(self, color: Color) -> Self {
        self.with_material(self.material.with_color(color))
    }

    fn with_ambient(self, ambient: f64) -> Self {
        self.with_material(self.material.with_ambient(ambient))
    }

    fn with_diffuse(self, diffuse: f64) -> Self {
        self.with_material(self.material.with_diffuse(diffuse))
    }

    fn with_specular(self, specular: f64) -> Self {
        self.with_material(self.material.with_specular(specular))
    }

    fn with_shininess(self, shininess: f64) -> Self {
        self.with_material(self.material.with_shininess(shininess))
    }
}

#[cfg(test)]
mod test {
    use crate::{util::assert_float_eq, Transformable};

    use super::*;

    #[test]
    fn shape_default_transformation() {
        let s = TestShape::shape();
        assert_eq!(s.inversed_transform(), Some(Transform::identity()));
    }

    #[test]
    fn default_shape_has_default_material() {
        let s = TestShape::shape();
        let m = s.material;
        assert_eq!(m, Material::default());
    }

    #[test]
    fn assigning_a_transformation() {
        let t = Transform::translation(2.0, 3.0, 4.0);
        let s = TestShape::shape().with_transform(t);
        assert_eq!(s.inversed_transform(), t.inverse());
    }

    #[test]
    fn default_material() {
        let s = TestShape::shape();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn assigning_a_material() {
        let m = Material::default().with_ambient(1.0);
        let s = TestShape::shape().with_material(m);
        assert_eq!(s.material, m);
    }

    #[test]
    fn shape_with_color() {
        let color = Color::new(0.1, 0.1, 0.1);
        let s = TestShape::shape().with_color(color);
        assert_eq!(s.material.color(), color);
    }

    #[test]
    fn shape_with_ambient() {
        let ambient = 0.6;
        let s = TestShape::shape().with_ambient(ambient);
        assert_float_eq!(s.material.ambient(), ambient);
    }

    #[test]
    fn shape_with_diffuse() {
        let diffuse = 0.2;
        let s = TestShape::shape().with_diffuse(diffuse);
        assert_float_eq!(s.material.diffuse(), diffuse);
    }

    #[test]
    fn shape_with_specular() {
        let specular = 0.6;
        let s = TestShape::shape().with_specular(specular);
        assert_float_eq!(s.material.specular(), specular);
    }

    #[test]
    fn shape_with_shininess() {
        let shininess = 1.5;
        let s = TestShape::shape().with_shininess(shininess);
        assert_float_eq!(s.material.shininess(), shininess);
    }

    #[test]
    fn intersections_contain_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = TestShape::shape().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let i = s.intersect(&r);
        assert_eq!(i.get(0).unwrap().object(), &s);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = TestShape::shape().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let local_ray = s.transform_ray(&r).unwrap();
        assert_eq!(local_ray.origin(), Point::new(0.0, 0.0, -2.5));
        assert_eq!(local_ray.direction(), Vector::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::shape().with_transform(Transform::translation(5.0, 0.0, 0.0));
        let local_ray = s.transform_ray(&r).unwrap();
        assert_eq!(local_ray.origin(), Point::new(-5.0, 0.0, -5.0));
        assert_eq!(local_ray.direction(), Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_translated_shape() {
        let s = TestShape::shape().with_transform(Transform::translation(0.0, 1.0, 0.0));
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
    fn normal_on_transformed_shape() {
        let m = Transform::rotation_z(std::f64::consts::PI / 5.0).scale(1.0, 0.5, 1.0);
        let s = TestShape::shape().with_transform(m);
        let n = s.normal_at(&Point::new(
            0.0,
            std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        ));

        assert_eq!(n.unwrap(), Vector::new(0.0, 0.97014, -0.24254));
    }
}
