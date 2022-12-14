use crate::{
    color::{self, Color},
    light::PointLight,
    patterns::{pattern::Pattern, PatternWorld},
    phong::PhongReflecionModel,
    point::Point,
    shapes::shape::Shape,
    vector::Vector,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Material {
    color: Color,
    model: PhongReflecionModel,
    pattern: Option<Pattern>,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            color: color::WHITE,
            model: PhongReflecionModel::default(),
            pattern: None,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
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

    pub fn pattern(&self) -> Option<&Pattern> {
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

    pub fn reflective(&self) -> f64 {
        self.reflective
    }

    pub fn with_reflective(mut self, reflective: f64) -> Material {
        self.reflective = reflective;
        self
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn with_transparency(mut self, transparency: f64) -> Material {
        self.transparency = transparency;
        self
    }

    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn with_refractive_index(mut self, refractive_index: f64) -> Material {
        self.refractive_index = refractive_index;
        self
    }

    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        position: &Point,
        eye_vector: &Vector,
        normal_vector: &Vector,
        shadowed: bool,
    ) -> Color {
        let color = match self.pattern {
            Some(pattern) => pattern.pattern_at_shape(object, position),
            None => self.color,
        };

        let effective_color = color & light.intensity();
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
    use crate::util::assert_float_eq;

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color(), color::WHITE);
        assert_eq!(m.model, PhongReflecionModel::default());
        assert!(m.pattern.is_none());
        assert_float_eq!(m.reflective, 0.0);
        assert_float_eq!(m.transparency, 0.0);
        assert_float_eq!(m.refractive_index, 1.0);
    }

    #[test]
    fn assigning_pattern() {
        let p = Pattern::stripe(color::WHITE, color::BLACK);
        let m = Material::default().with_pattern(p);
        assert_eq!(m.pattern, Some(p));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let s = Shape::dummy();

        let p = Pattern::stripe(color::WHITE, color::BLACK);

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
