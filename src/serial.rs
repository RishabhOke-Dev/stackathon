use std::{error::Error, fmt::Display};




#[derive(Debug)]
pub enum SerializationError {
    EndOfFile,
    InvalidTagByte(u8),
    InvalidUTF8Encoding(std::string::FromUtf8Error), 
    InvalidFile,
    InvalidVersion,
}

impl Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationError::EndOfFile => write!(f,"Serialization Error: Unexpected end of file/data while reading."),
            SerializationError::InvalidTagByte(t) => write!(f, "Serialization Error: Encountered invalid tag byte: {:#02X}", t),
            SerializationError::InvalidUTF8Encoding(e) => write!(f, "Serialization Error: Invalid UTF-8 encoding. {}", e),
            SerializationError::InvalidFile => write!(f, "Serialization Error: Incorrect file for stackathon library."),
            SerializationError::InvalidVersion => write!(f, "Serialization Error: Incompatible library version."),
        }
    }
}

impl Error for SerializationError {}

pub trait ByteSized {
    fn to_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
    where
        Self: Sized;
}