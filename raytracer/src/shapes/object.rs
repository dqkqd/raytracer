use crate::{
    intersection::IntersectionsFactor, transform::Transformable, Intersections, Material, Point,
    Ray, Vector,
};

pub trait ObjectMaterial {
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Material);
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
