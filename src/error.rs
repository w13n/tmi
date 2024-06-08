#[derive(Debug)]
pub enum TmiError {
    MemoryAccessU8ConversionError,
    UnmatchedLoopClose,
    UnmatchedLoopOpen,
    MemoryBoundsExceeded,
}

impl std::fmt::Display for TmiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TmiError::MemoryAccessU8ConversionError => {write!(f, "runtime error: cell value could not be converted to an unsigned 8-bit int to print")}
            TmiError::UnmatchedLoopClose => {write!(f, "parsing error: a ']' without a corresponding '[' was found")}
            TmiError::UnmatchedLoopOpen => {write!(f, "parsing error: a '[' without a corresponding ']' was found")}
            TmiError::MemoryBoundsExceeded => {write!(f, "runtime error: the memory limits have been exceeded")}
        }

    }
}

impl std::error::Error for TmiError {}
