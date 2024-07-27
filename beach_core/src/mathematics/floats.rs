use crate::mathematics::constants::*;

/// <https://stackoverflow.com/a/6400477/247218>
pub fn modulo(a: f32, b: f32) -> f32 {
    a - b * (a / b).floor()
}

/// Round to the nearest multiple.
pub fn round_to(value: f32, multiple: f32) -> f32 {
    (value / multiple).round() * multiple
}

/// Determine if two floating point numbers are almost equal.
pub fn is_almost_equal_to(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

/// Determine if two floating point numbers are almost equal.
pub fn is_almost_zero(value: f32) -> bool {
    value.abs() < EPSILON
}

/// Determine if a floating point number is close to an integer.
pub fn is_almost_integer(value: f32) -> Option<f32> {
    let integer = value.round();
    if is_almost_equal_to(value, integer) {
        Some(integer)
    } else {
        None
    }
}

/// Determine if a floating point number is close to a fraction of PI.
pub fn is_almost_fractional_pi(value: f32) -> Option<f32> {
    let rounded = round_to(value, EIGHTH_PI);
    if is_almost_equal_to(value, rounded) {
        Some(rounded)
    } else {
        None
    }
}

/// Determine if a floating point number is close to a fraction of root two.
pub fn is_almost_fractional_root_two(value: f32) -> Option<f32> {
    let rounded = round_to(value, ONE_OVER_ROOT_2);
    if is_almost_equal_to(value, rounded) {
        Some(rounded)
    } else {
        None
    }
}

/// Attempt to remove floating point errors by rounding to [`EPSILON`] unless:
/// - it's a fraction of π
/// - it's a fraction of √2
pub fn fix_floating(value: f32) -> f32 {
    if is_almost_zero(value) {
        0.0
    } else if let Some(value) = is_almost_fractional_pi(value) {
        value
    } else if let Some(value) = is_almost_fractional_root_two(value) {
        value
    }  else {
        round_to(value, EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _round_to() {
        assert_eq!(round_to(0.44, 0.1), 0.4);
        assert_eq!(round_to(0.45, 0.1), 0.5);
        assert_eq!(round_to(0.49999997, 0.1), 0.5);
        assert_eq!(round_to(0.5, 0.1), 0.5);
    }

    #[test]
    fn _modulo() {
        assert_eq!(modulo(0.0, 1.0), 0.0);
        assert_eq!(modulo(0.1, 1.0), 0.1);
        assert_eq!(modulo(0.9, 1.0), 0.9);
        assert_eq!(modulo(1.0, 1.0), 0.0);
        assert_eq!(modulo(-0.0, 1.0), 0.0);
        assert_eq!(modulo(-0.1, 1.0), 0.9);
        // assert_eq!(fix_floating_points(modulo(-0.9, 1.0)), 0.1);
        assert_eq!(modulo(-1.0, 1.0), 0.0);
    }

    #[test]
    fn _is_almost_integer() {
        assert_eq!(is_almost_integer(0.49999997), None);
        assert_eq!(is_almost_integer(1.0), Some(1.0));
        assert_eq!(is_almost_integer(1.1), None);
        assert_eq!(is_almost_integer(1.9), None);
        assert_eq!(is_almost_integer(1.99999), None);
        assert_eq!(is_almost_integer(1.999999), Some(2.0));
        assert_eq!(is_almost_integer(-1.99999), None);
        assert_eq!(is_almost_integer(-1.999999), Some(-2.0));
    }

    #[test]
    fn _is_almost_fractional_pi() {
        assert_eq!(is_almost_fractional_pi(0.0), Some(0.0));
        assert_eq!(is_almost_fractional_pi(0.00000001), Some(0.0));
        assert_eq!(is_almost_fractional_pi(PI), Some(PI));
        assert_eq!(
            is_almost_fractional_pi(EIGHTH_PI + 0.00000001),
            Some(EIGHTH_PI)
        );
        assert_eq!(is_almost_fractional_pi(PI + 0.00000001), Some(PI));
        assert_eq!(is_almost_fractional_pi(-PI - 0.00000001), Some(-PI));
    }

    #[test]
    fn _fix_floating_points() {
        assert_eq!(fix_floating(0.0), 0.0);
        assert_eq!(fix_floating(0.00000001), 0.0);
        assert_eq!(fix_floating(0.100000024), 0.1);
        assert_eq!(fix_floating(0.49999997), 0.5);
        assert_eq!(fix_floating(1.999999), 1.999999);
        assert_eq!(fix_floating(1.9999999), 2.0);
        assert_eq!(fix_floating(-1.999999), -1.999999);
        assert_eq!(fix_floating(-1.9999999), -2.0);
        assert_eq!(fix_floating(PI), PI);
        assert_eq!(fix_floating(PI + 0.00000001), PI);
        assert_eq!(fix_floating(-PI - 0.00000001), -PI);
        assert_eq!(fix_floating(EIGHTH_PI + 0.00000001), EIGHTH_PI);
        assert_eq!(fix_floating(1.2363448e-7), 0.0);
    }
}
