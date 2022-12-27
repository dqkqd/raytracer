use raytracer::{
    color, object::ObjectMaterial, Camera, Color, Plane, Point, PointLight, Sphere, Transform,
    Transformable, Vector, World,
};

const IMAGE_PPM: &str = "test.ppm";
const IMAGE_PNG: &str = "test.png";

fn main() {
    let floor = Plane::shape()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let middle = Sphere::shape()
        .with_color(Color::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_transform(Transform::translation(-0.5, 1.0, 0.5));

    let right = Sphere::shape()
        .with_color(Color::new(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_transform(Transform::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5));

    let left = Sphere::shape()
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_transform(Transform::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75));

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), color::WHITE);
    let world = World::new(light, vec![floor, middle, right, left]);

    let mut camera = Camera::new(500, 300, std::f64::consts::FRAC_PI_3);
    let view_transform = Transform::view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
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
