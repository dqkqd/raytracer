use crate::{
    transform::{transformable, InversedTransform},
    Color, Intersections, IntersectionsFactor, Material, Point, Ray, Sphere, Transform, Vector,
};

use super::object::{ObjectLocal, ObjectMaterial, ObjectWorld};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeKind {
    Sphere(Sphere),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shape {
    shape: ShapeKind,
    inversed_transform: InversedTransform,
    material: Material,
}

transformable!(Shape);

impl Shape {
    pub fn new(shape: ShapeKind) -> Shape {
        Shape {
            shape,
            inversed_transform: Some(Transform::identity()),
            material: Material::default(),
        }
    }
}

impl ObjectWorld for Shape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        self.transform_ray(ray)
            .map_or(Default::default(), |local_ray| {
                let roots = self.local_intersection(&local_ray);
                Intersections::new(roots, self, &local_ray)
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_shape_has_default_material() {
        let s = Sphere::shape();
        let m = s.material;
        assert_eq!(m, Material::default());
    }

    #[test]
    fn shape_with_material() {
        let m = Material::default().with_ambient(1.0);
        let s = Sphere::shape().with_material(m);
        assert_eq!(s.material, m);
    }

    #[test]
    fn shape_with_color() {
        let color = Color::new(0.1, 0.1, 0.1);
        let s = Sphere::shape().with_color(color);
        assert_eq!(s.material.color(), color);
    }

    #[test]
    fn shape_with_ambient() {
        let ambient = 0.6;
        let s = Sphere::shape().with_ambient(ambient);
        assert_eq!(s.material.ambient(), ambient);
    }

    #[test]
    fn shape_with_diffuse() {
        let diffuse = 0.2;
        let s = Sphere::shape().with_diffuse(diffuse);
        assert_eq!(s.material.diffuse(), diffuse);
    }

    #[test]
    fn shape_with_specular() {
        let specular = 0.6;
        let s = Sphere::shape().with_specular(specular);
        assert_eq!(s.material.specular(), specular);
    }

    #[test]
    fn shape_with_shininess() {
        let shininess = 1.5;
        let s = Sphere::shape().with_shininess(shininess);
        assert_eq!(s.material.shininess(), shininess);
    }
}
