#[derive(Debug)]
pub enum Error {
    Str(&'static str),
    String(String),
    EngineError(notray_engine::Error)
}

pub type Result<T> = core::result::Result<T, Error>;

type EngineResult<T> = notray_engine::Result<T>;
type EngineError = notray_engine::Error;

pub trait ResultCoalescing<T> {
    fn coalesce_err(self) -> Result<T>;
}

impl<T> ResultCoalescing<T> for EngineResult<T> {
    fn coalesce_err(self) -> Result<T> { self.map_err(|error| error.into()) }
}

impl From<EngineError> for Error {
    fn from(value: EngineError) -> Self {
        if let EngineError::Str(oops) = value {
            Self::Str(oops)
        } else {
            Self::EngineError(value)
        }
    }
}

pub trait EngineResultCoalescing<T> {
    fn coalesce_err(self) -> EngineResult<T>;
}

impl<T> EngineResultCoalescing<T> for Result<T> {
    fn coalesce_err(self) -> EngineResult<T> { self.map_err(|error| error.into()) }
}

impl From<Error> for EngineError {
    fn from(value: Error) -> Self {
        if let Error::Str(oops) = value {
            Self::Str(oops)
        } else {
            Self::Str("Error in main crate does not translate nicely to an engine error")
        }
    }
}
