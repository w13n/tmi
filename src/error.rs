#[derive(Debug)]
pub enum TmiError {
    MemoryAccessU8ConversionError,
    UnmatchedLoopClose,
    UnmatchedLoopOpen,
    MemoryLimitExceeded,
    MemoryUnderflow,
}

impl std::fmt::Display for TmiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TmiError::MemoryAccessU8ConversionError => {
                write!(f, "runtime error: cell value could not be converted to an unsigned 8-bit int to print")
            }
            TmiError::UnmatchedLoopClose => {
                write!(
                    f,
                    "parsing error: a ']' without a corresponding '[' was found"
                )
            }
            TmiError::UnmatchedLoopOpen => {
                write!(
                    f,
                    "parsing error: a '[' without a corresponding ']' was found"
                )
            }
            TmiError::MemoryLimitExceeded => {
                write!(f, "runtime error: the memory limit has been exceeded")
            }

            TmiError::MemoryUnderflow => {
                write!(f, "runtime error: the program attempted to access a negative memory cell")
            }
        }
    }
}

impl std::error::Error for TmiError {}
