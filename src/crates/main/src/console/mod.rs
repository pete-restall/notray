use std::cell::RefCell;

use notcurses::NotcursesResult;

use crate::{Error, Result, ResultCoalescing};

mod canvas;
use canvas::*;

mod console;
use console::*;

mod keyboard;
use keyboard::*;

mod screen;
use screen::*;

pub struct Notcurses {
    nc: RefCell<notcurses::Notcurses>
}

impl Notcurses {
    pub fn new() -> Result<Self> {
        Self::nc_new().coalesce_err()
    }

    fn nc_new() -> NotcursesResult<Self> {
        Ok(Self {
            nc: RefCell::from(
                notcurses::NotcursesBuilder::new()
                    .cli_mode(true)
                    .build()?)
        })
    }

    pub fn console(&self, width_pixels: u16, height_pixels: u16) -> Result<NotcursesConsole<'_>> {
        NotcursesConsole::new(&self.nc, width_pixels, height_pixels)
    }
}

impl<T> ResultCoalescing<T> for NotcursesResult<T> {
    fn coalesce_err(self) -> Result<T> {
        self.map_err(|error| Error::String(error.to_string()))
    }
}
