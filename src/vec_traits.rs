use bevy::prelude::*;

pub trait Vec2Flip {
    fn flip_x(self) -> Self;
    fn flip_y(self) -> Self;
}

impl Vec2Flip for IVec2 {
    fn flip_x(self) -> Self {
        Self::new(-self.x, self.y)
    }

    fn flip_y(self) -> Self {
        Self::new(self.x, -self.y)
    }
}
