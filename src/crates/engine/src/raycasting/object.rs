use super::{Angle, WorldCoordinates};

pub struct Object {
    position: WorldCoordinates,
    direction: Angle
}

impl Object {
    pub const fn new(position: WorldCoordinates, direction: Angle) -> Self {
        Self {
            position,
            direction
        }
    }

    pub fn position(&self) -> WorldCoordinates { self.position }

    pub fn set_position(&mut self, position: WorldCoordinates) {
        self.position = position;
    }

    pub fn direction(&self) -> Angle { self.direction }

    pub fn set_direction(&mut self, direction: Angle) {
        self.direction = direction;
    }
}
