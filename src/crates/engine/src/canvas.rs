use super::Result;

pub trait Canvas {
    fn set_pixel(&mut self, x: u16, y: u16, colour: Colour) -> Result<()>;
}

pub trait WellKnownColours {
    const BLACK: Colour;
    const TRANSPARENT: Colour;
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Colour(u8);

impl Colour {
    pub const fn new(index: u8) -> Self { Self(index) }

    pub const fn as_index(self) -> u8 { self.0 }
}
