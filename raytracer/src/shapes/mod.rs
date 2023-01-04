pub(super) mod dummy;

pub(crate) mod shape;

pub(crate) mod sphere;

pub(crate) mod plane;

pub(crate) mod cube;

pub(crate) mod cylinder;

pub(crate) mod cone;

pub(crate) mod group;

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
    cone::Cone, cube::Cube, cylinder::Cylinder, dummy::Dummy, group::Group, plane::Plane,
    sphere::Sphere,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ShapeKind {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Dummy(Dummy),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group),
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

    fn world_to_object(&self, point: &Point) -> Option<Point> {
        Some(point.transform(self.inversed_transform()?))
    }

    fn normal_to_world(&self, normal: &Vector) -> Option<Vector> {
        Some((self.transpose_inversed_transform()? * *normal).normalize())
    }

    fn normal_at(&self, point: &Point) -> Option<Vector> {
        let object_point = self.world_to_object(point)?;
        let local_normal = self.local_normal_at(&object_point);
        let world_normal = self.normal_to_world(&local_normal)?;
        Some(world_normal)
    }

    fn intersect(&self, ray: &Ray) -> Intersections;
}
