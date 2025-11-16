#[derive(Debug)]
pub enum Error {
    Str(&'static str),
    RaycastingOverflowX,
    RaycastingOverflowY,
    RaycastingFellOffTheWorld,
    TextureMappingOverflowX,
    TextureMappingOverflowY,
    TextureMappingOverflowDeltaY
}

pub type Result<T> = core::result::Result<T, Error>;
