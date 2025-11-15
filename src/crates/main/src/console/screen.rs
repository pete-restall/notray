use std::cell::RefCell;

use notcurses::Notcurses;

use notray_engine::{Canvas, Pollable, Result as EngineResult};

use crate::{EngineResultCoalescing, Result};
use super::{NotcursesCanvas, ResultCoalescing};

pub struct NotcursesScreen<'nc> {
    canvas: RefCell<NotcursesCanvas<'nc>>
}

impl<'nc> NotcursesScreen<'nc> {
    pub fn new(nc: &'nc RefCell<Notcurses>, width_pixels: u16, height_pixels: u16) -> Result<Self> {
        Ok(Self {
            canvas: RefCell::from(NotcursesCanvas::new(
                nc,
                nc.borrow().cli_plane().coalesce_err()?,
                width_pixels,
                height_pixels)?)
        })
    }

    pub fn pollable(&'nc self) -> impl Pollable {
        SharedCanvas { canvas: &self.canvas }
    }

    pub fn canvas(&'nc self) -> impl Canvas {
        SharedCanvas { canvas: &self.canvas }
    }
}

struct SharedCanvas<'c> {
    canvas: &'c RefCell<NotcursesCanvas<'c>>
}

impl<'c> Pollable for SharedCanvas<'c> {
    fn poll(&mut self) -> EngineResult<()> {
        self.canvas.borrow_mut().blit().coalesce_err()
    }
}

impl<'nc> Canvas for SharedCanvas<'nc> {
    fn set_pixel(&mut self, x: u16, y: u16, colour: u8) -> EngineResult<()> {
        self.canvas.borrow_mut().set_pixel(x, y, colour).coalesce_err()
    }
}
