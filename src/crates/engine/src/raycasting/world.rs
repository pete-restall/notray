use super::{Angle, WorldCoordinates};

pub trait World {
    fn spawn_at(&self) -> WorldCoordinates;
    fn spawn_angle(&self) -> Angle;
    fn probe_cell(&self, probe: &CellProbe) -> CellProbeResult;
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
    pub const fn default() -> Self {
        Self(0)
    }

    pub fn from_world_cell_id(cell_id: u8) -> Self {
        Self(cell_id)
    }
}

pub enum CellProbeResult {
    Empty,
    Opaque(CellTag),
    Transparent(CellTag),
    PossiblyTransparent(CellTag)
}
