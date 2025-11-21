use crate::HasFixedPoint;
use super::{Angle, WorldAbsoluteCoordinate};

pub trait EngineParameters {
    const MAX_RAY_CELL_PROBES: usize;
}

pub trait Trigonometry {
    fn sine(angle: Angle) -> <Angle as HasFixedPoint>::FixedPoint;
    fn cosine(angle: Angle) -> <Angle as HasFixedPoint>::FixedPoint;
}

#[macro_export]
macro_rules! raycasting_parameters {
    (pub struct $TypeName:ident {
        canvas: $canvas_width_pixels:literal x $canvas_height_pixels:literal pixels;
        field_of_view: $fov_degrees:literal degrees;
        sine_lookup_msbs: $sine_lookup_msbs:literal bits;
        sine_lookup_size: $sine_lookup_size_degrees:literal degrees;
    }) => {
        ::notray_procmacro::_raycasting_parameters!(
            $TypeName,
            $canvas_width_pixels,
            $canvas_height_pixels,
            $fov_degrees,
            $sine_lookup_msbs,
            $sine_lookup_size_degrees);
    };
}
