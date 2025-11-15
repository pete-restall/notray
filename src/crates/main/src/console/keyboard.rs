use std::cell::RefCell;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{spawn, JoinHandle};

use notcurses::{Input, InputType, Key, KeyMod, Notcurses, Received};

use notray_engine::{Pollable, QuitStimuli, Result, Stimuli};
use notray_engine::raycasting::CameraStimuli;

pub struct NotcursesKeyboard<'nc> {
    _nc: &'nc RefCell<Notcurses>,
    _thread: JoinHandle<()>,
    events: RefCell<Receiver<Input>>,
    state: RefCell<KeyboardState>
}

struct BackgroundKeyboard;

impl BackgroundKeyboard {
    // BUG: nc.poll_event() is _NOT_ non-blocking, so we have to use a background thread.
    // The notcurses_get() function is explicitly called out in notcurses_input (3) as
    // one of the very few APIs that are threading-friendly in this regard.  And yes, I
    // tried various shenanigans, including fcntl(), polling on the 'inputready_fd', etc.
    // and they were all fruitless except for an amazing few hours I'll never get back.
    //
    // The problem is, only one nc context can exist for the duration of the program and
    // the notcurses-rs puts in guardrails.  So this unsafe (and unsound) hackery using
    // the C bindings and random pointers is to work around that.
    //
    // It is an error to call notcurses_get() after notcurses_stop() so we'll have a
    // use-after-free issue here unless this loop is terminated prior to exiting the
    // program.  We also leave ourselves open to future library changes introducing extra
    // notcurses calls that also break the fragile assumptions at play here.  At this point
    // I don't care about UB on program termination or any of the other stuff >:-/
    pub fn run(nc: *mut notcurses::sys::c_api::notcurses, tx: Sender<Input>) {
        use notcurses::sys::{NcInput, NcReceived};
        use notcurses::sys::c_api::{notcurses_get, NcResult_i32};
        let mut event = NcInput::new_empty();
        loop {
            let result = unsafe { notcurses_get(nc, std::ptr::null(), &raw mut event) as NcResult_i32 };
            let event = (NcReceived::from(result as u32), event).into();
            if tx.send(event).is_err() {
                break;
            }
        }
    }
}

struct KeyboardState {
    q: KeyState,
    esc: KeyState,
    left_arrow: KeyState,
    right_arrow: KeyState,
    up_arrow: KeyState,
    down_arrow: KeyState,
    shift: KeyState
}

#[derive(Copy, Clone)]
struct KeyState {
    is_pressed: bool,
    was_pressed: bool,
    _has_changed: bool
}

impl KeyState {
    pub fn default() -> Self {
        Self::new(false, false, false)
    }

    pub fn new(is_pressed: bool, was_pressed: bool, has_changed: bool) -> Self {
        Self { is_pressed, was_pressed, _has_changed: has_changed }
    }
}

impl<'nc> NotcursesKeyboard<'nc> {
    pub fn new(nc: &'nc RefCell<Notcurses>) -> Self {
        use notcurses::sys::c_api::notcurses as nc_hackery;
        let nc_ptr_hackery = nc.borrow_mut().with_nc_mut(|nc| nc as *mut nc_hackery as usize);
        let (tx, rx) = mpsc::channel();
        Self {
            _nc: nc,
            _thread: spawn(move || BackgroundKeyboard::run(nc_ptr_hackery as *mut nc_hackery, tx)),
            events: RefCell::from(rx),
            state: RefCell::new(KeyboardState {
                q: KeyState::default(),
                esc: KeyState::default(),
                up_arrow: KeyState::default(),
                down_arrow: KeyState::default(),
                left_arrow: KeyState::default(),
                right_arrow: KeyState::default(),
                shift: KeyState::default()
            })
        }
    }

    pub fn stimuli(&self) -> impl Stimuli + QuitStimuli + CameraStimuli {
        KeyboardStimuli::new(&self.state)
    }

    pub fn pollable(&self) -> impl Pollable {
        KeyboardPollable::new(&self.events, &self.state)
    }
}

impl KeyboardState {
    pub fn reset(&mut self) {
        self.esc = Self::reset_key_state(self.esc);
        self.q = Self::reset_key_state(self.q);
        self.up_arrow = Self::reset_key_state(self.up_arrow);
        self.down_arrow = Self::reset_key_state(self.down_arrow);
        self.left_arrow = Self::reset_key_state(self.left_arrow);
        self.right_arrow = Self::reset_key_state(self.right_arrow);
        self.shift = Self::reset_key_state(self.shift);
    }

    fn reset_key_state(state: KeyState) -> KeyState {
        KeyState::new(false, state.is_pressed, state.is_pressed)
    }

    pub fn on_key_pressed(&mut self, received: Received, modifiers: KeyMod) {
        match received {
            Received::Key(Key::Esc) => self.esc = Self::set_key_state(self.esc),
            Received::Char('q') | Received::Char('Q') => self.q = Self::set_key_state(self.q),
            Received::Key(Key::Up) => self.up_arrow = Self::set_key_state(self.up_arrow),
            Received::Key(Key::Down) => self.down_arrow = Self::set_key_state(self.down_arrow),
            Received::Key(Key::Left) => self.left_arrow = Self::set_key_state(self.left_arrow),
            Received::Key(Key::Right) => self.right_arrow = Self::set_key_state(self.right_arrow),
            _ => { }
        }

        if modifiers.has_shift() {
            self.shift = Self::set_key_state(self.shift);
        }
    }

    fn set_key_state(state: KeyState) -> KeyState {
        KeyState::new(true, state.was_pressed, !state.was_pressed)
    }
}

struct KeyboardPollable<'kb> {
    events: &'kb RefCell<Receiver<Input>>,
    state: &'kb RefCell<KeyboardState>
}

impl<'kb> KeyboardPollable<'kb> {
    pub fn new(events: &'kb RefCell<Receiver<Input>>, state: &'kb RefCell<KeyboardState>) -> Self {
        Self { events, state }
    }
}

impl<'kb> Pollable for KeyboardPollable<'kb> {
    fn poll(&mut self) -> Result<()> {
        let mut state = self.state.borrow_mut();
        state.reset();
        while let Ok(event) = self.events.borrow_mut().try_recv() {
            match event.itype {
                InputType::Unknown | InputType::Repeat => state.on_key_pressed(event.received, event.keymod),
                _ => { }
            }
        }
        return Ok(());
    }
}

struct KeyboardStimuli<'kb> {
    state: &'kb RefCell<KeyboardState>
}

impl<'kb> KeyboardStimuli<'kb> {
    pub fn new(state: &'kb RefCell<KeyboardState>) -> Self {
        Self { state }
    }
}

impl<'kb> Stimuli for KeyboardStimuli<'kb> { }

impl<'kb> QuitStimuli for KeyboardStimuli<'kb> {
    fn should_quit(&self) -> bool {
        let state = self.state.borrow();
        state.q.is_pressed || state.esc.is_pressed
    }
}

impl<'kb> CameraStimuli for KeyboardStimuli<'kb> {
    fn should_move_forward(&self) -> bool {
        self.state.borrow().up_arrow.is_pressed
    }

    fn should_move_backward(&self) -> bool {
        self.state.borrow().down_arrow.is_pressed
    }

    fn should_turn_left(&self) -> bool {
        self.state.borrow().left_arrow.is_pressed
    }

    fn should_turn_right(&self) -> bool {
        self.state.borrow().right_arrow.is_pressed
    }

    fn is_fast(&self) -> bool {
        self.state.borrow().shift.is_pressed
    }
}
