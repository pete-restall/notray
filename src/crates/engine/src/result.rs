#[derive(Debug)]
pub enum Error {
    Str(&'static str),
    RaycastingOverflowX,
    RaycastingOverflowY,
    RaycastingFellOffTheWorld
}

pub type Result<T> = core::result::Result<T, Error>;
