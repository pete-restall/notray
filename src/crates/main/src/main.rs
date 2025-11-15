use notray_engine::{
    raycasting::{
        ProjectionPlaneParameters,
        Scene,
        worlds
    },
    GameLoop,
    raycasting_parameters
};

mod console;

mod result;
use result::*;

raycasting_parameters! {
    pub struct RaycastingParameters {
        canvas: 400 x 240 pixels;
        field_of_view: 62.5 degrees;
        sine_lookup_msbs: 6 bits; /* 6-14 for a single quadrant, 8-16 for a full table */
        sine_lookup_size: 90 degrees; /* 90 (single quadrant) or 360 degrees (full table) */
    }
}

fn main() -> Result<()> {
    let nc = console::Notcurses::new()?;
    let console = nc.console(RaycastingParameters::CANVAS_WIDTH_PIXELS, RaycastingParameters::CANVAS_HEIGHT_PIXELS)?;
    let stimuli = console.stimuli();
    let mut pollable = console.pollable();
    let mut canvas = console.canvas();

    let mut scene = Scene::<RaycastingParameters, _>::new(worlds::World1::new());
    let mut game_loop = GameLoop::new(
        &mut scene,
        &stimuli,
        &mut pollable,
        &mut canvas);

    game_loop.run().coalesce_err()
}
