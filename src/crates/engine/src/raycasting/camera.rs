use core::marker::PhantomData;

use fixed::traits::ToFixed;

use crate::{HasFixedPoint, Vector2d};
use super::*;

type FixedPoint = <Angle as HasFixedPoint>::FixedPoint;

pub struct Camera<TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry> {
    _parameters: PhantomData<TEngineParameters>,
    object: Object,
    direction_vector: Vector2d<FixedPoint>,
    projection_plane_vector: Vector2d<FixedPoint>
}

impl<TEngineParameters> Camera<TEngineParameters> where
    TEngineParameters: EngineParameters + ProjectionPlaneParameters + Trigonometry {

    pub fn new(object: Object) -> Self {
        let mut camera = Self {
            _parameters: PhantomData,
            object,
            direction_vector: Vector2d::default(),
            projection_plane_vector: Vector2d::default()
        };

        camera.object.set_position(camera.object.position());
        camera.set_direction(camera.object.direction());
        camera
    }

    pub fn position(&self) -> WorldCoordinates { self.object.position() }

    pub fn set_position(&mut self, position: WorldCoordinates) {
        self.object.set_position(position);
    }

    pub fn move_relative(&mut self, distance: WorldRelativeCoordinate) {
        let distance: fixed::types::I9F23 = distance.into();

        let delta_x = distance.saturating_mul(self.direction_vector.x().into());
        let x = delta_x.saturating_add(self.position().x().into())
            .unsigned_abs()
            .checked_to_fixed()
            .unwrap_or(WorldAbsoluteCoordinate::MAX);

        let delta_y = distance.saturating_mul(self.direction_vector.y().into());
        let y = delta_y.saturating_add(self.position().y().into())
            .unsigned_abs()
            .checked_to_fixed()
            .unwrap_or(WorldAbsoluteCoordinate::MAX);

        let new_position = WorldCoordinates::new(x, y);

        // TODO: collision detection...
        self.set_position(new_position);
    }

    pub fn direction(&self) -> Angle { self.object.direction() }

    pub fn turn(&mut self, delta: Angle) {
        self.set_direction(self.object.direction() + delta);
    }

    pub fn set_direction(&mut self, direction: Angle) {
        /*
            Matrix rotation:

                |cos(a) -sin(a)| * |x| = |x . cos(a) + y . -sin(a)|
                |sin(a)  cos(a)|   |y|   |x . sin(a) + y .  cos(a)|

            Full rotation of the initial direction and projection-plane (normal) vectors reduces error accumulation.
            The initial direction vector and its normal (ie. the projection-plane's vector) are:

                direction = (-1 0)
                normal    = ( 0 N)

            Working through the and simplifying, we have:

                direction = (    -cos(a),    -sin(a))
                normal    = (-N . sin(a), N . cos(a))
        */

        let sine = TEngineParameters::sine(direction);
        let cosine = TEngineParameters::cosine(direction);

        self.direction_vector = Vector2d::new(-cosine, -sine);
        self.projection_plane_vector = Vector2d::new(
            sine * -TEngineParameters::PROJECTION_PLANE_VECTOR_Y,
            cosine * TEngineParameters::PROJECTION_PLANE_VECTOR_Y);

        self.object.set_direction(direction);
    }

    pub fn direction_vector(&self) -> Vector2d<FixedPoint> { self.direction_vector }

    pub fn projection_plane_vector(&self) -> Vector2d<FixedPoint> { self.projection_plane_vector }

    pub fn is_facing_northwards(&self) -> bool { self.object.direction().is_within_quadrant_0_or_1() }

    pub fn is_facing_eastwards(&self) -> bool { self.object.direction().is_within_quadrant_3_or_0() }

    pub fn is_facing_southwards(&self) -> bool { self.object.direction().is_within_quadrant_2_or_3() }

    pub fn is_facing_westwards(&self) -> bool { self.object.direction().is_within_quadrant_1_or_2() }
}
