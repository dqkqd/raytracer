use crate::{color, phong::PhongReflecionModel, Color, Point, PointLight, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    color: Color,
    model: PhongReflecionModel,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            color: color::WHITE,
            model: PhongReflecionModel::default(),
        }
    }
}
impl Material {
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn with_color(mut self, color: Color) -> Material {
        self.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Material {
        self.model.set_ambient(ambient);
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Material {
        self.model.set_diffuse(diffuse);
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Material {
        self.model.set_specular(specular);
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Material {
        self.model.set_shininess(shininess);
        self
    }

    pub(crate) fn lighting(
        &self,
        light: &PointLight,
        position: &Point,
        eye_vector: &Vector,
        normal_vector: &Vector,
    ) -> Color {
        let effective_color = self.color & light.intensity();
        let light_vector = (light.position() - *position).normalize();
        let ambient = effective_color * self.model.ambient();
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
    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color(), color::WHITE);
        assert_eq!(m.model.ambient(), 0.1);
        assert_eq!(m.model.diffuse(), 0.9);
        assert_eq!(m.model.specular(), 0.9);
        assert_eq!(m.model.shininess(), 200.0);
    }
}
