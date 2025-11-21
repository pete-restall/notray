use crate::{Canvas, Colour, FrameRenderer, Result};
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
        TWorld: World + WorldRendering,
        TCanvas: Canvas {

    fn render_frame_onto(&mut self, canvas: &mut TCanvas) -> Result<()> {
        // TODO: Frame usage
        let mut frame = self.raycasting_context.on_frame_start(&self.camera, canvas)?;

        for x in 0..TEngineParameters::CANVAS_WIDTH_PIXELS {
            self.raycasting_context.cast_ray(&self.world)?;

            let cell_intersection = self.raycasting_context.cell_intersection();
            let projected_wall_height = if let Some(ref wall) = cell_intersection { wall.projected_wall_height_int() } else { 0 };
            let projected_wall_height_clipped = projected_wall_height.min(TEngineParameters::CANVAS_HEIGHT_PIXELS);
            let top_of_wall = (TEngineParameters::CANVAS_HEIGHT_PIXELS - projected_wall_height_clipped) / 2;
            let bottom_of_wall = TEngineParameters::CANVAS_HEIGHT_PIXELS - top_of_wall;

            let mut column = RenderingColumn::new(x, 0, cell_intersection);
            column.next_span(
                top_of_wall,
                0,
                TEngineParameters::CANVAS_HEIGHT_PIXELS / 2);
            {
                let mut sky = self.world.sky_for_column(self.raycasting_context.cell_tag(), &mut column);
                sky.render_column_onto(canvas)?;
            }

            column.next_span(
                projected_wall_height_clipped,
                (projected_wall_height - projected_wall_height_clipped) / 2,
                projected_wall_height);
            {
                let mut wall = self.world.wall_for_column(self.raycasting_context.cell_tag(), &mut column);
                wall.render_column_onto(canvas)?;
            }

            column.next_span(
                TEngineParameters::CANVAS_HEIGHT_PIXELS - bottom_of_wall,
                bottom_of_wall - TEngineParameters::CANVAS_HEIGHT_PIXELS / 2,
                TEngineParameters::CANVAS_HEIGHT_PIXELS);
            {
                let mut ground = self.world.ground_for_column(self.raycasting_context.cell_tag(), &mut column);
                ground.render_column_onto(canvas)?;
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

    pub fn set_pixel(&mut self, x: u16, y: u16, colour: Colour) -> Result<()> {
        self.canvas.set_pixel(x, y, colour)
    }
}
