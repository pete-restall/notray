use fixed::types::{I1F15, U0F16, U11F21};

pub trait ProjectionPlaneParameters {
    const CANVAS_WIDTH_PIXELS: u16;
    const CANVAS_HEIGHT_PIXELS: u16;

    const CANVAS_COLUMN_NORMALISING_FACTOR: U0F16;
    const ASPECT_RATIO_FOR_WALL_HEIGHT: U11F21;

    const PROJECTION_PLANE_VECTOR_Y: I1F15;
}
