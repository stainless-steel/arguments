//! Parser for command-line arguments.
//!
//! ## Example
//!
//! ```
//! // foo --no-bar --baz 42 --baz 69 --qux "Hello, world!"
//! let arguments = std::env::args();
//! # let arguments = vec![
//! #     "foo",
//! #     "--no-bar",
//! #     "--baz", "42",
//! #     "--baz", "69",
//! #     "--qux", "Hello, world!",
//! # ];
//! # let arguments = arguments.iter().map(|a| a.to_string());
//! let arguments = arguments::parse(arguments).unwrap();
//!
//! assert_eq!(arguments.program, "foo");
//! assert_eq!(arguments.get::<bool>("bar").unwrap(), false);
//! assert_eq!(arguments.get::<usize>("baz").unwrap(), 69);
//! assert_eq!(arguments.get_all::<usize>("baz").unwrap(), &[42, 69]);
//! assert_eq!(arguments.get::<String>("qux").unwrap(), "Hello, world!");
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
