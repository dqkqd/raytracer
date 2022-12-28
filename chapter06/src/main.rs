use raytracer::{
    color,
    object::{ObjectMaterial, ObjectWorld},
    Canvas, Color, Point, PointLight, Ray, Sphere,
};

const IMAGE_PPM: &str = "test.ppm";
const IMAGE_PNG: &str = "test.png";

fn main() {
    let canvas_pixels = 300;

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let shape_color = Color::new(1.0, 0.2, 1.0);
    let shape = Sphere::shape().with_color(shape_color);
    assert_eq!(shape_color, shape.material().color());

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), color::WHITE);

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let direction = (position - ray_origin).normalize();
            let r = Ray::new(ray_origin, direction);
            let eye = -direction;
            let xs = shape.intersect(&r);

            if let Some(i) = xs.hit() {
                let point = r.position(i.t());
                let normal = shape.normal_at(&point);
                if let Some(normal) = normal {
                    let color =
                        light.lighting(&shape, shape.material(), &point, &eye, &normal, false);
                    canvas.write_pixel(x, y, &color);
                }
            }
        }
    }

    canvas
        .write_ppm(IMAGE_PPM)
        .expect("error during write to file");

    let img = image::open(IMAGE_PPM).unwrap();
    img.save(IMAGE_PNG).unwrap();
}
