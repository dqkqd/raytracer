use crate::{
    transform::{transformable, InversedTransform},
    Transform,
};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    inversed_transform: InversedTransform,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

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
}

transformable!(Camera);

#[cfg(test)]
mod test {
    use crate::util::assert_float_eq;

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
}
