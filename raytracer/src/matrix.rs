use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub,
    SubAssign,
};

use crate::util::equal;

macro_rules! create_simple_matrix {
    ($matrix:ident, $size: expr) => {
        #[derive(Debug, Clone, Copy)]
        pub(crate) struct $matrix {
            matrix: [[f64; $size]; $size],
        }

        impl Default for $matrix {
            fn default() -> $matrix {
                $matrix {
                    matrix: [[0.0; $size]; $size],
                }
            }
        }

        impl $matrix {
            pub fn new(matrix: [[f64; $size]; $size]) -> $matrix {
                $matrix { matrix }
            }

            pub fn size(&self) -> usize {
                $size
            }

            pub fn identity() -> $matrix {
                let mut matrix = [[0.0; $size]; $size];
                for i in 0..$size {
                    matrix[i][i] = 1.0;
                }
                $matrix { matrix }
            }

            pub fn tranpose(&self) -> $matrix {
                let mut matrix = $matrix::default();
                for i in 0..$size {
                    for j in 0..$size {
                        matrix[i][j] = self[j][i];
                    }
                }
                matrix
            }
        }

        impl Index<usize> for $matrix {
            type Output = [f64; $size];
            fn index(&self, index: usize) -> &Self::Output {
                &self.matrix[index]
            }
        }

        impl IndexMut<usize> for $matrix {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.matrix[index]
            }
        }

        impl PartialEq for $matrix {
            fn eq(&self, other: &Self) -> bool {
                for i in 0..$size {
                    for j in 0..$size {
                        if !equal(self[i][j], other[i][j]) {
                            return false;
                        }
                    }
                }
                true
            }
        }

        impl Add<$matrix> for $matrix {
            type Output = $matrix;
            fn add(self, rhs: $matrix) -> Self::Output {
                let mut matrix = $matrix::default();
                for i in 0..$size {
                    for j in 0..$size {
                        matrix[i][j] = self[i][j] + rhs[i][j];
                    }
                }
                matrix
            }
        }

        impl AddAssign for $matrix {
            fn add_assign(&mut self, rhs: $matrix) {
                *self = *self + rhs
            }
        }

        impl Sub<$matrix> for $matrix {
            type Output = $matrix;
            fn sub(self, rhs: $matrix) -> Self::Output {
                let mut matrix = $matrix::default();
                for r in 0..$size {
                    for c in 0..$size {
                        matrix[r][c] = self[r][c] - rhs[r][c];
                    }
                }
                matrix
            }
        }

        impl SubAssign for $matrix {
            fn sub_assign(&mut self, rhs: Self) {
                *self = self.clone() - rhs;
            }
        }

        impl Mul for $matrix {
            type Output = $matrix;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut matrix = $matrix::default();
                for i in 0..$size {
                    for j in 0..$size {
                        for k in 0..$size {
                            matrix[i][j] += self[i][k] * rhs[k][j];
                        }
                    }
                }
                matrix
            }
        }

        impl MulAssign for $matrix {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl Mul<f64> for $matrix {
            type Output = $matrix;
            fn mul(self, rhs: f64) -> Self::Output {
                let mut matrix = self;
                for i in 0..$size {
                    for j in 0..$size {
                        matrix[i][j] *= rhs;
                    }
                }
                matrix
            }
        }

        impl MulAssign<f64> for $matrix {
            fn mul_assign(&mut self, rhs: f64) {
                *self = *self * rhs;
            }
        }

        impl Div<f64> for $matrix {
            type Output = Self;
            fn div(self, rhs: f64) -> Self::Output {
                self * (1.0 / rhs)
            }
        }

        impl DivAssign<f64> for $matrix {
            fn div_assign(&mut self, rhs: f64) {
                *self = *self / rhs;
            }
        }

        #[allow(clippy::suspicious_arithmetic_impl)]
        impl BitAnd for $matrix {
            type Output = $matrix;
            fn bitand(self, rhs: Self) -> Self::Output {
                let mut matrix = $matrix::default();
                for i in 0..$size {
                    for j in 0..$size {
                        matrix[i][j] = self[i][j] * rhs[i][j];
                    }
                }
                matrix
            }
        }

        impl BitAndAssign for $matrix {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = self.clone() & rhs;
            }
        }
    };
}

