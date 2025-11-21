use core::marker::PhantomData;

use fixed::FixedU16;
use fixed::traits::{LossyInto, ToFixed};
use fixed::types::{I2F14, I8F0, I8F24, U0F16, U11F21, U16F0, U2F14, U8F24};

use crate::{Canvas, Error, HasFixedPoint, Result, Vector2d};
use super::*;

pub struct RaycastingContext<TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry> {
    _parameters: PhantomData<TEngineParameters>,

    canvas_column_x: u16,
    canvas_column_angle: Angle,

    camera_direction_vector: Vector2d<<Angle as HasFixedPoint>::FixedPoint>,
    projection_plane_vector: Vector2d<<Angle as HasFixedPoint>::FixedPoint>,

    ray_origin: WorldCoordinates,
    ray_direction: Vector2d<I8F24>,
    ray_delta: Vector2d<U8F24>,
    ray_cell: WorldCoordinates,
    ray_abs_distance: Vector2d<U8F24>,
    ray_abs_distance_last: Vector2d<U8F24>,
    ray_cell_step: Vector2d<I8F0>,

    is_horizontal_ray_intersection: bool,
    distance_to_wall: U8F24,
    projected_wall_height: U11F21,
    cell_tag: Option<CellTag>
}

impl<TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry> RaycastingContext<TEngineParameters> {
    pub const fn default() -> Self {
        Self {
            _parameters: PhantomData,
            canvas_column_x: 0,
            canvas_column_angle: Angle::default(),
            camera_direction_vector: Vector2d::default(),
            projection_plane_vector: Vector2d::default(),
            ray_origin: WorldCoordinates::from_cell_top_left(0, 0),
            ray_direction: Vector2d::default(),
            ray_delta: Vector2d::default(),
            ray_cell: WorldCoordinates::from_cell_top_left(0, 0),
            ray_abs_distance: Vector2d::default(),
            ray_abs_distance_last: Vector2d::default(),
            ray_cell_step: Vector2d::default(),
            is_horizontal_ray_intersection: false,
            distance_to_wall: U8F24::MAX,
            projected_wall_height: U11F21::ZERO,
            cell_tag: None
        }
    }

