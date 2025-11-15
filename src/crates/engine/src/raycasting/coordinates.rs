use fixed::types::{I8F8, U8F8};

use crate::Vector2d;
use super::Angle;

pub type WorldAbsoluteCoordinate = U8F8;
pub type WorldRelativeCoordinate = I8F8;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct WorldCoordinates(Vector2d<WorldAbsoluteCoordinate>);

impl WorldCoordinates {
    pub const FACING_NORTH: Angle = Angle::from_raw(0x4000_u16 as i16);
    pub const FACING_EAST: Angle = Angle::from_raw(0x0000_u16 as i16);
    pub const FACING_SOUTH: Angle = Angle::from_raw(0xc000_u16 as i16);
    pub const FACING_WEST: Angle = Angle::from_raw(0x8000_u16 as i16);

    const HALF: WorldAbsoluteCoordinate = WorldAbsoluteCoordinate::lit("0.5");

    pub const fn new(x: WorldAbsoluteCoordinate, y: WorldAbsoluteCoordinate) -> Self {
        Self(Vector2d::new(x, y))
    }

    pub const fn from_cell_centre(x: u8, y: u8) -> Self {
        let cell = Self::from_cell_top_left(x, y);
        Self::new(
            cell.x().const_bitor(Self::HALF),
            cell.y().const_bitor(Self::HALF)
        )
    }

    pub const fn from_cell_top_left(x: u8, y: u8) -> Self {
        Self::new(
            WorldAbsoluteCoordinate::const_from_int(x as u16),
            WorldAbsoluteCoordinate::const_from_int(y as u16)
        )
    }

    pub const fn x(&self) -> WorldAbsoluteCoordinate { self.0.x() }

    pub fn set_x(&mut self, x: WorldAbsoluteCoordinate) { self.0.set_x(x); }

    pub fn cell_x_floor(&self) -> WorldAbsoluteCoordinate { self.x().floor() }

    pub fn cell_x_ceil(&self) -> WorldAbsoluteCoordinate {
        self.x().checked_ceil().unwrap_or(WorldAbsoluteCoordinate::MAX)
    }

    pub fn cell_x_int(&self) -> u8 { self.x().int().to_num() }

    pub fn cell_x_frac(&self) -> WorldAbsoluteCoordinate { self.x().frac() }

    pub const fn y(&self) -> WorldAbsoluteCoordinate { self.0.y() }

    pub fn set_y(&mut self, y: WorldAbsoluteCoordinate) { self.0.set_y(y); }

    pub fn cell_y_floor(&self) -> WorldAbsoluteCoordinate { self.y().floor() }

    pub fn cell_y_ceil(&self) -> WorldAbsoluteCoordinate {
        self.y().checked_ceil().unwrap_or(WorldAbsoluteCoordinate::MAX)
    }

    pub fn cell_y_int(&self) -> u8 { self.y().int().to_num() }

    pub fn cell_y_frac(&self) -> WorldAbsoluteCoordinate { self.y().frac() }
}
