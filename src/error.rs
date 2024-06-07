#[derive(Debug)]
pub struct TmiError {}

impl std::fmt::Display for TmiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tmi Error")
    }
}

impl std::error::Error for TmiError {}
