use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::{plist_err_t, plist_err_t_PLIST_ERR_FORMAT, plist_err_t_PLIST_ERR_INVALID_ARG, plist_err_t_PLIST_ERR_NO_MEM, plist_err_t_PLIST_ERR_PARSE, plist_err_t_PLIST_ERR_UNKNOWN};

#[derive(Debug)]
pub enum PlistError {
    InvalidArg,
    Format,
    Parse,
    NoMemory,
    Unknown,
    Dealloc
}

impl PlistError {
    pub fn try_from(error: plist_err_t) -> Result<(), Self> {
        match Self::from(error) {
            Some(error) => Err(error),
            None => Ok(())
        }
    }
}

impl PlistError {
    fn from(error: plist_err_t) -> Option<Self> {
        let error = match error {
            plist_err_t_PLIST_ERR_INVALID_ARG => Self::InvalidArg,
            plist_err_t_PLIST_ERR_FORMAT => Self::Format,
            plist_err_t_PLIST_ERR_PARSE => Self::Parse,
            plist_err_t_PLIST_ERR_NO_MEM => Self::NoMemory,
            plist_err_t_PLIST_ERR_UNKNOWN => Self::Unknown,
            _ => return None
        };

        Some(error)
    }
}

impl Display for PlistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::InvalidArg => "Invalid argument",
            Self::Format => "Invalid format",
            Self::Parse => "Parse failed",
            Self::NoMemory => "No memory",
            Self::Dealloc => "Dealloc plist_t",
            Self::Unknown => "Unknown"
        };

        write!(f, "{}", s)
    }
}

impl Error for PlistError {}
