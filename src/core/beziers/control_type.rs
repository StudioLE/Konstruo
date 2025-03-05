use ControlType::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlType {
    Start,
    StartHandle,
    EndHandle,
    End,
}

impl ControlType {
    /// Get a [`ControlType`] by index.
    #[must_use]
    pub fn by_index(index: usize) -> Self {
        let index = index % 4;
        match index {
            0 => Start,
            1 => StartHandle,
            2 => EndHandle,
            3 => End,
            _ => unreachable!(),
        }
    }
}
