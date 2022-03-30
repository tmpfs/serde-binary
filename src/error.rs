use std::fmt;
use thiserror::Error;

/// Errors thrown by the serde integration.
#[derive(Debug, Error)]
pub enum Error {
    /// Error generated by the serializer or deserializer.
    #[error("{0}")]
    Message(String),

    /// Error generated when a sequence has too many items.
    #[error("sequence has too many items, limit is 2^32")]
    TooManyItems,

    /// Error generated by the binary reader or writer.
    #[error(transparent)]
    Binary(#[from] binary_rw::BinaryError),
}

impl serde::ser::Error for Error {
    #[cold]
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    #[cold]
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}