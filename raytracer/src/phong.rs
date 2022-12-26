pub(crate) struct PhongReflecionModel {
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Default for PhongReflecionModel {
    fn default() -> PhongReflecionModel {
        PhongReflecionModel::new(0.1, 0.9, 0.9, 200.0)
    }
}
impl PhongReflecionModel {
    pub fn new(ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> PhongReflecionModel {
        PhongReflecionModel {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_shininess(&mut self, shininess: f64) {
        self.shininess = shininess
    }
}