macro_rules! implement_inverse_for_matrix {
    ($matrix:ident, $submatrix:ident) => {
        impl $matrix {
            pub fn submatrix(&self, r: usize, c: usize) -> $submatrix {
                let mut matrix = $submatrix::default();
                for i in 0..self.size() {
                    for j in 0..self.size() {
                        if i == r || j == c {
                            continue;
                        }
                        let x = if i > r { i - 1 } else { i };
                        let y = if j > c { j - 1 } else { j };
                        matrix[x][y] = self[i][j];
                    }
                }
                matrix
            }

            pub fn cofactor(&self, r: usize, c: usize) -> f64 {
                let submatrix = self.submatrix(r, c);
                let determinant = submatrix.determinant();
                match (r + c) % 2 {
                    0 => determinant,
                    _ => -determinant,
                }
            }

            pub fn determinant(&self) -> f64 {
                (0..self.size()).fold(0.0, |determinant, c| {
                    determinant + self[0][c] * self.cofactor(0, c)
                })
            }

            pub fn invertible(&self) -> bool {
                equal(self.determinant(), 0.0)
            }

            pub fn inverse(&self) -> Option<$matrix> {
                let determinant = self.determinant();
                if equal(determinant, 0.0) {
                    return None;
                }
                let mut matrix = $matrix::default();
                for i in 0..self.size() {
                    for j in 0..self.size() {
                        matrix[j][i] = self.cofactor(i, j) / determinant;
                    }
                }
                Some(matrix)
            }
        }
    };
}

create_simple_matrix!(Matrix2, 2);
create_simple_matrix!(Matrix3, 3);
create_simple_matrix!(Matrix4, 4);

