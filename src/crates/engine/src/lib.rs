#![no_std]

mod canvas;
pub use canvas::*;

mod frame_renderer;
pub use frame_renderer::*;

mod game_loop;
pub use game_loop::*;

pub mod raycasting;

mod pollable;
pub use pollable::*;

mod result;
pub use result::*;

mod stimuli;
pub use stimuli::*;

mod vector2d;
pub use vector2d::*;

pub trait HasFixedPoint {
    type FixedPoint: fixed::traits::Fixed;
}
