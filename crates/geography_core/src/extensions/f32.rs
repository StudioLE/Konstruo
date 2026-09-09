//! Extension methods on [`f32`].

/// Height of a vertex with no survey data.
pub const NO_DATA: f32 = f32::MIN;

/// Extension methods on [`f32`].
pub trait F32Extensions {
    /// Is this height the [`NO_DATA`] marker?
    fn is_no_data(self) -> bool;
}

impl F32Extensions for f32 {
    /// - Compares loosely, so any future re-encoding still reads as missing
    fn is_no_data(self) -> bool {
        self < -1e30
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_is_no_data() {
        // Act
        let marker = NO_DATA.is_no_data();
        let surveyed = 42.0_f32.is_no_data();
        let below = (-1e31_f32).is_no_data();
        let above = (-1e29_f32).is_no_data();
        // Assert
        assert!(marker);
        assert!(!surveyed);
        assert!(below);
        assert!(!above);
    }
}
