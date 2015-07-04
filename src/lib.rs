//! Parser for command-line arguments.
//!
//! ## Example
//!
//! ```
//! let args = std::env::args(); // foo --bar --baz 42 --qux 'To be?'
//! # let args = vec!["foo", "--bar", "--baz", "42", "--qux", "To be?"];
//! # let args = args.iter().map(|a| a.to_string());
//! let args = arguments::parse(args).unwrap();
//!
//! println!("Foo: {}", args.program);
//! println!("Bar: {}", args.get::<bool>("bar").unwrap());
//! println!("Baz: {}", args.get::<usize>("baz").unwrap());
//! println!("Qux: {}", args.get::<String>("qux").unwrap());
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
pub fn parse<I: Iterator<Item=String>>(stream: I) -> Result<Arguments> {
    Parser::new().parse(stream)
}
