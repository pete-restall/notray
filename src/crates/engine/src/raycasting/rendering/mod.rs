use crate::{Canvas, Result};
use crate::raycasting::RayCellIntersection;

mod solid_colour;
pub use solid_colour::*;

mod texture_mapping;
pub use texture_mapping::*;

pub trait ColumnRendering {
    fn render_column_onto<TCanvas: Canvas>(&mut self, canvas: &mut TCanvas) -> Result<()>;
}

pub struct RenderingColumn {
    screen_x: u16,
    screen_y: u16,
    clipped_span_length: u16,
    span_clip_offset: u16,
    unclipped_span_length: u16,
    screen_y_end: u16,
    raycasting: Option<RayCellIntersection>
}

impl RenderingColumn {
    pub const fn new(screen_x: u16, screen_y: u16, raycasting: Option<RayCellIntersection>) -> Self {
        Self {
            screen_x,
            screen_y,
            clipped_span_length: 0,
            span_clip_offset: 0,
            unclipped_span_length: 0,
            screen_y_end: 0,
            raycasting
        }
    }

    pub fn next_span(&mut self, clipped_span_length: u16, span_clip_offset: u16, unclipped_span_length: u16) {
        self.clipped_span_length = clipped_span_length;
        self.span_clip_offset = span_clip_offset;
        self.unclipped_span_length = unclipped_span_length;
        self.screen_y_end += clipped_span_length;
    }

    pub fn raycasting(&self) -> &Option<RayCellIntersection> { &self.raycasting }
}
