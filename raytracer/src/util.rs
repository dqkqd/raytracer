pub(super) const EPSILON: f64 = 1E-5;
pub(super) const INFINITY: f64 = 1E10;

pub(super) fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub(crate) fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> Vec<f64> {
    if equal(a.abs(), 0.0) {
        if equal(b.abs(), 0.0) {
            return Vec::new();
        } else {
            return vec![-c / b];
        }
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        let sqrt_d = discriminant.sqrt();
        vec![(-b - sqrt_d) / (2.0 * a), (-b + sqrt_d) / (2.0 * a)]
    } else if equal(discriminant.abs(), 0.0) {
        vec![-b / (2.0 * a)]
    } else {
        vec![]
    }
}

pub(crate) fn solve_linear_equation(a: f64, b: f64) -> Vec<f64> {
    solve_quadratic_equation(0.0, a, b)
}

pub(crate) fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let tmin;
    let tmax;

    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * INFINITY;
        tmax = tmax_numerator * INFINITY;
    }

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

#[allow(unused_macros)]
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

#[allow(unused_imports)]
pub(crate) use assert_float_eq;
