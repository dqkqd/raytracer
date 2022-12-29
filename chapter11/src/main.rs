use raytracer::{
    color, shapes::ShapeMaterial, Camera, Color, Pattern, Point, PointLight, Shape, Transform,
    Transformable, Vector, World,
};

const IMAGE_PPM: &str = "test.ppm";
const IMAGE_PNG: &str = "test.png";

fn main() {
    let wall = Shape::plane()
        .with_ambient(0.8)
        .with_diffuse(0.2)
        .with_specular(0.0)
        .with_pattern(Pattern::checker(
            Color::new(0.15, 0.15, 0.15),
            Color::new(0.85, 0.85, 0.85),
        ))
        .with_transform(
            Transform::rotation_x(std::f64::consts::FRAC_PI_2).translate(0.0, 0.0, 10.0),
        );

    let glass_ball = Shape::sphere()
        .with_ambient(0.0)
        .with_diffuse(0.0)
        .with_specular(0.9)
        .with_shininess(300.0)
        .with_reflective(0.9)
        .with_transparency(0.9)
        .with_refractive_index(1.5)
        .with_color(color::WHITE);

    let center = Shape::sphere()
        .with_ambient(0.0)
        .with_diffuse(0.0)
        .with_specular(0.9)
        .with_shininess(300.0)
        .with_reflective(0.9)
        .with_transparency(0.9)
        .with_refractive_index(1.0000034)
        .with_transform(Transform::scaling(0.5, 0.5, 0.5))
        .with_color(color::WHITE);

    let light = PointLight::new(Point::new(2.0, 10.0, -5.0), Color::new(0.9, 0.9, 0.9));
    let world = World::new(vec![light], vec![wall, glass_ball, center]);

    let mut camera = Camera::new(800, 800, 0.45);
    let view_transform = Transform::view_transform(
        Point::new(0.0, 0.0, -5.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    camera.set_transform(view_transform);

    let canvas = camera.render(&world);

    canvas
        .write_ppm(IMAGE_PPM)
        .expect("Error during write to file");

    let img = image::open(IMAGE_PPM).unwrap();
    img.save(IMAGE_PNG).unwrap();
}
