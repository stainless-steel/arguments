use options::Options;
use std::any::Any;
use std::str::FromStr;

/// Command-line arguments.
#[derive(Debug)]
pub struct Arguments {
    /// The name of the executable.
    pub program: String,
    /// Raw options (all strings).
    pub options: Options,
    /// Unclassified arguments.
    pub orphans: Vec<String>,
}

impl Arguments {
    /// Get the value of an option (if present) converted to a specific type (if
    /// possible).
    pub fn get<T: Any + Clone + FromStr>(&self, name: &str) -> Option<T> {
        self.options.get_ref::<String>(name).and_then(|string| string.parse().ok())
    }
}
