use core::fmt;
use std::{io, str::Utf8Error};

#[derive(Debug)]
pub enum BinaryFileReaderError {
    IO(io::Error),
    Utf8Error(Utf8Error),

    BufferUnderflow {
        requested_bytes: usize,
        current_offset: usize,
        available_bytes: usize,
    },

    Expect {},

    OufOfRange {
        buffer_size: usize,
        got: usize,
    },
}

impl From<io::Error> for BinaryFileReaderError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl std::error::Error for BinaryFileReaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BinaryFileReaderError::Utf8Error(err) => Some(err),
            BinaryFileReaderError::IO(err) => Some(err),
            BinaryFileReaderError::BufferUnderflow { .. } => None,
            BinaryFileReaderError::Expect {} => None,
            BinaryFileReaderError::OufOfRange { .. } => None,
        }
    }
}

impl From<Utf8Error> for BinaryFileReaderError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}
impl fmt::Display for BinaryFileReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryFileReaderError::Utf8Error(err) => write!(f,"Utf8Error: {}", err),
            BinaryFileReaderError::IO(err) => write!(f, "IO error: {}", err),
            BinaryFileReaderError::BufferUnderflow {
                requested_bytes,
                current_offset,
                available_bytes,
            } => write!(
                f,
                "Buffer underflow: requested {} bytes at offset {}, but only {} bytes are available",
                requested_bytes, current_offset, available_bytes
            ),
            BinaryFileReaderError::Expect {  } => write!(
                f,
                "", 
            ),
            BinaryFileReaderError::OufOfRange { .. } => write!(
                f,
                ""
            )
        }
    }
}
