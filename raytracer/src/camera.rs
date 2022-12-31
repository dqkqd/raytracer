use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    canvas::Canvas,
    point::Point,
    ray::Ray,
    transform::{transformable, InversedTransform, Transform},
    world::World,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    inversed_transform: InversedTransform,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

transformable!(Camera);

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = f64::tan(field_of_view / 2.0);
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = match aspect >= 1.0 {
            true => (half_view, half_view / aspect),
            false => (half_view * aspect, half_view),
        };

        let pixel_size = half_width * 2.0 / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            inversed_transform: Some(Transform::identity()),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Option<Ray> {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.inversed_transform? * Point::new(world_x, world_y, -1.0);
        let origin = self.inversed_transform? * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Some(Ray::new(origin, direction))
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        canvas
            .par_iter_mut()
            .enumerate()
            .for_each(|(y, vec_color)| {
                vec_color.par_iter_mut().enumerate().for_each(|(x, color)| {
                    if let Some(ray) = self.ray_for_pixel(x, y) {
                        *color = world.color_at(&ray);
                    }
                })
            });
        canvas
    }
}

#[cfg(test)]
mod test {

    use crate::{transform::Transformable, util::assert_float_eq, vector::Vector};

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f64::consts::FRAC_PI_2;

        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_float_eq!(c.field_of_view, field_of_view);
        assert_eq!(c.inversed_transform, Some(Transform::identity()));
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f64::consts::FRAC_PI_2);
        assert_float_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, std::f64::consts::FRAC_PI_2);
        assert_float_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50).unwrap();
        assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0).unwrap();
        assert_eq!(r.origin(), Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_when_camera_is_transformed() {
        let c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2).with_transform(
            Transform::translation(0.0, -2.0, 5.0).rotate_y(std::f64::consts::FRAC_PI_4),
        );
        let r = c.ray_for_pixel(100, 50).unwrap();
        assert_eq!(r.origin(), Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction(),
            Vector::new(
                std::f64::consts::FRAC_1_SQRT_2,
                0.0,
                -std::f64::consts::FRAC_1_SQRT_2
            )
        );
    }
}
