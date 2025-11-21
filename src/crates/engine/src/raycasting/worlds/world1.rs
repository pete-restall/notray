use crate::raycasting::*;
use crate::raycasting::worlds::assets;

pub struct World1 {
    textures: assets::Textures<'static>
}

impl World1 {
    pub const fn new() -> Self {
        Self {
            textures: assets::Textures::new()
        }
    }
}

impl World for World1 {
    fn spawn_at(&self) -> WorldCoordinates { WorldCoordinates::from_cell_centre(2, 2) }

    fn spawn_angle(&self) -> Angle { WorldCoordinates::FACING_SOUTH }

    fn probe_cell(&self, probe: &CellProbe) -> CellProbeResult {
        let cell_x = probe.at().cell_x_int() as usize;
        let cell_y = probe.at().cell_y_int() as usize;
        if let Some(cell_type) = CELLS.get(cell_y).and_then(|row| row.get(cell_x)) {
            if *cell_type == 0 {
                CellProbeResult::Empty
            } else {
                CellProbeResult::Opaque(CellTag::from_world_cell_id(*cell_type))
            }
        } else {
            CellProbeResult::Opaque(CellTag::from_world_cell_id(0))
        }
    }
}

impl WorldRendering for World1 {
    type SkyRenderer<'c> = SolidColourColumnRenderer<'c>;

    type WallRenderer<'c> = assets::TextureRenderer<'c>;

    type GroundRenderer<'c> = SolidColourColumnRenderer<'c>;

    fn sky_for_column<'c>(&self, _cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::SkyRenderer<'c> {
        Self::SkyRenderer::new(assets::Palette::SKY_LIGHTEST, column)
    }

    fn wall_for_column<'c>(&'c self, cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::WallRenderer<'c> {
        self.textures.new_renderer_for(cell, column)
    }

    fn ground_for_column<'c>(&self, _cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::GroundRenderer<'c> {
        Self::GroundRenderer::new(assets::Palette::GRASS_LIGHTEST, column)
    }
}

impl<'c> ColumnRendering for assets::TextureRenderer<'c> {
    fn render_column_onto<TCanvas: crate::Canvas>(&mut self, canvas: &mut TCanvas) -> crate::Result<()> {
        match self {
            assets::TextureRenderer::Unknown(renderer) => renderer.render_column_onto(canvas),
            assets::TextureRenderer::Brick1(renderer) => renderer.render_column_onto(canvas),
            assets::TextureRenderer::Stone1(renderer) => renderer.render_column_onto(canvas)
        }
    }
}

static CELLS: [[u8; 16]; 16] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2],
    [1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
];