impl Matrix2 {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

implement_inverse_for_matrix!(Matrix3, Matrix2);
implement_inverse_for_matrix!(Matrix4, Matrix3);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let matrix = Matrix4::default();
        for r in 0..4 {
            for c in 0..4 {
                assert_eq!(matrix[r][c], 0.0);
            }
        }
    }

    #[test]
    fn test_equal() {
        let matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn test_non_equal() {
        let matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 1.0],
        ]);

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn test_add() {
        let matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::new([
            [-1.0, 3.0, 5.0, 7.0],
            [8.0, 8.0, 8.0, 7.0],
            [13.0, 11.0, 13.0, 11.0],
            [6.0, 6.0, 10.0, 10.0],
        ]);

        assert_eq!(matrix1 + matrix2, expected);
    }

    #[test]
    fn test_add_assign() {
        let mut matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::new([
            [-1.0, 3.0, 5.0, 7.0],
            [8.0, 8.0, 8.0, 7.0],
            [13.0, 11.0, 13.0, 11.0],
            [6.0, 6.0, 10.0, 10.0],
        ]);

        matrix1 += matrix2;
        assert_eq!(matrix1, expected);
    }

    #[test]
    fn test_sub() {
        let matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::new([
            [3.0, 1.0, 1.0, 1.0],
            [2.0, 4.0, 6.0, 9.0],
            [5.0, 5.0, 1.0, 1.0],
            [4.0, 2.0, -4.0, -6.0],
        ]);

        assert_eq!(matrix1 - matrix2, expected);
    }

    #[test]
    fn test_sub_assign() {
        let mut matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::new([
            [3.0, 1.0, 1.0, 1.0],
            [2.0, 4.0, 6.0, 9.0],
            [5.0, 5.0, 1.0, 1.0],
            [4.0, 2.0, -4.0, -6.0],
        ]);

        matrix1 -= matrix2;
        assert_eq!(matrix1, expected);
    }

    #[test]
    fn test_mul() {
        let matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(matrix1 * matrix2, expected);
    }

    #[test]
    fn test_mul_assign() {
        let mut matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let matrix2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        matrix1 *= matrix2;
        assert_eq!(matrix1, expected);
    }

    #[test]
    fn test_mul_identity() {
        let matrix = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let identity = Matrix4::identity();
        assert_eq!(matrix.clone(), matrix * identity);
    }

    #[test]
    fn test_mul_scalar() {
        let matrix = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let expected = Matrix4::new([
            [2.0, 4.0, 6.0, 8.0],
            [10.0, 12.0, 14.0, 16.0],
            [18.0, 16.0, 14.0, 12.0],
            [10.0, 8.0, 6.0, 4.0],
        ]);

        assert_eq!(matrix * 2.0, expected);
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut matrix = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let expected = Matrix4::new([
            [2.0, 4.0, 6.0, 8.0],
            [10.0, 12.0, 14.0, 16.0],
            [18.0, 16.0, 14.0, 12.0],
            [10.0, 8.0, 6.0, 4.0],
        ]);

        matrix *= 2.0;
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_div_scalar() {
        let matrix = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let expected = Matrix4::new([
            [0.5, 1.0, 1.5, 2.0],
            [2.5, 3.0, 3.5, 4.0],
            [4.5, 4.0, 3.5, 3.0],
            [2.5, 2.0, 1.5, 1.0],
        ]);

        assert_eq!(matrix / 2.0, expected);
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut matrix = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let expected = Matrix4::new([
            [0.5, 1.0, 1.5, 2.0],
            [2.5, 3.0, 3.5, 4.0],
            [4.5, 4.0, 3.5, 3.0],
            [2.5, 2.0, 1.5, 1.0],
        ]);

        matrix /= 2.0;
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_element_wise() {
        let matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let matrix2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::new([
            [-2.0, 2.0, 6.0, 12.0],
            [15.0, 12.0, 7.0, -8.0],
            [36.0, 24.0, 42.0, 30.0],
            [5.0, 8.0, 21.0, 16.0],
        ]);

        assert_eq!(matrix1 & matrix2, expected);
    }

    #[test]
    fn test_element_wise_and_assign() {
        let mut matrix1 = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let matrix2 = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix4::new([
            [-2.0, 2.0, 6.0, 12.0],
            [15.0, 12.0, 7.0, -8.0],
            [36.0, 24.0, 42.0, 30.0],
            [5.0, 8.0, 21.0, 16.0],
        ]);

        matrix1 &= matrix2;
        assert_eq!(matrix1, expected);
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix4::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let tranposed = matrix.tranpose();
        let expected = Matrix4::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(tranposed, expected);
    }

    #[test]
    fn test_determinant_matrix2() {
        let matrix = Matrix2::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(matrix.determinant(), 17.0);
    }

    #[test]
    fn test_submatrix() {
        let matrix = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);

        let submatrix = matrix.submatrix(0, 2);
        let expected = Matrix3::new([[5.0, 6.0, 8.0], [9.0, 10.0, 12.0], [13.0, 14.0, 16.0]]);
        assert_eq!(submatrix, expected);

        let submatrix = matrix.submatrix(1, 1);
        let expected = Matrix3::new([[1.0, 3.0, 4.0], [9.0, 11.0, 12.0], [13.0, 15.0, 16.0]]);
        assert_eq!(submatrix, expected);
    }

    #[test]
    fn test_cofactor() {
        let matrix = Matrix3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(matrix.cofactor(0, 0), -12.0);
        assert_eq!(matrix.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_determinant() {
        let matrix = Matrix2::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(matrix.determinant(), 17.0);

        let matrix = Matrix3::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(matrix.determinant(), -196.0);

        let matrix = Matrix4::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.],
        ]);
        assert_eq!(matrix.determinant(), -4071.0);
    }

    #[test]
    fn test_inverse() {
        let matrix = Matrix4::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let inversed = matrix.inverse().unwrap();

        let expected = Matrix4::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_eq!(inversed, expected);
    }
}
