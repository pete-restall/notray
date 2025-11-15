use super::Result;

pub trait Canvas {
    fn set_pixel(&mut self, x: u16, y: u16, colour: u8) -> Result<()>;
}
