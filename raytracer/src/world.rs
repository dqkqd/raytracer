use crate::{
    intersect::intersection::ComputedIntersection,
    object::{ObjectMaterial, ObjectWorld},
    Color, Intersections, PointLight, Ray, Shape,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct World {
    light_source: Option<PointLight>,
    objects: Vec<Shape>,
}

impl World {
    pub fn new(light_source: PointLight, objects: Vec<Shape>) -> World {
        World {
            light_source: Some(light_source),
            objects,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        self.objects
            .iter()
            .map(|object| object.intersect(ray))
            .reduce(|merged_intersections, intersections| merged_intersections.merge(intersections))
            .unwrap_or_default()
    }

    pub fn shade_hit(&self, comp: &ComputedIntersection) -> Option<Color> {
        Some(self.light_source?.lighting(
            comp.object().material(),
            comp.point(),
            comp.eye_vector(),
            comp.normal_vector(),
        ))
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        self.intersect(ray)
            .hit()
            .map(|hit| self.shade_hit(hit).unwrap_or_default())
            .unwrap_or_default()
    }
}
#[cfg(test)]
mod test {

    use crate::{
        color, intersect::intersection::Intersection, transform::Transformable, Material, Point,
        Sphere, Transform, Vector,
    };

    use super::*;

    fn default_world() -> World {
        let point_light = PointLight::new(Point::new(-10.0, 10.0, -10.0), color::WHITE);
        let s1 = Sphere::shape().with_material(
            Material::default()
                .with_color(Color::new(0.8, 1.0, 0.6))
                .with_diffuse(0.7)
                .with_specular(0.2),
        );
        let s2 = Sphere::shape().with_transform(Transform::scaling(0.5, 0.5, 0.5));

        World {
            light_source: Some(point_light),
            objects: vec![s1, s2],
        }
    }

    #[test]
    fn create_world() {
        let w = World::default();
        assert!(w.light_source.is_none());
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn intersect_word_with_a_ray() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_eq!(xs.count(), 4);
        assert_eq!(xs.get(0).unwrap().t(), 4.0);
        assert_eq!(xs.get(1).unwrap().t(), 4.5);
        assert_eq!(xs.get(2).unwrap().t(), 5.5);
        assert_eq!(xs.get(3).unwrap().t(), 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = w.objects[0];
        let comp = Intersection::new(4.0, &s).prepare_computations(&r).unwrap();
        let c = w.shade_hit(&comp).unwrap();
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.light_source = Some(PointLight::new(Point::new(0.0, 0.25, 0.0), color::WHITE));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = w.objects[1];
        let comp = Intersection::new(0.5, &s).prepare_computations(&r).unwrap();
        let c = w.shade_hit(&comp).unwrap();
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, color::BLACK)
    }

    #[test]
    fn color_when_a_ray_hit() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.475833, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_the_ray() {
        let mut w = default_world();
        w.objects[0] = w.objects[0].with_ambient(1.0);
        w.objects[1] = w.objects[1].with_ambient(1.0);
        let inner = w.objects[1];
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, inner.material().color());
    }
}