    pub fn on_frame_start<'c, TCanvas>(&'c mut self, camera: &'c Camera<TEngineParameters>, canvas: &'c mut TCanvas) -> Result<Frame<'c, TEngineParameters, TCanvas>>
        where TCanvas: Canvas {

        self.ray_origin = camera.position();
        self.camera_direction_vector = camera.direction_vector();
        self.projection_plane_vector = camera.projection_plane_vector();

        self.canvas_column_x = 0;
        self._next_column()?;

        Ok(Frame::new(self, camera, canvas))
    }

    fn _next_column(&mut self) -> Result<()> {
        let ray_column_vector_scaling = TEngineParameters::CANVAS_COLUMN_NORMALISING_FACTOR.wide_mul(U16F0::from_num(self.canvas_column_x));
        let ray_vector_column_scaling = U2F14::from_bits(((ray_column_vector_scaling.to_bits() >> 2) & 0xffff) as u16);
        let ray_vector_column_scaling: I2F14 = ray_vector_column_scaling.cast_signed().sub_unsigned(FixedU16::ONE);

        self.ray_direction = Vector2d::new(
            self.projection_plane_vector.x()
                .wide_mul(ray_vector_column_scaling)
                .checked_add(self.camera_direction_vector.x().into())
                .ok_or(Error::RaycastingOverflowX)?
                .lossy_into(),
            self.projection_plane_vector.y()
                .wide_mul(ray_vector_column_scaling)
                .checked_add(self.camera_direction_vector.y().into())
                .ok_or(Error::RaycastingOverflowY)?
                .lossy_into());

        self.ray_delta = Vector2d::new(
            self.ray_direction.x().unsigned_abs().saturating_recip(),
            self.ray_direction.y().unsigned_abs().saturating_recip());

        self.ray_cell = self.ray_origin;

        let (initial_distance_x, cell_step_x) = if self.ray_direction.x() >= 0 {
            let initial_cell_x = self.ray_origin.cell_x_ceil();
            let distance_from_cell_edge_x = initial_cell_x - self.ray_origin.x();
            (distance_from_cell_edge_x.saturating_mul(self.ray_delta.x().lossy_into()), I8F0::ONE)
        } else {
            let initial_cell_x = self.ray_origin.cell_x_floor();
            let distance_from_cell_edge_x = self.ray_origin.x() - initial_cell_x;
            (distance_from_cell_edge_x.saturating_mul(self.ray_delta.x().lossy_into()), I8F0::NEG_ONE)
        };

        let (initial_distance_y, cell_step_y) = if self.ray_direction.y() >= 0 {
            let initial_cell_y = self.ray_origin.cell_y_ceil();
            let distance_from_cell_edge_y = initial_cell_y - self.ray_origin.y();
            (distance_from_cell_edge_y.saturating_mul(self.ray_delta.y().lossy_into()), I8F0::ONE)
        } else {
            let initial_cell_y = self.ray_origin.cell_y_floor();
            let distance_from_cell_edge_y = self.ray_origin.y() - initial_cell_y;
            (distance_from_cell_edge_y.saturating_mul(self.ray_delta.y().lossy_into()), I8F0::NEG_ONE)
        };

        self.ray_abs_distance = Vector2d::new(initial_distance_x.into(), initial_distance_y.into());
        self.ray_cell_step = Vector2d::new(cell_step_x, cell_step_y);

        Ok(())
    }

    pub fn next_column(&mut self) -> Result<bool> {
        self.canvas_column_x += 1;
        if self.canvas_column_x < TEngineParameters::CANVAS_WIDTH_PIXELS {
            self._next_column()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn cast_ray<TWorld: World>(&mut self, world: &TWorld) -> Result<()> {
        self.distance_to_wall = U8F24::MAX;
        self.projected_wall_height = U11F21::ZERO;
        for _ in 0..TEngineParameters::MAX_RAY_CELL_PROBES {
            self.is_horizontal_ray_intersection = self.ray_abs_distance.x() < self.ray_abs_distance.y();
            if self.is_horizontal_ray_intersection {
                self.ray_abs_distance_last.set_x(self.ray_abs_distance.x());
                self.ray_abs_distance.set_x(self.ray_abs_distance.x().saturating_add(self.ray_delta.x()));
                self.ray_cell.set_x(self.ray_cell.x().saturating_add_signed(self.ray_cell_step.x().into()));
            } else {
                self.ray_abs_distance_last.set_y(self.ray_abs_distance.y());
                self.ray_abs_distance.set_y(self.ray_abs_distance.y().saturating_add(self.ray_delta.y()));
                self.ray_cell.set_y(self.ray_cell.y().saturating_add_signed(self.ray_cell_step.y().into()));
            }

            // TODO: Place the match arms (but not the call to 'probe_cell') into another object that deals with (column) rendering...
            let probe = CellProbe::new(self.ray_cell);
            match world.probe_cell(&probe) {
                CellProbeResult::Opaque(cell_tag) => {
                    self.cell_tag = Some(cell_tag);
                    self.distance_to_wall = if self.is_horizontal_ray_intersection {
                        self.ray_abs_distance_last.x()
                    } else {
                        self.ray_abs_distance_last.y()
                    };

                    break;
                },

                CellProbeResult::PossiblyTransparent(_cell_tag) => {
                    /* TODO: Something like an object or a wall with a transparent texture, where
                       the ray needs to continue so that the background can be overdrawn by the
                       object with transparency; basically we will need a LIFO queue of
                       (x.x, y.y, distance) intersection tuples and the 'cell_tag' so we can traverse
                       it with the Painter's Algorithm during rendering */
                },

                CellProbeResult::Transparent(_cell_tag) => {
                    /* TODO: There is something of interest here, but this particular intersection
                       is empty - maybe a door is ajar, for example */
                }

                CellProbeResult::Empty => { }
            };
        }

        Ok(())
    }

    pub fn canvas_column_x(&self) -> u16 { self.canvas_column_x }

    pub fn canvas_column_angle(&self) -> Angle { self.canvas_column_angle }

    pub fn cell_intersection(&self) -> Option<RayCellIntersection> {
        if let Some(cell_tag) = self.cell_tag {
            let projected_wall_height = if self.distance_to_wall != 0 {
                TEngineParameters::ASPECT_RATIO_FOR_WALL_HEIGHT.saturating_div(self.distance_to_wall.lossy_into())
            } else {
                U11F21::ZERO
            };

            Some(RayCellIntersection::new(
                self.ray_origin,
                self.ray_direction,
                projected_wall_height,
                self.is_horizontal_ray_intersection,
                cell_tag))
        } else {
            None
        }
    }

    pub fn cell_tag(&self) -> Option<CellTag> { self.cell_tag }
}

pub struct RayCellIntersection {
    ray_origin: WorldCoordinates,
    ray_direction: Vector2d<I8F24>,
    projected_wall_height: U11F21,
    is_horizontal_intersection: bool,
    cell_tag: CellTag
}

impl RayCellIntersection {
    pub const fn new(
        ray_origin: WorldCoordinates,
        ray_direction: Vector2d<I8F24>,
        projected_wall_height: U11F21,
        is_horizontal_intersection: bool,
        cell_tag: CellTag) -> Self {

        Self {
            ray_origin,
            ray_direction,
            projected_wall_height,
            is_horizontal_intersection,
            cell_tag
        }
    }

    pub fn projected_wall_height_int(&self) -> u16 {
        let possibly_odd_wall_height: u16 = self.projected_wall_height.saturating_to_num();
        possibly_odd_wall_height & !1
    }

    pub fn cell_offset(&self) -> U0F16 {
        let intersection = if self.is_horizontal_intersection {
            self.projected_wall_height
                .cast_signed()
                .saturating_mul_add(self.ray_direction.y(), self.ray_origin.y().into())
        } else {
            self.projected_wall_height
                .cast_signed()
                .saturating_mul_add(self.ray_direction.x(), self.ray_origin.x().into())
        };

        intersection.cast_unsigned().frac().saturating_to_fixed()
    }

    pub fn is_horizontal_intersection(&self) -> bool { self.is_horizontal_intersection }

    pub fn cell_tag(&self) -> CellTag { self.cell_tag }
}
