use crate::{color, phong::PhongReflecionModel, Color};

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
    pub(crate) fn color(&self) -> Color {
        self.color
    }

    pub(crate) fn model(&self) -> &PhongReflecionModel {
        &self.model
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color(), color::WHITE);
        assert_eq!(m.model().ambient(), 0.1);
        assert_eq!(m.model().diffuse(), 0.9);
        assert_eq!(m.model().specular(), 0.9);
        assert_eq!(m.model().shininess(), 200.0);
    }
}
