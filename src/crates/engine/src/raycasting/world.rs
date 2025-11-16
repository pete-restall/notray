use super::{Angle, ColumnRendering, RenderingColumn, WorldCoordinates};

pub trait World {
    fn spawn_at(&self) -> WorldCoordinates;
    fn spawn_angle(&self) -> Angle;
    fn probe_cell(&self, probe: &CellProbe) -> CellProbeResult;
}

pub trait WorldRendering {
    type SkyRenderer<'c>: ColumnRendering where Self: 'c;
    type WallRenderer<'c>: ColumnRendering where Self: 'c;
    type GroundRenderer<'c>: ColumnRendering where Self: 'c;

    fn sky_for_column<'c>(&'c self, cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::SkyRenderer<'c>;
    fn wall_for_column<'c>(&'c self, cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::WallRenderer<'c>;
    fn ground_for_column<'c>(&'c self, cell: Option<CellTag>, column: &'c mut RenderingColumn) -> Self::GroundRenderer<'c>;
}

pub struct CellProbe {
    at: WorldCoordinates
}

impl CellProbe {
    pub const fn new(at: WorldCoordinates) -> Self {
        Self { at }
    }

    pub fn at(&self) -> WorldCoordinates { self.at }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct CellTag(u8);

impl CellTag {
    pub const fn from_world_cell_id(cell_id: u8) -> Self {
        Self(cell_id)
    }

    pub const fn world_cell_id(&self) -> u8 { self.0 }
}

pub enum CellProbeResult {
    Empty,
    Opaque(CellTag),
    Transparent(CellTag),
    PossiblyTransparent(CellTag)
}
