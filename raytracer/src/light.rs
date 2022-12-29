use crate::{Color, Material, Point, Shape, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }

    pub fn lighting(
        &self,
        object: &Shape,
        material: &Material,
        position: &Point,
        eye_vector: &Vector,
        normal_vector: &Vector,
        shadowed: bool,
    ) -> Color {
        material.lighting(object, self, position, eye_vector, normal_vector, shadowed)
    }
}

#[cfg(test)]
mod test {

    use crate::{color};

    use super::*;

    fn shape_material_point_setup() -> (Shape, Material, Point) {
        (
            Shape::dummy(),
            Material::default(),
            Point::new(0.0, 0.0, 0.0),
        )
    }

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = color::WHITE;
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        let (s, m, position) = shape_material_point_setup();
        let result = light.lighting(&s, &m, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let eyev = Vector::new(
            0.0,
            std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        );
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        let (s, m, position) = shape_material_point_setup();
        let result = light.lighting(&s, &m, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), color::WHITE);
        let (s, m, position) = shape_material_point_setup();
        let result = light.lighting(&s, &m, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let eyev = Vector::new(
            0.0,
            -std::f64::consts::FRAC_1_SQRT_2,
            -std::f64::consts::FRAC_1_SQRT_2,
        );
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), color::WHITE);
        let (s, m, position) = shape_material_point_setup();
        let result = light.lighting(&s, &m, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), color::WHITE);
        let (s, m, position) = shape_material_point_setup();
        let result = light.lighting(&s, &m, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        let shadowed = true;
        let (s, m, position) = shape_material_point_setup();
        let result = light.lighting(&s, &m, &position, &eyev, &normalv, shadowed);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
