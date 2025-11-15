use crate::{Canvas, FrameRenderer, Result};
use crate::raycasting::*;

pub struct Scene<TEngineParameters, TWorld>
    where
        TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry,
        TWorld: World {

    world: TWorld,
    camera: Camera<TEngineParameters>,
    raycasting_context: RaycastingContext<TEngineParameters>
}

impl<TEngineParameters, TWorld> Scene<TEngineParameters, TWorld>
    where
        TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry,
        TWorld: World {

    pub fn new(world: TWorld) -> Self {
        Self {
            camera: Camera::new(Object::new(world.spawn_at(), world.spawn_angle())),
            world,
            raycasting_context: RaycastingContext::default()
        }
    }
}

impl<TEngineParameters, TWorld> HasCameraMut for Scene<TEngineParameters, TWorld>
    where
        TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry,
        TWorld: World {

    type EngineParameters = TEngineParameters;

    fn camera_mut(&mut self) -> &mut Camera<Self::EngineParameters> { &mut self.camera }
}

impl<TEngineParameters, TWorld, TCanvas> FrameRenderer<TCanvas> for Scene<TEngineParameters, TWorld>
    where
        TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry,
        TWorld: World,
        TCanvas: Canvas {

    fn render_frame_onto(&mut self, canvas: &mut TCanvas) -> Result<()> {
        // TODO: Frame usage
        let mut frame = self.raycasting_context.on_frame_start(&self.camera, canvas)?;

        for x in 0..TEngineParameters::CANVAS_WIDTH_PIXELS {
            self.raycasting_context.cast_ray(&self.world)?;

            let projected_wall_height = self.raycasting_context.projected_wall_height();

            // TODO: Just a daft rendering of a solid colour at present - more sophistication is required
            let top_of_wall = (TEngineParameters::CANVAS_HEIGHT_PIXELS - projected_wall_height) / 2;
            let bottom_of_wall = TEngineParameters::CANVAS_HEIGHT_PIXELS - top_of_wall;
            for y in 0..top_of_wall {
                canvas.set_pixel(x, y, 1)?;
            }
            for y in top_of_wall..bottom_of_wall {
                canvas.set_pixel(x, y, if self.raycasting_context.is_wall_horizontal() { 2 } else { 3 })?;
            }
            for y in bottom_of_wall..TEngineParameters::CANVAS_HEIGHT_PIXELS {
                canvas.set_pixel(x, y, 4)?;
            }

            if !self.raycasting_context.next_column()? {
                break
            }
        }

        Ok(())
    }
}

pub struct Frame<'c, TEngineParameters, TCanvas>
    where
        TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry,
        TCanvas: Canvas {

    _camera: &'c Camera<TEngineParameters>,
    canvas: &'c mut TCanvas,
    _raycasting_context: &'c RaycastingContext<TEngineParameters>
}

impl<'c, TEngineParameters, TCanvas> Frame<'c, TEngineParameters, TCanvas>
    where
        TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry,
        TCanvas: Canvas {

    pub fn new(raycasting_context: &'c mut RaycastingContext<TEngineParameters>, camera: &'c Camera<TEngineParameters>, canvas: &'c mut TCanvas) -> Self
        where TCanvas: Canvas {

        Self { _camera: camera, canvas, _raycasting_context: raycasting_context }
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, colour: u8) -> Result<()> {
        self.canvas.set_pixel(x, y, colour)
    }
}
