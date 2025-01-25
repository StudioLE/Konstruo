use crate::core::mathematics::floats::modulo;

pub enum ClampFloat {
    Fixed(f32, f32),
    Wrapped(f32),
}

impl ClampFloat {
    #[must_use]
    pub fn clamp(&self, number: f32) -> f32 {
        match self {
            ClampFloat::Fixed(min, max) => number.clamp(*min, *max),
            ClampFloat::Wrapped(max) => modulo(number, *max),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn clamp_fixed() {
        // Arrange
        let min = -1.0;
        let max = 1.0;
        let clamp = ClampFloat::Fixed(min, max);

        // Act
        // Assert
        assert_eq!(clamp.clamp(max), max);
        assert_eq!(clamp.clamp(1.1), max);
        assert_eq!(clamp.clamp(0.9999), 0.9999);
        assert_eq!(clamp.clamp(0.0), 0.0);
        assert_eq!(clamp.clamp(min), min);
        assert_eq!(clamp.clamp(-1.1), min);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn clamp_wrapped() {
        // Arrange
        let max = 1.0;
        let clamp = ClampFloat::Wrapped(max);

        // Act
        // Assert
        assert_eq!(clamp.clamp(max), 0.0);
        assert_eq!(clamp.clamp(1.1), 0.100_000_024);
        assert_eq!(clamp.clamp(0.9999), 0.9999);
        assert_eq!(clamp.clamp(0.0), 0.0);
        assert_eq!(clamp.clamp(-1.1), 0.9);
    }
}
