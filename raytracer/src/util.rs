pub(super) const ESPILON: f64 = 1E-5;

pub(super) fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < ESPILON
}
