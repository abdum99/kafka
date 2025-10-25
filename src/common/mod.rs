use crate::{StrError};

pub mod api;
pub mod error;
pub mod response;
pub mod request;
pub mod types;

#[derive(thiserror::Error, Debug)]
pub enum EncodingError {
    #[error("unexpected end of input")]
    Eof,
    #[error("utf-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("varint too long")]
    VarIntTooLong,
    #[error("varint overflow")]
    VarIntOverflow,
    #[error("invalid length: {0}")]
    InvalidLength(i64),
}


pub trait DecodeFromBytes {
    fn read_from_u8(input: &[u8], offset: &mut usize) -> Result<Self, EncodingError>
        where Self: Sized;
}

pub trait EncodeToBytes {
    fn write_to_bytes(&self) -> Vec<u8>;
}
