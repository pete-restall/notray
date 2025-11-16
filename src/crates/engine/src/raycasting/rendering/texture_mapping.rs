use fixed::traits::ToFixed;
use fixed::types::{U0F16, U16F0, U16F16};

use crate::{Canvas, Colour, Error, Result, Vector2d};
use super::{ColumnRendering, RenderingColumn};

pub type TextureCoordinate = U0F16;
pub type TextureCoordinates = Vector2d<TextureCoordinate>;

pub trait Texture {
    fn get_texel_at(&self, coordinates: TextureCoordinates) -> Option<Colour>;
}

pub struct StretchedStaticTexture<'t, const WIDTH_PIXELS: u8, const HEIGHT_PIXELS: u8, const TRANSPARENT_COLOUR: u16> {
    pixels: &'t [u8]
}

impl<'t, const WIDTH_PIXELS: u8, const HEIGHT_PIXELS: u8, const TRANSPARENT_COLOUR: u16> StretchedStaticTexture<'t, WIDTH_PIXELS, HEIGHT_PIXELS, TRANSPARENT_COLOUR> {
    const WIDTH_PIXELS: usize = WIDTH_PIXELS as usize;
    const WIDTH_PIXELS_FIXED: U16F0 = U16F0::const_from_int(WIDTH_PIXELS as u16);
    const HEIGHT_PIXELS_FIXED: U16F0 = U16F0::const_from_int(HEIGHT_PIXELS as u16);

    pub const fn new(pixels: &'t [u8]) -> Self {
        Self { pixels }
    }
}

impl<'t, const WIDTH_PIXELS: u8, const HEIGHT_PIXELS: u8, const TRANSPARENT_COLOUR: u16> Texture for StretchedStaticTexture<'t, WIDTH_PIXELS, HEIGHT_PIXELS, TRANSPARENT_COLOUR> {
    fn get_texel_at(&self, coordinates: TextureCoordinates) -> Option<Colour> {
        let u: usize = (coordinates.x().wide_mul(Self::WIDTH_PIXELS_FIXED)).to_num();
        let v: usize = (coordinates.y().wide_mul(Self::HEIGHT_PIXELS_FIXED)).to_num();
        let texel = self.pixels[v * Self::WIDTH_PIXELS + u];
        if texel as u16 != TRANSPARENT_COLOUR {
            Some(Colour::new(texel))
        } else {
            None
        }
    }
}

pub struct TextureMappedColumnRenderer<'c, TTexture: Texture> {
    texture: &'c TTexture,
    column: &'c mut RenderingColumn
}

impl<'c, TTexture: Texture> TextureMappedColumnRenderer<'c, TTexture> {
    pub const fn new(texture: &'c TTexture, column: &'c mut RenderingColumn) -> Self {
        Self { texture, column }
    }
}

impl<'c, TTexture: Texture> ColumnRendering for TextureMappedColumnRenderer<'_, TTexture> {
    fn render_column_onto<TCanvas: Canvas>(&mut self, canvas: &mut TCanvas) -> Result<()> {
        let ray_intersection = &self.column.raycasting.as_ref().map(|x| x.cell_offset());
        if self.column.unclipped_span_length == 0 || ray_intersection.is_none() {
            return Ok(());
        }

        let texel_x: TextureCoordinate = ray_intersection
            .unwrap()
            .checked_to_fixed()
            .ok_or(Error::TextureMappingOverflowX)?;

        let texel_y: TextureCoordinate = U16F16::from_num(self.column.span_clip_offset)
            .saturating_div(self.column.unclipped_span_length.into())
            .checked_to_fixed()
            .ok_or(Error::TextureMappingOverflowY)?;

        let texel_dy: TextureCoordinate = U16F16::ONE
            .saturating_div(self.column.unclipped_span_length.into())
            .checked_to_fixed()
            .ok_or(Error::TextureMappingOverflowDeltaY)?;

        let mut texel_coordinates = TextureCoordinates::new(texel_x, texel_y);
        while self.column.screen_y < self.column.screen_y_end {
            if let Some(texel) = self.texture.get_texel_at(texel_coordinates) {
                canvas.set_pixel(self.column.screen_x, self.column.screen_y, texel)?;
            }

            self.column.screen_y += 1;
            texel_coordinates.set_y(texel_coordinates.y().wrapping_add(texel_dy));
        }

        Ok(())
    }
}
