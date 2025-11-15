use fixed::traits::Fixed;

#[derive(Copy, Clone, Debug)]
pub struct Vector2d<T: Fixed>(T, T);

impl<T: Fixed> Vector2d<T> {
    pub const fn default() -> Self {
        Self::new(T::ZERO, T::ZERO)
    }

    pub const fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    pub const fn x(&self) -> T { self.0 }

    pub fn set_x(&mut self, x: T) { self.0 = x; }

    pub const fn y(&self) -> T { self.1 }

    pub fn set_y(&mut self, y: T) { self.1 = y; }
}
