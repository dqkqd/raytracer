use crate::{
    transform::Transformable, Color, Intersections, IntersectionsFactor, Material, Point, Ray,
    Vector,
};

pub trait ObjectMaterial {
    fn material(&self) -> &Material;
    fn with_color(self, color: Color) -> Self;
    fn with_material(self, material: Material) -> Self;
}

pub trait ObjectLocal {
    fn local_normal_at(&self, local_point: &Point) -> Vector;
    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor;
}

pub trait ObjectWorld: Transformable + ObjectLocal {
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
