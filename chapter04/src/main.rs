use raytracer::{color, Canvas, Point, Transform};

const IMAGE_PPM: &str = "test.ppm";
const IMAGE_PNG: &str = "test.png";

fn main() {
    let width = 100;
    let height = 100;
    let mut canvas = Canvas::new(width, height);

    let (x0, y0) = (width / 2, height / 2);

    let radius = width / 3;
    let point = Point::new(0.0, radius as f64, 0.0);

    for i in 0..12 {
        let alpha = i as f64 * std::f64::consts::FRAC_PI_6;
        let transform = Transform::identity()
            .rotate_z(alpha)
            .translate(x0 as f64, y0 as f64, 0.0);
        let rotated_point = transform * point;
        let x = rotated_point.x() as usize;
        let y = rotated_point.y() as usize;
        canvas.write_pixel(x, y, &color::WHITE);
    }

    canvas
        .write_ppm(IMAGE_PPM)
        .expect("Error during write to file");

    let img = image::open(IMAGE_PPM).unwrap();
    img.save(IMAGE_PNG).unwrap();
}
