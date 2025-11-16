use crate::{Colour, WellKnownColours};

pub struct Palette;

impl Palette {
    pub const BLACK: Colour = Colour::new(0);
    pub const TRANSPARENT: Colour = Colour::new(0);
    pub const GRASS_LIGHTEST: Colour = Colour::new(48);
    pub const SKY_LIGHTEST: Colour = Colour::new(144);

    pub fn rgb_for(colour: Colour) -> (u8, u8, u8) {
        static PALETTE: &[u8; 256 * 3] = include_bytes!("palette.rgb");
        let index = colour.as_index() as usize * 3;
        let entry = &PALETTE[index..index + 3];
        (entry[0], entry[1], entry[2])
    }
}

impl WellKnownColours for Palette {
    const BLACK: Colour = Palette::BLACK;
    const TRANSPARENT: Colour = Palette::TRANSPARENT;
}

pub static BRICK_1_64x64: &[u8; 64 * 64] = include_bytes!("brick1-64x64.raw");
