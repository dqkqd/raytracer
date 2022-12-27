pub(super) const ESPILON: f64 = 1E-5;

pub(super) fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < ESPILON
}

pub(crate) fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a == 0.0 {
        if b == 0.0 {
            return Vec::new();
        } else {
            return vec![-c / b];
        }
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        let sqrt_d = discriminant.sqrt();
        vec![(-b - sqrt_d) / (2.0 * a), (-b + sqrt_d) / (2.0 * a)]
    } else if discriminant == 0.0 {
        vec![-b / (2.0 * a)]
    } else {
        vec![]
    }
}

macro_rules! assert_float_eq {
    ($left:expr, $right: expr) => {
        if !$crate::util::equal($left, $right) {
            panic!(
                "float equal: `(left == right)`\nleft: `{}`,\nright: `{}`,",
                $left, $right
            );
        }
    };
}
pub(crate) use assert_float_eq;
