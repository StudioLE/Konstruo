use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec6 {
    /// Left (-x)
    pub left: f32,
    /// Right (+x)
    pub right: f32,
    /// Front (-y)
    pub front: f32,
    /// Back (+y)
    pub back: f32,
    /// Bottom (-z)
    pub bottom: f32,
    /// Top (+z)
    pub top: f32,
}

impl Vec6 {
    pub const ZERO: Self = Self {
        left: 0.0,
        right: 0.0,
        front: 0.0,
        back: 0.0,
        bottom: 0.0,
        top: 0.0,
    };

    #[must_use]
    pub fn new(left: f32, right: f32, front: f32, back: f32, bottom: f32, top: f32) -> Self {
        Self {
            left,
            right,
            front,
            back,
            bottom,
            top,
        }
    }

    #[must_use]
    pub fn with_right(mut self, value: f32) -> Self {
        self.right = value;
        self
    }

    #[must_use]
    pub fn with_left(mut self, value: f32) -> Self {
        self.left = value;
        self
    }

    #[must_use]
    pub fn with_back(mut self, value: f32) -> Self {
        self.back = value;
        self
    }

    #[must_use]
    pub fn with_front(mut self, value: f32) -> Self {
        self.front = value;
        self
    }

    #[must_use]
    pub fn with_top(mut self, value: f32) -> Self {
        self.top = value;
        self
    }

    #[must_use]
    pub fn with_botom(mut self, value: f32) -> Self {
        self.bottom = value;
        self
    }

    #[must_use]
    pub fn splat(value: f32) -> Self {
        Self {
            right: value,
            left: value,
            back: value,
            front: value,
            top: value,
            bottom: value,
        }
    }

    /// Ensure that positive values are greater than negative values.
    #[must_use]
    pub fn fix_order(self) -> Self {
        let (x_neg, x_pos) = if self.right < self.left {
            (self.right, self.left)
        } else {
            (self.left, self.right)
        };
        let (y_neg, y_pos) = if self.back < self.front {
            (self.back, self.front)
        } else {
            (self.front, self.back)
        };
        let (z_neg, z_pos) = if self.top < self.bottom {
            (self.top, self.bottom)
        } else {
            (self.bottom, self.top)
        };
        Self {
            right: x_pos,
            left: x_neg,
            back: y_pos,
            front: y_neg,
            top: z_pos,
            bottom: z_neg,
        }
    }

    #[must_use]
    pub fn get_pos(&self) -> Vec3 {
        Vec3::new(self.right, self.back, self.top)
    }

    #[must_use]
    pub fn get_neg(&self) -> Vec3 {
        Vec3::new(self.left, self.front, self.bottom)
    }
}
