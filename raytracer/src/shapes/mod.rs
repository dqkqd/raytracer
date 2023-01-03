pub(super) mod dummy;

pub(crate) mod shape;

pub(crate) mod sphere;

pub(crate) mod plane;

pub(crate) mod cube;

pub(crate) mod cylinder;

pub(crate) mod cone;

use crate::{
    color::Color,
    intersect::{intersection::IntersectionsFactor, multiple_intersections::Intersections},
    material::Material,
    patterns::pattern::Pattern,
    point::Point,
    ray::Ray,
    transform::Transformable,
    vector::Vector,
};

use self::{
    cone::Cone, cube::Cube, cylinder::Cylinder, dummy::Dummy, plane::Plane, sphere::Sphere,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ShapeKind {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Dummy(Dummy),
    Cylinder(Cylinder),
    Cone(Cone),
}

pub(crate) trait ShapeMaterial {
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

pub(crate) trait ShapeLocal {
    fn local_normal_at(&self, local_point: &Point) -> Vector;
    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor;
}

pub(crate) trait ShapeWorld: Transformable + ShapeLocal {
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
