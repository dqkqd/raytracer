use std::ops::Mul;

use crate::{matrix::Matrix4, Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
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

    pub(crate) fn inverse(self) -> Option<Transform> {
        let matrix = self.matrix.inverse()?;
        Some(Transform { matrix })
    }

    pub(crate) fn tranpose(self) -> Transform {
        let matrix = self.matrix.tranpose();
        Transform { matrix }
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

impl Mul for Transform {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Transform {
            matrix: self.matrix * rhs.matrix,
        }
    }
}

// fluent api
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
    fn test_identity() {
        let transform = Transform::identity();
        let identity = Matrix4::identity();
        assert_eq!(transform.matrix, identity)
    }

    #[test]
    fn test_translate_point() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * point, Point::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_inv_translate_point() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);
        assert_eq!(
            transform.inverse().unwrap() * point,
            Point::new(-8.0, 7.0, 3.0)
        );
    }

    #[test]
    fn test_translate_vector() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let vector = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(transform * vector, vector);
    }

    #[test]
    fn test_scale_point() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let point = Point::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * point, Point::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_scale_vector() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let vector = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(transform * vector, Vector::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_inv_scale_vector() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let vector = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(
            transform.inverse().unwrap() * vector,
            Vector::new(-2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn test_reflection_using_scaling() {
        let transform = Transform::scaling(-1.0, 1.0, 1.0);
        let vector = Point::new(1.0, 2.0, 3.0);
        assert_eq!(
            transform.inverse().unwrap() * vector,
            Point::new(-1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn test_rotate_x() {
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
    fn test_rotate_y() {
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
    fn test_rotate_z() {
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
    fn test_shear_xy() {
        let transform = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_shear_xz() {
        let transform = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shear_yx() {
        let transform = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_shear_yz() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shear_zx() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shear_zy() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Point::new(2.0, 3.0, 4.0);
        assert_eq!(transform * point, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_chain_individual() {
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
    fn test_chained() {
        let point = Point::new(1.0, 0.0, 1.0);
        let rotation = Transform::rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling = Transform::scaling(5.0, 5.0, 5.0);
        let translation = Transform::translation(10.0, 5.0, 7.0);
        let transformation = translation * scaling * rotation;
        let point = transformation * point;
        assert_eq!(point, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn test_chained_fluent_api() {
        let point = Point::new(1.0, 0.0, 1.0);
        let transform = Transform::identity()
            .rotate_x(std::f64::consts::FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(transform * point, Point::new(15.0, 0.0, 7.0));
    }
}
