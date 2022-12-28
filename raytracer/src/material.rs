use crate::{
    color, patterns::PatternWorld, phong::PhongReflecionModel, Color, Pattern, Point, PointLight,
    Shape, Vector,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    color: Color,
    model: PhongReflecionModel,
    pattern: Option<Pattern>,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            color: color::WHITE,
            model: PhongReflecionModel::default(),
            pattern: None,
        }
    }
}

#[allow(dead_code)]
impl Material {
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn with_color(mut self, color: Color) -> Material {
        self.color = color;
        self
    }

    pub(crate) fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Material {
        self.pattern = Some(pattern);
        self
    }

    pub fn ambient(&self) -> f64 {
        self.model.ambient()
    }

    pub fn with_ambient(mut self, ambient: f64) -> Material {
        self.model.set_ambient(ambient);
        self
    }

    pub fn diffuse(&self) -> f64 {
        self.model.diffuse()
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Material {
        self.model.set_diffuse(diffuse);
        self
    }

    pub fn specular(&self) -> f64 {
        self.model.specular()
    }

    pub fn with_specular(mut self, specular: f64) -> Material {
        self.model.set_specular(specular);
        self
    }

    pub fn shininess(&self) -> f64 {
        self.model.shininess()
    }

    pub fn with_shininess(mut self, shininess: f64) -> Material {
        self.model.set_shininess(shininess);
        self
    }

    pub(crate) fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        position: &Point,
        eye_vector: &Vector,
        normal_vector: &Vector,
        shadowed: bool,
    ) -> Color {
        if let Some(pattern) = self.pattern {
            let color = pattern.pattern_at_shape(object, position);
            return color;
        }

        let effective_color = self.color & light.intensity();
        let light_vector = (light.position() - *position).normalize();
        let ambient = effective_color * self.model.ambient();

        if shadowed {
            return ambient;
        }

        let light_dot_normal = light_vector.dot(normal_vector);

        let (diffuse, specular) = match light_dot_normal < 0.0 {
            true => (color::BLACK, color::BLACK),
            false => {
                let diffuse = effective_color * self.model.diffuse() * light_dot_normal;
                let reflected_vector = -light_vector.reflect(normal_vector);
                let reflected_dot_eye = reflected_vector.dot(eye_vector);

                let specular = match reflected_dot_eye <= 0.0 {
                    true => color::BLACK,
                    false => {
                        let factor = reflected_dot_eye.powf(self.model.shininess());
                        light.intensity() * self.model.specular() * factor
                    }
                };

                (diffuse, specular)
            }
        };

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use crate::{patterns::stripe::StripedPattern, shapes::dummy_shape::TestShape};

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color(), color::WHITE);
        assert_eq!(m.model, PhongReflecionModel::default());
        assert!(m.pattern.is_none());
    }

    #[test]
    fn assigning_pattern() {
        let p = StripedPattern::pattern(color::WHITE, color::BLACK);
        let m = Material::default().with_pattern(p);
        assert_eq!(m.pattern, Some(p));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let s = TestShape::shape();

        let p = StripedPattern::pattern(color::WHITE, color::BLACK);

        let m = Material::default()
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0)
            .with_pattern(p);

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), color::WHITE);
        let c1 = m.lighting(
            &s,
            &light,
            &Point::new(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        let c2 = m.lighting(
            &s,
            &light,
            &Point::new(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(c1, color::WHITE);
        assert_eq!(c2, color::BLACK);
    }
}
