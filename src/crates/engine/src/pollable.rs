use super::Result;

pub trait Pollable {
    fn poll(&mut self) -> Result<()>;
}
