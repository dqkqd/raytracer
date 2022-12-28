use crate::{
    intersect::intersection::ComputedIntersection,
    shapes::{ShapeMaterial, ShapeWorld},
    util::equal,
    Color, Intersections, Point, PointLight, Ray, Shape,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct World {
    lights: Vec<PointLight>,
    objects: Vec<Shape>,
}

impl World {
    pub fn new(lights: Vec<PointLight>, objects: Vec<Shape>) -> World {
        World { lights, objects }
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        self.objects
            .iter()
            .map(|object| object.intersect(ray))
            .reduce(|merged_intersections, intersections| merged_intersections.merge(intersections))
            .unwrap_or_default()
    }

    pub fn shade_hit(&self, comp: &ComputedIntersection) -> Color {
        self.lights
            .iter()
            .fold(Color::default(), |total_color, light| {
                let shadowed = self.is_shadowed(light, comp.over_point());

                let surface = light.lighting(
                    comp.object(),
                    comp.object().material(),
                    comp.over_point(),
                    comp.eye_vector(),
                    comp.normal_vector(),
                    shadowed,
                );

                let reflected_color = self.reflected_color(comp);

                total_color + surface + reflected_color
            })
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        self.intersect(ray)
            .hit()
            .map(|hit| self.shade_hit(hit))
            .unwrap_or_default()
    }

    fn is_shadowed(&self, light: &PointLight, point: &Point) -> bool {
        let v = light.position() - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*point, direction);
        let intersections = self.intersect(&r);
        intersections.hit().map_or(false, |hit| hit.t() < distance)
    }

    pub(crate) fn reflected_color(&self, comp: &ComputedIntersection) -> Color {
        if equal(comp.object().material().reflective(), 0.0) {
            return Color::default();
        }

        let reflect_ray = Ray::new(*comp.over_point(), *comp.reflect_vector());
        let color = self.color_at(&reflect_ray);
        color * comp.object().material().reflective()
    }
}

#[cfg(test)]
mod test {

    use crate::{
        color, intersect::intersection::Intersection, shapes::ShapeMaterial,
        transform::Transformable, util::assert_float_eq, Camera, Material, Plane, Sphere,
        Transform, Vector,
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

        World::new(vec![point_light], vec![s1, s2])
    }

    #[test]
    fn create_world() {
        let w = World::default();
        assert_eq!(w.lights.len(), 0);
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn intersect_word_with_a_ray() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);
        assert_eq!(xs.count(), 4);
        assert_float_eq!(xs.get(0).unwrap().t(), 4.0);
        assert_float_eq!(xs.get(1).unwrap().t(), 4.5);
        assert_float_eq!(xs.get(2).unwrap().t(), 5.5);
        assert_float_eq!(xs.get(3).unwrap().t(), 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = w.objects[0];
        let comp = Intersection::new(4.0, &s).prepare_computations(&r).unwrap();
        let c = w.shade_hit(&comp);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights = vec![PointLight::new(Point::new(0.0, 0.25, 0.0), color::WHITE)];
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = w.objects[1];
        let comp = Intersection::new(0.5, &s).prepare_computations(&r).unwrap();
        let c = w.shade_hit(&comp);
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

    #[test]
    fn rendering_a_world_with_camera() {
        let w = default_world();

        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let view_transform = Transform::view_transform(from, to, up);

        let c = Camera::new(11, 11, std::f64::consts::FRAC_PI_2).with_transform(view_transform);
        let image = c.render(&w);
        assert_eq!(
            image.color(5, 5).unwrap(),
            &Color::new(0.38066, 0.47583, 0.2855)
        );
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&w.lights[0], &p));
    }

    #[test]
    fn shadowed_when_object_between_point_and_light() {
        let w = default_world();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&w.lights[0], &p));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = default_world();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&w.lights[0], &p));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = default_world();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&w.lights[0], &p));
    }

    #[test]
    fn shade_hit_with_intersection_in_shadow() {
        let light_source = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        let s1 = Sphere::shape();
        let s2 = Sphere::shape().with_transform(Transform::translation(0.0, 0.0, 10.0));
        let w = World::new(vec![light_source], vec![s1, s2]);
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let comp = Intersection::new(4.0, &s2)
            .prepare_computations(&r)
            .unwrap();
        let c = w.shade_hit(&comp);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_for_a_nonreflective_material() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut w = default_world();
        w.objects[1] = w.objects[1].with_ambient(1.0);
        let comps = Intersection::new(1.0, &w.objects[1])
            .prepare_computations(&r)
            .unwrap();
        let color = w.reflected_color(&comps);
        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn reflected_color_for_a_reflective_material() {
        let mut w = default_world();
        let shape = Plane::shape()
            .with_reflective(0.5)
            .with_transform(Transform::translation(0.0, -1.0, 0.0));
        w.objects.push(shape);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(
                0.0,
                -std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
            ),
        );

        let comps = Intersection::new(std::f64::consts::SQRT_2, &w.objects[2])
            .prepare_computations(&r)
            .unwrap();
        let color = w.reflected_color(&comps);
        assert_eq!(color, Color::new(0.19033, 0.23791, 0.14274));
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = default_world();
        let shape = Plane::shape()
            .with_reflective(0.5)
            .with_transform(Transform::translation(0.0, -1.0, 0.0));
        w.objects.push(shape);

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(
                0.0,
                -std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
            ),
        );

        let comps = Intersection::new(std::f64::consts::SQRT_2, &w.objects[2])
            .prepare_computations(&r)
            .unwrap();
        let color = w.shade_hit(&comps);
        assert_eq!(color, Color::new(0.87675, 0.92433, 0.82917));
    }
}
