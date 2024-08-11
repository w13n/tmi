#[derive(Debug)]
pub enum TmiError {
    UnmatchedLoopClose,
    UnmatchedLoopOpen,
    FileAccessError,
    MemoryAccessU8ConversionError,
    MemoryLimitExceeded,
    MemoryUnderflow,
}

impl std::fmt::Display for TmiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TmiError::FileAccessError => {
                write!(f, "parsing error: source code file could not be read")
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
            TmiError::MemoryAccessU8ConversionError => {
                write!(
                    f,
                    "runtime error: cell value could not be interpreted to print"
                )
            }
            TmiError::MemoryLimitExceeded => {
                write!(f, "runtime error: the memory limit has been exceeded")
            }

            TmiError::MemoryUnderflow => {
                write!(
                    f,
                    "runtime error: the program attempted to access a negative memory cell"
                )
            }
        }
    }
}

impl std::error::Error for TmiError {}
