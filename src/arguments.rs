use options::Options;
use std::any::{Any, TypeId};
use std::str::FromStr;

/// Command-line arguments.
pub struct Arguments {
    /// The name of the executable.
    pub program: String,
    /// Raw options (either booleans or strings).
    pub options: Options,
    /// Unclassified arguments.
    pub orphans: Vec<String>,
}

impl Arguments {
    /// Get the value of an option (if present) converted to a specific type (if
    /// possible).
    pub fn get<T: Any + Clone + FromStr>(&self, name: &str) -> Option<T> {
        let id = TypeId::of::<T>();
        if id == TypeId::of::<bool>() || id == TypeId::of::<String>() {
            self.options.get::<T>(name)
        } else {
            self.options.get_ref::<String>(name).and_then(|string| {
                match string.parse() {
                    Ok(value) => Some(value),
                    _ => None,
                }
            })
        }
    }
}
