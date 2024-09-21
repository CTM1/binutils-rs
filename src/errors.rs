use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    BfdError(BfdError, String),
    DisassembleInfoError(String),
    SectionError(String),
    CommonError(String),
    NulError(std::ffi::NulError),
    Utf8Error(std::str::Utf8Error),
    NullPointerError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BfdError(tag, msg) => write!(f, "BFD error {}: {}", tag as u32, msg),
            Error::DisassembleInfoError(msg) => write!(f, "Disassemble info error: {}", msg),
            Error::SectionError(section) => write!(f, "Can't find '{}' section!", section),
            Error::CommonError(msg) => write!(f, "Common error: {}", msg),
            Error::NulError(e) => write!(f, "Nul error: {}", e),
            Error::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            Error::NullPointerError(msg) => write!(f, "Null pointer error: {}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::NulError(e) => Some(e),
            Error::Utf8Error(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Self {
        Error::NulError(error)
    }
}

impl From<std::ffi::FromBytesWithNulError> for Error {
    fn from(error: std::ffi::FromBytesWithNulError) -> Self {
        Error::NulError(error.into())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Utf8Error(error)
    }
}