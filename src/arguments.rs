use options::Options;
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
    /// Get the last value of an option (if present) converted to a specific
    /// type (if possible).
    pub fn get<T: FromStr>(&self, name: &str) -> Option<T> {
        self.options
            .get_ref::<Vec<String>>(name)
            .and_then(|strings| strings.last().and_then(|string| string.parse().ok()))
    }

    /// Get all values of an option (if present) converted to a specific type
    /// (if possible).
    pub fn get_all<T: FromStr>(&self, name: &str) -> Option<Vec<T>> {
        self.options
            .get_ref::<Vec<String>>(name)
            .and_then(|strings| {
                strings
                    .iter()
                    .map(|string| string.parse().ok())
                    .collect::<Option<Vec<_>>>()
            })
    }
}
