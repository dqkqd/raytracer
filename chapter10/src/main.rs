use raytracer::{
    color, shapes::ShapeMaterial, Camera, CheckerPattern, Color, GradientPattern, Plane, Point,
    PointLight, RingPattern, Sphere, StripedPattern, Transform, Transformable, Vector, World,
};

const IMAGE_PPM: &str = "test.ppm";
const IMAGE_PNG: &str = "test.png";

fn main() {
    let floor = Plane::shape()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0)
        .with_pattern(
            CheckerPattern::pattern(color::WHITE, color::BLACK)
                .with_transform(Transform::scaling(0.4, 0.4, 0.4)),
        );

    let middle = Sphere::shape()
        .with_color(Color::new(0.1, 1.0, 0.5))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_transform(Transform::translation(-0.5, 1.0, 0.5))
        .with_pattern(
            StripedPattern::pattern(Color::from("#ff0000"), color::WHITE).with_transform(
                Transform::rotation_z(std::f64::consts::FRAC_PI_2).scale(0.3, 0.3, 0.3),
            ),
        );

    let right = Sphere::shape()
        .with_color(Color::new(0.5, 1.0, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_transform(Transform::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .with_pattern(
            RingPattern::pattern(Color::from("#ff0000"), Color::from("#0000ff"))
                .with_transform(Transform::scaling(0.05, 1.0, 1.0).rotate_y(1.6)),
        );

    let left = Sphere::shape()
        .with_color(Color::new(1.0, 0.8, 0.1))
        .with_diffuse(0.7)
        .with_specular(0.3)
        .with_transform(Transform::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75))
        .with_pattern(
            GradientPattern::pattern(color::BLACK, Color::from("#00ffff"))
                .with_transform(Transform::rotation_y(std::f64::consts::FRAC_PI_2 * 0.8)),
        );

    let light1 = PointLight::new(Point::new(-10.0, 10.0, -10.0), color::WHITE);
    let light2 = PointLight::new(Point::new(15.0, 10.0, -10.0), color::WHITE);
    let world = World::new(vec![light1, light2], vec![floor, middle, right, left]);

    let mut camera = Camera::new(800, 500, std::f64::consts::FRAC_PI_3);
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