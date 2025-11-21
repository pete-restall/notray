use fixed::types::U16F0;

use crate::Colour;
use super::{Texture, TextureCoordinates};

pub struct StretchedStaticTexture<'t, const WIDTH_PIXELS: u8, const HEIGHT_PIXELS: u8, const TRANSPARENT_COLOUR: u16> {
    pixels: &'t [u8]
}

impl<'t, const WIDTH_PIXELS: u8, const HEIGHT_PIXELS: u8, const TRANSPARENT_COLOUR: u16> StretchedStaticTexture<'t, WIDTH_PIXELS, HEIGHT_PIXELS, TRANSPARENT_COLOUR> {
    const WIDTH_PIXELS: usize = WIDTH_PIXELS as usize;
    const WIDTH_PIXELS_FIXED: U16F0 = U16F0::const_from_int(WIDTH_PIXELS as u16);
    const HEIGHT_PIXELS: usize = HEIGHT_PIXELS as usize;
    const HEIGHT_PIXELS_FIXED: U16F0 = U16F0::const_from_int(HEIGHT_PIXELS as u16);

    const _ENSURE_WIDTH_IS_POWER_2: () = assert!(WIDTH_PIXELS != 0 && WIDTH_PIXELS.is_power_of_two(), "Texture Width must be a power of two");
    const _ENSURE_HEIGHT_IS_POWER_2: () = assert!(HEIGHT_PIXELS != 0 && HEIGHT_PIXELS.is_power_of_two(), "Texture Height must be a power of two");

    pub const fn new(pixels: &'t [u8]) -> Self {
        Self { pixels }
    }
}

impl<'t, const WIDTH_PIXELS: u8, const HEIGHT_PIXELS: u8, const TRANSPARENT_COLOUR: u16> Texture for StretchedStaticTexture<'t, WIDTH_PIXELS, HEIGHT_PIXELS, TRANSPARENT_COLOUR> {
    fn get_texel_at(&self, coordinates: TextureCoordinates) -> Option<Colour> {
        let u: usize = (coordinates.x().wide_mul(Self::WIDTH_PIXELS_FIXED)).round().to_num();
        let u = u.min(Self::WIDTH_PIXELS - 1);

        let v: usize = (coordinates.y().wide_mul(Self::HEIGHT_PIXELS_FIXED)).round().to_num();
        let v = v.min(Self::HEIGHT_PIXELS - 1);

        let texel = self.pixels[v * Self::WIDTH_PIXELS + u];
        if texel as u16 != TRANSPARENT_COLOUR {
            Some(Colour::new(texel))
        } else {
            None
        }
    }
}
