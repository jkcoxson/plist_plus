// Jackson Coxson

#[derive(Debug)]
pub enum PlistError {
    Success,
    InvalidArg,
    Format,
    Parse,
    NoMem,
    Unknown,
}

impl From<i32> for PlistError {
    fn from(code: i32) -> PlistError {
        match code {
            0 => PlistError::Success,
            -1 => PlistError::InvalidArg,
            -2 => PlistError::Format,
            -3 => PlistError::Parse,
            -4 => PlistError::NoMem,
            _ => PlistError::Unknown,
        }
    }
}

impl std::fmt::Display for PlistError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlistError::Success => write!(f, "Success"),
            PlistError::InvalidArg => write!(f, "InvalidArg"),
            PlistError::Format => write!(f, "Format"),
            PlistError::Parse => write!(f, "Parse"),
            PlistError::NoMem => write!(f, "NoMem"),
            PlistError::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::error::Error for PlistError {}
