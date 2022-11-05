//! Parser for command-line arguments.
//!
//! ## Example
//!
//! ```
//! let arguments = std::env::args(); // foo --no-bar --baz 42 --qux 'To be?'
//! # let arguments = vec!["foo", "--no-bar", "--baz", "42", "--qux", "To be?"];
//! # let arguments = arguments.iter().map(|a| a.to_string());
//! let arguments = arguments::parse(arguments).unwrap();
//!
//! println!("Foo: {}", arguments.program);
//! println!("Bar: {}", arguments.get::<bool>("bar").unwrap());
//! println!("Baz: {}", arguments.get::<usize>("baz").unwrap());
//! println!("Qux: {}", arguments.get::<String>("qux").unwrap());
//! ```

extern crate options;

use std::{error, fmt};

/// An error.
pub struct Error(pub &'static str);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(Error($message)));
);

mod arguments;
mod parser;

pub use arguments::Arguments;
pub use options::Options;
pub use parser::Parser;

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Parse command-line arguments.
#[inline]
pub fn parse<I: Iterator<Item = String>>(stream: I) -> Result<Arguments> {
    Parser::new().parse(stream)
}
