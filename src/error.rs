use core::fmt;
use std::io;

#[derive(Debug)]
pub enum BinaryFileReaderError {
    IO(io::Error),

    BufferUnderflow {
        requested_bytes: usize,
        current_offset: usize,
        available_bytes: usize,
    },

    Expect {},
}

impl From<io::Error> for BinaryFileReaderError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl std::error::Error for BinaryFileReaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BinaryFileReaderError::IO(err) => Some(err),
            BinaryFileReaderError::BufferUnderflow { .. } => None,
            BinaryFileReaderError::Expect {} => None,
        }
    }
}

impl fmt::Display for BinaryFileReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            )
        }
    }
}
