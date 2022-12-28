use raytracer::{shapes::ShapeWorld, Canvas, Color, Point, Ray, Sphere};

const IMAGE_PPM: &str = "test.ppm";
const IMAGE_PNG: &str = "test.png";

fn main() {
    let canvas_pixels = 100;

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::from("#ff0000");
    let shape = Sphere::shape();
    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);
            if xs.hit().is_some() {
                canvas.write_pixel(x, y, &color);
            }
        }
    }

    canvas
        .write_ppm(IMAGE_PPM)
        .expect("error during write to file");

    let img = image::open(IMAGE_PPM).unwrap();
    img.save(IMAGE_PNG).unwrap();
}
