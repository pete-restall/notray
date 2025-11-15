use std::cell::RefCell;

use notray_engine::{Canvas, Pollable, QuitStimuli, Result as EngineResult, Stimuli};
use notray_engine::raycasting::CameraStimuli;

use crate::Result;
use super::{NotcursesKeyboard, ResultCoalescing, NotcursesScreen};

pub struct NotcursesConsole<'nc> {
    keyboard: NotcursesKeyboard<'nc>,
    screen: NotcursesScreen<'nc>
}

impl<'nc> NotcursesConsole<'nc> {
    pub fn new(nc: &'nc RefCell<notcurses::Notcurses>, width_pixels: u16, height_pixels: u16) -> Result<Self> {
        nc.borrow_mut().refresh().coalesce_err()?;

        let keyboard = NotcursesKeyboard::new(&nc);
        let screen = NotcursesScreen::new(&nc, width_pixels, height_pixels)?;
        Ok(Self {
            keyboard,
            screen
        })
    }

    pub fn stimuli(&self) -> impl Stimuli + QuitStimuli + CameraStimuli {
        self.keyboard.stimuli()
    }

    pub fn pollable(&'nc self) -> impl Pollable {
        NotcursesPollable {
            keyboard: self.keyboard.pollable(),
            screen: self.screen.pollable()
        }
    }

    pub fn canvas(&'nc self) -> impl Canvas {
        self.screen.canvas()
    }
}

struct NotcursesPollable<K: Pollable, S: Pollable> {
    keyboard: K,
    screen: S
}

impl<K: Pollable, S: Pollable> Pollable for NotcursesPollable<K, S> {
    fn poll(&mut self) -> EngineResult<()> {
        self.screen.poll()?;
        self.keyboard.poll()
    }
}
