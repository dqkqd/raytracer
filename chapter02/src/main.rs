use raytracer::{Canvas, Color, Point, Vector};

const IMAGE_PPM: &str = "test.ppm";
const IMAGE_PNG: &str = "test.png";

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Projectile {
        Projectile { position, velocity }
    }
    pub fn position(&self) -> Point {
        self.position
    }

    #[allow(dead_code)]
    pub fn velocity(&self) -> Vector {
        self.velocity
    }
    pub fn next(self, env: &Environment) -> Projectile {
        let position = self.position() + self.velocity();
        let velocity = self.velocity() + env.gravity() + env.wind();
        Projectile::new(position, velocity)
    }
}
struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Environment {
        Environment { gravity, wind }
    }
    pub fn gravity(&self) -> Vector {
        self.gravity
    }
    pub fn wind(&self) -> Vector {
        self.wind
    }
}

fn tick(env: Environment, proj: Projectile) -> Vec<Point> {
    let mut positions = vec![];
    let mut projectile = proj;
    loop {
        let p = projectile.position();
        if p.y() <= 0.0 {
            break;
        }
        positions.push(p);
        projectile = projectile.next(&env);
    }
    positions
}

fn main() {
    let p = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.0, 0.0).normalize() * 7.0,
    );
    let e = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));
    let positions = tick(e, p);

    let width = 550;
    let height = 200;
    let mut canvas = Canvas::new(width, height);
    let red = Color::new(1.0, 0.0, 0.0);
    for point in positions {
        let x = point.x().round() as usize;
        let y = point.y().round() as usize;
        canvas.write_pixel(x, height - y, &red);
    }
    canvas
        .write_ppm(IMAGE_PPM)
        .expect("Error during write to file");

    let img = image::open(IMAGE_PPM).unwrap();
    img.save(IMAGE_PNG).unwrap();
}
