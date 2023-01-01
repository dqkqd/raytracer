use crate::{
    color::Color,
    intersect::{intersection::IntersectionsFactor, multiple_intersections::Intersections},
    material::Material,
    patterns::pattern::Pattern,
    point::Point,
    ray::Ray,
    transform::{transformable, InversedTransform, Transform},
    vector::Vector,
};

use super::{
    cube::Cube, cylinder::Cylinder, dummy::Dummy, plane::Plane, sphere::Sphere, ShapeKind,
    ShapeLocal, ShapeMaterial, ShapeWorld,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Shape {
    shape: ShapeKind,
    inversed_transform: InversedTransform,
    material: Material,
}

transformable!(Shape);

#[allow(dead_code)]
impl Shape {
    fn new(shape: ShapeKind) -> Shape {
        Shape {
            shape,
            inversed_transform: Some(Transform::identity()),
            material: Material::default(),
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn sphere() -> Shape {
        Shape::new(ShapeKind::Sphere(Sphere::default()))
    }

    pub(crate) fn as_sphere(&self) -> Option<&Sphere> {
        match &self.shape {
            ShapeKind::Sphere(sphere) => Some(sphere),
            _ => None,
        }
    }

    pub fn plane() -> Shape {
        Shape::new(ShapeKind::Plane(Plane::default()))
    }

    pub(crate) fn as_plane(&self) -> Option<&Plane> {
        match &self.shape {
            ShapeKind::Plane(plane) => Some(plane),
            _ => None,
        }
    }

    pub fn cube() -> Shape {
        Shape::new(ShapeKind::Cube(Cube::default()))
    }

    pub(crate) fn as_cube(&self) -> Option<&Cube> {
        match &self.shape {
            ShapeKind::Cube(cube) => Some(cube),
            _ => None,
        }
    }

    pub fn cylinder() -> Shape {
        Shape::new(ShapeKind::Cylinder(Cylinder::default()))
    }

    pub fn closed_cylinder(minimum: f64, maximum: f64) -> Shape {
        Shape::new(ShapeKind::Cylinder(Cylinder::new(minimum, maximum)))
    }

    pub fn as_cylinder(&self) -> Option<&Cylinder> {
        match &self.shape {
            ShapeKind::Cylinder(cylinder) => Some(cylinder),
            _ => None,
        }
    }

    pub fn dummy() -> Shape {
        Shape::new(ShapeKind::Dummy(Dummy::default()))
    }
}

impl ShapeWorld for Shape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        self.transform_ray(ray)
            .map_or(Default::default(), |local_ray| {
                let roots = self.local_intersection(&local_ray);
                Intersections::new(roots, self, ray)
            })
    }
}

impl ShapeLocal for Shape {
    fn local_intersection(&self, local_ray: &Ray) -> IntersectionsFactor {
        match self.shape {
            ShapeKind::Sphere(s) => s.local_intersection(local_ray),
            ShapeKind::Dummy(s) => s.local_intersection(local_ray),
            ShapeKind::Plane(p) => p.local_intersection(local_ray),
            ShapeKind::Cube(c) => c.local_intersection(local_ray),
            ShapeKind::Cylinder(c) => c.local_intersection(local_ray),
        }
    }

    fn local_normal_at(&self, object_point: &Point) -> Vector {
        match self.shape {
            ShapeKind::Sphere(s) => s.local_normal_at(object_point),
            ShapeKind::Dummy(s) => s.local_normal_at(object_point),
            ShapeKind::Plane(p) => p.local_normal_at(object_point),
            ShapeKind::Cube(c) => c.local_normal_at(object_point),
            ShapeKind::Cylinder(c) => c.local_normal_at(object_point),
        }
    }
}

impl ShapeMaterial for Shape {
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

    fn with_ambient(self, ambient: f64) -> Self {
        self.with_material(self.material.with_ambient(ambient))
    }

    fn with_diffuse(self, diffuse: f64) -> Self {
        self.with_material(self.material.with_diffuse(diffuse))
    }

    fn with_specular(self, specular: f64) -> Self {
        self.with_material(self.material.with_specular(specular))
    }

    fn with_shininess(self, shininess: f64) -> Self {
        self.with_material(self.material.with_shininess(shininess))
    }

    fn with_reflective(self, reflective: f64) -> Self {
        self.with_material(self.material.with_reflective(reflective))
    }

    fn with_pattern(self, pattern: Pattern) -> Self {
        self.with_material(self.material.with_pattern(pattern))
    }

    fn with_transparency(self, transparency: f64) -> Self {
        self.with_material(self.material.with_transparency(transparency))
    }

    fn with_refractive_index(self, refractive_index: f64) -> Self {
        self.with_material(self.material.with_refractive_index(refractive_index))
    }
}

#[cfg(test)]
mod test {

    use crate::{transform::Transformable, util::assert_float_eq};

    use super::*;

    #[test]
    fn shape_default_transformation() {
        let s = Shape::dummy();
        assert_eq!(s.inversed_transform(), Some(Transform::identity()));
    }

    #[test]
    fn default_shape_has_default_material() {
        let s = Shape::dummy();
        let m = s.material;
        assert_eq!(m, Material::default());
    }

    #[test]
    fn assigning_a_transformation() {
        let t = Transform::translation(2.0, 3.0, 4.0);
        let s = Shape::dummy().with_transform(t);
        assert_eq!(s.inversed_transform(), t.inverse());
    }

    #[test]
    fn default_material() {
        let s = Shape::dummy();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn assigning_a_material() {
        let m = Material::default().with_ambient(1.0);
        let s = Shape::dummy().with_material(m);
        assert_eq!(s.material, m);
    }

    #[test]
    fn shape_with_color() {
        let color = Color::new(0.1, 0.1, 0.1);
        let s = Shape::dummy().with_color(color);
        assert_eq!(s.material.color(), color);
    }

    #[test]
    fn shape_with_ambient() {
        let ambient = 0.6;
        let s = Shape::dummy().with_ambient(ambient);
        assert_float_eq!(s.material.ambient(), ambient);
    }

    #[test]
    fn shape_with_diffuse() {
        let diffuse = 0.2;
        let s = Shape::dummy().with_diffuse(diffuse);
        assert_float_eq!(s.material.diffuse(), diffuse);
    }

    #[test]
    fn shape_with_specular() {
        let specular = 0.6;
        let s = Shape::dummy().with_specular(specular);
        assert_float_eq!(s.material.specular(), specular);
    }

    #[test]
    fn shape_with_shininess() {
        let shininess = 1.5;
        let s = Shape::dummy().with_shininess(shininess);
        assert_float_eq!(s.material.shininess(), shininess);
    }

    #[test]
    fn shape_with_reflective() {
        let reflective = 0.5;
        let s = Shape::dummy().with_reflective(reflective);
        assert_float_eq!(s.material.reflective(), reflective);
    }

    #[test]
    fn shape_with_transparency() {
        let transparency = 0.3;
        let s = Shape::dummy().with_transparency(transparency);
        assert_float_eq!(s.material.transparency(), transparency)
    }

    #[test]
    fn shape_with_refractive_index() {
        let refractive_index = 0.3;
        let s = Shape::dummy().with_refractive_index(refractive_index);
        assert_float_eq!(s.material.refractive_index(), refractive_index)
    }

    #[test]
    fn shape_with_default_pattern() {
        let p = Pattern::dummy();
        let s = Shape::dummy().with_pattern(p);
        assert_eq!(s.material.pattern(), Some(&p));
    }

    #[test]
    fn intersections_contain_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::dummy().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let i = s.intersect(&r);
        let o = i.get(0).map(|v| v.object());
        assert_eq!(o, Some(&s));
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::dummy().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let local_ray = s.transform_ray(&r);
        let expected_ray = Ray::new(Point::new(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5));
        assert_eq!(local_ray, Some(expected_ray))
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Shape::sphere().with_transform(Transform::translation(5.0, 0.0, 0.0));
        let local_ray = s.transform_ray(&r);
        let expected_ray = Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(local_ray, Some(expected_ray));
    }

    #[test]
    fn normal_on_translated_shape() {
        let s = Shape::dummy().with_transform(Transform::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Point::new(
            0.0,
            1.0 + std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        ));

        assert_eq!(
            n,
            Some(Vector::new(
                0.0,
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2
            ))
        );
    }

    #[test]
    fn normal_on_transformed_shape() {
        let m = Transform::rotation_z(std::f64::consts::PI / 5.0).scale(1.0, 0.5, 1.0);
        let s = Shape::dummy().with_transform(m);
        let n = s.normal_at(&Point::new(
            0.0,
            std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        ));

        assert_eq!(n, Some(Vector::new(0.0, 0.97014, -0.24254)));
    }
}
