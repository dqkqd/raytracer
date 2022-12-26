use crate::{
    intersection::IntersectionsFactor, transform::Transformable, Intersections, Material, Point,
    Ray, Sphere, Transform, Vector,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn transform_ray(&self, ray: &Ray) -> Option<Ray> {
        Some(ray.transform(self.inversed_transform()?))
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        self.transform_ray(ray)
            .map_or(Default::default(), |transformed_ray| {
                let mut roots = match self {
                    Shape::Sphere(sphere) => sphere.local_intersection(&transformed_ray),
                };
                roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                Intersections::intersect(roots, self)
            })
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
