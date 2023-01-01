use std::ops::Mul;

pub(crate) type InversedTransform = Option<Transform>;
pub(crate) trait Transformable {
    fn inversed_transform(&self) -> InversedTransform;
    fn set_transform(&mut self, transform: Transform);
    fn with_transform(self, transform: Transform) -> Self;
}

macro_rules! transformable {
    ($struct_name:ident) => {
        impl $crate::transform::Transformable for $struct_name {
            fn inversed_transform(&self) -> $crate::transform::InversedTransform {
                self.inversed_transform
            }
            fn set_transform(&mut self, transform: $crate::transform::Transform) {
                self.inversed_transform = transform.inverse();
            }

            fn with_transform(mut self, transform: $crate::transform::Transform) -> Self {
                self.set_transform(transform);
                self
            }
        }
    };
}

pub(crate) use transformable;

use crate::{matrix::Matrix4, point::Point, ray::Ray, vector::Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Transform {
    matrix: Matrix4,
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            matrix: Matrix4::identity(),
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Transform {
        let matrix = Matrix4::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Transform { matrix }
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Transform {
        let matrix = Matrix4::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Transform { matrix }
    }

    pub fn rotation_x(a: f64) -> Transform {
        let matrix = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, f64::cos(a), -f64::sin(a), 0.0],
            [0.0, f64::sin(a), f64::cos(a), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Transform { matrix }
    }

    pub fn rotation_y(a: f64) -> Transform {
        let matrix = Matrix4::new([
            [f64::cos(a), 0.0, f64::sin(a), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-f64::sin(a), 0.0, f64::cos(a), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Transform { matrix }
    }

    pub fn rotation_z(a: f64) -> Transform {
        let matrix = Matrix4::new([
            [f64::cos(a), -f64::sin(a), 0.0, 0.0],
            [f64::sin(a), f64::cos(a), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Transform { matrix }
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Transform {
        let matrix = Matrix4::new([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Transform { matrix }
    }

    pub(crate) fn inverse(self) -> InversedTransform {
        let matrix = self.matrix.inverse()?;
        Some(Transform { matrix })
    }

    pub(crate) fn tranpose(self) -> Transform {
        let matrix = self.matrix.tranpose();
        Transform { matrix }
    }

    pub fn view_transform(from: Point, to: Point, up: Vector) -> Transform {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(&upn);
        let true_up = left.cross(&forward);

        let orientation = Transform {
            matrix: Matrix4::new([
                [left.x(), left.y(), left.z(), 0.0],
                [true_up.x(), true_up.y(), true_up.z(), 0.0],
                [-forward.x(), -forward.y(), -forward.z(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
        };

        orientation * Transform::translation(-from.x(), -from.y(), -from.z())
    }
}

impl Mul<Point> for Transform {
    type Output = Point;
    fn mul(self, point: Point) -> Self::Output {
        let value: Vec<_> = (0..4)
            .map(|i| {
                self.matrix[i][0] * point.x()
                    + self.matrix[i][1] * point.y()
                    + self.matrix[i][2] * point.z()
                    + self.matrix[i][3] * point.w()
            })
            .collect();
        Point::new(value[0], value[1], value[2])
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;
    fn mul(self, vector: Vector) -> Self::Output {
        let value: Vec<_> = (0..4)
            .map(|i| {
                self.matrix[i][0] * vector.x()
                    + self.matrix[i][1] * vector.y()
                    + self.matrix[i][2] * vector.z()
                    + self.matrix[i][3] * vector.w()
            })
            .collect();
        Vector::new(value[0], value[1], value[2])
    }
}

impl Mul<Ray> for Transform {
    type Output = Ray;
    fn mul(self, ray: Ray) -> Self::Output {
        Ray::new(
            ray.origin().transform(self),
            ray.direction().transform(self),
        )
    }
}

impl Mul for Transform {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Transform {
            matrix: self.matrix * rhs.matrix,
        }
    }
}

// fluent api
#[allow(dead_code)]
impl Transform {
    pub fn translate(self, x: f64, y: f64, z: f64) -> Transform {
        Transform::translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Transform {
        Transform::scaling(x, y, z) * self
    }

    pub fn rotate_x(self, a: f64) -> Transform {
        Transform::rotation_x(a) * self
    }

    pub fn rotate_y(self, a: f64) -> Transform {
        Transform::rotation_y(a) * self
    }

    pub fn rotate_z(self, a: f64) -> Transform {
        Transform::rotation_z(a) * self
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Transform {
        Transform::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn transform_identity() {
        let transform = Transform::identity();
        let identity = Matrix4::identity();
        assert_eq!(transform.matrix, identity)
    }

    #[test]
    fn translate_point() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * point, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn inversed_translate_point() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(
            transform.inverse().map(|t| t * point),
            Some(Point::new(-8.0, 7.0, 3.0))
        );
    }

    #[test]
    fn translate_vector() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let vector = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * vector, vector);
    }

    #[test]
    fn scale_point() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let point = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * point, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scale_vector() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let vector = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * vector, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn inverse_scale_vector() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let vector = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(
            transform.inverse().map(|t| t * vector),
            Some(Vector::new(-2.0, 2.0, 2.0))
        );
    }

    #[test]
    fn reflection_using_scaling() {
        let transform = Transform::scaling(-1.0, 1.0, 1.0);
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(
            transform.inverse().map(|t| t * point),
            Some(Point::new(-1.0, 2.0, 3.0))
        );
    }

    #[test]
    fn rotate_x() {
        let transform = Transform::rotation_x(std::f64::consts::FRAC_PI_4);
        let point = Point::new(0.0, 1.0, 0.0);
        assert_eq!(
            transform * point,
            Point::new(0.0, 1.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0)),
        );

        let transform = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let point = Point::new(0.0, 1.0, 0.0);
        assert_eq!(transform * point, Point::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotate_y() {
        let transform = Transform::rotation_y(std::f64::consts::FRAC_PI_4);
        let point = Point::new(0.0, 0.0, 1.0);
        assert_eq!(
            transform * point,
            Point::new(1.0 / f64::sqrt(2.0), 0.0, 1.0 / f64::sqrt(2.0)),
        );

        let transform = Transform::rotation_y(std::f64::consts::FRAC_PI_2);
        let point = Point::new(0.0, 0.0, 1.0);
        assert_eq!(transform * point, Point::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotate_z() {
        let transform = Transform::rotation_z(std::f64::consts::FRAC_PI_4);
        let point = Point::new(0.0, 1.0, 0.0);
        assert_eq!(
            transform * point,
            Point::new(-1.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0), 0.0),
        );

        let transform = Transform::rotation_z(std::f64::consts::FRAC_PI_2);
        let point = Point::new(0.0, 1.0, 0.0);
        assert_eq!(transform * point, Point::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shear_xy() {
        let transform = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn shear_xz() {
        let transform = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn shear_yx() {
        let transform = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn shear_yz() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn shear_zx() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn shear_zy() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn chain_individual_point() {
        let point = Point::new(1.0, 0.0, 1.0);
        let rotation = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling = Transform::scaling(5.0, 5.0, 5.0);
        let translation = Transform::translation(10.0, 5.0, 7.0);

        let point = rotation * point;
        assert_eq!(point, Point::new(1.0, -1.0, 0.0));

        let point = scaling * point;
        assert_eq!(point, Point::new(5.0, -5.0, 0.0));

        let point = translation * point;
        assert_eq!(point, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transform() {
        let point = Point::new(1.0, 0.0, 1.0);
        let rotation = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling = Transform::scaling(5.0, 5.0, 5.0);
        let translation = Transform::translation(10.0, 5.0, 7.0);
        let transformation = translation * scaling * rotation;
        let point = transformation * point;
        assert_eq!(point, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_fluent_api() {
        let point = Point::new(1.0, 0.0, 1.0);
        let transform = Transform::identity()
            .rotate_x(std::f64::consts::FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(transform * point, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn view_transform_for_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transform::view_transform(from, to, up);
        assert_eq!(t, Transform::identity());
    }

    #[test]
    fn view_transform_looking_in_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transform::view_transform(from, to, up);
        assert_eq!(t, Transform::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transform_move_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transform::view_transform(from, to, up);
        assert_eq!(t, Transform::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transform() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = Transform::view_transform(from, to, up);

        let expected = Transform {
            matrix: Matrix4::new([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000],
            ]),
        };

        assert_eq!(t, expected);
    }
}
