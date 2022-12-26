use crate::{
    intersection::IntersectFactor, transform::Transformable, Intersections, Material, Point, Ray,
    Sphere, Transform, Vector,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn intersect_factor(&self, ray: &Ray) -> IntersectFactor {
        match self {
            Shape::Sphere(sphere) => sphere.intersect_factor(ray),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        Intersections::intersect(self, ray)
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.set_transform(transform);
        self
    }

    pub fn normal_at(&self, point: &Point) -> Option<Vector> {
        let object_point = point.transform(self.inversed_transform()?);
        let local_normal = match self {
            Shape::Sphere(sphere) => sphere.local_normal_at(&object_point),
        };
        let world_normal = self.inversed_transform()?.tranpose() * local_normal;
        Some(world_normal.normalize())
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(sphere) => sphere.material(),
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(sphere) => sphere.set_material(material),
        }
    }
}

impl Transformable for Shape {
    fn inversed_transform(&self) -> Option<Transform> {
        match self {
            Shape::Sphere(sphere) => sphere.inversed_transform(),
        }
    }

    fn set_transform(&mut self, transform: Transform) {
        match self {
            Shape::Sphere(sphere) => sphere.set_transform(transform),
        }
    }
}
