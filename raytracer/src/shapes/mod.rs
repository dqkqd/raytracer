use crate::{
    transform::Transformable, Color, Intersections, IntersectionsFactor, Material, Pattern, Point,
    Ray, Vector,
};

use self::{cube::Cube, dummy_shape::TestShape, plane::Plane, sphere::Sphere};

pub(super) mod dummy_shape;

pub(crate) mod shape;

pub mod sphere;

pub mod plane;

pub mod cube;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeKind {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    TestShape(TestShape),
}
pub trait ShapeMaterial {
    fn material(&self) -> &Material;
    fn with_material(self, material: Material) -> Self;

    fn with_color(self, color: Color) -> Self;
    fn with_ambient(self, ambient: f64) -> Self;
    fn with_diffuse(self, diffuse: f64) -> Self;
    fn with_specular(self, specular: f64) -> Self;
    fn with_shininess(self, shininess: f64) -> Self;
    fn with_reflective(self, reflective: f64) -> Self;
    fn with_transparency(self, transparency: f64) -> Self;
    fn with_refractive_index(self, refractive_index: f64) -> Self;

    fn with_pattern(self, pattern: Pattern) -> Self;
}

pub trait ShapeLocal {
    fn local_normal_at(&self, local_point: &Point) -> Vector;
    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor;
}

pub trait ShapeWorld: Transformable + ShapeLocal {
    fn transform_ray(&self, ray: &Ray) -> Option<Ray> {
        Some(ray.transform(self.inversed_transform()?))
    }

    fn normal_at(&self, point: &Point) -> Option<Vector> {
        let object_point = point.transform(self.inversed_transform()?);
        let local_normal = self.local_normal_at(&object_point);
        let world_normal = self.inversed_transform()?.tranpose() * local_normal;
        Some(world_normal.normalize())
    }

    fn intersect(&self, ray: &Ray) -> Intersections;
}
