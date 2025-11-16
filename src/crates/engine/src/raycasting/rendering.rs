use crate::{Canvas, Colour, Result};

pub trait ColumnRendering {
    fn render_column_onto<TCanvas: Canvas>(&mut self, canvas: &mut TCanvas) -> Result<()>;
}

pub struct RenderingColumn {
    screen_x: u16,
    screen_y: u16,
    span_length: u16,
    screen_y_end: u16
}
impl RenderingColumn {
    pub const fn new(screen_x: u16, screen_y: u16, span_length: u16) -> Self {
        Self {
            screen_x,
            screen_y,
            span_length,
            screen_y_end: screen_y + span_length
        }
    }

    pub fn next_span(&mut self, span_length: u16) {
        self.span_length = span_length;
        self.screen_y_end += span_length;
    }
}

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
