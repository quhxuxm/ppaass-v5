use bincode::error::{DecodeError, EncodeError};
use std::net::AddrParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    AddrParse(#[from] AddrParseError),
    #[error(transparent)]
    Encoder(#[from] EncodeError),
    #[error(transparent)]
    Decoder(#[from] DecodeError),
}
