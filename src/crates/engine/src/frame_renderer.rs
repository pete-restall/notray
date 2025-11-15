use super::{Canvas, Result};

pub trait FrameRenderer<TCanvas: Canvas> {
    fn render_frame_onto(&mut self, canvas: &mut TCanvas) -> Result<()>;
}
