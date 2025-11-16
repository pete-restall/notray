use crate::Colour;
use crate::raycasting::*;
use crate::raycasting::worlds::assets;

pub struct World1;

impl World1 {
    pub const fn new() -> Self { Self }
}

enum CellType {
    SolidColour(u8, u8)
}

// TODO: For this (static) world, the CellTag is an index into this array...the renderer needs to call back after raycasting to get at this stuff
static CELL_TYPES: [CellType; 2] = [
    CellType::SolidColour(0, 0),
    CellType::SolidColour(2, 3)
];

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

    type WallRenderer<'c> = SolidColourColumnRenderer<'c>;

    type GroundRenderer<'c> = SolidColourColumnRenderer<'c>;

    fn sky_for_column<'c>(&self, _cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::SkyRenderer<'c> {
        Self::SkyRenderer::new(assets::Palette::SKY_LIGHTEST, column)
    }

    fn wall_for_column<'c>(&self, _cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::WallRenderer<'c> {
        Self::WallRenderer::new(Colour::new(76), column)
    }

    fn ground_for_column<'c>(&self, _cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::GroundRenderer<'c> {
        Self::GroundRenderer::new(assets::Palette::GRASS_LIGHTEST, column)
    }
}

static CELLS: [[u8; 16]; 16] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
];
