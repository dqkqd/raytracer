use crate::{
    intersection::IntersectFactor, transform::Transformable, Intersections, Ray, Sphere, Transform,
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
