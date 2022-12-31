use std::path::PathBuf;

use tempfile::NamedTempFile;

use crate::world::World;

use super::{objects::object::Object, yaml};

fn create_named_temporary_file() -> String {
    let file = NamedTempFile::new().expect("Could not create temporary file");
    let filename = file
        .path()
        .to_str()
        .expect("Path could not convert to str")
        .to_string();
    file.close().expect("Error closing filename");

    let path = PathBuf::from(filename).with_extension("ppm");
    path.into_os_string()
        .into_string()
        .expect("Error create temporary file")
}

pub fn render(yaml_file: &str, image_file: &str) {
    let objects = yaml::from_file(yaml_file).expect("Error rendering yaml");

    let mut camera = None;
    let mut lights = Vec::new();
    let mut shapes = Vec::new();
    for object in objects {
        match object {
            Object::Camera(c) => camera = Some(c),
            Object::Light(light) => lights.push(light),
            Object::Shape(shape) => shapes.push(shape),
        }
    }

    let camera = camera.expect("Camera does not exist in yaml file");
    let world = World::new(lights, shapes);
    let canvas = camera.render(&world);

    let ppm_file = create_named_temporary_file();
    canvas
        .write_ppm(&ppm_file)
        .expect("Error write to ppm file");

    image::open(ppm_file)
        .expect("Error open ppm_file")
        .save(image_file)
        .expect("Error write to image");
}
