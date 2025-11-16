use crate::{Canvas, Colour, Result};
use super::{ColumnRendering, RenderingColumn};

pub struct SolidColourColumnRenderer<'c> {
    colour: Colour,
    column: &'c mut RenderingColumn
}

impl<'c> SolidColourColumnRenderer<'c> {
    pub const fn new(colour: Colour, column: &'c mut RenderingColumn) -> Self {
        Self { colour, column }
    }
}

impl ColumnRendering for SolidColourColumnRenderer<'_> {
    fn render_column_onto<TCanvas: Canvas>(&mut self, canvas: &mut TCanvas) -> Result<()> {
        while self.column.screen_y < self.column.screen_y_end {
            canvas.set_pixel(self.column.screen_x, self.column.screen_y, self.colour)?;
            self.column.screen_y += 1;
        }

        Ok(())
    }
}
