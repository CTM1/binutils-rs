use std::fmt;
use std::error::Error as StdError;
use std::ffi::{NulError, FromBytesWithNulError};
use std::str::Utf8Error;
// bfd lib errors

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BfdError {
    NoError = 0,
    SystemCall,
    InvalidTarget,
    WrongFormat,
    WrongObjectFormat,
    InvalidOperation,
    NoMemory,
    NoSymbols,
    NoArmap,
    NoMoreArchivedFiles,
    MalformedArchive,
    MissingDso,
    FileNotRecognized,
    FileAmbiguouslyRecognized,
    NoContents,
    NonrepresentableSection,
    NoDebugSection,
    BadValue,
    FileTruncated,
    FileTooBig,
    Sorry,
    OnInput,
    InvalidErrorCode,
}

impl From<u32> for BfdError {
    fn from(error: u32) -> Self {
        match error {
            0 => BfdError::NoError,
            1 => BfdError::SystemCall,
            2 => BfdError::InvalidTarget,
            3 => BfdError::WrongFormat,
            4 => BfdError::WrongObjectFormat,
            5 => BfdError::InvalidOperation,
            6 => BfdError::NoMemory,
            7 => BfdError::NoSymbols,
            8 => BfdError::NoArmap,
            9 => BfdError::NoMoreArchivedFiles,
            10 => BfdError::MalformedArchive,
            11 => BfdError::MissingDso,
            12 => BfdError::FileNotRecognized,
            13 => BfdError::FileAmbiguouslyRecognized,
            14 => BfdError::NoContents,
            15 => BfdError::NonrepresentableSection,
            16 => BfdError::NoDebugSection,
            17 => BfdError::BadValue,
            18 => BfdError::FileTruncated,
            19 => BfdError::FileTooBig,
            20 => BfdError::Sorry,
            21 => BfdError::OnInput,
            22 => BfdError::InvalidErrorCode,
            _ => BfdError::InvalidErrorCode,
        }
    }
}

impl From<BfdError> for u32 {
    fn from(error: BfdError) -> Self {
        error as u32
    }
}

#[derive(Debug)]
pub enum Error {
    BfdError(u32, String),
    DisassembleInfoError(String),
    SectionError(String),
    CommonError(String),
    NulError(NulError),
    FromBytesWithNulError(FromBytesWithNulError),
    Utf8Error(std::str::Utf8Error),
    NullPointerError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BfdError(code, msg) => write!(f, "BFD error {}: {}", code, msg),
            Error::DisassembleInfoError(msg) => write!(f, "Disassemble info error: {}", msg),
            Error::SectionError(section) => write!(f, "Can't find '{}' section!", section),
            Error::CommonError(msg) => write!(f, "Common error: {}", msg),
            Error::FromBytesWithNulError(e) => write!(f, "FromBytesWithNul error: {}", e),
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

impl From<NulError> for Error {
    fn from(error: NulError) -> Self {
        Error::NulError(error)
    }
}

impl From<FromBytesWithNulError> for Error {
    fn from(error: FromBytesWithNulError) -> Self {
        Error::FromBytesWithNulError(error)
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Error::Utf8Error(error)
    }
}