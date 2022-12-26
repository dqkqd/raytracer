use crate::{
    intersection::IntersectionsFactor, transform::transformable, Color, Intersections, Material,
    Point, Ray, Sphere, Transform, Vector,
};

use super::object::{ObjectLocal, ObjectMaterial, ObjectWorld};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeKind {
    Sphere(Sphere),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    shape: ShapeKind,
    inversed_transform: Option<Transform>,
    material: Material,
}

transformable!(Shape);

impl Shape {
    pub fn new(shape: ShapeKind) -> Shape {
        Shape {
            shape,
            inversed_transform: Transform::identity().inverse(),
            material: Material::default(),
        }
    }
}

impl ObjectWorld for Shape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        self.transform_ray(ray)
            .map_or(Default::default(), |local_ray| {
                let mut roots = self.local_intersection(&local_ray);
                roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                Intersections::intersect(roots, self)
            })
    }
}

impl ObjectLocal for Shape {
    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor {
        match self.shape {
            ShapeKind::Sphere(sphere) => sphere.local_intersection(local_ray),
        }
    }

    fn local_normal_at(&self, object_point: &Point) -> Vector {
        match self.shape {
            ShapeKind::Sphere(sphere) => sphere.local_normal_at(object_point),
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
}
