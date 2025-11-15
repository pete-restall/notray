use std::cell::RefCell;
use std::pin::Pin;

use notcurses::{Notcurses, Plane, Size, Visual, VisualBuilder};

use crate::{Error, Result};
use super::ResultCoalescing;

pub struct NotcursesCanvas<'nc> {
    nc: &'nc RefCell<Notcurses>,
    nc_plane: Plane,
    pixels: Pin<Box<[Pixel]>>,
    dimensions: Size,
    width_pixels: u16
}

impl<'nc> NotcursesCanvas<'nc> {
    pub fn new(nc: &'nc RefCell<Notcurses>, nc_plane: Plane, width_pixels: u16, height_pixels: u16) -> Result<Self> {
        let width_pixels_i32 = width_pixels as i32;
        let height_pixels_i32 = height_pixels as i32;
        if
            width_pixels < 128 || width_pixels > 1024 || !is_even(width_pixels_i32) ||
            height_pixels < 64 || height_pixels > 1024 || !is_even(height_pixels_i32) {

            Err(Error::Str("Canvas dimensions must be even numbers of at least 128x64 pixels, but no more than 1024x1024 pixels"))
        } else {
            let rgba = Box::into_pin(
                vec![Pixel { red: 0, green: 0, blue: 0, alpha: 0xff }; (width_pixels_i32 * height_pixels_i32) as usize]
                .into_boxed_slice());

            Ok(Self {
                nc,
                nc_plane,
                pixels: rgba,
                dimensions: Size::new(width_pixels_i32, height_pixels_i32),
                width_pixels
            })
        }
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, colour: u8) -> Result<()> {
        let (x, y, width) = (x as usize, y as usize, self.width_pixels as usize);
        let index = y * width + x;

        // TODO: A proper palette needs to be defined / loaded
        let (red, green, blue) = if colour == 1 { (0, 0, 255) } else if colour == 2 { (127, 0, 0) } else if colour == 3 { (255, 0, 0) } else if colour == 4 { (0, 255, 0) } else { (0, 0, 0) };
        *self.pixels.get_mut(index).unwrap() = Pixel { red, green, blue, alpha: 0xff };
        Ok(())
    }

    pub fn blit(&mut self) -> Result<()> {
        let mut visual = self.new_visual()?;
        visual
            .blit_child(&self.nc.borrow(), &mut self.nc_plane)
            .coalesce_err()?
            .render()
            .coalesce_err()
    }

    fn new_visual(&mut self) -> Result<Visual> {
        let (_, rgba_as_bytes, _) = unsafe { self.pixels.align_to::<u8>() };
        VisualBuilder::new()
            .blitter_pixel()
            .scale(notcurses::Scale::Stretch)
            .build_from_rgba(&self.nc.borrow(), rgba_as_bytes, self.dimensions)
            .coalesce_err()
    }
}

fn is_even(value: i32) -> bool { (value & 1) == 0 }

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}
