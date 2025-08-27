use alloc::boxed::Box;
use core::{
    error::Error,
    fmt::{Debug, Display},
};

use async_compat::Compat;
use tokio::net::{
    TcpStream,
    tcp::{OwnedReadHalf, OwnedWriteHalf, ReuniteError},
};

use crate::connection::{Combinable, RawWritable, Splittable};

impl Splittable for Compat<TcpStream> {
    fn into_split(self: Box<Self>) -> (Box<dyn Combinable>, Box<dyn RawWritable>) {
        let (read, write) = self.into_inner().into_split();
        (Box::new(Compat::new(read)), Box::new(Compat::new(write)))
    }
}

// -------------------------------------------------------------------------------------------------

impl Combinable for Compat<OwnedReadHalf> {
    fn into_combined(
        self: Box<Self>,
        write: Box<dyn RawWritable>,
    ) -> Result<Box<dyn Splittable>, Box<dyn Error + Send + Sync>> {
        if let Ok(write) = write.into_any().downcast::<Compat<OwnedWriteHalf>>() {
            match self.into_inner().reunite(write.into_inner()) {
                Ok(stream) => Ok(Box::new(Compat::new(stream))),
                Err(err) => Err(Box::new(AsyncTokioError::Tokio(err))),
            }
        } else {
            Err(Box::new(AsyncTokioError::IncorrectType))
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum AsyncTokioError {
    IncorrectType,
    #[expect(dead_code, reason = "Currently not public, may change in the future")]
    Tokio(ReuniteError),
}

impl AsyncTokioError {
    const fn message(&self) -> &'static str {
        match self {
            AsyncTokioError::IncorrectType => "Attempted to recombine incorrect socket types",
            AsyncTokioError::Tokio(..) => "Tokio error during socket recombination",
        }
    }
}

impl Error for AsyncTokioError {}
impl Display for AsyncTokioError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AsyncTokioError: {}", self.message())
    }
}
